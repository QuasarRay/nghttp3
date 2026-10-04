//! Ownership-safe temporary state for QPACK decoder instructions.
//!
//! The C decoder historically stored reference-counted name/value pointers in
//! its read state. Commit ecfae7ac fixed a null dereference by clearing those
//! slots after decrementing their final temporary references. Rust can encode
//! the stronger invariant directly: extracting a temporary moves it out of the
//! read state, so a later reset/free path cannot observe a stale owner.

/// A missing temporary required to complete a decoder operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Missing {
    Name,
    Value,
}

/// One extracted value temporary.
///
/// This type owns the bytes after they have been removed from the decoder read
/// state. Dropping it models an insertion/allocation failure without leaving a
/// stale owner behind in the state.
#[derive(Debug, Eq, PartialEq)]
pub struct PendingValue {
    value: Vec<u8>,
}

impl PendingValue {
    /// Borrows the extracted value.
    pub fn as_slice(&self) -> &[u8] {
        &self.value
    }

    /// Transfers ownership to the caller.
    pub fn into_inner(self) -> Vec<u8> {
        self.value
    }
}

/// Extracted literal-name/value temporaries.
#[derive(Debug, Eq, PartialEq)]
pub struct PendingLiteral {
    name: Vec<u8>,
    value: Vec<u8>,
}

impl PendingLiteral {
    /// Borrows the extracted name.
    pub fn name(&self) -> &[u8] {
        &self.name
    }

    /// Borrows the extracted value.
    pub fn value(&self) -> &[u8] {
        &self.value
    }

    /// Transfers both owners to the caller.
    pub fn into_parts(self) -> (Vec<u8>, Vec<u8>) {
        (self.name, self.value)
    }
}

/// Owned temporary state used while decoding one QPACK instruction.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct DecoderReadState {
    name: Option<Vec<u8>>,
    value: Option<Vec<u8>>,
}

impl DecoderReadState {
    /// Creates an empty decoder read state.
    pub const fn new() -> Self {
        Self {
            name: None,
            value: None,
        }
    }

    /// Replaces the pending literal name.
    pub fn set_name(&mut self, name: impl Into<Vec<u8>>) {
        self.name = Some(name.into());
    }

    /// Replaces the pending value.
    pub fn set_value(&mut self, value: impl Into<Vec<u8>>) {
        self.value = Some(value.into());
    }

    /// Returns whether a pending name is owned by the read state.
    pub const fn has_name(&self) -> bool {
        self.name.is_some()
    }

    /// Returns whether a pending value is owned by the read state.
    pub const fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Borrows the pending name.
    pub fn name(&self) -> Option<&[u8]> {
        self.name.as_deref()
    }

    /// Borrows the pending value.
    pub fn value(&self) -> Option<&[u8]> {
        self.value.as_deref()
    }

    /// Moves a value temporary out of the read state.
    ///
    /// On success, has_value() is necessarily false. This is the safe-state
    /// equivalent of the C fix that decremented rstate.value and then assigned
    /// NULL, except Rust makes retaining the same owner impossible.
    pub fn take_value(&mut self) -> Result<PendingValue, Missing> {
        self.value
            .take()
            .map(|value| PendingValue { value })
            .ok_or(Missing::Value)
    }

    /// Atomically moves a literal name and value out of the read state.
    ///
    /// If either temporary is absent, the state is left unchanged.
    pub fn take_literal(&mut self) -> Result<PendingLiteral, Missing> {
        if self.name.is_none() {
            return Err(Missing::Name);
        }
        if self.value.is_none() {
            return Err(Missing::Value);
        }

        let name = self.name.take().expect("checked above");
        let value = self.value.take().expect("checked above");
        Ok(PendingLiteral { name, value })
    }

    /// Clears any temporaries still owned by the state.
    pub fn reset(&mut self) {
        self.name = None;
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_extraction_clears_state_before_possible_failure() {
        let mut state = DecoderReadState::new();
        state.set_value(b"value".to_vec());

        let pending = state.take_value().unwrap();
        assert_eq!(pending.as_slice(), b"value");
        assert!(!state.has_value());

        // Simulate a downstream insertion/allocation failure.
        drop(pending);
        assert!(!state.has_value());
        state.reset();
        assert!(!state.has_value());
    }

    #[test]
    fn literal_extraction_moves_both_owners() {
        let mut state = DecoderReadState::new();
        state.set_name(b"name".to_vec());
        state.set_value(b"value".to_vec());

        let pending = state.take_literal().unwrap();
        assert_eq!(pending.name(), b"name");
        assert_eq!(pending.value(), b"value");
        assert!(!state.has_name());
        assert!(!state.has_value());
    }

    #[test]
    fn incomplete_literal_extraction_is_atomic() {
        let mut state = DecoderReadState::new();
        state.set_name(b"name".to_vec());

        assert_eq!(state.take_literal(), Err(Missing::Value));
        assert_eq!(state.name(), Some(&b"name"[..]));
        assert!(!state.has_value());

        state.reset();
        state.set_value(b"value".to_vec());
        assert_eq!(state.take_literal(), Err(Missing::Name));
        assert_eq!(state.value(), Some(&b"value"[..]));
    }

    #[test]
    fn historical_oom_stale_pointer_regression_ecfae7a() {
        let mut state = DecoderReadState::new();
        state.set_name(b"name".to_vec());
        state.set_value(b"value".to_vec());

        let pending = state.take_literal().unwrap();
        drop(pending); // insertion fails; temporary owners are freed here

        assert!(!state.has_name());
        assert!(!state.has_value());

        // Historical C reset/free would dereference stale slots here.
        state.reset();
        assert!(!state.has_name());
        assert!(!state.has_value());
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn take_value_moves_symbolic_owner_out() {
        let byte: u8 = kani::any();
        let mut state = DecoderReadState::new();
        state.set_value(vec![byte]);

        let pending = state.take_value().unwrap();
        assert_eq!(pending.as_slice(), &[byte]);
        assert!(!state.has_value());

        drop(pending);
        assert!(!state.has_value());
    }

    #[kani::proof]
    fn take_literal_moves_both_symbolic_owners_out() {
        let name_byte: u8 = kani::any();
        let value_byte: u8 = kani::any();
        let mut state = DecoderReadState::new();
        state.set_name(vec![name_byte]);
        state.set_value(vec![value_byte]);

        let pending = state.take_literal().unwrap();
        assert_eq!(pending.name(), &[name_byte]);
        assert_eq!(pending.value(), &[value_byte]);
        assert!(!state.has_name());
        assert!(!state.has_value());
    }

    #[kani::proof]
    fn failed_literal_take_does_not_partially_consume_name() {
        let name_byte: u8 = kani::any();
        let mut state = DecoderReadState::new();
        state.set_name(vec![name_byte]);

        assert_eq!(state.take_literal(), Err(Missing::Value));
        assert_eq!(state.name(), Some(&[name_byte][..]));
        assert!(!state.has_value());
    }

    #[kani::proof]
    fn historical_failure_then_reset_has_no_stale_owner() {
        let name_byte: u8 = kani::any();
        let value_byte: u8 = kani::any();
        let mut state = DecoderReadState::new();
        state.set_name(vec![name_byte]);
        state.set_value(vec![value_byte]);

        let pending = state.take_literal().unwrap();
        drop(pending); // model dynamic-table insertion failure
        assert!(!state.has_name());
        assert!(!state.has_value());

        state.reset();
        assert!(!state.has_name());
        assert!(!state.has_value());
    }
}
