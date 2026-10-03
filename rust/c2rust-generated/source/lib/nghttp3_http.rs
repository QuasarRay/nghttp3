extern "C" {
    pub type nghttp3_conn;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static nghttp3_downcase_tbl: [uint8_t; 0];
    fn sfparse_parser_init(
        sfp: *mut sfparse_parser,
        data: *const uint8_t,
        datalen: size_t,
    );
    fn sfparse_parser_dict(
        sfp: *mut sfparse_parser,
        dest_key: *mut sfparse_vec,
        dest_value: *mut sfparse_value,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type int8_t = i8;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type ptrdiff_t = isize;
pub type nghttp3_ssize = ptrdiff_t;
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
pub struct nghttp3_rcbuf {
    pub mem: *const nghttp3_mem,
    pub base: *mut uint8_t,
    pub len: size_t,
    pub r#ref: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_buf {
    pub begin: *mut uint8_t,
    pub end: *mut uint8_t,
    pub pos: *mut uint8_t,
    pub last: *mut uint8_t,
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
pub struct nghttp3_qpack_token(pub ::core::ffi::c_uint);
impl nghttp3_qpack_token {
    pub const NGHTTP3_QPACK_TOKEN__AUTHORITY: Self = Self(0);
    pub const NGHTTP3_QPACK_TOKEN__PATH: Self = Self(8);
    pub const NGHTTP3_QPACK_TOKEN_AGE: Self = Self(43);
    pub const NGHTTP3_QPACK_TOKEN_CONTENT_DISPOSITION: Self = Self(52);
    pub const NGHTTP3_QPACK_TOKEN_CONTENT_LENGTH: Self = Self(55);
    pub const NGHTTP3_QPACK_TOKEN_COOKIE: Self = Self(68);
    pub const NGHTTP3_QPACK_TOKEN_DATE: Self = Self(69);
    pub const NGHTTP3_QPACK_TOKEN_ETAG: Self = Self(71);
    pub const NGHTTP3_QPACK_TOKEN_IF_MODIFIED_SINCE: Self = Self(74);
    pub const NGHTTP3_QPACK_TOKEN_IF_NONE_MATCH: Self = Self(75);
    pub const NGHTTP3_QPACK_TOKEN_LAST_MODIFIED: Self = Self(77);
    pub const NGHTTP3_QPACK_TOKEN_LINK: Self = Self(78);
    pub const NGHTTP3_QPACK_TOKEN_LOCATION: Self = Self(79);
    pub const NGHTTP3_QPACK_TOKEN_REFERER: Self = Self(83);
    pub const NGHTTP3_QPACK_TOKEN_SET_COOKIE: Self = Self(85);
    pub const NGHTTP3_QPACK_TOKEN__METHOD: Self = Self(1);
    pub const NGHTTP3_QPACK_TOKEN__SCHEME: Self = Self(9);
    pub const NGHTTP3_QPACK_TOKEN__STATUS: Self = Self(11);
    pub const NGHTTP3_QPACK_TOKEN_ACCEPT: Self = Self(25);
    pub const NGHTTP3_QPACK_TOKEN_ACCEPT_ENCODING: Self = Self(27);
    pub const NGHTTP3_QPACK_TOKEN_ACCEPT_RANGES: Self = Self(29);
    pub const NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_HEADERS: Self = Self(32);
    pub const NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_ORIGIN: Self = Self(38);
    pub const NGHTTP3_QPACK_TOKEN_CACHE_CONTROL: Self = Self(46);
    pub const NGHTTP3_QPACK_TOKEN_CONTENT_ENCODING: Self = Self(53);
    pub const NGHTTP3_QPACK_TOKEN_CONTENT_TYPE: Self = Self(57);
    pub const NGHTTP3_QPACK_TOKEN_RANGE: Self = Self(82);
    pub const NGHTTP3_QPACK_TOKEN_STRICT_TRANSPORT_SECURITY: Self = Self(86);
    pub const NGHTTP3_QPACK_TOKEN_VARY: Self = Self(92);
    pub const NGHTTP3_QPACK_TOKEN_X_CONTENT_TYPE_OPTIONS: Self = Self(94);
    pub const NGHTTP3_QPACK_TOKEN_X_XSS_PROTECTION: Self = Self(98);
    pub const NGHTTP3_QPACK_TOKEN_ACCEPT_LANGUAGE: Self = Self(28);
    pub const NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_CREDENTIALS: Self = Self(30);
    pub const NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_METHODS: Self = Self(35);
    pub const NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_EXPOSE_HEADERS: Self = Self(39);
    pub const NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_REQUEST_HEADERS: Self = Self(40);
    pub const NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_REQUEST_METHOD: Self = Self(41);
    pub const NGHTTP3_QPACK_TOKEN_ALT_SVC: Self = Self(44);
    pub const NGHTTP3_QPACK_TOKEN_AUTHORIZATION: Self = Self(45);
    pub const NGHTTP3_QPACK_TOKEN_CONTENT_SECURITY_POLICY: Self = Self(56);
    pub const NGHTTP3_QPACK_TOKEN_EARLY_DATA: Self = Self(70);
    pub const NGHTTP3_QPACK_TOKEN_EXPECT_CT: Self = Self(72);
    pub const NGHTTP3_QPACK_TOKEN_FORWARDED: Self = Self(73);
    pub const NGHTTP3_QPACK_TOKEN_IF_RANGE: Self = Self(76);
    pub const NGHTTP3_QPACK_TOKEN_ORIGIN: Self = Self(80);
    pub const NGHTTP3_QPACK_TOKEN_PURPOSE: Self = Self(81);
    pub const NGHTTP3_QPACK_TOKEN_SERVER: Self = Self(84);
    pub const NGHTTP3_QPACK_TOKEN_TIMING_ALLOW_ORIGIN: Self = Self(89);
    pub const NGHTTP3_QPACK_TOKEN_UPGRADE_INSECURE_REQUESTS: Self = Self(90);
    pub const NGHTTP3_QPACK_TOKEN_USER_AGENT: Self = Self(91);
    pub const NGHTTP3_QPACK_TOKEN_X_FORWARDED_FOR: Self = Self(95);
    pub const NGHTTP3_QPACK_TOKEN_X_FRAME_OPTIONS: Self = Self(96);
    pub const NGHTTP3_QPACK_TOKEN_HOST: Self = Self(1000);
    pub const NGHTTP3_QPACK_TOKEN_CONNECTION: Self = Self(1001);
    pub const NGHTTP3_QPACK_TOKEN_KEEP_ALIVE: Self = Self(1002);
    pub const NGHTTP3_QPACK_TOKEN_PROXY_CONNECTION: Self = Self(1003);
    pub const NGHTTP3_QPACK_TOKEN_TRANSFER_ENCODING: Self = Self(1004);
    pub const NGHTTP3_QPACK_TOKEN_UPGRADE: Self = Self(1005);
    pub const NGHTTP3_QPACK_TOKEN_TE: Self = Self(1006);
    pub const NGHTTP3_QPACK_TOKEN__PROTOCOL: Self = Self(1007);
    pub const NGHTTP3_QPACK_TOKEN_PRIORITY: Self = Self(1008);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_qpack_nv {
    pub name: *mut nghttp3_rcbuf,
    pub value: *mut nghttp3_rcbuf,
    pub token: int32_t,
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
pub struct nghttp3_qpack_read_state {
    pub huffman_ctx: nghttp3_qpack_huffman_decode_context,
    pub namebuf: nghttp3_buf,
    pub valuebuf: nghttp3_buf,
    pub name: *mut nghttp3_rcbuf,
    pub value: *mut nghttp3_rcbuf,
    pub left: uint64_t,
    pub prefix: size_t,
    pub shift: size_t,
    pub absidx: uint64_t,
    pub never: ::core::ffi::c_int,
    pub dynamic: ::core::ffi::c_int,
    pub huffman_encoded: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_qpack_huffman_decode_context {
    pub fstate: uint16_t,
    pub flags: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_pq_entry {
    pub index: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_opl_entry {
    pub next: *mut nghttp3_opl_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_objalloc {
    pub balloc: nghttp3_balloc,
    pub opl: nghttp3_opl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_opl {
    pub head: *mut nghttp3_opl_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_balloc {
    pub mem: *const nghttp3_mem,
    pub blklen: size_t,
    pub head: *mut nghttp3_memblock_hd,
    pub buf: nghttp3_buf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_memblock_hd {
    pub c2rust_unnamed: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub next: *mut nghttp3_memblock_hd,
    pub pad: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_ringbuf {
    pub buf: *mut uint8_t,
    pub mem: *const nghttp3_mem,
    pub nmemb: size_t,
    pub size: size_t,
    pub first: size_t,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_qpack_stream_context {
    pub rstate: nghttp3_qpack_read_state,
    pub mem: *const nghttp3_mem,
    pub stream_id: int64_t,
    pub ricnt: uint64_t,
    pub base: uint64_t,
    pub state: nghttp3_qpack_request_stream_state,
    pub opcode: nghttp3_qpack_request_stream_opcode,
    pub dbase_sign: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nghttp3_qpack_request_stream_opcode(pub ::core::ffi::c_uint);
impl nghttp3_qpack_request_stream_opcode {
    pub const NGHTTP3_QPACK_RS_OPCODE_INDEXED: Self = Self(0);
    pub const NGHTTP3_QPACK_RS_OPCODE_INDEXED_PB: Self = Self(1);
    pub const NGHTTP3_QPACK_RS_OPCODE_INDEXED_NAME: Self = Self(2);
    pub const NGHTTP3_QPACK_RS_OPCODE_INDEXED_NAME_PB: Self = Self(3);
    pub const NGHTTP3_QPACK_RS_OPCODE_LITERAL: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nghttp3_qpack_request_stream_state(pub ::core::ffi::c_uint);
impl nghttp3_qpack_request_stream_state {
    pub const NGHTTP3_QPACK_RS_STATE_RICNT: Self = Self(0);
    pub const NGHTTP3_QPACK_RS_STATE_DBASE_SIGN: Self = Self(1);
    pub const NGHTTP3_QPACK_RS_STATE_DBASE: Self = Self(2);
    pub const NGHTTP3_QPACK_RS_STATE_OPCODE: Self = Self(3);
    pub const NGHTTP3_QPACK_RS_STATE_READ_INDEX: Self = Self(4);
    pub const NGHTTP3_QPACK_RS_STATE_CHECK_NAME_HUFFMAN: Self = Self(5);
    pub const NGHTTP3_QPACK_RS_STATE_READ_NAMELEN: Self = Self(6);
    pub const NGHTTP3_QPACK_RS_STATE_READ_NAME_HUFFMAN: Self = Self(7);
    pub const NGHTTP3_QPACK_RS_STATE_READ_NAME: Self = Self(8);
    pub const NGHTTP3_QPACK_RS_STATE_CHECK_VALUE_HUFFMAN: Self = Self(9);
    pub const NGHTTP3_QPACK_RS_STATE_READ_VALUELEN: Self = Self(10);
    pub const NGHTTP3_QPACK_RS_STATE_READ_VALUE_HUFFMAN: Self = Self(11);
    pub const NGHTTP3_QPACK_RS_STATE_READ_VALUE: Self = Self(12);
    pub const NGHTTP3_QPACK_RS_STATE_BLOCKED: Self = Self(13);
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
pub type nghttp3_read_data_callback = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        *mut nghttp3_vec,
        size_t,
        *mut uint32_t,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> nghttp3_ssize,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_data_reader {
    pub read_data: nghttp3_read_data_callback,
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
pub union C2Rust_Unnamed_0 {
    pub boolean: ::core::ffi::c_int,
    pub integer: int64_t,
    pub decimal: sfparse_decimal,
    pub vec: sfparse_vec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfparse_vec {
    pub base: *mut uint8_t,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfparse_decimal {
    pub numer: int64_t,
    pub denom: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfparse_value {
    pub r#type: sfparse_type,
    pub flags: uint32_t,
    pub c2rust_unnamed: C2Rust_Unnamed_0,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sfparse_type(pub ::core::ffi::c_uint);
impl sfparse_type {
    pub const SFPARSE_TYPE_BOOLEAN: Self = Self(0);
    pub const SFPARSE_TYPE_INTEGER: Self = Self(1);
    pub const SFPARSE_TYPE_DECIMAL: Self = Self(2);
    pub const SFPARSE_TYPE_STRING: Self = Self(3);
    pub const SFPARSE_TYPE_TOKEN: Self = Self(4);
    pub const SFPARSE_TYPE_BYTESEQ: Self = Self(5);
    pub const SFPARSE_TYPE_INNER_LIST: Self = Self(6);
    pub const SFPARSE_TYPE_DATE: Self = Self(7);
    pub const SFPARSE_TYPE_DISPSTRING: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfparse_parser {
    pub pos: *const uint8_t,
    pub end: *const uint8_t,
    pub state: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_stream {
    pub c2rust_unnamed: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub c2rust_unnamed: C2Rust_Unnamed_2,
    pub oplent: nghttp3_opl_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub mem: *const nghttp3_mem,
    pub out_chunk_objalloc: *mut nghttp3_objalloc,
    pub stream_objalloc: *mut nghttp3_objalloc,
    pub node: nghttp3_tnode,
    pub qpack_blocked_pe: nghttp3_pq_entry,
    pub callbacks: nghttp3_stream_callbacks,
    pub frq: nghttp3_ringbuf,
    pub chunks: nghttp3_ringbuf,
    pub outq: nghttp3_ringbuf,
    pub inq: nghttp3_ringbuf,
    pub qpack_sctx: nghttp3_qpack_stream_context,
    pub conn: *mut nghttp3_conn,
    pub user_data: *mut ::core::ffi::c_void,
    pub unsent_bytes: uint64_t,
    pub outq_idx: size_t,
    pub ack_base: uint64_t,
    pub ack_offset: uint64_t,
    pub unscheduled_nwrite: uint64_t,
    pub r#type: nghttp3_stream_type,
    pub rstate: nghttp3_stream_read_state,
    pub tx: C2Rust_Unnamed_4,
    pub rx: C2Rust_Unnamed_3,
    pub flags: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub hstate: nghttp3_stream_http_state,
    pub http: nghttp3_http_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_http_state {
    pub content_length: int64_t,
    pub recv_content_length: int64_t,
    pub pri: nghttp3_pri,
    pub status_code: int32_t,
    pub flags: uint32_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nghttp3_stream_http_state(pub ::core::ffi::c_uint);
impl nghttp3_stream_http_state {
    pub const NGHTTP3_HTTP_STATE_NONE: Self = Self(0);
    pub const NGHTTP3_HTTP_STATE_REQ_INITIAL: Self = Self(1);
    pub const NGHTTP3_HTTP_STATE_REQ_HEADERS_BEGIN: Self = Self(2);
    pub const NGHTTP3_HTTP_STATE_REQ_HEADERS_END: Self = Self(3);
    pub const NGHTTP3_HTTP_STATE_REQ_DATA_BEGIN: Self = Self(4);
    pub const NGHTTP3_HTTP_STATE_REQ_DATA_END: Self = Self(5);
    pub const NGHTTP3_HTTP_STATE_REQ_TRAILERS_BEGIN: Self = Self(6);
    pub const NGHTTP3_HTTP_STATE_REQ_TRAILERS_END: Self = Self(7);
    pub const NGHTTP3_HTTP_STATE_REQ_END: Self = Self(8);
    pub const NGHTTP3_HTTP_STATE_RESP_INITIAL: Self = Self(9);
    pub const NGHTTP3_HTTP_STATE_RESP_HEADERS_BEGIN: Self = Self(10);
    pub const NGHTTP3_HTTP_STATE_RESP_HEADERS_END: Self = Self(11);
    pub const NGHTTP3_HTTP_STATE_RESP_DATA_BEGIN: Self = Self(12);
    pub const NGHTTP3_HTTP_STATE_RESP_DATA_END: Self = Self(13);
    pub const NGHTTP3_HTTP_STATE_RESP_TRAILERS_BEGIN: Self = Self(14);
    pub const NGHTTP3_HTTP_STATE_RESP_TRAILERS_END: Self = Self(15);
    pub const NGHTTP3_HTTP_STATE_RESP_END: Self = Self(16);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub offset: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_stream_read_state {
    pub rvint: nghttp3_varint_read_state,
    pub iv: nghttp3_settings_entry,
    pub fr: nghttp3_frame,
    pub left: uint64_t,
    pub state: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union nghttp3_frame {
    pub hd: nghttp3_frame_hd,
    pub data: nghttp3_frame_data,
    pub headers: nghttp3_frame_headers,
    pub settings: nghttp3_frame_settings,
    pub goaway: nghttp3_frame_goaway,
    pub priority_update: nghttp3_frame_priority_update,
    pub origin: nghttp3_frame_origin,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_frame_origin {
    pub r#type: uint64_t,
    pub origin_list: nghttp3_vec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_frame_priority_update {
    pub r#type: uint64_t,
    pub pri_elem_id: int64_t,
    pub c2rust_unnamed: C2Rust_Unnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub c2rust_unnamed: C2Rust_Unnamed_6,
    pub pri: nghttp3_pri,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub data: *mut uint8_t,
    pub datalen: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_frame_goaway {
    pub r#type: uint64_t,
    pub id: int64_t,
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
pub struct nghttp3_settings_entry {
    pub id: uint64_t,
    pub value: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_frame_headers {
    pub r#type: uint64_t,
    pub nva: *mut nghttp3_nv,
    pub nvlen: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_frame_data {
    pub r#type: uint64_t,
    pub dr: nghttp3_data_reader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_frame_hd {
    pub r#type: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_varint_read_state {
    pub acc: uint64_t,
    pub left: size_t,
}
pub type nghttp3_stream_type = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_stream_callbacks {
    pub acked_data: nghttp3_stream_acked_data,
}
pub type nghttp3_stream_acked_data = Option<
    unsafe extern "C" fn(
        *mut nghttp3_stream,
        int64_t,
        uint64_t,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_tnode {
    pub pe: nghttp3_pq_entry,
    pub id: int64_t,
    pub cycle: uint64_t,
    pub pri: nghttp3_pri,
}
pub const NGHTTP3_ERR_INVALID_ARGUMENT: ::core::ffi::c_int = -101 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_MALFORMED_HTTP_HEADER: ::core::ffi::c_int = -105
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_REMOVE_HTTP_HEADER: ::core::ffi::c_int = -106
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_MALFORMED_HTTP_MESSAGING: ::core::ffi::c_int = -107
    as ::core::ffi::c_int;
pub const NGHTTP3_URGENCY_HIGH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NGHTTP3_URGENCY_LOW: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const NGHTTP3_HTTP_FLAG__AUTHORITY: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG__PATH: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG__METHOD: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG__SCHEME: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG_HOST: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG__STATUS: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG_REQ_HEADERS: ::core::ffi::c_uint = NGHTTP3_HTTP_FLAG__METHOD
    | NGHTTP3_HTTP_FLAG__PATH | NGHTTP3_HTTP_FLAG__SCHEME;
pub const NGHTTP3_HTTP_FLAG_PSEUDO_HEADER_DISALLOWED: ::core::ffi::c_uint = 0x40
    as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG_METH_CONNECT: ::core::ffi::c_uint = 0x80
    as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG_METH_HEAD: ::core::ffi::c_uint = 0x100
    as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG_METH_OPTIONS: ::core::ffi::c_uint = 0x200
    as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG_METH_ALL: ::core::ffi::c_uint = NGHTTP3_HTTP_FLAG_METH_CONNECT
    | NGHTTP3_HTTP_FLAG_METH_HEAD | NGHTTP3_HTTP_FLAG_METH_OPTIONS;
pub const NGHTTP3_HTTP_FLAG_PATH_REGULAR: ::core::ffi::c_uint = 0x400
    as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG_PATH_ASTERISK: ::core::ffi::c_uint = 0x800
    as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG_SCHEME_HTTP: ::core::ffi::c_uint = 0x1000
    as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG_EXPECT_FINAL_RESPONSE: ::core::ffi::c_uint = 0x2000
    as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG__PROTOCOL: ::core::ffi::c_uint = 0x4000
    as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG_PRIORITY: ::core::ffi::c_uint = 0x8000
    as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG_BAD_PRIORITY: ::core::ffi::c_uint = 0x10000
    as ::core::ffi::c_uint;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 90] = unsafe {
    ::core::mem::transmute::<
        [u8; 90],
        [::core::ffi::c_char; 90],
    >(
        *b"int nghttp3_http_on_header(nghttp3_http_state *, const nghttp3_qpack_nv *, int, int, int)\0",
    )
};
pub const NGHTTP3_MAX_VARINT: ::core::ffi::c_ulonglong = ((1 as ::core::ffi::c_ulonglong)
    << 62 as ::core::ffi::c_int)
    .wrapping_sub(1 as ::core::ffi::c_ulonglong);
#[inline]
unsafe extern "C" fn nghttp3_downcase_byte(mut c: uint8_t) -> uint8_t {
    return *(&raw const nghttp3_downcase_tbl as *const uint8_t).offset(c as isize);
}
pub const SFPARSE_ERR_EOF: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
unsafe extern "C" fn memieq(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
    mut n: size_t,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    let mut aa: *const uint8_t = a as *const uint8_t;
    let mut bb: *const uint8_t = b as *const uint8_t;
    i = 0 as size_t;
    while i < n {
        if *aa.offset(i as isize) as ::core::ffi::c_int
            != nghttp3_downcase_byte(*bb.offset(i as isize)) as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_status_code(
    mut s: *const uint8_t,
    mut len: size_t,
) -> int32_t {
    if len != 3 as size_t
        || '1' as ::core::ffi::c_int > *s.offset(0isize) as ::core::ffi::c_int
        || *s.offset(0isize) as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        || '0' as ::core::ffi::c_int > *s.offset(1isize) as ::core::ffi::c_int
        || *s.offset(1isize) as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        || '0' as ::core::ffi::c_int > *s.offset(2isize) as ::core::ffi::c_int
        || *s.offset(2isize) as ::core::ffi::c_int > '9' as ::core::ffi::c_int
    {
        return -1 as int32_t;
    }
    return (*s.offset(0isize) as int32_t - '0' as int32_t) * 100 as int32_t
        + (*s.offset(1isize) as int32_t - '0' as int32_t) * 10 as int32_t
        + (*s.offset(2isize) as int32_t - '0' as int32_t);
}
unsafe extern "C" fn parse_uint(mut s: *const uint8_t, mut len: size_t) -> int64_t {
    let mut n: uint64_t = 0 as uint64_t;
    let mut c: uint32_t = 0;
    let mut i: size_t = 0;
    if len == 0 as size_t {
        return -1 as int64_t;
    }
    i = 0 as size_t;
    while i < len {
        if '0' as ::core::ffi::c_int > *s.offset(i as isize) as ::core::ffi::c_int
            || *s.offset(i as isize) as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        {
            return -1 as int64_t;
        }
        c = (*s.offset(i as isize) as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
            as uint32_t;
        if n as ::core::ffi::c_ulonglong
            > NGHTTP3_MAX_VARINT
                .wrapping_sub(c as ::core::ffi::c_ulonglong)
                .wrapping_div(10 as ::core::ffi::c_ulonglong)
        {
            return -1 as int64_t;
        }
        n = n.wrapping_mul(10 as uint64_t).wrapping_add(c as uint64_t);
        i = i.wrapping_add(1);
    }
    return n as int64_t;
}
unsafe extern "C" fn check_pseudo_header(
    mut http: *mut nghttp3_http_state,
    mut nv: *const nghttp3_qpack_nv,
    mut flag: uint32_t,
) -> ::core::ffi::c_int {
    if (*http).flags & flag != 0 || (*(*nv).value).len == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    (*http).flags |= flag;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn expect_response_body(
    mut http: *const nghttp3_http_state,
) -> ::core::ffi::c_int {
    return ((*http).flags & NGHTTP3_HTTP_FLAG_METH_HEAD as uint32_t == 0 as uint32_t
        && (*http).status_code / 100 as int32_t != 1 as int32_t
        && (*http).status_code != 304 as int32_t
        && (*http).status_code != 204 as int32_t) as ::core::ffi::c_int;
}
unsafe extern "C" fn check_path_flags(
    mut http: *const nghttp3_http_state,
) -> ::core::ffi::c_int {
    return ((*http).flags & NGHTTP3_HTTP_FLAG_SCHEME_HTTP as uint32_t == 0 as uint32_t
        || ((*http).flags & NGHTTP3_HTTP_FLAG_PATH_REGULAR as uint32_t != 0
            || (*http).flags & NGHTTP3_HTTP_FLAG_METH_OPTIONS as uint32_t != 0
                && (*http).flags & NGHTTP3_HTTP_FLAG_PATH_ASTERISK as uint32_t != 0))
        as ::core::ffi::c_int;
}
unsafe extern "C" fn is_ws(mut c: uint8_t) -> ::core::ffi::c_int {
    match c as ::core::ffi::c_int {
        32 | 9 => return 1 as ::core::ffi::c_int,
        _ => return 0 as ::core::ffi::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_http_parse_priority(
    mut dest: *mut nghttp3_pri,
    mut value: *const uint8_t,
    mut valuelen: size_t,
) -> ::core::ffi::c_int {
    let mut pri: nghttp3_pri = *dest;
    let mut sfp: sfparse_parser = sfparse_parser {
        pos: ::core::ptr::null::<uint8_t>(),
        end: ::core::ptr::null::<uint8_t>(),
        state: 0,
    };
    let mut key: sfparse_vec = sfparse_vec {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    let mut val: sfparse_value = sfparse_value {
        r#type: sfparse_type::SFPARSE_TYPE_BOOLEAN,
        flags: 0,
        c2rust_unnamed: C2Rust_Unnamed_0 { boolean: 0 },
    };
    let mut rv: ::core::ffi::c_int = 0;
    sfparse_parser_init(&raw mut sfp, value, valuelen);
    loop {
        rv = sfparse_parser_dict(&raw mut sfp, &raw mut key, &raw mut val);
        if rv != 0 as ::core::ffi::c_int {
            if rv == SFPARSE_ERR_EOF {
                break;
            }
            return NGHTTP3_ERR_INVALID_ARGUMENT;
        } else {
            if key.len != 1 as size_t {
                continue;
            }
            match *key.base.offset(0isize) as ::core::ffi::c_int {
                105 => {
                    if val.r#type.0 != sfparse_type::SFPARSE_TYPE_BOOLEAN.0 {
                        return NGHTTP3_ERR_INVALID_ARGUMENT;
                    }
                    pri.0.inc = val.c2rust_unnamed.boolean as uint8_t;
                }
                117 => {
                    if val.r#type.0 != sfparse_type::SFPARSE_TYPE_INTEGER.0
                        || val.c2rust_unnamed.integer < NGHTTP3_URGENCY_HIGH as int64_t
                        || (NGHTTP3_URGENCY_LOW as int64_t) < val.c2rust_unnamed.integer
                    {
                        return NGHTTP3_ERR_INVALID_ARGUMENT;
                    }
                    pri.0.urgency = val.c2rust_unnamed.integer as uint32_t;
                }
                _ => {}
            }
        }
    }
    *dest = pri;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_pri_parse_priority_versioned(
    mut pri_version: ::core::ffi::c_int,
    mut dest: *mut nghttp3_pri,
    mut value: *const uint8_t,
    mut valuelen: size_t,
) -> ::core::ffi::c_int {
    return nghttp3_http_parse_priority(dest, value, valuelen);
}
static mut VALID_AUTHORITY_CHARS: [int8_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as int8_t,
    0,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    0,
    1 as int8_t,
    0,
    0,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    0,
    1 as int8_t,
    0,
    1 as int8_t,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    0,
    0,
    0,
    1 as int8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn check_authority(
    mut value: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut last: *const uint8_t = ::core::ptr::null::<uint8_t>();
    last = value.offset(len as isize);
    while value != last {
        if VALID_AUTHORITY_CHARS[*value as usize] == 0 {
            return 0 as ::core::ffi::c_int;
        }
        value = value.offset(1);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn check_scheme(
    mut value: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut last: *const uint8_t = ::core::ptr::null::<uint8_t>();
    if len == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    if !('A' as ::core::ffi::c_int <= *value as ::core::ffi::c_int
        && *value as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
        || 'a' as ::core::ffi::c_int <= *value as ::core::ffi::c_int
            && *value as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int)
    {
        return 0 as ::core::ffi::c_int;
    }
    last = value.offset(len as isize);
    value = value.offset(1);
    while value != last {
        if !('A' as ::core::ffi::c_int <= *value as ::core::ffi::c_int
            && *value as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
            || 'a' as ::core::ffi::c_int <= *value as ::core::ffi::c_int
                && *value as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
            || '0' as ::core::ffi::c_int <= *value as ::core::ffi::c_int
                && *value as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            || *value as ::core::ffi::c_int == '+' as ::core::ffi::c_int
            || *value as ::core::ffi::c_int == '-' as ::core::ffi::c_int
            || *value as ::core::ffi::c_int == '.' as ::core::ffi::c_int)
        {
            return 0 as ::core::ffi::c_int;
        }
        value = value.offset(1);
    }
    return 1 as ::core::ffi::c_int;
}
static mut VALID_METHOD_CHARS: [int8_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as int8_t,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    0,
    0,
    1 as int8_t,
    1 as int8_t,
    0,
    1 as int8_t,
    1 as int8_t,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    0,
    0,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    0,
    1 as int8_t,
    0,
    1 as int8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn check_method(
    mut value: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut last: *const uint8_t = ::core::ptr::null::<uint8_t>();
    if len == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    last = value.offset(len as isize);
    while value != last {
        if VALID_METHOD_CHARS[*value as usize] == 0 {
            return 0 as ::core::ffi::c_int;
        }
        value = value.offset(1);
    }
    return 1 as ::core::ffi::c_int;
}
static mut VALID_PATH_CHARS: [int8_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
];
unsafe extern "C" fn check_path(
    mut value: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut last: *const uint8_t = ::core::ptr::null::<uint8_t>();
    last = value.offset(len as isize);
    while value != last {
        if VALID_PATH_CHARS[*value as usize] == 0 {
            return 0 as ::core::ffi::c_int;
        }
        value = value.offset(1);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn http_request_on_header(
    mut http: *mut nghttp3_http_state,
    mut nv: *const nghttp3_qpack_nv,
    mut trailers: ::core::ffi::c_int,
    mut connect_protocol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pri: nghttp3_pri = nghttp3_pri { urgency: 0, inc: 0 };
    match (*nv).token {
        0 => {
            if check_authority((*(*nv).value).base, (*(*nv).value).len) == 0
                || check_pseudo_header(
                    http,
                    nv,
                    NGHTTP3_HTTP_FLAG__AUTHORITY as uint32_t,
                ) == 0
            {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
        }
        1 => {
            if check_method((*(*nv).value).base, (*(*nv).value).len) == 0
                || check_pseudo_header(http, nv, NGHTTP3_HTTP_FLAG__METHOD as uint32_t)
                    == 0
            {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
            match (*(*nv).value).len {
                4 => {
                    if ::core::mem::size_of::<[::core::ffi::c_char; 5]>()
                        .wrapping_sub(1usize) == (*(*nv).value).len
                        && memcmp(
                            b"HEAD\0".as_ptr() as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            (*(*nv).value).base as *const ::core::ffi::c_void,
                            (*(*nv).value).len,
                        ) == 0 as ::core::ffi::c_int
                    {
                        (*http).flags = ((*http).flags as ::core::ffi::c_uint
                            | NGHTTP3_HTTP_FLAG_METH_HEAD) as uint32_t;
                    }
                }
                7 => {
                    match *(*(*nv).value).base.offset(6isize) as ::core::ffi::c_int {
                        84 => {
                            if ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                                .wrapping_sub(1usize) == (*(*nv).value).len
                                && memcmp(
                                    b"CONNECT\0".as_ptr() as *const ::core::ffi::c_char
                                        as *const ::core::ffi::c_void,
                                    (*(*nv).value).base as *const ::core::ffi::c_void,
                                    (*(*nv).value).len,
                                ) == 0 as ::core::ffi::c_int
                            {
                                (*http).flags = ((*http).flags as ::core::ffi::c_uint
                                    | NGHTTP3_HTTP_FLAG_METH_CONNECT) as uint32_t;
                            }
                        }
                        83 => {
                            if ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                                .wrapping_sub(1usize) == (*(*nv).value).len
                                && memcmp(
                                    b"OPTIONS\0".as_ptr() as *const ::core::ffi::c_char
                                        as *const ::core::ffi::c_void,
                                    (*(*nv).value).base as *const ::core::ffi::c_void,
                                    (*(*nv).value).len,
                                ) == 0 as ::core::ffi::c_int
                            {
                                (*http).flags = ((*http).flags as ::core::ffi::c_uint
                                    | NGHTTP3_HTTP_FLAG_METH_OPTIONS) as uint32_t;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        8 => {
            if check_path((*(*nv).value).base, (*(*nv).value).len) == 0
                || check_pseudo_header(http, nv, NGHTTP3_HTTP_FLAG__PATH as uint32_t)
                    == 0
            {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
            if *(*(*nv).value).base.offset(0isize) as ::core::ffi::c_int
                == '/' as ::core::ffi::c_int
            {
                (*http).flags = ((*http).flags as ::core::ffi::c_uint
                    | NGHTTP3_HTTP_FLAG_PATH_REGULAR) as uint32_t;
            } else if (*(*nv).value).len == 1 as size_t
                && *(*(*nv).value).base.offset(0isize) as ::core::ffi::c_int
                    == '*' as ::core::ffi::c_int
            {
                (*http).flags = ((*http).flags as ::core::ffi::c_uint
                    | NGHTTP3_HTTP_FLAG_PATH_ASTERISK) as uint32_t;
            }
        }
        9 => {
            if check_scheme((*(*nv).value).base, (*(*nv).value).len) == 0
                || check_pseudo_header(http, nv, NGHTTP3_HTTP_FLAG__SCHEME as uint32_t)
                    == 0
            {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
            if ::core::mem::size_of::<[::core::ffi::c_char; 5]>().wrapping_sub(1usize)
                == (*(*nv).value).len
                && memieq(
                    b"http\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    (*(*nv).value).base as *const ::core::ffi::c_void,
                    (*(*nv).value).len,
                ) != 0
                || ::core::mem::size_of::<[::core::ffi::c_char; 6]>()
                    .wrapping_sub(1usize) == (*(*nv).value).len
                    && memieq(
                        b"https\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        (*(*nv).value).base as *const ::core::ffi::c_void,
                        (*(*nv).value).len,
                    ) != 0
            {
                (*http).flags = ((*http).flags as ::core::ffi::c_uint
                    | NGHTTP3_HTTP_FLAG_SCHEME_HTTP) as uint32_t;
            }
        }
        1007 => {
            if connect_protocol == 0
                || nghttp3_check_header_value((*(*nv).value).base, (*(*nv).value).len)
                    == 0
                || check_pseudo_header(http, nv, NGHTTP3_HTTP_FLAG__PROTOCOL as uint32_t)
                    == 0
            {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
        }
        1000 => {
            if check_authority((*(*nv).value).base, (*(*nv).value).len) == 0 {
                return NGHTTP3_ERR_REMOVE_HTTP_HEADER;
            }
            if check_pseudo_header(http, nv, NGHTTP3_HTTP_FLAG_HOST as uint32_t) == 0 {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
        }
        55 => {
            if trailers != 0 {
                return NGHTTP3_ERR_REMOVE_HTTP_HEADER;
            }
            if (*http).content_length != -1 as int64_t {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
            (*http).content_length = parse_uint((*(*nv).value).base, (*(*nv).value).len);
            if (*http).content_length == -1 as int64_t {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
        }
        1001 | 1002 | 1003 | 1004 | 1005 => return NGHTTP3_ERR_MALFORMED_HTTP_HEADER,
        1006 => {
            if !(::core::mem::size_of::<[::core::ffi::c_char; 9]>().wrapping_sub(1usize)
                == (*(*nv).value).len
                && memieq(
                    b"trailers\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    (*(*nv).value).base as *const ::core::ffi::c_void,
                    (*(*nv).value).len,
                ) != 0)
            {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
        }
        1008 => {
            if nghttp3_check_header_value((*(*nv).value).base, (*(*nv).value).len) == 0 {
                (*http).flags = ((*http).flags as ::core::ffi::c_uint
                    & !NGHTTP3_HTTP_FLAG_PRIORITY) as uint32_t;
                (*http).flags = ((*http).flags as ::core::ffi::c_uint
                    | NGHTTP3_HTTP_FLAG_BAD_PRIORITY) as uint32_t;
                return NGHTTP3_ERR_REMOVE_HTTP_HEADER;
            }
            if !(trailers != 0
                || (*http).flags & NGHTTP3_HTTP_FLAG_BAD_PRIORITY as uint32_t != 0)
            {
                pri = (*http).pri;
                if nghttp3_http_parse_priority(
                    &raw mut pri,
                    (*(*nv).value).base,
                    (*(*nv).value).len,
                ) == 0 as ::core::ffi::c_int
                {
                    (*http).pri = pri;
                    (*http).flags = ((*http).flags as ::core::ffi::c_uint
                        | NGHTTP3_HTTP_FLAG_PRIORITY) as uint32_t;
                } else {
                    (*http).flags = ((*http).flags as ::core::ffi::c_uint
                        & !NGHTTP3_HTTP_FLAG_PRIORITY) as uint32_t;
                    (*http).flags = ((*http).flags as ::core::ffi::c_uint
                        | NGHTTP3_HTTP_FLAG_BAD_PRIORITY) as uint32_t;
                }
            }
        }
        _ => {
            if *(*(*nv).name).base.offset(0isize) as ::core::ffi::c_int
                == ':' as ::core::ffi::c_int
            {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
            if nghttp3_check_header_value((*(*nv).value).base, (*(*nv).value).len) == 0 {
                return NGHTTP3_ERR_REMOVE_HTTP_HEADER;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn http_response_on_header(
    mut http: *mut nghttp3_http_state,
    mut nv: *const nghttp3_qpack_nv,
    mut trailers: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match (*nv).token {
        11 => {
            if check_pseudo_header(http, nv, NGHTTP3_HTTP_FLAG__STATUS as uint32_t) == 0
            {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
            (*http).status_code = parse_status_code(
                (*(*nv).value).base,
                (*(*nv).value).len,
            );
            if (*http).status_code == -1 as int32_t
                || (*http).status_code == 101 as int32_t
            {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
        }
        55 => {
            if trailers != 0 {
                return NGHTTP3_ERR_REMOVE_HTTP_HEADER;
            }
            if (*http).status_code == 204 as int32_t {
                if (*http).content_length != -1 as int64_t
                    || !(::core::mem::size_of::<[::core::ffi::c_char; 2]>()
                        .wrapping_sub(1usize) == (*(*nv).value).len
                        && memieq(
                            b"0\0".as_ptr() as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            (*(*nv).value).base as *const ::core::ffi::c_void,
                            (*(*nv).value).len,
                        ) != 0)
                {
                    return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
                }
                (*http).content_length = 0 as int64_t;
                return NGHTTP3_ERR_REMOVE_HTTP_HEADER;
            }
            if (*http).status_code / 100 as int32_t == 1 as int32_t
                || (*http).status_code / 100 as int32_t == 2 as int32_t
                    && (*http).flags & NGHTTP3_HTTP_FLAG_METH_CONNECT as uint32_t != 0
            {
                return NGHTTP3_ERR_REMOVE_HTTP_HEADER;
            }
            if (*http).content_length != -1 as int64_t {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
            (*http).content_length = parse_uint((*(*nv).value).base, (*(*nv).value).len);
            if (*http).content_length == -1 as int64_t {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
        }
        1001 | 1002 | 1003 | 1004 | 1005 => return NGHTTP3_ERR_MALFORMED_HTTP_HEADER,
        1006 => {
            if !(::core::mem::size_of::<[::core::ffi::c_char; 9]>().wrapping_sub(1usize)
                == (*(*nv).value).len
                && memieq(
                    b"trailers\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    (*(*nv).value).base as *const ::core::ffi::c_void,
                    (*(*nv).value).len,
                ) != 0)
            {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
        }
        _ => {
            if *(*(*nv).name).base.offset(0isize) as ::core::ffi::c_int
                == ':' as ::core::ffi::c_int
            {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
            if nghttp3_check_header_value((*(*nv).value).base, (*(*nv).value).len) == 0 {
                return NGHTTP3_ERR_REMOVE_HTTP_HEADER;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_http_on_header(
    mut http: *mut nghttp3_http_state,
    mut nv: *const nghttp3_qpack_nv,
    mut request: ::core::ffi::c_int,
    mut trailers: ::core::ffi::c_int,
    mut connect_protocol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*(*nv).name).len == 0 as size_t {
        (*http).flags = ((*http).flags as ::core::ffi::c_uint
            | NGHTTP3_HTTP_FLAG_PSEUDO_HEADER_DISALLOWED) as uint32_t;
        return NGHTTP3_ERR_REMOVE_HTTP_HEADER;
    }
    if *(*(*nv).name).base.offset(0isize) as ::core::ffi::c_int
        == ':' as ::core::ffi::c_int
    {
        if (*nv).token == -1 as int32_t || trailers != 0
            || (*http).flags & NGHTTP3_HTTP_FLAG_PSEUDO_HEADER_DISALLOWED as uint32_t
                != 0
        {
            return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
        }
    } else {
        (*http).flags = ((*http).flags as ::core::ffi::c_uint
            | NGHTTP3_HTTP_FLAG_PSEUDO_HEADER_DISALLOWED) as uint32_t;
        match http_check_nonempty_header_name((*(*nv).name).base, (*(*nv).name).len) {
            0 => return NGHTTP3_ERR_REMOVE_HTTP_HEADER,
            -1 => return NGHTTP3_ERR_MALFORMED_HTTP_HEADER,
            _ => {}
        }
    }
    '_c2rust_label: {
        if (*(*nv).name).len > 0 as size_t {} else {
            __assert_fail(
                b"nv->name->len > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_http.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                564 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    if request != 0 {
        return http_request_on_header(http, nv, trailers, connect_protocol);
    }
    return http_response_on_header(http, nv, trailers);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_http_on_request_headers(
    mut http: *mut nghttp3_http_state,
) -> ::core::ffi::c_int {
    if (*http).flags & NGHTTP3_HTTP_FLAG__PROTOCOL as uint32_t == 0
        && (*http).flags & NGHTTP3_HTTP_FLAG_METH_CONNECT as uint32_t != 0
    {
        if (*http).flags
            & (NGHTTP3_HTTP_FLAG__SCHEME as uint32_t
                | NGHTTP3_HTTP_FLAG__PATH as uint32_t) != 0
            || (*http).flags & NGHTTP3_HTTP_FLAG__AUTHORITY as uint32_t == 0 as uint32_t
        {
            return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
        }
        (*http).content_length = -1 as int64_t;
    } else {
        if (*http).flags & NGHTTP3_HTTP_FLAG_REQ_HEADERS as uint32_t
            != NGHTTP3_HTTP_FLAG_REQ_HEADERS as uint32_t
            || (*http).flags
                & (NGHTTP3_HTTP_FLAG__AUTHORITY as uint32_t
                    | NGHTTP3_HTTP_FLAG_HOST as uint32_t) == 0 as uint32_t
        {
            return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
        }
        if (*http).flags & NGHTTP3_HTTP_FLAG__PROTOCOL as uint32_t != 0 {
            if (*http).flags & NGHTTP3_HTTP_FLAG_METH_CONNECT as uint32_t
                == 0 as uint32_t
                || (*http).flags & NGHTTP3_HTTP_FLAG__AUTHORITY as uint32_t
                    == 0 as uint32_t
            {
                return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
            }
            (*http).content_length = -1 as int64_t;
        }
        if check_path_flags(http) == 0 {
            return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_http_on_response_headers(
    mut http: *mut nghttp3_http_state,
) -> ::core::ffi::c_int {
    if (*http).flags & NGHTTP3_HTTP_FLAG__STATUS as uint32_t == 0 as uint32_t {
        return NGHTTP3_ERR_MALFORMED_HTTP_HEADER;
    }
    if (*http).status_code / 100 as int32_t == 1 as int32_t {
        (*http).flags = (*http).flags & NGHTTP3_HTTP_FLAG_METH_ALL as uint32_t
            | NGHTTP3_HTTP_FLAG_EXPECT_FINAL_RESPONSE as uint32_t;
        (*http).content_length = -1 as int64_t;
        (*http).status_code = -1 as ::core::ffi::c_int as int32_t;
        return 0 as ::core::ffi::c_int;
    }
    (*http).flags = ((*http).flags as ::core::ffi::c_uint
        & !NGHTTP3_HTTP_FLAG_EXPECT_FINAL_RESPONSE) as uint32_t;
    if expect_response_body(http) == 0 {
        (*http).content_length = 0 as int64_t;
    } else if (*http).flags & NGHTTP3_HTTP_FLAG_METH_CONNECT as uint32_t != 0 {
        (*http).content_length = -1 as int64_t;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_http_on_remote_end_stream(
    mut stream: *const nghttp3_stream,
) -> ::core::ffi::c_int {
    if (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.flags
        & NGHTTP3_HTTP_FLAG_EXPECT_FINAL_RESPONSE as uint32_t != 0
        || (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.content_length
            != -1 as int64_t
            && (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.content_length
                != (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.recv_content_length
    {
        return NGHTTP3_ERR_MALFORMED_HTTP_MESSAGING;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_http_on_data_chunk(
    mut stream: *mut nghttp3_stream,
    mut n: size_t,
) -> ::core::ffi::c_int {
    (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.recv_content_length += n as int64_t;
    if (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.flags
        & NGHTTP3_HTTP_FLAG_EXPECT_FINAL_RESPONSE as uint32_t != 0
        || (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.content_length
            != -1 as int64_t
            && (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.recv_content_length
                > (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.content_length
    {
        return NGHTTP3_ERR_MALFORMED_HTTP_MESSAGING;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_http_record_request_method(
    mut stream: *mut nghttp3_stream,
    mut nva: *const nghttp3_nv,
    mut nvlen: size_t,
) {
    let mut i: size_t = 0;
    let mut nv: *const nghttp3_nv = ::core::ptr::null::<nghttp3_nv>();
    i = 0 as size_t;
    while i < nvlen {
        nv = nva.offset(i as isize);
        if !((*nv).namelen == 7 as size_t
            && *(*nv).name.offset(6isize) as ::core::ffi::c_int
                == 'd' as ::core::ffi::c_int
            && memcmp(
                b":metho\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                (*nv).name as *const ::core::ffi::c_void,
                (*nv).namelen.wrapping_sub(1 as size_t),
            ) == 0 as ::core::ffi::c_int)
        {
            i = i.wrapping_add(1);
        } else {
            if ::core::mem::size_of::<[::core::ffi::c_char; 8]>().wrapping_sub(1usize)
                == (*nv).valuelen
                && memcmp(
                    b"CONNECT\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    (*nv).value as *const ::core::ffi::c_void,
                    (*nv).valuelen,
                ) == 0 as ::core::ffi::c_int
            {
                (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.flags = ((*stream)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rx
                    .http
                    .flags as ::core::ffi::c_uint | NGHTTP3_HTTP_FLAG_METH_CONNECT)
                    as uint32_t;
                return;
            }
            if ::core::mem::size_of::<[::core::ffi::c_char; 5]>().wrapping_sub(1usize)
                == (*nv).valuelen
                && memcmp(
                    b"HEAD\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    (*nv).value as *const ::core::ffi::c_void,
                    (*nv).valuelen,
                ) == 0 as ::core::ffi::c_int
            {
                (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.flags = ((*stream)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .rx
                    .http
                    .flags as ::core::ffi::c_uint | NGHTTP3_HTTP_FLAG_METH_HEAD)
                    as uint32_t;
                return;
            }
            return;
        }
    }
}
static mut VALID_HD_NAME_CHARS: [int8_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as int8_t,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    0,
    0,
    1 as int8_t,
    1 as int8_t,
    0,
    1 as int8_t,
    1 as int8_t,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    -1 as int8_t,
    0,
    0,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    0,
    1 as int8_t,
    0,
    1 as int8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub unsafe extern "C" fn nghttp3_check_header_name(
    mut name: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut last: *const uint8_t = ::core::ptr::null::<uint8_t>();
    if len == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    if *name as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
        if len == 1 as size_t {
            return 0 as ::core::ffi::c_int;
        }
        name = name.offset(1);
        len = len.wrapping_sub(1);
    }
    last = name.offset(len as isize);
    while name != last {
        if VALID_HD_NAME_CHARS[*name as usize] as ::core::ffi::c_int
            != 1 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        name = name.offset(1);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn http_check_nonempty_header_name(
    mut name: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut last: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut rv: ::core::ffi::c_int = 0;
    last = name.offset(len as isize);
    while name != last {
        rv = VALID_HD_NAME_CHARS[*name as usize] as ::core::ffi::c_int;
        if rv != 1 as ::core::ffi::c_int {
            return rv;
        }
        name = name.offset(1);
    }
    return 1 as ::core::ffi::c_int;
}
static mut VALID_HD_VALUE_CHARS: [int8_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as int8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    0,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
    1 as int8_t,
];
#[no_mangle]
pub unsafe extern "C" fn nghttp3_check_header_value(
    mut value: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut last: *const uint8_t = ::core::ptr::null::<uint8_t>();
    match len {
        0 => return 1 as ::core::ffi::c_int,
        1 => {
            if is_ws(*value) != 0 {
                return 0 as ::core::ffi::c_int;
            }
        }
        _ => {
            if is_ws(*value) != 0
                || is_ws(
                    *value
                        .offset(len as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize)),
                ) != 0
            {
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    last = value.offset(len as isize);
    while value != last {
        if VALID_HD_VALUE_CHARS[*value as usize] == 0 {
            return 0 as ::core::ffi::c_int;
        }
        value = value.offset(1);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_pri_eq(
    mut a: *const nghttp3_pri,
    mut b: *const nghttp3_pri,
) -> ::core::ffi::c_int {
    return ((*a).0.urgency == (*b).0.urgency
        && (*a).0.inc as ::core::ffi::c_int == (*b).0.inc as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
