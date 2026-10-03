extern "C" {
    fn nghttp3_balloc_init(
        balloc: *mut nghttp3_balloc,
        blklen: size_t,
        mem: *const nghttp3_mem,
    );
    fn nghttp3_balloc_free(balloc: *mut nghttp3_balloc);
    fn nghttp3_balloc_clear(balloc: *mut nghttp3_balloc);
    fn nghttp3_opl_init(opl: *mut nghttp3_opl);
    fn nghttp3_opl_clear(opl: *mut nghttp3_opl);
}
pub type size_t = usize;
pub type uint8_t = u8;
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
#[no_mangle]
pub unsafe extern "C" fn nghttp3_objalloc_init(
    mut objalloc: *mut nghttp3_objalloc,
    mut blklen: size_t,
    mut mem: *const nghttp3_mem,
) {
    nghttp3_balloc_init(&raw mut (*objalloc).balloc, blklen, mem);
    nghttp3_opl_init(&raw mut (*objalloc).opl);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_objalloc_free(mut objalloc: *mut nghttp3_objalloc) {
    nghttp3_balloc_free(&raw mut (*objalloc).balloc);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_objalloc_clear(mut objalloc: *mut nghttp3_objalloc) {
    nghttp3_opl_clear(&raw mut (*objalloc).opl);
    nghttp3_balloc_clear(&raw mut (*objalloc).balloc);
}
