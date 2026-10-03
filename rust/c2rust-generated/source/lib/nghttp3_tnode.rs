extern "C" {
    fn nghttp3_pq_push(
        pq: *mut nghttp3_pq,
        item: *mut nghttp3_pq_entry,
    ) -> ::core::ffi::c_int;
    fn nghttp3_pq_top(pq: *const nghttp3_pq) -> *mut nghttp3_pq_entry;
    fn nghttp3_pq_empty(pq: *const nghttp3_pq) -> ::core::ffi::c_int;
    fn nghttp3_pq_size(pq: *const nghttp3_pq) -> size_t;
    fn nghttp3_pq_remove(pq: *mut nghttp3_pq, item: *mut nghttp3_pq_entry);
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type nghttp3_malloc = Option<
    unsafe extern "C" fn(size_t, *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
>;
pub type nghttp3_free = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
>;
pub type nghttp3_calloc = Option<
    unsafe extern "C" fn(
        size_t,
        size_t,
        *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void,
>;
pub type nghttp3_realloc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        size_t,
        *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_mem {
    pub user_data: *mut ::core::ffi::c_void,
    pub malloc: nghttp3_malloc,
    pub free: nghttp3_free,
    pub calloc: nghttp3_calloc,
    pub realloc: nghttp3_realloc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_pq {
    pub q: *mut *mut nghttp3_pq_entry,
    pub mem: *const nghttp3_mem,
    pub length: size_t,
    pub capacity: size_t,
    pub less: nghttp3_pq_less,
}
pub type nghttp3_pq_less = Option<
    unsafe extern "C" fn(
        *const nghttp3_pq_entry,
        *const nghttp3_pq_entry,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_pq_entry {
    pub index: size_t,
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct nghttp3_pri(pub C2Rust_nghttp3_pri_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_nghttp3_pri_Inner {
    pub urgency: uint32_t,
    pub inc: uint8_t,
}
#[allow(dead_code, non_upper_case_globals)]
const C2Rust_nghttp3_pri_PADDING: usize = ::core::mem::size_of::<nghttp3_pri>()
    - ::core::mem::size_of::<C2Rust_nghttp3_pri_Inner>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_tnode {
    pub pe: nghttp3_pq_entry,
    pub id: int64_t,
    pub cycle: uint64_t,
    pub pri: nghttp3_pri,
}
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NGHTTP3_DEFAULT_URGENCY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NGHTTP3_PQ_BAD_INDEX: ::core::ffi::c_ulong = SIZE_MAX;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 53] = unsafe {
    ::core::mem::transmute::<
        [u8; 53],
        [::core::ffi::c_char; 53],
    >(*b"void tnode_unschedule(nghttp3_tnode *, nghttp3_pq *)\0")
};
#[inline]
unsafe extern "C" fn nghttp3_max_unsigned_long_int(
    mut a: ::core::ffi::c_ulong,
    mut b: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    return if a < b { b } else { a };
}
pub const NGHTTP3_STREAM_MIN_WRITELEN: ::core::ffi::c_int = 800 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn nghttp3_tnode_init(
    mut tnode: *mut nghttp3_tnode,
    mut id: int64_t,
) {
    *tnode = nghttp3_tnode {
        pe: nghttp3_pq_entry {
            index: NGHTTP3_PQ_BAD_INDEX as size_t,
        },
        id: id,
        cycle: 0,
        pri: nghttp3_pri(C2Rust_nghttp3_pri_Inner {
            urgency: NGHTTP3_DEFAULT_URGENCY as uint32_t,
            inc: 0,
        }),
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_tnode_free(mut tnode: *mut nghttp3_tnode) {}
unsafe extern "C" fn tnode_unschedule(
    mut tnode: *mut nghttp3_tnode,
    mut pq: *mut nghttp3_pq,
) {
    '_c2rust_label: {
        if (*tnode).pe.index != 18446744073709551615 as size_t {} else {
            __assert_fail(
                b"tnode->pe.index != NGHTTP3_PQ_BAD_INDEX\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_tnode.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                45 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    nghttp3_pq_remove(pq, &raw mut (*tnode).pe);
    (*tnode).pe.index = NGHTTP3_PQ_BAD_INDEX as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_tnode_unschedule(
    mut tnode: *mut nghttp3_tnode,
    mut pq: *mut nghttp3_pq,
) {
    if (*tnode).pe.index == NGHTTP3_PQ_BAD_INDEX as size_t {
        return;
    }
    tnode_unschedule(tnode, pq);
}
unsafe extern "C" fn pq_get_first_cycle(mut pq: *const nghttp3_pq) -> uint64_t {
    let mut top: *mut nghttp3_tnode = ::core::ptr::null_mut::<nghttp3_tnode>();
    if nghttp3_pq_empty(pq) != 0 {
        return 0 as uint64_t;
    }
    top = (nghttp3_pq_top(pq) as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_tnode;
    return (*top).cycle;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_tnode_schedule(
    mut tnode: *mut nghttp3_tnode,
    mut pq: *mut nghttp3_pq,
    mut nwrite: uint64_t,
) -> ::core::ffi::c_int {
    let mut penalty: uint64_t = nwrite
        .wrapping_div(NGHTTP3_STREAM_MIN_WRITELEN as uint64_t);
    if (*tnode).pe.index == NGHTTP3_PQ_BAD_INDEX as size_t {
        (*tnode).cycle = pq_get_first_cycle(pq)
            .wrapping_add(
                if nwrite == 0 as uint64_t || (*tnode).pri.0.inc == 0 {
                    0 as uint64_t
                } else {
                    nghttp3_max_unsigned_long_int(
                        1 as ::core::ffi::c_ulong,
                        penalty as ::core::ffi::c_ulong,
                    ) as uint64_t
                },
            );
    } else if nwrite > 0 as uint64_t {
        if (*tnode).pri.0.inc == 0 || nghttp3_pq_size(pq) == 1 as size_t {
            return 0 as ::core::ffi::c_int;
        }
        nghttp3_pq_remove(pq, &raw mut (*tnode).pe);
        (*tnode).pe.index = NGHTTP3_PQ_BAD_INDEX as size_t;
        (*tnode).cycle = ((*tnode).cycle as ::core::ffi::c_ulong)
            .wrapping_add(
                nghttp3_max_unsigned_long_int(
                    1 as ::core::ffi::c_ulong,
                    penalty as ::core::ffi::c_ulong,
                ),
            ) as uint64_t;
    } else {
        return 0 as ::core::ffi::c_int
    }
    return nghttp3_pq_push(pq, &raw mut (*tnode).pe);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_tnode_is_scheduled(
    mut tnode: *const nghttp3_tnode,
) -> ::core::ffi::c_int {
    return ((*tnode).pe.index != NGHTTP3_PQ_BAD_INDEX as size_t) as ::core::ffi::c_int;
}
