extern "C" {
    fn nghttp3_range_intersect(
        a: *const nghttp3_range,
        b: *const nghttp3_range,
    ) -> nghttp3_range;
    fn nghttp3_range_len(r: *const nghttp3_range) -> uint64_t;
    fn nghttp3_range_eq(
        a: *const nghttp3_range,
        b: *const nghttp3_range,
    ) -> ::core::ffi::c_int;
    fn nghttp3_range_cut(
        left: *mut nghttp3_range,
        right: *mut nghttp3_range,
        a: *const nghttp3_range,
        b: *const nghttp3_range,
    );
    fn nghttp3_ksl_init(
        ksl: *mut nghttp3_ksl,
        compar: nghttp3_ksl_compar,
        search: nghttp3_ksl_search,
        keylen: size_t,
        mem: *const nghttp3_mem,
    );
    fn nghttp3_ksl_free(ksl: *mut nghttp3_ksl);
    fn nghttp3_ksl_insert(
        ksl: *mut nghttp3_ksl,
        it: *mut nghttp3_ksl_it,
        key: *const ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn nghttp3_ksl_remove_hint(
        ksl: *mut nghttp3_ksl,
        it: *mut nghttp3_ksl_it,
        hint: *const nghttp3_ksl_it,
        key: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn nghttp3_ksl_lower_bound_search(
        ksl: *const nghttp3_ksl,
        key: *const ::core::ffi::c_void,
        search: nghttp3_ksl_search,
    ) -> nghttp3_ksl_it;
    fn nghttp3_ksl_update_key(
        ksl: *mut nghttp3_ksl,
        old_key: *const ::core::ffi::c_void,
        new_key: *const ::core::ffi::c_void,
    );
    fn nghttp3_ksl_begin(ksl: *const nghttp3_ksl) -> nghttp3_ksl_it;
    fn nghttp3_ksl_len(ksl: *const nghttp3_ksl) -> size_t;
    fn nghttp3_ksl_range_search(
        ksl: *const nghttp3_ksl,
        blk: *mut nghttp3_ksl_blk,
        key: *const ::core::ffi::c_void,
    ) -> size_t;
    fn nghttp3_ksl_range_exclusive_search(
        ksl: *const nghttp3_ksl,
        blk: *mut nghttp3_ksl_blk,
        key: *const ::core::ffi::c_void,
    ) -> size_t;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint16_t = u16;
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
pub struct nghttp3_buf {
    pub begin: *mut uint8_t,
    pub end: *mut uint8_t,
    pub pos: *mut uint8_t,
    pub last: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_memblock_hd {
    pub c2rust_unnamed: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub next: *mut nghttp3_memblock_hd,
    pub pad: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_balloc {
    pub mem: *const nghttp3_mem,
    pub blklen: size_t,
    pub head: *mut nghttp3_memblock_hd,
    pub buf: nghttp3_buf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_opl_entry {
    pub next: *mut nghttp3_opl_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_opl {
    pub head: *mut nghttp3_opl_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_objalloc {
    pub balloc: nghttp3_balloc,
    pub opl: nghttp3_opl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_range {
    pub begin: uint64_t,
    pub end: uint64_t,
}
pub type nghttp3_ksl_key = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_ksl_node {
    pub c2rust_unnamed: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub blk: *mut nghttp3_ksl_blk,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_ksl_blk {
    pub c2rust_unnamed: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub c2rust_unnamed: C2Rust_Unnamed_2,
    pub oplent: nghttp3_opl_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub next: *mut nghttp3_ksl_blk,
    pub prev: *mut nghttp3_ksl_blk,
    pub nodes: [nghttp3_ksl_node; 32],
    pub keys: *mut uint8_t,
    pub n: uint32_t,
    pub aligned_keylen: uint16_t,
    pub leaf: uint8_t,
}
pub type nghttp3_ksl_compar = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_ksl {
    pub blkalloc: nghttp3_objalloc,
    pub root: *mut nghttp3_ksl_blk,
    pub front: *mut nghttp3_ksl_blk,
    pub back: *mut nghttp3_ksl_blk,
    pub compar: nghttp3_ksl_compar,
    pub search: nghttp3_ksl_search,
    pub n: size_t,
    pub keylen: size_t,
    pub aligned_keylen: size_t,
}
pub type nghttp3_ksl_search = Option<
    unsafe extern "C" fn(
        *const nghttp3_ksl,
        *mut nghttp3_ksl_blk,
        *const ::core::ffi::c_void,
    ) -> size_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_ksl_it {
    pub blk: *mut nghttp3_ksl_blk,
    pub i: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_gaptr {
    pub gap: nghttp3_ksl,
    pub mem: *const nghttp3_mem,
}
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[inline]
unsafe extern "C" fn nghttp3_ksl_blk_nth_key(
    mut blk: *const nghttp3_ksl_blk,
    mut n: size_t,
) -> *const ::core::ffi::c_void {
    return (*blk)
        .c2rust_unnamed
        .c2rust_unnamed
        .keys
        .offset(
            n.wrapping_mul((*blk).c2rust_unnamed.c2rust_unnamed.aligned_keylen as size_t)
                as isize,
        ) as *const ::core::ffi::c_void;
}
#[inline]
unsafe extern "C" fn nghttp3_ksl_it_next(mut it: *mut nghttp3_ksl_it) {
    (*it).i = (*it).i.wrapping_add(1);
    if (*it).i == (*(*it).blk).c2rust_unnamed.c2rust_unnamed.n as size_t
        && !(*(*it).blk).c2rust_unnamed.c2rust_unnamed.next.is_null()
    {
        (*it).blk = (*(*it).blk).c2rust_unnamed.c2rust_unnamed.next;
        (*it).i = 0 as size_t;
    }
}
#[inline]
unsafe extern "C" fn nghttp3_ksl_it_end(
    mut it: *const nghttp3_ksl_it,
) -> ::core::ffi::c_int {
    return ((*(*it).blk).c2rust_unnamed.c2rust_unnamed.n as size_t == (*it).i
        && (*(*it).blk).c2rust_unnamed.c2rust_unnamed.next.is_null())
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn nghttp3_ksl_it_key(
    mut it: *const nghttp3_ksl_it,
) -> *const ::core::ffi::c_void {
    return nghttp3_ksl_blk_nth_key((*it).blk, (*it).i);
}
#[inline]
unsafe extern "C" fn nghttp3_ksl_range_compar(
    mut lhs: *const ::core::ffi::c_void,
    mut rhs: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *const nghttp3_range = lhs as *const nghttp3_range;
    let mut b: *const nghttp3_range = rhs as *const nghttp3_range;
    return ((*a).begin < (*b).begin) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_gaptr_init(
    mut gaptr: *mut nghttp3_gaptr,
    mut mem: *const nghttp3_mem,
) {
    nghttp3_ksl_init(
        &raw mut (*gaptr).gap,
        Some(
            nghttp3_ksl_range_compar
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            nghttp3_ksl_range_search
                as unsafe extern "C" fn(
                    *const nghttp3_ksl,
                    *mut nghttp3_ksl_blk,
                    *const ::core::ffi::c_void,
                ) -> size_t,
        ),
        ::core::mem::size_of::<nghttp3_range>(),
        mem,
    );
    (*gaptr).mem = mem;
}
unsafe extern "C" fn gaptr_gap_init(
    mut gaptr: *mut nghttp3_gaptr,
) -> ::core::ffi::c_int {
    static mut end: nghttp3_range = nghttp3_range {
        begin: 0,
        end: UINT64_MAX as uint64_t,
    };
    return nghttp3_ksl_insert(
        &raw mut (*gaptr).gap,
        ::core::ptr::null_mut::<nghttp3_ksl_it>(),
        &raw const end as *const ::core::ffi::c_void,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_gaptr_free(mut gaptr: *mut nghttp3_gaptr) {
    if gaptr.is_null() {
        return;
    }
    nghttp3_ksl_free(&raw mut (*gaptr).gap);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_gaptr_push(
    mut gaptr: *mut nghttp3_gaptr,
    mut offset: uint64_t,
    mut datalen: uint64_t,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    let mut k: nghttp3_range = nghttp3_range { begin: 0, end: 0 };
    let mut m: nghttp3_range = nghttp3_range { begin: 0, end: 0 };
    let mut l: nghttp3_range = nghttp3_range { begin: 0, end: 0 };
    let mut r: nghttp3_range = nghttp3_range { begin: 0, end: 0 };
    let mut q: nghttp3_range = nghttp3_range {
        begin: offset,
        end: offset.wrapping_add(datalen),
    };
    let mut it: nghttp3_ksl_it = nghttp3_ksl_it {
        blk: ::core::ptr::null_mut::<nghttp3_ksl_blk>(),
        i: 0,
    };
    if nghttp3_ksl_len(&raw mut (*gaptr).gap) == 0 as size_t {
        rv = gaptr_gap_init(gaptr);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
    }
    it = nghttp3_ksl_lower_bound_search(
        &raw mut (*gaptr).gap,
        &raw mut q as *const ::core::ffi::c_void,
        Some(
            nghttp3_ksl_range_exclusive_search
                as unsafe extern "C" fn(
                    *const nghttp3_ksl,
                    *mut nghttp3_ksl_blk,
                    *const ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    while nghttp3_ksl_it_end(&raw mut it) == 0 {
        k = *(nghttp3_ksl_it_key(&raw mut it) as *mut nghttp3_range);
        m = nghttp3_range_intersect(&raw mut q, &raw mut k);
        if nghttp3_range_len(&raw mut m) == 0 {
            break;
        }
        if nghttp3_range_eq(&raw mut k, &raw mut m) != 0 {
            nghttp3_ksl_remove_hint(
                &raw mut (*gaptr).gap,
                &raw mut it,
                &raw mut it,
                &raw mut k as *const ::core::ffi::c_void,
            );
        } else {
            nghttp3_range_cut(&raw mut l, &raw mut r, &raw mut k, &raw mut m);
            if nghttp3_range_len(&raw mut l) != 0 {
                nghttp3_ksl_update_key(
                    &raw mut (*gaptr).gap,
                    &raw mut k as *const ::core::ffi::c_void,
                    &raw mut l as *const ::core::ffi::c_void,
                );
                if nghttp3_range_len(&raw mut r) != 0 {
                    rv = nghttp3_ksl_insert(
                        &raw mut (*gaptr).gap,
                        &raw mut it,
                        &raw mut r as *const ::core::ffi::c_void,
                        NULL_0,
                    );
                    if rv != 0 as ::core::ffi::c_int {
                        return rv;
                    }
                }
            } else if nghttp3_range_len(&raw mut r) != 0 {
                nghttp3_ksl_update_key(
                    &raw mut (*gaptr).gap,
                    &raw mut k as *const ::core::ffi::c_void,
                    &raw mut r as *const ::core::ffi::c_void,
                );
            }
            nghttp3_ksl_it_next(&raw mut it);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_gaptr_first_gap_offset(
    mut gaptr: *const nghttp3_gaptr,
) -> uint64_t {
    let mut it: nghttp3_ksl_it = nghttp3_ksl_it {
        blk: ::core::ptr::null_mut::<nghttp3_ksl_blk>(),
        i: 0,
    };
    if nghttp3_ksl_len(&raw const (*gaptr).gap) == 0 as size_t {
        return 0 as uint64_t;
    }
    it = nghttp3_ksl_begin(&raw const (*gaptr).gap);
    return (*(nghttp3_ksl_it_key(&raw mut it) as *mut nghttp3_range)).begin;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_gaptr_get_first_gap_after(
    mut gaptr: *const nghttp3_gaptr,
    mut offset: uint64_t,
) -> nghttp3_range {
    let mut it: nghttp3_ksl_it = nghttp3_ksl_it {
        blk: ::core::ptr::null_mut::<nghttp3_ksl_blk>(),
        i: 0,
    };
    if nghttp3_ksl_len(&raw const (*gaptr).gap) == 0 as size_t {
        let mut r: nghttp3_range = nghttp3_range {
            begin: 0,
            end: UINT64_MAX as uint64_t,
        };
        return r;
    }
    let mut c2rust_lvalue: nghttp3_range = nghttp3_range {
        begin: offset,
        end: offset.wrapping_add(1 as uint64_t),
    };
    it = nghttp3_ksl_lower_bound_search(
        &raw const (*gaptr).gap,
        &raw mut c2rust_lvalue as *const ::core::ffi::c_void,
        Some(
            nghttp3_ksl_range_exclusive_search
                as unsafe extern "C" fn(
                    *const nghttp3_ksl,
                    *mut nghttp3_ksl_blk,
                    *const ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    '_c2rust_label: {
        if nghttp3_ksl_it_end(&raw mut it) == 0 {} else {
            __assert_fail(
                b"!nghttp3_ksl_it_end(&it)\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_gaptr.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                137 as ::core::ffi::c_uint,
                b"nghttp3_range nghttp3_gaptr_get_first_gap_after(const nghttp3_gaptr *, uint64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    return *(nghttp3_ksl_it_key(&raw mut it) as *mut nghttp3_range);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_gaptr_is_pushed(
    mut gaptr: *const nghttp3_gaptr,
    mut offset: uint64_t,
    mut datalen: uint64_t,
) -> ::core::ffi::c_int {
    let mut q: nghttp3_range = nghttp3_range {
        begin: offset,
        end: offset.wrapping_add(datalen),
    };
    let mut it: nghttp3_ksl_it = nghttp3_ksl_it {
        blk: ::core::ptr::null_mut::<nghttp3_ksl_blk>(),
        i: 0,
    };
    let mut m: nghttp3_range = nghttp3_range { begin: 0, end: 0 };
    if nghttp3_ksl_len(&raw const (*gaptr).gap) == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    it = nghttp3_ksl_lower_bound_search(
        &raw const (*gaptr).gap,
        &raw mut q as *const ::core::ffi::c_void,
        Some(
            nghttp3_ksl_range_exclusive_search
                as unsafe extern "C" fn(
                    *const nghttp3_ksl,
                    *mut nghttp3_ksl_blk,
                    *const ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    '_c2rust_label: {
        if nghttp3_ksl_it_end(&raw mut it) == 0 {} else {
            __assert_fail(
                b"!nghttp3_ksl_it_end(&it)\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_gaptr.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                158 as ::core::ffi::c_uint,
                b"int nghttp3_gaptr_is_pushed(const nghttp3_gaptr *, uint64_t, uint64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    m = nghttp3_range_intersect(
        &raw mut q,
        nghttp3_ksl_it_key(&raw mut it) as *mut nghttp3_range,
    );
    return (nghttp3_range_len(&raw mut m) == 0 as uint64_t) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_gaptr_drop_first_gap(mut gaptr: *mut nghttp3_gaptr) {
    let mut it: nghttp3_ksl_it = nghttp3_ksl_it {
        blk: ::core::ptr::null_mut::<nghttp3_ksl_blk>(),
        i: 0,
    };
    let mut r: nghttp3_range = nghttp3_range { begin: 0, end: 0 };
    if nghttp3_ksl_len(&raw mut (*gaptr).gap) == 0 as size_t {
        return;
    }
    it = nghttp3_ksl_begin(&raw mut (*gaptr).gap);
    '_c2rust_label: {
        if nghttp3_ksl_it_end(&raw mut it) == 0 {} else {
            __assert_fail(
                b"!nghttp3_ksl_it_end(&it)\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_gaptr.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                175 as ::core::ffi::c_uint,
                b"void nghttp3_gaptr_drop_first_gap(nghttp3_gaptr *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    r = *(nghttp3_ksl_it_key(&raw mut it) as *mut nghttp3_range);
    nghttp3_ksl_remove_hint(
        &raw mut (*gaptr).gap,
        ::core::ptr::null_mut::<nghttp3_ksl_it>(),
        &raw mut it,
        &raw mut r as *const ::core::ffi::c_void,
    );
}
