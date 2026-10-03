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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[no_mangle]
pub unsafe extern "C" fn nghttp3_opl_init(mut opl: *mut nghttp3_opl) {
    (*opl).head = ::core::ptr::null_mut::<nghttp3_opl_entry>();
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_opl_push(
    mut opl: *mut nghttp3_opl,
    mut ent: *mut nghttp3_opl_entry,
) {
    (*ent).next = (*opl).head;
    (*opl).head = ent;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_opl_pop(
    mut opl: *mut nghttp3_opl,
) -> *mut nghttp3_opl_entry {
    let mut ent: *mut nghttp3_opl_entry = (*opl).head;
    if ent.is_null() {
        return ::core::ptr::null_mut::<nghttp3_opl_entry>();
    }
    (*opl).head = (*ent).next;
    return ent;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_opl_clear(mut opl: *mut nghttp3_opl) {
    (*opl).head = ::core::ptr::null_mut::<nghttp3_opl_entry>();
}
