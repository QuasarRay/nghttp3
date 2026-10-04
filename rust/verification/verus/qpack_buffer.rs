#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

pub const MIN_CAPACITY: int = 32;
pub const MAX_CAPACITY: int = 2_147_483_648;

pub open spec fn accepted_pre_round(required: int) -> bool {
    0 <= required && required <= MAX_CAPACITY
}

pub open spec fn historical_bug_domain(required: int) -> bool {
    required > MAX_CAPACITY
}

proof fn historical_boundary_is_partitioned()
    ensures
        accepted_pre_round(MAX_CAPACITY),
        historical_bug_domain(MAX_CAPACITY + 1),
        !accepted_pre_round(MAX_CAPACITY + 1),
{
}

proof fn minimum_is_accepted()
    ensures
        accepted_pre_round(MIN_CAPACITY),
{
}

pub open spec fn checked_missing(extra: int, left: int) -> bool {
    0 <= left && left < extra
}

proof fn missing_space_is_positive(extra: int, left: int)
    requires
        checked_missing(extra, left),
    ensures
        extra - left > 0,
{
}

} // verus!

fn main() {}
