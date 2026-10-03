extern "C" {
    fn nghttp3_mem_free(mem: *const nghttp3_mem, ptr: *mut ::core::ffi::c_void);
    fn nghttp3_mem_realloc(
        mem: *const nghttp3_mem,
        ptr: *mut ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type ptrdiff_t = isize;
pub type nghttp3_ssize = ptrdiff_t;
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
pub struct nghttp3_buf {
    pub begin: *mut uint8_t,
    pub end: *mut uint8_t,
    pub pos: *mut uint8_t,
    pub last: *mut uint8_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nghttp3_buf_type(pub ::core::ffi::c_uint);
impl nghttp3_buf_type {
    pub const NGHTTP3_BUF_TYPE_PRIVATE: Self = Self(0);
    pub const NGHTTP3_BUF_TYPE_SHARED: Self = Self(1);
    pub const NGHTTP3_BUF_TYPE_ALIEN: Self = Self(2);
    pub const NGHTTP3_BUF_TYPE_ALIEN_NO_ACK: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_typed_buf {
    pub buf: nghttp3_buf,
    pub r#type: nghttp3_buf_type,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NGHTTP3_ERR_NOMEM: ::core::ffi::c_int = -901 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn nghttp3_buf_init(mut buf: *mut nghttp3_buf) {
    (*buf).last = ::core::ptr::null_mut::<uint8_t>();
    (*buf).pos = (*buf).last;
    (*buf).end = (*buf).pos;
    (*buf).begin = (*buf).end;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_buf_wrap_init(
    mut buf: *mut nghttp3_buf,
    mut src: *mut uint8_t,
    mut len: size_t,
) {
    (*buf).last = src;
    (*buf).pos = (*buf).last;
    (*buf).begin = (*buf).pos;
    (*buf).end = (*buf).begin.offset(len as isize);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_buf_free(
    mut buf: *mut nghttp3_buf,
    mut mem: *const nghttp3_mem,
) {
    nghttp3_mem_free(mem, (*buf).begin as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_buf_left(mut buf: *const nghttp3_buf) -> size_t {
    return (*buf).end.offset_from((*buf).last) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_buf_len(mut buf: *const nghttp3_buf) -> size_t {
    return (*buf).last.offset_from((*buf).pos) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_buf_cap(mut buf: *const nghttp3_buf) -> size_t {
    return (*buf).end.offset_from((*buf).begin) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_buf_offset(mut buf: *const nghttp3_buf) -> size_t {
    return (*buf).pos.offset_from((*buf).begin) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_buf_reset(mut buf: *mut nghttp3_buf) {
    (*buf).last = (*buf).begin;
    (*buf).pos = (*buf).last;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_buf_reserve(
    mut buf: *mut nghttp3_buf,
    mut size: size_t,
    mut mem: *const nghttp3_mem,
) -> ::core::ffi::c_int {
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut pos_offset: nghttp3_ssize = 0;
    let mut last_offset: nghttp3_ssize = 0;
    if (*buf).end.offset_from((*buf).begin) as size_t >= size {
        return 0 as ::core::ffi::c_int;
    }
    pos_offset = (*buf).pos.offset_from((*buf).begin) as nghttp3_ssize;
    last_offset = (*buf).last.offset_from((*buf).begin) as nghttp3_ssize;
    p = nghttp3_mem_realloc(mem, (*buf).begin as *mut ::core::ffi::c_void, size)
        as *mut uint8_t;
    if p.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    *buf = nghttp3_buf {
        begin: p,
        end: p.offset(size as isize),
        pos: p.offset(pos_offset as isize),
        last: p.offset(last_offset as isize),
    };
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_buf_swap(
    mut a: *mut nghttp3_buf,
    mut b: *mut nghttp3_buf,
) {
    let mut c: nghttp3_buf = *a;
    *a = *b;
    *b = c;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_typed_buf_init(
    mut tbuf: *mut nghttp3_typed_buf,
    mut buf: *const nghttp3_buf,
    mut r#type: nghttp3_buf_type,
) {
    (*tbuf).buf = *buf;
    (*tbuf).r#type = r#type;
    (*tbuf).buf.begin = (*tbuf).buf.pos;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_typed_buf_shared_init(
    mut tbuf: *mut nghttp3_typed_buf,
    mut chunk: *const nghttp3_buf,
) {
    (*tbuf).buf = *chunk;
    (*tbuf).r#type = nghttp3_buf_type::NGHTTP3_BUF_TYPE_SHARED;
    (*tbuf).buf.pos = (*tbuf).buf.last;
    (*tbuf).buf.begin = (*tbuf).buf.pos;
}
