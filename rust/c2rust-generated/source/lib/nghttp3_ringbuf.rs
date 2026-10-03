extern "C" {
    fn nghttp3_mem_malloc(
        mem: *const nghttp3_mem,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn nghttp3_mem_free(mem: *const nghttp3_mem, ptr: *mut ::core::ffi::c_void);
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
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
pub struct nghttp3_ringbuf {
    pub buf: *mut uint8_t,
    pub mem: *const nghttp3_mem,
    pub nmemb: size_t,
    pub size: size_t,
    pub first: size_t,
    pub len: size_t,
}
pub const NGHTTP3_ERR_NOMEM: ::core::ffi::c_int = -901 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[inline]
unsafe extern "C" fn nghttp3_min_unsigned_long_int(
    mut a: ::core::ffi::c_ulong,
    mut b: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    return if a < b { a } else { b };
}
unsafe extern "C" fn ispow2(mut n: size_t) -> ::core::ffi::c_int {
    return (n != 0 && n & n.wrapping_sub(1 as size_t) == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ringbuf_init(
    mut rb: *mut nghttp3_ringbuf,
    mut nmemb: size_t,
    mut size: size_t,
    mut mem: *const nghttp3_mem,
) -> ::core::ffi::c_int {
    if nmemb != 0 {
        '_c2rust_label: {
            if ispow2(nmemb) != 0 {} else {
                __assert_fail(
                    b"ispow2(nmemb)\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_ringbuf.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    44 as ::core::ffi::c_uint,
                    b"int nghttp3_ringbuf_init(nghttp3_ringbuf *, size_t, size_t, const nghttp3_mem *)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        (*rb).buf = nghttp3_mem_malloc(mem, nmemb.wrapping_mul(size)) as *mut uint8_t;
        if (*rb).buf.is_null() {
            return NGHTTP3_ERR_NOMEM;
        }
    } else {
        (*rb).buf = ::core::ptr::null_mut::<uint8_t>();
    }
    (*rb).mem = mem;
    (*rb).nmemb = nmemb;
    (*rb).size = size;
    (*rb).first = 0 as size_t;
    (*rb).len = 0 as size_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ringbuf_free(mut rb: *mut nghttp3_ringbuf) {
    if rb.is_null() {
        return;
    }
    nghttp3_mem_free((*rb).mem, (*rb).buf as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ringbuf_push_front(
    mut rb: *mut nghttp3_ringbuf,
) -> *mut ::core::ffi::c_void {
    (*rb).first = (*rb).first.wrapping_sub(1 as size_t)
        & (*rb).nmemb.wrapping_sub(1 as size_t);
    (*rb).len = nghttp3_min_unsigned_long_int(
        (*rb).nmemb as ::core::ffi::c_ulong,
        ((*rb).len as ::core::ffi::c_ulong).wrapping_add(1 as ::core::ffi::c_ulong),
    ) as size_t;
    return (*rb).buf.offset((*rb).first.wrapping_mul((*rb).size) as isize)
        as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ringbuf_push_back(
    mut rb: *mut nghttp3_ringbuf,
) -> *mut ::core::ffi::c_void {
    let mut offset: size_t = (*rb).first.wrapping_add((*rb).len)
        & (*rb).nmemb.wrapping_sub(1 as size_t);
    if (*rb).len == (*rb).nmemb {
        (*rb).first = (*rb).first.wrapping_add(1 as size_t)
            & (*rb).nmemb.wrapping_sub(1 as size_t);
    } else {
        (*rb).len = (*rb).len.wrapping_add(1);
    }
    return (*rb).buf.offset(offset.wrapping_mul((*rb).size) as isize)
        as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ringbuf_pop_front(mut rb: *mut nghttp3_ringbuf) {
    (*rb).first = (*rb).first.wrapping_add(1 as size_t)
        & (*rb).nmemb.wrapping_sub(1 as size_t);
    (*rb).len = (*rb).len.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ringbuf_pop_back(mut rb: *mut nghttp3_ringbuf) {
    '_c2rust_label: {
        if (*rb).len != 0 {} else {
            __assert_fail(
                b"rb->len\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_ringbuf.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_uint,
                b"void nghttp3_ringbuf_pop_back(nghttp3_ringbuf *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*rb).len = (*rb).len.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ringbuf_resize(
    mut rb: *mut nghttp3_ringbuf,
    mut len: size_t,
) {
    '_c2rust_label: {
        if len <= (*rb).nmemb {} else {
            __assert_fail(
                b"len <= rb->nmemb\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_ringbuf.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                101 as ::core::ffi::c_uint,
                b"void nghttp3_ringbuf_resize(nghttp3_ringbuf *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*rb).len = len;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ringbuf_get(
    mut rb: *mut nghttp3_ringbuf,
    mut offset: size_t,
) -> *mut ::core::ffi::c_void {
    '_c2rust_label: {
        if offset < (*rb).len {} else {
            __assert_fail(
                b"offset < rb->len\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_ringbuf.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                106 as ::core::ffi::c_uint,
                b"void *nghttp3_ringbuf_get(nghttp3_ringbuf *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    offset = (*rb).first.wrapping_add(offset) & (*rb).nmemb.wrapping_sub(1 as size_t);
    return (*rb).buf.offset(offset.wrapping_mul((*rb).size) as isize)
        as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ringbuf_full(
    mut rb: *const nghttp3_ringbuf,
) -> ::core::ffi::c_int {
    return ((*rb).len == (*rb).nmemb) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ringbuf_reserve(
    mut rb: *mut nghttp3_ringbuf,
    mut nmemb: size_t,
) -> ::core::ffi::c_int {
    let mut buf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if (*rb).nmemb >= nmemb {
        return 0 as ::core::ffi::c_int;
    }
    '_c2rust_label: {
        if ispow2(nmemb) != 0 {} else {
            __assert_fail(
                b"ispow2(nmemb)\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_ringbuf.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                122 as ::core::ffi::c_uint,
                b"int nghttp3_ringbuf_reserve(nghttp3_ringbuf *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    buf = nghttp3_mem_malloc((*rb).mem, nmemb.wrapping_mul((*rb).size)) as *mut uint8_t;
    if buf.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    if !(*rb).buf.is_null() {
        if (*rb).first.wrapping_add((*rb).len) <= (*rb).nmemb {
            memcpy(
                buf as *mut ::core::ffi::c_void,
                (*rb).buf.offset((*rb).first.wrapping_mul((*rb).size) as isize)
                    as *const ::core::ffi::c_void,
                (*rb).len.wrapping_mul((*rb).size),
            );
            (*rb).first = 0 as size_t;
        } else {
            memcpy(
                buf as *mut ::core::ffi::c_void,
                (*rb).buf.offset((*rb).first.wrapping_mul((*rb).size) as isize)
                    as *const ::core::ffi::c_void,
                (*rb).nmemb.wrapping_sub((*rb).first).wrapping_mul((*rb).size),
            );
            memcpy(
                buf
                    .offset(
                        (*rb).nmemb.wrapping_sub((*rb).first).wrapping_mul((*rb).size)
                            as isize,
                    ) as *mut ::core::ffi::c_void,
                (*rb).buf as *const ::core::ffi::c_void,
                (*rb)
                    .len
                    .wrapping_sub((*rb).nmemb.wrapping_sub((*rb).first))
                    .wrapping_mul((*rb).size),
            );
            (*rb).first = 0 as size_t;
        }
        nghttp3_mem_free((*rb).mem, (*rb).buf as *mut ::core::ffi::c_void);
    }
    (*rb).buf = buf;
    (*rb).nmemb = nmemb;
    return 0 as ::core::ffi::c_int;
}
