#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

pub struct FrameState {
    pub owned_pending: bool,
    pub published: bool,
}

pub open spec fn pending() -> FrameState {
    FrameState {
        owned_pending: true,
        published: false,
    }
}

pub open spec fn admission_failure(_pre: FrameState) -> FrameState {
    FrameState {
        owned_pending: true,
        published: false,
    }
}

pub open spec fn admission_success(_pre: FrameState) -> FrameState {
    FrameState {
        owned_pending: false,
        published: true,
    }
}

pub open spec fn valid(state: FrameState) -> bool {
    !(state.owned_pending && state.published)
}

proof fn pending_is_single_owner()
    ensures
        valid(pending()),
        pending().owned_pending,
        !pending().published,
{
}

proof fn failure_preserves_unpublished_owner()
    ensures
        valid(admission_failure(pending())),
        admission_failure(pending()).owned_pending,
        !admission_failure(pending()).published,
{
}

proof fn success_transfers_owner_to_queue()
    ensures
        valid(admission_success(pending())),
        !admission_success(pending()).owned_pending,
        admission_success(pending()).published,
{
}

proof fn historical_leak_shape_is_not_required()
    ensures
        valid(FrameState {
            owned_pending: false,
            published: false,
        }),
{
}

} // verus!

fn main() {}
