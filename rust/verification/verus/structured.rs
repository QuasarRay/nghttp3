#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

pub open spec fn has_value_after_equals(eq: int, len: int) -> bool {
    0 <= eq && eq < len && eq + 1 < len
}

proof fn terminal_equals_has_no_value(len: int)
    requires
        0 < len,
    ensures
        !has_value_after_equals(len - 1, len),
{
}

pub open spec fn valid_urgency(value: int) -> bool {
    0 <= value && value <= 7
}

proof fn urgency_boundaries()
    ensures
        valid_urgency(0),
        valid_urgency(7),
        !valid_urgency(8),
        !valid_urgency(-1),
{
}

pub open spec fn cursor_can_read(pos: int, len: int) -> bool {
    0 <= pos && pos < len
}

proof fn advancing_terminal_equals_cannot_read(len: int)
    requires
        0 < len,
    ensures
        !cursor_can_read((len - 1) + 1, len),
{
}

} // verus!

fn main() {}
