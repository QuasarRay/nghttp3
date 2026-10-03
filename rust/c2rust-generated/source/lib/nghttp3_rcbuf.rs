extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn nghttp3_mem_malloc(
        mem: *const nghttp3_mem,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn nghttp3_mem_free(mem: *const nghttp3_mem, ptr: *mut ::core::ffi::c_void);
    fn nghttp3_cpymem(
        dest: *mut uint8_t,
        src: *const uint8_t,
        n: size_t,
    ) -> *mut uint8_t;
}
pub type size_t = usize;
pub type int32_t = i32;
pub type uint8_t = u8;
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
pub struct nghttp3_vec {
    pub base: *mut uint8_t,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_rcbuf {
    pub mem: *const nghttp3_mem,
    pub base: *mut uint8_t,
    pub len: size_t,
    pub r#ref: int32_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NGHTTP3_ERR_NOMEM: ::core::ffi::c_int = -901 as ::core::ffi::c_int;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 43] = unsafe {
    ::core::mem::transmute::<
        [u8; 43],
        [::core::ffi::c_char; 43],
    >(*b"void nghttp3_rcbuf_decref(nghttp3_rcbuf *)\0")
};
#[no_mangle]
pub unsafe extern "C" fn nghttp3_rcbuf_new(
    mut rcbuf_ptr: *mut *mut nghttp3_rcbuf,
    mut size: size_t,
    mut mem: *const nghttp3_mem,
) -> ::core::ffi::c_int {
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    p = nghttp3_mem_malloc(
        mem,
        ::core::mem::size_of::<nghttp3_rcbuf>().wrapping_add(size),
    ) as *mut uint8_t;
    if p.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    *rcbuf_ptr = p as *mut ::core::ffi::c_void as *mut nghttp3_rcbuf;
    (**rcbuf_ptr).mem = mem;
    (**rcbuf_ptr).base = p.offset(::core::mem::size_of::<nghttp3_rcbuf>() as isize);
    (**rcbuf_ptr).len = size;
    (**rcbuf_ptr).r#ref = 1 as ::core::ffi::c_int as int32_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_rcbuf_new2(
    mut rcbuf_ptr: *mut *mut nghttp3_rcbuf,
    mut src: *const uint8_t,
    mut srclen: size_t,
    mut mem: *const nghttp3_mem,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    rv = nghttp3_rcbuf_new(rcbuf_ptr, srclen.wrapping_add(1 as size_t), mem);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    (**rcbuf_ptr).len = srclen;
    p = (**rcbuf_ptr).base;
    if srclen != 0 {
        p = nghttp3_cpymem(p, src, srclen);
    }
    *p = '\0' as uint8_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_rcbuf_del(mut rcbuf: *mut nghttp3_rcbuf) {
    nghttp3_mem_free((*rcbuf).mem, rcbuf as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_rcbuf_incref(mut rcbuf: *mut nghttp3_rcbuf) {
    if (*rcbuf).r#ref == -1 as int32_t {
        return;
    }
    (*rcbuf).r#ref += 1;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_rcbuf_decref(mut rcbuf: *mut nghttp3_rcbuf) {
    if rcbuf.is_null() || (*rcbuf).r#ref == -1 as int32_t {
        return;
    }
    '_c2rust_label: {
        if (*rcbuf).r#ref > 0 as int32_t {} else {
            __assert_fail(
                b"rcbuf->ref > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_rcbuf.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                94 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    (*rcbuf).r#ref -= 1;
    if (*rcbuf).r#ref == 0 as int32_t {
        nghttp3_rcbuf_del(rcbuf);
    }
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_rcbuf_get_buf(
    mut rcbuf: *const nghttp3_rcbuf,
) -> nghttp3_vec {
    let mut res: nghttp3_vec = nghttp3_vec {
        base: (*rcbuf).base,
        len: (*rcbuf).len,
    };
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_rcbuf_is_static(
    mut rcbuf: *const nghttp3_rcbuf,
) -> ::core::ffi::c_int {
    return ((*rcbuf).r#ref == -1 as int32_t) as ::core::ffi::c_int;
}
