extern "C" {
    fn nghttp3_buf_init(buf: *mut nghttp3_buf);
    fn nghttp3_buf_free(buf: *mut nghttp3_buf, mem: *const nghttp3_mem);
    fn nghttp3_buf_left(buf: *const nghttp3_buf) -> size_t;
    fn nghttp3_buf_len(buf: *const nghttp3_buf) -> size_t;
    fn nghttp3_buf_reset(buf: *mut nghttp3_buf);
    fn nghttp3_qpack_encoder_encode(
        encoder: *mut nghttp3_qpack_encoder,
        pbuf: *mut nghttp3_buf,
        rbuf: *mut nghttp3_buf,
        ebuf: *mut nghttp3_buf,
        stream_id: int64_t,
        nva: *const nghttp3_nv,
        nvlen: size_t,
    ) -> ::core::ffi::c_int;
    fn nghttp3_qpack_decoder_write_decoder(
        decoder: *mut nghttp3_qpack_decoder,
        dbuf: *mut nghttp3_buf,
    );
    fn nghttp3_qpack_decoder_get_decoder_streamlen2(
        decoder: *const nghttp3_qpack_decoder,
    ) -> size_t;
    fn nghttp3_get_uvarint(dest: *mut uint64_t, p: *const uint8_t) -> *const uint8_t;
    fn nghttp3_get_uvarintlen(p: *const uint8_t) -> size_t;
    fn nghttp3_put_uvarint(p: *mut uint8_t, n: uint64_t) -> *mut uint8_t;
    fn nghttp3_put_uvarintlen(n: uint64_t) -> size_t;
    fn nghttp3_mem_malloc(
        mem: *const nghttp3_mem,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn nghttp3_tnode_init(tnode: *mut nghttp3_tnode, id: int64_t);
    fn nghttp3_tnode_free(tnode: *mut nghttp3_tnode);
    fn nghttp3_ringbuf_init(
        rb: *mut nghttp3_ringbuf,
        nmemb: size_t,
        size: size_t,
        mem: *const nghttp3_mem,
    ) -> ::core::ffi::c_int;
    fn nghttp3_ringbuf_free(rb: *mut nghttp3_ringbuf);
    fn nghttp3_ringbuf_push_back(rb: *mut nghttp3_ringbuf) -> *mut ::core::ffi::c_void;
    fn nghttp3_ringbuf_pop_front(rb: *mut nghttp3_ringbuf);
    fn nghttp3_ringbuf_get(
        rb: *mut nghttp3_ringbuf,
        offset: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn nghttp3_ringbuf_full(rb: *const nghttp3_ringbuf) -> ::core::ffi::c_int;
    fn nghttp3_ringbuf_reserve(
        rb: *mut nghttp3_ringbuf,
        nmemb: size_t,
    ) -> ::core::ffi::c_int;
    fn nghttp3_buf_wrap_init(buf: *mut nghttp3_buf, src: *mut uint8_t, len: size_t);
    fn nghttp3_buf_cap(buf: *const nghttp3_buf) -> size_t;
    fn nghttp3_typed_buf_init(
        tbuf: *mut nghttp3_typed_buf,
        buf: *const nghttp3_buf,
        r#type: nghttp3_buf_type,
    );
    fn nghttp3_typed_buf_shared_init(
        tbuf: *mut nghttp3_typed_buf,
        chunk: *const nghttp3_buf,
    );
    fn nghttp3_frame_write_hd(
        dest: *mut uint8_t,
        r#type: uint64_t,
        payloadlen: uint64_t,
    ) -> *mut uint8_t;
    fn nghttp3_frame_write_hd_len(r#type: uint64_t, payloadlen: uint64_t) -> size_t;
    fn nghttp3_frame_write_settings(
        dest: *mut uint8_t,
        fr: *const nghttp3_frame_settings,
        payloadlen: uint64_t,
    ) -> *mut uint8_t;
    fn nghttp3_frame_write_settings_len(
        pppayloadlen: *mut uint64_t,
        fr: *const nghttp3_frame_settings,
    ) -> size_t;
    fn nghttp3_frame_write_goaway(
        dest: *mut uint8_t,
        fr: *const nghttp3_frame_goaway,
        payloadlen: uint64_t,
    ) -> *mut uint8_t;
    fn nghttp3_frame_write_goaway_len(
        ppayloadlen: *mut uint64_t,
        fr: *const nghttp3_frame_goaway,
    ) -> size_t;
    fn nghttp3_frame_write_priority_update(
        dest: *mut uint8_t,
        fr: *const nghttp3_frame_priority_update,
        payloadlen: uint64_t,
    ) -> *mut uint8_t;
    fn nghttp3_frame_write_priority_update_len(
        ppayloadlen: *mut uint64_t,
        fr: *const nghttp3_frame_priority_update,
    ) -> size_t;
    fn nghttp3_frame_headers_free(
        fr: *mut nghttp3_frame_headers,
        mem: *const nghttp3_mem,
    );
    fn nghttp3_frame_priority_update_free(
        fr: *mut nghttp3_frame_priority_update,
        mem: *const nghttp3_mem,
    );
    fn nghttp3_balloc_get(
        balloc: *mut nghttp3_balloc,
        pbuf: *mut *mut ::core::ffi::c_void,
        n: size_t,
    ) -> ::core::ffi::c_int;
    fn nghttp3_opl_push(opl: *mut nghttp3_opl, ent: *mut nghttp3_opl_entry);
    fn nghttp3_opl_pop(opl: *mut nghttp3_opl) -> *mut nghttp3_opl_entry;
    fn nghttp3_qpack_stream_context_init(
        sctx: *mut nghttp3_qpack_stream_context,
        stream_id: int64_t,
        mem: *const nghttp3_mem,
    );
    fn nghttp3_qpack_stream_context_free(sctx: *mut nghttp3_qpack_stream_context);
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
    fn nghttp3_objalloc_chunk_len_get(
        objalloc: *mut nghttp3_objalloc,
        len: size_t,
    ) -> *mut nghttp3_chunk;
    fn nghttp3_cpymem(
        dest: *mut uint8_t,
        src: *const uint8_t,
        n: size_t,
    ) -> *mut uint8_t;
    fn nghttp3_http_on_remote_end_stream(
        stream: *const nghttp3_stream,
    ) -> ::core::ffi::c_int;
    fn nghttp3_vec_len_uvarint(
        dest: *mut uint64_t,
        vec: *const nghttp3_vec,
        n: size_t,
    ) -> ::core::ffi::c_int;
    fn nghttp3_unreachable_fail(
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type ptrdiff_t = isize;
pub type nghttp3_ssize = ptrdiff_t;
pub type nghttp3_tstamp = uint64_t;
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
pub struct nghttp3_qpack_encoder {
    pub ctx: nghttp3_qpack_context,
    pub dtable_map: nghttp3_qpack_map,
    pub streams: nghttp3_map,
    pub blocked_streams: nghttp3_ksl,
    pub min_cnts: nghttp3_pq,
    pub krcnt: uint64_t,
    pub state: nghttp3_qpack_decoder_stream_state,
    pub opcode: nghttp3_qpack_decoder_stream_opcode,
    pub rstate: nghttp3_qpack_read_state,
    pub min_dtable_update: size_t,
    pub last_max_dtable_update: size_t,
    pub uninterrupted_decoderlen: size_t,
    pub indexing_strat: nghttp3_qpack_indexing_strat,
    pub flags: uint8_t,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nghttp3_qpack_decoder_stream_opcode(pub ::core::ffi::c_uint);
impl nghttp3_qpack_decoder_stream_opcode {
    pub const NGHTTP3_QPACK_DS_OPCODE_ICNT_INCREMENT: Self = Self(0);
    pub const NGHTTP3_QPACK_DS_OPCODE_SECTION_ACK: Self = Self(1);
    pub const NGHTTP3_QPACK_DS_OPCODE_STREAM_CANCEL: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nghttp3_qpack_decoder_stream_state(pub ::core::ffi::c_uint);
impl nghttp3_qpack_decoder_stream_state {
    pub const NGHTTP3_QPACK_DS_STATE_OPCODE: Self = Self(0);
    pub const NGHTTP3_QPACK_DS_STATE_READ_NUMBER: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_pq {
    pub q: *mut *mut nghttp3_pq_entry,
    pub mem: *const nghttp3_mem,
    pub length: size_t,
    pub capacity: size_t,
    pub less: nghttp3_pq_less,
}
pub type nghttp3_pq_less = Option<
    unsafe extern "C" fn(
        *const nghttp3_pq_entry,
        *const nghttp3_pq_entry,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_pq_entry {
    pub index: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_ksl {
    pub blkalloc: nghttp3_objalloc,
    pub root: *mut nghttp3_ksl_blk,
    pub front: *mut nghttp3_ksl_blk,
    pub back: *mut nghttp3_ksl_blk,
    pub compar: nghttp3_ksl_compar,
    pub search: nghttp3_ksl_search,
    pub n: size_t,
    pub keylen: size_t,
    pub aligned_keylen: size_t,
}
pub type nghttp3_ksl_search = Option<
    unsafe extern "C" fn(
        *const nghttp3_ksl,
        *mut nghttp3_ksl_blk,
        *const ::core::ffi::c_void,
    ) -> size_t,
>;
pub type nghttp3_ksl_key = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_ksl_blk {
    pub c2rust_unnamed: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub c2rust_unnamed: C2Rust_Unnamed_0,
    pub oplent: nghttp3_opl_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_opl_entry {
    pub next: *mut nghttp3_opl_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub next: *mut nghttp3_ksl_blk,
    pub prev: *mut nghttp3_ksl_blk,
    pub nodes: [nghttp3_ksl_node; 32],
    pub keys: *mut uint8_t,
    pub n: uint32_t,
    pub aligned_keylen: uint16_t,
    pub leaf: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_ksl_node {
    pub c2rust_unnamed: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub blk: *mut nghttp3_ksl_blk,
    pub data: *mut ::core::ffi::c_void,
}
pub type nghttp3_ksl_compar = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
    pub c2rust_unnamed: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub next: *mut nghttp3_memblock_hd,
    pub pad: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_map {
    pub keys: *mut nghttp3_map_key_type,
    pub data: *mut *mut ::core::ffi::c_void,
    pub psl: *mut uint8_t,
    pub mem: *const nghttp3_mem,
    pub seed: uint64_t,
    pub size: size_t,
    pub hashbits: size_t,
}
pub type nghttp3_map_key_type = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_qpack_map {
    pub table: [*mut nghttp3_qpack_entry; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_qpack_entry {
    pub nv: nghttp3_qpack_nv,
    pub map_next: *mut nghttp3_qpack_entry,
    pub sum: size_t,
    pub absidx: uint64_t,
    pub hash: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_qpack_context {
    pub dtable: nghttp3_ringbuf,
    pub mem: *const nghttp3_mem,
    pub dtable_size: size_t,
    pub dtable_sum: size_t,
    pub hard_max_dtable_capacity: size_t,
    pub max_dtable_capacity: size_t,
    pub max_blocked_streams: size_t,
    pub next_absidx: uint64_t,
    pub bad: uint8_t,
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
pub struct nghttp3_qpack_decoder {
    pub ctx: nghttp3_qpack_context,
    pub state: nghttp3_qpack_encoder_stream_state,
    pub opcode: nghttp3_qpack_encoder_stream_opcode,
    pub rstate: nghttp3_qpack_read_state,
    pub dbuf: nghttp3_buf,
    pub written_icnt: uint64_t,
    pub max_concurrent_streams: size_t,
    pub uninterrupted_encoderlen: size_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nghttp3_qpack_encoder_stream_opcode(pub ::core::ffi::c_uint);
impl nghttp3_qpack_encoder_stream_opcode {
    pub const NGHTTP3_QPACK_ES_OPCODE_INSERT_INDEXED: Self = Self(0);
    pub const NGHTTP3_QPACK_ES_OPCODE_INSERT: Self = Self(1);
    pub const NGHTTP3_QPACK_ES_OPCODE_DUPLICATE: Self = Self(2);
    pub const NGHTTP3_QPACK_ES_OPCODE_SET_DTABLE_CAP: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nghttp3_qpack_encoder_stream_state(pub ::core::ffi::c_uint);
impl nghttp3_qpack_encoder_stream_state {
    pub const NGHTTP3_QPACK_ES_STATE_OPCODE: Self = Self(0);
    pub const NGHTTP3_QPACK_ES_STATE_READ_INDEX: Self = Self(1);
    pub const NGHTTP3_QPACK_ES_STATE_CHECK_NAME_HUFFMAN: Self = Self(2);
    pub const NGHTTP3_QPACK_ES_STATE_READ_NAMELEN: Self = Self(3);
    pub const NGHTTP3_QPACK_ES_STATE_READ_NAME_HUFFMAN: Self = Self(4);
    pub const NGHTTP3_QPACK_ES_STATE_READ_NAME: Self = Self(5);
    pub const NGHTTP3_QPACK_ES_STATE_CHECK_VALUE_HUFFMAN: Self = Self(6);
    pub const NGHTTP3_QPACK_ES_STATE_READ_VALUELEN: Self = Self(7);
    pub const NGHTTP3_QPACK_ES_STATE_READ_VALUE_HUFFMAN: Self = Self(8);
    pub const NGHTTP3_QPACK_ES_STATE_READ_VALUE: Self = Self(9);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_conn {
    pub out_chunk_objalloc: nghttp3_objalloc,
    pub stream_objalloc: nghttp3_objalloc,
    pub callbacks: nghttp3_callbacks,
    pub streams: nghttp3_map,
    pub qdec: nghttp3_qpack_decoder,
    pub qenc: nghttp3_qpack_encoder,
    pub qpack_blocked_streams: nghttp3_pq,
    pub glitch_rlim: nghttp3_ratelim,
    pub sched: [C2Rust_Unnamed_19; 8],
    pub mem: *const nghttp3_mem,
    pub user_data: *mut ::core::ffi::c_void,
    pub server: ::core::ffi::c_int,
    pub flags: uint16_t,
    pub local: C2Rust_Unnamed_17,
    pub remote: C2Rust_Unnamed_15,
    pub rx: C2Rust_Unnamed_11,
    pub tx: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub qpack: C2Rust_Unnamed_10,
    pub ctrl: *mut nghttp3_stream,
    pub qenc: *mut nghttp3_stream,
    pub qdec: *mut nghttp3_stream,
    pub goaway_id: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_stream {
    pub c2rust_unnamed: C2Rust_Unnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub c2rust_unnamed: C2Rust_Unnamed_5,
    pub oplent: nghttp3_opl_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
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
    pub tx: C2Rust_Unnamed_7,
    pub rx: C2Rust_Unnamed_6,
    pub flags: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
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
pub struct C2Rust_Unnamed_7 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub c2rust_unnamed: C2Rust_Unnamed_9,
    pub pri: nghttp3_pri,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
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
pub struct nghttp3_data_reader {
    pub read_data: nghttp3_read_data_callback,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub rbuf: nghttp3_buf,
    pub ebuf: nghttp3_buf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub goaway_id: int64_t,
    pub max_stream_id_bidi: int64_t,
    pub c2rust_unnamed: C2Rust_Unnamed_12,
    pub originbuf: *mut uint8_t,
    pub originbuflen: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub c2rust_unnamed: C2Rust_Unnamed_14,
    pub c2rust_unnamed_0: C2Rust_Unnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub originlen_offset: size_t,
    pub originlen: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub pri_fieldbuf: [uint8_t; 8],
    pub pri_fieldbuflen: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub bidi: C2Rust_Unnamed_16,
    pub settings: nghttp3_proto_settings,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub idtr: nghttp3_idtr,
    pub max_client_streams: uint64_t,
    pub num_streams: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_idtr {
    pub gap: nghttp3_gaptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_gaptr {
    pub gap: nghttp3_ksl,
    pub mem: *const nghttp3_mem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub origin_list: nghttp3_vec,
    pub settings: nghttp3_settings,
    pub uni: C2Rust_Unnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub max_pushes: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub spq: nghttp3_pq,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_ratelim {
    pub burst: uint64_t,
    pub rate: uint64_t,
    pub tokens: uint64_t,
    pub carry: uint64_t,
    pub ts: nghttp3_tstamp,
}
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
pub type nghttp3_recv_settings2 = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        *const nghttp3_proto_settings,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_rand = Option<unsafe extern "C" fn(*mut uint8_t, size_t) -> ()>;
pub type nghttp3_end_origin = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
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
pub type nghttp3_recv_settings = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        *const nghttp3_settings,
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
pub type nghttp3_reset_stream = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        uint64_t,
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
pub type nghttp3_end_headers = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        ::core::ffi::c_int,
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
pub type nghttp3_begin_headers = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
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
pub type nghttp3_stream_close = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        uint64_t,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type nghttp3_acked_stream_data = Option<
    unsafe extern "C" fn(
        *mut nghttp3_conn,
        int64_t,
        uint64_t,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nghttp3_buf_type(pub ::core::ffi::c_uint);
impl nghttp3_buf_type {
    pub const NGHTTP3_BUF_TYPE_PRIVATE: Self = Self(0);
    pub const NGHTTP3_BUF_TYPE_SHARED: Self = Self(1);
    pub const NGHTTP3_BUF_TYPE_ALIEN: Self = Self(2);
    pub const NGHTTP3_BUF_TYPE_ALIEN_NO_ACK: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_typed_buf {
    pub buf: nghttp3_buf,
    pub r#type: nghttp3_buf_type,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nghttp3_stream_http_event(pub ::core::ffi::c_uint);
impl nghttp3_stream_http_event {
    pub const NGHTTP3_HTTP_EVENT_DATA_BEGIN: Self = Self(0);
    pub const NGHTTP3_HTTP_EVENT_DATA_END: Self = Self(1);
    pub const NGHTTP3_HTTP_EVENT_HEADERS_BEGIN: Self = Self(2);
    pub const NGHTTP3_HTTP_EVENT_HEADERS_END: Self = Self(3);
    pub const NGHTTP3_HTTP_EVENT_MSG_END: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_chunk {
    pub oplent: nghttp3_opl_entry,
}
#[inline]
unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
    return ((__bsx as ::core::ffi::c_ulonglong
        & 0xff00000000000000 as ::core::ffi::c_ulonglong) >> 56 as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_ulonglong
            & 0xff000000000000 as ::core::ffi::c_ulonglong) >> 40 as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_ulonglong
            & 0xff0000000000 as ::core::ffi::c_ulonglong) >> 24 as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_ulonglong & 0xff00000000 as ::core::ffi::c_ulonglong)
            >> 8 as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_ulonglong & 0xff000000 as ::core::ffi::c_ulonglong)
            << 8 as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_ulonglong & 0xff0000 as ::core::ffi::c_ulonglong)
            << 24 as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_ulonglong & 0xff00 as ::core::ffi::c_ulonglong)
            << 40 as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_ulonglong & 0xff as ::core::ffi::c_ulonglong)
            << 56 as ::core::ffi::c_int) as __uint64_t;
}
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NGHTTP3_ERR_INVALID_ARGUMENT: ::core::ffi::c_int = -101 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_WOULDBLOCK: ::core::ffi::c_int = -103 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_MALFORMED_HTTP_MESSAGING: ::core::ffi::c_int = -107
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_STREAM_DATA_OVERFLOW: ::core::ffi::c_int = -112
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_H3_FRAME_UNEXPECTED: ::core::ffi::c_int = -601
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_NOMEM: ::core::ffi::c_int = -901 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_CALLBACK_FAILURE: ::core::ffi::c_int = -902 as ::core::ffi::c_int;
pub const NGHTTP3_DATA_FLAG_EOF: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const NGHTTP3_DATA_FLAG_NO_END_STREAM: ::core::ffi::c_uint = 0x2
    as ::core::ffi::c_uint;
pub const NGHTTP3_DEFAULT_URGENCY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NGHTTP3_PQ_BAD_INDEX: ::core::ffi::c_ulong = SIZE_MAX;
#[inline]
unsafe extern "C" fn nghttp3_ringbuf_len(mut rb: *const nghttp3_ringbuf) -> size_t {
    return (*rb).len;
}
pub const NGHTTP3_FRAME_DATA: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const NGHTTP3_FRAME_HEADERS: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const NGHTTP3_FRAME_SETTINGS: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const NGHTTP3_FRAME_GOAWAY: uint64_t = 7 as uint64_t;
pub const NGHTTP3_FRAME_PRIORITY_UPDATE: uint64_t = 984832 as uint64_t;
pub const NGHTTP3_FRAME_ORIGIN: uint64_t = 12 as uint64_t;
pub const NGHTTP3_SETTINGS_ID_MAX_FIELD_SECTION_SIZE: ::core::ffi::c_uint = 0x6
    as ::core::ffi::c_uint;
pub const NGHTTP3_SETTINGS_ID_QPACK_MAX_TABLE_CAPACITY: ::core::ffi::c_uint = 0x1
    as ::core::ffi::c_uint;
pub const NGHTTP3_SETTINGS_ID_QPACK_BLOCKED_STREAMS: ::core::ffi::c_uint = 0x7
    as ::core::ffi::c_uint;
pub const NGHTTP3_SETTINGS_ID_ENABLE_CONNECT_PROTOCOL: ::core::ffi::c_uint = 0x8
    as ::core::ffi::c_uint;
pub const NGHTTP3_SETTINGS_ID_H3_DATAGRAM: ::core::ffi::c_uint = 0x33
    as ::core::ffi::c_uint;
pub const NGHTTP3_MAX_VARINT: ::core::ffi::c_ulonglong = ((1 as ::core::ffi::c_ulonglong)
    << 62 as ::core::ffi::c_int)
    .wrapping_sub(1 as ::core::ffi::c_ulonglong);
#[inline]
unsafe extern "C" fn nghttp3_max_unsigned_long_int(
    mut a: ::core::ffi::c_ulong,
    mut b: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    return if a < b { b } else { a };
}
#[inline]
unsafe extern "C" fn nghttp3_min_unsigned_long_int(
    mut a: ::core::ffi::c_ulong,
    mut b: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    return if a < b { a } else { b };
}
pub const NGHTTP3_STREAM_MIN_CHUNK_SIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NGHTTP3_MIN_UNSENT_BYTES: ::core::ffi::c_int = 16382 as ::core::ffi::c_int;
pub const NGHTTP3_STREAM_FLAG_FC_BLOCKED: ::core::ffi::c_uint = 0x2
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_FLAG_READ_DATA_BLOCKED: ::core::ffi::c_uint = 0x4
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_FLAG_WRITE_END_STREAM: ::core::ffi::c_uint = 0x8
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_FLAG_SHUT_WR: ::core::ffi::c_uint = 0x100
    as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn nghttp3_objalloc_stream_release(
    mut objalloc: *mut nghttp3_objalloc,
    mut obj: *mut nghttp3_stream,
) {
    nghttp3_opl_push(&raw mut (*objalloc).opl, &raw mut (*obj).c2rust_unnamed.oplent);
}
#[inline]
unsafe extern "C" fn nghttp3_objalloc_chunk_release(
    mut objalloc: *mut nghttp3_objalloc,
    mut obj: *mut nghttp3_chunk,
) {
    nghttp3_opl_push(&raw mut (*objalloc).opl, &raw mut (*obj).oplent);
}
pub const NGHTTP3_HTTP_FLAG_METH_CONNECT: ::core::ffi::c_uint = 0x80
    as ::core::ffi::c_uint;
pub const NGHTTP3_HTTP_FLAG_EXPECT_FINAL_RESPONSE: ::core::ffi::c_uint = 0x2000
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_MAX_COPY_THRES: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn nghttp3_objalloc_stream_len_get(
    mut objalloc: *mut nghttp3_objalloc,
    mut len: size_t,
) -> *mut nghttp3_stream {
    let mut oplent: *mut nghttp3_opl_entry = nghttp3_opl_pop(&raw mut (*objalloc).opl);
    let mut obj: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    let mut rv: ::core::ffi::c_int = 0;
    if oplent.is_null() {
        rv = nghttp3_balloc_get(
            &raw mut (*objalloc).balloc,
            &raw mut obj as *mut *mut ::core::ffi::c_void,
            len,
        );
        if rv != 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<nghttp3_stream>();
        }
        return obj;
    }
    return (oplent as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_stream;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_objalloc_stream_get(
    mut objalloc: *mut nghttp3_objalloc,
) -> *mut nghttp3_stream {
    let mut oplent: *mut nghttp3_opl_entry = nghttp3_opl_pop(&raw mut (*objalloc).opl);
    let mut obj: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    let mut rv: ::core::ffi::c_int = 0;
    if oplent.is_null() {
        rv = nghttp3_balloc_get(
            &raw mut (*objalloc).balloc,
            &raw mut obj as *mut *mut ::core::ffi::c_void,
            ::core::mem::size_of::<nghttp3_stream>(),
        );
        if rv != 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<nghttp3_stream>();
        }
        return obj;
    }
    return (oplent as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_stream;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_new(
    mut pstream: *mut *mut nghttp3_stream,
    mut stream_id: int64_t,
    mut callbacks: *const nghttp3_stream_callbacks,
    mut out_chunk_objalloc: *mut nghttp3_objalloc,
    mut stream_objalloc: *mut nghttp3_objalloc,
    mut mem: *const nghttp3_mem,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = nghttp3_objalloc_stream_get(stream_objalloc);
    if stream.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    *stream = nghttp3_stream {
        c2rust_unnamed: C2Rust_Unnamed_4 {
            c2rust_unnamed: C2Rust_Unnamed_5 {
                mem: mem,
                out_chunk_objalloc: out_chunk_objalloc,
                stream_objalloc: stream_objalloc,
                node: nghttp3_tnode {
                    pe: nghttp3_pq_entry { index: 0 },
                    id: 0,
                    cycle: 0,
                    pri: nghttp3_pri { urgency: 0, inc: 0 },
                },
                qpack_blocked_pe: nghttp3_pq_entry {
                    index: NGHTTP3_PQ_BAD_INDEX as size_t,
                },
                callbacks: nghttp3_stream_callbacks {
                    acked_data: None,
                },
                frq: nghttp3_ringbuf {
                    buf: ::core::ptr::null_mut::<uint8_t>(),
                    mem: ::core::ptr::null::<nghttp3_mem>(),
                    nmemb: 0,
                    size: 0,
                    first: 0,
                    len: 0,
                },
                chunks: nghttp3_ringbuf {
                    buf: ::core::ptr::null_mut::<uint8_t>(),
                    mem: ::core::ptr::null::<nghttp3_mem>(),
                    nmemb: 0,
                    size: 0,
                    first: 0,
                    len: 0,
                },
                outq: nghttp3_ringbuf {
                    buf: ::core::ptr::null_mut::<uint8_t>(),
                    mem: ::core::ptr::null::<nghttp3_mem>(),
                    nmemb: 0,
                    size: 0,
                    first: 0,
                    len: 0,
                },
                inq: nghttp3_ringbuf {
                    buf: ::core::ptr::null_mut::<uint8_t>(),
                    mem: ::core::ptr::null::<nghttp3_mem>(),
                    nmemb: 0,
                    size: 0,
                    first: 0,
                    len: 0,
                },
                qpack_sctx: nghttp3_qpack_stream_context {
                    rstate: nghttp3_qpack_read_state {
                        huffman_ctx: nghttp3_qpack_huffman_decode_context {
                            fstate: 0,
                            flags: 0,
                        },
                        namebuf: nghttp3_buf {
                            begin: ::core::ptr::null_mut::<uint8_t>(),
                            end: ::core::ptr::null_mut::<uint8_t>(),
                            pos: ::core::ptr::null_mut::<uint8_t>(),
                            last: ::core::ptr::null_mut::<uint8_t>(),
                        },
                        valuebuf: nghttp3_buf {
                            begin: ::core::ptr::null_mut::<uint8_t>(),
                            end: ::core::ptr::null_mut::<uint8_t>(),
                            pos: ::core::ptr::null_mut::<uint8_t>(),
                            last: ::core::ptr::null_mut::<uint8_t>(),
                        },
                        name: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
                        value: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
                        left: 0,
                        prefix: 0,
                        shift: 0,
                        absidx: 0,
                        never: 0,
                        dynamic: 0,
                        huffman_encoded: 0,
                    },
                    mem: ::core::ptr::null::<nghttp3_mem>(),
                    stream_id: 0,
                    ricnt: 0,
                    base: 0,
                    state: nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_RICNT,
                    opcode: nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_INDEXED,
                    dbase_sign: 0,
                },
                conn: ::core::ptr::null_mut::<nghttp3_conn>(),
                user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                unsent_bytes: 0,
                outq_idx: 0,
                ack_base: 0,
                ack_offset: 0,
                unscheduled_nwrite: 0,
                r#type: 0,
                rstate: nghttp3_stream_read_state {
                    rvint: nghttp3_varint_read_state {
                        acc: 0,
                        left: 0,
                    },
                    iv: nghttp3_settings_entry {
                        id: 0,
                        value: 0,
                    },
                    fr: nghttp3_frame {
                        hd: nghttp3_frame_hd { r#type: 0 },
                    },
                    left: 0,
                    state: 0,
                },
                tx: C2Rust_Unnamed_7 { offset: 0 },
                rx: C2Rust_Unnamed_6 {
                    hstate: nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_NONE,
                    http: nghttp3_http_state {
                        content_length: -1 as int64_t,
                        recv_content_length: 0,
                        pri: nghttp3_pri(C2Rust_nghttp3_pri_Inner {
                            urgency: NGHTTP3_DEFAULT_URGENCY as uint32_t,
                            inc: 0,
                        }),
                        status_code: -1 as int32_t,
                        flags: 0,
                    },
                },
                flags: 0,
            },
        },
    };
    nghttp3_tnode_init(&raw mut (*stream).c2rust_unnamed.c2rust_unnamed.node, stream_id);
    nghttp3_ringbuf_init(
        &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.frq,
        0 as size_t,
        ::core::mem::size_of::<nghttp3_frame>(),
        mem,
    );
    nghttp3_ringbuf_init(
        &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.chunks,
        0 as size_t,
        ::core::mem::size_of::<nghttp3_buf>(),
        mem,
    );
    nghttp3_ringbuf_init(
        &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.outq,
        0 as size_t,
        ::core::mem::size_of::<nghttp3_typed_buf>(),
        mem,
    );
    nghttp3_ringbuf_init(
        &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.inq,
        0 as size_t,
        ::core::mem::size_of::<nghttp3_buf>(),
        mem,
    );
    nghttp3_qpack_stream_context_init(
        &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.qpack_sctx,
        stream_id,
        mem,
    );
    if !callbacks.is_null() {
        (*stream).c2rust_unnamed.c2rust_unnamed.callbacks = *callbacks;
    }
    *pstream = stream;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn delete_outq(
    mut outq: *mut nghttp3_ringbuf,
    mut mem: *const nghttp3_mem,
) {
    let mut tbuf: *mut nghttp3_typed_buf = ::core::ptr::null_mut::<nghttp3_typed_buf>();
    let mut i: size_t = 0;
    let mut len: size_t = nghttp3_ringbuf_len(outq);
    i = 0 as size_t;
    while i < len {
        tbuf = nghttp3_ringbuf_get(outq, i) as *mut nghttp3_typed_buf;
        if (*tbuf).r#type.0 == nghttp3_buf_type::NGHTTP3_BUF_TYPE_PRIVATE.0 {
            nghttp3_buf_free(&raw mut (*tbuf).buf, mem);
        }
        i = i.wrapping_add(1);
    }
    nghttp3_ringbuf_free(outq);
}
unsafe extern "C" fn delete_chunks(
    mut chunks: *mut nghttp3_ringbuf,
    mut mem: *const nghttp3_mem,
) {
    let mut buf: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    let mut i: size_t = 0;
    let mut len: size_t = nghttp3_ringbuf_len(chunks);
    i = 0 as size_t;
    while i < len {
        buf = nghttp3_ringbuf_get(chunks, i) as *mut nghttp3_buf;
        nghttp3_buf_free(buf, mem);
        i = i.wrapping_add(1);
    }
    nghttp3_ringbuf_free(chunks);
}
unsafe extern "C" fn delete_out_chunks(
    mut chunks: *mut nghttp3_ringbuf,
    mut out_chunk_objalloc: *mut nghttp3_objalloc,
    mut mem: *const nghttp3_mem,
) {
    let mut buf: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    let mut i: size_t = 0;
    let mut len: size_t = nghttp3_ringbuf_len(chunks);
    i = 0 as size_t;
    while i < len {
        buf = nghttp3_ringbuf_get(chunks, i) as *mut nghttp3_buf;
        if nghttp3_buf_cap(buf) == NGHTTP3_STREAM_MIN_CHUNK_SIZE as size_t {
            nghttp3_objalloc_chunk_release(
                out_chunk_objalloc,
                (*buf).begin as *mut ::core::ffi::c_void as *mut nghttp3_chunk,
            );
        } else {
            nghttp3_buf_free(buf, mem);
        }
        i = i.wrapping_add(1);
    }
    nghttp3_ringbuf_free(chunks);
}
unsafe extern "C" fn delete_frq(
    mut frq: *mut nghttp3_ringbuf,
    mut mem: *const nghttp3_mem,
) {
    let mut fr: *mut nghttp3_frame = ::core::ptr::null_mut::<nghttp3_frame>();
    let mut i: size_t = 0;
    let mut len: size_t = nghttp3_ringbuf_len(frq);
    i = 0 as size_t;
    while i < len {
        fr = nghttp3_ringbuf_get(frq, i) as *mut nghttp3_frame;
        match (*fr).hd.r#type {
            0x1 => {
                nghttp3_frame_headers_free(&raw mut (*fr).headers, mem);
            }
            NGHTTP3_FRAME_PRIORITY_UPDATE => {
                nghttp3_frame_priority_update_free(&raw mut (*fr).priority_update, mem);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    nghttp3_ringbuf_free(frq);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_del(mut stream: *mut nghttp3_stream) {
    if stream.is_null() {
        return;
    }
    nghttp3_qpack_stream_context_free(
        &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.qpack_sctx,
    );
    delete_chunks(
        &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.inq,
        (*stream).c2rust_unnamed.c2rust_unnamed.mem,
    );
    delete_outq(
        &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.outq,
        (*stream).c2rust_unnamed.c2rust_unnamed.mem,
    );
    delete_out_chunks(
        &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.chunks,
        (*stream).c2rust_unnamed.c2rust_unnamed.out_chunk_objalloc,
        (*stream).c2rust_unnamed.c2rust_unnamed.mem,
    );
    delete_frq(
        &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.frq,
        (*stream).c2rust_unnamed.c2rust_unnamed.mem,
    );
    nghttp3_tnode_free(&raw mut (*stream).c2rust_unnamed.c2rust_unnamed.node);
    nghttp3_objalloc_stream_release(
        (*stream).c2rust_unnamed.c2rust_unnamed.stream_objalloc,
        stream,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_varint_read_state_reset(
    mut rvint: *mut nghttp3_varint_read_state,
) {
    *rvint = nghttp3_varint_read_state {
        acc: 0 as uint64_t,
        left: 0,
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_read_state_reset(
    mut rstate: *mut nghttp3_stream_read_state,
) {
    *rstate = nghttp3_stream_read_state {
        rvint: nghttp3_varint_read_state {
            acc: 0 as uint64_t,
            left: 0,
        },
        iv: nghttp3_settings_entry {
            id: 0,
            value: 0,
        },
        fr: nghttp3_frame {
            hd: nghttp3_frame_hd { r#type: 0 },
        },
        left: 0,
        state: 0,
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_read_varint(
    mut rvint: *mut nghttp3_varint_read_state,
    mut begin: *const uint8_t,
    mut end: *const uint8_t,
    mut fin: ::core::ffi::c_int,
) -> nghttp3_ssize {
    let mut len: size_t = 0;
    let mut vlen: size_t = 0;
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    '_c2rust_label: {
        if begin != end {} else {
            __assert_fail(
                b"begin != end\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_uint,
                b"nghttp3_ssize nghttp3_read_varint(nghttp3_varint_read_state *, const uint8_t *, const uint8_t *, int)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if (*rvint).left == 0 as size_t {
        '_c2rust_label_0: {
            if (*rvint).acc == 0 as uint64_t {} else {
                __assert_fail(
                    b"rvint->acc == 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    190 as ::core::ffi::c_uint,
                    b"nghttp3_ssize nghttp3_read_varint(nghttp3_varint_read_state *, const uint8_t *, const uint8_t *, int)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        vlen = nghttp3_get_uvarintlen(begin);
        len = nghttp3_min_unsigned_long_int(
            vlen as ::core::ffi::c_ulong,
            end.offset_from(begin) as ::core::ffi::c_ulong,
        ) as size_t;
        if vlen <= len {
            nghttp3_get_uvarint(&raw mut (*rvint).acc, begin);
            return vlen as nghttp3_ssize;
        }
        if fin != 0 {
            return NGHTTP3_ERR_INVALID_ARGUMENT as nghttp3_ssize;
        }
        p = (&raw mut (*rvint).acc as *mut uint8_t)
            .offset(::core::mem::size_of::<uint64_t>().wrapping_sub(vlen) as isize);
        memcpy(p as *mut ::core::ffi::c_void, begin as *const ::core::ffi::c_void, len);
        *p = (*p as ::core::ffi::c_uint & 0x3f as ::core::ffi::c_uint) as uint8_t;
        (*rvint).left = vlen.wrapping_sub(len);
        return len as nghttp3_ssize;
    }
    len = nghttp3_min_unsigned_long_int(
        (*rvint).left as ::core::ffi::c_ulong,
        end.offset_from(begin) as ::core::ffi::c_ulong,
    ) as size_t;
    p = (&raw mut (*rvint).acc as *mut uint8_t)
        .offset(::core::mem::size_of::<uint64_t>().wrapping_sub((*rvint).left) as isize);
    memcpy(p as *mut ::core::ffi::c_void, begin as *const ::core::ffi::c_void, len);
    (*rvint).left = (*rvint).left.wrapping_sub(len);
    if (*rvint).left == 0 as size_t {
        (*rvint).acc = __bswap_64((*rvint).acc) as uint64_t;
    } else if fin != 0 {
        return NGHTTP3_ERR_INVALID_ARGUMENT as nghttp3_ssize
    }
    return len as nghttp3_ssize;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_frq_emplace(
    mut stream: *mut nghttp3_stream,
    mut pfr: *mut *mut nghttp3_frame,
) -> ::core::ffi::c_int {
    let mut frq: *mut nghttp3_ringbuf = &raw mut (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .frq;
    let mut rv: ::core::ffi::c_int = 0;
    if nghttp3_ringbuf_full(frq) != 0 {
        let mut nlen: size_t = nghttp3_max_unsigned_long_int(
            4 as ::core::ffi::c_ulong,
            (nghttp3_ringbuf_len(frq) as ::core::ffi::c_ulong)
                .wrapping_mul(2 as ::core::ffi::c_ulong),
        ) as size_t;
        rv = nghttp3_ringbuf_reserve(frq, nlen);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
    }
    *pfr = nghttp3_ringbuf_push_back(frq) as *mut nghttp3_frame;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_fill_outq(
    mut stream: *mut nghttp3_stream,
) -> ::core::ffi::c_int {
    let mut frq: *mut nghttp3_ringbuf = &raw mut (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .frq;
    let mut fr: *mut nghttp3_frame = ::core::ptr::null_mut::<nghttp3_frame>();
    let mut data_eof: ::core::ffi::c_int = 0;
    let mut rv: ::core::ffi::c_int = 0;
    while nghttp3_ringbuf_len(frq) != 0
        && (*stream).c2rust_unnamed.c2rust_unnamed.unsent_bytes
            < NGHTTP3_MIN_UNSENT_BYTES as uint64_t
    {
        fr = nghttp3_ringbuf_get(frq, 0 as size_t) as *mut nghttp3_frame;
        match (*fr).hd.r#type {
            0x4 => {
                rv = nghttp3_stream_write_settings(stream, &raw mut (*fr).settings);
                if rv != 0 as ::core::ffi::c_int {
                    return rv;
                }
            }
            0x1 => {
                rv = nghttp3_stream_write_headers(stream, &raw mut (*fr).headers);
                if rv != 0 as ::core::ffi::c_int {
                    return rv;
                }
                nghttp3_frame_headers_free(
                    &raw mut (*fr).headers,
                    (*stream).c2rust_unnamed.c2rust_unnamed.mem,
                );
            }
            0 => {
                rv = nghttp3_stream_write_data(
                    stream,
                    &raw mut data_eof,
                    &raw mut (*fr).data,
                );
                if rv != 0 as ::core::ffi::c_int {
                    return rv;
                }
                if (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
                    & NGHTTP3_STREAM_FLAG_READ_DATA_BLOCKED != 0
                {
                    return 0 as ::core::ffi::c_int;
                }
                if data_eof == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
            NGHTTP3_FRAME_GOAWAY => {
                rv = nghttp3_stream_write_goaway(stream, &raw mut (*fr).goaway);
                if rv != 0 as ::core::ffi::c_int {
                    return rv;
                }
            }
            NGHTTP3_FRAME_PRIORITY_UPDATE => {
                rv = nghttp3_stream_write_priority_update(
                    stream,
                    &raw mut (*fr).priority_update,
                );
                if rv != 0 as ::core::ffi::c_int {
                    return rv;
                }
                nghttp3_frame_priority_update_free(
                    &raw mut (*fr).priority_update,
                    (*stream).c2rust_unnamed.c2rust_unnamed.mem,
                );
            }
            NGHTTP3_FRAME_ORIGIN => {
                rv = nghttp3_stream_write_origin(stream, &raw mut (*fr).origin);
                if rv != 0 as ::core::ffi::c_int {
                    return rv;
                }
            }
            _ => {}
        }
        nghttp3_ringbuf_pop_front(frq);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_write_stream_type(
    mut stream: *mut nghttp3_stream,
) -> ::core::ffi::c_int {
    let mut len: size_t = nghttp3_put_uvarintlen(
        (*stream).c2rust_unnamed.c2rust_unnamed.r#type,
    );
    let mut chunk: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    let mut tbuf: nghttp3_typed_buf = nghttp3_typed_buf {
        buf: nghttp3_buf {
            begin: ::core::ptr::null_mut::<uint8_t>(),
            end: ::core::ptr::null_mut::<uint8_t>(),
            pos: ::core::ptr::null_mut::<uint8_t>(),
            last: ::core::ptr::null_mut::<uint8_t>(),
        },
        r#type: nghttp3_buf_type::NGHTTP3_BUF_TYPE_PRIVATE,
    };
    let mut rv: ::core::ffi::c_int = 0;
    rv = nghttp3_stream_ensure_chunk(stream, len);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    chunk = nghttp3_stream_get_chunk(stream);
    nghttp3_typed_buf_shared_init(&raw mut tbuf, chunk);
    (*chunk).last = nghttp3_put_uvarint(
        (*chunk).last,
        (*stream).c2rust_unnamed.c2rust_unnamed.r#type,
    );
    tbuf.buf.last = (*chunk).last;
    return nghttp3_stream_outq_add(stream, &raw mut tbuf);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_write_settings(
    mut stream: *mut nghttp3_stream,
    mut infr: *const nghttp3_frame_settings,
) -> ::core::ffi::c_int {
    let mut len: size_t = 0;
    let mut rv: ::core::ffi::c_int = 0;
    let mut chunk: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    let mut tbuf: nghttp3_typed_buf = nghttp3_typed_buf {
        buf: nghttp3_buf {
            begin: ::core::ptr::null_mut::<uint8_t>(),
            end: ::core::ptr::null_mut::<uint8_t>(),
            pos: ::core::ptr::null_mut::<uint8_t>(),
            last: ::core::ptr::null_mut::<uint8_t>(),
        },
        r#type: nghttp3_buf_type::NGHTTP3_BUF_TYPE_PRIVATE,
    };
    let mut ents: [nghttp3_settings_entry; 16] = [nghttp3_settings_entry {
        id: 0,
        value: 0,
    }; 16];
    let mut fr: nghttp3_frame_settings = nghttp3_frame_settings {
        r#type: NGHTTP3_FRAME_SETTINGS as uint64_t,
        niv: 3 as size_t,
        iv: &raw mut ents as *mut nghttp3_settings_entry,
        local_settings: ::core::ptr::null::<nghttp3_settings>(),
    };
    let mut local_settings: *const nghttp3_settings = (*infr).local_settings;
    let mut payloadlen: uint64_t = 0;
    ents[0usize] = nghttp3_settings_entry {
        id: NGHTTP3_SETTINGS_ID_MAX_FIELD_SECTION_SIZE as uint64_t,
        value: (*local_settings).max_field_section_size,
    };
    ents[1usize] = nghttp3_settings_entry {
        id: NGHTTP3_SETTINGS_ID_QPACK_MAX_TABLE_CAPACITY as uint64_t,
        value: (*local_settings).qpack_max_dtable_capacity as uint64_t,
    };
    ents[2usize] = nghttp3_settings_entry {
        id: NGHTTP3_SETTINGS_ID_QPACK_BLOCKED_STREAMS as uint64_t,
        value: (*local_settings).qpack_blocked_streams as uint64_t,
    };
    if (*local_settings).h3_datagram != 0 {
        ents[fr.niv] = nghttp3_settings_entry {
            id: NGHTTP3_SETTINGS_ID_H3_DATAGRAM as uint64_t,
            value: 1 as uint64_t,
        };
        fr.niv = fr.niv.wrapping_add(1);
    }
    if (*local_settings).enable_connect_protocol != 0 {
        ents[fr.niv] = nghttp3_settings_entry {
            id: NGHTTP3_SETTINGS_ID_ENABLE_CONNECT_PROTOCOL as uint64_t,
            value: 1 as uint64_t,
        };
        fr.niv = fr.niv.wrapping_add(1);
    }
    len = nghttp3_frame_write_settings_len(&raw mut payloadlen, &raw mut fr);
    rv = nghttp3_stream_ensure_chunk(stream, len);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    chunk = nghttp3_stream_get_chunk(stream);
    nghttp3_typed_buf_shared_init(&raw mut tbuf, chunk);
    (*chunk).last = nghttp3_frame_write_settings((*chunk).last, &raw mut fr, payloadlen);
    tbuf.buf.last = (*chunk).last;
    return nghttp3_stream_outq_add(stream, &raw mut tbuf);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_write_goaway(
    mut stream: *mut nghttp3_stream,
    mut fr: *const nghttp3_frame_goaway,
) -> ::core::ffi::c_int {
    let mut len: size_t = 0;
    let mut rv: ::core::ffi::c_int = 0;
    let mut chunk: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    let mut tbuf: nghttp3_typed_buf = nghttp3_typed_buf {
        buf: nghttp3_buf {
            begin: ::core::ptr::null_mut::<uint8_t>(),
            end: ::core::ptr::null_mut::<uint8_t>(),
            pos: ::core::ptr::null_mut::<uint8_t>(),
            last: ::core::ptr::null_mut::<uint8_t>(),
        },
        r#type: nghttp3_buf_type::NGHTTP3_BUF_TYPE_PRIVATE,
    };
    let mut payloadlen: uint64_t = 0;
    len = nghttp3_frame_write_goaway_len(&raw mut payloadlen, fr);
    rv = nghttp3_stream_ensure_chunk(stream, len);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    chunk = nghttp3_stream_get_chunk(stream);
    nghttp3_typed_buf_shared_init(&raw mut tbuf, chunk);
    (*chunk).last = nghttp3_frame_write_goaway((*chunk).last, fr, payloadlen);
    tbuf.buf.last = (*chunk).last;
    return nghttp3_stream_outq_add(stream, &raw mut tbuf);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_write_priority_update(
    mut stream: *mut nghttp3_stream,
    mut fr: *const nghttp3_frame_priority_update,
) -> ::core::ffi::c_int {
    let mut len: size_t = 0;
    let mut rv: ::core::ffi::c_int = 0;
    let mut chunk: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    let mut tbuf: nghttp3_typed_buf = nghttp3_typed_buf {
        buf: nghttp3_buf {
            begin: ::core::ptr::null_mut::<uint8_t>(),
            end: ::core::ptr::null_mut::<uint8_t>(),
            pos: ::core::ptr::null_mut::<uint8_t>(),
            last: ::core::ptr::null_mut::<uint8_t>(),
        },
        r#type: nghttp3_buf_type::NGHTTP3_BUF_TYPE_PRIVATE,
    };
    let mut payloadlen: uint64_t = 0;
    len = nghttp3_frame_write_priority_update_len(&raw mut payloadlen, fr);
    rv = nghttp3_stream_ensure_chunk(stream, len);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    chunk = nghttp3_stream_get_chunk(stream);
    nghttp3_typed_buf_shared_init(&raw mut tbuf, chunk);
    (*chunk).last = nghttp3_frame_write_priority_update((*chunk).last, fr, payloadlen);
    tbuf.buf.last = (*chunk).last;
    return nghttp3_stream_outq_add(stream, &raw mut tbuf);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_write_origin(
    mut stream: *mut nghttp3_stream,
    mut fr: *const nghttp3_frame_origin,
) -> ::core::ffi::c_int {
    let mut chunk: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    let mut buf: nghttp3_buf = nghttp3_buf {
        begin: ::core::ptr::null_mut::<uint8_t>(),
        end: ::core::ptr::null_mut::<uint8_t>(),
        pos: ::core::ptr::null_mut::<uint8_t>(),
        last: ::core::ptr::null_mut::<uint8_t>(),
    };
    let mut tbuf: nghttp3_typed_buf = nghttp3_typed_buf {
        buf: nghttp3_buf {
            begin: ::core::ptr::null_mut::<uint8_t>(),
            end: ::core::ptr::null_mut::<uint8_t>(),
            pos: ::core::ptr::null_mut::<uint8_t>(),
            last: ::core::ptr::null_mut::<uint8_t>(),
        },
        r#type: nghttp3_buf_type::NGHTTP3_BUF_TYPE_PRIVATE,
    };
    let mut rv: ::core::ffi::c_int = 0;
    rv = nghttp3_stream_ensure_chunk(
        stream,
        nghttp3_frame_write_hd_len((*fr).r#type, (*fr).origin_list.len as uint64_t),
    );
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    chunk = nghttp3_stream_get_chunk(stream);
    nghttp3_typed_buf_shared_init(&raw mut tbuf, chunk);
    (*chunk).last = nghttp3_frame_write_hd(
        (*chunk).last,
        (*fr).r#type,
        (*fr).origin_list.len as uint64_t,
    );
    tbuf.buf.last = (*chunk).last;
    rv = nghttp3_stream_outq_add(stream, &raw mut tbuf);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    if (*fr).origin_list.len == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    nghttp3_buf_wrap_init(&raw mut buf, (*fr).origin_list.base, (*fr).origin_list.len);
    buf.last = buf.end;
    nghttp3_typed_buf_init(
        &raw mut tbuf,
        &raw mut buf,
        nghttp3_buf_type::NGHTTP3_BUF_TYPE_ALIEN_NO_ACK,
    );
    return nghttp3_stream_outq_add(stream, &raw mut tbuf);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_write_headers(
    mut stream: *mut nghttp3_stream,
    mut fr: *const nghttp3_frame_headers,
) -> ::core::ffi::c_int {
    let mut conn: *mut nghttp3_conn = (*stream).c2rust_unnamed.c2rust_unnamed.conn;
    '_c2rust_label: {
        if !conn.is_null() {} else {
            __assert_fail(
                b"conn\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                485 as ::core::ffi::c_uint,
                b"int nghttp3_stream_write_headers(nghttp3_stream *, const nghttp3_frame_headers *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    return nghttp3_stream_write_header_block(
        stream,
        &raw mut (*conn).qenc,
        (*conn).tx.qenc,
        &raw mut (*conn).tx.qpack.rbuf,
        &raw mut (*conn).tx.qpack.ebuf,
        NGHTTP3_FRAME_HEADERS as uint64_t,
        (*fr).nva,
        (*fr).nvlen,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_write_header_block(
    mut stream: *mut nghttp3_stream,
    mut qenc: *mut nghttp3_qpack_encoder,
    mut qenc_stream: *mut nghttp3_stream,
    mut rbuf: *mut nghttp3_buf,
    mut ebuf: *mut nghttp3_buf,
    mut frame_type: uint64_t,
    mut nva: *const nghttp3_nv,
    mut nvlen: size_t,
) -> ::core::ffi::c_int {
    let mut pbuf: nghttp3_buf = nghttp3_buf {
        begin: ::core::ptr::null_mut::<uint8_t>(),
        end: ::core::ptr::null_mut::<uint8_t>(),
        pos: ::core::ptr::null_mut::<uint8_t>(),
        last: ::core::ptr::null_mut::<uint8_t>(),
    };
    let mut rv: ::core::ffi::c_int = 0;
    let mut len: size_t = 0;
    let mut chunk: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    let mut tbuf: nghttp3_typed_buf = nghttp3_typed_buf {
        buf: nghttp3_buf {
            begin: ::core::ptr::null_mut::<uint8_t>(),
            end: ::core::ptr::null_mut::<uint8_t>(),
            pos: ::core::ptr::null_mut::<uint8_t>(),
            last: ::core::ptr::null_mut::<uint8_t>(),
        },
        r#type: nghttp3_buf_type::NGHTTP3_BUF_TYPE_PRIVATE,
    };
    let mut raw_pbuf: [uint8_t; 16] = [0; 16];
    let mut pbuflen: size_t = 0;
    let mut rbuflen: size_t = 0;
    let mut ebuflen: size_t = 0;
    let mut payloadlen: uint64_t = 0;
    nghttp3_buf_wrap_init(
        &raw mut pbuf,
        &raw mut raw_pbuf as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 16]>(),
    );
    rv = nghttp3_qpack_encoder_encode(
        qenc,
        &raw mut pbuf,
        rbuf,
        ebuf,
        (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
        nva,
        nvlen,
    );
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    pbuflen = nghttp3_buf_len(&raw mut pbuf);
    rbuflen = nghttp3_buf_len(rbuf);
    ebuflen = nghttp3_buf_len(ebuf);
    payloadlen = pbuflen.wrapping_add(rbuflen) as uint64_t;
    len = nghttp3_frame_write_hd_len(frame_type, payloadlen).wrapping_add(pbuflen);
    if rbuflen <= NGHTTP3_STREAM_MAX_COPY_THRES as size_t {
        len = len.wrapping_add(rbuflen);
    }
    rv = nghttp3_stream_ensure_chunk(stream, len);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    chunk = nghttp3_stream_get_chunk(stream);
    nghttp3_typed_buf_shared_init(&raw mut tbuf, chunk);
    (*chunk).last = nghttp3_frame_write_hd((*chunk).last, frame_type, payloadlen);
    (*chunk).last = nghttp3_cpymem((*chunk).last, pbuf.pos, pbuflen);
    nghttp3_buf_init(&raw mut pbuf);
    if rbuflen > NGHTTP3_STREAM_MAX_COPY_THRES as size_t {
        tbuf.buf.last = (*chunk).last;
        rv = nghttp3_stream_outq_add(stream, &raw mut tbuf);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
        nghttp3_typed_buf_init(
            &raw mut tbuf,
            rbuf,
            nghttp3_buf_type::NGHTTP3_BUF_TYPE_PRIVATE,
        );
        rv = nghttp3_stream_outq_add(stream, &raw mut tbuf);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
        nghttp3_buf_init(rbuf);
    } else if rbuflen != 0 {
        (*chunk).last = nghttp3_cpymem((*chunk).last, (*rbuf).pos, rbuflen);
        tbuf.buf.last = (*chunk).last;
        rv = nghttp3_stream_outq_add(stream, &raw mut tbuf);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
        nghttp3_buf_reset(rbuf);
    }
    if ebuflen > NGHTTP3_STREAM_MAX_COPY_THRES as size_t {
        '_c2rust_label: {
            if !qenc_stream.is_null() {} else {
                __assert_fail(
                    b"qenc_stream\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    566 as ::core::ffi::c_uint,
                    b"int nghttp3_stream_write_header_block(nghttp3_stream *, nghttp3_qpack_encoder *, nghttp3_stream *, nghttp3_buf *, nghttp3_buf *, uint64_t, const nghttp3_nv *, size_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        nghttp3_typed_buf_init(
            &raw mut tbuf,
            ebuf,
            nghttp3_buf_type::NGHTTP3_BUF_TYPE_PRIVATE,
        );
        rv = nghttp3_stream_outq_add(qenc_stream, &raw mut tbuf);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
        nghttp3_buf_init(ebuf);
    } else if ebuflen != 0 {
        '_c2rust_label_0: {
            if !qenc_stream.is_null() {} else {
                __assert_fail(
                    b"qenc_stream\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    575 as ::core::ffi::c_uint,
                    b"int nghttp3_stream_write_header_block(nghttp3_stream *, nghttp3_qpack_encoder *, nghttp3_stream *, nghttp3_buf *, nghttp3_buf *, uint64_t, const nghttp3_nv *, size_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        rv = nghttp3_stream_ensure_chunk(qenc_stream, ebuflen);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
        chunk = nghttp3_stream_get_chunk(qenc_stream);
        nghttp3_typed_buf_shared_init(&raw mut tbuf, chunk);
        (*chunk).last = nghttp3_cpymem((*chunk).last, (*ebuf).pos, ebuflen);
        tbuf.buf.last = (*chunk).last;
        rv = nghttp3_stream_outq_add(qenc_stream, &raw mut tbuf);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
        nghttp3_buf_reset(ebuf);
    }
    '_c2rust_label_1: {
        if 0 as size_t == nghttp3_buf_len(&raw mut pbuf) {} else {
            __assert_fail(
                b"0 == nghttp3_buf_len(&pbuf)\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                595 as ::core::ffi::c_uint,
                b"int nghttp3_stream_write_header_block(nghttp3_stream *, nghttp3_qpack_encoder *, nghttp3_stream *, nghttp3_buf *, nghttp3_buf *, uint64_t, const nghttp3_nv *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if 0 as size_t == nghttp3_buf_len(rbuf) {} else {
            __assert_fail(
                b"0 == nghttp3_buf_len(rbuf)\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                596 as ::core::ffi::c_uint,
                b"int nghttp3_stream_write_header_block(nghttp3_stream *, nghttp3_qpack_encoder *, nghttp3_stream *, nghttp3_buf *, nghttp3_buf *, uint64_t, const nghttp3_nv *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if 0 as size_t == nghttp3_buf_len(ebuf) {} else {
            __assert_fail(
                b"0 == nghttp3_buf_len(ebuf)\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                597 as ::core::ffi::c_uint,
                b"int nghttp3_stream_write_header_block(nghttp3_stream *, nghttp3_qpack_encoder *, nghttp3_stream *, nghttp3_buf *, nghttp3_buf *, uint64_t, const nghttp3_nv *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_write_data(
    mut stream: *mut nghttp3_stream,
    mut peof: *mut ::core::ffi::c_int,
    mut fr: *const nghttp3_frame_data,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    let mut len: size_t = 0;
    let mut tbuf: nghttp3_typed_buf = nghttp3_typed_buf {
        buf: nghttp3_buf {
            begin: ::core::ptr::null_mut::<uint8_t>(),
            end: ::core::ptr::null_mut::<uint8_t>(),
            pos: ::core::ptr::null_mut::<uint8_t>(),
            last: ::core::ptr::null_mut::<uint8_t>(),
        },
        r#type: nghttp3_buf_type::NGHTTP3_BUF_TYPE_PRIVATE,
    };
    let mut buf: nghttp3_buf = nghttp3_buf {
        begin: ::core::ptr::null_mut::<uint8_t>(),
        end: ::core::ptr::null_mut::<uint8_t>(),
        pos: ::core::ptr::null_mut::<uint8_t>(),
        last: ::core::ptr::null_mut::<uint8_t>(),
    };
    let mut chunk: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    let mut read_data: nghttp3_read_data_callback = (*fr).dr.read_data;
    let mut conn: *mut nghttp3_conn = (*stream).c2rust_unnamed.c2rust_unnamed.conn;
    let mut datalen: uint64_t = 0;
    let mut flags: uint32_t = 0 as uint32_t;
    let mut vec: [nghttp3_vec; 8] = [nghttp3_vec {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    }; 8];
    let mut v: *mut nghttp3_vec = ::core::ptr::null_mut::<nghttp3_vec>();
    let mut sveccnt: nghttp3_ssize = 0;
    let mut i: size_t = 0;
    '_c2rust_label: {
        if (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
            & 0x4 as ::core::ffi::c_uint == 0
        {} else {
            __assert_fail(
                b"!(stream->flags & NGHTTP3_STREAM_FLAG_READ_DATA_BLOCKED)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                618 as ::core::ffi::c_uint,
                b"int nghttp3_stream_write_data(nghttp3_stream *, int *, const nghttp3_frame_data *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if read_data.is_some() {} else {
            __assert_fail(
                b"read_data\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                619 as ::core::ffi::c_uint,
                b"int nghttp3_stream_write_data(nghttp3_stream *, int *, const nghttp3_frame_data *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !conn.is_null() {} else {
            __assert_fail(
                b"conn\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                620 as ::core::ffi::c_uint,
                b"int nghttp3_stream_write_data(nghttp3_stream *, int *, const nghttp3_frame_data *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    *peof = 0 as ::core::ffi::c_int;
    sveccnt = read_data
        .expect(
            "non-null function pointer",
        )(
        conn,
        (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
        &raw mut vec as *mut nghttp3_vec,
        ::core::mem::size_of::<[nghttp3_vec; 8]>()
            .wrapping_div(::core::mem::size_of::<nghttp3_vec>()),
        &raw mut flags,
        (*conn).user_data,
        (*stream).c2rust_unnamed.c2rust_unnamed.user_data,
    );
    if sveccnt < 0 as nghttp3_ssize {
        if sveccnt == NGHTTP3_ERR_WOULDBLOCK as nghttp3_ssize {
            (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
                .c2rust_unnamed
                .c2rust_unnamed
                .flags as ::core::ffi::c_uint | NGHTTP3_STREAM_FLAG_READ_DATA_BLOCKED)
                as uint16_t;
            return 0 as ::core::ffi::c_int;
        }
        return NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    rv = nghttp3_vec_len_uvarint(
        &raw mut datalen,
        &raw mut vec as *mut nghttp3_vec,
        sveccnt as size_t,
    );
    if rv == -1 as ::core::ffi::c_int {
        return NGHTTP3_ERR_STREAM_DATA_OVERFLOW;
    }
    '_c2rust_label_2: {
        if datalen != 0 || flags & 0x1 as uint32_t != 0 {} else {
            __assert_fail(
                b"datalen || flags & NGHTTP3_DATA_FLAG_EOF\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                639 as ::core::ffi::c_uint,
                b"int nghttp3_stream_write_data(nghttp3_stream *, int *, const nghttp3_frame_data *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if flags & NGHTTP3_DATA_FLAG_EOF as uint32_t != 0 {
        *peof = 1 as ::core::ffi::c_int;
        if flags & NGHTTP3_DATA_FLAG_NO_END_STREAM as uint32_t == 0 {
            (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
                .c2rust_unnamed
                .c2rust_unnamed
                .flags as ::core::ffi::c_uint | NGHTTP3_STREAM_FLAG_WRITE_END_STREAM)
                as uint16_t;
            if datalen == 0 as uint64_t {
                if nghttp3_stream_outq_write_done(stream) != 0 {
                    nghttp3_buf_init(&raw mut buf);
                    nghttp3_typed_buf_init(
                        &raw mut tbuf,
                        &raw mut buf,
                        nghttp3_buf_type::NGHTTP3_BUF_TYPE_PRIVATE,
                    );
                    return nghttp3_stream_outq_add(stream, &raw mut tbuf);
                }
                return 0 as ::core::ffi::c_int;
            }
        }
        if datalen == 0 as uint64_t {
            return 0 as ::core::ffi::c_int;
        }
    }
    len = nghttp3_frame_write_hd_len(NGHTTP3_FRAME_DATA as uint64_t, datalen);
    rv = nghttp3_stream_ensure_chunk(stream, len);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    chunk = nghttp3_stream_get_chunk(stream);
    nghttp3_typed_buf_shared_init(&raw mut tbuf, chunk);
    (*chunk).last = nghttp3_frame_write_hd(
        (*chunk).last,
        NGHTTP3_FRAME_DATA as uint64_t,
        datalen,
    );
    tbuf.buf.last = (*chunk).last;
    rv = nghttp3_stream_outq_add(stream, &raw mut tbuf);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    '_c2rust_label_3: {
        if datalen != 0 {} else {
            __assert_fail(
                b"datalen\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                686 as ::core::ffi::c_uint,
                b"int nghttp3_stream_write_data(nghttp3_stream *, int *, const nghttp3_frame_data *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    i = 0 as size_t;
    while i < sveccnt as size_t {
        v = (&raw mut vec as *mut nghttp3_vec).offset(i as isize);
        if (*v).len != 0 as size_t {
            nghttp3_buf_wrap_init(&raw mut buf, (*v).base, (*v).len);
            buf.last = buf.end;
            nghttp3_typed_buf_init(
                &raw mut tbuf,
                &raw mut buf,
                nghttp3_buf_type::NGHTTP3_BUF_TYPE_ALIEN,
            );
            rv = nghttp3_stream_outq_add(stream, &raw mut tbuf);
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_write_qpack_decoder_stream(
    mut stream: *mut nghttp3_stream,
) -> ::core::ffi::c_int {
    let mut qdec: *mut nghttp3_qpack_decoder = ::core::ptr::null_mut::<
        nghttp3_qpack_decoder,
    >();
    let mut chunk: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    let mut rv: ::core::ffi::c_int = 0;
    let mut tbuf: nghttp3_typed_buf = nghttp3_typed_buf {
        buf: nghttp3_buf {
            begin: ::core::ptr::null_mut::<uint8_t>(),
            end: ::core::ptr::null_mut::<uint8_t>(),
            pos: ::core::ptr::null_mut::<uint8_t>(),
            last: ::core::ptr::null_mut::<uint8_t>(),
        },
        r#type: nghttp3_buf_type::NGHTTP3_BUF_TYPE_PRIVATE,
    };
    let mut len: size_t = 0;
    '_c2rust_label: {
        if !(*stream).c2rust_unnamed.c2rust_unnamed.conn.is_null() {} else {
            __assert_fail(
                b"stream->conn\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                712 as ::core::ffi::c_uint,
                b"int nghttp3_stream_write_qpack_decoder_stream(nghttp3_stream *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*(*stream).c2rust_unnamed.c2rust_unnamed.conn).tx.qdec == stream {} else {
            __assert_fail(
                b"stream->conn->tx.qdec == stream\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                713 as ::core::ffi::c_uint,
                b"int nghttp3_stream_write_qpack_decoder_stream(nghttp3_stream *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    qdec = &raw mut (*(*stream).c2rust_unnamed.c2rust_unnamed.conn).qdec;
    '_c2rust_label_1: {
        if !qdec.is_null() {} else {
            __assert_fail(
                b"qdec\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                717 as ::core::ffi::c_uint,
                b"int nghttp3_stream_write_qpack_decoder_stream(nghttp3_stream *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    len = nghttp3_qpack_decoder_get_decoder_streamlen2(qdec);
    if len == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    rv = nghttp3_stream_ensure_chunk(stream, len);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    chunk = nghttp3_stream_get_chunk(stream);
    nghttp3_typed_buf_shared_init(&raw mut tbuf, chunk);
    nghttp3_qpack_decoder_write_decoder(qdec, chunk);
    tbuf.buf.last = (*chunk).last;
    return nghttp3_stream_outq_add(stream, &raw mut tbuf);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_outq_add(
    mut stream: *mut nghttp3_stream,
    mut tbuf: *const nghttp3_typed_buf,
) -> ::core::ffi::c_int {
    let mut outq: *mut nghttp3_ringbuf = &raw mut (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .outq;
    let mut rv: ::core::ffi::c_int = 0;
    let mut dest: *mut nghttp3_typed_buf = ::core::ptr::null_mut::<nghttp3_typed_buf>();
    let mut len: size_t = nghttp3_ringbuf_len(outq);
    let mut buflen: size_t = nghttp3_buf_len(&raw const (*tbuf).buf);
    if buflen as ::core::ffi::c_ulonglong
        > NGHTTP3_MAX_VARINT
            .wrapping_sub(
                (*stream).c2rust_unnamed.c2rust_unnamed.tx.offset
                    as ::core::ffi::c_ulonglong,
            )
    {
        return NGHTTP3_ERR_STREAM_DATA_OVERFLOW;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.tx.offset = ((*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .tx
        .offset as ::core::ffi::c_ulong)
        .wrapping_add(buflen as ::core::ffi::c_ulong) as uint64_t;
    (*stream).c2rust_unnamed.c2rust_unnamed.unsent_bytes = ((*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .unsent_bytes as ::core::ffi::c_ulong)
        .wrapping_add(buflen as ::core::ffi::c_ulong) as uint64_t;
    if len != 0 {
        dest = nghttp3_ringbuf_get(outq, len.wrapping_sub(1 as size_t))
            as *mut nghttp3_typed_buf;
        if (*dest).r#type.0 == (*tbuf).r#type.0
            && (*dest).r#type.0 == nghttp3_buf_type::NGHTTP3_BUF_TYPE_SHARED.0
            && (*dest).buf.end == (*tbuf).buf.end && (*dest).buf.last == (*tbuf).buf.pos
        {
            if len == (*stream).c2rust_unnamed.c2rust_unnamed.outq_idx {
                (*stream).c2rust_unnamed.c2rust_unnamed.outq_idx = (*stream)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .outq_idx
                    .wrapping_sub(1);
            }
            (*dest).buf.last = (*tbuf).buf.last;
            '_c2rust_label: {
                if (*dest).buf.end == (*tbuf).buf.end {} else {
                    __assert_fail(
                        b"dest->buf.end == tbuf->buf.end\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        766 as ::core::ffi::c_uint,
                        b"int nghttp3_stream_outq_add(nghttp3_stream *, const nghttp3_typed_buf *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            return 0 as ::core::ffi::c_int;
        }
    }
    if nghttp3_ringbuf_full(outq) != 0 {
        let mut nlen: size_t = nghttp3_max_unsigned_long_int(
            4 as ::core::ffi::c_ulong,
            (len as ::core::ffi::c_ulong).wrapping_mul(2 as ::core::ffi::c_ulong),
        ) as size_t;
        rv = nghttp3_ringbuf_reserve(outq, nlen);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
    }
    dest = nghttp3_ringbuf_push_back(outq) as *mut nghttp3_typed_buf;
    *dest = *tbuf;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_ensure_chunk(
    mut stream: *mut nghttp3_stream,
    mut need: size_t,
) -> ::core::ffi::c_int {
    let mut chunks: *mut nghttp3_ringbuf = &raw mut (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .chunks;
    let mut chunk: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    let mut len: size_t = nghttp3_ringbuf_len(chunks);
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut rv: ::core::ffi::c_int = 0;
    let mut n: size_t = NGHTTP3_STREAM_MIN_CHUNK_SIZE as size_t;
    if len != 0 {
        chunk = nghttp3_ringbuf_get(chunks, len.wrapping_sub(1 as size_t))
            as *mut nghttp3_buf;
        if nghttp3_buf_left(chunk) >= need {
            return 0 as ::core::ffi::c_int;
        }
    }
    while n < need {
        n = n.wrapping_mul(2 as size_t);
    }
    if n == NGHTTP3_STREAM_MIN_CHUNK_SIZE as size_t {
        p = nghttp3_objalloc_chunk_len_get(
            (*stream).c2rust_unnamed.c2rust_unnamed.out_chunk_objalloc,
            n,
        ) as *mut uint8_t;
    } else {
        p = nghttp3_mem_malloc((*stream).c2rust_unnamed.c2rust_unnamed.mem, n)
            as *mut uint8_t;
    }
    if p.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    if nghttp3_ringbuf_full(chunks) != 0 {
        let mut nlen: size_t = nghttp3_max_unsigned_long_int(
            4 as ::core::ffi::c_ulong,
            (len as ::core::ffi::c_ulong).wrapping_mul(2 as ::core::ffi::c_ulong),
        ) as size_t;
        rv = nghttp3_ringbuf_reserve(chunks, nlen);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
    }
    chunk = nghttp3_ringbuf_push_back(chunks) as *mut nghttp3_buf;
    nghttp3_buf_wrap_init(chunk, p, n);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_get_chunk(
    mut stream: *mut nghttp3_stream,
) -> *mut nghttp3_buf {
    let mut chunks: *mut nghttp3_ringbuf = &raw mut (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .chunks;
    let mut len: size_t = nghttp3_ringbuf_len(chunks);
    '_c2rust_label: {
        if len != 0 {} else {
            __assert_fail(
                b"len\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                832 as ::core::ffi::c_uint,
                b"nghttp3_buf *nghttp3_stream_get_chunk(nghttp3_stream *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return nghttp3_ringbuf_get(chunks, len.wrapping_sub(1 as size_t))
        as *mut nghttp3_buf;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_is_blocked(
    mut stream: *const nghttp3_stream,
) -> ::core::ffi::c_int {
    return ((*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
        & NGHTTP3_STREAM_FLAG_FC_BLOCKED != 0
        || (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
            & NGHTTP3_STREAM_FLAG_SHUT_WR != 0
        || (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
            & NGHTTP3_STREAM_FLAG_READ_DATA_BLOCKED != 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_require_schedule(
    mut stream: *const nghttp3_stream,
) -> ::core::ffi::c_int {
    return (nghttp3_stream_outq_write_done(stream) == 0
        && (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
            & NGHTTP3_STREAM_FLAG_FC_BLOCKED == 0
        && (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
            & NGHTTP3_STREAM_FLAG_SHUT_WR == 0
        || nghttp3_ringbuf_len(&raw const (*stream).c2rust_unnamed.c2rust_unnamed.frq)
            != 0
            && (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
                & NGHTTP3_STREAM_FLAG_READ_DATA_BLOCKED == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_writev(
    mut stream: *mut nghttp3_stream,
    mut pfin: *mut ::core::ffi::c_int,
    mut vec: *mut nghttp3_vec,
    mut veccnt: size_t,
) -> size_t {
    let mut outq: *mut nghttp3_ringbuf = &raw mut (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .outq;
    let mut len: size_t = nghttp3_ringbuf_len(outq);
    let mut i: size_t = (*stream).c2rust_unnamed.c2rust_unnamed.outq_idx;
    let mut buflen: size_t = 0;
    let mut vbegin: *mut nghttp3_vec = vec;
    let mut vend: *mut nghttp3_vec = vec.offset(veccnt as isize);
    let mut tbuf: *mut nghttp3_typed_buf = ::core::ptr::null_mut::<nghttp3_typed_buf>();
    '_c2rust_label: {
        if veccnt > 0 as size_t {} else {
            __assert_fail(
                b"veccnt > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                860 as ::core::ffi::c_uint,
                b"size_t nghttp3_stream_writev(nghttp3_stream *, int *, nghttp3_vec *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    while i < len && vec != vend {
        tbuf = nghttp3_ringbuf_get(outq, i) as *mut nghttp3_typed_buf;
        buflen = nghttp3_buf_len(&raw mut (*tbuf).buf);
        if buflen != 0 as size_t {
            (*vec).base = (*tbuf).buf.pos;
            (*vec).len = buflen;
            vec = vec.offset(1);
        }
        i = i.wrapping_add(1);
    }
    *pfin = (nghttp3_ringbuf_len(&raw mut (*stream).c2rust_unnamed.c2rust_unnamed.frq)
        == 0 as size_t && i == len
        && (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
            & NGHTTP3_STREAM_FLAG_WRITE_END_STREAM != 0) as ::core::ffi::c_int;
    return vec.offset_from(vbegin) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_add_outq_offset(
    mut stream: *mut nghttp3_stream,
    mut n: size_t,
) {
    let mut outq: *mut nghttp3_ringbuf = &raw mut (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .outq;
    let mut i: size_t = 0;
    let mut len: size_t = nghttp3_ringbuf_len(outq);
    let mut buflen: size_t = 0;
    let mut tbuf: *mut nghttp3_typed_buf = ::core::ptr::null_mut::<nghttp3_typed_buf>();
    (*stream).c2rust_unnamed.c2rust_unnamed.unsent_bytes = ((*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .unsent_bytes as ::core::ffi::c_ulong)
        .wrapping_sub(n as ::core::ffi::c_ulong) as uint64_t;
    i = (*stream).c2rust_unnamed.c2rust_unnamed.outq_idx;
    while i < len {
        tbuf = nghttp3_ringbuf_get(outq, i) as *mut nghttp3_typed_buf;
        buflen = nghttp3_buf_len(&raw mut (*tbuf).buf);
        if n < buflen {
            (*tbuf).buf.pos = (*tbuf).buf.pos.offset(n as isize);
            break;
        } else {
            (*tbuf).buf.pos = (*tbuf).buf.last;
            n = n.wrapping_sub(buflen);
            i = i.wrapping_add(1);
        }
    }
    '_c2rust_label: {
        if i < len || n == 0 as size_t {} else {
            __assert_fail(
                b"i < len || n == 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                906 as ::core::ffi::c_uint,
                b"void nghttp3_stream_add_outq_offset(nghttp3_stream *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    (*stream).c2rust_unnamed.c2rust_unnamed.outq_idx = i;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_outq_write_done(
    mut stream: *const nghttp3_stream,
) -> ::core::ffi::c_int {
    let mut outq: *const nghttp3_ringbuf = &raw const (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .outq;
    let mut len: size_t = nghttp3_ringbuf_len(outq);
    return (len == 0 as size_t
        || (*stream).c2rust_unnamed.c2rust_unnamed.outq_idx >= len)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn stream_pop_outq_entry(
    mut stream: *mut nghttp3_stream,
    mut tbuf: *mut nghttp3_typed_buf,
) {
    let mut chunks: *mut nghttp3_ringbuf = &raw mut (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .chunks;
    let mut chunk: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    match (*tbuf).r#type {
        nghttp3_buf_type::NGHTTP3_BUF_TYPE_PRIVATE => {
            nghttp3_buf_free(
                &raw mut (*tbuf).buf,
                (*stream).c2rust_unnamed.c2rust_unnamed.mem,
            );
        }
        nghttp3_buf_type::NGHTTP3_BUF_TYPE_ALIEN
        | nghttp3_buf_type::NGHTTP3_BUF_TYPE_ALIEN_NO_ACK => {}
        nghttp3_buf_type::NGHTTP3_BUF_TYPE_SHARED => {
            '_c2rust_label: {
                if nghttp3_ringbuf_len(chunks) != 0 {} else {
                    __assert_fail(
                        b"nghttp3_ringbuf_len(chunks)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        931 as ::core::ffi::c_uint,
                        b"void stream_pop_outq_entry(nghttp3_stream *, nghttp3_typed_buf *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            chunk = nghttp3_ringbuf_get(chunks, 0 as size_t) as *mut nghttp3_buf;
            '_c2rust_label_0: {
                if (*chunk).end == (*tbuf).buf.end {} else {
                    __assert_fail(
                        b"chunk->end == tbuf->buf.end\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        935 as ::core::ffi::c_uint,
                        b"void stream_pop_outq_entry(nghttp3_stream *, nghttp3_typed_buf *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            if (*chunk).last == (*tbuf).buf.last {
                if nghttp3_buf_cap(chunk) == NGHTTP3_STREAM_MIN_CHUNK_SIZE as size_t {
                    nghttp3_objalloc_chunk_release(
                        (*stream).c2rust_unnamed.c2rust_unnamed.out_chunk_objalloc,
                        (*chunk).begin as *mut ::core::ffi::c_void as *mut nghttp3_chunk,
                    );
                } else {
                    nghttp3_buf_free(chunk, (*stream).c2rust_unnamed.c2rust_unnamed.mem);
                }
                nghttp3_ringbuf_pop_front(chunks);
            }
        }
        _ => {
            nghttp3_unreachable_fail(
                b"nghttp3_stream.c\0".as_ptr() as *const ::core::ffi::c_char,
                948 as ::core::ffi::c_int,
                b"stream_pop_outq_entry\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    nghttp3_ringbuf_pop_front(&raw mut (*stream).c2rust_unnamed.c2rust_unnamed.outq);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_update_ack_offset(
    mut stream: *mut nghttp3_stream,
    mut offset: uint64_t,
) -> ::core::ffi::c_int {
    let mut outq: *mut nghttp3_ringbuf = &raw mut (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .outq;
    let mut buflen: size_t = 0;
    let mut nack: uint64_t = 0;
    let mut tbuf: *mut nghttp3_typed_buf = ::core::ptr::null_mut::<nghttp3_typed_buf>();
    let mut rv: ::core::ffi::c_int = 0;
    while nghttp3_ringbuf_len(outq) != 0 {
        tbuf = nghttp3_ringbuf_get(outq, 0 as size_t) as *mut nghttp3_typed_buf;
        buflen = (*tbuf).buf.last.offset_from((*tbuf).buf.begin) as size_t;
        if (*tbuf).r#type.0 == nghttp3_buf_type::NGHTTP3_BUF_TYPE_ALIEN.0
            && (*stream).c2rust_unnamed.c2rust_unnamed.ack_offset < offset
            && (*stream).c2rust_unnamed.c2rust_unnamed.callbacks.acked_data.is_some()
        {
            nack = (nghttp3_min_unsigned_long_int(
                offset as ::core::ffi::c_ulong,
                ((*stream).c2rust_unnamed.c2rust_unnamed.ack_base
                    as ::core::ffi::c_ulong)
                    .wrapping_add(buflen as ::core::ffi::c_ulong),
            ) as uint64_t)
                .wrapping_sub((*stream).c2rust_unnamed.c2rust_unnamed.ack_offset);
            rv = (*stream)
                .c2rust_unnamed
                .c2rust_unnamed
                .callbacks
                .acked_data
                .expect(
                    "non-null function pointer",
                )(
                stream,
                (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
                nack,
                (*stream).c2rust_unnamed.c2rust_unnamed.user_data,
            );
            if rv != 0 as ::core::ffi::c_int {
                return NGHTTP3_ERR_CALLBACK_FAILURE;
            }
        }
        if !((*stream).c2rust_unnamed.c2rust_unnamed.outq_idx > 0 as size_t
            && offset
                >= (*stream)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .ack_base
                    .wrapping_add(buflen as uint64_t))
        {
            break;
        }
        stream_pop_outq_entry(stream, tbuf);
        (*stream).c2rust_unnamed.c2rust_unnamed.ack_base = ((*stream)
            .c2rust_unnamed
            .c2rust_unnamed
            .ack_base as ::core::ffi::c_ulong)
            .wrapping_add(buflen as ::core::ffi::c_ulong) as uint64_t;
        (*stream).c2rust_unnamed.c2rust_unnamed.ack_offset = (*stream)
            .c2rust_unnamed
            .c2rust_unnamed
            .ack_base;
        (*stream).c2rust_unnamed.c2rust_unnamed.outq_idx = (*stream)
            .c2rust_unnamed
            .c2rust_unnamed
            .outq_idx
            .wrapping_sub(1);
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.ack_offset = offset;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_buffer_data(
    mut stream: *mut nghttp3_stream,
    mut data: *const uint8_t,
    mut datalen: size_t,
) -> ::core::ffi::c_int {
    let mut inq: *mut nghttp3_ringbuf = &raw mut (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .inq;
    let mut len: size_t = nghttp3_ringbuf_len(inq);
    let mut buf: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    let mut nwrite: size_t = 0;
    let mut rawbuf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut bufleft: size_t = 0;
    let mut rv: ::core::ffi::c_int = 0;
    if len != 0 {
        buf = nghttp3_ringbuf_get(inq, len.wrapping_sub(1 as size_t))
            as *mut nghttp3_buf;
        bufleft = nghttp3_buf_left(buf);
        nwrite = nghttp3_min_unsigned_long_int(
            datalen as ::core::ffi::c_ulong,
            bufleft as ::core::ffi::c_ulong,
        ) as size_t;
        (*buf).last = nghttp3_cpymem((*buf).last, data, nwrite);
        data = data.offset(nwrite as isize);
        datalen = datalen.wrapping_sub(nwrite);
    }
    while datalen != 0 {
        if nghttp3_ringbuf_full(inq) != 0 {
            let mut nlen: size_t = nghttp3_max_unsigned_long_int(
                4 as ::core::ffi::c_ulong,
                (nghttp3_ringbuf_len(inq) as ::core::ffi::c_ulong)
                    .wrapping_mul(2 as ::core::ffi::c_ulong),
            ) as size_t;
            rv = nghttp3_ringbuf_reserve(inq, nlen);
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
        }
        rawbuf = nghttp3_mem_malloc(
            (*stream).c2rust_unnamed.c2rust_unnamed.mem,
            16384 as size_t,
        ) as *mut uint8_t;
        if rawbuf.is_null() {
            return NGHTTP3_ERR_NOMEM;
        }
        buf = nghttp3_ringbuf_push_back(inq) as *mut nghttp3_buf;
        nghttp3_buf_wrap_init(buf, rawbuf, 16384 as size_t);
        bufleft = nghttp3_buf_left(buf);
        nwrite = nghttp3_min_unsigned_long_int(
            datalen as ::core::ffi::c_ulong,
            bufleft as ::core::ffi::c_ulong,
        ) as size_t;
        (*buf).last = nghttp3_cpymem((*buf).last, data, nwrite);
        data = data.offset(nwrite as isize);
        datalen = datalen.wrapping_sub(nwrite);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_get_buffered_datalen(
    mut stream: *mut nghttp3_stream,
) -> size_t {
    let mut inq: *mut nghttp3_ringbuf = &raw mut (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .inq;
    let mut len: size_t = nghttp3_ringbuf_len(inq);
    let mut i: size_t = 0;
    let mut n: size_t = 0 as size_t;
    let mut buf: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    i = 0 as size_t;
    while i < len {
        buf = nghttp3_ringbuf_get(inq, i) as *mut nghttp3_buf;
        n = n.wrapping_add(nghttp3_buf_len(buf));
        i = i.wrapping_add(1);
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_transit_rx_http_state(
    mut stream: *mut nghttp3_stream,
    mut event: nghttp3_stream_http_event,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    match (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate {
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_NONE => {
            nghttp3_unreachable_fail(
                b"nghttp3_stream.c\0".as_ptr() as *const ::core::ffi::c_char,
                1063 as ::core::ffi::c_int,
                b"nghttp3_stream_transit_rx_http_state\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_INITIAL => {
            if event.0 != nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_HEADERS_BEGIN.0 {
                return NGHTTP3_ERR_H3_FRAME_UNEXPECTED;
            }
            (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_HEADERS_BEGIN;
            return 0 as ::core::ffi::c_int;
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_HEADERS_BEGIN => {
            '_c2rust_label: {
                if nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_HEADERS_END.0 == event.0
                {} else {
                    __assert_fail(
                        b"NGHTTP3_HTTP_EVENT_HEADERS_END == event\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1073 as ::core::ffi::c_uint,
                        b"int nghttp3_stream_transit_rx_http_state(nghttp3_stream *, nghttp3_stream_http_event)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_HEADERS_END;
            return 0 as ::core::ffi::c_int;
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_HEADERS_END => {
            match event {
                nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_HEADERS_BEGIN => {
                    if (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.flags
                        & NGHTTP3_HTTP_FLAG_METH_CONNECT as uint32_t != 0
                    {
                        return NGHTTP3_ERR_H3_FRAME_UNEXPECTED;
                    }
                    (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_TRAILERS_BEGIN;
                    return 0 as ::core::ffi::c_int;
                }
                nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_DATA_BEGIN => {
                    (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_DATA_BEGIN;
                    return 0 as ::core::ffi::c_int;
                }
                nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_MSG_END => {
                    rv = nghttp3_http_on_remote_end_stream(stream);
                    if rv != 0 as ::core::ffi::c_int {
                        return rv;
                    }
                    (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_END;
                    return 0 as ::core::ffi::c_int;
                }
                _ => {
                    nghttp3_unreachable_fail(
                        b"nghttp3_stream.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1096 as ::core::ffi::c_int,
                        b"nghttp3_stream_transit_rx_http_state\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            }
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_DATA_BEGIN => {
            '_c2rust_label_0: {
                if nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_DATA_END.0 == event.0
                {} else {
                    __assert_fail(
                        b"NGHTTP3_HTTP_EVENT_DATA_END == event\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1099 as ::core::ffi::c_uint,
                        b"int nghttp3_stream_transit_rx_http_state(nghttp3_stream *, nghttp3_stream_http_event)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_DATA_END;
            return 0 as ::core::ffi::c_int;
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_DATA_END => {
            match event {
                nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_DATA_BEGIN => {
                    (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_DATA_BEGIN;
                    return 0 as ::core::ffi::c_int;
                }
                nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_HEADERS_BEGIN => {
                    if (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.flags
                        & NGHTTP3_HTTP_FLAG_METH_CONNECT as uint32_t != 0
                    {
                        return NGHTTP3_ERR_H3_FRAME_UNEXPECTED;
                    }
                    (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_TRAILERS_BEGIN;
                    return 0 as ::core::ffi::c_int;
                }
                nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_MSG_END => {
                    rv = nghttp3_http_on_remote_end_stream(stream);
                    if rv != 0 as ::core::ffi::c_int {
                        return rv;
                    }
                    (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_END;
                    return 0 as ::core::ffi::c_int;
                }
                _ => {
                    nghttp3_unreachable_fail(
                        b"nghttp3_stream.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1122 as ::core::ffi::c_int,
                        b"nghttp3_stream_transit_rx_http_state\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            }
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_TRAILERS_BEGIN => {
            '_c2rust_label_1: {
                if nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_HEADERS_END.0 == event.0
                {} else {
                    __assert_fail(
                        b"NGHTTP3_HTTP_EVENT_HEADERS_END == event\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1125 as ::core::ffi::c_uint,
                        b"int nghttp3_stream_transit_rx_http_state(nghttp3_stream *, nghttp3_stream_http_event)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_TRAILERS_END;
            return 0 as ::core::ffi::c_int;
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_TRAILERS_END => {
            if event.0 != nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_MSG_END.0 {
                return NGHTTP3_ERR_H3_FRAME_UNEXPECTED;
            }
            rv = nghttp3_http_on_remote_end_stream(stream);
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
            (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_END;
            return 0 as ::core::ffi::c_int;
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_END => {
            return NGHTTP3_ERR_H3_FRAME_UNEXPECTED;
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_INITIAL => {
            if event.0 != nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_HEADERS_BEGIN.0 {
                return NGHTTP3_ERR_H3_FRAME_UNEXPECTED;
            }
            (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_HEADERS_BEGIN;
            return 0 as ::core::ffi::c_int;
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_HEADERS_BEGIN => {
            '_c2rust_label_2: {
                if nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_HEADERS_END.0 == event.0
                {} else {
                    __assert_fail(
                        b"NGHTTP3_HTTP_EVENT_HEADERS_END == event\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1149 as ::core::ffi::c_uint,
                        b"int nghttp3_stream_transit_rx_http_state(nghttp3_stream *, nghttp3_stream_http_event)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_HEADERS_END;
            return 0 as ::core::ffi::c_int;
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_HEADERS_END => {
            match event {
                nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_HEADERS_BEGIN => {
                    if (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.status_code
                        == -1 as int32_t
                    {
                        (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_HEADERS_BEGIN;
                        return 0 as ::core::ffi::c_int;
                    }
                    if (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.flags
                        & NGHTTP3_HTTP_FLAG_METH_CONNECT as uint32_t != 0
                        && (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.status_code
                            / 100 as int32_t == 2 as int32_t
                    {
                        return NGHTTP3_ERR_H3_FRAME_UNEXPECTED;
                    }
                    (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_TRAILERS_BEGIN;
                    return 0 as ::core::ffi::c_int;
                }
                nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_DATA_BEGIN => {
                    if (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.flags
                        & NGHTTP3_HTTP_FLAG_EXPECT_FINAL_RESPONSE as uint32_t != 0
                    {
                        return NGHTTP3_ERR_H3_FRAME_UNEXPECTED;
                    }
                    (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_DATA_BEGIN;
                    return 0 as ::core::ffi::c_int;
                }
                nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_MSG_END => {
                    rv = nghttp3_http_on_remote_end_stream(stream);
                    if rv != 0 as ::core::ffi::c_int {
                        return rv;
                    }
                    (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_END;
                    return 0 as ::core::ffi::c_int;
                }
                _ => {
                    nghttp3_unreachable_fail(
                        b"nghttp3_stream.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1179 as ::core::ffi::c_int,
                        b"nghttp3_stream_transit_rx_http_state\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            }
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_DATA_BEGIN => {
            '_c2rust_label_3: {
                if nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_DATA_END.0 == event.0
                {} else {
                    __assert_fail(
                        b"NGHTTP3_HTTP_EVENT_DATA_END == event\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1182 as ::core::ffi::c_uint,
                        b"int nghttp3_stream_transit_rx_http_state(nghttp3_stream *, nghttp3_stream_http_event)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_DATA_END;
            return 0 as ::core::ffi::c_int;
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_DATA_END => {
            match event {
                nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_DATA_BEGIN => {
                    (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_DATA_BEGIN;
                    return 0 as ::core::ffi::c_int;
                }
                nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_HEADERS_BEGIN => {
                    if (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.flags
                        & NGHTTP3_HTTP_FLAG_METH_CONNECT as uint32_t != 0
                        && (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.status_code
                            / 100 as int32_t == 2 as int32_t
                    {
                        return NGHTTP3_ERR_H3_FRAME_UNEXPECTED;
                    }
                    (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_TRAILERS_BEGIN;
                    return 0 as ::core::ffi::c_int;
                }
                nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_MSG_END => {
                    rv = nghttp3_http_on_remote_end_stream(stream);
                    if rv != 0 as ::core::ffi::c_int {
                        return rv;
                    }
                    (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_END;
                    return 0 as ::core::ffi::c_int;
                }
                _ => {
                    nghttp3_unreachable_fail(
                        b"nghttp3_stream.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1205 as ::core::ffi::c_int,
                        b"nghttp3_stream_transit_rx_http_state\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            }
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_TRAILERS_BEGIN => {
            '_c2rust_label_4: {
                if nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_HEADERS_END.0 == event.0
                {} else {
                    __assert_fail(
                        b"NGHTTP3_HTTP_EVENT_HEADERS_END == event\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_stream.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1208 as ::core::ffi::c_uint,
                        b"int nghttp3_stream_transit_rx_http_state(nghttp3_stream *, nghttp3_stream_http_event)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_TRAILERS_END;
            return 0 as ::core::ffi::c_int;
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_TRAILERS_END => {
            if event.0 != nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_MSG_END.0 {
                return NGHTTP3_ERR_H3_FRAME_UNEXPECTED;
            }
            rv = nghttp3_http_on_remote_end_stream(stream);
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
            (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_END;
            return 0 as ::core::ffi::c_int;
        }
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_END => {
            return NGHTTP3_ERR_H3_FRAME_UNEXPECTED;
        }
        _ => {
            nghttp3_unreachable_fail(
                b"nghttp3_stream.c\0".as_ptr() as *const ::core::ffi::c_char,
                1224 as ::core::ffi::c_int,
                b"nghttp3_stream_transit_rx_http_state\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_empty_headers_allowed(
    mut stream: *const nghttp3_stream,
) -> ::core::ffi::c_int {
    match (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate {
        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_TRAILERS_BEGIN
        | nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_TRAILERS_BEGIN => {
            return 0 as ::core::ffi::c_int;
        }
        _ => return NGHTTP3_ERR_MALFORMED_HTTP_MESSAGING,
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_stream_uni(
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    return (stream_id & 0x2 as int64_t != 0 as int64_t) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_client_stream_bidi(
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    return (stream_id & 0x3 as int64_t == 0 as int64_t) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_client_stream_uni(
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    return (stream_id & 0x3 as int64_t == 0x2 as int64_t) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_server_stream_uni(
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    return (stream_id & 0x3 as int64_t == 0x3 as int64_t) as ::core::ffi::c_int;
}
