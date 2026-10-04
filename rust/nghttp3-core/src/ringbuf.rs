//! Safe replacement for nghttp3's internal byte-addressed ring buffer.
//!
//! The original C implementation requires a power-of-two capacity and uses
//! masked physical indices. This implementation preserves those observable
//! ordering/overwrite semantics without raw allocation, pointer arithmetic, or
//! memcpy.

/// A fixed-logical-capacity ring buffer.
///
/// Capacity is either zero or a power of two, matching the original nghttp3
/// representation. Pushes on a full buffer overwrite the opposite end exactly
/// as the C implementation does.
#[derive(Debug)]
pub struct RingBuffer<T> {
    storage: Vec<Option<T>>,
    capacity: usize,
    first: usize,
    len: usize,
}

impl<T> RingBuffer<T> {
    /// Creates an empty ring buffer when capacity is zero or a power of two.
    pub fn with_capacity(capacity: usize) -> Option<Self> {
        if capacity != 0 && !capacity.is_power_of_two() {
            return None;
        }

        Some(Self {
            storage: std::iter::repeat_with(|| None).take(capacity).collect(),
            capacity,
            first: 0,
            len: 0,
        })
    }

    /// Returns the logical capacity.
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the number of stored elements.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns whether no elements are stored.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns whether the logical capacity is fully occupied.
    pub const fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    /// Returns the element at logical offset.
    pub fn get(&self, offset: usize) -> Option<&T> {
        if offset >= self.len || self.capacity == 0 {
            return None;
        }

        let index = (self.first + offset) & (self.capacity - 1);
        self.storage[index].as_ref()
    }

    /// Iterates from logical front to logical back.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.len).map(|offset| {
            self.get(offset)
                .expect("occupied logical ring-buffer slot must be initialized")
        })
    }

    /// Pushes at the back.
    ///
    /// When full, returns and overwrites the old front element.
    /// For zero capacity, returns value without storing it.
    pub fn push_back(&mut self, value: T) -> Option<T> {
        if self.capacity == 0 {
            return Some(value);
        }

        let mask = self.capacity - 1;
        let index = (self.first + self.len) & mask;
        let displaced = self.storage[index].replace(value);

        if self.len == self.capacity {
            self.first = (self.first + 1) & mask;
        } else {
            self.len += 1;
        }

        displaced
    }

    /// Pushes at the front.
    ///
    /// When full, returns and overwrites the old back element.
    /// For zero capacity, returns value without storing it.
    pub fn push_front(&mut self, value: T) -> Option<T> {
        if self.capacity == 0 {
            return Some(value);
        }

        let mask = self.capacity - 1;
        self.first = self.first.wrapping_sub(1) & mask;
        let displaced = self.storage[self.first].replace(value);

        if self.len < self.capacity {
            self.len += 1;
        }

        displaced
    }

    /// Removes and returns the logical front element.
    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let index = self.first;
        let value = self.storage[index].take();
        self.len -= 1;
        self.first = (self.first + 1) & (self.capacity - 1);
        value
    }

    /// Removes and returns the logical back element.
    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let index = (self.first + self.len - 1) & (self.capacity - 1);
        let value = self.storage[index].take();
        self.len -= 1;
        value
    }

    /// Grows logical capacity while preserving logical element order.
    ///
    /// Returns false when the requested larger capacity is not a power of
    /// two. Requests no larger than the existing capacity are no-ops.
    pub fn reserve(&mut self, new_capacity: usize) -> bool {
        if new_capacity <= self.capacity {
            return true;
        }
        if !new_capacity.is_power_of_two() {
            return false;
        }

        let mut next: Vec<Option<T>> = std::iter::repeat_with(|| None).take(new_capacity).collect();

        if self.capacity != 0 {
            let mask = self.capacity - 1;
            for (offset, slot) in next.iter_mut().take(self.len).enumerate() {
                let index = (self.first + offset) & mask;
                *slot = self.storage[index].take();
            }
        }

        self.storage = next;
        self.capacity = new_capacity;
        self.first = 0;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_back_overwrites_front_when_full() {
        let mut rb = RingBuffer::with_capacity(2).unwrap();
        assert_eq!(rb.push_back(1), None);
        assert_eq!(rb.push_back(2), None);
        assert_eq!(rb.push_back(3), Some(1));
        assert_eq!(rb.iter().copied().collect::<Vec<_>>(), vec![2, 3]);
    }

    #[test]
    fn push_front_overwrites_back_when_full() {
        let mut rb = RingBuffer::with_capacity(2).unwrap();
        rb.push_back(1);
        rb.push_back(2);
        assert_eq!(rb.push_front(0), Some(2));
        assert_eq!(rb.iter().copied().collect::<Vec<_>>(), vec![0, 1]);
    }

    #[test]
    fn wrapped_reserve_preserves_logical_order_regression_97cb58e() {
        let mut rb = RingBuffer::with_capacity(4).unwrap();
        for value in [1, 2, 3, 4] {
            rb.push_back(value);
        }

        assert_eq!(rb.pop_front(), Some(1));
        assert_eq!(rb.pop_front(), Some(2));
        rb.push_back(5);
        rb.push_back(6);

        assert_eq!(rb.iter().copied().collect::<Vec<_>>(), vec![3, 4, 5, 6]);
        assert!(rb.reserve(8));
        assert_eq!(rb.iter().copied().collect::<Vec<_>>(), vec![3, 4, 5, 6]);
        assert_eq!(rb.capacity(), 8);
    }

    #[test]
    fn zero_capacity_is_safe_and_non_storing() {
        let mut rb = RingBuffer::with_capacity(0).unwrap();
        assert_eq!(rb.push_back(7), Some(7));
        assert_eq!(rb.push_front(8), Some(8));
        assert_eq!(rb.pop_front(), None);
        assert!(rb.is_empty());
        assert!(rb.is_full());
    }

    #[test]
    fn rejects_non_power_of_two_growth() {
        assert!(RingBuffer::<u8>::with_capacity(3).is_none());

        let mut rb = RingBuffer::with_capacity(4).unwrap();
        assert!(!rb.reserve(6));
        assert_eq!(rb.capacity(), 4);
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn historical_wrapped_growth_preserves_symbolic_order() {
        let a: u8 = kani::any();
        let b: u8 = kani::any();
        let c: u8 = kani::any();
        let d: u8 = kani::any();
        let e: u8 = kani::any();
        let f: u8 = kani::any();

        let mut rb = RingBuffer::with_capacity(4).unwrap();
        rb.push_back(a);
        rb.push_back(b);
        rb.push_back(c);
        rb.push_back(d);
        assert_eq!(rb.pop_front(), Some(a));
        assert_eq!(rb.pop_front(), Some(b));
        rb.push_back(e);
        rb.push_back(f);

        assert!(rb.reserve(8));
        assert_eq!(rb.get(0), Some(&c));
        assert_eq!(rb.get(1), Some(&d));
        assert_eq!(rb.get(2), Some(&e));
        assert_eq!(rb.get(3), Some(&f));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.capacity(), 8);
    }

    #[kani::proof]
    fn full_push_back_drops_exactly_front() {
        let a: u8 = kani::any();
        let b: u8 = kani::any();
        let c: u8 = kani::any();

        let mut rb = RingBuffer::with_capacity(2).unwrap();
        rb.push_back(a);
        rb.push_back(b);
        assert_eq!(rb.push_back(c), Some(a));
        assert_eq!(rb.get(0), Some(&b));
        assert_eq!(rb.get(1), Some(&c));
        assert_eq!(rb.len(), 2);
    }
}
