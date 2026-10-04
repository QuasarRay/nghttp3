#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

pub enum FieldSlot {
    Empty,
    Present,
}

pub struct DecoderFieldState {
    pub name: FieldSlot,
    pub value: FieldSlot,
}

pub open spec fn indexed_finished(state: DecoderFieldState) -> bool {
    matches!(state.value, FieldSlot::Empty)
}

pub open spec fn literal_finished(state: DecoderFieldState) -> bool {
    matches!(state.name, FieldSlot::Empty)
        && matches!(state.value, FieldSlot::Empty)
}

proof fn indexed_attempt_clears_value(name: FieldSlot)
    ensures
        indexed_finished(DecoderFieldState {
            name,
            value: FieldSlot::Empty,
        }),
{
}

proof fn literal_attempt_clears_both()
    ensures
        literal_finished(DecoderFieldState {
            name: FieldSlot::Empty,
            value: FieldSlot::Empty,
        }),
{
}

} // verus!

fn main() {}
