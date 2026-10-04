//! Safe transient ownership state for QPACK decoder insert instructions.
//!
//! The historical C decoder stored reference-counted name/value pointers in
//! read state. Failure paths could decref those objects while leaving the stale
//! pointer published in the decoder state. Rust models the transient values as
//! owned `Option<T>` values and removes them from the state before insertion
//! is attempted.

/// Failure while finishing one decoder insertion instruction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FinishError<E> {
    MissingName,
    MissingValue,
    Insert(E),
}

/// Transient owned fields assembled while decoding one QPACK encoder-stream
/// instruction.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct DecoderFieldState<T> {
    name: Option<T>,
    value: Option<T>,
}

impl<T> DecoderFieldState<T> {
    pub const fn new() -> Self {
        Self {
            name: None,
            value: None,
        }
    }

    pub fn with_name(name: T) -> Self {
        Self {
            name: Some(name),
            value: None,
        }
    }

    pub fn with_value(value: T) -> Self {
        Self {
            name: None,
            value: Some(value),
        }
    }

    pub fn with_fields(name: T, value: T) -> Self {
        Self {
            name: Some(name),
            value: Some(value),
        }
    }

    pub fn set_name(&mut self, name: T) -> Option<T> {
        self.name.replace(name)
    }

    pub fn set_value(&mut self, value: T) -> Option<T> {
        self.value.replace(value)
    }

    pub const fn has_name(&self) -> bool {
        self.name.is_some()
    }

    pub const fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub const fn is_clear(&self) -> bool {
        self.name.is_none() && self.value.is_none()
    }

    /// Finishes an indexed insert, which owns a transient value but obtains its
    /// name from the static or dynamic table.
    ///
    /// The value is removed from decoder state *before* `insert` runs. Thus
    /// both success and insertion failure leave no stale value in read state.
    pub fn finish_indexed_insert<E>(
        &mut self,
        insert: impl FnOnce(T) -> Result<(), E>,
    ) -> Result<(), FinishError<E>> {
        let value = self.value.take().ok_or(FinishError::MissingValue)?;
        insert(value).map_err(FinishError::Insert)
    }

    /// Finishes an insert with a literal name and value.
    ///
    /// Both fields are removed before `insert` runs. Missing fields are
    /// diagnosed without consuming a complete counterpart.
    pub fn finish_literal_insert<E>(
        &mut self,
        insert: impl FnOnce(T, T) -> Result<(), E>,
    ) -> Result<(), FinishError<E>> {
        if self.name.is_none() {
            return Err(FinishError::MissingName);
        }
        if self.value.is_none() {
            return Err(FinishError::MissingValue);
        }

        let name = self.name.take().expect("checked above");
        let value = self.value.take().expect("checked above");
        insert(name, value).map_err(FinishError::Insert)
    }

    /// Clears any partially assembled instruction state.
    pub fn clear(&mut self) {
        self.name = None;
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexed_insert_failure_clears_transient_value() {
        let mut state = DecoderFieldState::with_value(vec![1, 2, 3]);
        let result = state.finish_indexed_insert(|_| Err::<(), _>("oom"));

        assert_eq!(result, Err(FinishError::Insert("oom")));
        assert!(!state.has_value());
    }

    #[test]
    fn literal_insert_failure_clears_both_transient_fields() {
        let mut state = DecoderFieldState::with_fields(vec![1], vec![2]);
        let result = state.finish_literal_insert(|_, _| Err::<(), _>("oom"));

        assert_eq!(result, Err(FinishError::Insert("oom")));
        assert!(state.is_clear());
    }

    #[test]
    fn successful_insert_also_clears_transient_state() {
        let mut indexed = DecoderFieldState::with_value(7_u8);
        assert_eq!(indexed.finish_indexed_insert(|_| Ok::<(), ()>(())), Ok(()));
        assert!(!indexed.has_value());

        let mut literal = DecoderFieldState::with_fields(1_u8, 2_u8);
        assert_eq!(
            literal.finish_literal_insert(|_, _| Ok::<(), ()>(())),
            Ok(())
        );
        assert!(literal.is_clear());
    }

    #[test]
    fn missing_literal_name_does_not_consume_value() {
        let mut state = DecoderFieldState::with_value(9_u8);
        assert_eq!(
            state.finish_literal_insert(|_, _| Ok::<(), ()>(())),
            Err(FinishError::MissingName)
        );
        assert!(state.has_value());
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn indexed_insert_clears_value_on_success_or_failure() {
        let value: u8 = kani::any();
        let fail: bool = kani::any();
        let mut state = DecoderFieldState::with_value(value);

        let _ = state.finish_indexed_insert(|_| if fail { Err(()) } else { Ok(()) });

        assert!(!state.has_value());
    }

    #[kani::proof]
    fn literal_insert_clears_name_and_value_on_success_or_failure() {
        let name: u8 = kani::any();
        let value: u8 = kani::any();
        let fail: bool = kani::any();
        let mut state = DecoderFieldState::with_fields(name, value);

        let _ = state.finish_literal_insert(|_, _| if fail { Err(()) } else { Ok(()) });

        assert!(state.is_clear());
    }
}
