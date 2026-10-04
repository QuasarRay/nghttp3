#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

/// Abstract typestate for one QPACK stream reference.
pub struct RefState {
    pub registered: bool,
    pub published: bool,
}

pub open spec fn pending() -> RefState {
    RefState {
        registered: false,
        published: false,
    }
}

pub open spec fn registration_success(_pre: RefState) -> RefState {
    RefState {
        registered: true,
        published: false,
    }
}

pub open spec fn registration_failure(pre: RefState) -> RefState {
    pre
}

pub open spec fn publish(pre: RefState) -> RefState
    recommends
        pre.registered,
        !pre.published,
{
    RefState {
        registered: true,
        published: true,
    }
}

pub open spec fn valid(state: RefState) -> bool {
    state.published ==> state.registered
}

proof fn pending_is_valid()
    ensures
        valid(pending()),
        !pending().registered,
        !pending().published,
{
}

proof fn failed_registration_cannot_publish()
    ensures
        !registration_failure(pending()).registered,
        !registration_failure(pending()).published,
        valid(registration_failure(pending())),
{
}

proof fn registration_then_publish_is_valid()
    ensures
        valid(publish(registration_success(pending()))),
        publish(registration_success(pending())).registered,
        publish(registration_success(pending())).published,
{
}

proof fn historical_bad_state_is_invalid()
    ensures
        !valid(RefState {
            registered: false,
            published: true,
        }),
{
}

} // verus!

fn main() {}
