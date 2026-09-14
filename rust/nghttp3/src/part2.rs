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
            pending_write: None,
            sent_offsets: HashMap::new(),
            ack_offsets: HashMap::new(),
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
        if !is_local_uni_stream(self.role, stream_id) {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
        }
        // SAFETY: the stream ID satisfies nghttp3's local-unidirectional-stream
        // precondition and `self.raw` is a live owned connection.
        cvt(unsafe { sys::nghttp3_conn_bind_control_stream(self.raw.as_ptr(), stream_id) })?;
        self.control_bound = true;
        Ok(())
    }

    pub fn bind_qpack_streams(
        &mut self,
        encoder_stream_id: i64,
        decoder_stream_id: i64,
    ) -> Result<()> {
        if !is_local_uni_stream(self.role, encoder_stream_id)
            || !is_local_uni_stream(self.role, decoder_stream_id)
            || encoder_stream_id == decoder_stream_id
        {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
        }
        // SAFETY: both IDs satisfy nghttp3's local-unidirectional-stream
        // preconditions and are distinct.
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
        if !is_peer_readable_stream(self.role, stream_id)
            || timestamp_ns == u64::MAX
            || self
                .last_timestamp_ns
                .is_some_and(|previous| timestamp_ns < previous)
        {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
        }
        self.last_timestamp_ns = Some(timestamp_ns);

        // SAFETY: the stream ID and timestamp satisfy nghttp3's assertions;
        // `data` remains valid for the call and is not retained.
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
        if self.pending_write.is_some() {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_STATE));
        }

        let mut raw_vecs = Vec::with_capacity(max_segments);
        raw_vecs.resize_with(max_segments, || sys::nghttp3_vec {
            base: ptr::null_mut(),
            len: 0,
        });
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
        let offered = segments
            .iter()
            .try_fold(0_usize, |total, segment| total.checked_add(segment.len()))
            .ok_or(Error(sys::NGHTTP3_ERR_INVALID_STATE))?;
        self.pending_write = Some((stream_id, offered));

        Ok(Some(WriteData {
            stream_id,
            fin: fin != 0,
            segments,
        }))
    }

    pub fn add_write_offset(&mut self, stream_id: i64, accepted: usize) -> Result<()> {
        let Some((offered_stream_id, offered)) = self.pending_write else {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_STATE));
        };
        if stream_id != offered_stream_id || accepted > offered {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
        }
        let accepted = u64::try_from(accepted)
            .map_err(|_| Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT))?;
        let current = self.sent_offsets.get(&stream_id).copied().unwrap_or(0);
        let next = current
            .checked_add(accepted)
            .ok_or(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT))?;

        // SAFETY: accepted is bounded by the exact bytes returned from the
        // preceding writev call, preventing nghttp3's unsigned underflow path.
        cvt(unsafe {
            sys::nghttp3_conn_add_write_offset(self.raw.as_ptr(), stream_id, accepted as usize)
        })?;
        self.sent_offsets.insert(stream_id, next);
        self.pending_write = None;
        Ok(())
    }

    pub fn add_ack_offset(&mut self, stream_id: i64, acknowledged: u64) -> Result<()> {
        let sent = self.sent_offsets.get(&stream_id).copied().unwrap_or(0);
        let current = self.ack_offsets.get(&stream_id).copied().unwrap_or(0);
        let next = current
            .checked_add(acknowledged)
            .ok_or(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT))?;
        if next > sent {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
        }

        // SAFETY: cumulative acknowledged bytes are bounded by bytes previously
        // reported as accepted by the transport.
        cvt(unsafe {
            sys::nghttp3_conn_add_ack_offset(self.raw.as_ptr(), stream_id, acknowledged)
        })?;
        self.ack_offsets.insert(stream_id, next);
        Ok(())
    }

    pub fn update_ack_offset(&mut self, stream_id: i64, offset: u64) -> Result<()> {
        let sent = self.sent_offsets.get(&stream_id).copied().unwrap_or(0);
        let current = self.ack_offsets.get(&stream_id).copied().unwrap_or(0);
        if offset < current || offset > sent {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
        }

        // SAFETY: the absolute acknowledged offset is monotonic and no larger
        // than bytes previously accepted by the transport.
        cvt(unsafe {
            sys::nghttp3_conn_update_ack_offset(self.raw.as_ptr(), stream_id, offset)
        })?;
        self.ack_offsets.insert(stream_id, offset);
        Ok(())
    }

    pub fn block_stream(&mut self, stream_id: i64) {
        // SAFETY: `self.raw` is live and this API ignores unknown streams.
        unsafe { sys::nghttp3_conn_block_stream(self.raw.as_ptr(), stream_id) }
    }

    pub fn unblock_stream(&mut self, stream_id: i64) -> Result<()> {
        // SAFETY: `self.raw` is live and this API ignores unknown streams.
        cvt(unsafe { sys::nghttp3_conn_unblock_stream(self.raw.as_ptr(), stream_id) })
    }

    pub fn resume_stream(&mut self, stream_id: i64) -> Result<()> {
        // SAFETY: `self.raw` is live and this API ignores unknown streams.
        cvt(unsafe { sys::nghttp3_conn_resume_stream(self.raw.as_ptr(), stream_id) })
    }

    pub fn shutdown_stream_write(&mut self, stream_id: i64) {
        // SAFETY: `self.raw` is live and this API ignores unknown streams.
        unsafe { sys::nghttp3_conn_shutdown_stream_write(self.raw.as_ptr(), stream_id) }
    }

    pub fn shutdown_stream_read(&mut self, stream_id: i64) -> Result<()> {
        // SAFETY: `self.raw` is live; nghttp3 safely ignores inapplicable IDs.
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
        if self
            .pending_write
            .is_some_and(|(pending_stream, _)| pending_stream == stream_id)
        {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_STATE));
        }

        let mut flags = sys::NGHTTP3_STREAM_CLOSE_FLAG_NONE;
        let rx = match rx_app_error_code {
            Some(code) => {
                flags |= sys::NGHTTP3_STREAM_CLOSE_FLAG_RX_APP_ERROR_CODE_SET;
                code
            }
            None => 0,
        };
        let tx = match tx_app_error_code {
            Some(code) => {
                flags |= sys::NGHTTP3_STREAM_CLOSE_FLAG_TX_APP_ERROR_CODE_SET;
                code
            }
            None => 0,
        };

        // SAFETY: `self.raw` is live; flags match which error-code fields are set.
        cvt(unsafe {
            sys::nghttp3_conn_close_stream2(self.raw.as_ptr(), flags, stream_id, rx, tx)
        })?;
        self.sent_offsets.remove(&stream_id);
        self.ack_offsets.remove(&stream_id);
        Ok(())
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
        if !is_client_bidi_stream(stream_id) {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
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
        if !is_client_bidi_stream(stream_id) {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
        }
        self.submit_with_body(stream_id, headers, body, false)
    }

    pub fn submit_info(&mut self, stream_id: i64, headers: &[Header]) -> Result<()> {
        if self.role != Role::Server || !self.qpack_bound {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_STATE));
        }
        if !is_client_bidi_stream(stream_id) {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
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
        if !is_client_bidi_stream(stream_id) {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
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
        // SAFETY: `self.raw` is live and a control stream is bound.
        cvt(unsafe { sys::nghttp3_conn_submit_shutdown_notice(self.raw.as_ptr()) })
    }

    pub fn shutdown(&mut self) -> Result<()> {
        if !self.control_bound {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_STATE));
        }
        // SAFETY: `self.raw` is live and a control stream is bound.
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
        // bytes are boxed and retained through nghttp3's acknowledgement signal.
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

fn is_valid_stream_id(stream_id: i64) -> bool {
    (0..=MAX_VARINT as i64).contains(&stream_id)
}

fn is_client_bidi_stream(stream_id: i64) -> bool {
    is_valid_stream_id(stream_id) && (stream_id & 0x03) == 0
}

fn is_local_uni_stream(role: Role, stream_id: i64) -> bool {
    if !is_valid_stream_id(stream_id) {
        return false;
    }
    let kind = stream_id & 0x03;
    match role {
        Role::Client => kind == 2,
        Role::Server => kind == 3,
    }
}

fn is_peer_readable_stream(role: Role, stream_id: i64) -> bool {
    if !is_valid_stream_id(stream_id) {
        return false;
    }
    let kind = stream_id & 0x03;
    match role {
        Role::Client => kind == 0 || kind == 3,
        Role::Server => kind == 0 || kind == 2,
    }
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
