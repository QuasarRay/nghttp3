//! RFC 9204 prefixed integer coding used by QPACK instructions.
//!
//! The encoded form is at most one prefix byte plus ten base-128 continuation
//! bytes for a u64 value, so the safe core uses fixed storage and performs no
//! allocation.

/// Maximum encoded size for one u64 QPACK prefixed integer.
pub const MAX_ENCODED_LEN: usize = 11;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EncodedPrefixedInt {
    bytes: [u8; MAX_ENCODED_LEN],
    len: u8,
}

impl EncodedPrefixedInt {
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..usize::from(self.len)]
    }

    pub const fn len(&self) -> usize {
        self.len as usize
    }

    pub const fn is_empty(&self) -> bool {
        false
    }
}

fn prefix_mask(prefix_bits: u8) -> Option<u8> {
    if !(1..=8).contains(&prefix_bits) {
        return None;
    }
    Some(u8::MAX >> (8 - prefix_bits))
}

/// Encodes one RFC 9204/HPACK-style prefixed integer.
///
/// `high_bits` contains the instruction bits that share the first byte. Its
/// lower `prefix_bits` must be clear.
pub fn encode_prefixed(
    prefix_bits: u8,
    high_bits: u8,
    value: u64,
) -> Option<EncodedPrefixedInt> {
    let mask = prefix_mask(prefix_bits)?;
    if high_bits & mask != 0 {
        return None;
    }

    let prefix_max = u64::from(mask);
    let mut bytes = [0_u8; MAX_ENCODED_LEN];

    if value < prefix_max {
        bytes[0] = high_bits | u8::try_from(value).ok()?;
        return Some(EncodedPrefixedInt { bytes, len: 1 });
    }

    bytes[0] = high_bits | mask;
    let mut remaining = value - prefix_max;
    let mut index = 1_usize;

    while remaining >= 128 {
        if index >= MAX_ENCODED_LEN {
            return None;
        }
        bytes[index] = (u8::try_from(remaining & 0x7f).ok()?) | 0x80;
        remaining >>= 7;
        index += 1;
    }

    if index >= MAX_ENCODED_LEN {
        return None;
    }
    bytes[index] = u8::try_from(remaining).ok()?;

    Some(EncodedPrefixedInt {
        bytes,
        len: u8::try_from(index + 1).ok()?,
    })
}

/// Decodes one prefixed integer, returning its value and consumed byte count.
///
/// Instruction/high bits in the first byte are ignored. Arithmetic overflow,
/// truncation, and invalid prefix widths are rejected.
pub fn decode_prefixed(input: &[u8], prefix_bits: u8) -> Option<(u64, usize)> {
    let mask = prefix_mask(prefix_bits)?;
    let first = *input.first()?;
    let prefix_max = u64::from(mask);
    let prefix_value = u64::from(first & mask);

    if prefix_value < prefix_max {
        return Some((prefix_value, 1));
    }

    let mut value = prefix_max;
    let mut shift = 0_u32;

    for index in 1..MAX_ENCODED_LEN {
        let byte = *input.get(index)?;
        let chunk = u64::from(byte & 0x7f);

        if shift >= 64 || chunk > (u64::MAX >> shift) {
            return None;
        }
        value = value.checked_add(chunk << shift)?;

        if byte & 0x80 == 0 {
            return Some((value, index + 1));
        }

        shift = shift.checked_add(7)?;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc_style_examples() {
        assert_eq!(
            encode_prefixed(5, 0x20, 10).unwrap().as_slice(),
            &[0x2a]
        );
        assert_eq!(
            encode_prefixed(5, 0, 31).unwrap().as_slice(),
            &[31, 0]
        );
        assert_eq!(
            encode_prefixed(5, 0, 1337).unwrap().as_slice(),
            &[31, 154, 10]
        );
    }

    #[test]
    fn roundtrips_boundaries_and_u64_max() {
        for value in [0, 30, 31, 32, 127, 128, u32::MAX as u64, u64::MAX] {
            for prefix in 1..=8 {
                let encoded = encode_prefixed(prefix, 0, value).unwrap();
                assert_eq!(
                    decode_prefixed(encoded.as_slice(), prefix),
                    Some((value, encoded.len()))
                );
            }
        }
    }

    #[test]
    fn rejects_invalid_prefix_and_overlapping_high_bits() {
        assert!(encode_prefixed(0, 0, 1).is_none());
        assert!(encode_prefixed(9, 0, 1).is_none());
        assert!(encode_prefixed(5, 0x01, 1).is_none());
        assert_eq!(decode_prefixed(&[0], 0), None);
    }

    #[test]
    fn rejects_truncated_continuation() {
        assert_eq!(decode_prefixed(&[31, 0x80], 5), None);
    }

    #[test]
    fn rejects_overflowing_continuation() {
        let mut encoded = [0xff_u8; MAX_ENCODED_LEN];
        encoded[MAX_ENCODED_LEN - 1] = 0x7f;
        assert_eq!(decode_prefixed(&encoded, 8), None);
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    #[kani::unwind(12)]
    fn prefix_five_roundtrip_u16() {
        let value: u16 = kani::any();
        let high: u8 = kani::any();
        kani::assume(high & 0x1f == 0);

        let encoded = encode_prefixed(5, high, u64::from(value)).unwrap();
        let decoded = decode_prefixed(encoded.as_slice(), 5).unwrap();

        assert_eq!(decoded.0, u64::from(value));
        assert_eq!(decoded.1, encoded.len());
    }

    #[kani::proof]
    fn below_prefix_max_is_one_byte() {
        let value: u8 = kani::any();
        kani::assume(value < 31);

        let encoded = encode_prefixed(5, 0x20, u64::from(value)).unwrap();
        assert_eq!(encoded.len(), 1);
        assert_eq!(encoded.as_slice()[0], 0x20 | value);
    }

    #[kani::proof]
    fn overlapping_instruction_bits_are_rejected() {
        let value: u64 = kani::any();
        assert!(encode_prefixed(5, 0x01, value).is_none());
    }
}
