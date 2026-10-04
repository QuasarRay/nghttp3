//! Owned QPACK input storage for requests that can become blocked.
//!
//! A retained decoder request must not borrow a temporary network/fuzzer chunk.
//! This type stores backing bytes and an offset; every slice view is derived
//! from that owner on demand, so no self-referential or externally borrowed
//! span can outlive the allocation.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedInput {
    data: Box<[u8]>,
    offset: usize,
}

impl OwnedInput {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self {
            data: data.into().into_boxed_slice(),
            offset: 0,
        }
    }

    pub fn remaining(&self) -> &[u8] {
        &self.data[self.offset..]
    }

    pub const fn consumed(&self) -> usize {
        self.offset
    }

    pub fn original_len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.offset == self.data.len()
    }

    /// Advances within the owned backing allocation.
    ///
    /// Returns false and leaves the state unchanged if `count` exceeds the
    /// remaining bytes.
    pub fn consume(&mut self, count: usize) -> bool {
        if count > self.remaining().len() {
            return false;
        }
        self.offset += count;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retained_request_owns_original_chunk_regression_fce4985() {
        let request = OwnedInput::new(vec![1, 2, 3, 4]);
        assert_eq!(request.remaining(), &[1, 2, 3, 4]);
        assert_eq!(request.original_len(), 4);
    }

    #[test]
    fn consume_derives_subslice_from_owned_backing() {
        let mut request = OwnedInput::new(vec![1, 2, 3, 4]);
        assert!(request.consume(2));
        assert_eq!(request.remaining(), &[3, 4]);
        assert_eq!(request.consumed(), 2);
    }

    #[test]
    fn overconsume_is_rejected_without_state_change() {
        let mut request = OwnedInput::new(vec![1, 2]);
        assert!(!request.consume(3));
        assert_eq!(request.remaining(), &[1, 2]);
        assert_eq!(request.consumed(), 0);
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn symbolic_consume_preserves_owned_suffix() {
        let a: u8 = kani::any();
        let b: u8 = kani::any();
        let c: u8 = kani::any();
        let count: u8 = kani::any();
        kani::assume(count <= 3);

        let mut input = OwnedInput::new(vec![a, b, c]);
        assert!(input.consume(usize::from(count)));
        assert_eq!(input.consumed(), usize::from(count));
        assert_eq!(input.remaining().len(), 3 - usize::from(count));

        match count {
            0 => assert_eq!(input.remaining(), &[a, b, c]),
            1 => assert_eq!(input.remaining(), &[b, c]),
            2 => assert_eq!(input.remaining(), &[c]),
            3 => assert!(input.remaining().is_empty()),
            _ => unreachable!(),
        }
    }

    #[kani::proof]
    fn overconsume_preserves_state() {
        let byte: u8 = kani::any();
        let mut input = OwnedInput::new(vec![byte]);

        assert!(!input.consume(2));
        assert_eq!(input.consumed(), 0);
        assert_eq!(input.remaining(), &[byte]);
    }
}
