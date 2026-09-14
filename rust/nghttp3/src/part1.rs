/// Result type used by the safe nghttp3 wrapper.
pub type Result<T> = std::result::Result<T, Error>;

/// An nghttp3 library error code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Error(i32);

impl Error {
    /// Returns the raw `NGHTTP3_ERR_*` value.
    pub const fn code(self) -> i32 {
        self.0
    }

    /// Returns whether nghttp3 classifies this error as fatal.
    pub fn is_fatal(self) -> bool {
        // SAFETY: `self.0` is an nghttp3 error code returned by the library.
        unsafe { sys::nghttp3_err_is_fatal(self.0) != 0 }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nghttp3 error {}", self.0)
    }
}

impl std::error::Error for Error {}

fn cvt(code: i32) -> Result<()> {
    if code == 0 {
        Ok(())
    } else {
        Err(Error(code))
    }
}

fn cvt_ssize(value: sys::nghttp3_ssize) -> Result<usize> {
    if value < 0 {
        Err(Error(value as i32))
    } else {
        Ok(value as usize)
    }
}

/// Whether a connection is used by the HTTP/3 client or server endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Role {
    Client,
    Server,
}

/// QPACK indexing policy for fields without a static token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IndexingStrategy {
    None,
    Eager,
}

impl IndexingStrategy {
    fn as_raw(self) -> sys::nghttp3_qpack_indexing_strat {
        match self {
            Self::None => sys::NGHTTP3_QPACK_INDEXING_STRAT_NONE,
            Self::Eager => sys::NGHTTP3_QPACK_INDEXING_STRAT_EAGER,
        }
    }
}

/// HTTP/3 settings used when a connection is created.
#[derive(Debug)]
pub struct Settings {
    raw: sys::nghttp3_settings,
}

impl Default for Settings {
    fn default() -> Self {
        let mut raw = std::mem::MaybeUninit::<sys::nghttp3_settings>::uninit();
        // SAFETY: nghttp3 initializes the complete versioned settings object.
        unsafe {
            sys::nghttp3_settings_default_versioned(
                sys::NGHTTP3_SETTINGS_VERSION as i32,
                raw.as_mut_ptr(),
            );
            Self {
                raw: raw.assume_init(),
            }
        }
    }
}

impl Settings {
    pub fn max_field_section_size(mut self, value: u64) -> Self {
        assert!(value <= MAX_VARINT, "HTTP/3 varint setting is out of range");
        self.raw.max_field_section_size = value;
        self
    }

    pub fn qpack_max_table_capacity(mut self, value: usize) -> Self {
        assert!(value as u64 <= MAX_VARINT, "QPACK table capacity is out of range");
        self.raw.qpack_max_dtable_capacity = value;
        self
    }

    pub fn qpack_encoder_max_table_capacity(mut self, value: usize) -> Self {
        assert!(value as u64 <= MAX_VARINT, "QPACK encoder capacity is out of range");
        self.raw.qpack_encoder_max_dtable_capacity = value;
        self
    }

    pub fn qpack_blocked_streams(mut self, value: usize) -> Self {
        assert!(value as u64 <= MAX_VARINT, "QPACK blocked streams is out of range");
        self.raw.qpack_blocked_streams = value;
        self
    }

    pub fn enable_connect_protocol(mut self, enabled: bool) -> Self {
        self.raw.enable_connect_protocol = u8::from(enabled);
        self
    }

    pub fn enable_h3_datagram(mut self, enabled: bool) -> Self {
        self.raw.h3_datagram = u8::from(enabled);
        self
    }

    pub fn glitch_rate_limit(mut self, burst: u64, rate_per_second: u64) -> Self {
        self.raw.glitch_ratelim_burst = burst;
        self.raw.glitch_ratelim_rate = rate_per_second;
        self
    }

    pub fn qpack_indexing_strategy(mut self, strategy: IndexingStrategy) -> Self {
        self.raw.qpack_indexing_strat = strategy.as_raw();
        self
    }
}

/// Settings received from the peer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PeerSettings {
    pub max_field_section_size: u64,
    pub qpack_max_table_capacity: usize,
    pub qpack_blocked_streams: usize,
    pub enable_connect_protocol: bool,
    pub h3_datagram: bool,
}

impl PeerSettings {
    unsafe fn from_raw(raw: *const sys::nghttp3_proto_settings) -> Self {
        // SAFETY: callers only pass the non-null callback pointer provided by nghttp3.
        let raw = unsafe { &*raw };
        Self {
            max_field_section_size: raw.max_field_section_size,
            qpack_max_table_capacity: raw.qpack_max_dtable_capacity,
            qpack_blocked_streams: raw.qpack_blocked_streams,
            enable_connect_protocol: raw.enable_connect_protocol != 0,
            h3_datagram: raw.h3_datagram != 0,
        }
    }
}

/// Safe header flags. Pointer-lifetime flags from the C API are intentionally
/// not exposed because this wrapper always lets nghttp3 copy submitted fields.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HeaderFlags(u8);

impl HeaderFlags {
    pub const NONE: Self = Self(0);
    pub const NEVER_INDEX: Self = Self(sys::NGHTTP3_NV_FLAG_NEVER_INDEX as u8);
    pub const TRY_INDEX: Self = Self(sys::NGHTTP3_NV_FLAG_TRY_INDEX as u8);

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn bits(self) -> u8 {
        self.0
    }
}

/// An owned HTTP field suitable for safe submission to nghttp3.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header {
    name: Vec<u8>,
    value: Vec<u8>,
    flags: HeaderFlags,
}

impl Header {
    pub fn new(name: impl Into<Vec<u8>>, value: impl Into<Vec<u8>>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            flags: HeaderFlags::NONE,
        }
    }

    pub fn with_flags(mut self, flags: HeaderFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn name(&self) -> &[u8] {
        &self.name
    }

    pub fn value(&self) -> &[u8] {
        &self.value
    }

    fn as_raw(&self) -> sys::nghttp3_nv {
        sys::nghttp3_nv {
            name: self.name.as_ptr(),
            value: self.value.as_ptr(),
            namelen: self.name.len(),
            valuelen: self.value.len(),
            flags: self.flags.bits(),
        }
    }
}

/// Details about why a stream was closed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamClose {
    pub stream_id: i64,
    pub rx_app_error_code: Option<u64>,
    pub tx_app_error_code: Option<u64>,
}

/// Callback interface invoked synchronously by nghttp3.
///
/// Implementations must not unwind through the C library. The wrapper catches
/// panics and converts them into `NGHTTP3_ERR_CALLBACK_FAILURE`.
pub trait Callbacks {
    fn acked_stream_data(&mut self, _stream_id: i64, _datalen: u64) -> Result<()> {
        Ok(())
    }

    fn stream_close(&mut self, _close: StreamClose) -> Result<()> {
        Ok(())
    }

    fn recv_data(&mut self, _stream_id: i64, _data: &[u8]) -> Result<()> {
        Ok(())
    }

    fn deferred_consume(&mut self, _stream_id: i64, _consumed: usize) -> Result<()> {
        Ok(())
    }

    fn begin_headers(&mut self, _stream_id: i64) -> Result<()> {
        Ok(())
    }

    fn recv_header(
        &mut self,
        _stream_id: i64,
        _token: i32,
        _name: &[u8],
        _value: &[u8],
        _flags: u8,
    ) -> Result<()> {
        Ok(())
    }

    fn end_headers(&mut self, _stream_id: i64, _fin: bool) -> Result<()> {
        Ok(())
    }

    fn begin_trailers(&mut self, _stream_id: i64) -> Result<()> {
        Ok(())
    }

    fn recv_trailer(
        &mut self,
        _stream_id: i64,
        _token: i32,
        _name: &[u8],
        _value: &[u8],
        _flags: u8,
    ) -> Result<()> {
        Ok(())
    }

    fn end_trailers(&mut self, _stream_id: i64, _fin: bool) -> Result<()> {
        Ok(())
    }

    fn stop_sending(&mut self, _stream_id: i64, _app_error_code: u64) -> Result<()> {
        Ok(())
    }

    fn end_stream(&mut self, _stream_id: i64) -> Result<()> {
        Ok(())
    }

    fn reset_stream(&mut self, _stream_id: i64, _app_error_code: u64) -> Result<()> {
        Ok(())
    }

    fn shutdown(&mut self, _id: i64) -> Result<()> {
        Ok(())
    }

    fn recv_settings(&mut self, _settings: PeerSettings) -> Result<()> {
        Ok(())
    }

    fn recv_origin(&mut self, _origin: &[u8]) -> Result<()> {
        Ok(())
    }

    fn end_origin(&mut self) -> Result<()> {
        Ok(())
    }
}

struct BodyState {
    data: Box<[u8]>,
    offset: usize,
}

struct CallbackState<C> {
    callbacks: C,
    bodies: HashMap<i64, BodyState>,
}

/// One batch of bytes that nghttp3 wants the QUIC transport to send.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WriteData {
    pub stream_id: i64,
    pub fin: bool,
    pub segments: Vec<Vec<u8>>,
}

impl WriteData {
    pub fn len(&self) -> usize {
        self.segments.iter().map(Vec::len).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// An owned nghttp3 HTTP/3 connection.
///
/// The connection is intentionally neither `Send` nor `Sync`; nghttp3 invokes
/// callbacks synchronously and the wrapper gives them exclusive access to the
/// callback object.
pub struct Connection<C: Callbacks> {
    raw: NonNull<sys::nghttp3_conn>,
    state: Box<CallbackState<C>>,
    role: Role,
    control_bound: bool,
    qpack_bound: bool,
    last_timestamp_ns: Option<u64>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}
