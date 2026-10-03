//! Normative source provenance carried by executable rules.

/// A stable reference from an executable rule back to its normative source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleSource {
    /// RFC number.
    pub rfc: u16,
    /// RFC section containing the normative requirement.
    pub section: &'static str,
    /// Human-readable rule identifier used by proofs and generated tests.
    pub rule: &'static str,
}

impl RuleSource {
    /// Constructs rule provenance.
    pub const fn new(rfc: u16, section: &'static str, rule: &'static str) -> Self {
        Self { rfc, section, rule }
    }
}
