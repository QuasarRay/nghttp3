//! Raw FFI bindings to nghttp3.
//!
//! The bindings are generated from the repository's public `nghttp3.h`
//! at build time and the bundled C library is linked statically. Public
//! function-like C macros are reproduced below as Rust forwarding functions.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    improper_ctypes
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Rust equivalent of the `nghttp3_settings_default` C macro.
///
/// # Safety
/// `settings` must satisfy the requirements of
/// `nghttp3_settings_default_versioned`.
#[inline]
pub unsafe fn nghttp3_settings_default(settings: *mut nghttp3_settings) {
    unsafe { nghttp3_settings_default_versioned(NGHTTP3_SETTINGS_VERSION as i32, settings) }
}

/// Rust equivalent of the `nghttp3_conn_client_new` C macro.
///
/// # Safety
/// All pointers must satisfy the requirements of
/// `nghttp3_conn_client_new_versioned`.
#[inline]
pub unsafe fn nghttp3_conn_client_new(
    pconn: *mut *mut nghttp3_conn,
    callbacks: *const nghttp3_callbacks,
    settings: *const nghttp3_settings,
    mem: *const nghttp3_mem,
    user_data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    unsafe {
        nghttp3_conn_client_new_versioned(
            pconn,
            NGHTTP3_CALLBACKS_VERSION as i32,
            callbacks,
            NGHTTP3_SETTINGS_VERSION as i32,
            settings,
            mem,
            user_data,
        )
    }
}

/// Rust equivalent of the `nghttp3_conn_server_new` C macro.
///
/// # Safety
/// All pointers must satisfy the requirements of
/// `nghttp3_conn_server_new_versioned`.
#[inline]
pub unsafe fn nghttp3_conn_server_new(
    pconn: *mut *mut nghttp3_conn,
    callbacks: *const nghttp3_callbacks,
    settings: *const nghttp3_settings,
    mem: *const nghttp3_mem,
    user_data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    unsafe {
        nghttp3_conn_server_new_versioned(
            pconn,
            NGHTTP3_CALLBACKS_VERSION as i32,
            callbacks,
            NGHTTP3_SETTINGS_VERSION as i32,
            settings,
            mem,
            user_data,
        )
    }
}

/// Rust equivalent of the `nghttp3_conn_set_server_stream_priority` C macro.
///
/// # Safety
/// Arguments must satisfy the requirements of the versioned function.
#[inline]
pub unsafe fn nghttp3_conn_set_server_stream_priority(
    conn: *mut nghttp3_conn,
    stream_id: i64,
    pri: *const nghttp3_pri,
) -> ::std::os::raw::c_int {
    unsafe {
        nghttp3_conn_set_server_stream_priority_versioned(
            conn,
            stream_id,
            NGHTTP3_PRI_VERSION as i32,
            pri,
        )
    }
}

/// Rust equivalent of the `nghttp3_conn_get_stream_priority` C macro.
///
/// # Safety
/// Arguments must satisfy the requirements of the versioned function.
#[inline]
pub unsafe fn nghttp3_conn_get_stream_priority(
    conn: *mut nghttp3_conn,
    dest: *mut nghttp3_pri,
    stream_id: i64,
) -> ::std::os::raw::c_int {
    unsafe {
        nghttp3_conn_get_stream_priority_versioned(
            conn,
            NGHTTP3_PRI_VERSION as i32,
            dest,
            stream_id,
        )
    }
}

/// Rust equivalent of the `nghttp3_conn_get_stream_priority2` C macro.
///
/// # Safety
/// Arguments must satisfy the requirements of the versioned function.
#[inline]
pub unsafe fn nghttp3_conn_get_stream_priority2(
    conn: *const nghttp3_conn,
    dest: *mut nghttp3_pri,
    stream_id: i64,
) -> ::std::os::raw::c_int {
    unsafe {
        nghttp3_conn_get_stream_priority2_versioned(
            conn,
            NGHTTP3_PRI_VERSION as i32,
            dest,
            stream_id,
        )
    }
}

/// Rust equivalent of the `nghttp3_pri_parse_priority` C macro.
///
/// # Safety
/// `dest` and `value` must satisfy the requirements of the versioned function.
#[inline]
pub unsafe fn nghttp3_pri_parse_priority(
    dest: *mut nghttp3_pri,
    value: *const u8,
    len: usize,
) -> ::std::os::raw::c_int {
    unsafe {
        nghttp3_pri_parse_priority_versioned(NGHTTP3_PRI_VERSION as i32, dest, value, len)
    }
}
