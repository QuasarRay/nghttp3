extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
}
pub type size_t = usize;
pub type ssize_t = isize;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[no_mangle]
pub unsafe extern "C" fn nghttp3_unreachable_fail(
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut func: *const ::core::ffi::c_char,
) -> ! {
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut buflen: size_t = 0;
    let mut rv: ::core::ffi::c_int = 0;
    rv = snprintf(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
        NGHTTP3_UNREACHABLE_TEMPLATE.as_ptr(),
        file,
        line,
        func,
    );
    if rv < 0 as ::core::ffi::c_int {
        abort();
    }
    buflen = (rv as size_t).wrapping_add(1 as size_t);
    buf = malloc(buflen) as *mut ::core::ffi::c_char;
    if buf.is_null() {
        abort();
    }
    rv = snprintf(buf, buflen, NGHTTP3_UNREACHABLE_TEMPLATE.as_ptr(), file, line, func);
    if rv < 0 as ::core::ffi::c_int {
        abort();
    }
    while write(STDERR_FILENO, buf as *const ::core::ffi::c_void, rv as size_t)
        == -1 as ssize_t && *__errno_location() == EINTR
    {}
    free(buf as *mut ::core::ffi::c_void);
    abort();
}
pub const NGHTTP3_UNREACHABLE_TEMPLATE: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<
        [u8; 24],
        [::core::ffi::c_char; 24],
    >(*b"%s:%d %s: Unreachable.\n\0")
};
