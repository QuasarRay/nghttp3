#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type nghttp3_debug_vprintf_callback = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, ::core::ffi::VaList) -> (),
>;
#[no_mangle]
pub unsafe extern "C" fn nghttp3_set_debug_vprintf_callback(
    mut debug_vprintf_callback: nghttp3_debug_vprintf_callback,
) {}
