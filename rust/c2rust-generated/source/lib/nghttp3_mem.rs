extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
unsafe extern "C" fn default_malloc(
    mut size: size_t,
    mut user_data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    return malloc(size);
}
unsafe extern "C" fn default_free(
    mut ptr: *mut ::core::ffi::c_void,
    mut user_data: *mut ::core::ffi::c_void,
) {
    free(ptr);
}
unsafe extern "C" fn default_calloc(
    mut nmemb: size_t,
    mut size: size_t,
    mut user_data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    return calloc(nmemb, size);
}
unsafe extern "C" fn default_realloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut user_data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    return realloc(ptr, size);
}
static mut mem_default: nghttp3_mem = nghttp3_mem {
    user_data: NULL,
    malloc: Some(
        default_malloc
            as unsafe extern "C" fn(
                size_t,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
    ),
    free: Some(
        default_free
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> (),
    ),
    calloc: Some(
        default_calloc
            as unsafe extern "C" fn(
                size_t,
                size_t,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
    ),
    realloc: Some(
        default_realloc
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                size_t,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
    ),
};
#[no_mangle]
pub unsafe extern "C" fn nghttp3_mem_default() -> *const nghttp3_mem {
    return &raw mut mem_default;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_mem_malloc(
    mut mem: *const nghttp3_mem,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    return (*mem).malloc.expect("non-null function pointer")(size, (*mem).user_data);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_mem_free(
    mut mem: *const nghttp3_mem,
    mut ptr: *mut ::core::ffi::c_void,
) {
    (*mem).free.expect("non-null function pointer")(ptr, (*mem).user_data);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_mem_calloc(
    mut mem: *const nghttp3_mem,
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    return (*mem)
        .calloc
        .expect("non-null function pointer")(nmemb, size, (*mem).user_data);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_mem_realloc(
    mut mem: *const nghttp3_mem,
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    return (*mem)
        .realloc
        .expect("non-null function pointer")(ptr, size, (*mem).user_data);
}
