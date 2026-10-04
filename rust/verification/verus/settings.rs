#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

pub const MAX_VARINT: u64 = 4_611_686_018_427_387_903;
pub const DEFAULT_QPACK_ENCODER_CAPACITY: u64 = 4_096;

pub open spec fn wire_value(value: int) -> bool {
    0 <= value && value <= MAX_VARINT as int
}

pub open spec fn current_default_max_field_section_size() -> int {
    MAX_VARINT as int
}

pub open spec fn current_default_qpack_decoder_capacity() -> int {
    0
}

pub open spec fn current_default_qpack_encoder_capacity() -> int {
    DEFAULT_QPACK_ENCODER_CAPACITY as int
}

pub open spec fn current_default_qpack_blocked_streams() -> int {
    0
}

proof fn current_wire_defaults_are_representable()
    ensures
        wire_value(current_default_max_field_section_size()),
        wire_value(current_default_qpack_decoder_capacity()),
        wire_value(current_default_qpack_encoder_capacity()),
        wire_value(current_default_qpack_blocked_streams()),
{
}

pub open spec fn v4_indexing_strategy_after_v3_upgrade() -> int {
    0
}

proof fn v3_upgrade_defaults_new_indexing_field()
    ensures
        v4_indexing_strategy_after_v3_upgrade() == 0,
{
}

} // verus!

fn main() {}
