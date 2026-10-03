extern "C" {
    fn nghttp3_mem_default() -> *const nghttp3_mem;
    fn nghttp3_rcbuf_decref(rcbuf: *mut nghttp3_rcbuf);
    fn nghttp3_buf_free(buf: *mut nghttp3_buf, mem: *const nghttp3_mem);
    fn nghttp3_buf_len(buf: *const nghttp3_buf) -> size_t;
    fn nghttp3_qpack_encoder_read_decoder(
        encoder: *mut nghttp3_qpack_encoder,
        src: *const uint8_t,
        srclen: size_t,
    ) -> nghttp3_ssize;
    fn nghttp3_qpack_encoder_set_max_dtable_capacity(
        encoder: *mut nghttp3_qpack_encoder,
        max_dtable_capacity: size_t,
    );
    fn nghttp3_qpack_encoder_set_max_blocked_streams(
        encoder: *mut nghttp3_qpack_encoder,
        max_blocked_streams: size_t,
    );
    fn nghttp3_qpack_encoder_set_indexing_strat(
        encoder: *mut nghttp3_qpack_encoder,
        strat: nghttp3_qpack_indexing_strat,
    );
    fn nghttp3_qpack_stream_context_get_ricnt2(
        sctx: *const nghttp3_qpack_stream_context,
    ) -> uint64_t;
    fn nghttp3_qpack_stream_context_reset(sctx: *mut nghttp3_qpack_stream_context);
    fn nghttp3_qpack_decoder_read_encoder(
        decoder: *mut nghttp3_qpack_decoder,
        src: *const uint8_t,
        srclen: size_t,
    ) -> nghttp3_ssize;
    fn nghttp3_qpack_decoder_get_icnt(decoder: *const nghttp3_qpack_decoder) -> uint64_t;
    fn nghttp3_qpack_decoder_read_request(
        decoder: *mut nghttp3_qpack_decoder,
        sctx: *mut nghttp3_qpack_stream_context,
        nv: *mut nghttp3_qpack_nv,
        pflags: *mut uint8_t,
        src: *const uint8_t,
        srclen: size_t,
        fin: ::core::ffi::c_int,
    ) -> nghttp3_ssize;
    fn nghttp3_qpack_decoder_cancel_stream(
        decoder: *mut nghttp3_qpack_decoder,
        stream_id: int64_t,
    ) -> ::core::ffi::c_int;
    fn nghttp3_qpack_decoder_set_max_concurrent_streams(
        decoder: *mut nghttp3_qpack_decoder,
        max_concurrent_streams: size_t,
    );
    fn nghttp3_err_is_fatal(liberr: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn nghttp3_mem_malloc(
        mem: *const nghttp3_mem,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn nghttp3_mem_free(mem: *const nghttp3_mem, ptr: *mut ::core::ffi::c_void);
    fn nghttp3_mem_calloc(
        mem: *const nghttp3_mem,
        nmemb: size_t,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn nghttp3_map_init(map: *mut nghttp3_map, seed: uint64_t, mem: *const nghttp3_mem);
    fn nghttp3_map_free(map: *mut nghttp3_map);
    fn nghttp3_map_insert(
        map: *mut nghttp3_map,
        key: nghttp3_map_key_type,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn nghttp3_map_find(
        map: *const nghttp3_map,
        key: nghttp3_map_key_type,
    ) -> *mut ::core::ffi::c_void;
    fn nghttp3_map_remove(
        map: *mut nghttp3_map,
        key: nghttp3_map_key_type,
    ) -> ::core::ffi::c_int;
    fn nghttp3_map_each(
        map: *const nghttp3_map,
        func: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        ptr: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn nghttp3_pq_init(
        pq: *mut nghttp3_pq,
        less: nghttp3_pq_less,
        mem: *const nghttp3_mem,
    );
    fn nghttp3_pq_free(pq: *mut nghttp3_pq);
    fn nghttp3_pq_push(
        pq: *mut nghttp3_pq,
        item: *mut nghttp3_pq_entry,
    ) -> ::core::ffi::c_int;
    fn nghttp3_pq_top(pq: *const nghttp3_pq) -> *mut nghttp3_pq_entry;
    fn nghttp3_pq_pop(pq: *mut nghttp3_pq);
    fn nghttp3_pq_empty(pq: *const nghttp3_pq) -> ::core::ffi::c_int;
    fn nghttp3_pq_size(pq: *const nghttp3_pq) -> size_t;
    fn nghttp3_pq_remove(pq: *mut nghttp3_pq, item: *mut nghttp3_pq_entry);
    fn nghttp3_tnode_unschedule(tnode: *mut nghttp3_tnode, pq: *mut nghttp3_pq);
    fn nghttp3_tnode_schedule(
        tnode: *mut nghttp3_tnode,
        pq: *mut nghttp3_pq,
        nwrite: uint64_t,
    ) -> ::core::ffi::c_int;
    fn nghttp3_tnode_is_scheduled(tnode: *const nghttp3_tnode) -> ::core::ffi::c_int;
    fn nghttp3_ringbuf_pop_front(rb: *mut nghttp3_ringbuf);
    fn nghttp3_ringbuf_get(
        rb: *mut nghttp3_ringbuf,
        offset: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn nghttp3_buf_wrap_init(buf: *mut nghttp3_buf, src: *mut uint8_t, len: size_t);
    fn nghttp3_nva_copy(
        pnva: *mut *mut nghttp3_nv,
        nva: *const nghttp3_nv,
        nvlen: size_t,
        mem: *const nghttp3_mem,
    ) -> ::core::ffi::c_int;
    fn nghttp3_nva_del(nva: *mut nghttp3_nv, mem: *const nghttp3_mem);
    fn nghttp3_balloc_get(
        balloc: *mut nghttp3_balloc,
        pbuf: *mut *mut ::core::ffi::c_void,
        n: size_t,
    ) -> ::core::ffi::c_int;
    fn nghttp3_opl_pop(opl: *mut nghttp3_opl) -> *mut nghttp3_opl_entry;
    fn nghttp3_objalloc_init(
        objalloc: *mut nghttp3_objalloc,
        blklen: size_t,
        mem: *const nghttp3_mem,
    );
    fn nghttp3_objalloc_free(objalloc: *mut nghttp3_objalloc);
    fn nghttp3_ksl_len(ksl: *const nghttp3_ksl) -> size_t;
    fn nghttp3_qpack_encoder_init(
        encoder: *mut nghttp3_qpack_encoder,
        hard_max_dtable_capacity: size_t,
        seed: uint64_t,
        mem: *const nghttp3_mem,
    );
    fn nghttp3_qpack_encoder_free(encoder: *mut nghttp3_qpack_encoder);
    fn nghttp3_qpack_decoder_init(
        decoder: *mut nghttp3_qpack_decoder,
        hard_max_dtable_capacity: size_t,
        max_blocked_streams: size_t,
        mem: *const nghttp3_mem,
    );
    fn nghttp3_qpack_decoder_free(decoder: *mut nghttp3_qpack_decoder);
    fn nghttp3_stream_new(
        pstream: *mut *mut nghttp3_stream,
        stream_id: int64_t,
        callbacks: *const nghttp3_stream_callbacks,
        out_chunk_objalloc: *mut nghttp3_objalloc,
        stream_objalloc: *mut nghttp3_objalloc,
        mem: *const nghttp3_mem,
    ) -> ::core::ffi::c_int;
    fn nghttp3_stream_del(stream: *mut nghttp3_stream);
    fn nghttp3_varint_read_state_reset(rvint: *mut nghttp3_varint_read_state);
    fn nghttp3_stream_read_state_reset(rstate: *mut nghttp3_stream_read_state);
    fn nghttp3_read_varint(
        rvint: *mut nghttp3_varint_read_state,
        begin: *const uint8_t,
        end: *const uint8_t,
        fin: ::core::ffi::c_int,
    ) -> nghttp3_ssize;
    fn nghttp3_stream_frq_emplace(
        stream: *mut nghttp3_stream,
        pfr: *mut *mut nghttp3_frame,
    ) -> ::core::ffi::c_int;
    fn nghttp3_stream_fill_outq(stream: *mut nghttp3_stream) -> ::core::ffi::c_int;
    fn nghttp3_stream_write_stream_type(
        stream: *mut nghttp3_stream,
    ) -> ::core::ffi::c_int;
    fn nghttp3_stream_writev(
        stream: *mut nghttp3_stream,
        pfin: *mut ::core::ffi::c_int,
        vec: *mut nghttp3_vec,
        veccnt: size_t,
    ) -> size_t;
    fn nghttp3_stream_write_qpack_decoder_stream(
        stream: *mut nghttp3_stream,
    ) -> ::core::ffi::c_int;
    fn nghttp3_stream_is_blocked(stream: *const nghttp3_stream) -> ::core::ffi::c_int;
    fn nghttp3_stream_add_outq_offset(stream: *mut nghttp3_stream, n: size_t);
    fn nghttp3_stream_outq_write_done(
        stream: *const nghttp3_stream,
    ) -> ::core::ffi::c_int;
    fn nghttp3_stream_update_ack_offset(
        stream: *mut nghttp3_stream,
        offset: uint64_t,
    ) -> ::core::ffi::c_int;
    fn nghttp3_stream_require_schedule(
        stream: *const nghttp3_stream,
    ) -> ::core::ffi::c_int;
    fn nghttp3_stream_buffer_data(
        stream: *mut nghttp3_stream,
        src: *const uint8_t,
        srclen: size_t,
    ) -> ::core::ffi::c_int;
    fn nghttp3_stream_get_buffered_datalen(stream: *mut nghttp3_stream) -> size_t;
    fn nghttp3_stream_transit_rx_http_state(
        stream: *mut nghttp3_stream,
        event: nghttp3_stream_http_event,
    ) -> ::core::ffi::c_int;
    fn nghttp3_stream_empty_headers_allowed(
        stream: *const nghttp3_stream,
    ) -> ::core::ffi::c_int;
    fn nghttp3_stream_uni(stream_id: int64_t) -> ::core::ffi::c_int;
    fn nghttp3_client_stream_bidi(stream_id: int64_t) -> ::core::ffi::c_int;
    fn nghttp3_client_stream_uni(stream_id: int64_t) -> ::core::ffi::c_int;
    fn nghttp3_server_stream_uni(stream_id: int64_t) -> ::core::ffi::c_int;
    fn nghttp3_gaptr_drop_first_gap(gaptr: *mut nghttp3_gaptr);
    fn nghttp3_idtr_init(idtr: *mut nghttp3_idtr, mem: *const nghttp3_mem);
    fn nghttp3_idtr_free(idtr: *mut nghttp3_idtr);
    fn nghttp3_idtr_open(
        idtr: *mut nghttp3_idtr,
        stream_id: int64_t,
    ) -> ::core::ffi::c_int;
    fn nghttp3_ratelim_init(
        rlim: *mut nghttp3_ratelim,
        burst: uint64_t,
        rate: uint64_t,
        ts: nghttp3_tstamp,
    );
    fn nghttp3_ratelim_drain(
        rlim: *mut nghttp3_ratelim,
        n: uint64_t,
        ts: nghttp3_tstamp,
    ) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn nghttp3_ord_stream_id(stream_id: int64_t) -> uint64_t;
    fn nghttp3_http_on_header(
        http: *mut nghttp3_http_state,
        nv: *const nghttp3_qpack_nv,
        request: ::core::ffi::c_int,
        trailers: ::core::ffi::c_int,
        connect_protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn nghttp3_http_on_request_headers(
        http: *mut nghttp3_http_state,
    ) -> ::core::ffi::c_int;
    fn nghttp3_http_on_response_headers(
        http: *mut nghttp3_http_state,
    ) -> ::core::ffi::c_int;
    fn nghttp3_http_on_data_chunk(
        stream: *mut nghttp3_stream,
        n: size_t,
    ) -> ::core::ffi::c_int;
    fn nghttp3_http_record_request_method(
        stream: *mut nghttp3_stream,
        nva: *const nghttp3_nv,
        nvlen: size_t,
    );
    fn nghttp3_http_parse_priority(
        dest: *mut nghttp3_pri,
        value: *const uint8_t,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn nghttp3_pri_eq(
        a: *const nghttp3_pri,
        b: *const nghttp3_pri,
    ) -> ::core::ffi::c_int;
    fn nghttp3_unreachable_fail(
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
    ) -> !;
    fn nghttp3_settings_convert_to_latest(
        dest: *mut nghttp3_settings,
        settings_version: ::core::ffi::c_int,
        src: *const nghttp3_settings,
    ) -> *const nghttp3_settings;
    fn nghttp3_callbacks_convert_to_latest(
        dest: *mut nghttp3_callbacks,
        callbacks_version: ::core::ffi::c_int,
        src: *const nghttp3_callbacks,
    ) -> *const nghttp3_callbacks;
}
pub type size_t = usize;
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
pub struct nghttp3_stream_http_event(pub ::core::ffi::c_uint);
impl nghttp3_stream_http_event {
    pub const NGHTTP3_HTTP_EVENT_DATA_BEGIN: Self = Self(0);
    pub const NGHTTP3_HTTP_EVENT_DATA_END: Self = Self(1);
    pub const NGHTTP3_HTTP_EVENT_HEADERS_BEGIN: Self = Self(2);
    pub const NGHTTP3_HTTP_EVENT_HEADERS_END: Self = Self(3);
    pub const NGHTTP3_HTTP_EVENT_MSG_END: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nghttp3_ctrl_stream_state(pub ::core::ffi::c_uint);
impl nghttp3_ctrl_stream_state {
    pub const NGHTTP3_CTRL_STREAM_STATE_FRAME_TYPE: Self = Self(0);
    pub const NGHTTP3_CTRL_STREAM_STATE_FRAME_LENGTH: Self = Self(1);
    pub const NGHTTP3_CTRL_STREAM_STATE_SETTINGS: Self = Self(2);
    pub const NGHTTP3_CTRL_STREAM_STATE_GOAWAY: Self = Self(3);
    pub const NGHTTP3_CTRL_STREAM_STATE_MAX_PUSH_ID: Self = Self(4);
    pub const NGHTTP3_CTRL_STREAM_STATE_IGN_FRAME: Self = Self(5);
    pub const NGHTTP3_CTRL_STREAM_STATE_SETTINGS_ID: Self = Self(6);
    pub const NGHTTP3_CTRL_STREAM_STATE_SETTINGS_VALUE: Self = Self(7);
    pub const NGHTTP3_CTRL_STREAM_STATE_PRIORITY_UPDATE_PRI_ELEM_ID: Self = Self(8);
    pub const NGHTTP3_CTRL_STREAM_STATE_PRIORITY_UPDATE: Self = Self(9);
    pub const NGHTTP3_CTRL_STREAM_STATE_ORIGIN_ORIGIN_LEN: Self = Self(10);
    pub const NGHTTP3_CTRL_STREAM_STATE_ORIGIN_ASCII_ORIGIN: Self = Self(11);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nghttp3_req_stream_state(pub ::core::ffi::c_uint);
impl nghttp3_req_stream_state {
    pub const NGHTTP3_REQ_STREAM_STATE_FRAME_TYPE: Self = Self(0);
    pub const NGHTTP3_REQ_STREAM_STATE_FRAME_LENGTH: Self = Self(1);
    pub const NGHTTP3_REQ_STREAM_STATE_DATA: Self = Self(2);
    pub const NGHTTP3_REQ_STREAM_STATE_HEADERS: Self = Self(3);
    pub const NGHTTP3_REQ_STREAM_STATE_IGN_FRAME: Self = Self(4);
    pub const NGHTTP3_REQ_STREAM_STATE_IGN_REST: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_chunk {
    pub oplent: nghttp3_opl_entry,
}
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NGHTTP3_ERR_INVALID_ARGUMENT: ::core::ffi::c_int = -101 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_INVALID_STATE: ::core::ffi::c_int = -102 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_STREAM_IN_USE: ::core::ffi::c_int = -104 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_MALFORMED_HTTP_HEADER: ::core::ffi::c_int = -105;
pub const NGHTTP3_ERR_REMOVE_HTTP_HEADER: ::core::ffi::c_int = -106;
pub const NGHTTP3_ERR_STREAM_NOT_FOUND: ::core::ffi::c_int = -110 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_CONN_CLOSING: ::core::ffi::c_int = -111 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED: ::core::ffi::c_int = -401
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_H3_FRAME_UNEXPECTED: ::core::ffi::c_int = -601
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_H3_FRAME_ERROR: ::core::ffi::c_int = -602 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_H3_MISSING_SETTINGS: ::core::ffi::c_int = -603
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_H3_CLOSED_CRITICAL_STREAM: ::core::ffi::c_int = -605
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_H3_GENERAL_PROTOCOL_ERROR: ::core::ffi::c_int = -606
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_H3_ID_ERROR: ::core::ffi::c_int = -607 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_H3_SETTINGS_ERROR: ::core::ffi::c_int = -608 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_H3_STREAM_CREATION_ERROR: ::core::ffi::c_int = -609
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_H3_EXCESSIVE_LOAD: ::core::ffi::c_int = -610 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_NOMEM: ::core::ffi::c_int = -901 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_CALLBACK_FAILURE: ::core::ffi::c_int = -902 as ::core::ffi::c_int;
pub const NGHTTP3_H3_NO_ERROR: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const NGHTTP3_H3_STREAM_CREATION_ERROR: ::core::ffi::c_int = 0x103
    as ::core::ffi::c_int;
pub const NGHTTP3_H3_REQUEST_REJECTED: ::core::ffi::c_int = 0x10b as ::core::ffi::c_int;
pub const NGHTTP3_QPACK_DECODE_FLAG_EMIT: ::core::ffi::c_uint = 0x1
    as ::core::ffi::c_uint;
pub const NGHTTP3_QPACK_DECODE_FLAG_FINAL: ::core::ffi::c_uint = 0x2
    as ::core::ffi::c_uint;
pub const NGHTTP3_QPACK_DECODE_FLAG_BLOCKED: ::core::ffi::c_uint = 0x4
    as ::core::ffi::c_uint;
pub const NGHTTP3_SHUTDOWN_NOTICE_STREAM_ID: ::core::ffi::c_ulonglong = ((1
    as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
    .wrapping_sub(4 as ::core::ffi::c_ulonglong);
pub const NGHTTP3_SHUTDOWN_NOTICE_PUSH_ID: ::core::ffi::c_ulonglong = ((1
    as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
    .wrapping_sub(1 as ::core::ffi::c_ulonglong);
pub const NGHTTP3_STREAM_CLOSE_FLAG_NONE: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_CLOSE_FLAG_RX_APP_ERROR_CODE_SET: ::core::ffi::c_uint = 0x1
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_CLOSE_FLAG_TX_APP_ERROR_CODE_SET: ::core::ffi::c_uint = 0x2
    as ::core::ffi::c_uint;
pub const NGHTTP3_DEFAULT_URGENCY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NGHTTP3_URGENCY_LOW: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const NGHTTP3_URGENCY_LEVELS: ::core::ffi::c_int = NGHTTP3_URGENCY_LOW
    + 1 as ::core::ffi::c_int;
pub const NGHTTP3_PQ_BAD_INDEX: ::core::ffi::c_ulong = SIZE_MAX;
pub const NGHTTP3_TNODE_MAX_CYCLE_GAP: ::core::ffi::c_ulonglong = (1
    as ::core::ffi::c_ulonglong) << 24 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn nghttp3_ringbuf_len(mut rb: *const nghttp3_ringbuf) -> size_t {
    return (*rb).len;
}
pub const NGHTTP3_FRAME_DATA: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const NGHTTP3_FRAME_HEADERS: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const NGHTTP3_FRAME_CANCEL_PUSH: uint64_t = 3 as uint64_t;
pub const NGHTTP3_FRAME_SETTINGS: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const NGHTTP3_FRAME_PUSH_PROMISE: uint64_t = 5 as uint64_t;
pub const NGHTTP3_FRAME_GOAWAY: ::core::ffi::c_uint = 0x7 as ::core::ffi::c_uint;
pub const NGHTTP3_FRAME_MAX_PUSH_ID: uint64_t = 13 as uint64_t;
pub const NGHTTP3_FRAME_PRIORITY_UPDATE: ::core::ffi::c_uint = 0xf0700
    as ::core::ffi::c_uint;
pub const NGHTTP3_FRAME_PRIORITY_UPDATE_PUSH_ID: uint64_t = 984833 as uint64_t;
pub const NGHTTP3_FRAME_ORIGIN: ::core::ffi::c_uint = 0xc as ::core::ffi::c_uint;
pub const NGHTTP3_H2_FRAME_PRIORITY: uint64_t = 2 as uint64_t;
pub const NGHTTP3_H2_FRAME_PING: uint64_t = 6 as uint64_t;
pub const NGHTTP3_H2_FRAME_WINDOW_UPDATE: uint64_t = 8 as uint64_t;
pub const NGHTTP3_H2_FRAME_CONTINUATION: uint64_t = 9 as uint64_t;
pub const NGHTTP3_SETTINGS_ID_MAX_FIELD_SECTION_SIZE: uint64_t = 6 as uint64_t;
pub const NGHTTP3_SETTINGS_ID_QPACK_MAX_TABLE_CAPACITY: uint64_t = 1 as uint64_t;
pub const NGHTTP3_SETTINGS_ID_QPACK_BLOCKED_STREAMS: uint64_t = 7 as uint64_t;
pub const NGHTTP3_SETTINGS_ID_ENABLE_CONNECT_PROTOCOL: uint64_t = 8 as uint64_t;
pub const NGHTTP3_SETTINGS_ID_H3_DATAGRAM: uint64_t = 51 as uint64_t;
pub const NGHTTP3_H2_SETTINGS_ID_ENABLE_PUSH: uint64_t = 2 as uint64_t;
pub const NGHTTP3_H2_SETTINGS_ID_MAX_CONCURRENT_STREAMS: uint64_t = 3 as uint64_t;
pub const NGHTTP3_H2_SETTINGS_ID_INITIAL_WINDOW_SIZE: uint64_t = 4 as uint64_t;
pub const NGHTTP3_H2_SETTINGS_ID_MAX_FRAME_SIZE: uint64_t = 5 as uint64_t;
#[inline]
unsafe extern "C" fn nghttp3_max_long_int(
    mut a: ::core::ffi::c_long,
    mut b: ::core::ffi::c_long,
) -> ::core::ffi::c_long {
    return if a < b { b } else { a };
}
#[inline]
unsafe extern "C" fn nghttp3_min_long_long_int(
    mut a: ::core::ffi::c_longlong,
    mut b: ::core::ffi::c_longlong,
) -> ::core::ffi::c_longlong {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn nghttp3_min_unsigned_long_int(
    mut a: ::core::ffi::c_ulong,
    mut b: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    return if a < b { a } else { b };
}
pub const NGHTTP3_STREAM_MIN_CHUNK_SIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NGHTTP3_STREAM_MIN_WRITELEN: ::core::ffi::c_int = 800 as ::core::ffi::c_int;
pub const NGHTTP3_STREAM_TYPE_CONTROL: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_TYPE_PUSH: uint64_t = 1 as uint64_t;
pub const NGHTTP3_STREAM_TYPE_QPACK_ENCODER: ::core::ffi::c_uint = 0x2
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_TYPE_QPACK_DECODER: ::core::ffi::c_uint = 0x3
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_TYPE_UNKNOWN: ::core::ffi::c_ulong = UINT64_MAX;
pub const NGHTTP3_STREAM_FLAG_TYPE_IDENTIFIED: ::core::ffi::c_uint = 0x1
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_FLAG_FC_BLOCKED: ::core::ffi::c_uint = 0x2
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_FLAG_READ_DATA_BLOCKED: ::core::ffi::c_uint = 0x4
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_FLAG_WRITE_END_STREAM: ::core::ffi::c_uint = 0x8
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_FLAG_QPACK_DECODE_BLOCKED: ::core::ffi::c_uint = 0x10
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_FLAG_READ_EOF: ::core::ffi::c_uint = 0x20
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_FLAG_SHUT_WR: ::core::ffi::c_uint = 0x100
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_FLAG_SHUT_RD: ::core::ffi::c_uint = 0x200
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_FLAG_SERVER_PRIORITY_SET: ::core::ffi::c_uint = 0x400
    as ::core::ffi::c_uint;
pub const NGHTTP3_STREAM_FLAG_PRIORITY_UPDATE_RECVED: ::core::ffi::c_uint = 0x800
    as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn nghttp3_objalloc_stream_init(
    mut objalloc: *mut nghttp3_objalloc,
    mut nmemb: size_t,
    mut mem: *const nghttp3_mem,
) {
    nghttp3_objalloc_init(
        objalloc,
        (::core::mem::size_of::<nghttp3_stream>().wrapping_add(0xf as size_t)
            & !(0xf as ::core::ffi::c_uint as size_t))
            .wrapping_mul(nmemb),
        mem,
    );
}
pub const NGHTTP3_CONN_FLAG_SETTINGS_RECVED: ::core::ffi::c_uint = 0x1
    as ::core::ffi::c_uint;
pub const NGHTTP3_CONN_FLAG_CONTROL_OPENED: ::core::ffi::c_uint = 0x2
    as ::core::ffi::c_uint;
pub const NGHTTP3_CONN_FLAG_QPACK_ENCODER_OPENED: ::core::ffi::c_uint = 0x4
    as ::core::ffi::c_uint;
pub const NGHTTP3_CONN_FLAG_QPACK_DECODER_OPENED: ::core::ffi::c_uint = 0x8
    as ::core::ffi::c_uint;
pub const NGHTTP3_CONN_FLAG_SHUTDOWN_COMMENCED: ::core::ffi::c_uint = 0x10
    as ::core::ffi::c_uint;
pub const NGHTTP3_CONN_FLAG_GOAWAY_RECVED: ::core::ffi::c_uint = 0x20
    as ::core::ffi::c_uint;
pub const NGHTTP3_CONN_FLAG_GOAWAY_QUEUED: ::core::ffi::c_uint = 0x40
    as ::core::ffi::c_uint;
pub const NGHTTP3_VARINT_MAX: ::core::ffi::c_ulonglong = ((1 as ::core::ffi::c_ulonglong)
    << 62 as ::core::ffi::c_int)
    .wrapping_sub(1 as ::core::ffi::c_ulonglong);
pub const NGHTTP3_HTTP_FLAG_PRIORITY: ::core::ffi::c_uint = 0x8000
    as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn nghttp3_objalloc_chunk_len_get(
    mut objalloc: *mut nghttp3_objalloc,
    mut len: size_t,
) -> *mut nghttp3_chunk {
    let mut oplent: *mut nghttp3_opl_entry = nghttp3_opl_pop(&raw mut (*objalloc).opl);
    let mut obj: *mut nghttp3_chunk = ::core::ptr::null_mut::<nghttp3_chunk>();
    let mut rv: ::core::ffi::c_int = 0;
    if oplent.is_null() {
        rv = nghttp3_balloc_get(
            &raw mut (*objalloc).balloc,
            &raw mut obj as *mut *mut ::core::ffi::c_void,
            len,
        );
        if rv != 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<nghttp3_chunk>();
        }
        return obj;
    }
    return (oplent as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_chunk;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_objalloc_chunk_get(
    mut objalloc: *mut nghttp3_objalloc,
) -> *mut nghttp3_chunk {
    let mut oplent: *mut nghttp3_opl_entry = nghttp3_opl_pop(&raw mut (*objalloc).opl);
    let mut obj: *mut nghttp3_chunk = ::core::ptr::null_mut::<nghttp3_chunk>();
    let mut rv: ::core::ffi::c_int = 0;
    if oplent.is_null() {
        rv = nghttp3_balloc_get(
            &raw mut (*objalloc).balloc,
            &raw mut obj as *mut *mut ::core::ffi::c_void,
            ::core::mem::size_of::<nghttp3_chunk>(),
        );
        if rv != 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<nghttp3_chunk>();
        }
        return obj;
    }
    return (oplent as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_chunk;
}
unsafe extern "C" fn conn_remote_stream_uni(
    mut conn: *const nghttp3_conn,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    if (*conn).server != 0 {
        return (stream_id & 0x3 as int64_t == 0x2 as int64_t) as ::core::ffi::c_int;
    }
    return (stream_id & 0x3 as int64_t == 0x3 as int64_t) as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_call_begin_headers(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    if (*conn).callbacks.begin_headers.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    rv = (*conn)
        .callbacks
        .begin_headers
        .expect(
            "non-null function pointer",
        )(
        conn,
        (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
        (*conn).user_data,
        (*stream).c2rust_unnamed.c2rust_unnamed.user_data,
    );
    if rv != 0 as ::core::ffi::c_int {
        return NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_call_end_headers(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut fin: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    if (*conn).callbacks.end_headers.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    rv = (*conn)
        .callbacks
        .end_headers
        .expect(
            "non-null function pointer",
        )(
        conn,
        (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
        fin,
        (*conn).user_data,
        (*stream).c2rust_unnamed.c2rust_unnamed.user_data,
    );
    if rv != 0 as ::core::ffi::c_int {
        return NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_call_begin_trailers(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    if (*conn).callbacks.begin_trailers.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    rv = (*conn)
        .callbacks
        .begin_trailers
        .expect(
            "non-null function pointer",
        )(
        conn,
        (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
        (*conn).user_data,
        (*stream).c2rust_unnamed.c2rust_unnamed.user_data,
    );
    if rv != 0 as ::core::ffi::c_int {
        return NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_call_end_trailers(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut fin: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    if (*conn).callbacks.end_trailers.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    rv = (*conn)
        .callbacks
        .end_trailers
        .expect(
            "non-null function pointer",
        )(
        conn,
        (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
        fin,
        (*conn).user_data,
        (*stream).c2rust_unnamed.c2rust_unnamed.user_data,
    );
    if rv != 0 as ::core::ffi::c_int {
        return NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_call_end_stream(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    if (*conn).callbacks.end_stream.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    rv = (*conn)
        .callbacks
        .end_stream
        .expect(
            "non-null function pointer",
        )(
        conn,
        (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
        (*conn).user_data,
        (*stream).c2rust_unnamed.c2rust_unnamed.user_data,
    );
    if rv != 0 as ::core::ffi::c_int {
        return NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_call_stop_sending(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut app_error_code: uint64_t,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    if (*conn).callbacks.stop_sending.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    rv = (*conn)
        .callbacks
        .stop_sending
        .expect(
            "non-null function pointer",
        )(
        conn,
        (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
        app_error_code,
        (*conn).user_data,
        (*stream).c2rust_unnamed.c2rust_unnamed.user_data,
    );
    if rv != 0 as ::core::ffi::c_int {
        return NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_call_reset_stream(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut app_error_code: uint64_t,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    if (*conn).callbacks.reset_stream.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    rv = (*conn)
        .callbacks
        .reset_stream
        .expect(
            "non-null function pointer",
        )(
        conn,
        (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
        app_error_code,
        (*conn).user_data,
        (*stream).c2rust_unnamed.c2rust_unnamed.user_data,
    );
    if rv != 0 as ::core::ffi::c_int {
        return NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_call_deferred_consume(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut nconsumed: size_t,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    if nconsumed == 0 as size_t || (*conn).callbacks.deferred_consume.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    rv = (*conn)
        .callbacks
        .deferred_consume
        .expect(
            "non-null function pointer",
        )(
        conn,
        (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
        nconsumed,
        (*conn).user_data,
        (*stream).c2rust_unnamed.c2rust_unnamed.user_data,
    );
    if rv != 0 as ::core::ffi::c_int {
        return NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_call_recv_settings(
    mut conn: *mut nghttp3_conn,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    if (*conn).callbacks.recv_settings2.is_none() {
        if (*conn).callbacks.recv_settings.is_none() {
            return 0 as ::core::ffi::c_int;
        }
        let mut c2rust_lvalue: nghttp3_settings = nghttp3_settings {
            max_field_section_size: (*conn).remote.settings.max_field_section_size,
            qpack_max_dtable_capacity: (*conn).remote.settings.qpack_max_dtable_capacity,
            qpack_encoder_max_dtable_capacity: 0,
            qpack_blocked_streams: (*conn).remote.settings.qpack_blocked_streams,
            enable_connect_protocol: (*conn).remote.settings.enable_connect_protocol,
            h3_datagram: (*conn).remote.settings.h3_datagram,
            origin_list: ::core::ptr::null::<nghttp3_vec>(),
            glitch_ratelim_burst: 0,
            glitch_ratelim_rate: 0,
            qpack_indexing_strat: nghttp3_qpack_indexing_strat::NGHTTP3_QPACK_INDEXING_STRAT_NONE,
        };
        rv = (*conn)
            .callbacks
            .recv_settings
            .expect(
                "non-null function pointer",
            )(conn, &raw mut c2rust_lvalue, (*conn).user_data);
        if rv != 0 as ::core::ffi::c_int {
            return NGHTTP3_ERR_CALLBACK_FAILURE;
        }
        return 0 as ::core::ffi::c_int;
    }
    rv = (*conn)
        .callbacks
        .recv_settings2
        .expect(
            "non-null function pointer",
        )(conn, &raw mut (*conn).remote.settings, (*conn).user_data);
    if rv != 0 as ::core::ffi::c_int {
        return NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_call_recv_origin(
    mut conn: *mut nghttp3_conn,
    mut origin: *const uint8_t,
    mut originlen: size_t,
) -> ::core::ffi::c_int {
    if (*conn).callbacks.recv_origin.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn)
        .callbacks
        .recv_origin
        .expect("non-null function pointer")(conn, origin, originlen, (*conn).user_data)
        != 0 as ::core::ffi::c_int
    {
        return NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_call_end_origin(
    mut conn: *mut nghttp3_conn,
) -> ::core::ffi::c_int {
    if (*conn).callbacks.end_origin.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn)
        .callbacks
        .end_origin
        .expect("non-null function pointer")(conn, (*conn).user_data)
        != 0 as ::core::ffi::c_int
    {
        return NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_glitch_ratelim_drain(
    mut conn: *mut nghttp3_conn,
    mut n: uint64_t,
    mut ts: nghttp3_tstamp,
) -> ::core::ffi::c_int {
    if ts == UINT64_MAX as nghttp3_tstamp {
        return 0 as ::core::ffi::c_int;
    }
    return nghttp3_ratelim_drain(&raw mut (*conn).glitch_rlim, n, ts);
}
unsafe extern "C" fn ricnt_less(
    mut lhsx: *const nghttp3_pq_entry,
    mut rhsx: *const nghttp3_pq_entry,
) -> ::core::ffi::c_int {
    let mut lhs: *mut nghttp3_stream = (lhsx as *mut ::core::ffi::c_char)
        .offset(-(56 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_stream;
    let mut rhs: *mut nghttp3_stream = (rhsx as *mut ::core::ffi::c_char)
        .offset(-(56 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_stream;
    return ((*lhs).c2rust_unnamed.c2rust_unnamed.qpack_sctx.ricnt
        < (*rhs).c2rust_unnamed.c2rust_unnamed.qpack_sctx.ricnt) as ::core::ffi::c_int;
}
unsafe extern "C" fn cycle_less(
    mut lhsx: *const nghttp3_pq_entry,
    mut rhsx: *const nghttp3_pq_entry,
) -> ::core::ffi::c_int {
    let mut lhs: *const nghttp3_tnode = (lhsx as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_tnode;
    let mut rhs: *const nghttp3_tnode = (rhsx as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_tnode;
    if (*lhs).cycle == (*rhs).cycle {
        return ((*lhs).id < (*rhs).id) as ::core::ffi::c_int;
    }
    return ((*rhs).cycle.wrapping_sub((*lhs).cycle) as ::core::ffi::c_ulonglong
        <= NGHTTP3_TNODE_MAX_CYCLE_GAP) as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_new(
    mut pconn: *mut *mut nghttp3_conn,
    mut server: ::core::ffi::c_int,
    mut callbacks_version: ::core::ffi::c_int,
    mut callbacks: *const nghttp3_callbacks,
    mut settings_version: ::core::ffi::c_int,
    mut settings: *const nghttp3_settings,
    mut mem: *const nghttp3_mem,
    mut user_data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut conn: *mut nghttp3_conn = ::core::ptr::null_mut::<nghttp3_conn>();
    let mut settings_latest: nghttp3_settings = nghttp3_settings {
        max_field_section_size: 0,
        qpack_max_dtable_capacity: 0,
        qpack_encoder_max_dtable_capacity: 0,
        qpack_blocked_streams: 0,
        enable_connect_protocol: 0,
        h3_datagram: 0,
        origin_list: ::core::ptr::null::<nghttp3_vec>(),
        glitch_ratelim_burst: 0,
        glitch_ratelim_rate: 0,
        qpack_indexing_strat: nghttp3_qpack_indexing_strat::NGHTTP3_QPACK_INDEXING_STRAT_NONE,
    };
    let mut callbacks_latest: nghttp3_callbacks = nghttp3_callbacks {
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
    let mut map_seed: uint64_t = 0;
    let mut i: size_t = 0;
    settings = nghttp3_settings_convert_to_latest(
        &raw mut settings_latest,
        settings_version,
        settings,
    );
    callbacks = nghttp3_callbacks_convert_to_latest(
        &raw mut callbacks_latest,
        callbacks_version,
        callbacks,
    );
    '_c2rust_label: {
        if (*settings).max_field_section_size as ::core::ffi::c_ulonglong
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong)
        {} else {
            __assert_fail(
                b"settings->max_field_section_size <= NGHTTP3_VARINT_MAX\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_uint,
                b"int conn_new(nghttp3_conn **, int, int, const nghttp3_callbacks *, int, const nghttp3_settings *, const nghttp3_mem *, void *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*settings).qpack_max_dtable_capacity as ::core::ffi::c_ulonglong
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong)
        {} else {
            __assert_fail(
                b"settings->qpack_max_dtable_capacity <= NGHTTP3_VARINT_MAX\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                302 as ::core::ffi::c_uint,
                b"int conn_new(nghttp3_conn **, int, int, const nghttp3_callbacks *, int, const nghttp3_settings *, const nghttp3_mem *, void *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*settings).qpack_encoder_max_dtable_capacity as ::core::ffi::c_ulonglong
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong)
        {} else {
            __assert_fail(
                b"settings->qpack_encoder_max_dtable_capacity <= NGHTTP3_VARINT_MAX\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                303 as ::core::ffi::c_uint,
                b"int conn_new(nghttp3_conn **, int, int, const nghttp3_callbacks *, int, const nghttp3_settings *, const nghttp3_mem *, void *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*settings).qpack_blocked_streams as ::core::ffi::c_ulonglong
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong)
        {} else {
            __assert_fail(
                b"settings->qpack_blocked_streams <= NGHTTP3_VARINT_MAX\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                304 as ::core::ffi::c_uint,
                b"int conn_new(nghttp3_conn **, int, int, const nghttp3_callbacks *, int, const nghttp3_settings *, const nghttp3_mem *, void *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if mem.is_null() {
        mem = nghttp3_mem_default();
    }
    conn = nghttp3_mem_calloc(mem, 1 as size_t, ::core::mem::size_of::<nghttp3_conn>())
        as *mut nghttp3_conn;
    if conn.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    nghttp3_objalloc_init(
        &raw mut (*conn).out_chunk_objalloc,
        (NGHTTP3_STREAM_MIN_CHUNK_SIZE * 16 as ::core::ffi::c_int) as size_t,
        mem,
    );
    nghttp3_objalloc_stream_init(&raw mut (*conn).stream_objalloc, 8 as size_t, mem);
    if (*callbacks).rand.is_some() {
        (*callbacks)
            .rand
            .expect(
                "non-null function pointer",
            )(&raw mut map_seed as *mut uint8_t, ::core::mem::size_of::<uint64_t>());
    } else {
        map_seed = 0 as uint64_t;
    }
    nghttp3_map_init(&raw mut (*conn).streams, map_seed, mem);
    nghttp3_qpack_decoder_init(
        &raw mut (*conn).qdec,
        (*settings).qpack_max_dtable_capacity,
        (*settings).qpack_blocked_streams,
        mem,
    );
    map_seed = map_seed.wrapping_add(1);
    nghttp3_qpack_encoder_init(
        &raw mut (*conn).qenc,
        (*settings).qpack_encoder_max_dtable_capacity,
        map_seed,
        mem,
    );
    nghttp3_qpack_encoder_set_indexing_strat(
        &raw mut (*conn).qenc,
        (*settings).qpack_indexing_strat,
    );
    nghttp3_pq_init(
        &raw mut (*conn).qpack_blocked_streams,
        Some(
            ricnt_less
                as unsafe extern "C" fn(
                    *const nghttp3_pq_entry,
                    *const nghttp3_pq_entry,
                ) -> ::core::ffi::c_int,
        ),
        mem,
    );
    i = 0 as size_t;
    while i < NGHTTP3_URGENCY_LEVELS as size_t {
        nghttp3_pq_init(
            &raw mut (*(&raw mut (*conn).sched as *mut C2Rust_Unnamed_19)
                .offset(i as isize))
                .spq,
            Some(
                cycle_less
                    as unsafe extern "C" fn(
                        *const nghttp3_pq_entry,
                        *const nghttp3_pq_entry,
                    ) -> ::core::ffi::c_int,
            ),
            mem,
        );
        i = i.wrapping_add(1);
    }
    nghttp3_idtr_init(&raw mut (*conn).remote.bidi.idtr, mem);
    nghttp3_ratelim_init(
        &raw mut (*conn).glitch_rlim,
        (*settings).glitch_ratelim_burst,
        (*settings).glitch_ratelim_rate,
        0 as nghttp3_tstamp,
    );
    (*conn).callbacks = *callbacks;
    (*conn).local.settings = *settings;
    if server != 0 {
        if !(*settings).origin_list.is_null() {
            (*conn).local.settings.origin_list = &raw mut (*conn).local.origin_list;
            (*conn).local.origin_list = *(*settings).origin_list;
        }
    } else {
        (*conn).local.settings.enable_connect_protocol = 0 as uint8_t;
        (*conn).local.settings.origin_list = ::core::ptr::null::<nghttp3_vec>();
    }
    (*conn).remote.settings.max_field_section_size = NGHTTP3_VARINT_MAX as uint64_t;
    (*conn).mem = mem;
    (*conn).user_data = user_data;
    (*conn).server = server;
    (*conn).rx.goaway_id = NGHTTP3_VARINT_MAX.wrapping_add(1 as ::core::ffi::c_ulonglong)
        as int64_t;
    (*conn).tx.goaway_id = NGHTTP3_VARINT_MAX.wrapping_add(1 as ::core::ffi::c_ulonglong)
        as int64_t;
    (*conn).rx.max_stream_id_bidi = -4 as int64_t;
    *pconn = conn;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_client_new_versioned(
    mut pconn: *mut *mut nghttp3_conn,
    mut callbacks_version: ::core::ffi::c_int,
    mut callbacks: *const nghttp3_callbacks,
    mut settings_version: ::core::ffi::c_int,
    mut settings: *const nghttp3_settings,
    mut mem: *const nghttp3_mem,
    mut user_data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    rv = conn_new(
        pconn,
        0 as ::core::ffi::c_int,
        callbacks_version,
        callbacks,
        settings_version,
        settings,
        mem,
        user_data,
    );
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_server_new_versioned(
    mut pconn: *mut *mut nghttp3_conn,
    mut callbacks_version: ::core::ffi::c_int,
    mut callbacks: *const nghttp3_callbacks,
    mut settings_version: ::core::ffi::c_int,
    mut settings: *const nghttp3_settings,
    mut mem: *const nghttp3_mem,
    mut user_data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    rv = conn_new(
        pconn,
        1 as ::core::ffi::c_int,
        callbacks_version,
        callbacks,
        settings_version,
        settings,
        mem,
        user_data,
    );
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn free_stream(
    mut data: *mut ::core::ffi::c_void,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = data as *mut nghttp3_stream;
    nghttp3_stream_del(stream);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_del(mut conn: *mut nghttp3_conn) {
    let mut i: size_t = 0;
    if conn.is_null() {
        return;
    }
    nghttp3_buf_free(&raw mut (*conn).tx.qpack.ebuf, (*conn).mem);
    nghttp3_buf_free(&raw mut (*conn).tx.qpack.rbuf, (*conn).mem);
    nghttp3_idtr_free(&raw mut (*conn).remote.bidi.idtr);
    i = 0 as size_t;
    while i < NGHTTP3_URGENCY_LEVELS as size_t {
        nghttp3_pq_free(
            &raw mut (*(&raw mut (*conn).sched as *mut C2Rust_Unnamed_19)
                .offset(i as isize))
                .spq,
        );
        i = i.wrapping_add(1);
    }
    nghttp3_pq_free(&raw mut (*conn).qpack_blocked_streams);
    nghttp3_qpack_encoder_free(&raw mut (*conn).qenc);
    nghttp3_qpack_decoder_free(&raw mut (*conn).qdec);
    nghttp3_map_each(
        &raw mut (*conn).streams,
        Some(
            free_stream
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        NULL,
    );
    nghttp3_map_free(&raw mut (*conn).streams);
    nghttp3_objalloc_free(&raw mut (*conn).stream_objalloc);
    nghttp3_objalloc_free(&raw mut (*conn).out_chunk_objalloc);
    nghttp3_mem_free((*conn).mem, (*conn).rx.originbuf as *mut ::core::ffi::c_void);
    nghttp3_mem_free((*conn).mem, conn as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn conn_bidi_idtr_open(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    rv = nghttp3_idtr_open(&raw mut (*conn).remote.bidi.idtr, stream_id);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    if nghttp3_ksl_len(&raw mut (*conn).remote.bidi.idtr.gap.gap) > 32 as size_t {
        nghttp3_gaptr_drop_first_gap(&raw mut (*conn).remote.bidi.idtr.gap);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_read_stream(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
    mut src: *const uint8_t,
    mut srclen: size_t,
    mut fin: ::core::ffi::c_int,
) -> nghttp3_ssize {
    return nghttp3_conn_read_stream2(
        conn,
        stream_id,
        src,
        srclen,
        fin,
        UINT64_MAX as nghttp3_tstamp,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_read_stream2(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
    mut src: *const uint8_t,
    mut srclen: size_t,
    mut fin: ::core::ffi::c_int,
    mut ts: nghttp3_tstamp,
) -> nghttp3_ssize {
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    let mut bidi_nproc: size_t = 0;
    let mut rv: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if stream_id >= 0 as int64_t {} else {
            __assert_fail(
                b"stream_id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                475 as ::core::ffi::c_uint,
                b"nghttp3_ssize nghttp3_conn_read_stream2(nghttp3_conn *, int64_t, const uint8_t *, size_t, int, nghttp3_tstamp)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if stream_id
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong) as int64_t
        {} else {
            __assert_fail(
                b"stream_id <= (int64_t)NGHTTP3_MAX_VARINT\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                476 as ::core::ffi::c_uint,
                b"nghttp3_ssize nghttp3_conn_read_stream2(nghttp3_conn *, int64_t, const uint8_t *, size_t, int, nghttp3_tstamp)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    stream = nghttp3_conn_find_stream(conn, stream_id) as *mut nghttp3_stream;
    if stream.is_null() {
        if (*conn).server != 0 {
            if nghttp3_client_stream_bidi(stream_id) != 0 {
                rv = conn_bidi_idtr_open(conn, stream_id);
                if rv != 0 as ::core::ffi::c_int {
                    if nghttp3_err_is_fatal(rv) != 0 {
                        return rv as nghttp3_ssize;
                    }
                }
                (*conn).rx.max_stream_id_bidi = nghttp3_max_long_int(
                    (*conn).rx.max_stream_id_bidi as ::core::ffi::c_long,
                    stream_id as ::core::ffi::c_long,
                ) as int64_t;
                rv = nghttp3_conn_create_stream(conn, &raw mut stream, stream_id);
                if rv != 0 as ::core::ffi::c_int {
                    return rv as nghttp3_ssize;
                }
                if (*conn).flags as ::core::ffi::c_uint & NGHTTP3_CONN_FLAG_GOAWAY_QUEUED
                    != 0 && (*conn).tx.goaway_id <= stream_id
                {
                    (*stream).c2rust_unnamed.c2rust_unnamed.rstate.state = nghttp3_req_stream_state::NGHTTP3_REQ_STREAM_STATE_IGN_REST
                        .0 as ::core::ffi::c_int;
                    rv = nghttp3_conn_reject_stream(conn, stream);
                    if rv != 0 as ::core::ffi::c_int {
                        return rv as nghttp3_ssize;
                    }
                }
            } else if nghttp3_client_stream_uni(stream_id) == 0 {
                return NGHTTP3_ERR_H3_STREAM_CREATION_ERROR as nghttp3_ssize
            } else {
                if srclen == 0 as size_t && fin != 0 {
                    return 0 as nghttp3_ssize;
                }
                rv = nghttp3_conn_create_stream(conn, &raw mut stream, stream_id);
                if rv != 0 as ::core::ffi::c_int {
                    return rv as nghttp3_ssize;
                }
            }
            (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_INITIAL;
        } else if nghttp3_server_stream_uni(stream_id) != 0 {
            if srclen == 0 as size_t && fin != 0 {
                return 0 as nghttp3_ssize;
            }
            rv = nghttp3_conn_create_stream(conn, &raw mut stream, stream_id);
            if rv != 0 as ::core::ffi::c_int {
                return rv as nghttp3_ssize;
            }
            (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_INITIAL;
        } else {
            return NGHTTP3_ERR_H3_STREAM_CREATION_ERROR as nghttp3_ssize
        }
    } else if (*conn).server != 0 {
        '_c2rust_label_1: {
            if nghttp3_client_stream_bidi(stream_id) != 0
                || nghttp3_client_stream_uni(stream_id) != 0
            {} else {
                __assert_fail(
                    b"nghttp3_client_stream_bidi(stream_id) || nghttp3_client_stream_uni(stream_id)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    548 as ::core::ffi::c_uint,
                    b"nghttp3_ssize nghttp3_conn_read_stream2(nghttp3_conn *, int64_t, const uint8_t *, size_t, int, nghttp3_tstamp)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
    } else {
        '_c2rust_label_2: {
            if nghttp3_client_stream_bidi(stream_id) != 0
                || nghttp3_server_stream_uni(stream_id) != 0
            {} else {
                __assert_fail(
                    b"nghttp3_client_stream_bidi(stream_id) || nghttp3_server_stream_uni(stream_id)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    551 as ::core::ffi::c_uint,
                    b"nghttp3_ssize nghttp3_conn_read_stream2(nghttp3_conn *, int64_t, const uint8_t *, size_t, int, nghttp3_tstamp)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
    }
    if srclen == 0 as size_t && fin == 0 {
        return 0 as nghttp3_ssize;
    }
    if fin != 0 {
        (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
            .c2rust_unnamed
            .c2rust_unnamed
            .flags as ::core::ffi::c_uint | NGHTTP3_STREAM_FLAG_READ_EOF) as uint16_t;
    }
    if nghttp3_stream_uni(stream_id) != 0 {
        return nghttp3_conn_read_uni(conn, stream, src, srclen, fin, ts);
    }
    return nghttp3_conn_read_bidi(
        conn,
        &raw mut bidi_nproc,
        stream,
        src,
        srclen,
        fin,
        ts,
    );
}
unsafe extern "C" fn conn_read_type(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut src: *const uint8_t,
    mut srclen: size_t,
    mut fin: ::core::ffi::c_int,
) -> nghttp3_ssize {
    let mut rstate: *mut nghttp3_stream_read_state = &raw mut (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .rstate;
    let mut rvint: *mut nghttp3_varint_read_state = &raw mut (*rstate).rvint;
    let mut nread: nghttp3_ssize = 0;
    let mut stream_type: uint64_t = 0;
    '_c2rust_label: {
        if srclen != 0 {} else {
            __assert_fail(
                b"srclen\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                578 as ::core::ffi::c_uint,
                b"nghttp3_ssize conn_read_type(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, int)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    nread = nghttp3_read_varint(rvint, src, src.offset(srclen as isize), fin);
    if nread < 0 as nghttp3_ssize {
        return NGHTTP3_ERR_H3_GENERAL_PROTOCOL_ERROR as nghttp3_ssize;
    }
    if (*rvint).left != 0 {
        return nread;
    }
    stream_type = (*rvint).acc;
    nghttp3_varint_read_state_reset(rvint);
    match stream_type {
        0 => {
            if (*conn).flags as ::core::ffi::c_uint & NGHTTP3_CONN_FLAG_CONTROL_OPENED
                != 0
            {
                return NGHTTP3_ERR_H3_STREAM_CREATION_ERROR as nghttp3_ssize;
            }
            (*conn).flags = ((*conn).flags as ::core::ffi::c_uint
                | NGHTTP3_CONN_FLAG_CONTROL_OPENED) as uint16_t;
            (*stream).c2rust_unnamed.c2rust_unnamed.r#type = NGHTTP3_STREAM_TYPE_CONTROL
                as nghttp3_stream_type;
            (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_FRAME_TYPE
                .0 as ::core::ffi::c_int;
        }
        NGHTTP3_STREAM_TYPE_PUSH => {
            return NGHTTP3_ERR_H3_STREAM_CREATION_ERROR as nghttp3_ssize;
        }
        0x2 => {
            if (*conn).flags as ::core::ffi::c_uint
                & NGHTTP3_CONN_FLAG_QPACK_ENCODER_OPENED != 0
            {
                return NGHTTP3_ERR_H3_STREAM_CREATION_ERROR as nghttp3_ssize;
            }
            (*conn).flags = ((*conn).flags as ::core::ffi::c_uint
                | NGHTTP3_CONN_FLAG_QPACK_ENCODER_OPENED) as uint16_t;
            (*stream).c2rust_unnamed.c2rust_unnamed.r#type = NGHTTP3_STREAM_TYPE_QPACK_ENCODER
                as nghttp3_stream_type;
        }
        0x3 => {
            if (*conn).flags as ::core::ffi::c_uint
                & NGHTTP3_CONN_FLAG_QPACK_DECODER_OPENED != 0
            {
                return NGHTTP3_ERR_H3_STREAM_CREATION_ERROR as nghttp3_ssize;
            }
            (*conn).flags = ((*conn).flags as ::core::ffi::c_uint
                | NGHTTP3_CONN_FLAG_QPACK_DECODER_OPENED) as uint16_t;
            (*stream).c2rust_unnamed.c2rust_unnamed.r#type = NGHTTP3_STREAM_TYPE_QPACK_DECODER
                as nghttp3_stream_type;
        }
        _ => {
            (*stream).c2rust_unnamed.c2rust_unnamed.r#type = NGHTTP3_STREAM_TYPE_UNKNOWN
                as nghttp3_stream_type;
        }
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .flags as ::core::ffi::c_uint | NGHTTP3_STREAM_FLAG_TYPE_IDENTIFIED) as uint16_t;
    return nread;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_read_uni(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut src: *const uint8_t,
    mut srclen: size_t,
    mut fin: ::core::ffi::c_int,
    mut ts: nghttp3_tstamp,
) -> nghttp3_ssize {
    let mut nread: nghttp3_ssize = 0 as nghttp3_ssize;
    let mut nconsumed: nghttp3_ssize = 0 as nghttp3_ssize;
    let mut rv: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if srclen != 0 || fin != 0 {} else {
            __assert_fail(
                b"srclen || fin\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                638 as ::core::ffi::c_uint,
                b"nghttp3_ssize nghttp3_conn_read_uni(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, int, nghttp3_tstamp)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
        & NGHTTP3_STREAM_FLAG_TYPE_IDENTIFIED == 0
    {
        if srclen == 0 as size_t && fin != 0 {
            if (*stream).c2rust_unnamed.c2rust_unnamed.rstate.rvint.left != 0 {
                return NGHTTP3_ERR_H3_GENERAL_PROTOCOL_ERROR as nghttp3_ssize;
            }
            if conn_glitch_ratelim_drain(conn, 1 as uint64_t, ts)
                != 0 as ::core::ffi::c_int
            {
                return NGHTTP3_ERR_H3_EXCESSIVE_LOAD as nghttp3_ssize;
            }
            return conn_delete_stream(
                conn,
                stream,
                NGHTTP3_STREAM_CLOSE_FLAG_NONE as uint32_t,
                0 as uint64_t,
                0 as uint64_t,
            ) as nghttp3_ssize;
        }
        nread = conn_read_type(conn, stream, src, srclen, fin);
        if nread < 0 as nghttp3_ssize {
            return nread as ::core::ffi::c_int as nghttp3_ssize;
        }
        if (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
            & NGHTTP3_STREAM_FLAG_TYPE_IDENTIFIED == 0
        {
            '_c2rust_label_0: {
                if nread as size_t == srclen {} else {
                    __assert_fail(
                        b"(size_t)nread == srclen\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        663 as ::core::ffi::c_uint,
                        b"nghttp3_ssize nghttp3_conn_read_uni(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, int, nghttp3_tstamp)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            return srclen as nghttp3_ssize;
        }
        src = src.offset(nread as isize);
        srclen = srclen.wrapping_sub(nread as size_t);
        if (*stream).c2rust_unnamed.c2rust_unnamed.r#type
            == NGHTTP3_STREAM_TYPE_UNKNOWN as nghttp3_stream_type
        {
            if conn_glitch_ratelim_drain(conn, 1 as uint64_t, ts)
                != 0 as ::core::ffi::c_int
            {
                return NGHTTP3_ERR_H3_EXCESSIVE_LOAD as nghttp3_ssize;
            }
            if fin == 0 {
                rv = conn_call_stop_sending(
                    conn,
                    stream,
                    NGHTTP3_H3_STREAM_CREATION_ERROR as uint64_t,
                );
                if rv != 0 as ::core::ffi::c_int {
                    return rv as nghttp3_ssize;
                }
            }
        }
        if srclen == 0 as size_t {
            return nread;
        }
    }
    match (*stream).c2rust_unnamed.c2rust_unnamed.r#type {
        0 => {
            if fin != 0 {
                return NGHTTP3_ERR_H3_CLOSED_CRITICAL_STREAM as nghttp3_ssize;
            }
            nconsumed = nghttp3_conn_read_control(conn, stream, src, srclen, ts);
        }
        0x2 => {
            if fin != 0 {
                return NGHTTP3_ERR_H3_CLOSED_CRITICAL_STREAM as nghttp3_ssize;
            }
            nconsumed = nghttp3_conn_read_qpack_encoder(conn, src, srclen, ts);
        }
        0x3 => {
            if fin != 0 {
                return NGHTTP3_ERR_H3_CLOSED_CRITICAL_STREAM as nghttp3_ssize;
            }
            nconsumed = nghttp3_conn_read_qpack_decoder(conn, src, srclen);
        }
        NGHTTP3_STREAM_TYPE_UNKNOWN => {
            nconsumed = srclen as nghttp3_ssize;
        }
        _ => {
            nghttp3_unreachable_fail(
                b"nghttp3_conn.c\0".as_ptr() as *const ::core::ffi::c_char,
                713 as ::core::ffi::c_int,
                b"nghttp3_conn_read_uni\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    if nconsumed < 0 as nghttp3_ssize {
        return nconsumed;
    }
    return nread + nconsumed;
}
unsafe extern "C" fn conn_reset_rx_originlen(mut conn: *mut nghttp3_conn) {
    (*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen_offset = 0 as size_t;
    (*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen = 0 as uint16_t;
}
unsafe extern "C" fn frame_fin(
    mut rstate: *const nghttp3_stream_read_state,
    mut len: size_t,
) -> ::core::ffi::c_int {
    return (len >= (*rstate).left as size_t) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_read_control(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut src: *const uint8_t,
    mut srclen: size_t,
    mut ts: nghttp3_tstamp,
) -> nghttp3_ssize {
    let mut p: *const uint8_t = src;
    let mut end: *const uint8_t = src.offset(srclen as isize);
    let mut rv: ::core::ffi::c_int = 0;
    let mut rstate: *mut nghttp3_stream_read_state = &raw mut (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .rstate;
    let mut rvint: *mut nghttp3_varint_read_state = &raw mut (*rstate).rvint;
    let mut nread: nghttp3_ssize = 0;
    let mut nconsumed: size_t = 0 as size_t;
    let mut busy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: size_t = 0;
    let mut pri_field_value: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut pri_field_valuelen: size_t = 0 as size_t;
    '_c2rust_label: {
        if srclen != 0 {} else {
            __assert_fail(
                b"srclen\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                747 as ::core::ffi::c_uint,
                b"nghttp3_ssize nghttp3_conn_read_control(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, nghttp3_tstamp)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    while p != end || busy != 0 {
        busy = 0 as ::core::ffi::c_int;
        's_1391: {
            'c_8667: {
                'c_8693: {
                    match (*rstate).state {
                        0 => {
                            '_c2rust_label_0: {
                                if end.offset_from(p) > 0isize {} else {
                                    __assert_fail(
                                        b"end - p > 0\0".as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        753 as ::core::ffi::c_uint,
                                        b"nghttp3_ssize nghttp3_conn_read_control(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, nghttp3_tstamp)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            nread = nghttp3_read_varint(
                                rvint,
                                p,
                                end,
                                0 as ::core::ffi::c_int,
                            );
                            if nread < 0 as nghttp3_ssize {
                                return NGHTTP3_ERR_H3_GENERAL_PROTOCOL_ERROR
                                    as nghttp3_ssize;
                            }
                            p = p.offset(nread as isize);
                            nconsumed = nconsumed.wrapping_add(nread as size_t);
                            if (*rvint).left != 0 {
                                return nconsumed as nghttp3_ssize;
                            }
                            (*rstate).fr.hd.r#type = (*rvint).acc;
                            nghttp3_varint_read_state_reset(rvint);
                            (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_FRAME_LENGTH
                                .0 as ::core::ffi::c_int;
                            if p == end {
                                return nconsumed as nghttp3_ssize;
                            }
                            break 'c_8667;
                        }
                        1 => {
                            break 'c_8667;
                        }
                        2 => {
                            loop {
                                if (*rstate).left == 0 as uint64_t {
                                    rv = conn_call_recv_settings(conn);
                                    if rv != 0 as ::core::ffi::c_int {
                                        return rv as nghttp3_ssize;
                                    }
                                    nghttp3_stream_read_state_reset(rstate);
                                    break;
                                } else {
                                    if p == end {
                                        return nconsumed as nghttp3_ssize;
                                    }
                                    len = nghttp3_min_unsigned_long_int(
                                        (*rstate).left as ::core::ffi::c_ulong,
                                        end.offset_from(p) as ::core::ffi::c_ulong,
                                    ) as size_t;
                                    '_c2rust_label_2: {
                                        if len > 0 as size_t {} else {
                                            __assert_fail(
                                                b"len > 0\0".as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                914 as ::core::ffi::c_uint,
                                                b"nghttp3_ssize nghttp3_conn_read_control(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, nghttp3_tstamp)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                            );
                                        }
                                    };
                                    nread = nghttp3_read_varint(
                                        rvint,
                                        p,
                                        p.offset(len as isize),
                                        frame_fin(rstate, len),
                                    );
                                    if nread < 0 as nghttp3_ssize {
                                        return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                                    }
                                    p = p.offset(nread as isize);
                                    nconsumed = nconsumed.wrapping_add(nread as size_t);
                                    (*rstate).left = (*rstate)
                                        .left
                                        .wrapping_sub(nread as uint64_t);
                                    if (*rvint).left != 0 {
                                        (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_SETTINGS_ID
                                            .0 as ::core::ffi::c_int;
                                        return nconsumed as nghttp3_ssize;
                                    }
                                    (*(*rstate).fr.settings.iv.offset(0isize)).id = (*rvint)
                                        .acc;
                                    nghttp3_varint_read_state_reset(rvint);
                                    if (*rstate).left == 0 as uint64_t {
                                        return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                                    }
                                    len = len.wrapping_sub(nread as size_t);
                                    if len == 0 as size_t {
                                        (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_SETTINGS_VALUE
                                            .0 as ::core::ffi::c_int;
                                        break;
                                    } else {
                                        nread = nghttp3_read_varint(
                                            rvint,
                                            p,
                                            p.offset(len as isize),
                                            frame_fin(rstate, len),
                                        );
                                        if nread < 0 as nghttp3_ssize {
                                            return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                                        }
                                        p = p.offset(nread as isize);
                                        nconsumed = nconsumed.wrapping_add(nread as size_t);
                                        (*rstate).left = (*rstate)
                                            .left
                                            .wrapping_sub(nread as uint64_t);
                                        if (*rvint).left != 0 {
                                            (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_SETTINGS_VALUE
                                                .0 as ::core::ffi::c_int;
                                            return nconsumed as nghttp3_ssize;
                                        }
                                        (*(*rstate).fr.settings.iv.offset(0isize)).value = (*rvint)
                                            .acc;
                                        nghttp3_varint_read_state_reset(rvint);
                                        rv = nghttp3_conn_on_settings_entry_received(
                                            conn,
                                            &raw mut (*rstate).fr.settings,
                                        );
                                        if rv != 0 as ::core::ffi::c_int {
                                            return rv as nghttp3_ssize;
                                        }
                                    }
                                }
                            }
                            break 's_1391;
                        }
                        6 => {
                            len = nghttp3_min_unsigned_long_int(
                                (*rstate).left as ::core::ffi::c_ulong,
                                end.offset_from(p) as ::core::ffi::c_ulong,
                            ) as size_t;
                            '_c2rust_label_3: {
                                if len > 0 as size_t {} else {
                                    __assert_fail(
                                        b"len > 0\0".as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        965 as ::core::ffi::c_uint,
                                        b"nghttp3_ssize nghttp3_conn_read_control(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, nghttp3_tstamp)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            nread = nghttp3_read_varint(
                                rvint,
                                p,
                                p.offset(len as isize),
                                frame_fin(rstate, len),
                            );
                            if nread < 0 as nghttp3_ssize {
                                return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                            }
                            p = p.offset(nread as isize);
                            nconsumed = nconsumed.wrapping_add(nread as size_t);
                            (*rstate).left = (*rstate)
                                .left
                                .wrapping_sub(nread as uint64_t);
                            if (*rvint).left != 0 {
                                return nconsumed as nghttp3_ssize;
                            }
                            (*(*rstate).fr.settings.iv.offset(0isize)).id = (*rvint).acc;
                            nghttp3_varint_read_state_reset(rvint);
                            if (*rstate).left == 0 as uint64_t {
                                return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                            }
                            (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_SETTINGS_VALUE
                                .0 as ::core::ffi::c_int;
                            if p == end {
                                return nconsumed as nghttp3_ssize;
                            }
                            break 'c_8693;
                        }
                        7 => {
                            break 'c_8693;
                        }
                        3 => {
                            len = nghttp3_min_unsigned_long_int(
                                (*rstate).left as ::core::ffi::c_ulong,
                                end.offset_from(p) as ::core::ffi::c_ulong,
                            ) as size_t;
                            '_c2rust_label_5: {
                                if len > 0 as size_t {} else {
                                    __assert_fail(
                                        b"len > 0\0".as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1026 as ::core::ffi::c_uint,
                                        b"nghttp3_ssize nghttp3_conn_read_control(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, nghttp3_tstamp)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            nread = nghttp3_read_varint(
                                rvint,
                                p,
                                p.offset(len as isize),
                                frame_fin(rstate, len),
                            );
                            if nread < 0 as nghttp3_ssize {
                                return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                            }
                            p = p.offset(nread as isize);
                            nconsumed = nconsumed.wrapping_add(nread as size_t);
                            (*rstate).left = (*rstate)
                                .left
                                .wrapping_sub(nread as uint64_t);
                            if (*rvint).left != 0 {
                                return nconsumed as nghttp3_ssize;
                            }
                            if (*conn).server == 0
                                && nghttp3_client_stream_bidi((*rvint).acc as int64_t) == 0
                            {
                                return NGHTTP3_ERR_H3_ID_ERROR as nghttp3_ssize;
                            }
                            if (*conn).rx.goaway_id < (*rvint).acc as int64_t {
                                return NGHTTP3_ERR_H3_ID_ERROR as nghttp3_ssize;
                            }
                            if (*conn).rx.goaway_id == (*rvint).acc as int64_t
                                && conn_glitch_ratelim_drain(conn, 1 as uint64_t, ts)
                                    != 0 as ::core::ffi::c_int
                            {
                                return NGHTTP3_ERR_H3_EXCESSIVE_LOAD as nghttp3_ssize;
                            }
                            (*conn).flags = ((*conn).flags as ::core::ffi::c_uint
                                | NGHTTP3_CONN_FLAG_GOAWAY_RECVED) as uint16_t;
                            (*conn).rx.goaway_id = (*rvint).acc as int64_t;
                            nghttp3_varint_read_state_reset(rvint);
                            if (*conn).callbacks.shutdown.is_some() {
                                rv = (*conn)
                                    .callbacks
                                    .shutdown
                                    .expect(
                                        "non-null function pointer",
                                    )(conn, (*conn).rx.goaway_id, (*conn).user_data);
                                if rv != 0 as ::core::ffi::c_int {
                                    return NGHTTP3_ERR_CALLBACK_FAILURE as nghttp3_ssize;
                                }
                            }
                            nghttp3_stream_read_state_reset(rstate);
                            break 's_1391;
                        }
                        4 => {
                            len = nghttp3_min_unsigned_long_int(
                                (*rstate).left as ::core::ffi::c_ulong,
                                end.offset_from(p) as ::core::ffi::c_ulong,
                            ) as size_t;
                            '_c2rust_label_6: {
                                if len > 0 as size_t {} else {
                                    __assert_fail(
                                        b"len > 0\0".as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1069 as ::core::ffi::c_uint,
                                        b"nghttp3_ssize nghttp3_conn_read_control(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, nghttp3_tstamp)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            nread = nghttp3_read_varint(
                                rvint,
                                p,
                                p.offset(len as isize),
                                frame_fin(rstate, len),
                            );
                            if nread < 0 as nghttp3_ssize {
                                return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                            }
                            p = p.offset(nread as isize);
                            nconsumed = nconsumed.wrapping_add(nread as size_t);
                            (*rstate).left = (*rstate)
                                .left
                                .wrapping_sub(nread as uint64_t);
                            if (*rvint).left != 0 {
                                return nconsumed as nghttp3_ssize;
                            }
                            if (*conn).local.uni.max_pushes
                                > (*rvint).acc.wrapping_add(1 as uint64_t)
                            {
                                return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                            }
                            if (*conn).local.uni.max_pushes
                                == (*rvint).acc.wrapping_add(1 as uint64_t)
                                && conn_glitch_ratelim_drain(conn, 1 as uint64_t, ts)
                                    != 0 as ::core::ffi::c_int
                            {
                                return NGHTTP3_ERR_H3_EXCESSIVE_LOAD as nghttp3_ssize;
                            }
                            (*conn).local.uni.max_pushes = (*rvint)
                                .acc
                                .wrapping_add(1 as uint64_t);
                            nghttp3_varint_read_state_reset(rvint);
                            nghttp3_stream_read_state_reset(rstate);
                            break 's_1391;
                        }
                        8 => {
                            len = nghttp3_min_unsigned_long_int(
                                (*rstate).left as ::core::ffi::c_ulong,
                                end.offset_from(p) as ::core::ffi::c_ulong,
                            ) as size_t;
                            '_c2rust_label_7: {
                                if len > 0 as size_t {} else {
                                    __assert_fail(
                                        b"len > 0\0".as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1100 as ::core::ffi::c_uint,
                                        b"nghttp3_ssize nghttp3_conn_read_control(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, nghttp3_tstamp)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            nread = nghttp3_read_varint(
                                rvint,
                                p,
                                p.offset(len as isize),
                                frame_fin(rstate, len),
                            );
                            if nread < 0 as nghttp3_ssize {
                                return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                            }
                            p = p.offset(nread as isize);
                            nconsumed = nconsumed.wrapping_add(nread as size_t);
                            (*rstate).left = (*rstate)
                                .left
                                .wrapping_sub(nread as uint64_t);
                            if (*rvint).left != 0 {
                                return nconsumed as nghttp3_ssize;
                            }
                            (*rstate).fr.priority_update.pri_elem_id = (*rvint).acc
                                as int64_t;
                            nghttp3_varint_read_state_reset(rvint);
                            if (*rstate).left == 0 as uint64_t {
                                (*rstate).fr.priority_update.c2rust_unnamed.pri = nghttp3_pri(C2Rust_nghttp3_pri_Inner {
                                    urgency: NGHTTP3_DEFAULT_URGENCY as uint32_t,
                                    inc: 0,
                                });
                                rv = nghttp3_conn_on_priority_update(
                                    conn,
                                    &raw mut (*rstate).fr.priority_update,
                                );
                                if rv != 0 as ::core::ffi::c_int {
                                    return rv as nghttp3_ssize;
                                }
                                nghttp3_stream_read_state_reset(rstate);
                                break 's_1391;
                            } else {
                                (*conn).rx.c2rust_unnamed.c2rust_unnamed.pri_fieldbuflen = 0
                                    as size_t;
                                (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_PRIORITY_UPDATE
                                    .0 as ::core::ffi::c_int;
                                if p == end {
                                    return nconsumed as nghttp3_ssize;
                                }
                            }
                        }
                        9 => {}
                        10 => {
                            len = nghttp3_min_unsigned_long_int(
                                (*rstate).left as ::core::ffi::c_ulong,
                                end.offset_from(p) as ::core::ffi::c_ulong,
                            ) as size_t;
                            '_c2rust_label_9: {
                                if len > 0 as size_t {} else {
                                    __assert_fail(
                                        b"len > 0\0".as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1199 as ::core::ffi::c_uint,
                                        b"nghttp3_ssize nghttp3_conn_read_control(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, nghttp3_tstamp)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            loop {
                                nread = 0 as nghttp3_ssize;
                                while (*conn)
                                    .rx
                                    .c2rust_unnamed
                                    .c2rust_unnamed_0
                                    .originlen_offset < ::core::mem::size_of::<uint16_t>()
                                    && (nread as size_t) < len
                                {
                                    (*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen = ((*conn)
                                        .rx
                                        .c2rust_unnamed
                                        .c2rust_unnamed_0
                                        .originlen as ::core::ffi::c_int
                                        * 256 as ::core::ffi::c_int) as uint16_t;
                                    let c2rust_fresh0 = p;
                                    p = p.offset(1);
                                    (*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen = ((*conn)
                                        .rx
                                        .c2rust_unnamed
                                        .c2rust_unnamed_0
                                        .originlen as ::core::ffi::c_int
                                        + *c2rust_fresh0 as ::core::ffi::c_int) as uint16_t;
                                    (*conn)
                                        .rx
                                        .c2rust_unnamed
                                        .c2rust_unnamed_0
                                        .originlen_offset = (*conn)
                                        .rx
                                        .c2rust_unnamed
                                        .c2rust_unnamed_0
                                        .originlen_offset
                                        .wrapping_add(1);
                                    nread += 1;
                                }
                                nconsumed = nconsumed.wrapping_add(nread as size_t);
                                (*rstate).left = (*rstate)
                                    .left
                                    .wrapping_sub(nread as uint64_t);
                                len = len.wrapping_sub(nread as size_t);
                                if (*conn)
                                    .rx
                                    .c2rust_unnamed
                                    .c2rust_unnamed_0
                                    .originlen_offset < ::core::mem::size_of::<uint16_t>()
                                {
                                    if (*rstate).left == 0 as uint64_t {
                                        return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                                    }
                                    return nconsumed as nghttp3_ssize;
                                }
                                if (*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen
                                    as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                    || (*rstate).left
                                        < (*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen
                                            as uint64_t
                                {
                                    return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                                }
                                if len
                                    < (*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen
                                        as size_t
                                {
                                    if (*conn).rx.originbuf.is_null() {
                                        (*conn).rx.originbuf = nghttp3_mem_malloc(
                                            (*conn).mem,
                                            UINT16_MAX as size_t,
                                        ) as *mut uint8_t;
                                        if (*conn).rx.originbuf.is_null() {
                                            return NGHTTP3_ERR_NOMEM as nghttp3_ssize;
                                        }
                                    }
                                    memcpy(
                                        (*conn).rx.originbuf as *mut ::core::ffi::c_void,
                                        p as *const ::core::ffi::c_void,
                                        len,
                                    );
                                    nconsumed = nconsumed.wrapping_add(len);
                                    (*rstate).left = ((*rstate).left as ::core::ffi::c_ulong)
                                        .wrapping_sub(len as ::core::ffi::c_ulong) as uint64_t;
                                    (*conn).rx.originbuflen = len;
                                    (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_ORIGIN_ASCII_ORIGIN
                                        .0 as ::core::ffi::c_int;
                                    return nconsumed as nghttp3_ssize;
                                }
                                rv = conn_call_recv_origin(
                                    conn,
                                    p,
                                    (*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen
                                        as size_t,
                                );
                                if rv != 0 as ::core::ffi::c_int {
                                    return rv as nghttp3_ssize;
                                }
                                p = p
                                    .offset(
                                        (*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen
                                            as ::core::ffi::c_int as isize,
                                    );
                                nconsumed = nconsumed
                                    .wrapping_add(
                                        (*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen
                                            as size_t,
                                    );
                                (*rstate).left = (*rstate)
                                    .left
                                    .wrapping_sub(
                                        (*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen
                                            as uint64_t,
                                    );
                                if (*rstate).left == 0 as uint64_t {
                                    rv = conn_call_end_origin(conn);
                                    if rv != 0 as ::core::ffi::c_int {
                                        return rv as nghttp3_ssize;
                                    }
                                    nghttp3_stream_read_state_reset(rstate);
                                    break;
                                } else {
                                    len = len
                                        .wrapping_sub(
                                            (*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen
                                                as size_t,
                                        );
                                    conn_reset_rx_originlen(conn);
                                    if p == end {
                                        return nconsumed as nghttp3_ssize;
                                    }
                                }
                            }
                            break 's_1391;
                        }
                        11 => {
                            len = nghttp3_min_unsigned_long_int(
                                ((*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen
                                    as ::core::ffi::c_ulong)
                                    .wrapping_sub(
                                        (*conn).rx.originbuflen as ::core::ffi::c_ulong,
                                    ),
                                end.offset_from(p) as ::core::ffi::c_ulong,
                            ) as size_t;
                            '_c2rust_label_10: {
                                if len > 0 as size_t {} else {
                                    __assert_fail(
                                        b"len > 0\0".as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1288 as ::core::ffi::c_uint,
                                        b"nghttp3_ssize nghttp3_conn_read_control(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, nghttp3_tstamp)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            memcpy(
                                (*conn)
                                    .rx
                                    .originbuf
                                    .offset((*conn).rx.originbuflen as isize)
                                    as *mut ::core::ffi::c_void,
                                p as *const ::core::ffi::c_void,
                                len,
                            );
                            (*conn).rx.originbuflen = (*conn)
                                .rx
                                .originbuflen
                                .wrapping_add(len);
                            p = p.offset(len as isize);
                            nconsumed = nconsumed.wrapping_add(len);
                            (*rstate).left = ((*rstate).left as ::core::ffi::c_ulong)
                                .wrapping_sub(len as ::core::ffi::c_ulong) as uint64_t;
                            if (*conn).rx.originbuflen
                                < (*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen
                                    as size_t
                            {
                                return nconsumed as nghttp3_ssize;
                            }
                            rv = conn_call_recv_origin(
                                conn,
                                (*conn).rx.originbuf,
                                (*conn).rx.c2rust_unnamed.c2rust_unnamed_0.originlen
                                    as size_t,
                            );
                            if rv != 0 as ::core::ffi::c_int {
                                return rv as nghttp3_ssize;
                            }
                            if (*rstate).left != 0 {
                                conn_reset_rx_originlen(conn);
                                (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_ORIGIN_ORIGIN_LEN
                                    .0 as ::core::ffi::c_int;
                                break 's_1391;
                            } else {
                                rv = conn_call_end_origin(conn);
                                if rv != 0 as ::core::ffi::c_int {
                                    return rv as nghttp3_ssize;
                                }
                                nghttp3_stream_read_state_reset(rstate);
                                break 's_1391;
                            }
                        }
                        5 => {
                            len = nghttp3_min_unsigned_long_int(
                                (*rstate).left as ::core::ffi::c_ulong,
                                end.offset_from(p) as ::core::ffi::c_ulong,
                            ) as size_t;
                            p = p.offset(len as isize);
                            nconsumed = nconsumed.wrapping_add(len);
                            (*rstate).left = ((*rstate).left as ::core::ffi::c_ulong)
                                .wrapping_sub(len as ::core::ffi::c_ulong) as uint64_t;
                            if (*rstate).left != 0 {
                                return nconsumed as nghttp3_ssize;
                            }
                            nghttp3_stream_read_state_reset(rstate);
                            break 's_1391;
                        }
                        _ => {
                            nghttp3_unreachable_fail(
                                b"nghttp3_conn.c\0".as_ptr() as *const ::core::ffi::c_char,
                                1335 as ::core::ffi::c_int,
                                b"nghttp3_conn_read_control\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    len = nghttp3_min_unsigned_long_int(
                        (*rstate).left as ::core::ffi::c_ulong,
                        end.offset_from(p) as ::core::ffi::c_ulong,
                    ) as size_t;
                    '_c2rust_label_8: {
                        if len > 0 as size_t {} else {
                            __assert_fail(
                                b"len > 0\0".as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1143 as ::core::ffi::c_uint,
                                b"nghttp3_ssize nghttp3_conn_read_control(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, nghttp3_tstamp)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    if (*conn).rx.c2rust_unnamed.c2rust_unnamed.pri_fieldbuflen
                        == 0 as size_t && (*rstate).left == len as uint64_t
                    {
                        if len > ::core::mem::size_of::<[uint8_t; 8]>() {
                            busy = 1 as ::core::ffi::c_int;
                            (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_IGN_FRAME
                                .0 as ::core::ffi::c_int;
                            break 's_1391;
                        } else {
                            pri_field_value = p;
                            pri_field_valuelen = len;
                        }
                    } else if len
                        .wrapping_add(
                            (*conn).rx.c2rust_unnamed.c2rust_unnamed.pri_fieldbuflen,
                        ) > ::core::mem::size_of::<[uint8_t; 8]>()
                    {
                        busy = 1 as ::core::ffi::c_int;
                        (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_IGN_FRAME
                            .0 as ::core::ffi::c_int;
                        break 's_1391;
                    } else {
                        memcpy(
                            (&raw mut (*conn)
                                .rx
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .pri_fieldbuf as *mut uint8_t)
                                .offset(
                                    (*conn).rx.c2rust_unnamed.c2rust_unnamed.pri_fieldbuflen
                                        as isize,
                                ) as *mut ::core::ffi::c_void,
                            p as *const ::core::ffi::c_void,
                            len,
                        );
                        (*conn).rx.c2rust_unnamed.c2rust_unnamed.pri_fieldbuflen = (*conn)
                            .rx
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .pri_fieldbuflen
                            .wrapping_add(len);
                        if (*rstate).left == len as uint64_t {
                            pri_field_value = &raw mut (*conn)
                                .rx
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .pri_fieldbuf as *mut uint8_t;
                            pri_field_valuelen = (*conn)
                                .rx
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .pri_fieldbuflen;
                        }
                    }
                    p = p.offset(len as isize);
                    nconsumed = nconsumed.wrapping_add(len);
                    (*rstate).left = ((*rstate).left as ::core::ffi::c_ulong)
                        .wrapping_sub(len as ::core::ffi::c_ulong) as uint64_t;
                    if (*rstate).left != 0 {
                        return nconsumed as nghttp3_ssize;
                    }
                    (*rstate).fr.priority_update.c2rust_unnamed.pri = nghttp3_pri(C2Rust_nghttp3_pri_Inner {
                        urgency: NGHTTP3_DEFAULT_URGENCY as uint32_t,
                        inc: 0,
                    });
                    if nghttp3_http_parse_priority(
                        &raw mut (*rstate).fr.priority_update.c2rust_unnamed.pri,
                        pri_field_value,
                        pri_field_valuelen,
                    ) != 0 as ::core::ffi::c_int
                    {
                        return NGHTTP3_ERR_H3_GENERAL_PROTOCOL_ERROR as nghttp3_ssize;
                    }
                    rv = nghttp3_conn_on_priority_update(
                        conn,
                        &raw mut (*rstate).fr.priority_update,
                    );
                    if rv != 0 as ::core::ffi::c_int {
                        return rv as nghttp3_ssize;
                    }
                    nghttp3_stream_read_state_reset(rstate);
                    break 's_1391;
                }
                len = nghttp3_min_unsigned_long_int(
                    (*rstate).left as ::core::ffi::c_ulong,
                    end.offset_from(p) as ::core::ffi::c_ulong,
                ) as size_t;
                '_c2rust_label_4: {
                    if len > 0 as size_t {} else {
                        __assert_fail(
                            b"len > 0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            992 as ::core::ffi::c_uint,
                            b"nghttp3_ssize nghttp3_conn_read_control(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, nghttp3_tstamp)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                nread = nghttp3_read_varint(
                    rvint,
                    p,
                    p.offset(len as isize),
                    frame_fin(rstate, len),
                );
                if nread < 0 as nghttp3_ssize {
                    return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                }
                p = p.offset(nread as isize);
                nconsumed = nconsumed.wrapping_add(nread as size_t);
                (*rstate).left = (*rstate).left.wrapping_sub(nread as uint64_t);
                if (*rvint).left != 0 {
                    return nconsumed as nghttp3_ssize;
                }
                (*(*rstate).fr.settings.iv.offset(0isize)).value = (*rvint).acc;
                nghttp3_varint_read_state_reset(rvint);
                rv = nghttp3_conn_on_settings_entry_received(
                    conn,
                    &raw mut (*rstate).fr.settings,
                );
                if rv != 0 as ::core::ffi::c_int {
                    return rv as nghttp3_ssize;
                }
                if (*rstate).left != 0 {
                    (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_SETTINGS
                        .0 as ::core::ffi::c_int;
                    break 's_1391;
                } else {
                    rv = conn_call_recv_settings(conn);
                    if rv != 0 as ::core::ffi::c_int {
                        return rv as nghttp3_ssize;
                    }
                    nghttp3_stream_read_state_reset(rstate);
                    break 's_1391;
                }
            }
            '_c2rust_label_1: {
                if end.offset_from(p) > 0isize {} else {
                    __assert_fail(
                        b"end - p > 0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        773 as ::core::ffi::c_uint,
                        b"nghttp3_ssize nghttp3_conn_read_control(nghttp3_conn *, nghttp3_stream *, const uint8_t *, size_t, nghttp3_tstamp)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            nread = nghttp3_read_varint(rvint, p, end, 0 as ::core::ffi::c_int);
            if nread < 0 as nghttp3_ssize {
                return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
            }
            p = p.offset(nread as isize);
            nconsumed = nconsumed.wrapping_add(nread as size_t);
            if (*rvint).left != 0 {
                return nconsumed as nghttp3_ssize;
            }
            (*rstate).left = (*rvint).acc;
            nghttp3_varint_read_state_reset(rvint);
            if (*conn).flags as ::core::ffi::c_uint & NGHTTP3_CONN_FLAG_SETTINGS_RECVED
                == 0
            {
                if (*rstate).fr.hd.r#type != NGHTTP3_FRAME_SETTINGS as uint64_t {
                    return NGHTTP3_ERR_H3_MISSING_SETTINGS as nghttp3_ssize;
                }
                (*conn).flags = ((*conn).flags as ::core::ffi::c_uint
                    | NGHTTP3_CONN_FLAG_SETTINGS_RECVED) as uint16_t;
            } else if (*rstate).fr.hd.r#type == NGHTTP3_FRAME_SETTINGS as uint64_t {
                return NGHTTP3_ERR_H3_FRAME_UNEXPECTED as nghttp3_ssize
            }
            match (*rstate).fr.hd.r#type {
                0x4 => {
                    if (*rstate).left == 0 as uint64_t {
                        rv = conn_call_recv_settings(conn);
                        if rv != 0 as ::core::ffi::c_int {
                            return rv as nghttp3_ssize;
                        }
                        nghttp3_stream_read_state_reset(rstate);
                    } else {
                        (*rstate).fr.settings.iv = &raw mut (*rstate).iv;
                        (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_SETTINGS
                            .0 as ::core::ffi::c_int;
                    }
                }
                0x7 => {
                    if (*rstate).left == 0 as uint64_t {
                        return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                    }
                    (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_GOAWAY
                        .0 as ::core::ffi::c_int;
                }
                NGHTTP3_FRAME_MAX_PUSH_ID => {
                    if (*conn).server == 0 {
                        return NGHTTP3_ERR_H3_FRAME_UNEXPECTED as nghttp3_ssize;
                    }
                    if (*rstate).left == 0 as uint64_t {
                        return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                    }
                    (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_MAX_PUSH_ID
                        .0 as ::core::ffi::c_int;
                }
                0xf0700 => {
                    if (*conn).server == 0 {
                        return NGHTTP3_ERR_H3_FRAME_UNEXPECTED as nghttp3_ssize;
                    }
                    if (*rstate).left == 0 as uint64_t {
                        return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
                    }
                    if conn_glitch_ratelim_drain(conn, 1 as uint64_t, ts)
                        != 0 as ::core::ffi::c_int
                    {
                        return NGHTTP3_ERR_H3_EXCESSIVE_LOAD as nghttp3_ssize;
                    }
                    (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_PRIORITY_UPDATE_PRI_ELEM_ID
                        .0 as ::core::ffi::c_int;
                }
                NGHTTP3_FRAME_PRIORITY_UPDATE_PUSH_ID => {
                    return NGHTTP3_ERR_H3_ID_ERROR as nghttp3_ssize;
                }
                0xc => {
                    if conn_glitch_ratelim_drain(conn, 1 as uint64_t, ts)
                        != 0 as ::core::ffi::c_int
                    {
                        return NGHTTP3_ERR_H3_EXCESSIVE_LOAD as nghttp3_ssize;
                    }
                    if (*conn).server != 0
                        || (*conn).callbacks.recv_origin.is_none()
                            && (*conn).callbacks.end_origin.is_none()
                    {
                        busy = 1 as ::core::ffi::c_int;
                        (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_IGN_FRAME
                            .0 as ::core::ffi::c_int;
                    } else if (*rstate).left == 0 as uint64_t {
                        rv = conn_call_end_origin(conn);
                        if rv != 0 as ::core::ffi::c_int {
                            return rv as nghttp3_ssize;
                        }
                        nghttp3_stream_read_state_reset(rstate);
                    } else {
                        conn_reset_rx_originlen(conn);
                        (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_ORIGIN_ORIGIN_LEN
                            .0 as ::core::ffi::c_int;
                    }
                }
                NGHTTP3_FRAME_CANCEL_PUSH
                | 0
                | 0x1
                | NGHTTP3_FRAME_PUSH_PROMISE
                | NGHTTP3_H2_FRAME_PRIORITY
                | NGHTTP3_H2_FRAME_PING
                | NGHTTP3_H2_FRAME_WINDOW_UPDATE
                | NGHTTP3_H2_FRAME_CONTINUATION => {
                    return NGHTTP3_ERR_H3_FRAME_UNEXPECTED as nghttp3_ssize;
                }
                _ => {
                    if conn_glitch_ratelim_drain(conn, 1 as uint64_t, ts)
                        != 0 as ::core::ffi::c_int
                    {
                        return NGHTTP3_ERR_H3_EXCESSIVE_LOAD as nghttp3_ssize;
                    }
                    busy = 1 as ::core::ffi::c_int;
                    (*rstate).state = nghttp3_ctrl_stream_state::NGHTTP3_CTRL_STREAM_STATE_IGN_FRAME
                        .0 as ::core::ffi::c_int;
                }
            }
        }
    }
    return nconsumed as nghttp3_ssize;
}
unsafe extern "C" fn conn_delete_stream(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut flags: uint32_t,
    mut rx_app_error_code: uint64_t,
    mut tx_app_error_code: uint64_t,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    let mut app_error_code: uint64_t = 0;
    rv = conn_call_deferred_consume(
        conn,
        stream,
        nghttp3_stream_get_buffered_datalen(stream),
    );
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    if (*stream).c2rust_unnamed.c2rust_unnamed.qpack_blocked_pe.index
        != NGHTTP3_PQ_BAD_INDEX as size_t
    {
        nghttp3_conn_qpack_blocked_streams_remove(conn, stream);
        rv = nghttp3_qpack_decoder_cancel_stream(
            &raw mut (*conn).qdec,
            (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
        );
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
    }
    if (*conn).callbacks.stream_close2.is_some() {
        rv = (*conn)
            .callbacks
            .stream_close2
            .expect(
                "non-null function pointer",
            )(
            conn,
            flags,
            (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
            rx_app_error_code,
            tx_app_error_code,
            (*conn).user_data,
            (*stream).c2rust_unnamed.c2rust_unnamed.user_data,
        );
        if rv != 0 as ::core::ffi::c_int {
            return NGHTTP3_ERR_CALLBACK_FAILURE;
        }
    } else if (*conn).callbacks.stream_close.is_some() {
        app_error_code = NGHTTP3_H3_NO_ERROR as uint64_t;
        if flags & NGHTTP3_STREAM_CLOSE_FLAG_RX_APP_ERROR_CODE_SET as uint32_t != 0 {
            app_error_code = rx_app_error_code;
        }
        if app_error_code == NGHTTP3_H3_NO_ERROR as uint64_t
            && flags & NGHTTP3_STREAM_CLOSE_FLAG_TX_APP_ERROR_CODE_SET as uint32_t != 0
        {
            app_error_code = tx_app_error_code;
        }
        rv = (*conn)
            .callbacks
            .stream_close
            .expect(
                "non-null function pointer",
            )(
            conn,
            (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
            app_error_code,
            (*conn).user_data,
            (*stream).c2rust_unnamed.c2rust_unnamed.user_data,
        );
        if rv != 0 as ::core::ffi::c_int {
            return NGHTTP3_ERR_CALLBACK_FAILURE;
        }
    }
    if (*conn).server != 0
        && nghttp3_client_stream_bidi((*stream).c2rust_unnamed.c2rust_unnamed.node.id)
            != 0
    {
        '_c2rust_label: {
            if (*conn).remote.bidi.num_streams > 0 as size_t {} else {
                __assert_fail(
                    b"conn->remote.bidi.num_streams > 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1390 as ::core::ffi::c_uint,
                    b"int conn_delete_stream(nghttp3_conn *, nghttp3_stream *, uint32_t, uint64_t, uint64_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        (*conn).remote.bidi.num_streams = (*conn)
            .remote
            .bidi
            .num_streams
            .wrapping_sub(1);
    }
    rv = nghttp3_map_remove(
        &raw mut (*conn).streams,
        (*stream).c2rust_unnamed.c2rust_unnamed.node.id as nghttp3_map_key_type,
    );
    '_c2rust_label_0: {
        if 0 as ::core::ffi::c_int == rv {} else {
            __assert_fail(
                b"0 == rv\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1398 as ::core::ffi::c_uint,
                b"int conn_delete_stream(nghttp3_conn *, nghttp3_stream *, uint32_t, uint64_t, uint64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    nghttp3_stream_del(stream);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_process_blocked_stream_data(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut ts: nghttp3_tstamp,
) -> ::core::ffi::c_int {
    let mut buf: *mut nghttp3_buf = ::core::ptr::null_mut::<nghttp3_buf>();
    let mut nproc: size_t = 0;
    let mut nconsumed: nghttp3_ssize = 0;
    let mut rv: ::core::ffi::c_int = 0;
    let mut len: size_t = 0;
    '_c2rust_label: {
        if nghttp3_client_stream_bidi((*stream).c2rust_unnamed.c2rust_unnamed.node.id)
            != 0
        {} else {
            __assert_fail(
                b"nghttp3_client_stream_bidi(stream->node.id)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1414 as ::core::ffi::c_uint,
                b"int conn_process_blocked_stream_data(nghttp3_conn *, nghttp3_stream *, nghttp3_tstamp)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    loop {
        len = nghttp3_ringbuf_len(&raw mut (*stream).c2rust_unnamed.c2rust_unnamed.inq);
        if len == 0 as size_t {
            break;
        }
        buf = nghttp3_ringbuf_get(
            &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.inq,
            0 as size_t,
        ) as *mut nghttp3_buf;
        nconsumed = nghttp3_conn_read_bidi(
            conn,
            &raw mut nproc,
            stream,
            (*buf).pos,
            nghttp3_buf_len(buf),
            (len == 1 as size_t
                && (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
                    & NGHTTP3_STREAM_FLAG_READ_EOF != 0) as ::core::ffi::c_int,
            ts,
        );
        if nconsumed < 0 as nghttp3_ssize {
            return nconsumed as ::core::ffi::c_int;
        }
        (*buf).pos = (*buf).pos.offset(nproc as isize);
        rv = conn_call_deferred_consume(conn, stream, nconsumed as size_t);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
        if nghttp3_buf_len(buf) == 0 as size_t {
            nghttp3_buf_free(buf, (*stream).c2rust_unnamed.c2rust_unnamed.mem);
            nghttp3_ringbuf_pop_front(
                &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.inq,
            );
        }
        if (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
            & NGHTTP3_STREAM_FLAG_QPACK_DECODE_BLOCKED != 0
        {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_read_qpack_encoder(
    mut conn: *mut nghttp3_conn,
    mut src: *const uint8_t,
    mut srclen: size_t,
    mut ts: nghttp3_tstamp,
) -> nghttp3_ssize {
    let mut nconsumed: nghttp3_ssize = nghttp3_qpack_decoder_read_encoder(
        &raw mut (*conn).qdec,
        src,
        srclen,
    );
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    let mut rv: ::core::ffi::c_int = 0;
    if nconsumed < 0 as nghttp3_ssize {
        return nconsumed;
    }
    while nghttp3_pq_empty(&raw mut (*conn).qpack_blocked_streams) == 0 {
        stream = (nghttp3_pq_top(&raw mut (*conn).qpack_blocked_streams)
            as *mut ::core::ffi::c_char)
            .offset(-(56 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
            as *mut nghttp3_stream;
        if nghttp3_qpack_stream_context_get_ricnt2(
            &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.qpack_sctx,
        ) > nghttp3_qpack_decoder_get_icnt(&raw mut (*conn).qdec)
        {
            break;
        }
        nghttp3_conn_qpack_blocked_streams_pop(conn);
        (*stream).c2rust_unnamed.c2rust_unnamed.qpack_blocked_pe.index = NGHTTP3_PQ_BAD_INDEX
            as size_t;
        (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
            .c2rust_unnamed
            .c2rust_unnamed
            .flags as ::core::ffi::c_int
            & !NGHTTP3_STREAM_FLAG_QPACK_DECODE_BLOCKED as uint16_t
                as ::core::ffi::c_int) as uint16_t;
        rv = conn_process_blocked_stream_data(conn, stream, ts);
        if rv != 0 as ::core::ffi::c_int {
            return rv as nghttp3_ssize;
        }
    }
    return nconsumed;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_read_qpack_decoder(
    mut conn: *mut nghttp3_conn,
    mut src: *const uint8_t,
    mut srclen: size_t,
) -> nghttp3_ssize {
    return nghttp3_qpack_encoder_read_decoder(&raw mut (*conn).qenc, src, srclen);
}
unsafe extern "C" fn stream_get_sched_node(
    mut stream: *mut nghttp3_stream,
) -> *mut nghttp3_tnode {
    return &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.node;
}
unsafe extern "C" fn conn_update_stream_priority(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut pri: *const nghttp3_pri,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if nghttp3_client_stream_bidi((*stream).c2rust_unnamed.c2rust_unnamed.node.id)
            != 0
        {} else {
            __assert_fail(
                b"nghttp3_client_stream_bidi(stream->node.id)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1497 as ::core::ffi::c_uint,
                b"int conn_update_stream_priority(nghttp3_conn *, nghttp3_stream *, const nghttp3_pri *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if nghttp3_pri_eq(&raw mut (*stream).c2rust_unnamed.c2rust_unnamed.node.pri, pri)
        != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    nghttp3_conn_unschedule_stream(conn, stream);
    (*stream).c2rust_unnamed.c2rust_unnamed.node.pri = *pri;
    if nghttp3_stream_require_schedule(stream) != 0 {
        return nghttp3_conn_schedule_stream(conn, stream);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_read_bidi(
    mut conn: *mut nghttp3_conn,
    mut pnproc: *mut size_t,
    mut stream: *mut nghttp3_stream,
    mut src: *const uint8_t,
    mut srclen: size_t,
    mut fin: ::core::ffi::c_int,
    mut ts: nghttp3_tstamp,
) -> nghttp3_ssize {
    let mut p: *const uint8_t = src;
    let mut end: *const uint8_t = if !src.is_null() {
        src.offset(srclen as isize)
    } else {
        src
    };
    let mut rv: ::core::ffi::c_int = 0;
    let mut rstate: *mut nghttp3_stream_read_state = &raw mut (*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .rstate;
    let mut rvint: *mut nghttp3_varint_read_state = &raw mut (*rstate).rvint;
    let mut nread: nghttp3_ssize = 0;
    let mut nconsumed: size_t = 0 as size_t;
    let mut busy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: size_t = 0;
    if (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
        & NGHTTP3_STREAM_FLAG_SHUT_RD != 0
    {
        *pnproc = srclen;
        return srclen as nghttp3_ssize;
    }
    if (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
        & NGHTTP3_STREAM_FLAG_QPACK_DECODE_BLOCKED != 0
    {
        *pnproc = 0 as size_t;
        if srclen == 0 as size_t {
            return 0 as nghttp3_ssize;
        }
        rv = nghttp3_stream_buffer_data(stream, p, end.offset_from(p) as size_t);
        if rv != 0 as ::core::ffi::c_int {
            return rv as nghttp3_ssize;
        }
        return 0 as nghttp3_ssize;
    }
    while p != end || busy != 0 {
        busy = 0 as ::core::ffi::c_int;
        match (*rstate).state {
            0 => {
                '_c2rust_label: {
                    if end.offset_from(p) > 0isize {} else {
                        __assert_fail(
                            b"end - p > 0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1551 as ::core::ffi::c_uint,
                            b"nghttp3_ssize nghttp3_conn_read_bidi(nghttp3_conn *, size_t *, nghttp3_stream *, const uint8_t *, size_t, int, nghttp3_tstamp)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                nread = nghttp3_read_varint(rvint, p, end, fin);
                if nread < 0 as nghttp3_ssize {
                    return NGHTTP3_ERR_H3_GENERAL_PROTOCOL_ERROR as nghttp3_ssize;
                }
                p = p.offset(nread as isize);
                nconsumed = nconsumed.wrapping_add(nread as size_t);
                if (*rvint).left != 0 {
                    break;
                }
                (*rstate).fr.hd.r#type = (*rvint).acc;
                nghttp3_varint_read_state_reset(rvint);
                (*rstate).state = nghttp3_req_stream_state::NGHTTP3_REQ_STREAM_STATE_FRAME_LENGTH
                    .0 as ::core::ffi::c_int;
                if p == end {
                    break;
                }
            }
            1 => {}
            2 => {
                len = nghttp3_min_unsigned_long_int(
                    (*rstate).left as ::core::ffi::c_ulong,
                    end.offset_from(p) as ::core::ffi::c_ulong,
                ) as size_t;
                rv = nghttp3_conn_on_data(conn, stream, p, len);
                if rv != 0 as ::core::ffi::c_int {
                    return rv as nghttp3_ssize;
                }
                p = p.offset(len as isize);
                (*rstate).left = ((*rstate).left as ::core::ffi::c_ulong)
                    .wrapping_sub(len as ::core::ffi::c_ulong) as uint64_t;
                if (*rstate).left != 0 {
                    break;
                }
                rv = nghttp3_stream_transit_rx_http_state(
                    stream,
                    nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_DATA_END,
                );
                '_c2rust_label_3: {
                    if 0 as ::core::ffi::c_int == rv {} else {
                        __assert_fail(
                            b"0 == rv\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1682 as ::core::ffi::c_uint,
                            b"nghttp3_ssize nghttp3_conn_read_bidi(nghttp3_conn *, size_t *, nghttp3_stream *, const uint8_t *, size_t, int, nghttp3_tstamp)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                nghttp3_stream_read_state_reset(rstate);
                continue;
            }
            3 => {
                len = nghttp3_min_unsigned_long_int(
                    (*rstate).left as ::core::ffi::c_ulong,
                    end.offset_from(p) as ::core::ffi::c_ulong,
                ) as size_t;
                nread = nghttp3_conn_on_headers(
                    conn,
                    stream,
                    p,
                    len,
                    (len == (*rstate).left as size_t) as ::core::ffi::c_int,
                );
                if nread < 0 as nghttp3_ssize {
                    return nread;
                }
                p = p.offset(nread as isize);
                nconsumed = nconsumed.wrapping_add(nread as size_t);
                (*rstate).left = (*rstate).left.wrapping_sub(nread as uint64_t);
                if (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
                    & NGHTTP3_STREAM_FLAG_QPACK_DECODE_BLOCKED != 0
                {
                    if p != end
                        && nghttp3_stream_get_buffered_datalen(stream) == 0 as size_t
                    {
                        rv = nghttp3_stream_buffer_data(
                            stream,
                            p,
                            end.offset_from(p) as size_t,
                        );
                        if rv != 0 as ::core::ffi::c_int {
                            return rv as nghttp3_ssize;
                        }
                    }
                    *pnproc = p.offset_from(src) as size_t;
                    return nconsumed as nghttp3_ssize;
                }
                if (*rstate).left != 0 {
                    break;
                }
                match (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate {
                    nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_HEADERS_BEGIN => {
                        rv = nghttp3_http_on_request_headers(
                            &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.rx.http,
                        );
                    }
                    nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_HEADERS_BEGIN => {
                        rv = nghttp3_http_on_response_headers(
                            &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.rx.http,
                        );
                    }
                    nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_TRAILERS_BEGIN
                    | nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_TRAILERS_BEGIN => {
                        rv = 0 as ::core::ffi::c_int;
                    }
                    _ => {
                        nghttp3_unreachable_fail(
                            b"nghttp3_conn.c\0".as_ptr() as *const ::core::ffi::c_char,
                            1725 as ::core::ffi::c_int,
                            b"nghttp3_conn_read_bidi\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
                if rv != 0 as ::core::ffi::c_int {
                    return rv as nghttp3_ssize;
                }
                's_528: {
                    match (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate {
                        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_HEADERS_BEGIN => {
                            if (*conn).server != 0
                                && (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.flags
                                    & NGHTTP3_HTTP_FLAG_PRIORITY as uint32_t != 0
                                && (*stream).c2rust_unnamed.c2rust_unnamed.flags
                                    as ::core::ffi::c_uint
                                    & NGHTTP3_STREAM_FLAG_PRIORITY_UPDATE_RECVED == 0
                                && (*stream).c2rust_unnamed.c2rust_unnamed.flags
                                    as ::core::ffi::c_uint
                                    & NGHTTP3_STREAM_FLAG_SERVER_PRIORITY_SET == 0
                            {
                                rv = conn_update_stream_priority(
                                    conn,
                                    stream,
                                    &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.rx.http.pri,
                                );
                                if rv != 0 as ::core::ffi::c_int {
                                    return rv as nghttp3_ssize;
                                }
                            }
                        }
                        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_HEADERS_BEGIN => {}
                        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_TRAILERS_BEGIN
                        | nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_TRAILERS_BEGIN => {
                            rv = conn_call_end_trailers(
                                conn,
                                stream,
                                (p == end && fin != 0) as ::core::ffi::c_int,
                            );
                            break 's_528;
                        }
                        _ => {
                            nghttp3_unreachable_fail(
                                b"nghttp3_conn.c\0".as_ptr() as *const ::core::ffi::c_char,
                                1754 as ::core::ffi::c_int,
                                b"nghttp3_conn_read_bidi\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    rv = conn_call_end_headers(
                        conn,
                        stream,
                        (p == end && fin != 0) as ::core::ffi::c_int,
                    );
                }
                if rv != 0 as ::core::ffi::c_int {
                    return rv as nghttp3_ssize;
                }
                rv = nghttp3_stream_transit_rx_http_state(
                    stream,
                    nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_HEADERS_END,
                );
                '_c2rust_label_4: {
                    if 0 as ::core::ffi::c_int == rv {} else {
                        __assert_fail(
                            b"0 == rv\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1763 as ::core::ffi::c_uint,
                            b"nghttp3_ssize nghttp3_conn_read_bidi(nghttp3_conn *, size_t *, nghttp3_stream *, const uint8_t *, size_t, int, nghttp3_tstamp)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                nghttp3_stream_read_state_reset(rstate);
                continue;
            }
            4 => {
                len = nghttp3_min_unsigned_long_int(
                    (*rstate).left as ::core::ffi::c_ulong,
                    end.offset_from(p) as ::core::ffi::c_ulong,
                ) as size_t;
                p = p.offset(len as isize);
                nconsumed = nconsumed.wrapping_add(len);
                (*rstate).left = ((*rstate).left as ::core::ffi::c_ulong)
                    .wrapping_sub(len as ::core::ffi::c_ulong) as uint64_t;
                if (*rstate).left != 0 {
                    break;
                }
                nghttp3_stream_read_state_reset(rstate);
                continue;
            }
            5 => {
                nconsumed = nconsumed.wrapping_add(end.offset_from(p) as size_t);
                *pnproc = end.offset_from(src) as size_t;
                return nconsumed as nghttp3_ssize;
            }
            _ => {
                continue;
            }
        }
        '_c2rust_label_0: {
            if end.offset_from(p) > 0isize {} else {
                __assert_fail(
                    b"end - p > 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1571 as ::core::ffi::c_uint,
                    b"nghttp3_ssize nghttp3_conn_read_bidi(nghttp3_conn *, size_t *, nghttp3_stream *, const uint8_t *, size_t, int, nghttp3_tstamp)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        nread = nghttp3_read_varint(rvint, p, end, fin);
        if nread < 0 as nghttp3_ssize {
            return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize;
        }
        p = p.offset(nread as isize);
        nconsumed = nconsumed.wrapping_add(nread as size_t);
        if (*rvint).left != 0 {
            break;
        }
        (*rstate).left = (*rvint).acc;
        nghttp3_varint_read_state_reset(rvint);
        match (*rstate).fr.hd.r#type {
            0 => {
                rv = nghttp3_stream_transit_rx_http_state(
                    stream,
                    nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_DATA_BEGIN,
                );
                if rv != 0 as ::core::ffi::c_int {
                    return rv as nghttp3_ssize;
                }
                if (*rstate).left == 0 as uint64_t {
                    rv = nghttp3_stream_transit_rx_http_state(
                        stream,
                        nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_DATA_END,
                    );
                    '_c2rust_label_1: {
                        if 0 as ::core::ffi::c_int == rv {} else {
                            __assert_fail(
                                b"0 == rv\0".as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1597 as ::core::ffi::c_uint,
                                b"nghttp3_ssize nghttp3_conn_read_bidi(nghttp3_conn *, size_t *, nghttp3_stream *, const uint8_t *, size_t, int, nghttp3_tstamp)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    nghttp3_stream_read_state_reset(rstate);
                } else {
                    (*rstate).state = nghttp3_req_stream_state::NGHTTP3_REQ_STREAM_STATE_DATA
                        .0 as ::core::ffi::c_int;
                }
            }
            0x1 => {
                rv = nghttp3_stream_transit_rx_http_state(
                    stream,
                    nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_HEADERS_BEGIN,
                );
                if rv != 0 as ::core::ffi::c_int {
                    return rv as nghttp3_ssize;
                }
                if (*rstate).left == 0 as uint64_t {
                    rv = nghttp3_stream_empty_headers_allowed(stream);
                    if rv != 0 as ::core::ffi::c_int {
                        return rv as nghttp3_ssize;
                    }
                    rv = nghttp3_stream_transit_rx_http_state(
                        stream,
                        nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_HEADERS_END,
                    );
                    '_c2rust_label_2: {
                        if 0 as ::core::ffi::c_int == rv {} else {
                            __assert_fail(
                                b"0 == rv\0".as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1618 as ::core::ffi::c_uint,
                                b"nghttp3_ssize nghttp3_conn_read_bidi(nghttp3_conn *, size_t *, nghttp3_stream *, const uint8_t *, size_t, int, nghttp3_tstamp)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    nghttp3_stream_read_state_reset(rstate);
                } else {
                    match (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate {
                        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_HEADERS_BEGIN
                        | nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_HEADERS_BEGIN => {
                            rv = conn_call_begin_headers(conn, stream);
                        }
                        nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_TRAILERS_BEGIN
                        | nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_TRAILERS_BEGIN => {
                            rv = conn_call_begin_trailers(conn, stream);
                        }
                        _ => {
                            nghttp3_unreachable_fail(
                                b"nghttp3_conn.c\0".as_ptr() as *const ::core::ffi::c_char,
                                1634 as ::core::ffi::c_int,
                                b"nghttp3_conn_read_bidi\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    if rv != 0 as ::core::ffi::c_int {
                        return rv as nghttp3_ssize;
                    }
                    (*rstate).state = nghttp3_req_stream_state::NGHTTP3_REQ_STREAM_STATE_HEADERS
                        .0 as ::core::ffi::c_int;
                }
            }
            NGHTTP3_FRAME_PUSH_PROMISE
            | NGHTTP3_FRAME_CANCEL_PUSH
            | 0x4
            | 0x7
            | NGHTTP3_FRAME_MAX_PUSH_ID
            | 0xf0700
            | NGHTTP3_FRAME_PRIORITY_UPDATE_PUSH_ID
            | NGHTTP3_H2_FRAME_PRIORITY
            | NGHTTP3_H2_FRAME_PING
            | NGHTTP3_H2_FRAME_WINDOW_UPDATE
            | NGHTTP3_H2_FRAME_CONTINUATION => {
                return NGHTTP3_ERR_H3_FRAME_UNEXPECTED as nghttp3_ssize;
            }
            _ => {
                if conn_glitch_ratelim_drain(conn, 1 as uint64_t, ts)
                    != 0 as ::core::ffi::c_int
                {
                    return NGHTTP3_ERR_H3_EXCESSIVE_LOAD as nghttp3_ssize;
                }
                busy = 1 as ::core::ffi::c_int;
                (*rstate).state = nghttp3_req_stream_state::NGHTTP3_REQ_STREAM_STATE_IGN_FRAME
                    .0 as ::core::ffi::c_int;
            }
        }
    }
    if fin != 0 {
        match (*rstate).state {
            0 => {
                if (*rvint).left != 0 {
                    return NGHTTP3_ERR_H3_GENERAL_PROTOCOL_ERROR as nghttp3_ssize;
                }
                rv = nghttp3_stream_transit_rx_http_state(
                    stream,
                    nghttp3_stream_http_event::NGHTTP3_HTTP_EVENT_MSG_END,
                );
                if rv != 0 as ::core::ffi::c_int {
                    return rv as nghttp3_ssize;
                }
                rv = conn_call_end_stream(conn, stream);
                if rv != 0 as ::core::ffi::c_int {
                    return rv as nghttp3_ssize;
                }
            }
            5 => {}
            _ => return NGHTTP3_ERR_H3_FRAME_ERROR as nghttp3_ssize,
        }
    }
    *pnproc = p.offset_from(src) as size_t;
    return nconsumed as nghttp3_ssize;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_on_data(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut data: *const uint8_t,
    mut datalen: size_t,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    rv = nghttp3_http_on_data_chunk(stream, datalen);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    if (*conn).callbacks.recv_data.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    rv = (*conn)
        .callbacks
        .recv_data
        .expect(
            "non-null function pointer",
        )(
        conn,
        (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
        data,
        datalen,
        (*conn).user_data,
        (*stream).c2rust_unnamed.c2rust_unnamed.user_data,
    );
    if rv != 0 as ::core::ffi::c_int {
        return NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_get_sched_pq(
    mut conn: *mut nghttp3_conn,
    mut tnode: *mut nghttp3_tnode,
) -> *mut nghttp3_pq {
    '_c2rust_label: {
        if (*tnode).pri.0.urgency
            < (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t
        {} else {
            __assert_fail(
                b"tnode->pri.urgency < NGHTTP3_URGENCY_LEVELS\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1838 as ::core::ffi::c_uint,
                b"nghttp3_pq *conn_get_sched_pq(nghttp3_conn *, nghttp3_tnode *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    return &raw mut (*(&raw mut (*conn).sched as *mut C2Rust_Unnamed_19)
        .offset((*tnode).pri.0.urgency as isize))
        .spq;
}
unsafe extern "C" fn conn_decode_headers(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut src: *const uint8_t,
    mut srclen: size_t,
    mut fin: ::core::ffi::c_int,
) -> nghttp3_ssize {
    let mut nread: nghttp3_ssize = 0;
    let mut rv: ::core::ffi::c_int = 0;
    let mut qdec: *mut nghttp3_qpack_decoder = &raw mut (*conn).qdec;
    let mut nv: nghttp3_qpack_nv = nghttp3_qpack_nv {
        name: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        value: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        token: 0,
        flags: 0,
    };
    let mut flags: uint8_t = 0;
    let mut buf: nghttp3_buf = nghttp3_buf {
        begin: ::core::ptr::null_mut::<uint8_t>(),
        end: ::core::ptr::null_mut::<uint8_t>(),
        pos: ::core::ptr::null_mut::<uint8_t>(),
        last: ::core::ptr::null_mut::<uint8_t>(),
    };
    let mut recv_header: nghttp3_recv_header = None;
    let mut http: *mut nghttp3_http_state = ::core::ptr::null_mut::<
        nghttp3_http_state,
    >();
    let mut request: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut trailers: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    's_55: {
        'c_6762: {
            match (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate {
                nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_HEADERS_BEGIN => {
                    request = 1 as ::core::ffi::c_int;
                    break 'c_6762;
                }
                nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_HEADERS_BEGIN => {
                    break 'c_6762;
                }
                nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_TRAILERS_BEGIN => {
                    request = 1 as ::core::ffi::c_int;
                }
                nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_TRAILERS_BEGIN => {}
                _ => {
                    nghttp3_unreachable_fail(
                        b"nghttp3_conn.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1873 as ::core::ffi::c_int,
                        b"conn_decode_headers\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            }
            trailers = 1 as ::core::ffi::c_int;
            recv_header = (*conn).callbacks.recv_trailer;
            break 's_55;
        }
        recv_header = (*conn).callbacks.recv_header;
    }
    http = &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.rx.http
        as *mut nghttp3_http_state;
    nghttp3_buf_wrap_init(&raw mut buf, src as *mut uint8_t, srclen);
    buf.last = buf.end;
    loop {
        nread = nghttp3_qpack_decoder_read_request(
            qdec,
            &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.qpack_sctx,
            &raw mut nv,
            &raw mut flags,
            buf.pos,
            nghttp3_buf_len(&raw mut buf),
            fin,
        );
        if nread < 0 as nghttp3_ssize {
            return nread as ::core::ffi::c_int as nghttp3_ssize;
        }
        buf.pos = buf.pos.offset(nread as isize);
        if flags as ::core::ffi::c_uint & NGHTTP3_QPACK_DECODE_FLAG_BLOCKED != 0 {
            if (*conn).local.settings.qpack_blocked_streams
                <= nghttp3_pq_size(&raw mut (*conn).qpack_blocked_streams)
            {
                return NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED as nghttp3_ssize;
            }
            (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
                .c2rust_unnamed
                .c2rust_unnamed
                .flags as ::core::ffi::c_uint | NGHTTP3_STREAM_FLAG_QPACK_DECODE_BLOCKED)
                as uint16_t;
            rv = nghttp3_conn_qpack_blocked_streams_push(conn, stream);
            if rv != 0 as ::core::ffi::c_int {
                return rv as nghttp3_ssize;
            }
            break;
        } else if flags as ::core::ffi::c_uint & NGHTTP3_QPACK_DECODE_FLAG_FINAL != 0 {
            nghttp3_qpack_stream_context_reset(
                &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.qpack_sctx,
            );
            break;
        } else {
            if nread == 0 as nghttp3_ssize {
                break;
            }
            if flags as ::core::ffi::c_uint & NGHTTP3_QPACK_DECODE_FLAG_EMIT != 0 {
                rv = nghttp3_http_on_header(
                    http,
                    &raw mut nv,
                    request,
                    trailers,
                    ((*conn).server != 0
                        && (*conn).local.settings.enable_connect_protocol
                            as ::core::ffi::c_int != 0) as ::core::ffi::c_int,
                );
                match rv {
                    NGHTTP3_ERR_MALFORMED_HTTP_HEADER => {}
                    NGHTTP3_ERR_REMOVE_HTTP_HEADER => {
                        rv = 0 as ::core::ffi::c_int;
                    }
                    0 => {
                        if recv_header.is_some() {
                            rv = recv_header
                                .expect(
                                    "non-null function pointer",
                                )(
                                conn,
                                (*stream).c2rust_unnamed.c2rust_unnamed.node.id,
                                nv.token,
                                nv.name,
                                nv.value,
                                nv.flags,
                                (*conn).user_data,
                                (*stream).c2rust_unnamed.c2rust_unnamed.user_data,
                            );
                            if rv != 0 as ::core::ffi::c_int {
                                rv = NGHTTP3_ERR_CALLBACK_FAILURE;
                            }
                        }
                    }
                    _ => {
                        nghttp3_unreachable_fail(
                            b"nghttp3_conn.c\0".as_ptr() as *const ::core::ffi::c_char,
                            1934 as ::core::ffi::c_int,
                            b"conn_decode_headers\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
                nghttp3_rcbuf_decref(nv.name);
                nghttp3_rcbuf_decref(nv.value);
                if rv != 0 as ::core::ffi::c_int {
                    return rv as nghttp3_ssize;
                }
            }
        }
    }
    return buf.pos.offset_from(src);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_on_headers(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut src: *const uint8_t,
    mut srclen: size_t,
    mut fin: ::core::ffi::c_int,
) -> nghttp3_ssize {
    if srclen == 0 as size_t && fin == 0 {
        return 0 as nghttp3_ssize;
    }
    return conn_decode_headers(conn, stream, src, srclen, fin);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_on_settings_entry_received(
    mut conn: *mut nghttp3_conn,
    mut fr: *const nghttp3_frame_settings,
) -> ::core::ffi::c_int {
    let mut ent: *const nghttp3_settings_entry = (*fr).iv.offset(0isize);
    let mut dest: *mut nghttp3_proto_settings = &raw mut (*conn).remote.settings;
    match (*ent).id {
        NGHTTP3_SETTINGS_ID_MAX_FIELD_SECTION_SIZE => {
            (*dest).max_field_section_size = (*ent).value;
        }
        NGHTTP3_SETTINGS_ID_QPACK_MAX_TABLE_CAPACITY => {
            if (*dest).qpack_max_dtable_capacity != 0 as size_t {
                return NGHTTP3_ERR_H3_SETTINGS_ERROR;
            }
            if (*ent).value != 0 as uint64_t {
                (*dest).qpack_max_dtable_capacity = (*ent).value as size_t;
                nghttp3_qpack_encoder_set_max_dtable_capacity(
                    &raw mut (*conn).qenc,
                    (*ent).value as size_t,
                );
            }
        }
        NGHTTP3_SETTINGS_ID_QPACK_BLOCKED_STREAMS => {
            if (*dest).qpack_blocked_streams != 0 as size_t {
                return NGHTTP3_ERR_H3_SETTINGS_ERROR;
            }
            if (*ent).value != 0 as uint64_t {
                (*dest).qpack_blocked_streams = (*ent).value as size_t;
                nghttp3_qpack_encoder_set_max_blocked_streams(
                    &raw mut (*conn).qenc,
                    nghttp3_min_unsigned_long_int(
                        100 as ::core::ffi::c_ulong,
                        (*ent).value as ::core::ffi::c_ulong,
                    ) as size_t,
                );
            }
        }
        NGHTTP3_SETTINGS_ID_ENABLE_CONNECT_PROTOCOL => {
            if (*conn).server == 0 {
                match (*ent).value {
                    0 => {
                        if (*dest).enable_connect_protocol != 0 {
                            return NGHTTP3_ERR_H3_SETTINGS_ERROR;
                        }
                    }
                    1 => {}
                    _ => return NGHTTP3_ERR_H3_SETTINGS_ERROR,
                }
                (*dest).enable_connect_protocol = (*ent).value as uint8_t;
            }
        }
        NGHTTP3_SETTINGS_ID_H3_DATAGRAM => {
            match (*ent).value {
                0 | 1 => {}
                _ => return NGHTTP3_ERR_H3_SETTINGS_ERROR,
            }
            (*dest).h3_datagram = (*ent).value as uint8_t;
        }
        NGHTTP3_H2_SETTINGS_ID_ENABLE_PUSH
        | NGHTTP3_H2_SETTINGS_ID_MAX_CONCURRENT_STREAMS
        | NGHTTP3_H2_SETTINGS_ID_INITIAL_WINDOW_SIZE
        | NGHTTP3_H2_SETTINGS_ID_MAX_FRAME_SIZE => return NGHTTP3_ERR_H3_SETTINGS_ERROR,
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn conn_on_priority_update_stream(
    mut conn: *mut nghttp3_conn,
    mut fr: *const nghttp3_frame_priority_update,
) -> ::core::ffi::c_int {
    let mut stream_id: int64_t = (*fr).pri_elem_id;
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    let mut rv: ::core::ffi::c_int = 0;
    if nghttp3_client_stream_bidi(stream_id) == 0
        || nghttp3_ord_stream_id(stream_id) > (*conn).remote.bidi.max_client_streams
    {
        return NGHTTP3_ERR_H3_ID_ERROR;
    }
    stream = nghttp3_conn_find_stream(conn, stream_id) as *mut nghttp3_stream;
    if stream.is_null() {
        if (*conn).flags as ::core::ffi::c_uint & NGHTTP3_CONN_FLAG_GOAWAY_QUEUED != 0
            && (*conn).tx.goaway_id <= stream_id
        {
            return 0 as ::core::ffi::c_int;
        }
        rv = conn_bidi_idtr_open(conn, stream_id);
        if rv != 0 as ::core::ffi::c_int {
            if nghttp3_err_is_fatal(rv) != 0 {
                return rv;
            }
            '_c2rust_label: {
                if rv == -104 as ::core::ffi::c_int {} else {
                    __assert_fail(
                        b"rv == NGHTTP3_ERR_STREAM_IN_USE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2080 as ::core::ffi::c_uint,
                        b"int conn_on_priority_update_stream(nghttp3_conn *, const nghttp3_frame_priority_update *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            return 0 as ::core::ffi::c_int;
        }
        (*conn).rx.max_stream_id_bidi = nghttp3_max_long_int(
            (*conn).rx.max_stream_id_bidi as ::core::ffi::c_long,
            stream_id as ::core::ffi::c_long,
        ) as int64_t;
        rv = nghttp3_conn_create_stream(conn, &raw mut stream, stream_id);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
        (*stream).c2rust_unnamed.c2rust_unnamed.node.pri = (*fr).c2rust_unnamed.pri;
        (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
            .c2rust_unnamed
            .c2rust_unnamed
            .flags as ::core::ffi::c_uint | NGHTTP3_STREAM_FLAG_PRIORITY_UPDATE_RECVED)
            as uint16_t;
        (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_REQ_INITIAL;
        return 0 as ::core::ffi::c_int;
    }
    if (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
        & NGHTTP3_STREAM_FLAG_SERVER_PRIORITY_SET != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .flags as ::core::ffi::c_uint | NGHTTP3_STREAM_FLAG_PRIORITY_UPDATE_RECVED)
        as uint16_t;
    return conn_update_stream_priority(
        conn,
        stream,
        &raw const (*fr).c2rust_unnamed.pri,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_on_priority_update(
    mut conn: *mut nghttp3_conn,
    mut fr: *const nghttp3_frame_priority_update,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if (*conn).server != 0 {} else {
            __assert_fail(
                b"conn->server\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2111 as ::core::ffi::c_uint,
                b"int nghttp3_conn_on_priority_update(nghttp3_conn *, const nghttp3_frame_priority_update *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*fr).r#type == 0xf0700 as uint64_t {} else {
            __assert_fail(
                b"fr->type == NGHTTP3_FRAME_PRIORITY_UPDATE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2112 as ::core::ffi::c_uint,
                b"int nghttp3_conn_on_priority_update(nghttp3_conn *, const nghttp3_frame_priority_update *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    return conn_on_priority_update_stream(conn, fr);
}
unsafe extern "C" fn conn_stream_acked_data(
    mut stream: *mut nghttp3_stream,
    mut stream_id: int64_t,
    mut datalen: uint64_t,
    mut user_data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut conn: *mut nghttp3_conn = (*stream).c2rust_unnamed.c2rust_unnamed.conn;
    let mut rv: ::core::ffi::c_int = 0;
    if (*conn).callbacks.acked_stream_data.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    rv = (*conn)
        .callbacks
        .acked_stream_data
        .expect(
            "non-null function pointer",
        )(conn, stream_id, datalen, (*conn).user_data, user_data);
    if rv != 0 as ::core::ffi::c_int {
        return NGHTTP3_ERR_CALLBACK_FAILURE;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_create_stream(
    mut conn: *mut nghttp3_conn,
    mut pstream: *mut *mut nghttp3_stream,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    let mut rv: ::core::ffi::c_int = 0;
    static mut callbacks: nghttp3_stream_callbacks = nghttp3_stream_callbacks {
        acked_data: Some(
            conn_stream_acked_data
                as unsafe extern "C" fn(
                    *mut nghttp3_stream,
                    int64_t,
                    uint64_t,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    };
    rv = nghttp3_stream_new(
        &raw mut stream,
        stream_id,
        &raw const callbacks,
        &raw mut (*conn).out_chunk_objalloc,
        &raw mut (*conn).stream_objalloc,
        (*conn).mem,
    );
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.conn = conn;
    rv = nghttp3_map_insert(
        &raw mut (*conn).streams,
        (*stream).c2rust_unnamed.c2rust_unnamed.node.id as nghttp3_map_key_type,
        stream as *mut ::core::ffi::c_void,
    );
    if rv != 0 as ::core::ffi::c_int {
        nghttp3_stream_del(stream);
        return rv;
    }
    if (*conn).server != 0 && nghttp3_client_stream_bidi(stream_id) != 0 {
        (*conn).remote.bidi.num_streams = (*conn)
            .remote
            .bidi
            .num_streams
            .wrapping_add(1);
    }
    *pstream = stream;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_find_stream(
    mut conn: *const nghttp3_conn,
    mut stream_id: int64_t,
) -> *mut nghttp3_stream {
    return nghttp3_map_find(
        &raw const (*conn).streams,
        stream_id as nghttp3_map_key_type,
    ) as *mut nghttp3_stream;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_bind_control_stream(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    let mut fr: *mut nghttp3_frame = ::core::ptr::null_mut::<nghttp3_frame>();
    let mut rv: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if stream_id >= 0 as int64_t {} else {
            __assert_fail(
                b"stream_id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2178 as ::core::ffi::c_uint,
                b"int nghttp3_conn_bind_control_stream(nghttp3_conn *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if stream_id
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong) as int64_t
        {} else {
            __assert_fail(
                b"stream_id <= (int64_t)NGHTTP3_MAX_VARINT\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2179 as ::core::ffi::c_uint,
                b"int nghttp3_conn_bind_control_stream(nghttp3_conn *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*conn).server == 0 || nghttp3_server_stream_uni(stream_id) != 0 {} else {
            __assert_fail(
                b"!conn->server || nghttp3_server_stream_uni(stream_id)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2180 as ::core::ffi::c_uint,
                b"int nghttp3_conn_bind_control_stream(nghttp3_conn *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*conn).server != 0 || nghttp3_client_stream_uni(stream_id) != 0 {} else {
            __assert_fail(
                b"conn->server || nghttp3_client_stream_uni(stream_id)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2181 as ::core::ffi::c_uint,
                b"int nghttp3_conn_bind_control_stream(nghttp3_conn *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if !(*conn).tx.ctrl.is_null() {
        return NGHTTP3_ERR_INVALID_STATE;
    }
    rv = nghttp3_conn_create_stream(conn, &raw mut stream, stream_id);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.r#type = NGHTTP3_STREAM_TYPE_CONTROL
        as nghttp3_stream_type;
    (*conn).tx.ctrl = stream as *mut nghttp3_stream;
    rv = nghttp3_stream_write_stream_type(stream);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    rv = nghttp3_stream_frq_emplace(stream, &raw mut fr);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    (*fr).settings = nghttp3_frame_settings {
        r#type: NGHTTP3_FRAME_SETTINGS as uint64_t,
        niv: 0,
        iv: ::core::ptr::null_mut::<nghttp3_settings_entry>(),
        local_settings: &raw mut (*conn).local.settings,
    };
    if !(*conn).local.settings.origin_list.is_null() {
        '_c2rust_label_3: {
            if (*conn).server != 0 {} else {
                __assert_fail(
                    b"conn->server\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2212 as ::core::ffi::c_uint,
                    b"int nghttp3_conn_bind_control_stream(nghttp3_conn *, int64_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        rv = nghttp3_stream_frq_emplace(stream, &raw mut fr);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
        (*fr).origin = nghttp3_frame_origin {
            r#type: NGHTTP3_FRAME_ORIGIN as uint64_t,
            origin_list: *(*conn).local.settings.origin_list,
        };
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_bind_qpack_streams(
    mut conn: *mut nghttp3_conn,
    mut qenc_stream_id: int64_t,
    mut qdec_stream_id: int64_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    let mut rv: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if qenc_stream_id >= 0 as int64_t {} else {
            __assert_fail(
                b"qenc_stream_id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2233 as ::core::ffi::c_uint,
                b"int nghttp3_conn_bind_qpack_streams(nghttp3_conn *, int64_t, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if qenc_stream_id
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong) as int64_t
        {} else {
            __assert_fail(
                b"qenc_stream_id <= (int64_t)NGHTTP3_MAX_VARINT\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2234 as ::core::ffi::c_uint,
                b"int nghttp3_conn_bind_qpack_streams(nghttp3_conn *, int64_t, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if qdec_stream_id >= 0 as int64_t {} else {
            __assert_fail(
                b"qdec_stream_id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2235 as ::core::ffi::c_uint,
                b"int nghttp3_conn_bind_qpack_streams(nghttp3_conn *, int64_t, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if qdec_stream_id
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong) as int64_t
        {} else {
            __assert_fail(
                b"qdec_stream_id <= (int64_t)NGHTTP3_MAX_VARINT\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2236 as ::core::ffi::c_uint,
                b"int nghttp3_conn_bind_qpack_streams(nghttp3_conn *, int64_t, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if (*conn).server == 0 || nghttp3_server_stream_uni(qenc_stream_id) != 0
        {} else {
            __assert_fail(
                b"!conn->server || nghttp3_server_stream_uni(qenc_stream_id)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2237 as ::core::ffi::c_uint,
                b"int nghttp3_conn_bind_qpack_streams(nghttp3_conn *, int64_t, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if (*conn).server == 0 || nghttp3_server_stream_uni(qdec_stream_id) != 0
        {} else {
            __assert_fail(
                b"!conn->server || nghttp3_server_stream_uni(qdec_stream_id)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2238 as ::core::ffi::c_uint,
                b"int nghttp3_conn_bind_qpack_streams(nghttp3_conn *, int64_t, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_5: {
        if (*conn).server != 0 || nghttp3_client_stream_uni(qenc_stream_id) != 0
        {} else {
            __assert_fail(
                b"conn->server || nghttp3_client_stream_uni(qenc_stream_id)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2239 as ::core::ffi::c_uint,
                b"int nghttp3_conn_bind_qpack_streams(nghttp3_conn *, int64_t, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_6: {
        if (*conn).server != 0 || nghttp3_client_stream_uni(qdec_stream_id) != 0
        {} else {
            __assert_fail(
                b"conn->server || nghttp3_client_stream_uni(qdec_stream_id)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2240 as ::core::ffi::c_uint,
                b"int nghttp3_conn_bind_qpack_streams(nghttp3_conn *, int64_t, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if !(*conn).tx.qenc.is_null() || !(*conn).tx.qdec.is_null() {
        return NGHTTP3_ERR_INVALID_STATE;
    }
    rv = nghttp3_conn_create_stream(conn, &raw mut stream, qenc_stream_id);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.r#type = NGHTTP3_STREAM_TYPE_QPACK_ENCODER
        as nghttp3_stream_type;
    (*conn).tx.qenc = stream as *mut nghttp3_stream;
    rv = nghttp3_stream_write_stream_type(stream);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    rv = nghttp3_conn_create_stream(conn, &raw mut stream, qdec_stream_id);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.r#type = NGHTTP3_STREAM_TYPE_QPACK_DECODER
        as nghttp3_stream_type;
    (*conn).tx.qdec = stream as *mut nghttp3_stream;
    return nghttp3_stream_write_stream_type(stream);
}
unsafe extern "C" fn conn_writev_stream(
    mut conn: *mut nghttp3_conn,
    mut pstream_id: *mut int64_t,
    mut pfin: *mut ::core::ffi::c_int,
    mut vec: *mut nghttp3_vec,
    mut veccnt: size_t,
    mut stream: *mut nghttp3_stream,
) -> nghttp3_ssize {
    let mut rv: ::core::ffi::c_int = 0;
    let mut n: size_t = 0;
    '_c2rust_label: {
        if veccnt > 0 as size_t {} else {
            __assert_fail(
                b"veccnt > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2278 as ::core::ffi::c_uint,
                b"nghttp3_ssize conn_writev_stream(nghttp3_conn *, int64_t *, int *, nghttp3_vec *, size_t, nghttp3_stream *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
        & NGHTTP3_STREAM_FLAG_READ_DATA_BLOCKED == 0
    {
        rv = nghttp3_stream_fill_outq(stream);
        if rv != 0 as ::core::ffi::c_int {
            return rv as nghttp3_ssize;
        }
    }
    if nghttp3_stream_uni((*stream).c2rust_unnamed.c2rust_unnamed.node.id) == 0
        && !(*conn).tx.qenc.is_null() && nghttp3_stream_is_blocked((*conn).tx.qenc) == 0
    {
        n = nghttp3_stream_writev((*conn).tx.qenc, pfin, vec, veccnt);
        if n != 0 {
            *pstream_id = (*(*conn).tx.qenc).c2rust_unnamed.c2rust_unnamed.node.id;
            return n as nghttp3_ssize;
        }
    }
    n = nghttp3_stream_writev(stream, pfin, vec, veccnt);
    if n == 0 as size_t && *pfin == 0 as ::core::ffi::c_int {
        return 0 as nghttp3_ssize;
    }
    *pstream_id = (*stream).c2rust_unnamed.c2rust_unnamed.node.id;
    return n as nghttp3_ssize;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_writev_stream(
    mut conn: *mut nghttp3_conn,
    mut pstream_id: *mut int64_t,
    mut pfin: *mut ::core::ffi::c_int,
    mut vec: *mut nghttp3_vec,
    mut veccnt: size_t,
) -> nghttp3_ssize {
    let mut ncnt: nghttp3_ssize = 0;
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    let mut rv: ::core::ffi::c_int = 0;
    *pstream_id = -1 as int64_t;
    *pfin = 0 as ::core::ffi::c_int;
    if veccnt == 0 as size_t {
        return 0 as nghttp3_ssize;
    }
    if !(*conn).tx.ctrl.is_null() && nghttp3_stream_is_blocked((*conn).tx.ctrl) == 0 {
        ncnt = conn_writev_stream(conn, pstream_id, pfin, vec, veccnt, (*conn).tx.ctrl);
        if ncnt != 0 {
            return ncnt;
        }
    }
    if !(*conn).tx.qdec.is_null() && nghttp3_stream_is_blocked((*conn).tx.qdec) == 0 {
        rv = nghttp3_stream_write_qpack_decoder_stream((*conn).tx.qdec);
        if rv != 0 as ::core::ffi::c_int {
            return rv as nghttp3_ssize;
        }
        ncnt = conn_writev_stream(conn, pstream_id, pfin, vec, veccnt, (*conn).tx.qdec);
        if ncnt != 0 {
            return ncnt;
        }
    }
    if !(*conn).tx.qenc.is_null() && nghttp3_stream_is_blocked((*conn).tx.qenc) == 0 {
        ncnt = conn_writev_stream(conn, pstream_id, pfin, vec, veccnt, (*conn).tx.qenc);
        if ncnt != 0 {
            return ncnt;
        }
    }
    stream = nghttp3_conn_get_next_tx_stream(conn) as *mut nghttp3_stream;
    if stream.is_null() {
        return 0 as nghttp3_ssize;
    }
    ncnt = conn_writev_stream(conn, pstream_id, pfin, vec, veccnt, stream);
    if ncnt < 0 as nghttp3_ssize {
        return ncnt;
    }
    if nghttp3_client_stream_bidi((*stream).c2rust_unnamed.c2rust_unnamed.node.id) != 0
        && nghttp3_stream_require_schedule(stream) == 0
    {
        nghttp3_conn_unschedule_stream(conn, stream);
    }
    return ncnt;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_get_next_tx_stream(
    mut conn: *mut nghttp3_conn,
) -> *mut nghttp3_stream {
    let mut i: size_t = 0;
    let mut tnode: *mut nghttp3_tnode = ::core::ptr::null_mut::<nghttp3_tnode>();
    let mut pq: *mut nghttp3_pq = ::core::ptr::null_mut::<nghttp3_pq>();
    i = 0 as size_t;
    while i < NGHTTP3_URGENCY_LEVELS as size_t {
        pq = &raw mut (*(&raw mut (*conn).sched as *mut C2Rust_Unnamed_19)
            .offset(i as isize))
            .spq;
        if nghttp3_pq_empty(pq) != 0 {
            i = i.wrapping_add(1);
        } else {
            tnode = (nghttp3_pq_top(pq) as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize))
                as *mut ::core::ffi::c_void as *mut nghttp3_tnode;
            return (tnode as *mut ::core::ffi::c_char)
                .offset(-(24 as ::core::ffi::c_ulong as isize))
                as *mut ::core::ffi::c_void as *mut nghttp3_stream;
        }
    }
    return ::core::ptr::null_mut::<nghttp3_stream>();
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_add_write_offset(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
    mut n: size_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = nghttp3_conn_find_stream(conn, stream_id);
    if stream.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    nghttp3_stream_add_outq_offset(stream, n);
    (*stream).c2rust_unnamed.c2rust_unnamed.unscheduled_nwrite = ((*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .unscheduled_nwrite as ::core::ffi::c_ulong)
        .wrapping_add(n as ::core::ffi::c_ulong) as uint64_t;
    if nghttp3_client_stream_bidi((*stream).c2rust_unnamed.c2rust_unnamed.node.id) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if nghttp3_stream_require_schedule(stream) == 0 {
        nghttp3_conn_unschedule_stream(conn, stream);
        return 0 as ::core::ffi::c_int;
    }
    if (*stream).c2rust_unnamed.c2rust_unnamed.unscheduled_nwrite
        < NGHTTP3_STREAM_MIN_WRITELEN as uint64_t
    {
        return 0 as ::core::ffi::c_int;
    }
    return nghttp3_conn_schedule_stream(conn, stream);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_add_ack_offset(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
    mut n: uint64_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = nghttp3_conn_find_stream(conn, stream_id);
    if stream.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    return nghttp3_stream_update_ack_offset(
        stream,
        (*stream).c2rust_unnamed.c2rust_unnamed.ack_offset.wrapping_add(n),
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_update_ack_offset(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
    mut offset: uint64_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = nghttp3_conn_find_stream(conn, stream_id);
    if stream.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*stream).c2rust_unnamed.c2rust_unnamed.ack_offset > offset {
        return NGHTTP3_ERR_INVALID_ARGUMENT;
    }
    return nghttp3_stream_update_ack_offset(stream, offset);
}
unsafe extern "C" fn conn_submit_headers_data(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
    mut nva: *const nghttp3_nv,
    mut nvlen: size_t,
    mut dr: *const nghttp3_data_reader,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    let mut nnva: *mut nghttp3_nv = ::core::ptr::null_mut::<nghttp3_nv>();
    let mut fr: *mut nghttp3_frame = ::core::ptr::null_mut::<nghttp3_frame>();
    rv = nghttp3_nva_copy(&raw mut nnva, nva, nvlen, (*conn).mem);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    rv = nghttp3_stream_frq_emplace(stream, &raw mut fr);
    if rv != 0 as ::core::ffi::c_int {
        nghttp3_nva_del(nnva, (*conn).mem);
        return rv;
    }
    (*fr).headers = nghttp3_frame_headers {
        r#type: NGHTTP3_FRAME_HEADERS as uint64_t,
        nva: nnva,
        nvlen: nvlen,
    };
    if !dr.is_null() {
        rv = nghttp3_stream_frq_emplace(stream, &raw mut fr);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
        (*fr).data = nghttp3_frame_data {
            r#type: NGHTTP3_FRAME_DATA as uint64_t,
            dr: *dr,
        };
    }
    if nghttp3_stream_require_schedule(stream) != 0 {
        return nghttp3_conn_schedule_stream(conn, stream);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_schedule_stream(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
) -> ::core::ffi::c_int {
    let mut node: *mut nghttp3_tnode = stream_get_sched_node(stream);
    let mut rv: ::core::ffi::c_int = 0;
    rv = nghttp3_tnode_schedule(
        node,
        conn_get_sched_pq(conn, node),
        (*stream).c2rust_unnamed.c2rust_unnamed.unscheduled_nwrite,
    );
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.unscheduled_nwrite = 0 as uint64_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_ensure_stream_scheduled(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
) -> ::core::ffi::c_int {
    if nghttp3_tnode_is_scheduled(stream_get_sched_node(stream)) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    return nghttp3_conn_schedule_stream(conn, stream);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_unschedule_stream(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
) {
    let mut node: *mut nghttp3_tnode = stream_get_sched_node(stream);
    nghttp3_tnode_unschedule(node, conn_get_sched_pq(conn, node));
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_submit_request(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
    mut nva: *const nghttp3_nv,
    mut nvlen: size_t,
    mut dr: *const nghttp3_data_reader,
    mut stream_user_data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    let mut rv: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if (*conn).server == 0 {} else {
            __assert_fail(
                b"!conn->server\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2526 as ::core::ffi::c_uint,
                b"int nghttp3_conn_submit_request(nghttp3_conn *, int64_t, const nghttp3_nv *, size_t, const nghttp3_data_reader *, void *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !(*conn).tx.qenc.is_null() {} else {
            __assert_fail(
                b"conn->tx.qenc\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2527 as ::core::ffi::c_uint,
                b"int nghttp3_conn_submit_request(nghttp3_conn *, int64_t, const nghttp3_nv *, size_t, const nghttp3_data_reader *, void *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if stream_id >= 0 as int64_t {} else {
            __assert_fail(
                b"stream_id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2528 as ::core::ffi::c_uint,
                b"int nghttp3_conn_submit_request(nghttp3_conn *, int64_t, const nghttp3_nv *, size_t, const nghttp3_data_reader *, void *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if stream_id
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong) as int64_t
        {} else {
            __assert_fail(
                b"stream_id <= (int64_t)NGHTTP3_MAX_VARINT\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2529 as ::core::ffi::c_uint,
                b"int nghttp3_conn_submit_request(nghttp3_conn *, int64_t, const nghttp3_nv *, size_t, const nghttp3_data_reader *, void *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if nghttp3_client_stream_bidi(stream_id) != 0 {} else {
            __assert_fail(
                b"nghttp3_client_stream_bidi(stream_id)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2530 as ::core::ffi::c_uint,
                b"int nghttp3_conn_submit_request(nghttp3_conn *, int64_t, const nghttp3_nv *, size_t, const nghttp3_data_reader *, void *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if (*conn).flags as ::core::ffi::c_uint & NGHTTP3_CONN_FLAG_GOAWAY_RECVED != 0 {
        return NGHTTP3_ERR_CONN_CLOSING;
    }
    stream = nghttp3_conn_find_stream(conn, stream_id) as *mut nghttp3_stream;
    if !stream.is_null() {
        return NGHTTP3_ERR_STREAM_IN_USE;
    }
    rv = nghttp3_conn_create_stream(conn, &raw mut stream, stream_id);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.rx.hstate = nghttp3_stream_http_state::NGHTTP3_HTTP_STATE_RESP_INITIAL;
    (*stream).c2rust_unnamed.c2rust_unnamed.user_data = stream_user_data;
    (*stream).c2rust_unnamed.c2rust_unnamed.node.pri.0.inc = 1 as uint8_t;
    nghttp3_http_record_request_method(stream, nva, nvlen);
    if dr.is_null() {
        (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
            .c2rust_unnamed
            .c2rust_unnamed
            .flags as ::core::ffi::c_uint | NGHTTP3_STREAM_FLAG_WRITE_END_STREAM)
            as uint16_t;
    }
    return conn_submit_headers_data(conn, stream, nva, nvlen, dr);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_submit_info(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
    mut nva: *const nghttp3_nv,
    mut nvlen: size_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    '_c2rust_label: {
        if (*conn).server != 0 {} else {
            __assert_fail(
                b"conn->server\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2566 as ::core::ffi::c_uint,
                b"int nghttp3_conn_submit_info(nghttp3_conn *, int64_t, const nghttp3_nv *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !(*conn).tx.qenc.is_null() {} else {
            __assert_fail(
                b"conn->tx.qenc\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2567 as ::core::ffi::c_uint,
                b"int nghttp3_conn_submit_info(nghttp3_conn *, int64_t, const nghttp3_nv *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    stream = nghttp3_conn_find_stream(conn, stream_id) as *mut nghttp3_stream;
    if stream.is_null() {
        return NGHTTP3_ERR_STREAM_NOT_FOUND;
    }
    return conn_submit_headers_data(
        conn,
        stream,
        nva,
        nvlen,
        ::core::ptr::null::<nghttp3_data_reader>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_submit_response(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
    mut nva: *const nghttp3_nv,
    mut nvlen: size_t,
    mut dr: *const nghttp3_data_reader,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    '_c2rust_label: {
        if (*conn).server != 0 {} else {
            __assert_fail(
                b"conn->server\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2583 as ::core::ffi::c_uint,
                b"int nghttp3_conn_submit_response(nghttp3_conn *, int64_t, const nghttp3_nv *, size_t, const nghttp3_data_reader *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !(*conn).tx.qenc.is_null() {} else {
            __assert_fail(
                b"conn->tx.qenc\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2584 as ::core::ffi::c_uint,
                b"int nghttp3_conn_submit_response(nghttp3_conn *, int64_t, const nghttp3_nv *, size_t, const nghttp3_data_reader *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    stream = nghttp3_conn_find_stream(conn, stream_id) as *mut nghttp3_stream;
    if stream.is_null() {
        return NGHTTP3_ERR_STREAM_NOT_FOUND;
    }
    if dr.is_null() {
        (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
            .c2rust_unnamed
            .c2rust_unnamed
            .flags as ::core::ffi::c_uint | NGHTTP3_STREAM_FLAG_WRITE_END_STREAM)
            as uint16_t;
    }
    return conn_submit_headers_data(conn, stream, nva, nvlen, dr);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_submit_trailers(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
    mut nva: *const nghttp3_nv,
    mut nvlen: size_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    '_c2rust_label: {
        if !(*conn).tx.qenc.is_null() {} else {
            __assert_fail(
                b"conn->tx.qenc\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2603 as ::core::ffi::c_uint,
                b"int nghttp3_conn_submit_trailers(nghttp3_conn *, int64_t, const nghttp3_nv *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    stream = nghttp3_conn_find_stream(conn, stream_id) as *mut nghttp3_stream;
    if stream.is_null() {
        return NGHTTP3_ERR_STREAM_NOT_FOUND;
    }
    if (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
        & NGHTTP3_STREAM_FLAG_WRITE_END_STREAM != 0
    {
        return NGHTTP3_ERR_INVALID_STATE;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .flags as ::core::ffi::c_uint | NGHTTP3_STREAM_FLAG_WRITE_END_STREAM)
        as uint16_t;
    return conn_submit_headers_data(
        conn,
        stream,
        nva,
        nvlen,
        ::core::ptr::null::<nghttp3_data_reader>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_submit_shutdown_notice(
    mut conn: *mut nghttp3_conn,
) -> ::core::ffi::c_int {
    let mut fr: *mut nghttp3_frame = ::core::ptr::null_mut::<nghttp3_frame>();
    let mut rv: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !(*conn).tx.ctrl.is_null() {} else {
            __assert_fail(
                b"conn->tx.ctrl\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2623 as ::core::ffi::c_uint,
                b"int nghttp3_conn_submit_shutdown_notice(nghttp3_conn *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    rv = nghttp3_stream_frq_emplace((*conn).tx.ctrl, &raw mut fr);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    (*fr).goaway = nghttp3_frame_goaway {
        r#type: NGHTTP3_FRAME_GOAWAY as uint64_t,
        id: (if (*conn).server != 0 {
            NGHTTP3_SHUTDOWN_NOTICE_STREAM_ID
        } else {
            NGHTTP3_SHUTDOWN_NOTICE_PUSH_ID
        }) as int64_t,
    };
    '_c2rust_label_0: {
        if (*fr).goaway.id <= (*conn).tx.goaway_id {} else {
            __assert_fail(
                b"fr->goaway.id <= conn->tx.goaway_id\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2636 as ::core::ffi::c_uint,
                b"int nghttp3_conn_submit_shutdown_notice(nghttp3_conn *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*conn).tx.goaway_id = (*fr).goaway.id;
    (*conn).flags = ((*conn).flags as ::core::ffi::c_uint
        | NGHTTP3_CONN_FLAG_GOAWAY_QUEUED) as uint16_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_shutdown(
    mut conn: *mut nghttp3_conn,
) -> ::core::ffi::c_int {
    let mut fr: *mut nghttp3_frame = ::core::ptr::null_mut::<nghttp3_frame>();
    let mut rv: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !(*conn).tx.ctrl.is_null() {} else {
            __assert_fail(
                b"conn->tx.ctrl\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2648 as ::core::ffi::c_uint,
                b"int nghttp3_conn_shutdown(nghttp3_conn *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    rv = nghttp3_stream_frq_emplace((*conn).tx.ctrl, &raw mut fr);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    (*fr).goaway = nghttp3_frame_goaway {
        r#type: NGHTTP3_FRAME_GOAWAY as uint64_t,
        id: (if (*conn).server != 0 {
            nghttp3_min_long_long_int(
                ((1 as ::core::ffi::c_longlong) << 62 as ::core::ffi::c_int)
                    - 4 as ::core::ffi::c_longlong,
                ((*conn).rx.max_stream_id_bidi + 4 as int64_t) as ::core::ffi::c_longlong,
            )
        } else {
            0 as ::core::ffi::c_longlong
        }) as int64_t,
    };
    '_c2rust_label_0: {
        if (*fr).goaway.id <= (*conn).tx.goaway_id {} else {
            __assert_fail(
                b"fr->goaway.id <= conn->tx.goaway_id\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2662 as ::core::ffi::c_uint,
                b"int nghttp3_conn_shutdown(nghttp3_conn *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*conn).tx.goaway_id = (*fr).goaway.id;
    (*conn).flags = ((*conn).flags as ::core::ffi::c_uint
        | (NGHTTP3_CONN_FLAG_GOAWAY_QUEUED | NGHTTP3_CONN_FLAG_SHUTDOWN_COMMENCED))
        as uint16_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_reject_stream(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    rv = conn_call_stop_sending(conn, stream, NGHTTP3_H3_REQUEST_REJECTED as uint64_t);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    return conn_call_reset_stream(conn, stream, NGHTTP3_H3_REQUEST_REJECTED as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_block_stream(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
) {
    let mut stream: *mut nghttp3_stream = nghttp3_conn_find_stream(conn, stream_id);
    if stream.is_null() {
        return;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .flags as ::core::ffi::c_uint | NGHTTP3_STREAM_FLAG_FC_BLOCKED) as uint16_t;
    (*stream).c2rust_unnamed.c2rust_unnamed.unscheduled_nwrite = 0 as uint64_t;
    if nghttp3_client_stream_bidi((*stream).c2rust_unnamed.c2rust_unnamed.node.id) != 0 {
        nghttp3_conn_unschedule_stream(conn, stream);
    }
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_shutdown_stream_write(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
) {
    let mut stream: *mut nghttp3_stream = nghttp3_conn_find_stream(conn, stream_id);
    if stream.is_null() {
        return;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .flags as ::core::ffi::c_uint | NGHTTP3_STREAM_FLAG_SHUT_WR) as uint16_t;
    (*stream).c2rust_unnamed.c2rust_unnamed.unscheduled_nwrite = 0 as uint64_t;
    if nghttp3_client_stream_bidi((*stream).c2rust_unnamed.c2rust_unnamed.node.id) != 0 {
        nghttp3_conn_unschedule_stream(conn, stream);
    }
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_unblock_stream(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = nghttp3_conn_find_stream(conn, stream_id);
    if stream.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .flags as ::core::ffi::c_int
        & !NGHTTP3_STREAM_FLAG_FC_BLOCKED as uint16_t as ::core::ffi::c_int) as uint16_t;
    if nghttp3_client_stream_bidi((*stream).c2rust_unnamed.c2rust_unnamed.node.id) != 0
        && nghttp3_stream_require_schedule(stream) != 0
    {
        return nghttp3_conn_ensure_stream_scheduled(conn, stream);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_is_stream_writable(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    return nghttp3_conn_is_stream_writable2(conn, stream_id);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_is_stream_writable2(
    mut conn: *const nghttp3_conn,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    let mut stream: *const nghttp3_stream = nghttp3_conn_find_stream(conn, stream_id);
    if stream.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    return ((*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
        & (NGHTTP3_STREAM_FLAG_FC_BLOCKED | NGHTTP3_STREAM_FLAG_READ_DATA_BLOCKED
            | NGHTTP3_STREAM_FLAG_SHUT_WR) == 0 as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_resume_stream(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = nghttp3_conn_find_stream(conn, stream_id);
    if stream.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .flags as ::core::ffi::c_int
        & !NGHTTP3_STREAM_FLAG_READ_DATA_BLOCKED as uint16_t as ::core::ffi::c_int)
        as uint16_t;
    if nghttp3_client_stream_bidi((*stream).c2rust_unnamed.c2rust_unnamed.node.id) != 0
        && nghttp3_stream_require_schedule(stream) != 0
    {
        return nghttp3_conn_ensure_stream_scheduled(conn, stream);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_close_stream(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
    mut app_error_code: uint64_t,
) -> ::core::ffi::c_int {
    return nghttp3_conn_close_stream2(
        conn,
        NGHTTP3_STREAM_CLOSE_FLAG_RX_APP_ERROR_CODE_SET as uint32_t
            | NGHTTP3_STREAM_CLOSE_FLAG_TX_APP_ERROR_CODE_SET as uint32_t,
        stream_id,
        app_error_code,
        app_error_code,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_close_stream2(
    mut conn: *mut nghttp3_conn,
    mut flags: uint32_t,
    mut stream_id: int64_t,
    mut rx_app_error_code: uint64_t,
    mut tx_app_error_code: uint64_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = nghttp3_conn_find_stream(conn, stream_id);
    if stream.is_null() {
        return NGHTTP3_ERR_STREAM_NOT_FOUND;
    }
    if nghttp3_stream_uni(stream_id) != 0
        && (*stream).c2rust_unnamed.c2rust_unnamed.r#type
            != NGHTTP3_STREAM_TYPE_UNKNOWN as nghttp3_stream_type
    {
        return NGHTTP3_ERR_H3_CLOSED_CRITICAL_STREAM;
    }
    nghttp3_conn_unschedule_stream(conn, stream);
    return conn_delete_stream(conn, stream, flags, rx_app_error_code, tx_app_error_code);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_shutdown_stream_read(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    '_c2rust_label: {
        if stream_id >= 0 as int64_t {} else {
            __assert_fail(
                b"stream_id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2795 as ::core::ffi::c_uint,
                b"int nghttp3_conn_shutdown_stream_read(nghttp3_conn *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if stream_id
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong) as int64_t
        {} else {
            __assert_fail(
                b"stream_id <= (int64_t)NGHTTP3_MAX_VARINT\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2796 as ::core::ffi::c_uint,
                b"int nghttp3_conn_shutdown_stream_read(nghttp3_conn *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if nghttp3_client_stream_bidi(stream_id) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    stream = nghttp3_conn_find_stream(conn, stream_id) as *mut nghttp3_stream;
    if !stream.is_null() {
        if (*stream).c2rust_unnamed.c2rust_unnamed.flags as ::core::ffi::c_uint
            & NGHTTP3_STREAM_FLAG_SHUT_RD != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
            .c2rust_unnamed
            .c2rust_unnamed
            .flags as ::core::ffi::c_uint | NGHTTP3_STREAM_FLAG_SHUT_RD) as uint16_t;
    }
    return nghttp3_qpack_decoder_cancel_stream(&raw mut (*conn).qdec, stream_id);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_qpack_blocked_streams_push(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if (*stream).c2rust_unnamed.c2rust_unnamed.qpack_blocked_pe.index
            == 18446744073709551615 as size_t
        {} else {
            __assert_fail(
                b"stream->qpack_blocked_pe.index == NGHTTP3_PQ_BAD_INDEX\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2816 as ::core::ffi::c_uint,
                b"int nghttp3_conn_qpack_blocked_streams_push(nghttp3_conn *, nghttp3_stream *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    return nghttp3_pq_push(
        &raw mut (*conn).qpack_blocked_streams,
        &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.qpack_blocked_pe,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_qpack_blocked_streams_pop(
    mut conn: *mut nghttp3_conn,
) {
    '_c2rust_label: {
        if nghttp3_pq_empty(&raw mut (*conn).qpack_blocked_streams) == 0 {} else {
            __assert_fail(
                b"!nghttp3_pq_empty(&conn->qpack_blocked_streams)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2823 as ::core::ffi::c_uint,
                b"void nghttp3_conn_qpack_blocked_streams_pop(nghttp3_conn *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    nghttp3_pq_pop(&raw mut (*conn).qpack_blocked_streams);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_qpack_blocked_streams_remove(
    mut conn: *mut nghttp3_conn,
    mut stream: *mut nghttp3_stream,
) {
    '_c2rust_label: {
        if nghttp3_pq_empty(&raw mut (*conn).qpack_blocked_streams) == 0 {} else {
            __assert_fail(
                b"!nghttp3_pq_empty(&conn->qpack_blocked_streams)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2829 as ::core::ffi::c_uint,
                b"void nghttp3_conn_qpack_blocked_streams_remove(nghttp3_conn *, nghttp3_stream *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*stream).c2rust_unnamed.c2rust_unnamed.qpack_blocked_pe.index
            != 18446744073709551615 as size_t
        {} else {
            __assert_fail(
                b"stream->qpack_blocked_pe.index != NGHTTP3_PQ_BAD_INDEX\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2830 as ::core::ffi::c_uint,
                b"void nghttp3_conn_qpack_blocked_streams_remove(nghttp3_conn *, nghttp3_stream *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    nghttp3_pq_remove(
        &raw mut (*conn).qpack_blocked_streams,
        &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.qpack_blocked_pe,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_set_max_client_streams_bidi(
    mut conn: *mut nghttp3_conn,
    mut max_streams: uint64_t,
) {
    '_c2rust_label: {
        if (*conn).server != 0 {} else {
            __assert_fail(
                b"conn->server\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2837 as ::core::ffi::c_uint,
                b"void nghttp3_conn_set_max_client_streams_bidi(nghttp3_conn *, uint64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*conn).remote.bidi.max_client_streams <= max_streams {} else {
            __assert_fail(
                b"conn->remote.bidi.max_client_streams <= max_streams\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2838 as ::core::ffi::c_uint,
                b"void nghttp3_conn_set_max_client_streams_bidi(nghttp3_conn *, uint64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    (*conn).remote.bidi.max_client_streams = max_streams;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_set_max_concurrent_streams(
    mut conn: *mut nghttp3_conn,
    mut max_concurrent_streams: size_t,
) {
    nghttp3_qpack_decoder_set_max_concurrent_streams(
        &raw mut (*conn).qdec,
        max_concurrent_streams,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_set_stream_user_data(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
    mut stream_user_data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = nghttp3_conn_find_stream(conn, stream_id);
    if stream.is_null() {
        return NGHTTP3_ERR_STREAM_NOT_FOUND;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.user_data = stream_user_data;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_get_stream_user_data(
    mut conn: *const nghttp3_conn,
    mut stream_id: int64_t,
) -> *mut ::core::ffi::c_void {
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    '_c2rust_label: {
        if stream_id >= 0 as int64_t {} else {
            __assert_fail(
                b"stream_id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2866 as ::core::ffi::c_uint,
                b"void *nghttp3_conn_get_stream_user_data(const nghttp3_conn *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if stream_id
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong) as int64_t
        {} else {
            __assert_fail(
                b"stream_id <= (int64_t)NGHTTP3_MAX_VARINT\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2867 as ::core::ffi::c_uint,
                b"void *nghttp3_conn_get_stream_user_data(const nghttp3_conn *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    stream = nghttp3_conn_find_stream(conn, stream_id) as *mut nghttp3_stream;
    if stream.is_null() {
        return NULL;
    }
    return (*stream).c2rust_unnamed.c2rust_unnamed.user_data;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_get_frame_payload_left(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
) -> uint64_t {
    return nghttp3_conn_get_frame_payload_left2(conn, stream_id);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_get_frame_payload_left2(
    mut conn: *const nghttp3_conn,
    mut stream_id: int64_t,
) -> uint64_t {
    let mut stream: *const nghttp3_stream = ::core::ptr::null::<nghttp3_stream>();
    let mut uni: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    '_c2rust_label: {
        if stream_id >= 0 as int64_t {} else {
            __assert_fail(
                b"stream_id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2887 as ::core::ffi::c_uint,
                b"uint64_t nghttp3_conn_get_frame_payload_left2(const nghttp3_conn *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if stream_id
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong) as int64_t
        {} else {
            __assert_fail(
                b"stream_id <= (int64_t)NGHTTP3_MAX_VARINT\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2888 as ::core::ffi::c_uint,
                b"uint64_t nghttp3_conn_get_frame_payload_left2(const nghttp3_conn *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if nghttp3_client_stream_bidi(stream_id) == 0 {
        uni = conn_remote_stream_uni(conn, stream_id);
        if uni == 0 {
            return 0 as uint64_t;
        }
    }
    stream = nghttp3_conn_find_stream(conn, stream_id);
    if stream.is_null() {
        return 0 as uint64_t;
    }
    if uni != 0
        && (*stream).c2rust_unnamed.c2rust_unnamed.r#type
            != NGHTTP3_STREAM_TYPE_CONTROL as nghttp3_stream_type
    {
        return 0 as uint64_t;
    }
    return (*stream).c2rust_unnamed.c2rust_unnamed.rstate.left;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_get_stream_priority_versioned(
    mut conn: *mut nghttp3_conn,
    mut pri_version: ::core::ffi::c_int,
    mut dest: *mut nghttp3_pri,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    return nghttp3_conn_get_stream_priority2_versioned(
        conn,
        pri_version,
        dest,
        stream_id,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_get_stream_priority2_versioned(
    mut conn: *const nghttp3_conn,
    mut pri_version: ::core::ffi::c_int,
    mut dest: *mut nghttp3_pri,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    let mut stream: *const nghttp3_stream = ::core::ptr::null::<nghttp3_stream>();
    '_c2rust_label: {
        if (*conn).server != 0 {} else {
            __assert_fail(
                b"conn->server\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2924 as ::core::ffi::c_uint,
                b"int nghttp3_conn_get_stream_priority2_versioned(const nghttp3_conn *, int, nghttp3_pri *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if stream_id >= 0 as int64_t {} else {
            __assert_fail(
                b"stream_id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2925 as ::core::ffi::c_uint,
                b"int nghttp3_conn_get_stream_priority2_versioned(const nghttp3_conn *, int, nghttp3_pri *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if stream_id
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong) as int64_t
        {} else {
            __assert_fail(
                b"stream_id <= (int64_t)NGHTTP3_MAX_VARINT\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2926 as ::core::ffi::c_uint,
                b"int nghttp3_conn_get_stream_priority2_versioned(const nghttp3_conn *, int, nghttp3_pri *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if nghttp3_client_stream_bidi(stream_id) == 0 {
        return NGHTTP3_ERR_INVALID_ARGUMENT;
    }
    stream = nghttp3_conn_find_stream(conn, stream_id);
    if stream.is_null() {
        return NGHTTP3_ERR_STREAM_NOT_FOUND;
    }
    *dest = (*stream).c2rust_unnamed.c2rust_unnamed.node.pri;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_set_client_stream_priority(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
    mut data: *const uint8_t,
    mut datalen: size_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    let mut fr: *mut nghttp3_frame = ::core::ptr::null_mut::<nghttp3_frame>();
    let mut rv: ::core::ffi::c_int = 0;
    let mut buf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    '_c2rust_label: {
        if (*conn).server == 0 {} else {
            __assert_fail(
                b"!conn->server\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2951 as ::core::ffi::c_uint,
                b"int nghttp3_conn_set_client_stream_priority(nghttp3_conn *, int64_t, const uint8_t *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if stream_id >= 0 as int64_t {} else {
            __assert_fail(
                b"stream_id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2952 as ::core::ffi::c_uint,
                b"int nghttp3_conn_set_client_stream_priority(nghttp3_conn *, int64_t, const uint8_t *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if stream_id
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong) as int64_t
        {} else {
            __assert_fail(
                b"stream_id <= (int64_t)NGHTTP3_MAX_VARINT\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2953 as ::core::ffi::c_uint,
                b"int nghttp3_conn_set_client_stream_priority(nghttp3_conn *, int64_t, const uint8_t *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if nghttp3_client_stream_bidi(stream_id) == 0 {
        return NGHTTP3_ERR_INVALID_ARGUMENT;
    }
    stream = nghttp3_conn_find_stream(conn, stream_id) as *mut nghttp3_stream;
    if stream.is_null() {
        return NGHTTP3_ERR_STREAM_NOT_FOUND;
    }
    if datalen != 0 {
        buf = nghttp3_mem_malloc((*conn).mem, datalen) as *mut uint8_t;
        if buf.is_null() {
            return NGHTTP3_ERR_NOMEM;
        }
        memcpy(
            buf as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            datalen,
        );
    }
    '_c2rust_label_2: {
        if !(*conn).tx.ctrl.is_null() {} else {
            __assert_fail(
                b"conn->tx.ctrl\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2973 as ::core::ffi::c_uint,
                b"int nghttp3_conn_set_client_stream_priority(nghttp3_conn *, int64_t, const uint8_t *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    rv = nghttp3_stream_frq_emplace((*conn).tx.ctrl, &raw mut fr);
    if rv != 0 as ::core::ffi::c_int {
        nghttp3_mem_free((*conn).mem, buf as *mut ::core::ffi::c_void);
        return rv;
    }
    (*fr).priority_update = nghttp3_frame_priority_update {
        r#type: NGHTTP3_FRAME_PRIORITY_UPDATE as uint64_t,
        pri_elem_id: stream_id,
        c2rust_unnamed: C2Rust_Unnamed_8 {
            c2rust_unnamed: C2Rust_Unnamed_9 {
                data: buf,
                datalen: datalen,
            },
        },
    };
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_set_server_stream_priority_versioned(
    mut conn: *mut nghttp3_conn,
    mut stream_id: int64_t,
    mut pri_version: ::core::ffi::c_int,
    mut pri: *const nghttp3_pri,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = ::core::ptr::null_mut::<nghttp3_stream>();
    '_c2rust_label: {
        if (*conn).server != 0 {} else {
            __assert_fail(
                b"conn->server\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2998 as ::core::ffi::c_uint,
                b"int nghttp3_conn_set_server_stream_priority_versioned(nghttp3_conn *, int64_t, int, const nghttp3_pri *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*pri).0.urgency
            < (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t
        {} else {
            __assert_fail(
                b"pri->urgency < NGHTTP3_URGENCY_LEVELS\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2999 as ::core::ffi::c_uint,
                b"int nghttp3_conn_set_server_stream_priority_versioned(nghttp3_conn *, int64_t, int, const nghttp3_pri *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*pri).0.inc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*pri).0.inc as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        {} else {
            __assert_fail(
                b"pri->inc == 0 || pri->inc == 1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                3000 as ::core::ffi::c_uint,
                b"int nghttp3_conn_set_server_stream_priority_versioned(nghttp3_conn *, int64_t, int, const nghttp3_pri *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if stream_id >= 0 as int64_t {} else {
            __assert_fail(
                b"stream_id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                3001 as ::core::ffi::c_uint,
                b"int nghttp3_conn_set_server_stream_priority_versioned(nghttp3_conn *, int64_t, int, const nghttp3_pri *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if stream_id
            <= ((1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong) as int64_t
        {} else {
            __assert_fail(
                b"stream_id <= (int64_t)NGHTTP3_MAX_VARINT\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                3002 as ::core::ffi::c_uint,
                b"int nghttp3_conn_set_server_stream_priority_versioned(nghttp3_conn *, int64_t, int, const nghttp3_pri *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if nghttp3_client_stream_bidi(stream_id) == 0 {
        return NGHTTP3_ERR_INVALID_ARGUMENT;
    }
    stream = nghttp3_conn_find_stream(conn, stream_id) as *mut nghttp3_stream;
    if stream.is_null() {
        return NGHTTP3_ERR_STREAM_NOT_FOUND;
    }
    (*stream).c2rust_unnamed.c2rust_unnamed.flags = ((*stream)
        .c2rust_unnamed
        .c2rust_unnamed
        .flags as ::core::ffi::c_uint | NGHTTP3_STREAM_FLAG_SERVER_PRIORITY_SET)
        as uint16_t;
    return conn_update_stream_priority(conn, stream, pri);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_is_drained(
    mut conn: *mut nghttp3_conn,
) -> ::core::ffi::c_int {
    return nghttp3_conn_is_drained2(conn);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_is_drained2(
    mut conn: *const nghttp3_conn,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if (*conn).server != 0 {} else {
            __assert_fail(
                b"conn->server\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_conn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                3023 as ::core::ffi::c_uint,
                b"int nghttp3_conn_is_drained2(const nghttp3_conn *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return ((*conn).flags as ::core::ffi::c_uint & NGHTTP3_CONN_FLAG_SHUTDOWN_COMMENCED
        != 0 && (*conn).remote.bidi.num_streams == 0 as size_t
        && nghttp3_stream_outq_write_done((*conn).tx.ctrl) != 0
        && nghttp3_ringbuf_len(
            &raw mut (*(*conn).tx.ctrl).c2rust_unnamed.c2rust_unnamed.frq,
        ) == 0 as size_t) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_conn_is_stream_flushed(
    mut conn: *const nghttp3_conn,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_stream = nghttp3_conn_find_stream(conn, stream_id);
    let mut fr: *const nghttp3_frame = ::core::ptr::null::<nghttp3_frame>();
    if stream.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    if nghttp3_stream_outq_write_done(stream) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if nghttp3_ringbuf_len(&raw mut (*stream).c2rust_unnamed.c2rust_unnamed.frq)
        == 0 as size_t
    {
        return 1 as ::core::ffi::c_int;
    }
    fr = nghttp3_ringbuf_get(
        &raw mut (*stream).c2rust_unnamed.c2rust_unnamed.frq,
        0 as size_t,
    ) as *const nghttp3_frame;
    return ((*fr).hd.r#type == NGHTTP3_FRAME_DATA as uint64_t) as ::core::ffi::c_int;
}
