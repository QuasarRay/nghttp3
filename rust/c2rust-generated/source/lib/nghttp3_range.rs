pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_range {
    pub begin: uint64_t,
    pub end: uint64_t,
}
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
#[no_mangle]
pub unsafe extern "C" fn nghttp3_range_init(
    mut r: *mut nghttp3_range,
    mut begin: uint64_t,
    mut end: uint64_t,
) {
    *r = nghttp3_range {
        begin: begin,
        end: end,
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_range_intersect(
    mut a: *const nghttp3_range,
    mut b: *const nghttp3_range,
) -> nghttp3_range {
    let mut r: nghttp3_range = nghttp3_range { begin: 0, end: 0 };
    let mut begin: uint64_t = nghttp3_max_unsigned_long_int(
        (*a).begin as ::core::ffi::c_ulong,
        (*b).begin as ::core::ffi::c_ulong,
    ) as uint64_t;
    let mut end: uint64_t = nghttp3_min_unsigned_long_int(
        (*a).end as ::core::ffi::c_ulong,
        (*b).end as ::core::ffi::c_ulong,
    ) as uint64_t;
    if begin < end {
        nghttp3_range_init(&raw mut r, begin, end);
    } else {
        r = nghttp3_range {
            begin: 0 as uint64_t,
            end: 0,
        };
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_range_len(mut r: *const nghttp3_range) -> uint64_t {
    return (*r).end.wrapping_sub((*r).begin);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_range_eq(
    mut a: *const nghttp3_range,
    mut b: *const nghttp3_range,
) -> ::core::ffi::c_int {
    return ((*a).begin == (*b).begin && (*a).end == (*b).end) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_range_cut(
    mut left: *mut nghttp3_range,
    mut right: *mut nghttp3_range,
    mut a: *const nghttp3_range,
    mut b: *const nghttp3_range,
) {
    *left = nghttp3_range {
        begin: (*a).begin,
        end: (*b).begin,
    };
    *right = nghttp3_range {
        begin: (*b).end,
        end: (*a).end,
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_range_not_after(
    mut a: *const nghttp3_range,
    mut b: *const nghttp3_range,
) -> ::core::ffi::c_int {
    return ((*a).end <= (*b).end) as ::core::ffi::c_int;
}
