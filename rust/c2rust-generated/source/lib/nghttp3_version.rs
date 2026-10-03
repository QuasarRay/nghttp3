#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_info {
    pub age: ::core::ffi::c_int,
    pub version_num: ::core::ffi::c_int,
    pub version_str: *const ::core::ffi::c_char,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NGHTTP3_VERSION: [::core::ffi::c_char; 8] = unsafe {
    ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"1.18.90\0")
};
pub const NGHTTP3_VERSION_NUM: ::core::ffi::c_int = 0x1125a as ::core::ffi::c_int;
pub const NGHTTP3_VERSION_AGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut version: nghttp3_info = nghttp3_info {
    age: NGHTTP3_VERSION_AGE,
    version_num: NGHTTP3_VERSION_NUM,
    version_str: NGHTTP3_VERSION.as_ptr(),
};
#[no_mangle]
pub unsafe extern "C" fn nghttp3_version(
    mut least_version: ::core::ffi::c_int,
) -> *const nghttp3_info {
    if least_version > NGHTTP3_VERSION_NUM {
        return ::core::ptr::null::<nghttp3_info>();
    }
    return &raw mut version;
}
