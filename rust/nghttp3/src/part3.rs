fn callback_table<C: Callbacks>() -> sys::nghttp3_callbacks {
    sys::nghttp3_callbacks {
        acked_stream_data: Some(acked_stream_data_trampoline::<C>),
        stream_close: None,
        recv_data: Some(recv_data_trampoline::<C>),
        deferred_consume: Some(deferred_consume_trampoline::<C>),
        begin_headers: Some(begin_headers_trampoline::<C>),
        recv_header: Some(recv_header_trampoline::<C>),
        end_headers: Some(end_headers_trampoline::<C>),
        begin_trailers: Some(begin_trailers_trampoline::<C>),
        recv_trailer: Some(recv_trailer_trampoline::<C>),
        end_trailers: Some(end_trailers_trampoline::<C>),
        stop_sending: Some(stop_sending_trampoline::<C>),
        end_stream: Some(end_stream_trampoline::<C>),
        reset_stream: Some(reset_stream_trampoline::<C>),
        shutdown: Some(shutdown_trampoline::<C>),
        recv_settings: None,
        recv_origin: Some(recv_origin_trampoline::<C>),
        end_origin: Some(end_origin_trampoline::<C>),
        rand: Some(rand_trampoline),
        recv_settings2: Some(recv_settings_trampoline::<C>),
        stream_close2: Some(stream_close_trampoline::<C>),
    }
}

fn invoke<C: Callbacks>(
    user_data: *mut c_void,
    f: impl FnOnce(&mut CallbackState<C>) -> Result<()>,
) -> i32 {
    if user_data.is_null() {
        return sys::NGHTTP3_ERR_CALLBACK_FAILURE;
    }

    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // SAFETY: user_data always points to the boxed `CallbackState<C>` owned
        // by the live `Connection<C>`.
        let state = unsafe { &mut *user_data.cast::<CallbackState<C>>() };
        f(state)
    }));

    match outcome {
        Ok(Ok(())) => 0,
        Ok(Err(err)) if err.code() == sys::NGHTTP3_ERR_CALLBACK_FAILURE => err.code(),
        Ok(Err(_)) | Err(_) => sys::NGHTTP3_ERR_CALLBACK_FAILURE,
    }
}

unsafe fn bytes_from_raw<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    if len == 0 {
        &[]
    } else {
        // SAFETY: caller guarantees a non-null pointer to `len` readable bytes.
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

unsafe fn rcbuf_bytes<'a>(buf: *const sys::nghttp3_rcbuf) -> &'a [u8] {
    if buf.is_null() {
        return &[];
    }
    // SAFETY: `buf` is a live callback-owned rcbuf for the duration of the call.
    let raw = unsafe { sys::nghttp3_rcbuf_get_buf(buf) };
    // SAFETY: nghttp3 guarantees the returned vector describes readable bytes.
    unsafe { bytes_from_raw(raw.base.cast_const(), raw.len) }
}

unsafe extern "C" fn rand_trampoline(dest: *mut u8, destlen: usize) {
    if destlen == 0 {
        return;
    }
    if dest.is_null() {
        std::process::abort();
    }
    // SAFETY: nghttp3 supplies a writable output buffer of `destlen` bytes.
    let dest = unsafe { slice::from_raw_parts_mut(dest, destlen) };
    if getrandom::fill(dest).is_err() {
        // nghttp3's callback has no error return. Failing closed avoids silently
        // falling back to a predictable hash seed on an unsupported platform.
        std::process::abort();
    }
}

unsafe extern "C" fn acked_stream_data_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    stream_id: i64,
    datalen: u64,
    conn_user_data: *mut c_void,
    _stream_user_data: *mut c_void,
) -> i32 {
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        let result = state.callbacks.acked_stream_data(stream_id, datalen);

        // After read_data returns EOF, `offset` is the number of body bytes
        // still retained for nghttp3. acked_stream_data is the API's explicit
        // signal that those application-owned bytes are safe to release.
        let release = if let Some(body) = state.bodies.get_mut(&stream_id) {
            let acked = usize::try_from(datalen).unwrap_or(usize::MAX);
            body.offset = body.offset.saturating_sub(acked);
            body.offset == 0
        } else {
            false
        };
        if release {
            state.bodies.remove(&stream_id);
        }

        result
    })
}

unsafe extern "C" fn stream_close_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    flags: u32,
    stream_id: i64,
    rx_app_error_code: u64,
    tx_app_error_code: u64,
    conn_user_data: *mut c_void,
    _stream_user_data: *mut c_void,
) -> i32 {
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        let close = StreamClose {
            stream_id,
            rx_app_error_code: ((flags
                & sys::NGHTTP3_STREAM_CLOSE_FLAG_RX_APP_ERROR_CODE_SET)
                != 0)
                .then_some(rx_app_error_code),
            tx_app_error_code: ((flags
                & sys::NGHTTP3_STREAM_CLOSE_FLAG_TX_APP_ERROR_CODE_SET)
                != 0)
                .then_some(tx_app_error_code),
        };
        state.callbacks.stream_close(close)
    })
}

unsafe extern "C" fn recv_data_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    stream_id: i64,
    data: *const u8,
    datalen: usize,
    conn_user_data: *mut c_void,
    _stream_user_data: *mut c_void,
) -> i32 {
    // SAFETY: nghttp3 provides a readable callback buffer of `datalen` bytes.
    let data = unsafe { bytes_from_raw(data, datalen) };
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state.callbacks.recv_data(stream_id, data)
    })
}

unsafe extern "C" fn deferred_consume_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    stream_id: i64,
    consumed: usize,
    conn_user_data: *mut c_void,
    _stream_user_data: *mut c_void,
) -> i32 {
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state.callbacks.deferred_consume(stream_id, consumed)
    })
}

unsafe extern "C" fn begin_headers_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    stream_id: i64,
    conn_user_data: *mut c_void,
    _stream_user_data: *mut c_void,
) -> i32 {
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state.callbacks.begin_headers(stream_id)
    })
}

unsafe extern "C" fn recv_header_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    stream_id: i64,
    token: i32,
    name: *mut sys::nghttp3_rcbuf,
    value: *mut sys::nghttp3_rcbuf,
    flags: u8,
    conn_user_data: *mut c_void,
    _stream_user_data: *mut c_void,
) -> i32 {
    // SAFETY: rcbufs remain live for the duration of this callback.
    let name = unsafe { rcbuf_bytes(name) };
    // SAFETY: rcbufs remain live for the duration of this callback.
    let value = unsafe { rcbuf_bytes(value) };
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state
            .callbacks
            .recv_header(stream_id, token, name, value, flags)
    })
}

unsafe extern "C" fn end_headers_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    stream_id: i64,
    fin: i32,
    conn_user_data: *mut c_void,
    _stream_user_data: *mut c_void,
) -> i32 {
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state.callbacks.end_headers(stream_id, fin != 0)
    })
}

unsafe extern "C" fn begin_trailers_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    stream_id: i64,
    conn_user_data: *mut c_void,
    _stream_user_data: *mut c_void,
) -> i32 {
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state.callbacks.begin_trailers(stream_id)
    })
}

unsafe extern "C" fn recv_trailer_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    stream_id: i64,
    token: i32,
    name: *mut sys::nghttp3_rcbuf,
    value: *mut sys::nghttp3_rcbuf,
    flags: u8,
    conn_user_data: *mut c_void,
    _stream_user_data: *mut c_void,
) -> i32 {
    // SAFETY: rcbufs remain live for the duration of this callback.
    let name = unsafe { rcbuf_bytes(name) };
    // SAFETY: rcbufs remain live for the duration of this callback.
    let value = unsafe { rcbuf_bytes(value) };
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state
            .callbacks
            .recv_trailer(stream_id, token, name, value, flags)
    })
}

unsafe extern "C" fn end_trailers_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    stream_id: i64,
    fin: i32,
    conn_user_data: *mut c_void,
    _stream_user_data: *mut c_void,
) -> i32 {
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state.callbacks.end_trailers(stream_id, fin != 0)
    })
}

unsafe extern "C" fn stop_sending_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    stream_id: i64,
    app_error_code: u64,
    conn_user_data: *mut c_void,
    _stream_user_data: *mut c_void,
) -> i32 {
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state.callbacks.stop_sending(stream_id, app_error_code)
    })
}

unsafe extern "C" fn end_stream_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    stream_id: i64,
    conn_user_data: *mut c_void,
    _stream_user_data: *mut c_void,
) -> i32 {
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state.callbacks.end_stream(stream_id)
    })
}

unsafe extern "C" fn reset_stream_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    stream_id: i64,
    app_error_code: u64,
    conn_user_data: *mut c_void,
    _stream_user_data: *mut c_void,
) -> i32 {
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state.callbacks.reset_stream(stream_id, app_error_code)
    })
}

unsafe extern "C" fn shutdown_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    id: i64,
    conn_user_data: *mut c_void,
) -> i32 {
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state.callbacks.shutdown(id)
    })
}

unsafe extern "C" fn recv_settings_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    settings: *const sys::nghttp3_proto_settings,
    conn_user_data: *mut c_void,
) -> i32 {
    if settings.is_null() {
        return sys::NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    // SAFETY: nghttp3 provides a valid settings pointer for this callback.
    let settings = unsafe { PeerSettings::from_raw(settings) };
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state.callbacks.recv_settings(settings)
    })
}

unsafe extern "C" fn recv_origin_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    origin: *const u8,
    originlen: usize,
    conn_user_data: *mut c_void,
) -> i32 {
    // SAFETY: nghttp3 provides a readable callback buffer of `originlen` bytes.
    let origin = unsafe { bytes_from_raw(origin, originlen) };
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state.callbacks.recv_origin(origin)
    })
}

unsafe extern "C" fn end_origin_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    conn_user_data: *mut c_void,
) -> i32 {
    invoke(conn_user_data, |state: &mut CallbackState<C>| {
        state.callbacks.end_origin()
    })
}

unsafe extern "C" fn read_data_trampoline<C: Callbacks>(
    _conn: *mut sys::nghttp3_conn,
    stream_id: i64,
    vec: *mut sys::nghttp3_vec,
    veccnt: usize,
    pflags: *mut u32,
    conn_user_data: *mut c_void,
    _stream_user_data: *mut c_void,
) -> sys::nghttp3_ssize {
    if conn_user_data.is_null() || pflags.is_null() {
        return sys::NGHTTP3_ERR_CALLBACK_FAILURE as sys::nghttp3_ssize;
    }

    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // SAFETY: the pointer is the live boxed state owned by Connection<C>.
        let state = unsafe { &mut *conn_user_data.cast::<CallbackState<C>>() };
        let Some(body) = state.bodies.get_mut(&stream_id) else {
            // SAFETY: pflags is validated non-null above.
            unsafe { *pflags = sys::NGHTTP3_DATA_FLAG_EOF };
            return 0;
        };

        if body.offset == body.data.len() {
            // SAFETY: pflags is validated non-null above.
            unsafe { *pflags = sys::NGHTTP3_DATA_FLAG_EOF };
            return 0;
        }

        if vec.is_null() || veccnt == 0 {
            return sys::NGHTTP3_ERR_CALLBACK_FAILURE as sys::nghttp3_ssize;
        }

        let remaining = &mut body.data[body.offset..];
        // SAFETY: vec points to at least one output slot. The boxed body buffer
        // is stable and retained until acked_stream_data says its bytes are safe
        // to release (or until the connection is dropped).
        unsafe {
            (*vec).base = remaining.as_mut_ptr();
            (*vec).len = remaining.len();
            *pflags = sys::NGHTTP3_DATA_FLAG_EOF;
        }
        body.offset = body.data.len();
        1
    }));

    match outcome {
        Ok(value) => value,
        Err(_) => sys::NGHTTP3_ERR_CALLBACK_FAILURE as sys::nghttp3_ssize,
    }
}
