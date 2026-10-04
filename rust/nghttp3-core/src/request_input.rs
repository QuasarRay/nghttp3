//! Owned request-input storage for blocked QPACK decoding.
//!
//! Upstream fuzz commit fce49858 fixed an ASAN crash by copying request bytes
//! into storage owned by the retained Request object. The previous harness kept
//! a shallow nghttp3_buf view into a temporary fuzz chunk; blocked requests
//! could outlive that chunk.
//!
//! Rust avoids self-referential spans here. The request owns one Vec<u8> and
//! represents the current unread region with a checked offset.

/// Attempted to advance beyond the owned request bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdvanceError {
    pub requested: usize,
    pub remaining: usize,
}

/// Request bytes with an owned, bounds-checked read cursor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedRequestInput {
    data: Vec<u8>,
    offset: usize,
}

impl OwnedRequestInput {
    /// Takes ownership of an existing request buffer without copying.
    pub fn from_vec(data: Vec<u8>) -> Self {
        Self { data, offset: 0 }
    }

    /// Copies bytes into storage owned by the retained request.
    pub fn copy_from_slice(data: &[u8]) -> Self {
        Self::from_vec(data.to_vec())
    }

    /// Returns the full owned input, including already consumed bytes.
    pub fn all(&self) -> &[u8] {
        &self.data
    }

    /// Returns the unread request bytes.
    pub fn remaining(&self) -> &[u8] {
        &self.data[self.offset..]
    }

    /// Returns the unread byte count.
    pub fn remaining_len(&self) -> usize {
        self.data.len() - self.offset
    }

    /// Returns the current absolute cursor offset.
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Returns whether all input has been consumed.
    pub fn is_empty(&self) -> bool {
        self.offset == self.data.len()
    }

    /// Advances the cursor after a decoder consumes bytes.
    ///
    /// On error the cursor is unchanged.
    pub fn advance(&mut self, consumed: usize) -> Result<(), AdvanceError> {
        let remaining = self.remaining_len();
        if consumed > remaining {
            return Err(AdvanceError {
                requested: consumed,
                remaining,
            });
        }

        self.offset += consumed;
        Ok(())
    }

    /// Restores the cursor to the beginning while retaining owned storage.
    pub fn rewind(&mut self) {
        self.offset = 0;
    }

    /// Recovers the backing allocation.
    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copied_input_outlives_temporary_source_regression_fce49858() {
        let request = {
            let temporary_chunk = vec![1_u8, 2, 3, 4];
            OwnedRequestInput::copy_from_slice(&temporary_chunk)
        };

        // The temporary source has been dropped, but the blocked request still
        // owns valid bytes.
        assert_eq!(request.remaining(), &[1, 2, 3, 4]);
    }

    #[test]
    fn cursor_advance_uses_owned_storage() {
        let mut request = OwnedRequestInput::from_vec(vec![1, 2, 3, 4]);

        request.advance(2).unwrap();
        assert_eq!(request.offset(), 2);
        assert_eq!(request.remaining(), &[3, 4]);

        request.advance(2).unwrap();
        assert!(request.is_empty());
        assert_eq!(request.remaining(), &[]);
    }

    #[test]
    fn failed_advance_is_atomic() {
        let mut request = OwnedRequestInput::from_vec(vec![1, 2, 3]);
        request.advance(1).unwrap();

        assert_eq!(
            request.advance(3),
            Err(AdvanceError {
                requested: 3,
                remaining: 2,
            })
        );
        assert_eq!(request.offset(), 1);
        assert_eq!(request.remaining(), &[2, 3]);
    }

    #[test]
    fn rewind_keeps_the_same_owned_bytes() {
        let mut request = OwnedRequestInput::from_vec(vec![5, 6, 7]);
        request.advance(2).unwrap();
        request.rewind();

        assert_eq!(request.offset(), 0);
        assert_eq!(request.remaining(), &[5, 6, 7]);
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn bounded_symbolic_advance_preserves_cursor_invariant() {
        let a: u8 = kani::any();
        let b: u8 = kani::any();
        let c: u8 = kani::any();
        let consumed: u8 = kani::any();
        kani::assume(consumed <= 3);

        let mut request = OwnedRequestInput::from_vec(vec![a, b, c]);
        request.advance(usize::from(consumed)).unwrap();

        assert_eq!(request.offset(), usize::from(consumed));
        assert_eq!(request.remaining_len(), 3 - usize::from(consumed));
        assert!(request.offset() <= request.all().len());
    }

    #[kani::proof]
    fn out_of_bounds_advance_leaves_offset_unchanged() {
        let a: u8 = kani::any();
        let b: u8 = kani::any();
        let mut request = OwnedRequestInput::from_vec(vec![a, b]);
        request.advance(1).unwrap();

        let before = request.offset();
        assert!(request.advance(2).is_err());
        assert_eq!(request.offset(), before);
        assert_eq!(request.remaining(), &[b]);
    }

    #[kani::proof]
    fn rewind_restores_full_symbolic_input() {
        let a: u8 = kani::any();
        let b: u8 = kani::any();
        let mut request = OwnedRequestInput::from_vec(vec![a, b]);
        request.advance(2).unwrap();
        request.rewind();

        assert_eq!(request.remaining(), &[a, b]);
        assert_eq!(request.offset(), 0);
    }
}
