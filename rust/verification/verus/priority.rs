#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

pub const URGENCY_HIGH: int = 0;
pub const URGENCY_LOW: int = 7;

pub open spec fn valid_urgency(value: int) -> bool {
    URGENCY_HIGH <= value && value <= URGENCY_LOW
}

/// After consuming '=' at index eq_pos, a value byte exists iff the next
/// position remains strictly inside the input.
pub open spec fn value_exists_after_equals(eq_pos: int, len: int) -> bool {
    eq_pos + 1 < len
}

proof fn trailing_equals_has_no_value(eq_pos: int, len: int)
    requires
        0 <= eq_pos,
        eq_pos < len,
        eq_pos + 1 == len,
    ensures
        !value_exists_after_equals(eq_pos, len),
{
}

proof fn urgency_boundaries_are_valid()
    ensures
        valid_urgency(URGENCY_HIGH),
        valid_urgency(3),
        valid_urgency(URGENCY_LOW),
        !valid_urgency(-1),
        !valid_urgency(8),
{
}

pub open spec fn checked_advance(pos: int, len: int) -> bool {
    0 <= pos && pos < len
}

proof fn checked_cursor_read_stays_in_bounds(pos: int, len: int)
    requires
        checked_advance(pos, len),
    ensures
        0 <= pos < len,
{
}

} // verus!

fn main() {}
