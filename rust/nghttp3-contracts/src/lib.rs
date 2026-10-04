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
use nghttp3_core::{priority, qpack_buffer, qpack_read_state, qpack_reference, ringbuf, settings, varint};

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


/// Verification model for RFC 9218 Priority field parsing.
#[derive(lambars_verification::VerificationModel)]
pub struct PriorityContract;

/// Canonical urgency encodings round-trip through the safe parser.
#[verification_case(id = "RFC9218.priority.urgency")]
pub fn priority_urgency_roundtrip(urgency: u8) -> bool {
    if urgency > priority::URGENCY_LOW {
        return false;
    }

    let input = [b'u', b'=', b'0' + urgency];
    pipe!(
        priority::parse(&input, priority::Priority::default()),
        |parsed: Result<priority::Priority, priority::ParseError>| {
            parsed.is_ok_and(|value| value.urgency == urgency)
        }
    )
}

boundary_cases!(
    priority_urgency_roundtrip;
    priority_urgency_high = priority::URGENCY_HIGH,
    priority_urgency_default = priority::DEFAULT_URGENCY,
    priority_urgency_low = priority::URGENCY_LOW,
);

dual_verify!(
    priority_rejects_historical_trailing_equals,
    "history.aed3107.priority-trailing-equals",
    { priority::parse(b"u=", priority::Priority::default()).is_err() }
);

dual_verify!(
    priority_rejects_historical_parameter_trailing_equals,
    "history.aed3107.parameter-trailing-equals",
    {
        priority::parse(b"x=?1;foo=", priority::Priority::default()).is_err()
    }
);

dual_verify!(
    priority_incremental_bare_true,
    "RFC9218.priority.incremental-bare-true",
    {
        priority::parse(b"i", priority::Priority::default())
            .is_ok_and(|value| value.incremental)
    }
);

#[test]
fn priority_verification_model_is_registered() {
    assert_eq!(
        <PriorityContract as VerificationModel>::TYPE_NAME,
        "PriorityContract"
    );
}


/// Verification model for checked QPACK buffer-growth arithmetic.
#[derive(lambars_verification::VerificationModel)]
pub struct QpackBufferContract;

/// Every accepted pre-rounding size remains within the reference ceiling.
#[verification_case(id = "history.8a8d45c.qpack-buffer-rounding")]
pub fn qpack_growth_boundary(required: usize) -> bool {
    qpack_buffer::rounded_capacity(required).is_ok_and(|rounded| {
        rounded >= required.max(qpack_buffer::MIN_CAPACITY)
            && rounded <= qpack_buffer::MAX_CAPACITY
            && rounded.is_power_of_two()
    })
}

boundary_cases!(
    qpack_growth_boundary;
    qpack_minimum = qpack_buffer::MIN_CAPACITY,
    qpack_just_over_minimum = qpack_buffer::MIN_CAPACITY + 1,
    qpack_max_minus_one = qpack_buffer::MAX_CAPACITY - 1,
    qpack_maximum = qpack_buffer::MAX_CAPACITY,
);

dual_verify!(
    qpack_rejects_historical_rounding_overflow,
    "history.8a8d45c.qpack-buffer-max-plus-one",
    {
        qpack_buffer::rounded_capacity(qpack_buffer::MAX_CAPACITY + 1)
            == Err(qpack_buffer::GrowthError::TooLarge)
    }
);

dual_verify!(
    qpack_rejects_platform_addition_overflow,
    "nghttp3.qpack-buffer.checked-addition",
    {
        qpack_buffer::reserve_capacity(usize::MAX, 0, 1)
            == Err(qpack_buffer::GrowthError::ArithmeticOverflow)
    }
);

#[test]
fn qpack_buffer_verification_model_is_registered() {
    assert_eq!(
        <QpackBufferContract as VerificationModel>::TYPE_NAME,
        "QpackBufferContract"
    );
}


/// Verification model for QPACK decoder temporary ownership.
#[derive(lambars_verification::VerificationModel)]
pub struct QpackReadStateContract;

/// Replays the stale-owner failure path fixed by historical commit ecfae7ac.
#[verification_case(id = "history.ecfae7a.qpack-decoder-stale-owner")]
pub fn qpack_decoder_oom_stale_owner_guard() -> bool {
    let mut state = qpack_read_state::DecoderReadState::new();
    state.set_name(b"name".to_vec());
    state.set_value(b"value".to_vec());

    pipe!(
        state.take_literal(),
        |pending: Result<qpack_read_state::PendingLiteral, qpack_read_state::Missing>| {
            pending.is_ok()
        }
    ) && !state.has_name()
        && !state.has_value()
}

/// Incomplete literal extraction must not partially consume an existing owner.
#[verification_case(id = "nghttp3.qpack-read-state.atomic-literal-take")]
pub fn qpack_literal_take_is_atomic() -> bool {
    let mut state = qpack_read_state::DecoderReadState::new();
    state.set_name(b"name".to_vec());

    state.take_literal() == Err(qpack_read_state::Missing::Value)
        && state.name() == Some(&b"name"[..])
        && !state.has_value()
}

dual_verify!(
    qpack_decoder_oom_stale_owner_history_regression,
    "history.ecfae7a.qpack-decoder-stale-owner",
    { qpack_decoder_oom_stale_owner_guard() }
);

dual_verify!(
    qpack_literal_take_atomicity,
    "nghttp3.qpack-read-state.atomic-literal-take",
    { qpack_literal_take_is_atomic() }
);

dual_verify!(
    qpack_reset_after_failed_publish_is_idempotent,
    "history.ecfae7a.reset-after-failed-publish",
    {
        let mut state = qpack_read_state::DecoderReadState::new();
        state.set_value(b"value".to_vec());
        let pending = state.take_value().unwrap();
        drop(pending);
        state.reset();
        state.reset();
        !state.has_value()
    }
);

#[test]
fn qpack_read_state_verification_model_is_registered() {
    assert_eq!(
        <QpackReadStateContract as VerificationModel>::TYPE_NAME,
        "QpackReadStateContract"
    );
}


/// Verification model for transactional QPACK reference publication.
#[derive(lambars_verification::VerificationModel)]
pub struct QpackReferenceContract;

/// Historical registration failure must not publish an owner into the stream.
#[verification_case(id = "history.62743057.qpack-register-before-publish")]
pub fn qpack_registration_failure_keeps_stream_empty() -> bool {
    let pending = qpack_reference::PendingReference::new(7_u8);
    let stream = qpack_reference::StreamReferences::<u8, ()>::new();

    pipe!(
        pending.register(|_| Err::<(), _>(0_u8)),
        |result: Result<
            qpack_reference::RegisteredReference<u8, ()>,
            qpack_reference::RegistrationFailure<u8, u8>,
        >| result.is_err()
    ) && stream.is_empty()
}

/// A successfully registered owner can be published exactly once by move.
#[verification_case(id = "nghttp3.qpack-reference.registered-before-published")]
pub fn qpack_registered_reference_can_publish() -> bool {
    let registered = qpack_reference::PendingReference::new(7_u8)
        .register(|_| Ok::<_, ()>(11_u8))
        .unwrap();
    let mut stream = qpack_reference::StreamReferences::new();
    stream.publish(registered);

    stream.len() == 1
        && stream.front().is_some_and(|entry| {
            entry.value() == &7 && entry.registration() == &11
        })
}

dual_verify!(
    qpack_double_free_history_registration_failure,
    "history.62743057.qpack-register-before-publish",
    { qpack_registration_failure_keeps_stream_empty() }
);

dual_verify!(
    qpack_registered_before_published_typestate,
    "nghttp3.qpack-reference.registered-before-published",
    { qpack_registered_reference_can_publish() }
);

#[test]
fn qpack_reference_verification_model_is_registered() {
    assert_eq!(
        <QpackReferenceContract as VerificationModel>::TYPE_NAME,
        "QpackReferenceContract"
    );
}
