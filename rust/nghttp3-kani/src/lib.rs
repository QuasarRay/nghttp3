//! Kani and runtime regressions generated from shared semantic predicates.

#![deny(unsafe_code)]

use lambars_verification::{boundary_cases, verification_case};
use nghttp3_spec::{MAX_VARINT, VarIntError, encode_varint, varint_len, varint_roundtrips};

#[verification_case(id = "rfc9000.varint.roundtrip")]
fn boundary_roundtrip(value: u64) -> bool {
    varint_roundtrips(value)
}

boundary_cases!(
    boundary_roundtrip;
    varint_zero = 0_u64,
    varint_one_octet_max = 63_u64,
    varint_two_octet_min = 64_u64,
    varint_two_octet_max = 16_383_u64,
    varint_four_octet_min = 16_384_u64,
    varint_four_octet_max = 1_073_741_823_u64,
    varint_eight_octet_min = 1_073_741_824_u64,
    varint_max = MAX_VARINT,
);

#[cfg(kani)]
#[kani::proof]
fn kani_varint_roundtrip_all_values() {
    let value: u64 = kani::any();
    kani::assume(value <= MAX_VARINT);
    assert!(varint_roundtrips(value));
}

#[cfg(kani)]
#[kani::proof]
fn kani_varint_rejects_values_above_domain() {
    let value: u64 = kani::any();
    kani::assume(value > MAX_VARINT);
    assert_eq!(varint_len(value), Err(VarIntError::TooLarge));
    assert_eq!(encode_varint(value), Err(VarIntError::TooLarge));
}
