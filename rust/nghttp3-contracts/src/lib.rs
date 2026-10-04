//! Executable protocol contracts generated with Lambars metaprogramming.
//!
//! This crate is intentionally separate from `nghttp3-core`: the production
//! protocol core stays small and verifier-friendly while this layer compresses
//! repeated runtime/Kani contract boilerplate.

#![forbid(unsafe_code)]

use lambars::pipe;
use lambars_verification::{boundary_cases, dual_verify, verification_case};
#[cfg(test)]
use lambars_verification::VerificationModel;
use nghttp3_core::{settings, varint};

/// Verification model for RFC 9000 Section 16 variable-length integers.
#[derive(lambars_verification::VerificationModel)]
pub struct VarintContract;

/// One semantic round-trip predicate shared by generated runtime and Kani cases.
#[verification_case(id = "RFC9000.section16.varint-roundtrip")]
pub fn varint_roundtrip(value: u64) -> bool {
    if value > varint::MAX {
        return false;
    }

    pipe!(value, varint::encode, |encoded: Option<varint::Encoded>| {
        encoded
            .and_then(|bytes| {
                varint::decode(bytes.as_slice())
                    .map(|(decoded, consumed)| (bytes, decoded, consumed))
            })
            .is_some_and(|(bytes, decoded, consumed)| decoded == value && consumed == bytes.len())
    })
}

/// Historical stream-length overflow invariant from nghttp3 commit 07e84d61.
#[verification_case(id = "history.07e84d61.stream-data-overflow")]
pub fn historical_stream_length_guard() -> bool {
    pipe!(&[varint::MAX][..], varint::checked_sum_lengths) == Some(varint::MAX)
        && pipe!(&[varint::MAX, 1][..], varint::checked_sum_lengths).is_none()
        && pipe!(&[varint::MAX - 1, 1][..], varint::checked_sum_lengths) == Some(varint::MAX)
        && pipe!(&[varint::MAX - 1, 2][..], varint::checked_sum_lengths).is_none()
}

boundary_cases!(
    varint_roundtrip;
    zero = 0_u64,
    six_bit_max = 63_u64,
    fourteen_bit_min = 64_u64,
    fourteen_bit_max = 16_383_u64,
    thirty_bit_min = 16_384_u64,
    thirty_bit_max = 1_073_741_823_u64,
    sixty_two_bit_min = 1_073_741_824_u64,
    sixty_two_bit_max = varint::MAX,
);

dual_verify!(
    rejects_value_above_varint_domain,
    "RFC9000.section16.max-plus-one",
    { varint::encode(varint::MAX + 1).is_none() }
);

dual_verify!(
    stream_data_overflow_history_regression,
    "history.07e84d61.stream-data-overflow",
    { historical_stream_length_guard() }
);

#[test]
fn lambars_verification_model_is_registered() {
    assert_eq!(
        <VarintContract as VerificationModel>::TYPE_NAME,
        "VarintContract"
    );
}


/// Verification model for versioned HTTP/3 settings semantics.
#[derive(lambars_verification::VerificationModel)]
pub struct SettingsContract;

/// Current implementation defaults stay inside every wire-encoded domain.
#[verification_case(id = "settings.current.defaults")]
pub fn settings_defaults_are_consistent() -> bool {
    pipe!(settings::Settings::default(), |value: settings::Settings| {
        value.max_field_section_size <= varint::MAX
            && value.qpack_max_table_capacity <= varint::MAX
            && value.qpack_encoder_max_table_capacity <= varint::MAX
            && value.qpack_blocked_streams <= varint::MAX
            && value.qpack_indexing_strategy == settings::IndexingStrategy::None
    })
}

/// The safe field-size setter accepts exactly QUIC variable-integer values.
#[verification_case(id = "RFC9114.settings.max-field-section-size.domain")]
pub fn max_field_section_size_domain(value: u64) -> bool {
    settings::Settings::default()
        .with_max_field_section_size(value)
        .is_some()
        == (value <= varint::MAX)
}

boundary_cases!(
    max_field_section_size_domain;
    settings_field_zero = 0_u64,
    settings_field_six_bit_max = 63_u64,
    settings_field_fourteen_bit_max = 16_383_u64,
    settings_field_thirty_bit_max = 1_073_741_823_u64,
    settings_field_sixty_two_bit_max = varint::MAX,
);

dual_verify!(
    settings_reject_max_field_section_size_overflow,
    "RFC9114.settings.max-field-section-size.max-plus-one",
    {
        settings::Settings::default()
            .with_max_field_section_size(varint::MAX + 1)
            .is_none()
    }
);

dual_verify!(
    settings_v3_upgrade_defaults_v4_indexing,
    "nghttp3.settings.v3-to-v4.indexing-default",
    {
        settings::Settings::from(settings::SettingsV3::default()).qpack_indexing_strategy
            == settings::IndexingStrategy::None
    }
);

#[test]
fn settings_verification_model_is_registered() {
    assert_eq!(
        <SettingsContract as VerificationModel>::TYPE_NAME,
        "SettingsContract"
    );
}
