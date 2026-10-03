extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn nghttp3_balloc_get(
        balloc: *mut nghttp3_balloc,
        pbuf: *mut *mut ::core::ffi::c_void,
        n: size_t,
    ) -> ::core::ffi::c_int;
    fn nghttp3_opl_push(opl: *mut nghttp3_opl, ent: *mut nghttp3_opl_entry);
    fn nghttp3_opl_pop(opl: *mut nghttp3_opl) -> *mut nghttp3_opl_entry;
    fn nghttp3_objalloc_init(
        objalloc: *mut nghttp3_objalloc,
        blklen: size_t,
        mem: *const nghttp3_mem,
    );
    fn nghttp3_objalloc_free(objalloc: *mut nghttp3_objalloc);
    fn nghttp3_objalloc_clear(objalloc: *mut nghttp3_objalloc);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type int64_t = i64;
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
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NGHTTP3_ERR_INVALID_ARGUMENT: ::core::ffi::c_int = -101 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_NOMEM: ::core::ffi::c_int = -901 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn nghttp3_max_unsigned_long_int(
    mut a: ::core::ffi::c_ulong,
    mut b: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    return if a < b { b } else { a };
}
#[inline]
unsafe extern "C" fn nghttp3_min_unsigned_long_int(
    mut a: ::core::ffi::c_ulong,
    mut b: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    return if a < b { a } else { b };
}
pub const NGHTTP3_KSL_DEGR: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NGHTTP3_KSL_MAX_NBLK: ::core::ffi::c_int = 2 as ::core::ffi::c_int
    * NGHTTP3_KSL_DEGR;
pub const NGHTTP3_KSL_MIN_NBLK: ::core::ffi::c_int = NGHTTP3_KSL_DEGR;
#[inline]
unsafe extern "C" fn nghttp3_objalloc_ksl_blk_release(
    mut objalloc: *mut nghttp3_objalloc,
    mut obj: *mut nghttp3_ksl_blk,
) {
    nghttp3_opl_push(&raw mut (*objalloc).opl, &raw mut (*obj).c2rust_unnamed.oplent);
}
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
unsafe extern "C" fn nghttp3_ksl_range_compar(
    mut lhs: *const ::core::ffi::c_void,
    mut rhs: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *const nghttp3_range = lhs as *const nghttp3_range;
    let mut b: *const nghttp3_range = rhs as *const nghttp3_range;
    return ((*a).begin < (*b).begin) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn nghttp3_ksl_range_exclusive_compar(
    mut lhs: *const ::core::ffi::c_void,
    mut rhs: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *const nghttp3_range = lhs as *const nghttp3_range;
    let mut b: *const nghttp3_range = rhs as *const nghttp3_range;
    return ((*a).begin < (*b).begin
        && !(nghttp3_max_unsigned_long_int(
            (*a).begin as ::core::ffi::c_ulong,
            (*b).begin as ::core::ffi::c_ulong,
        )
            < nghttp3_min_unsigned_long_int(
                (*a).end as ::core::ffi::c_ulong,
                (*b).end as ::core::ffi::c_ulong,
            ))) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn nghttp3_ksl_uint64_less(
    mut lhs: *const ::core::ffi::c_void,
    mut rhs: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(lhs as *const uint64_t) < *(rhs as *const uint64_t)) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn nghttp3_ksl_int64_greater(
    mut lhs: *const ::core::ffi::c_void,
    mut rhs: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(lhs as *const int64_t) > *(rhs as *const int64_t)) as ::core::ffi::c_int;
}
static mut null_blk: nghttp3_ksl_blk = nghttp3_ksl_blk {
    c2rust_unnamed: C2Rust_Unnamed_1 {
        c2rust_unnamed: C2Rust_Unnamed_2 {
            next: ::core::ptr::null_mut::<nghttp3_ksl_blk>(),
            prev: ::core::ptr::null_mut::<nghttp3_ksl_blk>(),
            nodes: [nghttp3_ksl_node {
                c2rust_unnamed: C2Rust_Unnamed_0 {
                    blk: ::core::ptr::null_mut::<nghttp3_ksl_blk>(),
                },
            }; 32],
            keys: ::core::ptr::null_mut::<uint8_t>(),
            n: 0,
            aligned_keylen: 0,
            leaf: 0,
        },
    },
};
#[no_mangle]
pub unsafe extern "C" fn nghttp3_objalloc_ksl_blk_len_get(
    mut objalloc: *mut nghttp3_objalloc,
    mut len: size_t,
) -> *mut nghttp3_ksl_blk {
    let mut oplent: *mut nghttp3_opl_entry = nghttp3_opl_pop(&raw mut (*objalloc).opl);
    let mut obj: *mut nghttp3_ksl_blk = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    let mut rv: ::core::ffi::c_int = 0;
    if oplent.is_null() {
        rv = nghttp3_balloc_get(
            &raw mut (*objalloc).balloc,
            &raw mut obj as *mut *mut ::core::ffi::c_void,
            len,
        );
        if rv != 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<nghttp3_ksl_blk>();
        }
        return obj;
    }
    return (oplent as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_ksl_blk;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_objalloc_ksl_blk_get(
    mut objalloc: *mut nghttp3_objalloc,
) -> *mut nghttp3_ksl_blk {
    let mut oplent: *mut nghttp3_opl_entry = nghttp3_opl_pop(&raw mut (*objalloc).opl);
    let mut obj: *mut nghttp3_ksl_blk = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    let mut rv: ::core::ffi::c_int = 0;
    if oplent.is_null() {
        rv = nghttp3_balloc_get(
            &raw mut (*objalloc).balloc,
            &raw mut obj as *mut *mut ::core::ffi::c_void,
            ::core::mem::size_of::<nghttp3_ksl_blk>(),
        );
        if rv != 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<nghttp3_ksl_blk>();
        }
        return obj;
    }
    return (oplent as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_ksl_blk;
}
pub const NGHTTP3_KSL_ALIGNED_BLKLEN: usize = ::core::mem::size_of::<nghttp3_ksl_blk>()
    .wrapping_add(0x7usize) & !(0x7 as ::core::ffi::c_uint as usize);
unsafe extern "C" fn ksl_blklen(mut aligned_keylen: size_t) -> size_t {
    return NGHTTP3_KSL_ALIGNED_BLKLEN
        .wrapping_add((NGHTTP3_KSL_MAX_NBLK as size_t).wrapping_mul(aligned_keylen));
}
unsafe extern "C" fn ksl_set_nth_key(
    mut ksl: *const nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut n: size_t,
    mut key: *const ::core::ffi::c_void,
) {
    memcpy(
        (*blk)
            .c2rust_unnamed
            .c2rust_unnamed
            .keys
            .offset(n.wrapping_mul((*ksl).aligned_keylen) as isize)
            as *mut ::core::ffi::c_void,
        key,
        (*ksl).keylen,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_init(
    mut ksl: *mut nghttp3_ksl,
    mut compar: nghttp3_ksl_compar,
    mut search: nghttp3_ksl_search,
    mut keylen: size_t,
    mut mem: *const nghttp3_mem,
) {
    let mut aligned_keylen: size_t = 0;
    '_c2rust_label: {
        if keylen >= ::core::mem::size_of::<uint64_t>() {} else {
            __assert_fail(
                b"keylen >= sizeof(uint64_t)\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                60 as ::core::ffi::c_uint,
                b"void nghttp3_ksl_init(nghttp3_ksl *, nghttp3_ksl_compar, nghttp3_ksl_search, size_t, const nghttp3_mem *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    aligned_keylen = keylen.wrapping_add(0x7 as size_t)
        & !(0x7 as ::core::ffi::c_uint as size_t);
    '_c2rust_label_0: {
        if aligned_keylen <= 65535 as size_t {} else {
            __assert_fail(
                b"aligned_keylen <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                64 as ::core::ffi::c_uint,
                b"void nghttp3_ksl_init(nghttp3_ksl *, nghttp3_ksl_compar, nghttp3_ksl_search, size_t, const nghttp3_mem *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    nghttp3_objalloc_init(
        &raw mut (*ksl).blkalloc,
        ksl_blklen(aligned_keylen).wrapping_add(0xf as size_t)
            & !(0xf as ::core::ffi::c_uint as size_t),
        mem,
    );
    (*ksl).root = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    (*ksl).back = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    (*ksl).front = (*ksl).back;
    (*ksl).compar = compar;
    (*ksl).search = search;
    (*ksl).n = 0 as size_t;
    (*ksl).keylen = keylen;
    (*ksl).aligned_keylen = aligned_keylen;
}
unsafe extern "C" fn ksl_blk_objalloc_new(
    mut ksl: *mut nghttp3_ksl,
) -> *mut nghttp3_ksl_blk {
    let mut blk: *mut nghttp3_ksl_blk = nghttp3_objalloc_ksl_blk_len_get(
        &raw mut (*ksl).blkalloc,
        ksl_blklen((*ksl).aligned_keylen),
    );
    if blk.is_null() {
        return ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    }
    (*blk).c2rust_unnamed.c2rust_unnamed.keys = (blk as *mut uint8_t)
        .offset(NGHTTP3_KSL_ALIGNED_BLKLEN as isize);
    (*blk).c2rust_unnamed.c2rust_unnamed.aligned_keylen = (*ksl).aligned_keylen
        as uint16_t;
    return blk;
}
unsafe extern "C" fn ksl_blk_objalloc_del(
    mut ksl: *mut nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
) {
    nghttp3_objalloc_ksl_blk_release(&raw mut (*ksl).blkalloc, blk);
}
unsafe extern "C" fn ksl_root_init(mut ksl: *mut nghttp3_ksl) -> ::core::ffi::c_int {
    let mut root: *mut nghttp3_ksl_blk = ksl_blk_objalloc_new(ksl);
    if root.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    (*root).c2rust_unnamed.c2rust_unnamed.prev = ::core::ptr::null_mut::<
        nghttp3_ksl_blk,
    >();
    (*root).c2rust_unnamed.c2rust_unnamed.next = (*root)
        .c2rust_unnamed
        .c2rust_unnamed
        .prev;
    (*root).c2rust_unnamed.c2rust_unnamed.n = 0 as uint32_t;
    (*root).c2rust_unnamed.c2rust_unnamed.leaf = 1 as uint8_t;
    (*ksl).root = root;
    (*ksl).back = root;
    (*ksl).front = (*ksl).back;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_free(mut ksl: *mut nghttp3_ksl) {
    if ksl.is_null() || (*ksl).root.is_null() {
        return;
    }
    nghttp3_objalloc_free(&raw mut (*ksl).blkalloc);
}
unsafe extern "C" fn ksl_split_blk(
    mut ksl: *mut nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
) -> *mut nghttp3_ksl_blk {
    let mut rblk: *mut nghttp3_ksl_blk = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    rblk = ksl_blk_objalloc_new(ksl);
    if rblk.is_null() {
        return ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    }
    (*rblk).c2rust_unnamed.c2rust_unnamed.next = (*blk)
        .c2rust_unnamed
        .c2rust_unnamed
        .next;
    (*blk).c2rust_unnamed.c2rust_unnamed.next = rblk;
    if !(*rblk).c2rust_unnamed.c2rust_unnamed.next.is_null() {
        (*(*rblk).c2rust_unnamed.c2rust_unnamed.next)
            .c2rust_unnamed
            .c2rust_unnamed
            .prev = rblk;
    } else if (*ksl).back == blk {
        (*ksl).back = rblk;
    }
    (*rblk).c2rust_unnamed.c2rust_unnamed.prev = blk;
    (*rblk).c2rust_unnamed.c2rust_unnamed.leaf = (*blk)
        .c2rust_unnamed
        .c2rust_unnamed
        .leaf;
    (*rblk).c2rust_unnamed.c2rust_unnamed.n = (*blk)
        .c2rust_unnamed
        .c2rust_unnamed
        .n
        .wrapping_div(2 as uint32_t);
    (*blk).c2rust_unnamed.c2rust_unnamed.n = (*blk)
        .c2rust_unnamed
        .c2rust_unnamed
        .n
        .wrapping_sub((*rblk).c2rust_unnamed.c2rust_unnamed.n);
    memcpy(
        &raw mut (*rblk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node
            as *mut ::core::ffi::c_void,
        (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node)
            .offset((*blk).c2rust_unnamed.c2rust_unnamed.n as isize)
            as *const ::core::ffi::c_void,
        ((*rblk).c2rust_unnamed.c2rust_unnamed.n as size_t)
            .wrapping_mul(::core::mem::size_of::<nghttp3_ksl_node>()),
    );
    memcpy(
        (*rblk).c2rust_unnamed.c2rust_unnamed.keys as *mut ::core::ffi::c_void,
        (*blk)
            .c2rust_unnamed
            .c2rust_unnamed
            .keys
            .offset(
                ((*blk).c2rust_unnamed.c2rust_unnamed.n as size_t)
                    .wrapping_mul((*ksl).aligned_keylen) as isize,
            ) as *const ::core::ffi::c_void,
        ((*rblk).c2rust_unnamed.c2rust_unnamed.n as size_t)
            .wrapping_mul((*ksl).aligned_keylen),
    );
    '_c2rust_label: {
        if (*blk).c2rust_unnamed.c2rust_unnamed.n >= 16 as uint32_t {} else {
            __assert_fail(
                b"blk->n >= NGHTTP3_KSL_MIN_NBLK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                178 as ::core::ffi::c_uint,
                b"nghttp3_ksl_blk *ksl_split_blk(nghttp3_ksl *, nghttp3_ksl_blk *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*rblk).c2rust_unnamed.c2rust_unnamed.n >= 16 as uint32_t {} else {
            __assert_fail(
                b"rblk->n >= NGHTTP3_KSL_MIN_NBLK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                179 as ::core::ffi::c_uint,
                b"nghttp3_ksl_blk *ksl_split_blk(nghttp3_ksl *, nghttp3_ksl_blk *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    return rblk;
}
unsafe extern "C" fn ksl_split_node(
    mut ksl: *mut nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut i: size_t,
) -> ::core::ffi::c_int {
    let mut lblk: *mut nghttp3_ksl_blk = (*blk)
        .c2rust_unnamed
        .c2rust_unnamed
        .nodes[i]
        .c2rust_unnamed
        .blk;
    let mut rblk: *mut nghttp3_ksl_blk = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    rblk = ksl_split_blk(ksl, lblk);
    if rblk.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    memmove(
        (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node)
            .offset(i.wrapping_add(2 as size_t) as isize) as *mut ::core::ffi::c_void,
        (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node)
            .offset(i.wrapping_add(1 as size_t) as isize) as *const ::core::ffi::c_void,
        ((*blk).c2rust_unnamed.c2rust_unnamed.n as size_t)
            .wrapping_sub(i.wrapping_add(1 as size_t))
            .wrapping_mul(::core::mem::size_of::<nghttp3_ksl_node>()),
    );
    memmove(
        (*blk)
            .c2rust_unnamed
            .c2rust_unnamed
            .keys
            .offset(
                i.wrapping_add(1 as size_t).wrapping_mul((*ksl).aligned_keylen) as isize,
            ) as *mut ::core::ffi::c_void,
        (*blk)
            .c2rust_unnamed
            .c2rust_unnamed
            .keys
            .offset(i.wrapping_mul((*ksl).aligned_keylen) as isize)
            as *const ::core::ffi::c_void,
        ((*blk).c2rust_unnamed.c2rust_unnamed.n as size_t)
            .wrapping_sub(i)
            .wrapping_mul((*ksl).aligned_keylen),
    );
    (*blk)
        .c2rust_unnamed
        .c2rust_unnamed
        .nodes[i.wrapping_add(1 as size_t)]
        .c2rust_unnamed
        .blk = rblk;
    (*blk).c2rust_unnamed.c2rust_unnamed.n = (*blk)
        .c2rust_unnamed
        .c2rust_unnamed
        .n
        .wrapping_add(1);
    ksl_set_nth_key(
        ksl,
        blk,
        i,
        nghttp3_ksl_blk_nth_key(
            lblk,
            (*lblk).c2rust_unnamed.c2rust_unnamed.n.wrapping_sub(1 as uint32_t) as size_t,
        ),
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ksl_split_root(mut ksl: *mut nghttp3_ksl) -> ::core::ffi::c_int {
    let mut rblk: *mut nghttp3_ksl_blk = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    let mut lblk: *mut nghttp3_ksl_blk = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    let mut nroot: *mut nghttp3_ksl_blk = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    nroot = ksl_blk_objalloc_new(ksl);
    if nroot.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    rblk = ksl_split_blk(ksl, (*ksl).root);
    if rblk.is_null() {
        ksl_blk_objalloc_del(ksl, nroot);
        return NGHTTP3_ERR_NOMEM;
    }
    lblk = (*ksl).root;
    (*nroot).c2rust_unnamed.c2rust_unnamed.prev = ::core::ptr::null_mut::<
        nghttp3_ksl_blk,
    >();
    (*nroot).c2rust_unnamed.c2rust_unnamed.next = (*nroot)
        .c2rust_unnamed
        .c2rust_unnamed
        .prev;
    (*nroot).c2rust_unnamed.c2rust_unnamed.n = 2 as uint32_t;
    (*nroot).c2rust_unnamed.c2rust_unnamed.leaf = 0 as uint8_t;
    ksl_set_nth_key(
        ksl,
        nroot,
        0 as size_t,
        nghttp3_ksl_blk_nth_key(
            lblk,
            (*lblk).c2rust_unnamed.c2rust_unnamed.n.wrapping_sub(1 as uint32_t) as size_t,
        ),
    );
    (*nroot).c2rust_unnamed.c2rust_unnamed.nodes[0usize].c2rust_unnamed.blk = lblk;
    ksl_set_nth_key(
        ksl,
        nroot,
        1 as size_t,
        nghttp3_ksl_blk_nth_key(
            rblk,
            (*rblk).c2rust_unnamed.c2rust_unnamed.n.wrapping_sub(1 as uint32_t) as size_t,
        ),
    );
    (*nroot).c2rust_unnamed.c2rust_unnamed.nodes[1usize].c2rust_unnamed.blk = rblk;
    (*ksl).root = nroot;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ksl_insert_node(
    mut ksl: *mut nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut i: size_t,
    mut key: *const ::core::ffi::c_void,
    mut data: *mut ::core::ffi::c_void,
) {
    '_c2rust_label: {
        if (*blk).c2rust_unnamed.c2rust_unnamed.n
            < (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint32_t
        {} else {
            __assert_fail(
                b"blk->n < NGHTTP3_KSL_MAX_NBLK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                267 as ::core::ffi::c_uint,
                b"void ksl_insert_node(nghttp3_ksl *, nghttp3_ksl_blk *, size_t, const nghttp3_ksl_key *, void *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    memmove(
        (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node)
            .offset(i.wrapping_add(1 as size_t) as isize) as *mut ::core::ffi::c_void,
        (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node)
            .offset(i as isize) as *const ::core::ffi::c_void,
        ((*blk).c2rust_unnamed.c2rust_unnamed.n as size_t)
            .wrapping_sub(i)
            .wrapping_mul(::core::mem::size_of::<nghttp3_ksl_node>()),
    );
    memmove(
        (*blk)
            .c2rust_unnamed
            .c2rust_unnamed
            .keys
            .offset(
                i.wrapping_add(1 as size_t).wrapping_mul((*ksl).aligned_keylen) as isize,
            ) as *mut ::core::ffi::c_void,
        (*blk)
            .c2rust_unnamed
            .c2rust_unnamed
            .keys
            .offset(i.wrapping_mul((*ksl).aligned_keylen) as isize)
            as *const ::core::ffi::c_void,
        ((*blk).c2rust_unnamed.c2rust_unnamed.n as size_t)
            .wrapping_sub(i)
            .wrapping_mul((*ksl).aligned_keylen),
    );
    ksl_set_nth_key(ksl, blk, i, key);
    (*blk).c2rust_unnamed.c2rust_unnamed.nodes[i].c2rust_unnamed.data = data;
    (*blk).c2rust_unnamed.c2rust_unnamed.n = (*blk)
        .c2rust_unnamed
        .c2rust_unnamed
        .n
        .wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_insert(
    mut ksl: *mut nghttp3_ksl,
    mut it: *mut nghttp3_ksl_it,
    mut key: *const ::core::ffi::c_void,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut blk: *mut nghttp3_ksl_blk = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    let mut node: *mut nghttp3_ksl_node = ::core::ptr::null_mut::<nghttp3_ksl_node>();
    let mut i: size_t = 0;
    let mut rv: ::core::ffi::c_int = 0;
    if (*ksl).root.is_null() {
        rv = ksl_root_init(ksl);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
    }
    if (*(*ksl).root).c2rust_unnamed.c2rust_unnamed.n == NGHTTP3_KSL_MAX_NBLK as uint32_t
    {
        rv = ksl_split_root(ksl);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
    }
    blk = (*ksl).root;
    loop {
        i = (*ksl).search.expect("non-null function pointer")(ksl, blk, key);
        if (*blk).c2rust_unnamed.c2rust_unnamed.leaf != 0 {
            if i < (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t
                && (*ksl)
                    .compar
                    .expect(
                        "non-null function pointer",
                    )(key, nghttp3_ksl_blk_nth_key(blk, i)) == 0
            {
                if !it.is_null() {
                    *it = nghttp3_ksl_end(ksl);
                }
                return NGHTTP3_ERR_INVALID_ARGUMENT;
            }
            ksl_insert_node(ksl, blk, i, key, data);
            (*ksl).n = (*ksl).n.wrapping_add(1);
            if !it.is_null() {
                nghttp3_ksl_it_init(it, blk, i);
            }
            return 0 as ::core::ffi::c_int;
        }
        if i == (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t {
            while (*blk).c2rust_unnamed.c2rust_unnamed.leaf == 0 {
                node = (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes
                    as *mut nghttp3_ksl_node)
                    .offset(
                        (*blk)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .n
                            .wrapping_sub(1 as uint32_t) as isize,
                    );
                if (*(*node).c2rust_unnamed.blk).c2rust_unnamed.c2rust_unnamed.n
                    == NGHTTP3_KSL_MAX_NBLK as uint32_t
                {
                    rv = ksl_split_node(
                        ksl,
                        blk,
                        (*blk)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .n
                            .wrapping_sub(1 as uint32_t) as size_t,
                    );
                    if rv != 0 as ::core::ffi::c_int {
                        return rv;
                    }
                    node = (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes
                        as *mut nghttp3_ksl_node)
                        .offset(
                            (*blk)
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .n
                                .wrapping_sub(1 as uint32_t) as isize,
                        );
                }
                ksl_set_nth_key(
                    ksl,
                    blk,
                    (*blk).c2rust_unnamed.c2rust_unnamed.n.wrapping_sub(1 as uint32_t)
                        as size_t,
                    key,
                );
                blk = (*node).c2rust_unnamed.blk;
            }
            ksl_insert_node(
                ksl,
                blk,
                (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t,
                key,
                data,
            );
            (*ksl).n = (*ksl).n.wrapping_add(1);
            if !it.is_null() {
                nghttp3_ksl_it_init(
                    it,
                    blk,
                    (*blk).c2rust_unnamed.c2rust_unnamed.n.wrapping_sub(1 as uint32_t)
                        as size_t,
                );
            }
            return 0 as ::core::ffi::c_int;
        }
        node = (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes
            as *mut nghttp3_ksl_node)
            .offset(i as isize);
        if (*(*node).c2rust_unnamed.blk).c2rust_unnamed.c2rust_unnamed.n
            == NGHTTP3_KSL_MAX_NBLK as uint32_t
        {
            rv = ksl_split_node(ksl, blk, i);
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
            if (*ksl)
                .compar
                .expect(
                    "non-null function pointer",
                )(nghttp3_ksl_blk_nth_key(blk, i), key) != 0
            {
                node = (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes
                    as *mut nghttp3_ksl_node)
                    .offset(i.wrapping_add(1 as size_t) as isize);
            }
        }
        blk = (*node).c2rust_unnamed.blk;
    };
}
unsafe extern "C" fn ksl_remove_node(
    mut ksl: *mut nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut i: size_t,
) {
    memmove(
        (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node)
            .offset(i as isize) as *mut ::core::ffi::c_void,
        (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node)
            .offset(i.wrapping_add(1 as size_t) as isize) as *const ::core::ffi::c_void,
        ((*blk).c2rust_unnamed.c2rust_unnamed.n as size_t)
            .wrapping_sub(i.wrapping_add(1 as size_t))
            .wrapping_mul(::core::mem::size_of::<nghttp3_ksl_node>()),
    );
    memmove(
        (*blk)
            .c2rust_unnamed
            .c2rust_unnamed
            .keys
            .offset(i.wrapping_mul((*ksl).aligned_keylen) as isize)
            as *mut ::core::ffi::c_void,
        (*blk)
            .c2rust_unnamed
            .c2rust_unnamed
            .keys
            .offset(
                i.wrapping_add(1 as size_t).wrapping_mul((*ksl).aligned_keylen) as isize,
            ) as *const ::core::ffi::c_void,
        ((*blk).c2rust_unnamed.c2rust_unnamed.n as size_t)
            .wrapping_sub(i.wrapping_add(1 as size_t))
            .wrapping_mul((*ksl).aligned_keylen),
    );
    (*blk).c2rust_unnamed.c2rust_unnamed.n = (*blk)
        .c2rust_unnamed
        .c2rust_unnamed
        .n
        .wrapping_sub(1);
}
unsafe extern "C" fn ksl_merge_node(
    mut ksl: *mut nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut i: size_t,
) -> *mut nghttp3_ksl_blk {
    let mut lnode: *mut nghttp3_ksl_node = ::core::ptr::null_mut::<nghttp3_ksl_node>();
    let mut lblk: *mut nghttp3_ksl_blk = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    let mut rblk: *mut nghttp3_ksl_blk = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    '_c2rust_label: {
        if i.wrapping_add(1 as size_t) < (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t
        {} else {
            __assert_fail(
                b"i + 1 < blk->n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                401 as ::core::ffi::c_uint,
                b"nghttp3_ksl_blk *ksl_merge_node(nghttp3_ksl *, nghttp3_ksl_blk *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    lnode = (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes
        as *mut nghttp3_ksl_node)
        .offset(i as isize);
    lblk = (*lnode).c2rust_unnamed.blk;
    rblk = (*blk)
        .c2rust_unnamed
        .c2rust_unnamed
        .nodes[i.wrapping_add(1 as size_t)]
        .c2rust_unnamed
        .blk;
    '_c2rust_label_0: {
        if (*lblk)
            .c2rust_unnamed
            .c2rust_unnamed
            .n
            .wrapping_add((*rblk).c2rust_unnamed.c2rust_unnamed.n)
            <= (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint32_t
        {} else {
            __assert_fail(
                b"lblk->n + rblk->n <= NGHTTP3_KSL_MAX_NBLK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                408 as ::core::ffi::c_uint,
                b"nghttp3_ksl_blk *ksl_merge_node(nghttp3_ksl *, nghttp3_ksl_blk *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    memcpy(
        (&raw mut (*lblk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node)
            .offset((*lblk).c2rust_unnamed.c2rust_unnamed.n as isize)
            as *mut ::core::ffi::c_void,
        &raw mut (*rblk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node
            as *const ::core::ffi::c_void,
        ((*rblk).c2rust_unnamed.c2rust_unnamed.n as size_t)
            .wrapping_mul(::core::mem::size_of::<nghttp3_ksl_node>()),
    );
    memcpy(
        (*lblk)
            .c2rust_unnamed
            .c2rust_unnamed
            .keys
            .offset(
                ((*lblk).c2rust_unnamed.c2rust_unnamed.n as size_t)
                    .wrapping_mul((*ksl).aligned_keylen) as isize,
            ) as *mut ::core::ffi::c_void,
        (*rblk).c2rust_unnamed.c2rust_unnamed.keys as *const ::core::ffi::c_void,
        ((*rblk).c2rust_unnamed.c2rust_unnamed.n as size_t)
            .wrapping_mul((*ksl).aligned_keylen),
    );
    (*lblk).c2rust_unnamed.c2rust_unnamed.n = (*lblk)
        .c2rust_unnamed
        .c2rust_unnamed
        .n
        .wrapping_add((*rblk).c2rust_unnamed.c2rust_unnamed.n);
    (*lblk).c2rust_unnamed.c2rust_unnamed.next = (*rblk)
        .c2rust_unnamed
        .c2rust_unnamed
        .next;
    if !(*lblk).c2rust_unnamed.c2rust_unnamed.next.is_null() {
        (*(*lblk).c2rust_unnamed.c2rust_unnamed.next)
            .c2rust_unnamed
            .c2rust_unnamed
            .prev = lblk;
    } else if (*ksl).back == rblk {
        (*ksl).back = lblk;
    }
    ksl_blk_objalloc_del(ksl, rblk);
    if (*ksl).root == blk && (*blk).c2rust_unnamed.c2rust_unnamed.n == 2 as uint32_t {
        ksl_blk_objalloc_del(ksl, (*ksl).root);
        (*ksl).root = lblk;
    } else {
        ksl_remove_node(ksl, blk, i.wrapping_add(1 as size_t));
        ksl_set_nth_key(
            ksl,
            blk,
            i,
            nghttp3_ksl_blk_nth_key(
                lblk,
                (*lblk).c2rust_unnamed.c2rust_unnamed.n.wrapping_sub(1 as uint32_t)
                    as size_t,
            ),
        );
    }
    return lblk;
}
unsafe extern "C" fn ksl_shift_left(
    mut ksl: *mut nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut i: size_t,
) {
    let mut lnode: *mut nghttp3_ksl_node = ::core::ptr::null_mut::<nghttp3_ksl_node>();
    let mut rnode: *mut nghttp3_ksl_node = ::core::ptr::null_mut::<nghttp3_ksl_node>();
    let mut lblk: *mut nghttp3_ksl_blk = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    let mut rblk: *mut nghttp3_ksl_blk = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    let mut n: size_t = 0;
    '_c2rust_label: {
        if i > 0 as size_t {} else {
            __assert_fail(
                b"i > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                448 as ::core::ffi::c_uint,
                b"void ksl_shift_left(nghttp3_ksl *, nghttp3_ksl_blk *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    lnode = (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes
        as *mut nghttp3_ksl_node)
        .offset(i.wrapping_sub(1 as size_t) as isize);
    rnode = (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes
        as *mut nghttp3_ksl_node)
        .offset(i as isize);
    lblk = (*lnode).c2rust_unnamed.blk;
    rblk = (*rnode).c2rust_unnamed.blk;
    '_c2rust_label_0: {
        if (*lblk).c2rust_unnamed.c2rust_unnamed.n
            < (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint32_t
        {} else {
            __assert_fail(
                b"lblk->n < NGHTTP3_KSL_MAX_NBLK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                456 as ::core::ffi::c_uint,
                b"void ksl_shift_left(nghttp3_ksl *, nghttp3_ksl_blk *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*rblk).c2rust_unnamed.c2rust_unnamed.n > 16 as uint32_t {} else {
            __assert_fail(
                b"rblk->n > NGHTTP3_KSL_MIN_NBLK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                457 as ::core::ffi::c_uint,
                b"void ksl_shift_left(nghttp3_ksl *, nghttp3_ksl_blk *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    n = (*lblk)
        .c2rust_unnamed
        .c2rust_unnamed
        .n
        .wrapping_add((*rblk).c2rust_unnamed.c2rust_unnamed.n)
        .wrapping_add(1 as uint32_t)
        .wrapping_div(2 as uint32_t)
        .wrapping_sub((*lblk).c2rust_unnamed.c2rust_unnamed.n) as size_t;
    '_c2rust_label_2: {
        if n > 0 as size_t {} else {
            __assert_fail(
                b"n > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                461 as ::core::ffi::c_uint,
                b"void ksl_shift_left(nghttp3_ksl *, nghttp3_ksl_blk *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if (*lblk).c2rust_unnamed.c2rust_unnamed.n as size_t
            <= ((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as size_t)
                .wrapping_sub(n)
        {} else {
            __assert_fail(
                b"lblk->n <= NGHTTP3_KSL_MAX_NBLK - n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                462 as ::core::ffi::c_uint,
                b"void ksl_shift_left(nghttp3_ksl *, nghttp3_ksl_blk *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if (*rblk).c2rust_unnamed.c2rust_unnamed.n as size_t
            >= (16 as size_t).wrapping_add(n)
        {} else {
            __assert_fail(
                b"rblk->n >= NGHTTP3_KSL_MIN_NBLK + n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                463 as ::core::ffi::c_uint,
                b"void ksl_shift_left(nghttp3_ksl *, nghttp3_ksl_blk *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    memcpy(
        (&raw mut (*lblk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node)
            .offset((*lblk).c2rust_unnamed.c2rust_unnamed.n as isize)
            as *mut ::core::ffi::c_void,
        &raw mut (*rblk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node
            as *const ::core::ffi::c_void,
        n.wrapping_mul(::core::mem::size_of::<nghttp3_ksl_node>()),
    );
    memcpy(
        (*lblk)
            .c2rust_unnamed
            .c2rust_unnamed
            .keys
            .offset(
                ((*lblk).c2rust_unnamed.c2rust_unnamed.n as size_t)
                    .wrapping_mul((*ksl).aligned_keylen) as isize,
            ) as *mut ::core::ffi::c_void,
        (*rblk).c2rust_unnamed.c2rust_unnamed.keys as *const ::core::ffi::c_void,
        n.wrapping_mul((*ksl).aligned_keylen),
    );
    (*lblk).c2rust_unnamed.c2rust_unnamed.n = (*lblk)
        .c2rust_unnamed
        .c2rust_unnamed
        .n
        .wrapping_add(n as uint32_t);
    (*rblk).c2rust_unnamed.c2rust_unnamed.n = (*rblk)
        .c2rust_unnamed
        .c2rust_unnamed
        .n
        .wrapping_sub(n as uint32_t);
    ksl_set_nth_key(
        ksl,
        blk,
        i.wrapping_sub(1 as size_t),
        nghttp3_ksl_blk_nth_key(
            lblk,
            (*lblk).c2rust_unnamed.c2rust_unnamed.n.wrapping_sub(1 as uint32_t) as size_t,
        ),
    );
    memmove(
        &raw mut (*rblk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node
            as *mut ::core::ffi::c_void,
        (&raw mut (*rblk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node)
            .offset(n as isize) as *const ::core::ffi::c_void,
        ((*rblk).c2rust_unnamed.c2rust_unnamed.n as size_t)
            .wrapping_mul(::core::mem::size_of::<nghttp3_ksl_node>()),
    );
    memmove(
        (*rblk).c2rust_unnamed.c2rust_unnamed.keys as *mut ::core::ffi::c_void,
        (*rblk)
            .c2rust_unnamed
            .c2rust_unnamed
            .keys
            .offset(n.wrapping_mul((*ksl).aligned_keylen) as isize)
            as *const ::core::ffi::c_void,
        ((*rblk).c2rust_unnamed.c2rust_unnamed.n as size_t)
            .wrapping_mul((*ksl).aligned_keylen),
    );
}
unsafe extern "C" fn ksl_shift_right(
    mut ksl: *mut nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut i: size_t,
) {
    let mut lnode: *mut nghttp3_ksl_node = ::core::ptr::null_mut::<nghttp3_ksl_node>();
    let mut rnode: *mut nghttp3_ksl_node = ::core::ptr::null_mut::<nghttp3_ksl_node>();
    let mut lblk: *mut nghttp3_ksl_blk = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    let mut rblk: *mut nghttp3_ksl_blk = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    let mut n: size_t = 0;
    '_c2rust_label: {
        if i
            < (*blk).c2rust_unnamed.c2rust_unnamed.n.wrapping_sub(1 as uint32_t)
                as size_t
        {} else {
            __assert_fail(
                b"i < blk->n - 1\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                491 as ::core::ffi::c_uint,
                b"void ksl_shift_right(nghttp3_ksl *, nghttp3_ksl_blk *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    lnode = (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes
        as *mut nghttp3_ksl_node)
        .offset(i as isize);
    rnode = (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes
        as *mut nghttp3_ksl_node)
        .offset(i.wrapping_add(1 as size_t) as isize);
    lblk = (*lnode).c2rust_unnamed.blk;
    rblk = (*rnode).c2rust_unnamed.blk;
    '_c2rust_label_0: {
        if (*lblk).c2rust_unnamed.c2rust_unnamed.n > 16 as uint32_t {} else {
            __assert_fail(
                b"lblk->n > NGHTTP3_KSL_MIN_NBLK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                499 as ::core::ffi::c_uint,
                b"void ksl_shift_right(nghttp3_ksl *, nghttp3_ksl_blk *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*rblk).c2rust_unnamed.c2rust_unnamed.n
            < (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint32_t
        {} else {
            __assert_fail(
                b"rblk->n < NGHTTP3_KSL_MAX_NBLK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                500 as ::core::ffi::c_uint,
                b"void ksl_shift_right(nghttp3_ksl *, nghttp3_ksl_blk *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    n = (*lblk)
        .c2rust_unnamed
        .c2rust_unnamed
        .n
        .wrapping_add((*rblk).c2rust_unnamed.c2rust_unnamed.n)
        .wrapping_add(1 as uint32_t)
        .wrapping_div(2 as uint32_t)
        .wrapping_sub((*rblk).c2rust_unnamed.c2rust_unnamed.n) as size_t;
    '_c2rust_label_2: {
        if n > 0 as size_t {} else {
            __assert_fail(
                b"n > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                504 as ::core::ffi::c_uint,
                b"void ksl_shift_right(nghttp3_ksl *, nghttp3_ksl_blk *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if (*lblk).c2rust_unnamed.c2rust_unnamed.n as size_t
            >= (16 as size_t).wrapping_add(n)
        {} else {
            __assert_fail(
                b"lblk->n >= NGHTTP3_KSL_MIN_NBLK + n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                505 as ::core::ffi::c_uint,
                b"void ksl_shift_right(nghttp3_ksl *, nghttp3_ksl_blk *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if (*rblk).c2rust_unnamed.c2rust_unnamed.n as size_t
            <= ((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as size_t)
                .wrapping_sub(n)
        {} else {
            __assert_fail(
                b"rblk->n <= NGHTTP3_KSL_MAX_NBLK - n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                506 as ::core::ffi::c_uint,
                b"void ksl_shift_right(nghttp3_ksl *, nghttp3_ksl_blk *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    memmove(
        (&raw mut (*rblk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node)
            .offset(n as isize) as *mut ::core::ffi::c_void,
        &raw mut (*rblk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node
            as *const ::core::ffi::c_void,
        ((*rblk).c2rust_unnamed.c2rust_unnamed.n as size_t)
            .wrapping_mul(::core::mem::size_of::<nghttp3_ksl_node>()),
    );
    memmove(
        (*rblk)
            .c2rust_unnamed
            .c2rust_unnamed
            .keys
            .offset(n.wrapping_mul((*ksl).aligned_keylen) as isize)
            as *mut ::core::ffi::c_void,
        (*rblk).c2rust_unnamed.c2rust_unnamed.keys as *const ::core::ffi::c_void,
        ((*rblk).c2rust_unnamed.c2rust_unnamed.n as size_t)
            .wrapping_mul((*ksl).aligned_keylen),
    );
    (*rblk).c2rust_unnamed.c2rust_unnamed.n = (*rblk)
        .c2rust_unnamed
        .c2rust_unnamed
        .n
        .wrapping_add(n as uint32_t);
    (*lblk).c2rust_unnamed.c2rust_unnamed.n = (*lblk)
        .c2rust_unnamed
        .c2rust_unnamed
        .n
        .wrapping_sub(n as uint32_t);
    memcpy(
        &raw mut (*rblk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node
            as *mut ::core::ffi::c_void,
        (&raw mut (*lblk).c2rust_unnamed.c2rust_unnamed.nodes as *mut nghttp3_ksl_node)
            .offset((*lblk).c2rust_unnamed.c2rust_unnamed.n as isize)
            as *const ::core::ffi::c_void,
        n.wrapping_mul(::core::mem::size_of::<nghttp3_ksl_node>()),
    );
    memcpy(
        (*rblk).c2rust_unnamed.c2rust_unnamed.keys as *mut ::core::ffi::c_void,
        (*lblk)
            .c2rust_unnamed
            .c2rust_unnamed
            .keys
            .offset(
                ((*lblk).c2rust_unnamed.c2rust_unnamed.n as size_t)
                    .wrapping_mul((*ksl).aligned_keylen) as isize,
            ) as *const ::core::ffi::c_void,
        n.wrapping_mul((*ksl).aligned_keylen),
    );
    ksl_set_nth_key(
        ksl,
        blk,
        i,
        nghttp3_ksl_blk_nth_key(
            lblk,
            (*lblk).c2rust_unnamed.c2rust_unnamed.n.wrapping_sub(1 as uint32_t) as size_t,
        ),
    );
}
unsafe extern "C" fn key_equal(
    mut compar: nghttp3_ksl_compar,
    mut lhs: *const ::core::ffi::c_void,
    mut rhs: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (compar.expect("non-null function pointer")(lhs, rhs) == 0
        && compar.expect("non-null function pointer")(rhs, lhs) == 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_remove_hint(
    mut ksl: *mut nghttp3_ksl,
    mut it: *mut nghttp3_ksl_it,
    mut hint: *const nghttp3_ksl_it,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut blk: *mut nghttp3_ksl_blk = (*hint).blk;
    '_c2rust_label: {
        if !(*ksl).root.is_null() {} else {
            __assert_fail(
                b"ksl->root\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                538 as ::core::ffi::c_uint,
                b"int nghttp3_ksl_remove_hint(nghttp3_ksl *, nghttp3_ksl_it *, const nghttp3_ksl_it *, const nghttp3_ksl_key *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if blk != (*ksl).root
        && (*blk).c2rust_unnamed.c2rust_unnamed.n == NGHTTP3_KSL_MIN_NBLK as uint32_t
    {
        return nghttp3_ksl_remove(ksl, it, key);
    }
    ksl_remove_node(ksl, blk, (*hint).i);
    (*ksl).n = (*ksl).n.wrapping_sub(1);
    if !it.is_null() {
        if (*hint).i == (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t
            && !(*blk).c2rust_unnamed.c2rust_unnamed.next.is_null()
        {
            nghttp3_ksl_it_init(
                it,
                (*blk).c2rust_unnamed.c2rust_unnamed.next,
                0 as size_t,
            );
        } else {
            nghttp3_ksl_it_init(it, blk, (*hint).i);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_remove(
    mut ksl: *mut nghttp3_ksl,
    mut it: *mut nghttp3_ksl_it,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut blk: *mut nghttp3_ksl_blk = (*ksl).root;
    let mut node: *mut nghttp3_ksl_node = ::core::ptr::null_mut::<nghttp3_ksl_node>();
    let mut i: size_t = 0;
    if blk.is_null() {
        return NGHTTP3_ERR_INVALID_ARGUMENT;
    }
    if (*blk).c2rust_unnamed.c2rust_unnamed.leaf == 0
        && (*blk).c2rust_unnamed.c2rust_unnamed.n == 2 as uint32_t
        && (*(*blk).c2rust_unnamed.c2rust_unnamed.nodes[0usize].c2rust_unnamed.blk)
            .c2rust_unnamed
            .c2rust_unnamed
            .n == NGHTTP3_KSL_MIN_NBLK as uint32_t
        && (*(*blk).c2rust_unnamed.c2rust_unnamed.nodes[1usize].c2rust_unnamed.blk)
            .c2rust_unnamed
            .c2rust_unnamed
            .n == NGHTTP3_KSL_MIN_NBLK as uint32_t
    {
        blk = ksl_merge_node(ksl, blk, 0 as size_t);
    }
    loop {
        i = (*ksl).search.expect("non-null function pointer")(ksl, blk, key);
        if i == (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t {
            if !it.is_null() {
                *it = nghttp3_ksl_end(ksl);
            }
            return NGHTTP3_ERR_INVALID_ARGUMENT;
        }
        if (*blk).c2rust_unnamed.c2rust_unnamed.leaf != 0 {
            if (*ksl)
                .compar
                .expect(
                    "non-null function pointer",
                )(key, nghttp3_ksl_blk_nth_key(blk, i)) != 0
            {
                if !it.is_null() {
                    *it = nghttp3_ksl_end(ksl);
                }
                return NGHTTP3_ERR_INVALID_ARGUMENT;
            }
            ksl_remove_node(ksl, blk, i);
            (*ksl).n = (*ksl).n.wrapping_sub(1);
            if !it.is_null() {
                if (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t == i
                    && !(*blk).c2rust_unnamed.c2rust_unnamed.next.is_null()
                {
                    nghttp3_ksl_it_init(
                        it,
                        (*blk).c2rust_unnamed.c2rust_unnamed.next,
                        0 as size_t,
                    );
                } else {
                    nghttp3_ksl_it_init(it, blk, i);
                }
            }
            return 0 as ::core::ffi::c_int;
        }
        node = (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes
            as *mut nghttp3_ksl_node)
            .offset(i as isize);
        if (*(*node).c2rust_unnamed.blk).c2rust_unnamed.c2rust_unnamed.n
            > NGHTTP3_KSL_MIN_NBLK as uint32_t
        {
            blk = (*node).c2rust_unnamed.blk;
        } else {
            '_c2rust_label: {
                if (*(*node).c2rust_unnamed.blk).c2rust_unnamed.c2rust_unnamed.n
                    == 16 as uint32_t
                {} else {
                    __assert_fail(
                        b"node->blk->n == NGHTTP3_KSL_MIN_NBLK\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        616 as ::core::ffi::c_uint,
                        b"int nghttp3_ksl_remove(nghttp3_ksl *, nghttp3_ksl_it *, const nghttp3_ksl_key *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            if i.wrapping_add(1 as size_t)
                < (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t
                && (*(*blk)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .nodes[i.wrapping_add(1 as size_t)]
                    .c2rust_unnamed
                    .blk)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .n > NGHTTP3_KSL_MIN_NBLK as uint32_t
            {
                ksl_shift_left(ksl, blk, i.wrapping_add(1 as size_t));
                blk = (*node).c2rust_unnamed.blk;
            } else if i > 0 as size_t
                && (*(*blk)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .nodes[i.wrapping_sub(1 as size_t)]
                    .c2rust_unnamed
                    .blk)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .n > NGHTTP3_KSL_MIN_NBLK as uint32_t
            {
                ksl_shift_right(ksl, blk, i.wrapping_sub(1 as size_t));
                blk = (*node).c2rust_unnamed.blk;
            } else if i.wrapping_add(1 as size_t)
                < (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t
            {
                blk = ksl_merge_node(ksl, blk, i);
            } else {
                '_c2rust_label_0: {
                    if i > 0 as size_t {} else {
                        __assert_fail(
                            b"i > 0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            637 as ::core::ffi::c_uint,
                            b"int nghttp3_ksl_remove(nghttp3_ksl *, nghttp3_ksl_it *, const nghttp3_ksl_key *)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                blk = ksl_merge_node(ksl, blk, i.wrapping_sub(1 as size_t));
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_lower_bound(
    mut ksl: *const nghttp3_ksl,
    mut key: *const ::core::ffi::c_void,
) -> nghttp3_ksl_it {
    return nghttp3_ksl_lower_bound_search(ksl, key, (*ksl).search);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_lower_bound_search(
    mut ksl: *const nghttp3_ksl,
    mut key: *const ::core::ffi::c_void,
    mut search: nghttp3_ksl_search,
) -> nghttp3_ksl_it {
    let mut blk: *mut nghttp3_ksl_blk = (*ksl).root;
    let mut it: nghttp3_ksl_it = nghttp3_ksl_it {
        blk: ::core::ptr::null_mut::<nghttp3_ksl_blk>(),
        i: 0,
    };
    let mut i: size_t = 0;
    if blk.is_null() {
        nghttp3_ksl_it_init(&raw mut it, &raw mut null_blk, 0 as size_t);
        return it;
    }
    loop {
        i = search.expect("non-null function pointer")(ksl, blk, key);
        if (*blk).c2rust_unnamed.c2rust_unnamed.leaf != 0 {
            if i == (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t
                && !(*blk).c2rust_unnamed.c2rust_unnamed.next.is_null()
            {
                blk = (*blk).c2rust_unnamed.c2rust_unnamed.next;
                i = 0 as size_t;
            }
            nghttp3_ksl_it_init(&raw mut it, blk, i);
            return it;
        }
        if i == (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t {
            while (*blk).c2rust_unnamed.c2rust_unnamed.leaf == 0 {
                blk = (*blk)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .nodes[(*blk)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .n
                        .wrapping_sub(1 as uint32_t) as usize]
                    .c2rust_unnamed
                    .blk;
            }
            if !(*blk).c2rust_unnamed.c2rust_unnamed.next.is_null() {
                blk = (*blk).c2rust_unnamed.c2rust_unnamed.next;
                i = 0 as size_t;
            } else {
                i = (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t;
            }
            nghttp3_ksl_it_init(&raw mut it, blk, i);
            return it;
        }
        blk = (*blk).c2rust_unnamed.c2rust_unnamed.nodes[i].c2rust_unnamed.blk;
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_update_key(
    mut ksl: *mut nghttp3_ksl,
    mut old_key: *const ::core::ffi::c_void,
    mut new_key: *const ::core::ffi::c_void,
) {
    let mut blk: *mut nghttp3_ksl_blk = (*ksl).root;
    let mut node: *mut nghttp3_ksl_node = ::core::ptr::null_mut::<nghttp3_ksl_node>();
    let mut node_key: *const ::core::ffi::c_void = ::core::ptr::null::<
        ::core::ffi::c_void,
    >();
    let mut i: size_t = 0;
    '_c2rust_label: {
        if !(*ksl).root.is_null() {} else {
            __assert_fail(
                b"ksl->root\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                703 as ::core::ffi::c_uint,
                b"void nghttp3_ksl_update_key(nghttp3_ksl *, const nghttp3_ksl_key *, const nghttp3_ksl_key *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    loop {
        i = (*ksl).search.expect("non-null function pointer")(ksl, blk, old_key);
        '_c2rust_label_0: {
            if i < (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t {} else {
                __assert_fail(
                    b"i < blk->n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    708 as ::core::ffi::c_uint,
                    b"void nghttp3_ksl_update_key(nghttp3_ksl *, const nghttp3_ksl_key *, const nghttp3_ksl_key *)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        node = (&raw mut (*blk).c2rust_unnamed.c2rust_unnamed.nodes
            as *mut nghttp3_ksl_node)
            .offset(i as isize);
        node_key = nghttp3_ksl_blk_nth_key(blk, i);
        if (*blk).c2rust_unnamed.c2rust_unnamed.leaf != 0 {
            '_c2rust_label_1: {
                if key_equal((*ksl).compar, node_key, old_key) != 0 {} else {
                    __assert_fail(
                        b"key_equal(ksl->compar, node_key, old_key)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        713 as ::core::ffi::c_uint,
                        b"void nghttp3_ksl_update_key(nghttp3_ksl *, const nghttp3_ksl_key *, const nghttp3_ksl_key *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            ksl_set_nth_key(ksl, blk, i, new_key);
            return;
        }
        if key_equal((*ksl).compar, node_key, old_key) != 0
            || (*ksl).compar.expect("non-null function pointer")(node_key, new_key) != 0
        {
            ksl_set_nth_key(ksl, blk, i, new_key);
        }
        blk = (*node).c2rust_unnamed.blk;
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_len(mut ksl: *const nghttp3_ksl) -> size_t {
    return (*ksl).n;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_clear(mut ksl: *mut nghttp3_ksl) {
    if (*ksl).root.is_null() {
        return;
    }
    (*ksl).root = ::core::ptr::null_mut::<nghttp3_ksl_blk>();
    (*ksl).back = (*ksl).root;
    (*ksl).front = (*ksl).back;
    (*ksl).n = 0 as size_t;
    nghttp3_objalloc_clear(&raw mut (*ksl).blkalloc);
}
unsafe extern "C" fn ksl_print(
    mut ksl: *const nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut level: size_t,
) {
    let mut i: size_t = 0;
    fprintf(
        stderr,
        b"LV=%zu n=%u\n\0".as_ptr() as *const ::core::ffi::c_char,
        level,
        (*blk).c2rust_unnamed.c2rust_unnamed.n,
    );
    if (*blk).c2rust_unnamed.c2rust_unnamed.leaf != 0 {
        i = 0 as size_t;
        while i < (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t {
            fprintf(
                stderr,
                b" %ld\0".as_ptr() as *const ::core::ffi::c_char,
                *(nghttp3_ksl_blk_nth_key(blk, i) as *mut int64_t),
            );
            i = i.wrapping_add(1);
        }
        fprintf(stderr, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        return;
    }
    i = 0 as size_t;
    while i < (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t {
        ksl_print(
            ksl,
            (*blk).c2rust_unnamed.c2rust_unnamed.nodes[i].c2rust_unnamed.blk,
            level.wrapping_add(1 as size_t),
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_print(mut ksl: *const nghttp3_ksl) {
    if (*ksl).root.is_null() {
        return;
    }
    ksl_print(ksl, (*ksl).root, 0 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_begin(
    mut ksl: *const nghttp3_ksl,
) -> nghttp3_ksl_it {
    let mut it: nghttp3_ksl_it = nghttp3_ksl_it {
        blk: ::core::ptr::null_mut::<nghttp3_ksl_blk>(),
        i: 0,
    };
    if !(*ksl).root.is_null() {
        nghttp3_ksl_it_init(&raw mut it, (*ksl).front, 0 as size_t);
    } else {
        nghttp3_ksl_it_init(&raw mut it, &raw mut null_blk, 0 as size_t);
    }
    return it;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_end(mut ksl: *const nghttp3_ksl) -> nghttp3_ksl_it {
    let mut it: nghttp3_ksl_it = nghttp3_ksl_it {
        blk: ::core::ptr::null_mut::<nghttp3_ksl_blk>(),
        i: 0,
    };
    if !(*ksl).root.is_null() {
        nghttp3_ksl_it_init(
            &raw mut it,
            (*ksl).back,
            (*(*ksl).back).c2rust_unnamed.c2rust_unnamed.n as size_t,
        );
    } else {
        nghttp3_ksl_it_init(&raw mut it, &raw mut null_blk, 0 as size_t);
    }
    return it;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_it_init(
    mut it: *mut nghttp3_ksl_it,
    mut blk: *mut nghttp3_ksl_blk,
    mut i: size_t,
) {
    (*it).blk = blk;
    (*it).i = i;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_it_prev(mut it: *mut nghttp3_ksl_it) {
    '_c2rust_label: {
        if nghttp3_ksl_it_begin(it) == 0 {} else {
            __assert_fail(
                b"!nghttp3_ksl_it_begin(it)\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ksl.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                806 as ::core::ffi::c_uint,
                b"void nghttp3_ksl_it_prev(nghttp3_ksl_it *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*it).i == 0 as size_t {
        (*it).blk = (*(*it).blk).c2rust_unnamed.c2rust_unnamed.prev;
        (*it).i = (*(*it).blk)
            .c2rust_unnamed
            .c2rust_unnamed
            .n
            .wrapping_sub(1 as uint32_t) as size_t;
    } else {
        (*it).i = (*it).i.wrapping_sub(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_it_begin(
    mut it: *const nghttp3_ksl_it,
) -> ::core::ffi::c_int {
    return ((*it).i == 0 as size_t
        && (*(*it).blk).c2rust_unnamed.c2rust_unnamed.prev.is_null())
        as ::core::ffi::c_int;
}
unsafe extern "C" fn ksl_range_search(
    mut ksl: *const nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut key: *const ::core::ffi::c_void,
) -> size_t {
    let mut i: size_t = 0;
    let mut node_key: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    i = 0 as size_t;
    node_key = (*blk).c2rust_unnamed.c2rust_unnamed.keys;
    while i < (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t
        && nghttp3_ksl_range_compar(node_key as *const ::core::ffi::c_void, key) != 0
    {
        i = i.wrapping_add(1);
        node_key = node_key.offset((*ksl).aligned_keylen as isize);
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_range_search(
    mut ksl: *const nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut key: *const ::core::ffi::c_void,
) -> size_t {
    return ksl_range_search(ksl, blk, key);
}
unsafe extern "C" fn ksl_range_exclusive_search(
    mut ksl: *const nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut key: *const ::core::ffi::c_void,
) -> size_t {
    let mut i: size_t = 0;
    let mut node_key: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    i = 0 as size_t;
    node_key = (*blk).c2rust_unnamed.c2rust_unnamed.keys;
    while i < (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t
        && nghttp3_ksl_range_exclusive_compar(
            node_key as *const ::core::ffi::c_void,
            key,
        ) != 0
    {
        i = i.wrapping_add(1);
        node_key = node_key.offset((*ksl).aligned_keylen as isize);
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_range_exclusive_search(
    mut ksl: *const nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut key: *const ::core::ffi::c_void,
) -> size_t {
    return ksl_range_exclusive_search(ksl, blk, key);
}
unsafe extern "C" fn ksl_uint64_less_search(
    mut ksl: *const nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut key: *const ::core::ffi::c_void,
) -> size_t {
    let mut i: size_t = 0;
    let mut node_key: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    i = 0 as size_t;
    node_key = (*blk).c2rust_unnamed.c2rust_unnamed.keys;
    while i < (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t
        && nghttp3_ksl_uint64_less(node_key as *const ::core::ffi::c_void, key) != 0
    {
        i = i.wrapping_add(1);
        node_key = node_key.offset((*ksl).aligned_keylen as isize);
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_uint64_less_search(
    mut ksl: *const nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut key: *const ::core::ffi::c_void,
) -> size_t {
    return ksl_uint64_less_search(ksl, blk, key);
}
unsafe extern "C" fn ksl_int64_greater_search(
    mut ksl: *const nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut key: *const ::core::ffi::c_void,
) -> size_t {
    let mut i: size_t = 0;
    let mut node_key: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    i = 0 as size_t;
    node_key = (*blk).c2rust_unnamed.c2rust_unnamed.keys;
    while i < (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t
        && nghttp3_ksl_int64_greater(node_key as *const ::core::ffi::c_void, key) != 0
    {
        i = i.wrapping_add(1);
        node_key = node_key.offset((*ksl).aligned_keylen as isize);
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ksl_int64_greater_search(
    mut ksl: *const nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut key: *const ::core::ffi::c_void,
) -> size_t {
    return ksl_int64_greater_search(ksl, blk, key);
}
