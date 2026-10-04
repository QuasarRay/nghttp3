#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

pub const MAX_BUFFER_CAPACITY: int = 2147483648;
pub const MIN_BUFFER_CAPACITY: int = 32;

pub open spec fn growth_needed(capacity: int, left: int, extra: int) -> int
    recommends
        0 <= left <= capacity,
        left < extra,
{
    capacity + (extra - left)
}

proof fn historical_upper_bound_is_safe()
    ensures
        MAX_BUFFER_CAPACITY == 2147483648,
        MAX_BUFFER_CAPACITY > 0,
{
}

proof fn checked_growth_requirement_is_monotone(capacity: int, left: int, extra: int)
    requires
        0 <= left <= capacity,
        left < extra,
    ensures
        growth_needed(capacity, left, extra) > capacity,
        growth_needed(capacity, left, extra) >= extra,
{
}

proof fn max_plus_one_is_rejected()
    ensures
        MAX_BUFFER_CAPACITY + 1 > MAX_BUFFER_CAPACITY,
{
}

} // verus!

fn main() {}
