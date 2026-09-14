impl<C: Callbacks> Connection<C> {
    pub fn new(role: Role, settings: &Settings, callbacks: C) -> Result<Self> {
        let mut state = Box::new(CallbackState {
            callbacks,
            bodies: HashMap::new(),
        });
        let mut raw = ptr::null_mut();
        let raw_callbacks = callback_table::<C>();
        let user_data = (&mut *state as *mut CallbackState<C>).cast::<c_void>();

        // SAFETY: all pointers remain valid for the call. nghttp3 copies the
        // callback table and settings; `state` is boxed and remains stable for
        // the entire connection lifetime.
        let rv = unsafe {
            match role {
                Role::Client => sys::nghttp3_conn_client_new_versioned(
                    &mut raw,
                    sys::NGHTTP3_CALLBACKS_VERSION as i32,
                    &raw_callbacks,
                    sys::NGHTTP3_SETTINGS_VERSION as i32,
                    &settings.raw,
                    ptr::null(),
                    user_data,
                ),
                Role::Server => sys::nghttp3_conn_server_new_versioned(
                    &mut raw,
                    sys::NGHTTP3_CALLBACKS_VERSION as i32,
                    &raw_callbacks,
                    sys::NGHTTP3_SETTINGS_VERSION as i32,
                    &settings.raw,
                    ptr::null(),
                    user_data,
                ),
            }
        };
        cvt(rv)?;

        Ok(Self {
            raw: NonNull::new(raw).expect("nghttp3 returned success with a null connection"),
            state,
            role,
            control_bound: false,
            qpack_bound: false,
            last_timestamp_ns: None,
            _not_send_or_sync: PhantomData,
        })
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn callbacks(&self) -> &C {
        &self.state.callbacks
    }

    pub fn callbacks_mut(&mut self) -> &mut C {
        &mut self.state.callbacks
    }

    pub fn bind_control_stream(&mut self, stream_id: i64) -> Result<()> {
        // SAFETY: `self.raw` is a live owned nghttp3 connection.
        cvt(unsafe { sys::nghttp3_conn_bind_control_stream(self.raw.as_ptr(), stream_id) })?;
        self.control_bound = true;
        Ok(())
    }

    pub fn bind_qpack_streams(
        &mut self,
        encoder_stream_id: i64,
        decoder_stream_id: i64,
    ) -> Result<()> {
        // SAFETY: `self.raw` is a live owned nghttp3 connection.
        cvt(unsafe {
            sys::nghttp3_conn_bind_qpack_streams(
                self.raw.as_ptr(),
                encoder_stream_id,
                decoder_stream_id,
            )
        })?;
        self.qpack_bound = true;
        Ok(())
    }

    pub fn read_stream(
        &mut self,
        stream_id: i64,
        data: &[u8],
        fin: bool,
        timestamp_ns: u64,
    ) -> Result<usize> {
        if self
            .last_timestamp_ns
            .is_some_and(|previous| timestamp_ns < previous)
        {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
        }
        self.last_timestamp_ns = Some(timestamp_ns);

        // SAFETY: `data` remains valid for the duration of the call and nghttp3
        // does not retain this input pointer.
        let rv = unsafe {
            sys::nghttp3_conn_read_stream2(
                self.raw.as_ptr(),
                stream_id,
                data.as_ptr(),
                data.len(),
                i32::from(fin),
                timestamp_ns,
            )
        };
        cvt_ssize(rv)
    }

    pub fn next_write(&mut self, max_segments: usize) -> Result<Option<WriteData>> {
        if max_segments == 0 {
            return Ok(None);
        }

        let mut raw_vecs = vec![sys::nghttp3_vec { base: ptr::null_mut(), len: 0 }; max_segments];
        let mut stream_id = -1_i64;
        let mut fin = 0_i32;

        // SAFETY: nghttp3 writes at most `raw_vecs.len()` elements and the
        // returned buffers stay valid until the connection is mutated again.
        let count = unsafe {
            sys::nghttp3_conn_writev_stream(
                self.raw.as_ptr(),
                &mut stream_id,
                &mut fin,
                raw_vecs.as_mut_ptr(),
                raw_vecs.len(),
            )
        };
        let count = cvt_ssize(count)?;

        if stream_id == -1 {
            return Ok(None);
        }

        let mut segments = Vec::with_capacity(count);
        for raw in raw_vecs.into_iter().take(count) {
            // Copying here keeps the safe API independent from nghttp3's
            // internal buffer lifetime and permits the caller to hand bytes to
            // an asynchronous QUIC transport.
            segments.push(unsafe { bytes_from_raw(raw.base.cast_const(), raw.len) }.to_vec());
        }

        Ok(Some(WriteData {
            stream_id,
            fin: fin != 0,
            segments,
        }))
    }

    pub fn add_write_offset(&mut self, stream_id: i64, accepted: usize) -> Result<()> {
        // SAFETY: `self.raw` is live; nghttp3 validates stream state.
        cvt(unsafe {
            sys::nghttp3_conn_add_write_offset(self.raw.as_ptr(), stream_id, accepted)
        })
    }

    pub fn add_ack_offset(&mut self, stream_id: i64, acknowledged: u64) -> Result<()> {
        // SAFETY: `self.raw` is live; nghttp3 validates stream state.
        cvt(unsafe {
            sys::nghttp3_conn_add_ack_offset(self.raw.as_ptr(), stream_id, acknowledged)
        })
    }

    pub fn update_ack_offset(&mut self, stream_id: i64, offset: u64) -> Result<()> {
        // SAFETY: `self.raw` is live; nghttp3 validates stream state.
        cvt(unsafe {
            sys::nghttp3_conn_update_ack_offset(self.raw.as_ptr(), stream_id, offset)
        })
    }

    pub fn block_stream(&mut self, stream_id: i64) {
        // SAFETY: `self.raw` is live.
        unsafe { sys::nghttp3_conn_block_stream(self.raw.as_ptr(), stream_id) }
    }

    pub fn unblock_stream(&mut self, stream_id: i64) -> Result<()> {
        // SAFETY: `self.raw` is live.
        cvt(unsafe { sys::nghttp3_conn_unblock_stream(self.raw.as_ptr(), stream_id) })
    }

    pub fn resume_stream(&mut self, stream_id: i64) -> Result<()> {
        // SAFETY: `self.raw` is live.
        cvt(unsafe { sys::nghttp3_conn_resume_stream(self.raw.as_ptr(), stream_id) })
    }

    pub fn shutdown_stream_write(&mut self, stream_id: i64) {
        // SAFETY: `self.raw` is live.
        unsafe { sys::nghttp3_conn_shutdown_stream_write(self.raw.as_ptr(), stream_id) }
    }

    pub fn shutdown_stream_read(&mut self, stream_id: i64) -> Result<()> {
        // SAFETY: `self.raw` is live.
        cvt(unsafe { sys::nghttp3_conn_shutdown_stream_read(self.raw.as_ptr(), stream_id) })
    }

    pub fn is_stream_writable(&self, stream_id: i64) -> bool {
        // SAFETY: immutable query on a live connection.
        unsafe { sys::nghttp3_conn_is_stream_writable2(self.raw.as_ptr(), stream_id) != 0 }
    }

    pub fn is_stream_flushed(&self, stream_id: i64) -> bool {
        // SAFETY: immutable query on a live connection.
        unsafe { sys::nghttp3_conn_is_stream_flushed(self.raw.as_ptr(), stream_id) != 0 }
    }

    pub fn close_stream(
        &mut self,
        stream_id: i64,
        rx_app_error_code: Option<u64>,
        tx_app_error_code: Option<u64>,
    ) -> Result<()> {
        let mut flags = sys::NGHTTP3_STREAM_CLOSE_FLAG_NONE as u32;
        let rx = match rx_app_error_code {
            Some(code) => {
                flags |= sys::NGHTTP3_STREAM_CLOSE_FLAG_RX_APP_ERROR_CODE_SET as u32;
                code
            }
            None => 0,
        };
        let tx = match tx_app_error_code {
            Some(code) => {
                flags |= sys::NGHTTP3_STREAM_CLOSE_FLAG_TX_APP_ERROR_CODE_SET as u32;
                code
            }
            None => 0,
        };

        // SAFETY: `self.raw` is live; flags match which error-code fields are set.
        cvt(unsafe {
            sys::nghttp3_conn_close_stream2(self.raw.as_ptr(), flags, stream_id, rx, tx)
        })
    }

    pub fn set_max_client_streams_bidi(&mut self, max_streams: u64) {
        // SAFETY: `self.raw` is live.
        unsafe { sys::nghttp3_conn_set_max_client_streams_bidi(self.raw.as_ptr(), max_streams) }
    }

    pub fn set_max_concurrent_streams(&mut self, max_streams: usize) {
        // SAFETY: `self.raw` is live.
        unsafe { sys::nghttp3_conn_set_max_concurrent_streams(self.raw.as_ptr(), max_streams) }
    }

    pub fn submit_request(
        &mut self,
        stream_id: i64,
        headers: &[Header],
        body: Option<Vec<u8>>,
    ) -> Result<()> {
        if self.role != Role::Client || !self.qpack_bound {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_STATE));
        }
        self.submit_with_body(stream_id, headers, body, true)
    }

    pub fn submit_response(
        &mut self,
        stream_id: i64,
        headers: &[Header],
        body: Option<Vec<u8>>,
    ) -> Result<()> {
        if self.role != Role::Server || !self.qpack_bound {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_STATE));
        }
        self.submit_with_body(stream_id, headers, body, false)
    }

    pub fn submit_info(&mut self, stream_id: i64, headers: &[Header]) -> Result<()> {
        if self.role != Role::Server || !self.qpack_bound {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_STATE));
        }
        let raw_headers = raw_headers(headers);
        // SAFETY: raw header pointers remain valid for the call and nghttp3 copies them.
        cvt(unsafe {
            sys::nghttp3_conn_submit_info(
                self.raw.as_ptr(),
                stream_id,
                raw_headers.as_ptr(),
                raw_headers.len(),
            )
        })
    }

    pub fn submit_trailers(&mut self, stream_id: i64, headers: &[Header]) -> Result<()> {
        if !self.qpack_bound {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_STATE));
        }
        let raw_headers = raw_headers(headers);
        // SAFETY: raw header pointers remain valid for the call and nghttp3 copies them.
        cvt(unsafe {
            sys::nghttp3_conn_submit_trailers(
                self.raw.as_ptr(),
                stream_id,
                raw_headers.as_ptr(),
                raw_headers.len(),
            )
        })
    }

    pub fn submit_shutdown_notice(&mut self) -> Result<()> {
        if !self.control_bound {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_STATE));
        }
        // SAFETY: `self.raw` is live.
        cvt(unsafe { sys::nghttp3_conn_submit_shutdown_notice(self.raw.as_ptr()) })
    }

    pub fn shutdown(&mut self) -> Result<()> {
        if !self.control_bound {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_STATE));
        }
        // SAFETY: `self.raw` is live.
        cvt(unsafe { sys::nghttp3_conn_shutdown(self.raw.as_ptr()) })
    }

    pub fn is_drained(&self) -> bool {
        // This query is only defined for servers; make misuse a harmless false.
        self.role == Role::Server
            // SAFETY: immutable query on a live server connection.
            && unsafe { sys::nghttp3_conn_is_drained2(self.raw.as_ptr()) != 0 }
    }

    fn submit_with_body(
        &mut self,
        stream_id: i64,
        headers: &[Header],
        body: Option<Vec<u8>>,
        request: bool,
    ) -> Result<()> {
        let raw_headers = raw_headers(headers);
        if body.is_some() && self.state.bodies.contains_key(&stream_id) {
            return Err(Error(sys::NGHTTP3_ERR_STREAM_IN_USE));
        }
        let reader = sys::nghttp3_data_reader {
            read_data: Some(read_data_trampoline::<C>),
        };
        let reader_ptr = if let Some(body) = body {
            self.state.bodies.insert(
                stream_id,
                BodyState {
                    data: body.into_boxed_slice(),
                    offset: 0,
                },
            );
            &reader as *const _
        } else {
            ptr::null()
        };

        // SAFETY: nghttp3 copies headers and the reader function pointer. Body
        // bytes live in `self.state.bodies` until the stream-close callback.
        let rv = unsafe {
            if request {
                sys::nghttp3_conn_submit_request(
                    self.raw.as_ptr(),
                    stream_id,
                    raw_headers.as_ptr(),
                    raw_headers.len(),
                    reader_ptr,
                    ptr::null_mut(),
                )
            } else {
                sys::nghttp3_conn_submit_response(
                    self.raw.as_ptr(),
                    stream_id,
                    raw_headers.as_ptr(),
                    raw_headers.len(),
                    reader_ptr,
                )
            }
        };

        if rv != 0 {
            self.state.bodies.remove(&stream_id);
        }
        cvt(rv)
    }
}

impl<C: Callbacks> Drop for Connection<C> {
    fn drop(&mut self) {
        // SAFETY: this wrapper exclusively owns the connection and drops it once.
        unsafe { sys::nghttp3_conn_del(self.raw.as_ptr()) }
    }
}

fn raw_headers(headers: &[Header]) -> Vec<sys::nghttp3_nv> {
    headers.iter().map(Header::as_raw).collect()
}

/// Returns whether `name` is a valid lowercase HTTP/3 field name.
pub fn is_valid_header_name(name: &[u8]) -> bool {
    // SAFETY: `name` is valid for the duration of this read-only call.
    unsafe { sys::nghttp3_check_header_name(name.as_ptr(), name.len()) != 0 }
}

/// Returns whether `value` is a valid HTTP field value.
pub fn is_valid_header_value(value: &[u8]) -> bool {
    // SAFETY: `value` is valid for the duration of this read-only call.
    unsafe { sys::nghttp3_check_header_value(value.as_ptr(), value.len()) != 0 }
}

/// Returns the runtime nghttp3 version string.
pub fn version() -> &'static str {
    // SAFETY: nghttp3 returns a pointer to static version metadata for 0.
    let info = unsafe { sys::nghttp3_version(0) };
    if info.is_null() {
        return "unknown";
    }
    // SAFETY: version_str is a static, NUL-terminated string owned by nghttp3.
    unsafe { CStr::from_ptr((*info).version_str) }
        .to_str()
        .unwrap_or("unknown")
}

/// Parses an RFC 9218 Priority field value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Priority {
    pub urgency: u32,
    pub incremental: bool,
}

impl Default for Priority {
    fn default() -> Self {
        Self {
            urgency: 3,
            incremental: false,
        }
    }
}

impl Priority {
    pub fn parse(value: &[u8]) -> Result<Self> {
        let mut raw = sys::nghttp3_pri {
            urgency: Self::default().urgency,
            inc: u8::from(Self::default().incremental),
        };
        // SAFETY: input and output pointers are valid for the duration of the call.
        cvt(unsafe {
            sys::nghttp3_pri_parse_priority_versioned(
                sys::NGHTTP3_PRI_VERSION as i32,
                &mut raw,
                value.as_ptr(),
                value.len(),
            )
        })?;
        Ok(Self {
            urgency: raw.urgency,
            incremental: raw.inc != 0,
        })
    }
}
