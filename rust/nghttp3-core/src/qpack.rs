//! Checked QPACK buffer-growth policy.
//!
//! The original C implementation rounds requested capacity to a power of two
//! and rejects requests above 2^31. This module preserves that policy without
//! unchecked integer arithmetic or shift-width edge cases.

/// Largest QPACK scratch-buffer capacity accepted by the historical policy.
pub const MAX_BUFFER_CAPACITY: usize = 1usize << 31;
/// Minimum allocated capacity once growth is needed.
pub const MIN_BUFFER_CAPACITY: usize = 32;

/// Computes the capacity needed to guarantee at least `extra_size` bytes free.
///
/// `current_left` must describe free space inside `current_capacity`.
/// When no growth is needed, the existing capacity is returned unchanged.
/// Growth returns a power-of-two capacity no larger than 2^31.
pub fn reserve_capacity(
    current_capacity: usize,
    current_left: usize,
    extra_size: usize,
) -> Option<usize> {
    if current_left > current_capacity {
        return None;
    }
    if current_left >= extra_size {
        return Some(current_capacity);
    }

    let missing = extra_size.checked_sub(current_left)?;
    let needed = current_capacity.checked_add(missing)?;
    let target = needed.max(MIN_BUFFER_CAPACITY);

    if target > MAX_BUFFER_CAPACITY {
        return None;
    }

    let rounded = target.checked_next_power_of_two()?;
    (rounded <= MAX_BUFFER_CAPACITY).then_some(rounded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_growth_preserves_capacity() {
        assert_eq!(reserve_capacity(64, 32, 16), Some(64));
    }

    #[test]
    fn rounds_growth_to_power_of_two() {
        assert_eq!(reserve_capacity(32, 0, 1), Some(64));
        assert_eq!(reserve_capacity(0, 0, 1), Some(32));
    }

    #[test]
    fn historical_upper_bound_regression() {
        assert_eq!(
            reserve_capacity(0, 0, MAX_BUFFER_CAPACITY),
            Some(MAX_BUFFER_CAPACITY)
        );
        assert_eq!(
            reserve_capacity(0, 0, MAX_BUFFER_CAPACITY + 1),
            None
        );
    }

    #[test]
    fn checked_arithmetic_rejects_wrapping_request() {
        assert_eq!(reserve_capacity(usize::MAX, 0, 1), None);
    }

    #[test]
    fn rejects_invalid_buffer_state() {
        assert_eq!(reserve_capacity(1, 2, 1), None);
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn successful_growth_is_bounded_and_sufficient() {
        let capacity: usize = kani::any();
        let left: usize = kani::any();
        let extra: usize = kani::any();
        kani::assume(left <= capacity);
        kani::assume(left < extra);

        if let Some(next) = reserve_capacity(capacity, left, extra) {
            assert!(next.is_power_of_two());
            assert!(next <= MAX_BUFFER_CAPACITY);
            assert!(next >= capacity);
            assert!(next - capacity >= extra - left);
        }
    }

    #[kani::proof]
    fn no_growth_is_identity() {
        let capacity: usize = kani::any();
        let left: usize = kani::any();
        let extra: usize = kani::any();
        kani::assume(left <= capacity);
        kani::assume(left >= extra);

        assert_eq!(reserve_capacity(capacity, left, extra), Some(capacity));
    }

    #[kani::proof]
    fn historical_rounding_boundary_is_exact() {
        assert_eq!(
            reserve_capacity(0, 0, MAX_BUFFER_CAPACITY),
            Some(MAX_BUFFER_CAPACITY)
        );
        assert_eq!(
            reserve_capacity(0, 0, MAX_BUFFER_CAPACITY + 1),
            None
        );
    }
}
