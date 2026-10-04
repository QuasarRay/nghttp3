#![allow(unused_imports)]

use vstd::prelude::*;

verus! {

pub struct RequestCursor {
    pub len: int,
    pub offset: int,
}

pub open spec fn valid(cursor: RequestCursor) -> bool {
    0 <= cursor.offset && cursor.offset <= cursor.len
}

pub open spec fn remaining(cursor: RequestCursor) -> int
    recommends
        valid(cursor),
{
    cursor.len - cursor.offset
}

pub open spec fn can_advance(cursor: RequestCursor, consumed: int) -> bool {
    valid(cursor) && 0 <= consumed && consumed <= remaining(cursor)
}

pub open spec fn advance(cursor: RequestCursor, consumed: int) -> RequestCursor
    recommends
        can_advance(cursor, consumed),
{
    RequestCursor {
        len: cursor.len,
        offset: cursor.offset + consumed,
    }
}

proof fn successful_advance_preserves_bounds(cursor: RequestCursor, consumed: int)
    requires
        can_advance(cursor, consumed),
    ensures
        valid(advance(cursor, consumed)),
        remaining(advance(cursor, consumed)) == remaining(cursor) - consumed,
{
}

proof fn failed_advance_must_not_change_cursor(cursor: RequestCursor, consumed: int)
    requires
        valid(cursor),
        consumed > remaining(cursor),
    ensures
        !can_advance(cursor, consumed),
{
}

proof fn rewind_is_valid(cursor: RequestCursor)
    requires
        valid(cursor),
    ensures
        valid(RequestCursor { len: cursor.len, offset: 0 }),
        remaining(RequestCursor { len: cursor.len, offset: 0 }) == cursor.len,
{
}

} // verus!

fn main() {}
