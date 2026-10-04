#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

pub struct OwnedInputState {
    pub len: int,
    pub offset: int,
}

pub open spec fn valid(state: OwnedInputState) -> bool {
    0 <= state.offset <= state.len
}

pub open spec fn remaining(state: OwnedInputState) -> int
    recommends valid(state),
{
    state.len - state.offset
}

proof fn checked_consume_preserves_bounds(state: OwnedInputState, count: int)
    requires
        valid(state),
        0 <= count <= remaining(state),
    ensures
        valid(OwnedInputState {
            len: state.len,
            offset: state.offset + count,
        }),
        remaining(OwnedInputState {
            len: state.len,
            offset: state.offset + count,
        }) == remaining(state) - count,
{
}

proof fn rejected_overconsume_preserves_state(state: OwnedInputState, count: int)
    requires
        valid(state),
        count > remaining(state),
    ensures
        valid(state),
{
}

} // verus!

fn main() {}
