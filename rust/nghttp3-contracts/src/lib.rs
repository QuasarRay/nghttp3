//! Executable protocol contracts generated with Lambars metaprogramming.
//!
//! This crate is intentionally separate from `nghttp3-core`: the production
//! protocol core stays small and verifier-friendly while this layer compresses
//! repeated runtime/Kani contract boilerplate.

#![forbid(unsafe_code)]

use lambars::pipe;
#[cfg(test)]
use lambars_verification::VerificationModel;
use lambars_verification::{boundary_cases, dual_verify, verification_case};
use nghttp3_core::{owned_input, priority_update, qpack, qpack_decoder, qpack_stream, ringbuf, settings, structured, varint};

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
    pipe!(
        settings::Settings::default(),
        |value: settings::Settings| {
            value.max_field_section_size <= varint::MAX
                && value.qpack_max_table_capacity <= varint::MAX
                && value.qpack_encoder_max_table_capacity <= varint::MAX
                && value.qpack_blocked_streams <= varint::MAX
                && value.qpack_indexing_strategy == settings::IndexingStrategy::None
        }
    )
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

/// Verification model for the safe internal ring buffer.
#[derive(lambars_verification::VerificationModel)]
pub struct RingBufferContract;

/// Replays the wrapped-growth state that historically corrupted C storage.
#[verification_case(id = "history.97cb58e.ringbuf-wrapped-reserve")]
pub fn historical_ringbuf_growth_guard() -> bool {
    let mut rb = ringbuf::RingBuffer::with_capacity(4).unwrap();
    for value in [1_u8, 2, 3, 4] {
        rb.push_back(value);
    }
    rb.pop_front();
    rb.pop_front();
    rb.push_back(5);
    rb.push_back(6);

    rb.reserve(8)
        && pipe!(
            rb.iter().copied().collect::<Vec<_>>(),
            |values: Vec<u8>| values == [3, 4, 5, 6]
        )
        && rb.capacity() == 8
}

dual_verify!(
    ringbuf_wrapped_reserve_history_regression,
    "history.97cb58e.ringbuf-wrapped-reserve",
    { historical_ringbuf_growth_guard() }
);

dual_verify!(
    ringbuf_rejects_non_power_of_two_capacity,
    "nghttp3.ringbuf.capacity.power-of-two",
    { ringbuf::RingBuffer::<u8>::with_capacity(3).is_none() }
);

#[test]
fn ringbuf_verification_model_is_registered() {
    assert_eq!(
        <RingBufferContract as VerificationModel>::TYPE_NAME,
        "RingBufferContract"
    );
}

/// Verification model for safe RFC 9218 / Structured Fields parsing.
#[derive(lambars_verification::VerificationModel)]
pub struct PriorityParserContract;

#[verification_case(id = "history.aed3107.priority-trailing-equals")]
pub fn historical_priority_trailing_equals_guard() -> bool {
    structured::parse_priority(b"u=", structured::Priority::default())
        == Err(structured::ParseError::TrailingEquals)
}

#[verification_case(id = "history.aed3107.parameter-trailing-equals")]
pub fn historical_parameter_trailing_equals_guard() -> bool {
    structured::parse_item_with_params(b"?1;foo=") == Err(structured::ParseError::TrailingEquals)
}

#[verification_case(id = "RFC9218.urgency.single-digit-domain")]
pub fn priority_single_digit_domain(digit: u8) -> bool {
    let Some(byte) = b'0'.checked_add(digit) else {
        return digit > structured::URGENCY_LOW;
    };
    let input = [b'u', b'=', byte];
    structured::parse_priority(&input, structured::Priority::default()).is_ok()
        == (digit <= structured::URGENCY_LOW)
}

boundary_cases!(
    priority_single_digit_domain;
    urgency_high = 0_u8,
    urgency_default = structured::DEFAULT_URGENCY,
    urgency_low = structured::URGENCY_LOW,
    urgency_first_invalid = 8_u8,
    urgency_digit_invalid = 9_u8,
);

dual_verify!(
    priority_trailing_equals_history_regression,
    "history.aed3107.priority-trailing-equals",
    { historical_priority_trailing_equals_guard() }
);

dual_verify!(
    parameter_trailing_equals_history_regression,
    "history.aed3107.parameter-trailing-equals",
    { historical_parameter_trailing_equals_guard() }
);

#[test]
fn priority_parser_verification_model_is_registered() {
    assert_eq!(
        <PriorityParserContract as VerificationModel>::TYPE_NAME,
        "PriorityParserContract"
    );
}

/// Verification model for checked QPACK buffer-growth arithmetic.
#[derive(lambars_verification::VerificationModel)]
pub struct QpackGrowthContract;

#[verification_case(id = "history.8a8d45c.qpack-growth-bound")]
pub fn qpack_growth_boundary(extra: usize) -> bool {
    qpack::reserve_capacity(0, 0, extra).is_some() == (extra <= qpack::MAX_BUFFER_CAPACITY)
}

boundary_cases!(
    qpack_growth_boundary;
    qpack_zero = 0_usize,
    qpack_minimum = qpack::MIN_BUFFER_CAPACITY,
    qpack_maximum = qpack::MAX_BUFFER_CAPACITY,
    qpack_first_rejected = qpack::MAX_BUFFER_CAPACITY + 1,
);

dual_verify!(
    qpack_growth_historical_upper_bound,
    "history.8a8d45c.qpack-growth-bound",
    {
        qpack::reserve_capacity(0, 0, qpack::MAX_BUFFER_CAPACITY)
            == Some(qpack::MAX_BUFFER_CAPACITY)
            && qpack::reserve_capacity(0, 0, qpack::MAX_BUFFER_CAPACITY + 1).is_none()
    }
);

dual_verify!(
    qpack_growth_checked_addition,
    "nghttp3.qpack.growth.checked-addition",
    { qpack::reserve_capacity(usize::MAX, 0, 1).is_none() }
);

#[test]
fn qpack_growth_verification_model_is_registered() {
    assert_eq!(
        <QpackGrowthContract as VerificationModel>::TYPE_NAME,
        "QpackGrowthContract"
    );
}

/// Verification model for QPACK decoder transient ownership.
#[derive(lambars_verification::VerificationModel)]
pub struct QpackDecoderOwnershipContract;

#[verification_case(id = "history.ecfae7.indexed-insert-clears-value")]
pub fn qpack_indexed_insert_failure_clears_value() -> bool {
    let mut state = qpack_decoder::DecoderFieldState::with_value(1_u8);
    let result = state.finish_indexed_insert(|_| Err::<(), _>(()));
    result == Err(qpack_decoder::FinishError::Insert(())) && !state.has_value()
}

#[verification_case(id = "history.ecfae7.literal-insert-clears-fields")]
pub fn qpack_literal_insert_failure_clears_fields() -> bool {
    let mut state = qpack_decoder::DecoderFieldState::with_fields(1_u8, 2_u8);
    let result = state.finish_literal_insert(|_, _| Err::<(), _>(()));
    result == Err(qpack_decoder::FinishError::Insert(())) && state.is_clear()
}

dual_verify!(
    qpack_indexed_oom_history_regression,
    "history.ecfae7.indexed-insert-clears-value",
    { qpack_indexed_insert_failure_clears_value() }
);

dual_verify!(
    qpack_literal_oom_history_regression,
    "history.ecfae7.literal-insert-clears-fields",
    { qpack_literal_insert_failure_clears_fields() }
);

#[test]
fn qpack_decoder_ownership_model_is_registered() {
    assert_eq!(
        <QpackDecoderOwnershipContract as VerificationModel>::TYPE_NAME,
        "QpackDecoderOwnershipContract"
    );
}


/// Verification model for QPACK registration-before-publication ordering.
#[derive(lambars_verification::VerificationModel)]
pub struct QpackPublicationContract;

#[verification_case(id = "history.62743057.registration-before-publication")]
pub fn qpack_failed_registration_is_not_published() -> bool {
    let mut refs = qpack_stream::PublishedRefs::new();
    let result = refs.register_then_publish(1_u8, |_| Err::<(), _>(()));
    result == Err(()) && refs.is_empty()
}

dual_verify!(
    qpack_double_free_history_regression,
    "history.62743057.registration-before-publication",
    { qpack_failed_registration_is_not_published() }
);

#[test]
fn qpack_publication_model_is_registered() {
    assert_eq!(
        <QpackPublicationContract as VerificationModel>::TYPE_NAME,
        "QpackPublicationContract"
    );
}


/// Verification model for owned PRIORITY_UPDATE failure paths.
#[derive(lambars_verification::VerificationModel)]
pub struct PriorityUpdateOwnershipContract;

#[verification_case(id = "history.9bf7d876.priority-update-failure-ownership")]
pub fn priority_update_failure_returns_owner() -> bool {
    let pending = priority_update::PendingPriorityUpdate::new(4, b"u=1".to_vec());
    let error = pending.try_queue(|| Err::<(), _>(())).unwrap_err();

    error.pending.stream_id() == 4 && error.pending.data() == b"u=1"
}

dual_verify!(
    priority_update_leak_history_regression,
    "history.9bf7d876.priority-update-failure-ownership",
    { priority_update_failure_returns_owner() }
);

#[test]
fn priority_update_ownership_model_is_registered() {
    assert_eq!(
        <PriorityUpdateOwnershipContract as VerificationModel>::TYPE_NAME,
        "PriorityUpdateOwnershipContract"
    );
}


/// Verification model for retained QPACK input ownership.
#[derive(lambars_verification::VerificationModel)]
pub struct OwnedQpackInputContract;

#[verification_case(id = "history.fce4985.retained-input-owned")]
pub fn retained_qpack_input_owns_bytes() -> bool {
    let input = owned_input::OwnedInput::new(vec![1_u8, 2, 3]);
    input.original_len() == 3 && input.remaining() == [1, 2, 3]
}

#[verification_case(id = "history.fce4985.checked-subspan")]
pub fn retained_qpack_overconsume_preserves_state() -> bool {
    let mut input = owned_input::OwnedInput::new(vec![1_u8]);
    !input.consume(2) && input.consumed() == 0 && input.remaining() == [1]
}

dual_verify!(
    retained_qpack_input_history_regression,
    "history.fce4985.retained-input-owned",
    { retained_qpack_input_owns_bytes() }
);

dual_verify!(
    retained_qpack_subspan_boundary,
    "history.fce4985.checked-subspan",
    { retained_qpack_overconsume_preserves_state() }
);

#[test]
fn owned_qpack_input_model_is_registered() {
    assert_eq!(
        <OwnedQpackInputContract as VerificationModel>::TYPE_NAME,
        "OwnedQpackInputContract"
    );
}
