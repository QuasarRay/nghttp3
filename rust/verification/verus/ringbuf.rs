#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

pub open spec fn physical_index(first: int, offset: int, capacity: int) -> int
    recommends
        capacity > 0,
{
    (first + offset) % capacity
}

proof fn physical_index_is_in_bounds(first: int, offset: int, capacity: int)
    requires
        capacity > 0,
        0 <= first < capacity,
        0 <= offset,
    ensures
        0 <= physical_index(first, offset, capacity) < capacity,
{
}

pub open spec fn post_reserve_index(offset: int) -> int {
    offset
}

proof fn reserve_normalizes_logical_order(offset: int, new_capacity: int)
    requires
        0 <= offset < new_capacity,
        new_capacity > 0,
    ensures
        post_reserve_index(offset) == offset,
        0 <= post_reserve_index(offset) < new_capacity,
{
}

} // verus!

fn main() {}
