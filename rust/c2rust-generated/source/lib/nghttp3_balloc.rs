extern "C" {
    fn nghttp3_buf_left(buf: *const nghttp3_buf) -> size_t;
    fn nghttp3_mem_malloc(
        mem: *const nghttp3_mem,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn nghttp3_mem_free(mem: *const nghttp3_mem, ptr: *mut ::core::ffi::c_void);
    fn nghttp3_buf_wrap_init(buf: *mut nghttp3_buf, src: *mut uint8_t, len: size_t);
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint64_t = u64;
pub type uintptr_t = usize;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NGHTTP3_ERR_NOMEM: ::core::ffi::c_int = -901 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn nghttp3_balloc_init(
    mut balloc: *mut nghttp3_balloc,
    mut blklen: size_t,
    mut mem: *const nghttp3_mem,
) {
    '_c2rust_label: {
        if blklen & 0xf as size_t == 0 as size_t {} else {
            __assert_fail(
                b"(blklen & 0xFU) == 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_balloc.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                34 as ::core::ffi::c_uint,
                b"void nghttp3_balloc_init(nghttp3_balloc *, size_t, const nghttp3_mem *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    (*balloc).mem = mem;
    (*balloc).blklen = blklen;
    (*balloc).head = ::core::ptr::null_mut::<nghttp3_memblock_hd>();
    nghttp3_buf_wrap_init(
        &raw mut (*balloc).buf,
        b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_void
            as *mut uint8_t,
        0 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_balloc_free(mut balloc: *mut nghttp3_balloc) {
    if balloc.is_null() {
        return;
    }
    nghttp3_balloc_clear(balloc);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_balloc_clear(mut balloc: *mut nghttp3_balloc) {
    let mut p: *mut nghttp3_memblock_hd = ::core::ptr::null_mut::<nghttp3_memblock_hd>();
    let mut next: *mut nghttp3_memblock_hd = ::core::ptr::null_mut::<
        nghttp3_memblock_hd,
    >();
    p = (*balloc).head;
    while !p.is_null() {
        next = (*p).c2rust_unnamed.next;
        nghttp3_mem_free((*balloc).mem, p as *mut ::core::ffi::c_void);
        p = next;
    }
    (*balloc).head = ::core::ptr::null_mut::<nghttp3_memblock_hd>();
    nghttp3_buf_wrap_init(
        &raw mut (*balloc).buf,
        b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_void
            as *mut uint8_t,
        0 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_balloc_get(
    mut balloc: *mut nghttp3_balloc,
    mut pbuf: *mut *mut ::core::ffi::c_void,
    mut n: size_t,
) -> ::core::ffi::c_int {
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut hd: *mut nghttp3_memblock_hd = ::core::ptr::null_mut::<
        nghttp3_memblock_hd,
    >();
    '_c2rust_label: {
        if n <= (*balloc).blklen {} else {
            __assert_fail(
                b"n <= balloc->blklen\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_balloc.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                66 as ::core::ffi::c_uint,
                b"int nghttp3_balloc_get(nghttp3_balloc *, void **, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if nghttp3_buf_left(&raw mut (*balloc).buf) < n {
        p = nghttp3_mem_malloc(
            (*balloc).mem,
            ::core::mem::size_of::<nghttp3_memblock_hd>()
                .wrapping_add(0x8 as size_t)
                .wrapping_add((*balloc).blklen),
        ) as *mut uint8_t;
        if p.is_null() {
            return NGHTTP3_ERR_NOMEM;
        }
        hd = p as *mut ::core::ffi::c_void as *mut nghttp3_memblock_hd;
        (*hd).c2rust_unnamed.next = (*balloc).head;
        (*balloc).head = hd;
        nghttp3_buf_wrap_init(
            &raw mut (*balloc).buf,
            ::core::ptr::from_exposed_addr_mut::<
                uint8_t,
            >(
                ((p.expose_addr() as uintptr_t)
                    .wrapping_add(
                        ::core::mem::size_of::<nghttp3_memblock_hd>() as uintptr_t,
                    )
                    .wrapping_add(0xf as uintptr_t)
                    & !(0xf as ::core::ffi::c_uint as uintptr_t)) as usize,
            ),
            (*balloc).blklen,
        );
    }
    '_c2rust_label_0: {
        if (*balloc).buf.last.expose_addr() as uintptr_t & 0xf as uintptr_t
            == 0 as uintptr_t
        {} else {
            __assert_fail(
                b"((uintptr_t)balloc->buf.last & 0xFU) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_balloc.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_uint,
                b"int nghttp3_balloc_get(nghttp3_balloc *, void **, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *pbuf = (*balloc).buf.last as *mut ::core::ffi::c_void;
    (*balloc).buf.last = (*balloc)
        .buf
        .last
        .offset(
            (n.wrapping_add(0xf as size_t) & !(0xf as ::core::ffi::c_uint as size_t))
                as isize,
        );
    return 0 as ::core::ffi::c_int;
}
