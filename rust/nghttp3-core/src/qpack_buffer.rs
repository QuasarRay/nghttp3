//! Checked buffer-growth arithmetic for QPACK encoding.
//!
//! This isolates the arithmetic behind historical nghttp3 commit 8a8d45cb,
//! which fixed an overflow when rounding requests above 2^31 to the next power
//! of two through a 32-bit shift.

/// Minimum allocation used by the C QPACK encoder.
pub const MIN_CAPACITY: usize = 32;

/// Largest pre-rounding request accepted by the reference implementation.
pub const MAX_CAPACITY: usize = 1_usize << 31;

/// Buffer-growth failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrowthError {
    /// Intermediate size arithmetic exceeded the platform size type.
    ArithmeticOverflow,
    /// The request cannot be rounded within the reference 2^31 ceiling.
    TooLarge,
}

/// Rounds a required capacity to the reference QPACK allocation size.
pub fn rounded_capacity(required: usize) -> Result<usize, GrowthError> {
    let required = required.max(MIN_CAPACITY);
    if required > MAX_CAPACITY {
        return Err(GrowthError::TooLarge);
    }

    let rounded = required
        .checked_next_power_of_two()
        .ok_or(GrowthError::TooLarge)?;

    if rounded > MAX_CAPACITY {
        return Err(GrowthError::TooLarge);
    }

    Ok(rounded)
}

/// Computes the allocation required to guarantee extra_size bytes free.
///
/// Ok(None) means the current buffer already has enough free space.
pub fn reserve_capacity(
    current_capacity: usize,
    left: usize,
    extra_size: usize,
) -> Result<Option<usize>, GrowthError> {
    if left >= extra_size {
        return Ok(None);
    }

    let missing = extra_size - left;
    let required = current_capacity
        .checked_add(missing)
        .ok_or(GrowthError::ArithmeticOverflow)?;

    rounded_capacity(required).map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn historical_next_power_of_two_overflow_regression_8a8d45c() {
        assert_eq!(rounded_capacity(MAX_CAPACITY), Ok(MAX_CAPACITY));
        assert_eq!(
            rounded_capacity(MAX_CAPACITY - 1),
            Ok(MAX_CAPACITY)
        );
        assert_eq!(
            rounded_capacity(MAX_CAPACITY + 1),
            Err(GrowthError::TooLarge)
        );
    }

    #[test]
    fn preserves_minimum_allocation() {
        for required in 0..=MIN_CAPACITY {
            assert_eq!(rounded_capacity(required), Ok(MIN_CAPACITY));
        }
    }

    #[test]
    fn rounds_representative_requests() {
        assert_eq!(rounded_capacity(33), Ok(64));
        assert_eq!(rounded_capacity(65), Ok(128));
        assert_eq!(rounded_capacity(1025), Ok(2048));
    }

    #[test]
    fn reserve_is_noop_when_space_is_already_available() {
        assert_eq!(reserve_capacity(128, 64, 64), Ok(None));
        assert_eq!(reserve_capacity(128, 65, 64), Ok(None));
    }

    #[test]
    fn reserve_uses_checked_size_arithmetic() {
        assert_eq!(
            reserve_capacity(usize::MAX, 0, 1),
            Err(GrowthError::ArithmeticOverflow)
        );
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn every_accepted_32_bit_request_rounds_safely() {
        let required: u32 = kani::any();
        kani::assume(u64::from(required) <= MAX_CAPACITY as u64);

        let required = required as usize;
        let rounded = rounded_capacity(required).unwrap();

        assert!(rounded >= required.max(MIN_CAPACITY));
        assert!(rounded <= MAX_CAPACITY);
        assert!(rounded.is_power_of_two());
    }

    #[kani::proof]
    fn historical_max_plus_one_is_rejected() {
        assert_eq!(
            rounded_capacity(MAX_CAPACITY + 1),
            Err(GrowthError::TooLarge)
        );
    }

    #[kani::proof]
    fn checked_addition_never_wraps() {
        let current_capacity: usize = kani::any();
        let extra_size: usize = kani::any();

        let result = reserve_capacity(current_capacity, 0, extra_size);
        if current_capacity.checked_add(extra_size).is_none() {
            assert_eq!(result, Err(GrowthError::ArithmeticOverflow));
        }
    }

    #[kani::proof]
    fn successful_growth_covers_missing_space() {
        let current_capacity: u32 = kani::any();
        let left: u32 = kani::any();
        let extra_size: u32 = kani::any();

        kani::assume(left < extra_size);
        let required = u64::from(current_capacity) + u64::from(extra_size - left);
        kani::assume(required <= MAX_CAPACITY as u64);

        let result = reserve_capacity(
            current_capacity as usize,
            left as usize,
            extra_size as usize,
        )
        .unwrap()
        .unwrap();

        assert!(result as u64 >= required.max(MIN_CAPACITY as u64));
        assert!(result <= MAX_CAPACITY);
        assert!(result.is_power_of_two());
    }
}
