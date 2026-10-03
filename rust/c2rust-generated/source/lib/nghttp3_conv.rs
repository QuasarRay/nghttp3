extern "C" {
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn memcpy(
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
    fn nghttp3_cpymem(
        dest: *mut uint8_t,
        src: *const uint8_t,
        n: size_t,
    ) -> *mut uint8_t;
    fn nghttp3_unreachable_fail(
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
    ) -> !;
}
pub type __uint64_t = u64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type size_t = usize;
pub type int64_t = i64;
#[inline]
unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
    return ((__bsx as ::core::ffi::c_ulonglong
        & 0xff00000000000000 as ::core::ffi::c_ulonglong) >> 56 as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_ulonglong
            & 0xff000000000000 as ::core::ffi::c_ulonglong) >> 40 as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_ulonglong
            & 0xff0000000000 as ::core::ffi::c_ulonglong) >> 24 as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_ulonglong & 0xff00000000 as ::core::ffi::c_ulonglong)
            >> 8 as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_ulonglong & 0xff000000 as ::core::ffi::c_ulonglong)
            << 8 as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_ulonglong & 0xff0000 as ::core::ffi::c_ulonglong)
            << 24 as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_ulonglong & 0xff00 as ::core::ffi::c_ulonglong)
            << 40 as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_ulonglong & 0xff as ::core::ffi::c_ulonglong)
            << 56 as ::core::ffi::c_int) as __uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_get_uvarint(
    mut dest: *mut uint64_t,
    mut p: *const uint8_t,
) -> *const uint8_t {
    let mut n16: uint16_t = 0;
    let mut n32: uint32_t = 0;
    let mut n64: uint64_t = 0;
    match *p as ::core::ffi::c_int >> 6 as ::core::ffi::c_int {
        0 => {
            let c2rust_fresh0 = p;
            p = p.offset(1);
            *dest = *c2rust_fresh0 as uint64_t;
            return p;
        }
        1 => {
            memcpy(
                &raw mut n16 as *mut ::core::ffi::c_void,
                p as *const ::core::ffi::c_void,
                2 as size_t,
            );
            n16 = ntohs(n16);
            n16 = (n16 as ::core::ffi::c_uint & 0x3fff as ::core::ffi::c_uint)
                as uint16_t;
            *dest = n16 as uint64_t;
            return p.offset(2 as ::core::ffi::c_int as isize);
        }
        2 => {
            memcpy(
                &raw mut n32 as *mut ::core::ffi::c_void,
                p as *const ::core::ffi::c_void,
                4 as size_t,
            );
            n32 = ntohl(n32);
            n32 = (n32 as ::core::ffi::c_uint & 0x3fffffff as ::core::ffi::c_uint)
                as uint32_t;
            *dest = n32 as uint64_t;
            return p.offset(4 as ::core::ffi::c_int as isize);
        }
        3 => {
            memcpy(
                &raw mut n64 as *mut ::core::ffi::c_void,
                p as *const ::core::ffi::c_void,
                8 as size_t,
            );
            n64 = __bswap_64(n64) as uint64_t;
            n64 = (n64 as ::core::ffi::c_ulong
                & 0x3fffffffffffffff as ::core::ffi::c_ulong) as uint64_t;
            *dest = n64;
            return p.offset(8 as ::core::ffi::c_int as isize);
        }
        _ => {
            nghttp3_unreachable_fail(
                b"nghttp3_conv.c\0".as_ptr() as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_int,
                b"nghttp3_get_uvarint\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_get_uvarintlen(mut p: *const uint8_t) -> size_t {
    return ((1 as ::core::ffi::c_uint)
        << (*p as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_get_varint(
    mut dest: *mut int64_t,
    mut p: *const uint8_t,
) -> *const uint8_t {
    let mut n: uint64_t = 0;
    p = nghttp3_get_uvarint(&raw mut n, p);
    *dest = n as int64_t;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_put_uint64be(
    mut p: *mut uint8_t,
    mut n: uint64_t,
) -> *mut uint8_t {
    n = __bswap_64(n) as uint64_t;
    return nghttp3_cpymem(
        p,
        &raw mut n as *const uint8_t,
        ::core::mem::size_of::<uint64_t>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_put_uint32be(
    mut p: *mut uint8_t,
    mut n: uint32_t,
) -> *mut uint8_t {
    n = htonl(n);
    return nghttp3_cpymem(
        p,
        &raw mut n as *const uint8_t,
        ::core::mem::size_of::<uint32_t>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_put_uint16be(
    mut p: *mut uint8_t,
    mut n: uint16_t,
) -> *mut uint8_t {
    n = htons(n);
    return nghttp3_cpymem(
        p,
        &raw mut n as *const uint8_t,
        ::core::mem::size_of::<uint16_t>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_put_uvarint(
    mut p: *mut uint8_t,
    mut n: uint64_t,
) -> *mut uint8_t {
    let mut rv: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if n < 64 as uint64_t {
        let c2rust_fresh1 = p;
        p = p.offset(1);
        *c2rust_fresh1 = n as uint8_t;
        return p;
    }
    if n < 16384 as uint64_t {
        rv = nghttp3_put_uint16be(p, n as uint16_t);
        *p = (*p as ::core::ffi::c_uint | 0x40 as ::core::ffi::c_uint) as uint8_t;
        return rv;
    }
    if n < 1073741824 as uint64_t {
        rv = nghttp3_put_uint32be(p, n as uint32_t);
        *p = (*p as ::core::ffi::c_uint | 0x80 as ::core::ffi::c_uint) as uint8_t;
        return rv;
    }
    '_c2rust_label: {
        if (n as ::core::ffi::c_ulonglong)
            < 4611686018427387904 as ::core::ffi::c_ulonglong
        {} else {
            __assert_fail(
                b"n < 4611686018427387904ULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conv.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                113 as ::core::ffi::c_uint,
                b"uint8_t *nghttp3_put_uvarint(uint8_t *, uint64_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    rv = nghttp3_put_uint64be(p, n);
    *p = (*p as ::core::ffi::c_uint | 0xc0 as ::core::ffi::c_uint) as uint8_t;
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_put_uvarintlen(mut n: uint64_t) -> size_t {
    if n < 64 as uint64_t {
        return 1 as size_t;
    }
    if n < 16384 as uint64_t {
        return 2 as size_t;
    }
    if n < 1073741824 as uint64_t {
        return 4 as size_t;
    }
    '_c2rust_label: {
        if (n as ::core::ffi::c_ulonglong)
            < 4611686018427387904 as ::core::ffi::c_ulonglong
        {} else {
            __assert_fail(
                b"n < 4611686018427387904ULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conv.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_uint,
                b"size_t nghttp3_put_uvarintlen(uint64_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return 8 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ord_stream_id(mut stream_id: int64_t) -> uint64_t {
    return ((stream_id >> 2 as ::core::ffi::c_int) as uint64_t)
        .wrapping_add(1 as uint64_t);
}
