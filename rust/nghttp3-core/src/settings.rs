//! Safe, runtime-independent HTTP/3 settings semantics.
//!
//! The C ABI versions are layout prefixes. This module models their semantics
//! with owned Rust data instead: no raw pointer or partial-struct access crosses
//! into the verified core.

use crate::varint;

/// nghttp3's default local QPACK encoder table-capacity ceiling.
pub const DEFAULT_QPACK_ENCODER_MAX_TABLE_CAPACITY: u64 = 4096;

/// nghttp3's default local glitch-rate limiter burst.
pub const DEFAULT_GLITCH_RATE_LIMIT_BURST: u64 = 1000;

/// nghttp3's default local glitch-rate limiter replenishment rate.
pub const DEFAULT_GLITCH_RATE_LIMIT_RATE: u64 = 33;

/// QPACK dynamic-table indexing policy introduced by nghttp3 settings V4.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum IndexingStrategy {
    /// Do not eagerly index fields without a known static token.
    #[default]
    None,
    /// Attempt to index all fields without a known static token.
    Eager,
}

/// Semantics exposed by nghttp3 settings V3.
///
/// The ORIGIN frame payload is owned here. The C implementation stores a
/// borrowed pointer whose backing bytes must outlive the connection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettingsV3 {
    pub max_field_section_size: u64,
    pub qpack_max_table_capacity: u64,
    pub qpack_encoder_max_table_capacity: u64,
    pub qpack_blocked_streams: u64,
    pub enable_connect_protocol: bool,
    pub h3_datagram: bool,
    pub origin_list: Option<Vec<u8>>,
    pub glitch_rate_limit_burst: u64,
    pub glitch_rate_limit_rate: u64,
}

impl Default for SettingsV3 {
    fn default() -> Self {
        Self {
            max_field_section_size: varint::MAX,
            qpack_max_table_capacity: 0,
            qpack_encoder_max_table_capacity: DEFAULT_QPACK_ENCODER_MAX_TABLE_CAPACITY,
            qpack_blocked_streams: 0,
            enable_connect_protocol: false,
            h3_datagram: false,
            origin_list: None,
            glitch_rate_limit_burst: DEFAULT_GLITCH_RATE_LIMIT_BURST,
            glitch_rate_limit_rate: DEFAULT_GLITCH_RATE_LIMIT_RATE,
        }
    }
}

/// Current nghttp3 settings semantics (V4).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Settings {
    pub max_field_section_size: u64,
    pub qpack_max_table_capacity: u64,
    pub qpack_encoder_max_table_capacity: u64,
    pub qpack_blocked_streams: u64,
    pub enable_connect_protocol: bool,
    pub h3_datagram: bool,
    pub origin_list: Option<Vec<u8>>,
    pub glitch_rate_limit_burst: u64,
    pub glitch_rate_limit_rate: u64,
    pub qpack_indexing_strategy: IndexingStrategy,
}

impl Default for Settings {
    fn default() -> Self {
        SettingsV3::default().into()
    }
}

impl From<SettingsV3> for Settings {
    fn from(old: SettingsV3) -> Self {
        Self {
            max_field_section_size: old.max_field_section_size,
            qpack_max_table_capacity: old.qpack_max_table_capacity,
            qpack_encoder_max_table_capacity: old.qpack_encoder_max_table_capacity,
            qpack_blocked_streams: old.qpack_blocked_streams,
            enable_connect_protocol: old.enable_connect_protocol,
            h3_datagram: old.h3_datagram,
            origin_list: old.origin_list,
            glitch_rate_limit_burst: old.glitch_rate_limit_burst,
            glitch_rate_limit_rate: old.glitch_rate_limit_rate,
            qpack_indexing_strategy: IndexingStrategy::None,
        }
    }
}

impl From<Settings> for SettingsV3 {
    fn from(current: Settings) -> Self {
        Self {
            max_field_section_size: current.max_field_section_size,
            qpack_max_table_capacity: current.qpack_max_table_capacity,
            qpack_encoder_max_table_capacity: current.qpack_encoder_max_table_capacity,
            qpack_blocked_streams: current.qpack_blocked_streams,
            enable_connect_protocol: current.enable_connect_protocol,
            h3_datagram: current.h3_datagram,
            origin_list: current.origin_list,
            glitch_rate_limit_burst: current.glitch_rate_limit_burst,
            glitch_rate_limit_rate: current.glitch_rate_limit_rate,
        }
    }
}

impl Settings {
    /// Sets SETTINGS_MAX_FIELD_SECTION_SIZE when it fits QUIC's integer domain.
    pub fn with_max_field_section_size(mut self, value: u64) -> Option<Self> {
        (value <= varint::MAX).then(|| {
            self.max_field_section_size = value;
            self
        })
    }

    /// Sets SETTINGS_QPACK_MAX_TABLE_CAPACITY when representable on the wire.
    pub fn with_qpack_max_table_capacity(mut self, value: u64) -> Option<Self> {
        (value <= varint::MAX).then(|| {
            self.qpack_max_table_capacity = value;
            self
        })
    }

    /// Sets the local encoder table-capacity ceiling within the wire domain.
    pub fn with_qpack_encoder_max_table_capacity(mut self, value: u64) -> Option<Self> {
        (value <= varint::MAX).then(|| {
            self.qpack_encoder_max_table_capacity = value;
            self
        })
    }

    /// Sets SETTINGS_QPACK_BLOCKED_STREAMS when representable on the wire.
    pub fn with_qpack_blocked_streams(mut self, value: u64) -> Option<Self> {
        (value <= varint::MAX).then(|| {
            self.qpack_blocked_streams = value;
            self
        })
    }

    /// Replaces the owned serialized ORIGIN frame payload.
    pub fn with_origin_list(mut self, payload: impl Into<Vec<u8>>) -> Self {
        self.origin_list = Some(payload.into());
        self
    }

    /// Sets the nghttp3-local glitch-rate limiter controls.
    pub fn with_glitch_rate_limit(mut self, burst: u64, rate: u64) -> Self {
        self.glitch_rate_limit_burst = burst;
        self.glitch_rate_limit_rate = rate;
        self
    }

    /// Sets the V4 QPACK indexing strategy.
    pub fn with_qpack_indexing_strategy(mut self, strategy: IndexingStrategy) -> Self {
        self.qpack_indexing_strategy = strategy;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_c_oracle() {
        let rust = Settings::default();
        let c = nghttp3::Settings::default();

        assert_eq!(
            rust.max_field_section_size,
            c.max_field_section_size_value()
        );
        assert_eq!(
            rust.qpack_max_table_capacity,
            c.qpack_max_table_capacity_value() as u64
        );
        assert_eq!(
            rust.qpack_encoder_max_table_capacity,
            c.qpack_encoder_max_table_capacity_value() as u64
        );
        assert_eq!(
            rust.qpack_blocked_streams,
            c.qpack_blocked_streams_value() as u64
        );
        assert_eq!(rust.enable_connect_protocol, c.connect_protocol_enabled());
        assert_eq!(rust.h3_datagram, c.h3_datagram_enabled());
        assert_eq!(rust.origin_list.is_some(), c.has_origin_list());
        assert_eq!(
            (rust.glitch_rate_limit_burst, rust.glitch_rate_limit_rate),
            c.glitch_rate_limit_values()
        );
        assert_eq!(
            c.qpack_indexing_strategy_value(),
            Some(nghttp3::IndexingStrategy::None)
        );
        assert_eq!(rust.qpack_indexing_strategy, IndexingStrategy::None);
    }

    #[test]
    fn v3_upgrade_and_downgrade_match_original_semantics() {
        let old = SettingsV3 {
            max_field_section_size: 1_000_000_007,
            qpack_max_table_capacity: 1_000_000_009,
            qpack_encoder_max_table_capacity: 781_506_803,
            qpack_blocked_streams: 478_324_193,
            enable_connect_protocol: true,
            h3_datagram: true,
            origin_list: Some(b"foo".to_vec()),
            glitch_rate_limit_burst: 74_111,
            glitch_rate_limit_rate: 6_831,
        };

        let upgraded = Settings::from(old.clone());
        assert_eq!(upgraded.qpack_indexing_strategy, IndexingStrategy::None);
        assert_eq!(SettingsV3::from(upgraded), old);
    }

    #[test]
    fn wire_domain_setters_reject_max_plus_one() {
        assert!(Settings::default()
            .with_max_field_section_size(varint::MAX)
            .is_some());
        assert!(Settings::default()
            .with_max_field_section_size(varint::MAX + 1)
            .is_none());
        assert!(Settings::default()
            .with_qpack_max_table_capacity(varint::MAX + 1)
            .is_none());
        assert!(Settings::default()
            .with_qpack_blocked_streams(varint::MAX + 1)
            .is_none());
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn defaults_are_inside_wire_domain() {
        let settings = Settings::default();
        assert!(settings.max_field_section_size <= varint::MAX);
        assert!(settings.qpack_max_table_capacity <= varint::MAX);
        assert!(settings.qpack_encoder_max_table_capacity <= varint::MAX);
        assert!(settings.qpack_blocked_streams <= varint::MAX);
    }

    #[kani::proof]
    fn max_field_section_builder_matches_domain() {
        let value: u64 = kani::any();
        let accepted = Settings::default()
            .with_max_field_section_size(value)
            .is_some();
        assert_eq!(accepted, value <= varint::MAX);
    }

    #[kani::proof]
    fn v3_upgrade_preserves_existing_scalars() {
        let max_field_section_size: u64 = kani::any();
        let qpack_max_table_capacity: u64 = kani::any();
        let qpack_encoder_max_table_capacity: u64 = kani::any();
        let qpack_blocked_streams: u64 = kani::any();
        let enable_connect_protocol: bool = kani::any();
        let h3_datagram: bool = kani::any();
        let glitch_rate_limit_burst: u64 = kani::any();
        let glitch_rate_limit_rate: u64 = kani::any();

        let old = SettingsV3 {
            max_field_section_size,
            qpack_max_table_capacity,
            qpack_encoder_max_table_capacity,
            qpack_blocked_streams,
            enable_connect_protocol,
            h3_datagram,
            origin_list: None,
            glitch_rate_limit_burst,
            glitch_rate_limit_rate,
        };
        let upgraded = Settings::from(old);

        assert_eq!(upgraded.max_field_section_size, max_field_section_size);
        assert_eq!(upgraded.qpack_max_table_capacity, qpack_max_table_capacity);
        assert_eq!(
            upgraded.qpack_encoder_max_table_capacity,
            qpack_encoder_max_table_capacity
        );
        assert_eq!(upgraded.qpack_blocked_streams, qpack_blocked_streams);
        assert_eq!(upgraded.enable_connect_protocol, enable_connect_protocol);
        assert_eq!(upgraded.h3_datagram, h3_datagram);
        assert_eq!(upgraded.glitch_rate_limit_burst, glitch_rate_limit_burst);
        assert_eq!(upgraded.glitch_rate_limit_rate, glitch_rate_limit_rate);
        assert_eq!(upgraded.qpack_indexing_strategy, IndexingStrategy::None);
    }
}
