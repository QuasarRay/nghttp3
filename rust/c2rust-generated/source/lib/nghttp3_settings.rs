extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn nghttp3_unreachable_fail(
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_vec {
    pub base: *mut uint8_t,
    pub len: size_t,
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
pub const NGHTTP3_SETTINGS_V1: ::core::ffi::c_int = 1;
pub const NGHTTP3_SETTINGS_V2: ::core::ffi::c_int = 2;
pub const NGHTTP3_SETTINGS_V3: ::core::ffi::c_int = 3;
pub const NGHTTP3_SETTINGS_V4: ::core::ffi::c_int = 4;
pub const NGHTTP3_SETTINGS_VERSION: ::core::ffi::c_int = NGHTTP3_SETTINGS_V4;
pub const NGHTTP3_DEFAULT_GLITCH_RATELIM_BURST: ::core::ffi::c_int = 1000
    as ::core::ffi::c_int;
pub const NGHTTP3_DEFAULT_GLITCH_RATELIM_RATE: ::core::ffi::c_int = 33
    as ::core::ffi::c_int;
pub const NGHTTP3_VARINT_MAX: ::core::ffi::c_ulonglong = ((1 as ::core::ffi::c_ulonglong)
    << 62 as ::core::ffi::c_int)
    .wrapping_sub(1 as ::core::ffi::c_ulonglong);
pub const NGHTTP3_QPACK_ENCODER_MAX_DTABLE_CAPACITY: ::core::ffi::c_int = 4096
    as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn nghttp3_settings_default_versioned(
    mut settings_version: ::core::ffi::c_int,
    mut settings: *mut nghttp3_settings,
) {
    let mut len: size_t = nghttp3_settingslen_version(settings_version);
    memset(settings as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, len);
    's_31: {
        match settings_version {
            NGHTTP3_SETTINGS_VERSION | NGHTTP3_SETTINGS_V3 => {
                (*settings).glitch_ratelim_burst = NGHTTP3_DEFAULT_GLITCH_RATELIM_BURST
                    as uint64_t;
                (*settings).glitch_ratelim_rate = NGHTTP3_DEFAULT_GLITCH_RATELIM_RATE
                    as uint64_t;
            }
            NGHTTP3_SETTINGS_V2 | NGHTTP3_SETTINGS_V1 => {}
            _ => {
                break 's_31;
            }
        }
        (*settings).max_field_section_size = NGHTTP3_VARINT_MAX as uint64_t;
        (*settings).qpack_encoder_max_dtable_capacity = NGHTTP3_QPACK_ENCODER_MAX_DTABLE_CAPACITY
            as size_t;
    };
}
unsafe extern "C" fn settings_copy(
    mut dest: *mut nghttp3_settings,
    mut src: *const nghttp3_settings,
    mut settings_version: ::core::ffi::c_int,
) {
    '_c2rust_label: {
        if settings_version != 4 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"settings_version != NGHTTP3_SETTINGS_VERSION\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_settings.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                61 as ::core::ffi::c_uint,
                b"void settings_copy(nghttp3_settings *, const nghttp3_settings *, int)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    memcpy(
        dest as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        nghttp3_settingslen_version(settings_version),
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_settings_convert_to_latest(
    mut dest: *mut nghttp3_settings,
    mut settings_version: ::core::ffi::c_int,
    mut src: *const nghttp3_settings,
) -> *const nghttp3_settings {
    if settings_version == NGHTTP3_SETTINGS_VERSION {
        return src;
    }
    nghttp3_settings_default_versioned(NGHTTP3_SETTINGS_VERSION, dest);
    settings_copy(dest, src, settings_version);
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_settings_convert_to_old(
    mut settings_version: ::core::ffi::c_int,
    mut dest: *mut nghttp3_settings,
    mut src: *const nghttp3_settings,
) {
    '_c2rust_label: {
        if settings_version != 4 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"settings_version != NGHTTP3_SETTINGS_VERSION\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_settings.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                83 as ::core::ffi::c_uint,
                b"void nghttp3_settings_convert_to_old(int, nghttp3_settings *, const nghttp3_settings *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    settings_copy(dest, src, settings_version);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_settingslen_version(
    mut settings_version: ::core::ffi::c_int,
) -> size_t {
    let mut settings: nghttp3_settings = nghttp3_settings {
        max_field_section_size: 0,
        qpack_max_dtable_capacity: 0,
        qpack_encoder_max_dtable_capacity: 0,
        qpack_blocked_streams: 0,
        enable_connect_protocol: 0,
        h3_datagram: 0,
        origin_list: ::core::ptr::null::<nghttp3_vec>(),
        glitch_ratelim_burst: 0,
        glitch_ratelim_rate: 0,
        qpack_indexing_strat: nghttp3_qpack_indexing_strat::NGHTTP3_QPACK_INDEXING_STRAT_NONE,
    };
    match settings_version {
        NGHTTP3_SETTINGS_VERSION => return ::core::mem::size_of::<nghttp3_settings>(),
        NGHTTP3_SETTINGS_V3 => {
            return (56 as size_t).wrapping_add(::core::mem::size_of::<uint64_t>());
        }
        NGHTTP3_SETTINGS_V2 => {
            return (40 as size_t)
                .wrapping_add(::core::mem::size_of::<*const nghttp3_vec>());
        }
        NGHTTP3_SETTINGS_V1 => {
            return (33 as size_t).wrapping_add(::core::mem::size_of::<uint8_t>());
        }
        _ => {
            nghttp3_unreachable_fail(
                b"nghttp3_settings.c\0".as_ptr() as *const ::core::ffi::c_char,
                104 as ::core::ffi::c_int,
                b"nghttp3_settingslen_version\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
