extern "C" {
    fn nghttp3_put_uvarint(p: *mut uint8_t, n: uint64_t) -> *mut uint8_t;
    fn nghttp3_put_uvarintlen(n: uint64_t) -> size_t;
    fn nghttp3_mem_malloc(
        mem: *const nghttp3_mem,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn nghttp3_mem_free(mem: *const nghttp3_mem, ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn nghttp3_cpymem(
        dest: *mut uint8_t,
        src: *const uint8_t,
        n: size_t,
    ) -> *mut uint8_t;
    fn nghttp3_downcase(s: *mut uint8_t, len: size_t);
}
pub type size_t = usize;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub struct nghttp3_vec {
    pub base: *mut uint8_t,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_nv {
    pub name: *const uint8_t,
    pub value: *const uint8_t,
    pub namelen: size_t,
    pub valuelen: size_t,
    pub flags: uint8_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nghttp3_qpack_indexing_strat(pub ::core::ffi::c_uint);
impl nghttp3_qpack_indexing_strat {
    pub const NGHTTP3_QPACK_INDEXING_STRAT_NONE: Self = Self(0);
    pub const NGHTTP3_QPACK_INDEXING_STRAT_EAGER: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_settings {
    pub max_field_section_size: uint64_t,
    pub qpack_max_dtable_capacity: size_t,
    pub qpack_encoder_max_dtable_capacity: size_t,
    pub qpack_blocked_streams: size_t,
    pub enable_connect_protocol: uint8_t,
    pub h3_datagram: uint8_t,
    pub origin_list: *const nghttp3_vec,
    pub glitch_ratelim_burst: uint64_t,
    pub glitch_ratelim_rate: uint64_t,
    pub qpack_indexing_strat: nghttp3_qpack_indexing_strat,
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct nghttp3_pri(pub C2Rust_nghttp3_pri_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_nghttp3_pri_Inner {
    pub urgency: uint32_t,
    pub inc: uint8_t,
}
#[allow(dead_code, non_upper_case_globals)]
const C2Rust_nghttp3_pri_PADDING: usize = ::core::mem::size_of::<nghttp3_pri>()
    - ::core::mem::size_of::<C2Rust_nghttp3_pri_Inner>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_frame_headers {
    pub r#type: uint64_t,
    pub nva: *mut nghttp3_nv,
    pub nvlen: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_settings_entry {
    pub id: uint64_t,
    pub value: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_frame_settings {
    pub r#type: uint64_t,
    pub niv: size_t,
    pub iv: *mut nghttp3_settings_entry,
    pub local_settings: *const nghttp3_settings,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_frame_goaway {
    pub r#type: uint64_t,
    pub id: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_frame_priority_update {
    pub r#type: uint64_t,
    pub pri_elem_id: int64_t,
    pub c2rust_unnamed: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub c2rust_unnamed: C2Rust_Unnamed_0,
    pub pri: nghttp3_pri,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub data: *mut uint8_t,
    pub datalen: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_frame_origin {
    pub r#type: uint64_t,
    pub origin_list: nghttp3_vec,
}
pub const NGHTTP3_ERR_NOMEM: ::core::ffi::c_int = -901 as ::core::ffi::c_int;
pub const NGHTTP3_NV_FLAG_NO_COPY_NAME: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const NGHTTP3_NV_FLAG_NO_COPY_VALUE: ::core::ffi::c_uint = 0x4
    as ::core::ffi::c_uint;
pub const NGHTTP3_FRAME_SETTINGS: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const NGHTTP3_FRAME_GOAWAY: ::core::ffi::c_uint = 0x7 as ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[no_mangle]
pub unsafe extern "C" fn nghttp3_frame_write_hd(
    mut p: *mut uint8_t,
    mut r#type: uint64_t,
    mut payloadlen: uint64_t,
) -> *mut uint8_t {
    p = nghttp3_put_uvarint(p, r#type);
    p = nghttp3_put_uvarint(p, payloadlen);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_frame_write_hd_len(
    mut r#type: uint64_t,
    mut payloadlen: uint64_t,
) -> size_t {
    return nghttp3_put_uvarintlen(r#type)
        .wrapping_add(nghttp3_put_uvarintlen(payloadlen));
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_frame_write_settings(
    mut p: *mut uint8_t,
    mut fr: *const nghttp3_frame_settings,
    mut payloadlen: uint64_t,
) -> *mut uint8_t {
    let mut i: size_t = 0;
    p = nghttp3_frame_write_hd(p, (*fr).r#type, payloadlen);
    i = 0 as size_t;
    while i < (*fr).niv {
        p = nghttp3_put_uvarint(p, (*(*fr).iv.offset(i as isize)).id);
        p = nghttp3_put_uvarint(p, (*(*fr).iv.offset(i as isize)).value);
        i = i.wrapping_add(1);
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_frame_write_settings_len(
    mut ppayloadlen: *mut uint64_t,
    mut fr: *const nghttp3_frame_settings,
) -> size_t {
    let mut payloadlen: size_t = 0 as size_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < (*fr).niv {
        payloadlen = payloadlen
            .wrapping_add(
                nghttp3_put_uvarintlen((*(*fr).iv.offset(i as isize)).id)
                    .wrapping_add(
                        nghttp3_put_uvarintlen((*(*fr).iv.offset(i as isize)).value),
                    ),
            );
        i = i.wrapping_add(1);
    }
    *ppayloadlen = payloadlen as uint64_t;
    return nghttp3_put_uvarintlen(NGHTTP3_FRAME_SETTINGS as uint64_t)
        .wrapping_add(nghttp3_put_uvarintlen(payloadlen as uint64_t))
        .wrapping_add(payloadlen);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_frame_write_goaway(
    mut p: *mut uint8_t,
    mut fr: *const nghttp3_frame_goaway,
    mut payloadlen: uint64_t,
) -> *mut uint8_t {
    p = nghttp3_frame_write_hd(p, (*fr).r#type, payloadlen);
    p = nghttp3_put_uvarint(p, (*fr).id as uint64_t);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_frame_write_goaway_len(
    mut ppayloadlen: *mut uint64_t,
    mut fr: *const nghttp3_frame_goaway,
) -> size_t {
    let mut payloadlen: size_t = nghttp3_put_uvarintlen((*fr).id as uint64_t);
    *ppayloadlen = payloadlen as uint64_t;
    return nghttp3_put_uvarintlen(NGHTTP3_FRAME_GOAWAY as uint64_t)
        .wrapping_add(nghttp3_put_uvarintlen(payloadlen as uint64_t))
        .wrapping_add(payloadlen);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_frame_write_priority_update(
    mut p: *mut uint8_t,
    mut fr: *const nghttp3_frame_priority_update,
    mut payloadlen: uint64_t,
) -> *mut uint8_t {
    p = nghttp3_frame_write_hd(p, (*fr).r#type, payloadlen);
    p = nghttp3_put_uvarint(p, (*fr).pri_elem_id as uint64_t);
    if (*fr).c2rust_unnamed.c2rust_unnamed.datalen != 0 {
        p = nghttp3_cpymem(
            p,
            (*fr).c2rust_unnamed.c2rust_unnamed.data,
            (*fr).c2rust_unnamed.c2rust_unnamed.datalen,
        );
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_frame_write_priority_update_len(
    mut ppayloadlen: *mut uint64_t,
    mut fr: *const nghttp3_frame_priority_update,
) -> size_t {
    let mut payloadlen: size_t = nghttp3_put_uvarintlen((*fr).pri_elem_id as uint64_t)
        .wrapping_add((*fr).c2rust_unnamed.c2rust_unnamed.datalen);
    *ppayloadlen = payloadlen as uint64_t;
    return nghttp3_put_uvarintlen((*fr).r#type)
        .wrapping_add(nghttp3_put_uvarintlen(payloadlen as uint64_t))
        .wrapping_add(payloadlen);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_frame_write_origin(
    mut p: *mut uint8_t,
    mut fr: *const nghttp3_frame_origin,
    mut payloadlen: uint64_t,
) -> *mut uint8_t {
    p = nghttp3_frame_write_hd(p, (*fr).r#type, payloadlen);
    if (*fr).origin_list.len != 0 {
        p = nghttp3_cpymem(p, (*fr).origin_list.base, (*fr).origin_list.len);
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_frame_write_origin_len(
    mut ppayloadlen: *mut uint64_t,
    mut fr: *const nghttp3_frame_origin,
) -> size_t {
    let mut payloadlen: size_t = (*fr).origin_list.len;
    *ppayloadlen = payloadlen as uint64_t;
    return nghttp3_put_uvarintlen((*fr).r#type)
        .wrapping_add(nghttp3_put_uvarintlen(payloadlen as uint64_t))
        .wrapping_add(payloadlen);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_nva_copy(
    mut pnva: *mut *mut nghttp3_nv,
    mut nva: *const nghttp3_nv,
    mut nvlen: size_t,
    mut mem: *const nghttp3_mem,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut buflen: size_t = 0 as size_t;
    let mut p: *mut nghttp3_nv = ::core::ptr::null_mut::<nghttp3_nv>();
    if nvlen == 0 as size_t {
        *pnva = ::core::ptr::null_mut::<nghttp3_nv>();
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as size_t;
    while i < nvlen {
        if (*nva.offset(i as isize)).flags as ::core::ffi::c_uint
            & NGHTTP3_NV_FLAG_NO_COPY_NAME == 0 as ::core::ffi::c_uint
        {
            buflen = buflen
                .wrapping_add(
                    (*nva.offset(i as isize)).namelen.wrapping_add(1 as size_t),
                );
        }
        if (*nva.offset(i as isize)).flags as ::core::ffi::c_uint
            & NGHTTP3_NV_FLAG_NO_COPY_VALUE == 0 as ::core::ffi::c_uint
        {
            buflen = buflen
                .wrapping_add(
                    (*nva.offset(i as isize)).valuelen.wrapping_add(1 as size_t),
                );
        }
        i = i.wrapping_add(1);
    }
    buflen = (buflen as ::core::ffi::c_ulong)
        .wrapping_add(
            ::core::mem::size_of::<nghttp3_nv>().wrapping_mul(nvlen)
                as ::core::ffi::c_ulong,
        ) as size_t;
    *pnva = nghttp3_mem_malloc(mem, buflen) as *mut nghttp3_nv;
    if (*pnva).is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    p = *pnva;
    data = (*pnva as *mut uint8_t)
        .offset(::core::mem::size_of::<nghttp3_nv>().wrapping_mul(nvlen) as isize);
    i = 0 as size_t;
    while i < nvlen {
        (*p).flags = (*nva.offset(i as isize)).flags;
        if (*nva.offset(i as isize)).flags as ::core::ffi::c_uint
            & NGHTTP3_NV_FLAG_NO_COPY_NAME != 0
        {
            (*p).name = (*nva.offset(i as isize)).name;
            (*p).namelen = (*nva.offset(i as isize)).namelen;
        } else {
            if (*nva.offset(i as isize)).namelen != 0 {
                memcpy(
                    data as *mut ::core::ffi::c_void,
                    (*nva.offset(i as isize)).name as *const ::core::ffi::c_void,
                    (*nva.offset(i as isize)).namelen,
                );
                nghttp3_downcase(data, (*nva.offset(i as isize)).namelen);
            }
            (*p).name = data;
            (*p).namelen = (*nva.offset(i as isize)).namelen;
            *data.offset((*p).namelen as isize) = '\0' as uint8_t;
            data = data
                .offset(
                    (*nva.offset(i as isize)).namelen.wrapping_add(1 as size_t) as isize,
                );
        }
        if (*nva.offset(i as isize)).flags as ::core::ffi::c_uint
            & NGHTTP3_NV_FLAG_NO_COPY_VALUE != 0
        {
            (*p).value = (*nva.offset(i as isize)).value;
            (*p).valuelen = (*nva.offset(i as isize)).valuelen;
        } else {
            if (*nva.offset(i as isize)).valuelen != 0 {
                memcpy(
                    data as *mut ::core::ffi::c_void,
                    (*nva.offset(i as isize)).value as *const ::core::ffi::c_void,
                    (*nva.offset(i as isize)).valuelen,
                );
            }
            (*p).value = data;
            (*p).valuelen = (*nva.offset(i as isize)).valuelen;
            *data.offset((*p).valuelen as isize) = '\0' as uint8_t;
            data = data
                .offset(
                    (*nva.offset(i as isize)).valuelen.wrapping_add(1 as size_t) as isize,
                );
        }
        p = p.offset(1);
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_nva_del(
    mut nva: *mut nghttp3_nv,
    mut mem: *const nghttp3_mem,
) {
    nghttp3_mem_free(mem, nva as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_frame_headers_free(
    mut fr: *mut nghttp3_frame_headers,
    mut mem: *const nghttp3_mem,
) {
    if fr.is_null() {
        return;
    }
    nghttp3_nva_del((*fr).nva, mem);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_frame_priority_update_free(
    mut fr: *mut nghttp3_frame_priority_update,
    mut mem: *const nghttp3_mem,
) {
    if fr.is_null() {
        return;
    }
    nghttp3_mem_free(
        mem,
        (*fr).c2rust_unnamed.c2rust_unnamed.data as *mut ::core::ffi::c_void,
    );
}
