extern "C" {
    fn nghttp3_gaptr_init(gaptr: *mut nghttp3_gaptr, mem: *const nghttp3_mem);
    fn nghttp3_gaptr_free(gaptr: *mut nghttp3_gaptr);
    fn nghttp3_gaptr_push(
        gaptr: *mut nghttp3_gaptr,
        offset: uint64_t,
        datalen: uint64_t,
    ) -> ::core::ffi::c_int;
    fn nghttp3_gaptr_is_pushed(
        gaptr: *const nghttp3_gaptr,
        offset: uint64_t,
        datalen: uint64_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
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
pub struct nghttp3_gaptr {
    pub gap: nghttp3_ksl,
    pub mem: *const nghttp3_mem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_idtr {
    pub gap: nghttp3_gaptr,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NGHTTP3_ERR_STREAM_IN_USE: ::core::ffi::c_int = -104 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn nghttp3_idtr_init(
    mut idtr: *mut nghttp3_idtr,
    mut mem: *const nghttp3_mem,
) {
    nghttp3_gaptr_init(&raw mut (*idtr).gap, mem);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_idtr_free(mut idtr: *mut nghttp3_idtr) {
    if idtr.is_null() {
        return;
    }
    nghttp3_gaptr_free(&raw mut (*idtr).gap);
}
unsafe extern "C" fn id_from_stream_id(mut stream_id: int64_t) -> uint64_t {
    return (stream_id >> 2 as ::core::ffi::c_int) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_idtr_open(
    mut idtr: *mut nghttp3_idtr,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    let mut q: uint64_t = 0;
    q = id_from_stream_id(stream_id);
    if nghttp3_gaptr_is_pushed(&raw mut (*idtr).gap, q, 1 as uint64_t) != 0 {
        return NGHTTP3_ERR_STREAM_IN_USE;
    }
    return nghttp3_gaptr_push(&raw mut (*idtr).gap, q, 1 as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_idtr_is_open(
    mut idtr: *const nghttp3_idtr,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    let mut q: uint64_t = 0;
    q = id_from_stream_id(stream_id);
    return nghttp3_gaptr_is_pushed(&raw const (*idtr).gap, q, 1 as uint64_t);
}
