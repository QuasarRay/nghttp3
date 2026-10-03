extern "C" {
    pub type nghttp3_rcbuf;
    pub type nghttp3_conn;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint32_t = u32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_proto_settings {
    pub max_field_section_size: uint64_t,
    pub qpack_max_dtable_capacity: size_t,
    pub qpack_blocked_streams: size_t,
    pub enable_connect_protocol: uint8_t,
    pub h3_datagram: uint8_t,
}
pub type nghttp3_acked_stream_data = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        uint64_t,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_stream_close = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        uint64_t,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_recv_data = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        *const uint8_t,
        size_t,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_deferred_consume = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        size_t,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_begin_headers = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_recv_header = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        int32_t,
        *mut nghttp3_rcbuf,
        *mut nghttp3_rcbuf,
        uint8_t,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_end_headers = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        ::core::ffi::c_int,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_end_stream = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_stop_sending = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        uint64_t,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_reset_stream = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        uint64_t,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_shutdown = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_recv_settings = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        *const nghttp3_settings,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_recv_origin = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        *const uint8_t,
        size_t,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_end_origin = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_rand = Option<unsafe extern "C" fn(*mut uint8_t, size_t) -> ()>;
pub type nghttp3_recv_settings2 = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        *const nghttp3_proto_settings,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_stream_close2 = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        uint32_t,
        int64_t,
        uint64_t,
        uint64_t,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_callbacks {
    pub acked_stream_data: nghttp3_acked_stream_data,
    pub stream_close: nghttp3_stream_close,
    pub recv_data: nghttp3_recv_data,
    pub deferred_consume: nghttp3_deferred_consume,
    pub begin_headers: nghttp3_begin_headers,
    pub recv_header: nghttp3_recv_header,
    pub end_headers: nghttp3_end_headers,
    pub begin_trailers: nghttp3_begin_headers,
    pub recv_trailer: nghttp3_recv_header,
    pub end_trailers: nghttp3_end_headers,
    pub stop_sending: nghttp3_stop_sending,
    pub end_stream: nghttp3_end_stream,
    pub reset_stream: nghttp3_reset_stream,
    pub shutdown: nghttp3_shutdown,
    pub recv_settings: nghttp3_recv_settings,
    pub recv_origin: nghttp3_recv_origin,
    pub end_origin: nghttp3_end_origin,
    pub rand: nghttp3_rand,
    pub recv_settings2: nghttp3_recv_settings2,
    pub stream_close2: nghttp3_stream_close2,
}
pub const NGHTTP3_CALLBACKS_V1: ::core::ffi::c_int = 1;
pub const NGHTTP3_CALLBACKS_V2: ::core::ffi::c_int = 2;
pub const NGHTTP3_CALLBACKS_V3: ::core::ffi::c_int = 3;
pub const NGHTTP3_CALLBACKS_V4: ::core::ffi::c_int = 4;
pub const NGHTTP3_CALLBACKS_VERSION: ::core::ffi::c_int = NGHTTP3_CALLBACKS_V4;
unsafe extern "C" fn callbacks_copy(
    mut dest: *mut nghttp3_callbacks,
    mut src: *const nghttp3_callbacks,
    mut callbacks_version: ::core::ffi::c_int,
) {
    '_c2rust_label: {
        if callbacks_version != 4 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"callbacks_version != NGHTTP3_CALLBACKS_VERSION\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_callbacks.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                35 as ::core::ffi::c_uint,
                b"void callbacks_copy(nghttp3_callbacks *, const nghttp3_callbacks *, int)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    memcpy(
        dest as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        nghttp3_callbackslen_version(callbacks_version),
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_callbacks_convert_to_latest(
    mut dest: *mut nghttp3_callbacks,
    mut callbacks_version: ::core::ffi::c_int,
    mut src: *const nghttp3_callbacks,
) -> *const nghttp3_callbacks {
    if callbacks_version == NGHTTP3_CALLBACKS_VERSION {
        return src;
    }
    *dest = nghttp3_callbacks {
        acked_stream_data: None,
        stream_close: None,
        recv_data: None,
        deferred_consume: None,
        begin_headers: None,
        recv_header: None,
        end_headers: None,
        begin_trailers: None,
        recv_trailer: None,
        end_trailers: None,
        stop_sending: None,
        end_stream: None,
        reset_stream: None,
        shutdown: None,
        recv_settings: None,
        recv_origin: None,
        end_origin: None,
        rand: None,
        recv_settings2: None,
        stream_close2: None,
    };
    callbacks_copy(dest, src, callbacks_version);
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_callbacks_convert_to_old(
    mut callbacks_version: ::core::ffi::c_int,
    mut dest: *mut nghttp3_callbacks,
    mut src: *const nghttp3_callbacks,
) {
    '_c2rust_label: {
        if callbacks_version != 4 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"callbacks_version != NGHTTP3_CALLBACKS_VERSION\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_callbacks.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                58 as ::core::ffi::c_uint,
                b"void nghttp3_callbacks_convert_to_old(int, nghttp3_callbacks *, const nghttp3_callbacks *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    callbacks_copy(dest, src, callbacks_version);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_callbackslen_version(
    mut callbacks_version: ::core::ffi::c_int,
) -> size_t {
    let mut callbacks: nghttp3_callbacks = nghttp3_callbacks {
        acked_stream_data: None,
        stream_close: None,
        recv_data: None,
        deferred_consume: None,
        begin_headers: None,
        recv_header: None,
        end_headers: None,
        begin_trailers: None,
        recv_trailer: None,
        end_trailers: None,
        stop_sending: None,
        end_stream: None,
        reset_stream: None,
        shutdown: None,
        recv_settings: None,
        recv_origin: None,
        end_origin: None,
        rand: None,
        recv_settings2: None,
        stream_close2: None,
    };
    match callbacks_version {
        NGHTTP3_CALLBACKS_VERSION => return ::core::mem::size_of::<nghttp3_callbacks>(),
        NGHTTP3_CALLBACKS_V3 => {
            return (144 as size_t)
                .wrapping_add(::core::mem::size_of::<nghttp3_recv_settings2>());
        }
        NGHTTP3_CALLBACKS_V2 => {
            return (136 as size_t).wrapping_add(::core::mem::size_of::<nghttp3_rand>());
        }
        NGHTTP3_CALLBACKS_V1 => {
            return (112 as size_t)
                .wrapping_add(::core::mem::size_of::<nghttp3_recv_settings>());
        }
        _ => {
            nghttp3_unreachable_fail(
                b"nghttp3_callbacks.c\0".as_ptr() as *const ::core::ffi::c_char,
                78 as ::core::ffi::c_int,
                b"nghttp3_callbackslen_version\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
