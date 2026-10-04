#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

/// Abstract ownership state for the temporary decoder name/value slots.
pub struct ReadOwners {
    pub name: bool,
    pub value: bool,
}

pub open spec fn take_value(pre: ReadOwners) -> ReadOwners
    recommends
        pre.value,
{
    ReadOwners {
        name: pre.name,
        value: false,
    }
}

pub open spec fn take_literal(pre: ReadOwners) -> ReadOwners
    recommends
        pre.name,
        pre.value,
{
    ReadOwners {
        name: false,
        value: false,
    }
}

pub open spec fn reset(_pre: ReadOwners) -> ReadOwners {
    ReadOwners {
        name: false,
        value: false,
    }
}

proof fn value_move_clears_only_value(pre: ReadOwners)
    requires
        pre.value,
    ensures
        !take_value(pre).value,
        take_value(pre).name == pre.name,
{
}

proof fn literal_move_clears_both(pre: ReadOwners)
    requires
        pre.name,
        pre.value,
    ensures
        !take_literal(pre).name,
        !take_literal(pre).value,
{
}

proof fn failed_publish_then_reset_cannot_restore_owner(pre: ReadOwners)
    requires
        pre.value,
    ensures
        !reset(take_value(pre)).name,
        !reset(take_value(pre)).value,
{
}

proof fn historical_literal_failure_path_has_no_stale_owner()
    ensures
        !reset(ReadOwners { name: false, value: false }).name,
        !reset(ReadOwners { name: false, value: false }).value,
{
}

} // verus!

fn main() {}
