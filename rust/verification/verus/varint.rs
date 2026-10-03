#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

pub const MAX_VARINT: u64 = 4_611_686_018_427_387_903;

pub open spec fn encoded_len_spec(value: int) -> int {
    if value < 64 {
        1
    } else if value < 16_384 {
        2
    } else if value < 1_073_741_824 {
        4
    } else {
        8
    }
}

pub fn encoded_len(value: u64) -> (len: usize)
    requires
        value <= MAX_VARINT,
    ensures
        len as int == encoded_len_spec(value as int),
        len == 1 || len == 2 || len == 4 || len == 8,
{
    if value < 64 {
        1
    } else if value < 16_384 {
        2
    } else if value < 1_073_741_824 {
        4
    } else {
        8
    }
}

proof fn classifier_boundaries(value: int)
    requires
        0 <= value,
        value <= MAX_VARINT as int,
    ensures
        encoded_len_spec(value) == 1
            || encoded_len_spec(value) == 2
            || encoded_len_spec(value) == 4
            || encoded_len_spec(value) == 8,
{
}

} // verus!

fn main() {}
