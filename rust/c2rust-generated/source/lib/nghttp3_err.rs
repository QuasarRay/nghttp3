pub type uint64_t = u64;
pub const NGHTTP3_ERR_INVALID_ARGUMENT: ::core::ffi::c_int = -101;
pub const NGHTTP3_ERR_INVALID_STATE: ::core::ffi::c_int = -102;
pub const NGHTTP3_ERR_WOULDBLOCK: ::core::ffi::c_int = -103;
pub const NGHTTP3_ERR_STREAM_IN_USE: ::core::ffi::c_int = -104;
pub const NGHTTP3_ERR_MALFORMED_HTTP_HEADER: ::core::ffi::c_int = -105;
pub const NGHTTP3_ERR_REMOVE_HTTP_HEADER: ::core::ffi::c_int = -106;
pub const NGHTTP3_ERR_MALFORMED_HTTP_MESSAGING: ::core::ffi::c_int = -107;
pub const NGHTTP3_ERR_QPACK_FATAL: ::core::ffi::c_int = -108;
pub const NGHTTP3_ERR_QPACK_HEADER_TOO_LARGE: ::core::ffi::c_int = -109;
pub const NGHTTP3_ERR_STREAM_NOT_FOUND: ::core::ffi::c_int = -110;
pub const NGHTTP3_ERR_CONN_CLOSING: ::core::ffi::c_int = -111;
pub const NGHTTP3_ERR_STREAM_DATA_OVERFLOW: ::core::ffi::c_int = -112;
pub const NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED: ::core::ffi::c_int = -401;
pub const NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR: ::core::ffi::c_int = -402;
pub const NGHTTP3_ERR_QPACK_DECODER_STREAM_ERROR: ::core::ffi::c_int = -403;
pub const NGHTTP3_ERR_H3_FRAME_UNEXPECTED: ::core::ffi::c_int = -601;
pub const NGHTTP3_ERR_H3_FRAME_ERROR: ::core::ffi::c_int = -602;
pub const NGHTTP3_ERR_H3_MISSING_SETTINGS: ::core::ffi::c_int = -603;
pub const NGHTTP3_ERR_H3_INTERNAL_ERROR: ::core::ffi::c_int = -604;
pub const NGHTTP3_ERR_H3_CLOSED_CRITICAL_STREAM: ::core::ffi::c_int = -605;
pub const NGHTTP3_ERR_H3_GENERAL_PROTOCOL_ERROR: ::core::ffi::c_int = -606;
pub const NGHTTP3_ERR_H3_ID_ERROR: ::core::ffi::c_int = -607;
pub const NGHTTP3_ERR_H3_SETTINGS_ERROR: ::core::ffi::c_int = -608;
pub const NGHTTP3_ERR_H3_STREAM_CREATION_ERROR: ::core::ffi::c_int = -609;
pub const NGHTTP3_ERR_H3_EXCESSIVE_LOAD: ::core::ffi::c_int = -610;
pub const NGHTTP3_ERR_FATAL: ::core::ffi::c_int = -900 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_NOMEM: ::core::ffi::c_int = -901;
pub const NGHTTP3_ERR_CALLBACK_FAILURE: ::core::ffi::c_int = -902;
pub const NGHTTP3_H3_NO_ERROR: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const NGHTTP3_H3_GENERAL_PROTOCOL_ERROR: ::core::ffi::c_int = 0x101
    as ::core::ffi::c_int;
pub const NGHTTP3_H3_INTERNAL_ERROR: ::core::ffi::c_int = 0x102 as ::core::ffi::c_int;
pub const NGHTTP3_H3_STREAM_CREATION_ERROR: ::core::ffi::c_int = 0x103
    as ::core::ffi::c_int;
pub const NGHTTP3_H3_CLOSED_CRITICAL_STREAM: ::core::ffi::c_int = 0x104
    as ::core::ffi::c_int;
pub const NGHTTP3_H3_FRAME_UNEXPECTED: ::core::ffi::c_int = 0x105 as ::core::ffi::c_int;
pub const NGHTTP3_H3_FRAME_ERROR: ::core::ffi::c_int = 0x106 as ::core::ffi::c_int;
pub const NGHTTP3_H3_EXCESSIVE_LOAD: ::core::ffi::c_int = 0x107 as ::core::ffi::c_int;
pub const NGHTTP3_H3_ID_ERROR: ::core::ffi::c_int = 0x108 as ::core::ffi::c_int;
pub const NGHTTP3_H3_SETTINGS_ERROR: ::core::ffi::c_int = 0x109 as ::core::ffi::c_int;
pub const NGHTTP3_H3_MISSING_SETTINGS: ::core::ffi::c_int = 0x10a as ::core::ffi::c_int;
pub const NGHTTP3_H3_MESSAGE_ERROR: ::core::ffi::c_int = 0x10e as ::core::ffi::c_int;
pub const NGHTTP3_QPACK_DECOMPRESSION_FAILED: ::core::ffi::c_int = 0x200
    as ::core::ffi::c_int;
pub const NGHTTP3_QPACK_ENCODER_STREAM_ERROR: ::core::ffi::c_int = 0x201
    as ::core::ffi::c_int;
pub const NGHTTP3_QPACK_DECODER_STREAM_ERROR: ::core::ffi::c_int = 0x202
    as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn nghttp3_strerror(
    mut liberr: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    match liberr {
        NGHTTP3_ERR_INVALID_ARGUMENT => {
            return b"ERR_INVALID_ARGUMENT\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_INVALID_STATE => {
            return b"ERR_INVALID_STATE\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_WOULDBLOCK => {
            return b"ERR_WOULDBLOCK\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_STREAM_IN_USE => {
            return b"ERR_STREAM_IN_USE\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_MALFORMED_HTTP_HEADER => {
            return b"ERR_MALFORMED_HTTP_HEADER\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_REMOVE_HTTP_HEADER => {
            return b"ERR_REMOVE_HTTP_HEADER\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_MALFORMED_HTTP_MESSAGING => {
            return b"ERR_MALFORMED_HTTP_MESSAGING\0".as_ptr()
                as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_QPACK_FATAL => {
            return b"ERR_QPACK_FATAL\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_QPACK_HEADER_TOO_LARGE => {
            return b"ERR_QPACK_HEADER_TOO_LARGE\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_STREAM_NOT_FOUND => {
            return b"ERR_STREAM_NOT_FOUND\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_CONN_CLOSING => {
            return b"ERR_CONN_CLOSING\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_STREAM_DATA_OVERFLOW => {
            return b"ERR_STREAM_DATA_OVERFLOW\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED => {
            return b"ERR_QPACK_DECOMPRESSION_FAILED\0".as_ptr()
                as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR => {
            return b"ERR_QPACK_ENCODER_STREAM_ERROR\0".as_ptr()
                as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_QPACK_DECODER_STREAM_ERROR => {
            return b"ERR_QPACK_DECODER_STREAM_ERROR\0".as_ptr()
                as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_H3_FRAME_UNEXPECTED => {
            return b"ERR_H3_FRAME_UNEXPECTED\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_H3_FRAME_ERROR => {
            return b"ERR_H3_FRAME_ERROR\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_H3_MISSING_SETTINGS => {
            return b"ERR_H3_MISSING_SETTINGS\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_H3_INTERNAL_ERROR => {
            return b"ERR_H3_INTERNAL_ERROR\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_H3_CLOSED_CRITICAL_STREAM => {
            return b"ERR_CLOSED_CRITICAL_STREAM\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_H3_GENERAL_PROTOCOL_ERROR => {
            return b"ERR_H3_GENERAL_PROTOCOL_ERROR\0".as_ptr()
                as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_H3_ID_ERROR => {
            return b"ERR_H3_ID_ERROR\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_H3_SETTINGS_ERROR => {
            return b"ERR_H3_SETTINGS_ERROR\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_H3_STREAM_CREATION_ERROR => {
            return b"ERR_H3_STREAM_CREATION_ERROR\0".as_ptr()
                as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_H3_EXCESSIVE_LOAD => {
            return b"ERR_H3_EXCESSIVE_LOAD\0".as_ptr() as *const ::core::ffi::c_char;
        }
        NGHTTP3_ERR_NOMEM => return b"ERR_NOMEM\0".as_ptr() as *const ::core::ffi::c_char,
        NGHTTP3_ERR_CALLBACK_FAILURE => {
            return b"ERR_CALLBACK_FAILURE\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return b"(unknown)\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_err_infer_quic_app_error_code(
    mut liberr: ::core::ffi::c_int,
) -> uint64_t {
    match liberr {
        0 => return NGHTTP3_H3_NO_ERROR as uint64_t,
        NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED => {
            return NGHTTP3_QPACK_DECOMPRESSION_FAILED as uint64_t;
        }
        NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR => {
            return NGHTTP3_QPACK_ENCODER_STREAM_ERROR as uint64_t;
        }
        NGHTTP3_ERR_QPACK_DECODER_STREAM_ERROR => {
            return NGHTTP3_QPACK_DECODER_STREAM_ERROR as uint64_t;
        }
        NGHTTP3_ERR_H3_FRAME_UNEXPECTED => return NGHTTP3_H3_FRAME_UNEXPECTED as uint64_t,
        NGHTTP3_ERR_H3_FRAME_ERROR => return NGHTTP3_H3_FRAME_ERROR as uint64_t,
        NGHTTP3_ERR_H3_MISSING_SETTINGS => return NGHTTP3_H3_MISSING_SETTINGS as uint64_t,
        NGHTTP3_ERR_H3_INTERNAL_ERROR
        | NGHTTP3_ERR_NOMEM
        | NGHTTP3_ERR_CALLBACK_FAILURE
        | NGHTTP3_ERR_QPACK_FATAL
        | NGHTTP3_ERR_QPACK_HEADER_TOO_LARGE
        | NGHTTP3_ERR_STREAM_DATA_OVERFLOW => {
            return NGHTTP3_H3_INTERNAL_ERROR as uint64_t;
        }
        NGHTTP3_ERR_H3_CLOSED_CRITICAL_STREAM => {
            return NGHTTP3_H3_CLOSED_CRITICAL_STREAM as uint64_t;
        }
        NGHTTP3_ERR_H3_GENERAL_PROTOCOL_ERROR => {
            return NGHTTP3_H3_GENERAL_PROTOCOL_ERROR as uint64_t;
        }
        NGHTTP3_ERR_H3_ID_ERROR => return NGHTTP3_H3_ID_ERROR as uint64_t,
        NGHTTP3_ERR_H3_SETTINGS_ERROR => return NGHTTP3_H3_SETTINGS_ERROR as uint64_t,
        NGHTTP3_ERR_H3_STREAM_CREATION_ERROR => {
            return NGHTTP3_H3_STREAM_CREATION_ERROR as uint64_t;
        }
        NGHTTP3_ERR_H3_EXCESSIVE_LOAD => return NGHTTP3_H3_EXCESSIVE_LOAD as uint64_t,
        NGHTTP3_ERR_MALFORMED_HTTP_HEADER | NGHTTP3_ERR_MALFORMED_HTTP_MESSAGING => {
            return NGHTTP3_H3_MESSAGE_ERROR as uint64_t;
        }
        _ => return NGHTTP3_H3_GENERAL_PROTOCOL_ERROR as uint64_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_err_is_fatal(
    mut liberr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (liberr < NGHTTP3_ERR_FATAL) as ::core::ffi::c_int;
}
