/// A QPACK-encoded field section and the accompanying encoder-stream bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EncodedQpack {
    pub prefix: Vec<u8>,
    pub field_section: Vec<u8>,
    pub encoder_stream: Vec<u8>,
}

struct NativeBuffer {
    raw: sys::nghttp3_buf,
}

impl NativeBuffer {
    fn new() -> Self {
        let mut raw = sys::nghttp3_buf {
            begin: ptr::null_mut(),
            end: ptr::null_mut(),
            pos: ptr::null_mut(),
            last: ptr::null_mut(),
        };
        // SAFETY: `raw` points to writable storage for a public nghttp3 buffer.
        unsafe { sys::nghttp3_buf_init(&mut raw) };
        Self { raw }
    }

    fn to_vec(&self) -> Vec<u8> {
        if self.raw.pos.is_null() || self.raw.last.is_null() {
            return Vec::new();
        }
        // SAFETY: both pointers belong to the same nghttp3-owned allocation.
        let len = unsafe { self.raw.last.offset_from(self.raw.pos) as usize };
        // SAFETY: `pos..last` is the initialized readable range of the buffer.
        unsafe { bytes_from_raw(self.raw.pos.cast_const(), len) }.to_vec()
    }
}

impl Drop for NativeBuffer {
    fn drop(&mut self) {
        // SAFETY: these buffers are allocated by the default allocator because
        // the safe QPACK wrappers always construct their objects with `mem = NULL`.
        unsafe { sys::nghttp3_buf_free(&mut self.raw, sys::nghttp3_mem_default()) }
    }
}

/// Owned standalone QPACK encoder.
pub struct QpackEncoder {
    raw: NonNull<sys::nghttp3_qpack_encoder>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

impl QpackEncoder {
    /// Creates an encoder with an explicit unpredictable seed.
    pub fn new(hard_max_table_capacity: usize, seed: u64) -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: output pointer is valid and NULL selects nghttp3's default allocator.
        cvt(unsafe {
            sys::nghttp3_qpack_encoder_new2(
                &mut raw,
                hard_max_table_capacity,
                seed,
                ptr::null(),
            )
        })?;
        Ok(Self {
            raw: NonNull::new(raw).expect("nghttp3 returned success with a null QPACK encoder"),
            _not_send_or_sync: PhantomData,
        })
    }

    pub fn encode(&mut self, stream_id: i64, headers: &[Header]) -> Result<EncodedQpack> {
        if !is_valid_stream_id(stream_id) {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
        }
        let raw_headers = raw_headers(headers);
        let mut prefix = NativeBuffer::new();
        let mut field_section = NativeBuffer::new();
        let mut encoder_stream = NativeBuffer::new();

        // SAFETY: all buffers use the same default allocator as the encoder;
        // header pointers remain valid during the call.
        cvt(unsafe {
            sys::nghttp3_qpack_encoder_encode(
                self.raw.as_ptr(),
                &mut prefix.raw,
                &mut field_section.raw,
                &mut encoder_stream.raw,
                stream_id,
                raw_headers.as_ptr(),
                raw_headers.len(),
            )
        })?;

        Ok(EncodedQpack {
            prefix: prefix.to_vec(),
            field_section: field_section.to_vec(),
            encoder_stream: encoder_stream.to_vec(),
        })
    }

    pub fn read_decoder(&mut self, data: &[u8]) -> Result<usize> {
        // SAFETY: input is readable for the duration of the call and is not retained.
        cvt_ssize(unsafe {
            sys::nghttp3_qpack_encoder_read_decoder(
                self.raw.as_ptr(),
                data.as_ptr(),
                data.len(),
            )
        })
    }

    pub fn set_max_table_capacity(&mut self, capacity: usize) {
        // SAFETY: live owned encoder; nghttp3 truncates to its configured hard limit.
        unsafe { sys::nghttp3_qpack_encoder_set_max_dtable_capacity(self.raw.as_ptr(), capacity) }
    }

    pub fn set_max_blocked_streams(&mut self, max_streams: usize) {
        // SAFETY: live owned encoder.
        unsafe { sys::nghttp3_qpack_encoder_set_max_blocked_streams(self.raw.as_ptr(), max_streams) }
    }

    pub fn set_indexing_strategy(&mut self, strategy: IndexingStrategy) {
        // SAFETY: live owned encoder and a valid public enum value.
        unsafe {
            sys::nghttp3_qpack_encoder_set_indexing_strat(self.raw.as_ptr(), strategy.as_raw())
        }
    }

    pub fn blocked_streams(&self) -> usize {
        // SAFETY: immutable query on a live encoder.
        unsafe { sys::nghttp3_qpack_encoder_get_num_blocked_streams2(self.raw.as_ptr()) }
    }

    pub fn acknowledge_everything(&mut self) {
        // SAFETY: live owned encoder. This is an explicit debugging helper in nghttp3.
        unsafe { sys::nghttp3_qpack_encoder_ack_everything(self.raw.as_ptr()) }
    }
}

impl Drop for QpackEncoder {
    fn drop(&mut self) {
        // SAFETY: this wrapper exclusively owns the encoder and frees it once.
        unsafe { sys::nghttp3_qpack_encoder_del(self.raw.as_ptr()) }
    }
}

/// Per-stream state used by the standalone QPACK decoder.
pub struct QpackStreamContext {
    raw: NonNull<sys::nghttp3_qpack_stream_context>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

impl QpackStreamContext {
    pub fn new(stream_id: i64) -> Result<Self> {
        if !is_valid_stream_id(stream_id) {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
        }
        let mut raw = ptr::null_mut();
        // SAFETY: output pointer is valid and NULL selects the default allocator.
        cvt(unsafe {
            sys::nghttp3_qpack_stream_context_new(&mut raw, stream_id, ptr::null())
        })?;
        Ok(Self {
            raw: NonNull::new(raw).expect("nghttp3 returned success with a null QPACK context"),
            _not_send_or_sync: PhantomData,
        })
    }

    pub fn required_insert_count(&self) -> u64 {
        // SAFETY: immutable query on a live context.
        unsafe { sys::nghttp3_qpack_stream_context_get_ricnt2(self.raw.as_ptr()) }
    }

    pub fn reset(&mut self) {
        // SAFETY: live owned context.
        unsafe { sys::nghttp3_qpack_stream_context_reset(self.raw.as_ptr()) }
    }
}

impl Drop for QpackStreamContext {
    fn drop(&mut self) {
        // SAFETY: this wrapper exclusively owns the context and frees it once.
        unsafe { sys::nghttp3_qpack_stream_context_del(self.raw.as_ptr()) }
    }
}

/// Reference-counted QPACK buffer returned by nghttp3.
pub struct RcBuffer {
    raw: NonNull<sys::nghttp3_rcbuf>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

impl RcBuffer {
    unsafe fn from_owned_raw(raw: *mut sys::nghttp3_rcbuf) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self {
            raw,
            _not_send_or_sync: PhantomData,
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: `self.raw` owns a live rcbuf reference.
        let raw = unsafe { sys::nghttp3_rcbuf_get_buf(self.raw.as_ptr()) };
        // SAFETY: the returned byte range remains live while this RcBuffer does.
        unsafe { bytes_from_raw(raw.base.cast_const(), raw.len) }
    }

    pub fn is_static(&self) -> bool {
        // SAFETY: immutable query on a live rcbuf.
        unsafe { sys::nghttp3_rcbuf_is_static(self.raw.as_ptr()) != 0 }
    }
}

impl Clone for RcBuffer {
    fn clone(&self) -> Self {
        // SAFETY: incrementing a live rcbuf creates another owned reference.
        unsafe { sys::nghttp3_rcbuf_incref(self.raw.as_ptr()) };
        Self {
            raw: self.raw,
            _not_send_or_sync: PhantomData,
        }
    }
}

impl Drop for RcBuffer {
    fn drop(&mut self) {
        // SAFETY: this drops exactly one owned reference.
        unsafe { sys::nghttp3_rcbuf_decref(self.raw.as_ptr()) }
    }
}

impl fmt::Debug for RcBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RcBuffer")
            .field("bytes", &self.as_bytes())
            .field("static", &self.is_static())
            .finish()
    }
}

/// A decoded QPACK header field with owned reference-counted name and value.
#[derive(Clone, Debug)]
pub struct QpackHeader {
    pub name: RcBuffer,
    pub value: RcBuffer,
    pub token: i32,
    pub flags: u8,
}

/// Decoder flags produced while consuming a QPACK field section.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QpackDecodeFlags(u8);

impl QpackDecodeFlags {
    pub const fn bits(self) -> u8 {
        self.0
    }

    pub const fn emitted(self) -> bool {
        self.0 & sys::NGHTTP3_QPACK_DECODE_FLAG_EMIT as u8 != 0
    }

    pub const fn final_section(self) -> bool {
        self.0 & sys::NGHTTP3_QPACK_DECODE_FLAG_FINAL as u8 != 0
    }

    pub const fn blocked(self) -> bool {
        self.0 & sys::NGHTTP3_QPACK_DECODE_FLAG_BLOCKED as u8 != 0
    }
}

/// Result of one standalone QPACK request-stream decode step.
#[derive(Debug)]
pub struct QpackRead {
    pub consumed: usize,
    pub flags: QpackDecodeFlags,
    pub header: Option<QpackHeader>,
}

/// Owned standalone QPACK decoder.
pub struct QpackDecoder {
    raw: NonNull<sys::nghttp3_qpack_decoder>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

impl QpackDecoder {
    pub fn new(hard_max_table_capacity: usize, max_blocked_streams: usize) -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: output pointer is valid and NULL selects nghttp3's default allocator.
        cvt(unsafe {
            sys::nghttp3_qpack_decoder_new(
                &mut raw,
                hard_max_table_capacity,
                max_blocked_streams,
                ptr::null(),
            )
        })?;
        Ok(Self {
            raw: NonNull::new(raw).expect("nghttp3 returned success with a null QPACK decoder"),
            _not_send_or_sync: PhantomData,
        })
    }

    pub fn read_encoder(&mut self, data: &[u8]) -> Result<usize> {
        // SAFETY: input is readable for the call and is not retained.
        cvt_ssize(unsafe {
            sys::nghttp3_qpack_decoder_read_encoder(
                self.raw.as_ptr(),
                data.as_ptr(),
                data.len(),
            )
        })
    }

    pub fn insert_count(&self) -> u64 {
        // SAFETY: immutable query on a live decoder.
        unsafe { sys::nghttp3_qpack_decoder_get_icnt(self.raw.as_ptr()) }
    }

    pub fn read_request(
        &mut self,
        context: &mut QpackStreamContext,
        data: &[u8],
        fin: bool,
    ) -> Result<QpackRead> {
        let mut raw_header = std::mem::MaybeUninit::<sys::nghttp3_qpack_nv>::uninit();
        let mut flags = 0_u8;
        // SAFETY: all pointers are live for the call. nghttp3 initializes `flags`
        // and initializes `raw_header` when EMIT is set.
        let consumed = cvt_ssize(unsafe {
            sys::nghttp3_qpack_decoder_read_request(
                self.raw.as_ptr(),
                context.raw.as_ptr(),
                raw_header.as_mut_ptr(),
                &mut flags,
                data.as_ptr(),
                data.len(),
                i32::from(fin),
            )
        })?;

        let flags = QpackDecodeFlags(flags);
        let header = if flags.emitted() {
            // SAFETY: EMIT guarantees that nghttp3 initialized the output field.
            let raw = unsafe { raw_header.assume_init() };
            // SAFETY: on EMIT, each rcbuf reference is transferred to the caller.
            let name = unsafe { RcBuffer::from_owned_raw(raw.name) };
            // SAFETY: same ownership rule for the value reference.
            let value = unsafe { RcBuffer::from_owned_raw(raw.value) };
            match (name, value) {
                (Some(name), Some(value)) => Some(QpackHeader {
                    name,
                    value,
                    token: raw.token,
                    flags: raw.flags,
                }),
                _ => return Err(Error(sys::NGHTTP3_ERR_INVALID_STATE)),
            }
        } else {
            None
        };

        Ok(QpackRead {
            consumed,
            flags,
            header,
        })
    }

    pub fn decoder_stream(&mut self) -> Vec<u8> {
        // SAFETY: immutable length query on a live decoder.
        let len = unsafe { sys::nghttp3_qpack_decoder_get_decoder_streamlen2(self.raw.as_ptr()) };
        if len == 0 {
            return Vec::new();
        }
        let mut data = vec![0_u8; len];
        let begin = data.as_mut_ptr();
        // SAFETY: begin points to `len` writable bytes.
        let end = unsafe { begin.add(len) };
        let mut raw = sys::nghttp3_buf {
            begin,
            end,
            pos: begin,
            last: begin,
        };
        // SAFETY: the buffer has exactly the capacity reported by nghttp3.
        unsafe { sys::nghttp3_qpack_decoder_write_decoder(self.raw.as_ptr(), &mut raw) };
        // SAFETY: write_decoder keeps `last` within begin..=end by contract.
        let written = unsafe { raw.last.offset_from(begin) as usize };
        data.truncate(written);
        data
    }

    pub fn cancel_stream(&mut self, stream_id: i64) -> Result<()> {
        if !is_valid_stream_id(stream_id) {
            return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
        }
        // SAFETY: live decoder and valid HTTP/3 stream ID.
        cvt(unsafe { sys::nghttp3_qpack_decoder_cancel_stream(self.raw.as_ptr(), stream_id) })
    }

    pub fn set_max_table_capacity(&mut self, capacity: usize) -> Result<()> {
        // SAFETY: live decoder; nghttp3 validates against the configured hard limit.
        cvt(unsafe {
            sys::nghttp3_qpack_decoder_set_max_dtable_capacity(self.raw.as_ptr(), capacity)
        })
    }

    pub fn set_max_concurrent_streams(&mut self, max_streams: usize) {
        // SAFETY: live decoder.
        unsafe {
            sys::nghttp3_qpack_decoder_set_max_concurrent_streams(
                self.raw.as_ptr(),
                max_streams,
            )
        }
    }
}

impl Drop for QpackDecoder {
    fn drop(&mut self) {
        // SAFETY: this wrapper exclusively owns the decoder and frees it once.
        unsafe { sys::nghttp3_qpack_decoder_del(self.raw.as_ptr()) }
    }
}

impl Error {
    /// Returns nghttp3's textual description for this error.
    pub fn message(self) -> &'static str {
        // SAFETY: nghttp3 returns a static NUL-terminated error string.
        let raw = unsafe { sys::nghttp3_strerror(self.0) };
        if raw.is_null() {
            return "unknown nghttp3 error";
        }
        // SAFETY: non-null static C string returned by nghttp3.
        unsafe { CStr::from_ptr(raw) }
            .to_str()
            .unwrap_or("unknown nghttp3 error")
    }

    /// Returns the HTTP/3 application error code inferred by nghttp3.
    pub fn quic_app_error_code(self) -> u64 {
        // SAFETY: `self.0` is a library error code produced by this wrapper/nghttp3.
        unsafe { sys::nghttp3_err_infer_quic_app_error_code(self.0) }
    }
}

/// Decodes one QUIC variable-length unsigned integer when enough input is present.
pub fn decode_uvarint(input: &[u8]) -> Option<(u64, usize)> {
    let first = input.first()?;
    // SAFETY: `first` points to one readable byte, which is all this function reads.
    let required = unsafe { sys::nghttp3_get_uvarintlen(first as *const u8) };
    if input.len() < required {
        return None;
    }
    let mut value = 0_u64;
    // SAFETY: the length check above guarantees a complete encoded integer.
    unsafe { sys::nghttp3_get_uvarint(&mut value, input.as_ptr()) };
    Some((value, required))
}

/// Encodes a QUIC variable-length unsigned integer.
pub fn encode_uvarint(value: u64) -> Result<Vec<u8>> {
    if value > MAX_VARINT {
        return Err(Error(sys::NGHTTP3_ERR_INVALID_ARGUMENT));
    }
    // SAFETY: value is within the documented QUIC varint range.
    let len = unsafe { sys::nghttp3_put_uvarintlen(value) };
    let mut output = vec![0_u8; len];
    // SAFETY: output has the exact capacity requested by nghttp3.
    unsafe { sys::nghttp3_put_uvarint(output.as_mut_ptr(), value) };
    Ok(output)
}
