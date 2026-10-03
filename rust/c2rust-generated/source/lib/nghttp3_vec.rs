pub type size_t = usize;
pub type uint8_t = u8;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_vec {
    pub base: *mut uint8_t,
    pub len: size_t,
}
pub const NGHTTP3_MAX_VARINT: ::core::ffi::c_ulonglong = ((1 as ::core::ffi::c_ulonglong)
    << 62 as ::core::ffi::c_int)
    .wrapping_sub(1 as ::core::ffi::c_ulonglong);
#[no_mangle]
pub unsafe extern "C" fn nghttp3_vec_len(
    mut vec: *const nghttp3_vec,
    mut n: size_t,
) -> uint64_t {
    let mut i: size_t = 0;
    let mut res: uint64_t = 0 as uint64_t;
    i = 0 as size_t;
    while i < n {
        res = (res as ::core::ffi::c_ulong)
            .wrapping_add((*vec.offset(i as isize)).len as ::core::ffi::c_ulong)
            as uint64_t;
        i = i.wrapping_add(1);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_vec_len_uvarint(
    mut dest: *mut uint64_t,
    mut vec: *const nghttp3_vec,
    mut n: size_t,
) -> ::core::ffi::c_int {
    let mut res: uint64_t = 0 as uint64_t;
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < n {
        len = (*vec.offset(i as isize)).len;
        if len as ::core::ffi::c_ulonglong
            > NGHTTP3_MAX_VARINT.wrapping_sub(res as ::core::ffi::c_ulonglong)
        {
            return -1 as ::core::ffi::c_int;
        }
        res = (res as ::core::ffi::c_ulong).wrapping_add(len as ::core::ffi::c_ulong)
            as uint64_t;
        i = i.wrapping_add(1);
    }
    *dest = res;
    return 0 as ::core::ffi::c_int;
}
