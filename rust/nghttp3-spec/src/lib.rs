//! Runtime-free executable protocol semantics.
//!
//! This crate is intentionally independent from any networking executor. It
//! contains pure, safe representations that can be consumed by ordinary tests,
//! Kani harnesses, and later Verus refinement proofs.

#![deny(unsafe_code)]

use lambars::pipe;

/// Largest value representable by the QUIC variable-length integer encoding
/// defined by RFC 9000 §16.
pub const MAX_VARINT: u64 = (1_u64 << 62) - 1;

/// Errors produced by the executable QUIC varint model.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VarIntError {
    /// The value exceeds QUIC's 62-bit variable-integer domain.
    TooLarge,
    /// The input does not contain all octets selected by its prefix.
    Truncated,
}

/// Stack-only encoded QUIC variable integer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EncodedVarInt {
    bytes: [u8; 8],
    len: u8,
}

impl EncodedVarInt {
    /// Encoded bytes without heap allocation.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..usize::from(self.len)]
    }

    /// Encoded wire length.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    /// QUIC varints are never empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

/// Return the unique QUIC wire width for a valid value.
pub const fn varint_len(value: u64) -> Result<usize, VarIntError> {
    match value {
        0..=63 => Ok(1),
        64..=16_383 => Ok(2),
        16_384..=1_073_741_823 => Ok(4),
        1_073_741_824..=MAX_VARINT => Ok(8),
        _ => Err(VarIntError::TooLarge),
    }
}

const fn encoded_len_from_first(first: u8) -> usize {
    1_usize << (first >> 6)
}

/// Encode one QUIC variable-length integer.
pub fn encode_varint(value: u64) -> Result<EncodedVarInt, VarIntError> {
    let len = varint_len(value)?;
    let mut bytes = [0_u8; 8];

    match len {
        1 => {
            bytes[0] = value as u8;
        }
        2 => {
            let wire = (value as u16) | 0x4000;
            bytes[..2].copy_from_slice(&wire.to_be_bytes());
        }
        4 => {
            let wire = (value as u32) | 0x8000_0000;
            bytes[..4].copy_from_slice(&wire.to_be_bytes());
        }
        8 => {
            let wire = value | 0xC000_0000_0000_0000;
            bytes.copy_from_slice(&wire.to_be_bytes());
        }
        _ => unreachable!("varint_len only returns QUIC wire widths"),
    }

    Ok(EncodedVarInt {
        bytes,
        len: len as u8,
    })
}

/// Decode one complete QUIC variable-length integer.
///
/// Returns the value plus the number of consumed octets. Trailing input is not
/// consumed.
pub fn decode_varint(input: &[u8]) -> Result<(u64, usize), VarIntError> {
    let first = *input.first().ok_or(VarIntError::Truncated)?;
    let len = pipe!(first, encoded_len_from_first);

    if input.len() < len {
        return Err(VarIntError::Truncated);
    }

    let mut value = u64::from(first & 0x3f);
    for octet in &input[1..len] {
        value = (value << 8) | u64::from(*octet);
    }

    debug_assert!(value <= MAX_VARINT);
    Ok((value, len))
}

/// Shared semantic predicate used by runtime, Kani, and later Verus adapters.
#[must_use]
pub fn varint_roundtrips(value: u64) -> bool {
    let Ok(expected_len) = varint_len(value) else {
        return false;
    };
    let Ok(encoded) = encode_varint(value) else {
        return false;
    };
    let Ok((decoded, consumed)) = decode_varint(encoded.as_bytes()) else {
        return false;
    };

    decoded == value && consumed == expected_len && encoded.len() == expected_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_boundaries_roundtrip() {
        for value in [
            0,
            63,
            64,
            16_383,
            16_384,
            1_073_741_823,
            1_073_741_824,
            MAX_VARINT,
        ] {
            assert!(varint_roundtrips(value), "boundary {value} failed");
        }
    }

    #[test]
    fn too_large_is_rejected() {
        assert_eq!(encode_varint(MAX_VARINT + 1), Err(VarIntError::TooLarge));
        assert_eq!(varint_len(u64::MAX), Err(VarIntError::TooLarge));
    }

    #[test]
    fn truncation_is_rejected_for_every_width() {
        for value in [64, 16_384, 1_073_741_824] {
            let encoded = encode_varint(value).unwrap();
            assert_eq!(
                decode_varint(&encoded.as_bytes()[..encoded.len() - 1]),
                Err(VarIntError::Truncated)
            );
        }
    }
}
