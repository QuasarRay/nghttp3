//! QUIC variable-length integer encoding.
//!
//! The wire format is defined by RFC 9000 section 16. The two most significant
//! bits of the first byte select a total width of 1, 2, 4, or 8 bytes; the
//! remaining bits encode an unsigned integer in network byte order.

/// Maximum value representable by a QUIC variable-length integer.
pub const MAX: u64 = (1_u64 << 62) - 1;

/// An encoded QUIC variable-length integer without heap allocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Encoded {
    bytes: [u8; 8],
    len: u8,
}

impl Encoded {
    /// Returns exactly the bytes belonging to this encoded value.
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..usize::from(self.len)]
    }

    /// Returns the encoded width in bytes.
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    /// QUIC variable integers are never empty.
    pub const fn is_empty(&self) -> bool {
        false
    }
}

/// Returns the canonical encoded width for value, or None when it is
/// outside the QUIC variable-integer domain.
pub const fn encoded_len(value: u64) -> Option<usize> {
    if value < (1_u64 << 6) {
        Some(1)
    } else if value < (1_u64 << 14) {
        Some(2)
    } else if value < (1_u64 << 30) {
        Some(4)
    } else if value <= MAX {
        Some(8)
    } else {
        None
    }
}

/// Encodes value using QUIC's canonical shortest representation.
pub fn encode(value: u64) -> Option<Encoded> {
    let len = encoded_len(value)?;
    let mut bytes = [0_u8; 8];

    match len {
        1 => {
            bytes[0] = value as u8;
        }
        2 => {
            let raw = (value as u16).to_be_bytes();
            bytes[0] = raw[0] | 0x40;
            bytes[1] = raw[1];
        }
        4 => {
            let raw = (value as u32).to_be_bytes();
            bytes[0] = raw[0] | 0x80;
            bytes[1] = raw[1];
            bytes[2] = raw[2];
            bytes[3] = raw[3];
        }
        8 => {
            let raw = value.to_be_bytes();
            bytes[0] = raw[0] | 0xc0;
            bytes[1] = raw[1];
            bytes[2] = raw[2];
            bytes[3] = raw[3];
            bytes[4] = raw[4];
            bytes[5] = raw[5];
            bytes[6] = raw[6];
            bytes[7] = raw[7];
        }
        _ => unreachable!("encoded_len only returns QUIC widths"),
    }

    Some(Encoded {
        bytes,
        len: len as u8,
    })
}

/// Decodes one QUIC variable-length integer.
///
/// Returns None if the input does not contain the complete integer selected
/// by its first-byte prefix. Extra bytes after the integer are ignored.
pub fn decode(input: &[u8]) -> Option<(u64, usize)> {
    let first = *input.first()?;
    let len = 1_usize << (first >> 6);

    if input.len() < len {
        return None;
    }

    let mut value = u64::from(first & 0x3f);
    let mut i = 1;
    while i < len {
        value = (value << 8) | u64::from(input[i]);
        i += 1;
    }

    Some((value, len))
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOUNDARIES: [u64; 12] = [
        0,
        1,
        63,
        64,
        16_383,
        16_384,
        1_073_741_823,
        1_073_741_824,
        MAX - 1,
        MAX,
        4_294_967_295,
        0x0123_4567_89ab_cdef,
    ];

    #[test]
    fn boundary_roundtrips() {
        for value in BOUNDARIES {
            let encoded = encode(value).unwrap();
            let (decoded, consumed) = decode(encoded.as_slice()).unwrap();
            assert_eq!(decoded, value);
            assert_eq!(consumed, encoded.len());
        }
    }

    #[test]
    fn rejects_values_outside_quic_domain() {
        assert_eq!(encoded_len(MAX + 1), None);
        assert_eq!(encode(MAX + 1), None);
        assert_eq!(encoded_len(u64::MAX), None);
    }

    #[test]
    fn rejects_truncated_encodings() {
        for value in [64, 16_384, 1_073_741_824] {
            let encoded = encode(value).unwrap();
            for prefix_len in 0..encoded.len() {
                assert_eq!(decode(&encoded.as_slice()[..prefix_len]), None);
            }
        }
    }

    #[test]
    fn differential_against_c_oracle() {
        for value in (0_u64..=65_535).chain(BOUNDARIES) {
            let ours = encode(value).unwrap();
            let c = nghttp3::encode_uvarint(value).unwrap();
            assert_eq!(ours.as_slice(), c.as_slice(), "value {value}");

            let ours_decoded = decode(&c).unwrap();
            let c_decoded = nghttp3::decode_uvarint(ours.as_slice()).unwrap();
            assert_eq!(ours_decoded, c_decoded, "value {value}");
        }
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn roundtrip_every_valid_value() {
        let value: u64 = kani::any();
        kani::assume(value <= MAX);

        let encoded = encode(value).expect("all valid values encode");
        let (decoded, consumed) = decode(encoded.as_slice()).expect("self-encoding is complete");

        assert_eq!(decoded, value);
        assert_eq!(consumed, encoded.len());
    }

    #[kani::proof]
    fn canonical_width_matches_prefix() {
        let value: u64 = kani::any();
        kani::assume(value <= MAX);

        let encoded = encode(value).unwrap();
        let prefix_width = 1_usize << (encoded.as_slice()[0] >> 6);

        assert_eq!(prefix_width, encoded.len());
        assert_eq!(encoded_len(value), Some(encoded.len()));
    }

    #[kani::proof]
    fn truncated_input_is_rejected() {
        let bytes: [u8; 8] = kani::any();
        let available: u8 = kani::any();
        kani::assume(available <= 8);

        let available = usize::from(available);
        let required = 1_usize << (bytes[0] >> 6);
        if available < required {
            assert_eq!(decode(&bytes[..available]), None);
        }
    }
}
