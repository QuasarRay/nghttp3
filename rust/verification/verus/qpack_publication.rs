#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

pub struct PublicationState {
    pub registered: bool,
    pub published: bool,
}

pub open spec fn valid(state: PublicationState) -> bool {
    state.published ==> state.registered
}

proof fn failed_registration_cannot_publish()
    ensures
        valid(PublicationState {
            registered: false,
            published: false,
        }),
{
}

proof fn successful_registration_may_publish()
    ensures
        valid(PublicationState {
            registered: true,
            published: true,
        }),
{
}

proof fn published_implies_registered(state: PublicationState)
    requires
        valid(state),
        state.published,
    ensures
        state.registered,
{
}

} // verus!

fn main() {}
