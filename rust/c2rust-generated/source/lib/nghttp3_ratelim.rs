extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
}
pub type uint64_t = u64;
pub type nghttp3_tstamp = uint64_t;
pub type nghttp3_duration = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_ratelim {
    pub burst: uint64_t,
    pub rate: uint64_t,
    pub tokens: uint64_t,
    pub carry: uint64_t,
    pub ts: nghttp3_tstamp,
}
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const NGHTTP3_NANOSECONDS: nghttp3_duration = 1 as ::core::ffi::c_ulonglong
    as nghttp3_duration;
pub const NGHTTP3_MICROSECONDS: nghttp3_duration = (1000 as ::core::ffi::c_ulonglong)
    .wrapping_mul(NGHTTP3_NANOSECONDS as ::core::ffi::c_ulonglong) as nghttp3_duration;
pub const NGHTTP3_MILLISECONDS: nghttp3_duration = (1000 as ::core::ffi::c_ulonglong)
    .wrapping_mul(NGHTTP3_MICROSECONDS as ::core::ffi::c_ulonglong) as nghttp3_duration;
pub const NGHTTP3_SECONDS: nghttp3_duration = (1000 as ::core::ffi::c_ulonglong)
    .wrapping_mul(NGHTTP3_MILLISECONDS as ::core::ffi::c_ulonglong) as nghttp3_duration;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 55] = unsafe {
    ::core::mem::transmute::<
        [u8; 55],
        [::core::ffi::c_char; 55],
    >(*b"void ratelim_update(nghttp3_ratelim *, nghttp3_tstamp)\0")
};
#[inline]
unsafe extern "C" fn nghttp3_min_unsigned_long_int(
    mut a: ::core::ffi::c_ulong,
    mut b: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ratelim_init(
    mut rlim: *mut nghttp3_ratelim,
    mut burst: uint64_t,
    mut rate: uint64_t,
    mut ts: nghttp3_tstamp,
) {
    burst = nghttp3_min_unsigned_long_int(
        burst as ::core::ffi::c_ulong,
        (18446744073709551615 as ::core::ffi::c_ulong)
            .wrapping_div(
                (1000 as ::core::ffi::c_ulonglong)
                    .wrapping_mul(
                        (1000 as ::core::ffi::c_ulonglong)
                            .wrapping_mul(
                                (1000 as ::core::ffi::c_ulonglong)
                                    .wrapping_mul(
                                        1 as ::core::ffi::c_ulonglong as nghttp3_duration
                                            as ::core::ffi::c_ulonglong,
                                    ) as nghttp3_duration as ::core::ffi::c_ulonglong,
                            ) as nghttp3_duration as ::core::ffi::c_ulonglong,
                    ) as ::core::ffi::c_ulong,
            ),
    ) as uint64_t;
    *rlim = nghttp3_ratelim {
        burst: burst,
        rate: rate,
        tokens: burst,
        carry: 0,
        ts: ts,
    };
}
unsafe extern "C" fn ratelim_update(
    mut rlim: *mut nghttp3_ratelim,
    mut ts: nghttp3_tstamp,
) {
    let mut d: uint64_t = 0;
    let mut gain: uint64_t = 0;
    let mut gps: uint64_t = 0;
    '_c2rust_label: {
        if ts >= (*rlim).ts {} else {
            __assert_fail(
                b"ts >= rlim->ts\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_ratelim.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                49 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    if ts == (*rlim).ts {
        return;
    }
    d = ts.wrapping_sub((*rlim).ts) as uint64_t;
    (*rlim).ts = ts;
    if (*rlim).rate
        <= (UINT64_MAX as uint64_t).wrapping_sub((*rlim).carry).wrapping_div(d)
    {
        gain = (*rlim).rate.wrapping_mul(d).wrapping_add((*rlim).carry);
        gps = gain.wrapping_div(NGHTTP3_SECONDS);
        if gps < (*rlim).burst && (*rlim).tokens < (*rlim).burst.wrapping_sub(gps) {
            (*rlim).tokens = (*rlim).tokens.wrapping_add(gps);
            (*rlim).carry = gain.wrapping_rem(NGHTTP3_SECONDS);
            return;
        }
    }
    (*rlim).tokens = (*rlim).burst;
    (*rlim).carry = 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_ratelim_drain(
    mut rlim: *mut nghttp3_ratelim,
    mut n: uint64_t,
    mut ts: nghttp3_tstamp,
) -> ::core::ffi::c_int {
    ratelim_update(rlim, ts);
    if (*rlim).tokens < n {
        return -1 as ::core::ffi::c_int;
    }
    (*rlim).tokens = (*rlim).tokens.wrapping_sub(n);
    return 0 as ::core::ffi::c_int;
}
