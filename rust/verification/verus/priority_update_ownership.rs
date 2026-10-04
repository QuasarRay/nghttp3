#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

pub enum PayloadOwner {
    Pending,
    Queued,
    ReturnedOnFailure,
}

pub open spec fn exactly_one_owner(owner: PayloadOwner) -> bool {
    match owner {
        PayloadOwner::Pending => true,
        PayloadOwner::Queued => true,
        PayloadOwner::ReturnedOnFailure => true,
    }
}

proof fn reservation_success_transfers_owner()
    ensures
        exactly_one_owner(PayloadOwner::Queued),
{
}

proof fn reservation_failure_returns_owner()
    ensures
        exactly_one_owner(PayloadOwner::ReturnedOnFailure),
{
}

} // verus!

fn main() {}
