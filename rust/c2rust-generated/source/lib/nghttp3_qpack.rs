extern "C" {
    fn nghttp3_rcbuf_incref(rcbuf: *mut nghttp3_rcbuf);
    fn nghttp3_rcbuf_decref(rcbuf: *mut nghttp3_rcbuf);
    fn nghttp3_buf_init(buf: *mut nghttp3_buf);
    fn nghttp3_buf_free(buf: *mut nghttp3_buf, mem: *const nghttp3_mem);
    fn nghttp3_buf_left(buf: *const nghttp3_buf) -> size_t;
    fn nghttp3_buf_len(buf: *const nghttp3_buf) -> size_t;
    fn nghttp3_buf_reset(buf: *mut nghttp3_buf);
    fn nghttp3_rcbuf_new(
        rcbuf_ptr: *mut *mut nghttp3_rcbuf,
        size: size_t,
        mem: *const nghttp3_mem,
    ) -> ::core::ffi::c_int;
    fn nghttp3_rcbuf_new2(
        rcbuf_ptr: *mut *mut nghttp3_rcbuf,
        src: *const uint8_t,
        srclen: size_t,
        mem: *const nghttp3_mem,
    ) -> ::core::ffi::c_int;
    fn nghttp3_mem_malloc(
        mem: *const nghttp3_mem,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn nghttp3_mem_free(mem: *const nghttp3_mem, ptr: *mut ::core::ffi::c_void);
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
    fn nghttp3_map_clear(map: *mut nghttp3_map);
    fn nghttp3_map_size(map: *const nghttp3_map) -> size_t;
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
    fn nghttp3_pq_empty(pq: *const nghttp3_pq) -> ::core::ffi::c_int;
    fn nghttp3_pq_remove(pq: *mut nghttp3_pq, item: *mut nghttp3_pq_entry);
    fn nghttp3_pq_clear(pq: *mut nghttp3_pq);
    fn nghttp3_ringbuf_init(
        rb: *mut nghttp3_ringbuf,
        nmemb: size_t,
        size: size_t,
        mem: *const nghttp3_mem,
    ) -> ::core::ffi::c_int;
    fn nghttp3_ringbuf_free(rb: *mut nghttp3_ringbuf);
    fn nghttp3_ringbuf_push_front(rb: *mut nghttp3_ringbuf) -> *mut ::core::ffi::c_void;
    fn nghttp3_ringbuf_push_back(rb: *mut nghttp3_ringbuf) -> *mut ::core::ffi::c_void;
    fn nghttp3_ringbuf_pop_front(rb: *mut nghttp3_ringbuf);
    fn nghttp3_ringbuf_pop_back(rb: *mut nghttp3_ringbuf);
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
    fn nghttp3_buf_reserve(
        buf: *mut nghttp3_buf,
        size: size_t,
        mem: *const nghttp3_mem,
    ) -> ::core::ffi::c_int;
    fn nghttp3_ksl_init(
        ksl: *mut nghttp3_ksl,
        compar: nghttp3_ksl_compar,
        search: nghttp3_ksl_search,
        keylen: size_t,
        mem: *const nghttp3_mem,
    );
    fn nghttp3_ksl_free(ksl: *mut nghttp3_ksl);
    fn nghttp3_ksl_insert(
        ksl: *mut nghttp3_ksl,
        it: *mut nghttp3_ksl_it,
        key: *const ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn nghttp3_ksl_remove_hint(
        ksl: *mut nghttp3_ksl,
        it: *mut nghttp3_ksl_it,
        hint: *const nghttp3_ksl_it,
        key: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn nghttp3_ksl_lower_bound(
        ksl: *const nghttp3_ksl,
        key: *const ::core::ffi::c_void,
    ) -> nghttp3_ksl_it;
    fn nghttp3_ksl_len(ksl: *const nghttp3_ksl) -> size_t;
    fn nghttp3_ksl_clear(ksl: *mut nghttp3_ksl);
    fn nghttp3_qpack_huffman_encode_count(src: *const uint8_t, len: size_t) -> size_t;
    fn nghttp3_qpack_huffman_encode(
        dest: *mut uint8_t,
        src: *const uint8_t,
        srclen: size_t,
    ) -> *mut uint8_t;
    fn nghttp3_qpack_huffman_decode_context_init(
        ctx: *mut nghttp3_qpack_huffman_decode_context,
    );
    fn nghttp3_qpack_huffman_decode(
        ctx: *mut nghttp3_qpack_huffman_decode_context,
        dest: *mut uint8_t,
        src: *const uint8_t,
        srclen: size_t,
        fin: ::core::ffi::c_int,
    ) -> nghttp3_ssize;
    fn nghttp3_qpack_huffman_decode_failure_state(
        ctx: *const nghttp3_qpack_huffman_decode_context,
    ) -> ::core::ffi::c_int;
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
    fn nghttp3_cpymem(
        dest: *mut uint8_t,
        src: *const uint8_t,
        n: size_t,
    ) -> *mut uint8_t;
    fn nghttp3_unreachable_fail(
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
    ) -> !;
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
pub struct nghttp3_qpack_header_block_ref {
    pub max_cnts_pe: nghttp3_pq_entry,
    pub min_cnts_pe: nghttp3_pq_entry,
    pub max_cnt: uint64_t,
    pub min_cnt: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_blocked_streams_key {
    pub max_cnt: uint64_t,
    pub id: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_qpack_stream {
    pub stream_id: int64_t,
    pub refs: nghttp3_ringbuf,
    pub max_cnts: nghttp3_pq,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_ksl_it {
    pub blk: *mut nghttp3_ksl_blk,
    pub i: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_qpack_lookup_result {
    pub index: nghttp3_ssize,
    pub name_value_match: ::core::ffi::c_int,
    pub pb_index: nghttp3_ssize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_qpack_static_header {
    pub name: nghttp3_rcbuf,
    pub value: nghttp3_rcbuf,
    pub token: int32_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nghttp3_qpack_indexing_mode(pub ::core::ffi::c_uint);
impl nghttp3_qpack_indexing_mode {
    pub const NGHTTP3_QPACK_INDEXING_MODE_LITERAL: Self = Self(0);
    pub const NGHTTP3_QPACK_INDEXING_MODE_STORE: Self = Self(1);
    pub const NGHTTP3_QPACK_INDEXING_MODE_NEVER: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_qpack_static_entry {
    pub absidx: uint64_t,
    pub token: int32_t,
    pub hash: uint32_t,
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
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NGHTTP3_ERR_INVALID_ARGUMENT: ::core::ffi::c_int = -101 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_QPACK_FATAL: ::core::ffi::c_int = -108 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_QPACK_HEADER_TOO_LARGE: ::core::ffi::c_int = -109
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED: ::core::ffi::c_int = -401
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR: ::core::ffi::c_int = -402
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_QPACK_DECODER_STREAM_ERROR: ::core::ffi::c_int = -403
    as ::core::ffi::c_int;
pub const NGHTTP3_ERR_NOMEM: ::core::ffi::c_int = -901 as ::core::ffi::c_int;
pub const NGHTTP3_NV_FLAG_NONE: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const NGHTTP3_NV_FLAG_NEVER_INDEX: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const NGHTTP3_NV_FLAG_TRY_INDEX: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const NGHTTP3_QPACK_DECODE_FLAG_NONE: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const NGHTTP3_QPACK_DECODE_FLAG_EMIT: ::core::ffi::c_uint = 0x1
    as ::core::ffi::c_uint;
pub const NGHTTP3_QPACK_DECODE_FLAG_FINAL: ::core::ffi::c_uint = 0x2
    as ::core::ffi::c_uint;
pub const NGHTTP3_QPACK_DECODE_FLAG_BLOCKED: ::core::ffi::c_uint = 0x4
    as ::core::ffi::c_uint;
pub const NGHTTP3_PQ_BAD_INDEX: ::core::ffi::c_ulong = SIZE_MAX;
#[inline]
unsafe extern "C" fn nghttp3_ringbuf_len(mut rb: *const nghttp3_ringbuf) -> size_t {
    return (*rb).len;
}
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
#[inline]
unsafe extern "C" fn nghttp3_ksl_blk_nth_key(
    mut blk: *const nghttp3_ksl_blk,
    mut n: size_t,
) -> *const ::core::ffi::c_void {
    return (*blk)
        .c2rust_unnamed
        .c2rust_unnamed
        .keys
        .offset(
            n.wrapping_mul((*blk).c2rust_unnamed.c2rust_unnamed.aligned_keylen as size_t)
                as isize,
        ) as *const ::core::ffi::c_void;
}
#[inline]
unsafe extern "C" fn nghttp3_ksl_it_get(
    mut it: *const nghttp3_ksl_it,
) -> *mut ::core::ffi::c_void {
    return (*(*it).blk).c2rust_unnamed.c2rust_unnamed.nodes[(*it).i].c2rust_unnamed.data;
}
#[inline]
unsafe extern "C" fn nghttp3_ksl_it_end(
    mut it: *const nghttp3_ksl_it,
) -> ::core::ffi::c_int {
    return ((*(*it).blk).c2rust_unnamed.c2rust_unnamed.n as size_t == (*it).i
        && (*(*it).blk).c2rust_unnamed.c2rust_unnamed.next.is_null())
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn nghttp3_ksl_it_key(
    mut it: *const nghttp3_ksl_it,
) -> *const ::core::ffi::c_void {
    return nghttp3_ksl_blk_nth_key((*it).blk, (*it).i);
}
#[inline]
unsafe extern "C" fn nghttp3_qpack_huffman_estimate_decode_length(
    mut len: size_t,
) -> size_t {
    return len.wrapping_mul(8 as size_t).wrapping_div(5 as size_t);
}
pub const NGHTTP3_QPACK_INT_MAX: ::core::ffi::c_ulonglong = ((1
    as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int)
    .wrapping_sub(1 as ::core::ffi::c_ulonglong);
pub const NGHTTP3_QPACK_MAX_NAMELEN: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NGHTTP3_QPACK_MAX_VALUELEN: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const NGHTTP3_QPACK_MAX_ENCODERLEN: ::core::ffi::c_int = 128 as ::core::ffi::c_int
    * 1024 as ::core::ffi::c_int;
pub const NGHTTP3_QPACK_MAX_DECODERLEN: ::core::ffi::c_int = 4 as ::core::ffi::c_int
    * 1024 as ::core::ffi::c_int;
pub const NGHTTP3_QPACK_ENTRY_OVERHEAD: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const NGHTTP3_QPACK_MAP_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NGHTTP3_QPACK_ENCODER_FLAG_NONE: ::core::ffi::c_uint = 0
    as ::core::ffi::c_uint;
pub const NGHTTP3_QPACK_ENCODER_FLAG_PENDING_SET_DTABLE_CAP: ::core::ffi::c_uint = 0x1
    as ::core::ffi::c_uint;
pub const NGHTTP3_QPACK_MAX_QPACK_STREAMS: ::core::ffi::c_int = 2000
    as ::core::ffi::c_int;
static mut token_stable: [nghttp3_qpack_static_entry; 99] = [
    nghttp3_qpack_static_entry {
        absidx: 0 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__AUTHORITY.0 as int32_t,
        hash: 3153725150 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 15 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0 as int32_t,
        hash: 695666056 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 16 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0 as int32_t,
        hash: 695666056 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 17 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0 as int32_t,
        hash: 695666056 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 18 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0 as int32_t,
        hash: 695666056 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 19 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0 as int32_t,
        hash: 695666056 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 20 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0 as int32_t,
        hash: 695666056 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 21 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0 as int32_t,
        hash: 695666056 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 1 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__PATH.0 as int32_t,
        hash: 3292848686 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 22 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__SCHEME.0 as int32_t,
        hash: 2510477674 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 23 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__SCHEME.0 as int32_t,
        hash: 2510477674 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 24 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        hash: 4000288983 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 25 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        hash: 4000288983 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 26 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        hash: 4000288983 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 27 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        hash: 4000288983 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 28 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        hash: 4000288983 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 63 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        hash: 4000288983 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 64 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        hash: 4000288983 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 65 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        hash: 4000288983 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 66 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        hash: 4000288983 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 67 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        hash: 4000288983 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 68 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        hash: 4000288983 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 69 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        hash: 4000288983 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 70 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        hash: 4000288983 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 71 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        hash: 4000288983 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 29 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCEPT.0 as int32_t,
        hash: 136609321 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 30 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCEPT.0 as int32_t,
        hash: 136609321 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 31 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCEPT_ENCODING.0 as int32_t,
        hash: 3379649177 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 72 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCEPT_LANGUAGE.0 as int32_t,
        hash: 1979086614 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 32 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCEPT_RANGES.0 as int32_t,
        hash: 1713753958 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 73 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_CREDENTIALS
            .0 as int32_t,
        hash: 901040780 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 74 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_CREDENTIALS
            .0 as int32_t,
        hash: 901040780 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 33 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_HEADERS.0
            as int32_t,
        hash: 1524311232 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 34 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_HEADERS.0
            as int32_t,
        hash: 1524311232 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 75 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_HEADERS.0
            as int32_t,
        hash: 1524311232 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 76 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_METHODS.0
            as int32_t,
        hash: 2175229868 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 77 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_METHODS.0
            as int32_t,
        hash: 2175229868 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 78 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_METHODS.0
            as int32_t,
        hash: 2175229868 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 35 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_ORIGIN.0
            as int32_t,
        hash: 2710797292 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 79 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_EXPOSE_HEADERS.0
            as int32_t,
        hash: 2449824425 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 80 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_REQUEST_HEADERS.0
            as int32_t,
        hash: 3599549072 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 81 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_REQUEST_METHOD.0
            as int32_t,
        hash: 2417078055 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 82 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_REQUEST_METHOD.0
            as int32_t,
        hash: 2417078055 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 2 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_AGE.0 as int32_t,
        hash: 742476188 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 83 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ALT_SVC.0 as int32_t,
        hash: 2148877059 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 84 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_AUTHORIZATION.0 as int32_t,
        hash: 2436257726 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 36 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CACHE_CONTROL.0 as int32_t,
        hash: 1355326669 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 37 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CACHE_CONTROL.0 as int32_t,
        hash: 1355326669 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 38 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CACHE_CONTROL.0 as int32_t,
        hash: 1355326669 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 39 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CACHE_CONTROL.0 as int32_t,
        hash: 1355326669 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 40 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CACHE_CONTROL.0 as int32_t,
        hash: 1355326669 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 41 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CACHE_CONTROL.0 as int32_t,
        hash: 1355326669 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 3 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_DISPOSITION.0 as int32_t,
        hash: 3889184348 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 42 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_ENCODING.0 as int32_t,
        hash: 65203592 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 43 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_ENCODING.0 as int32_t,
        hash: 65203592 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 4 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_LENGTH.0 as int32_t,
        hash: 1308181789 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 85 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_SECURITY_POLICY.0
            as int32_t,
        hash: 1569039836 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 44 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        hash: 4244048277 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 45 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        hash: 4244048277 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 46 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        hash: 4244048277 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 47 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        hash: 4244048277 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 48 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        hash: 4244048277 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 49 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        hash: 4244048277 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 50 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        hash: 4244048277 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 51 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        hash: 4244048277 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 52 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        hash: 4244048277 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 53 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        hash: 4244048277 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 54 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        hash: 4244048277 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 5 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_COOKIE.0 as int32_t,
        hash: 2007449791 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 6 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_DATE.0 as int32_t,
        hash: 3564297305 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 86 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_EARLY_DATA.0 as int32_t,
        hash: 4080895051 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 7 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ETAG.0 as int32_t,
        hash: 113792960 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 87 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_EXPECT_CT.0 as int32_t,
        hash: 1183214960 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 88 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_FORWARDED.0 as int32_t,
        hash: 1485178027 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 8 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_IF_MODIFIED_SINCE.0 as int32_t,
        hash: 2213050793 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 9 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_IF_NONE_MATCH.0 as int32_t,
        hash: 2536202615 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 89 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_IF_RANGE.0 as int32_t,
        hash: 2340978238 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 10 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_LAST_MODIFIED.0 as int32_t,
        hash: 3226950251 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 11 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_LINK.0 as int32_t,
        hash: 232457833 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 12 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_LOCATION.0 as int32_t,
        hash: 200649126 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 90 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ORIGIN.0 as int32_t,
        hash: 3649018447 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 91 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_PURPOSE.0 as int32_t,
        hash: 4212263681 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 55 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_RANGE.0 as int32_t,
        hash: 4208725202 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 13 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_REFERER.0 as int32_t,
        hash: 3969579366 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 92 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_SERVER.0 as int32_t,
        hash: 1085029842 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 14 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_SET_COOKIE.0 as int32_t,
        hash: 1848371000 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 56 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_STRICT_TRANSPORT_SECURITY.0
            as int32_t,
        hash: 4138147361 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 57 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_STRICT_TRANSPORT_SECURITY.0
            as int32_t,
        hash: 4138147361 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 58 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_STRICT_TRANSPORT_SECURITY.0
            as int32_t,
        hash: 4138147361 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 93 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_TIMING_ALLOW_ORIGIN.0 as int32_t,
        hash: 2432297564 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 94 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_UPGRADE_INSECURE_REQUESTS.0
            as int32_t,
        hash: 2479169413 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 95 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_USER_AGENT.0 as int32_t,
        hash: 606444526 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 59 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_VARY.0 as int32_t,
        hash: 1085005381 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 60 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_VARY.0 as int32_t,
        hash: 1085005381 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 61 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_X_CONTENT_TYPE_OPTIONS.0
            as int32_t,
        hash: 3644557769 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 96 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_X_FORWARDED_FOR.0 as int32_t,
        hash: 2914187656 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 97 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_X_FRAME_OPTIONS.0 as int32_t,
        hash: 3993834824 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 98 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_X_FRAME_OPTIONS.0 as int32_t,
        hash: 3993834824 as uint32_t,
    },
    nghttp3_qpack_static_entry {
        absidx: 62 as uint64_t,
        token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_X_XSS_PROTECTION.0 as int32_t,
        hash: 2501058888 as uint32_t,
    },
];
static mut stable: [nghttp3_qpack_static_header; 99] = [nghttp3_qpack_static_header {
    name: nghttp3_rcbuf {
        mem: ::core::ptr::null::<nghttp3_mem>(),
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
        r#ref: 0,
    },
    value: nghttp3_rcbuf {
        mem: ::core::ptr::null::<nghttp3_mem>(),
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
        r#ref: 0,
    },
    token: 0,
}; 99];
unsafe extern "C" fn memeq(
    mut s1: *const ::core::ffi::c_void,
    mut s2: *const ::core::ffi::c_void,
    mut n: size_t,
) -> ::core::ffi::c_int {
    return (n == 0 as size_t || memcmp(s1, s2, n) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn qpack_lookup_token(
    mut name: *const uint8_t,
    mut namelen: size_t,
) -> int32_t {
    match namelen {
        2 => {
            match *name.offset(1isize) as ::core::ffi::c_int {
                101 => {
                    if memeq(
                        b"t\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        1 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_TE.0 as int32_t;
                    }
                }
                _ => {}
            }
        }
        3 => {
            match *name.offset(2isize) as ::core::ffi::c_int {
                101 => {
                    if memeq(
                        b"ag\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        2 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_AGE.0 as int32_t;
                    }
                }
                _ => {}
            }
        }
        4 => {
            match *name.offset(3isize) as ::core::ffi::c_int {
                101 => {
                    if memeq(
                        b"dat\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        3 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_DATE.0
                            as int32_t;
                    }
                }
                103 => {
                    if memeq(
                        b"eta\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        3 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ETAG.0
                            as int32_t;
                    }
                }
                107 => {
                    if memeq(
                        b"lin\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        3 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_LINK.0
                            as int32_t;
                    }
                }
                116 => {
                    if memeq(
                        b"hos\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        3 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_HOST.0
                            as int32_t;
                    }
                }
                121 => {
                    if memeq(
                        b"var\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        3 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_VARY.0
                            as int32_t;
                    }
                }
                _ => {}
            }
        }
        5 => {
            match *name.offset(4isize) as ::core::ffi::c_int {
                101 => {
                    if memeq(
                        b"rang\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        4 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_RANGE.0
                            as int32_t;
                    }
                }
                104 => {
                    if memeq(
                        b":pat\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        4 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__PATH.0
                            as int32_t;
                    }
                }
                _ => {}
            }
        }
        6 => {
            match *name.offset(5isize) as ::core::ffi::c_int {
                101 => {
                    if memeq(
                        b"cooki\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        5 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_COOKIE.0
                            as int32_t;
                    }
                }
                110 => {
                    if memeq(
                        b"origi\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        5 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ORIGIN.0
                            as int32_t;
                    }
                }
                114 => {
                    if memeq(
                        b"serve\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        5 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_SERVER.0
                            as int32_t;
                    }
                }
                116 => {
                    if memeq(
                        b"accep\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        5 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCEPT.0
                            as int32_t;
                    }
                }
                _ => {}
            }
        }
        7 => {
            match *name.offset(6isize) as ::core::ffi::c_int {
                99 => {
                    if memeq(
                        b"alt-sv\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        6 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ALT_SVC.0
                            as int32_t;
                    }
                }
                100 => {
                    if memeq(
                        b":metho\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        6 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0
                            as int32_t;
                    }
                }
                101 => {
                    if memeq(
                        b":schem\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        6 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__SCHEME.0
                            as int32_t;
                    }
                    if memeq(
                        b"purpos\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        6 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_PURPOSE.0
                            as int32_t;
                    }
                    if memeq(
                        b"upgrad\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        6 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_UPGRADE.0
                            as int32_t;
                    }
                }
                114 => {
                    if memeq(
                        b"refere\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        6 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_REFERER.0
                            as int32_t;
                    }
                }
                115 => {
                    if memeq(
                        b":statu\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        6 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0
                            as int32_t;
                    }
                }
                _ => {}
            }
        }
        8 => {
            match *name.offset(7isize) as ::core::ffi::c_int {
                101 => {
                    if memeq(
                        b"if-rang\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        7 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_IF_RANGE.0
                            as int32_t;
                    }
                }
                110 => {
                    if memeq(
                        b"locatio\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        7 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_LOCATION.0
                            as int32_t;
                    }
                }
                121 => {
                    if memeq(
                        b"priorit\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        7 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_PRIORITY.0
                            as int32_t;
                    }
                }
                _ => {}
            }
        }
        9 => {
            match *name.offset(8isize) as ::core::ffi::c_int {
                100 => {
                    if memeq(
                        b"forwarde\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        8 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_FORWARDED.0
                            as int32_t;
                    }
                }
                108 => {
                    if memeq(
                        b":protoco\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        8 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__PROTOCOL.0
                            as int32_t;
                    }
                }
                116 => {
                    if memeq(
                        b"expect-c\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        8 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_EXPECT_CT.0
                            as int32_t;
                    }
                }
                _ => {}
            }
        }
        10 => {
            match *name.offset(9isize) as ::core::ffi::c_int {
                97 => {
                    if memeq(
                        b"early-dat\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        9 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_EARLY_DATA.0
                            as int32_t;
                    }
                }
                101 => {
                    if memeq(
                        b"keep-aliv\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        9 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_KEEP_ALIVE.0
                            as int32_t;
                    }
                    if memeq(
                        b"set-cooki\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        9 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_SET_COOKIE.0
                            as int32_t;
                    }
                }
                110 => {
                    if memeq(
                        b"connectio\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        9 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONNECTION.0
                            as int32_t;
                    }
                }
                116 => {
                    if memeq(
                        b"user-agen\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        9 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_USER_AGENT.0
                            as int32_t;
                    }
                }
                121 => {
                    if memeq(
                        b":authorit\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        9 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__AUTHORITY.0
                            as int32_t;
                    }
                }
                _ => {}
            }
        }
        12 => {
            match *name.offset(11isize) as ::core::ffi::c_int {
                101 => {
                    if memeq(
                        b"content-typ\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        11 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0
                            as int32_t;
                    }
                }
                _ => {}
            }
        }
        13 => {
            match *name.offset(12isize) as ::core::ffi::c_int {
                100 => {
                    if memeq(
                        b"last-modifie\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        12 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_LAST_MODIFIED.0
                            as int32_t;
                    }
                }
                104 => {
                    if memeq(
                        b"if-none-matc\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        12 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_IF_NONE_MATCH.0
                            as int32_t;
                    }
                }
                108 => {
                    if memeq(
                        b"cache-contro\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        12 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CACHE_CONTROL.0
                            as int32_t;
                    }
                }
                110 => {
                    if memeq(
                        b"authorizatio\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        12 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_AUTHORIZATION.0
                            as int32_t;
                    }
                }
                115 => {
                    if memeq(
                        b"accept-range\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        12 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCEPT_RANGES.0
                            as int32_t;
                    }
                }
                _ => {}
            }
        }
        14 => {
            match *name.offset(13isize) as ::core::ffi::c_int {
                104 => {
                    if memeq(
                        b"content-lengt\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        13 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_LENGTH.0
                            as int32_t;
                    }
                }
                _ => {}
            }
        }
        15 => {
            match *name.offset(14isize) as ::core::ffi::c_int {
                101 => {
                    if memeq(
                        b"accept-languag\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        14 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCEPT_LANGUAGE.0
                            as int32_t;
                    }
                }
                103 => {
                    if memeq(
                        b"accept-encodin\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        14 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCEPT_ENCODING.0
                            as int32_t;
                    }
                }
                114 => {
                    if memeq(
                        b"x-forwarded-fo\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        14 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_X_FORWARDED_FOR.0
                            as int32_t;
                    }
                }
                115 => {
                    if memeq(
                        b"x-frame-option\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        14 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_X_FRAME_OPTIONS.0
                            as int32_t;
                    }
                }
                _ => {}
            }
        }
        16 => {
            match *name.offset(15isize) as ::core::ffi::c_int {
                103 => {
                    if memeq(
                        b"content-encodin\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        15 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_ENCODING
                            .0 as int32_t;
                    }
                }
                110 => {
                    if memeq(
                        b"proxy-connectio\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        15 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_PROXY_CONNECTION
                            .0 as int32_t;
                    }
                    if memeq(
                        b"x-xss-protectio\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        15 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_X_XSS_PROTECTION
                            .0 as int32_t;
                    }
                }
                _ => {}
            }
        }
        17 => {
            match *name.offset(16isize) as ::core::ffi::c_int {
                101 => {
                    if memeq(
                        b"if-modified-sinc\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        16 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_IF_MODIFIED_SINCE
                            .0 as int32_t;
                    }
                }
                103 => {
                    if memeq(
                        b"transfer-encodin\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        16 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_TRANSFER_ENCODING
                            .0 as int32_t;
                    }
                }
                _ => {}
            }
        }
        19 => {
            match *name.offset(18isize) as ::core::ffi::c_int {
                110 => {
                    if memeq(
                        b"content-dispositio\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        18 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_DISPOSITION
                            .0 as int32_t;
                    }
                    if memeq(
                        b"timing-allow-origi\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        18 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_TIMING_ALLOW_ORIGIN
                            .0 as int32_t;
                    }
                }
                _ => {}
            }
        }
        22 => {
            match *name.offset(21isize) as ::core::ffi::c_int {
                115 => {
                    if memeq(
                        b"x-content-type-option\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        21 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_X_CONTENT_TYPE_OPTIONS
                            .0 as int32_t;
                    }
                }
                _ => {}
            }
        }
        23 => {
            match *name.offset(22isize) as ::core::ffi::c_int {
                121 => {
                    if memeq(
                        b"content-security-polic\0".as_ptr()
                            as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        22 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_SECURITY_POLICY
                            .0 as int32_t;
                    }
                }
                _ => {}
            }
        }
        25 => {
            match *name.offset(24isize) as ::core::ffi::c_int {
                115 => {
                    if memeq(
                        b"upgrade-insecure-request\0".as_ptr()
                            as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        24 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_UPGRADE_INSECURE_REQUESTS
                            .0 as int32_t;
                    }
                }
                121 => {
                    if memeq(
                        b"strict-transport-securit\0".as_ptr()
                            as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        24 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_STRICT_TRANSPORT_SECURITY
                            .0 as int32_t;
                    }
                }
                _ => {}
            }
        }
        27 => {
            match *name.offset(26isize) as ::core::ffi::c_int {
                110 => {
                    if memeq(
                        b"access-control-allow-origi\0".as_ptr()
                            as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        26 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_ORIGIN
                            .0 as int32_t;
                    }
                }
                _ => {}
            }
        }
        28 => {
            match *name.offset(27isize) as ::core::ffi::c_int {
                115 => {
                    if memeq(
                        b"access-control-allow-header\0".as_ptr()
                            as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        27 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_HEADERS
                            .0 as int32_t;
                    }
                    if memeq(
                        b"access-control-allow-method\0".as_ptr()
                            as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        27 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_METHODS
                            .0 as int32_t;
                    }
                }
                _ => {}
            }
        }
        29 => {
            match *name.offset(28isize) as ::core::ffi::c_int {
                100 => {
                    if memeq(
                        b"access-control-request-metho\0".as_ptr()
                            as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        28 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_REQUEST_METHOD
                            .0 as int32_t;
                    }
                }
                115 => {
                    if memeq(
                        b"access-control-expose-header\0".as_ptr()
                            as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        28 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_EXPOSE_HEADERS
                            .0 as int32_t;
                    }
                }
                _ => {}
            }
        }
        30 => {
            match *name.offset(29isize) as ::core::ffi::c_int {
                115 => {
                    if memeq(
                        b"access-control-request-header\0".as_ptr()
                            as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        29 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_REQUEST_HEADERS
                            .0 as int32_t;
                    }
                }
                _ => {}
            }
        }
        32 => {
            match *name.offset(31isize) as ::core::ffi::c_int {
                115 => {
                    if memeq(
                        b"access-control-allow-credential\0".as_ptr()
                            as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        31 as size_t,
                    ) != 0
                    {
                        return nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_CREDENTIALS
                            .0 as int32_t;
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
    return -1 as int32_t;
}
unsafe extern "C" fn table_space(mut namelen: size_t, mut valuelen: size_t) -> size_t {
    return (NGHTTP3_QPACK_ENTRY_OVERHEAD as size_t)
        .wrapping_add(namelen)
        .wrapping_add(valuelen);
}
unsafe extern "C" fn qpack_nv_name_eq(
    mut a: *const nghttp3_qpack_nv,
    mut b: *const nghttp3_nv,
) -> ::core::ffi::c_int {
    return ((*(*a).name).len == (*b).namelen
        && memeq(
            (*(*a).name).base as *const ::core::ffi::c_void,
            (*b).name as *const ::core::ffi::c_void,
            (*b).namelen,
        ) != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn qpack_nv_value_eq(
    mut a: *const nghttp3_qpack_nv,
    mut b: *const nghttp3_nv,
) -> ::core::ffi::c_int {
    return ((*(*a).value).len == (*b).valuelen
        && memeq(
            (*(*a).value).base as *const ::core::ffi::c_void,
            (*b).value as *const ::core::ffi::c_void,
            (*b).valuelen,
        ) != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn qpack_map_init(mut map: *mut nghttp3_qpack_map) {
    *map = nghttp3_qpack_map {
        table: [
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
            ::core::ptr::null_mut::<nghttp3_qpack_entry>(),
        ],
    };
}
unsafe extern "C" fn qpack_map_insert(
    mut map: *mut nghttp3_qpack_map,
    mut ent: *mut nghttp3_qpack_entry,
) {
    let mut bucket: *mut *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        *mut nghttp3_qpack_entry,
    >();
    bucket = (&raw mut (*map).table as *mut *mut nghttp3_qpack_entry)
        .offset(
            ((*ent).hash
                & (NGHTTP3_QPACK_MAP_SIZE - 1 as ::core::ffi::c_int) as uint32_t)
                as isize,
        );
    if (*bucket).is_null() {
        *bucket = ent;
        return;
    }
    (*ent).map_next = *bucket;
    *bucket = ent;
}
unsafe extern "C" fn qpack_map_remove(
    mut map: *mut nghttp3_qpack_map,
    mut ent: *mut nghttp3_qpack_entry,
) {
    let mut dst: *mut *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        *mut nghttp3_qpack_entry,
    >();
    dst = (&raw mut (*map).table as *mut *mut nghttp3_qpack_entry)
        .offset(
            ((*ent).hash
                & (NGHTTP3_QPACK_MAP_SIZE - 1 as ::core::ffi::c_int) as uint32_t)
                as isize,
        );
    while !(*dst).is_null() {
        if *dst != ent {
            dst = &raw mut (**dst).map_next;
        } else {
            *dst = (*ent).map_next;
            (*ent).map_next = ::core::ptr::null_mut::<nghttp3_qpack_entry>();
            return;
        }
    }
}
unsafe extern "C" fn qpack_context_can_reference(
    mut ctx: *mut nghttp3_qpack_context,
    mut absidx: uint64_t,
) -> ::core::ffi::c_int {
    let mut ent: *mut nghttp3_qpack_entry = nghttp3_qpack_context_dtable_get(
        ctx,
        absidx,
    );
    return ((*ctx).dtable_sum.wrapping_sub((*ent).sum) <= (*ctx).max_dtable_capacity)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn encoder_qpack_map_find(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut exact_match: *mut ::core::ffi::c_int,
    mut pmatch: *mut *mut nghttp3_qpack_entry,
    mut ppb_match: *mut *mut nghttp3_qpack_entry,
    mut nv: *const nghttp3_nv,
    mut token: int32_t,
    mut hash: uint32_t,
    mut krcnt: uint64_t,
    mut allow_blocking: ::core::ffi::c_int,
    mut name_only: ::core::ffi::c_int,
) {
    let mut p: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<nghttp3_qpack_entry>();
    *exact_match = 0 as ::core::ffi::c_int;
    *pmatch = ::core::ptr::null_mut::<nghttp3_qpack_entry>();
    *ppb_match = ::core::ptr::null_mut::<nghttp3_qpack_entry>();
    p = (*encoder)
        .dtable_map
        .table[(hash & (NGHTTP3_QPACK_MAP_SIZE - 1 as ::core::ffi::c_int) as uint32_t)
        as usize];
    while !p.is_null() {
        if !(token != (*p).nv.token
            || token == -1 as int32_t
                && (hash != (*p).hash || qpack_nv_name_eq(&raw mut (*p).nv, nv) == 0)
            || qpack_context_can_reference(&raw mut (*encoder).ctx, (*p).absidx) == 0)
        {
            if allow_blocking != 0 || (*p).absidx.wrapping_add(1 as uint64_t) <= krcnt {
                if (*pmatch).is_null() {
                    *pmatch = p;
                    if name_only != 0 {
                        return;
                    }
                }
                if qpack_nv_value_eq(&raw mut (*p).nv, nv) != 0 {
                    *pmatch = p;
                    *exact_match = 1 as ::core::ffi::c_int;
                    return;
                }
            } else if (*ppb_match).is_null()
                && qpack_nv_value_eq(&raw mut (*p).nv, nv) != 0
            {
                *ppb_match = p;
            }
        }
        p = (*p).map_next;
    }
}
unsafe extern "C" fn qpack_context_init(
    mut ctx: *mut nghttp3_qpack_context,
    mut hard_max_dtable_capacity: size_t,
    mut max_blocked_streams: size_t,
    mut mem: *const nghttp3_mem,
) {
    nghttp3_ringbuf_init(
        &raw mut (*ctx).dtable,
        0 as size_t,
        ::core::mem::size_of::<*mut nghttp3_qpack_entry>(),
        mem,
    );
    (*ctx).mem = mem;
    (*ctx).dtable_size = 0 as size_t;
    (*ctx).dtable_sum = 0 as size_t;
    (*ctx).hard_max_dtable_capacity = hard_max_dtable_capacity;
    (*ctx).max_dtable_capacity = 0 as size_t;
    (*ctx).max_blocked_streams = max_blocked_streams;
    (*ctx).next_absidx = 0 as uint64_t;
    (*ctx).bad = 0 as uint8_t;
}
unsafe extern "C" fn qpack_context_free(mut ctx: *mut nghttp3_qpack_context) {
    let mut ent: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    let mut i: size_t = 0;
    let mut len: size_t = nghttp3_ringbuf_len(&raw mut (*ctx).dtable);
    i = 0 as size_t;
    while i < len {
        ent = *(nghttp3_ringbuf_get(&raw mut (*ctx).dtable, i)
            as *mut *mut nghttp3_qpack_entry);
        nghttp3_qpack_entry_free(ent);
        nghttp3_mem_free((*ctx).mem, ent as *mut ::core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    nghttp3_ringbuf_free(&raw mut (*ctx).dtable);
}
unsafe extern "C" fn ref_min_cnt_less(
    mut lhsx: *const nghttp3_pq_entry,
    mut rhsx: *const nghttp3_pq_entry,
) -> ::core::ffi::c_int {
    let mut lhs: *mut nghttp3_qpack_header_block_ref = (lhsx as *mut ::core::ffi::c_char)
        .offset(-(8 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_qpack_header_block_ref;
    let mut rhs: *mut nghttp3_qpack_header_block_ref = (rhsx as *mut ::core::ffi::c_char)
        .offset(-(8 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_qpack_header_block_ref;
    return ((*lhs).min_cnt < (*rhs).min_cnt) as ::core::ffi::c_int;
}
unsafe extern "C" fn max_cnt_greater(
    mut lhs: *const ::core::ffi::c_void,
    mut rhs: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *const nghttp3_blocked_streams_key = lhs
        as *const nghttp3_blocked_streams_key;
    let mut b: *const nghttp3_blocked_streams_key = rhs
        as *const nghttp3_blocked_streams_key;
    return ((*a).max_cnt > (*b).max_cnt
        || (*a).max_cnt == (*b).max_cnt && (*a).id < (*b).id) as ::core::ffi::c_int;
}
unsafe extern "C" fn ksl_max_cnt_greater_search(
    mut ksl: *const nghttp3_ksl,
    mut blk: *mut nghttp3_ksl_blk,
    mut key: *const ::core::ffi::c_void,
) -> size_t {
    let mut i: size_t = 0;
    let mut node_key: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    i = 0 as size_t;
    node_key = (*blk).c2rust_unnamed.c2rust_unnamed.keys;
    while i < (*blk).c2rust_unnamed.c2rust_unnamed.n as size_t
        && max_cnt_greater(node_key as *const ::core::ffi::c_void, key) != 0
    {
        i = i.wrapping_add(1);
        node_key = node_key.offset((*ksl).aligned_keylen as isize);
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_init(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut hard_max_dtable_capacity: size_t,
    mut seed: uint64_t,
    mut mem: *const nghttp3_mem,
) {
    qpack_context_init(
        &raw mut (*encoder).ctx,
        hard_max_dtable_capacity,
        0 as size_t,
        mem,
    );
    nghttp3_map_init(&raw mut (*encoder).streams, seed, mem);
    nghttp3_ksl_init(
        &raw mut (*encoder).blocked_streams,
        Some(
            max_cnt_greater
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            ksl_max_cnt_greater_search
                as unsafe extern "C" fn(
                    *const nghttp3_ksl,
                    *mut nghttp3_ksl_blk,
                    *const ::core::ffi::c_void,
                ) -> size_t,
        ),
        ::core::mem::size_of::<nghttp3_blocked_streams_key>(),
        mem,
    );
    qpack_map_init(&raw mut (*encoder).dtable_map);
    nghttp3_pq_init(
        &raw mut (*encoder).min_cnts,
        Some(
            ref_min_cnt_less
                as unsafe extern "C" fn(
                    *const nghttp3_pq_entry,
                    *const nghttp3_pq_entry,
                ) -> ::core::ffi::c_int,
        ),
        mem,
    );
    (*encoder).krcnt = 0 as uint64_t;
    (*encoder).state = nghttp3_qpack_decoder_stream_state::NGHTTP3_QPACK_DS_STATE_OPCODE;
    (*encoder).opcode = nghttp3_qpack_decoder_stream_opcode(
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*encoder).min_dtable_update = SIZE_MAX as size_t;
    (*encoder).last_max_dtable_update = 0 as size_t;
    (*encoder).uninterrupted_decoderlen = 0 as size_t;
    (*encoder).indexing_strat = nghttp3_qpack_indexing_strat::NGHTTP3_QPACK_INDEXING_STRAT_NONE;
    (*encoder).flags = NGHTTP3_QPACK_ENCODER_FLAG_NONE as uint8_t;
    nghttp3_qpack_read_state_reset(&raw mut (*encoder).rstate);
}
unsafe extern "C" fn map_stream_free(
    mut data: *mut ::core::ffi::c_void,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut mem: *const nghttp3_mem = ptr as *const nghttp3_mem;
    let mut stream: *mut nghttp3_qpack_stream = data as *mut nghttp3_qpack_stream;
    nghttp3_qpack_stream_del(stream, mem);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_free(
    mut encoder: *mut nghttp3_qpack_encoder,
) {
    nghttp3_pq_free(&raw mut (*encoder).min_cnts);
    nghttp3_ksl_free(&raw mut (*encoder).blocked_streams);
    nghttp3_map_each(
        &raw mut (*encoder).streams,
        Some(
            map_stream_free
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        (*encoder).ctx.mem as *mut ::core::ffi::c_void,
    );
    nghttp3_map_free(&raw mut (*encoder).streams);
    qpack_context_free(&raw mut (*encoder).ctx);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_set_max_dtable_capacity(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut max_dtable_capacity: size_t,
) {
    max_dtable_capacity = nghttp3_min_unsigned_long_int(
        max_dtable_capacity as ::core::ffi::c_ulong,
        (*encoder).ctx.hard_max_dtable_capacity as ::core::ffi::c_ulong,
    ) as size_t;
    if (*encoder).ctx.max_dtable_capacity == max_dtable_capacity {
        return;
    }
    (*encoder).flags = ((*encoder).flags as ::core::ffi::c_uint
        | NGHTTP3_QPACK_ENCODER_FLAG_PENDING_SET_DTABLE_CAP) as uint8_t;
    if (*encoder).min_dtable_update > max_dtable_capacity {
        (*encoder).min_dtable_update = max_dtable_capacity;
        (*encoder).ctx.max_dtable_capacity = max_dtable_capacity;
    }
    (*encoder).last_max_dtable_update = max_dtable_capacity;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_set_max_blocked_streams(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut max_blocked_streams: size_t,
) {
    (*encoder).ctx.max_blocked_streams = max_blocked_streams;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_set_indexing_strat(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut strat: nghttp3_qpack_indexing_strat,
) {
    (*encoder).indexing_strat = strat;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_get_min_cnt(
    mut encoder: *const nghttp3_qpack_encoder,
) -> uint64_t {
    '_c2rust_label: {
        if nghttp3_pq_empty(&raw const (*encoder).min_cnts) == 0 {} else {
            __assert_fail(
                b"!nghttp3_pq_empty(&encoder->min_cnts)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                971 as ::core::ffi::c_uint,
                b"uint64_t nghttp3_qpack_encoder_get_min_cnt(const nghttp3_qpack_encoder *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    return (*((nghttp3_pq_top(&raw const (*encoder).min_cnts)
        as *mut ::core::ffi::c_char)
        .offset(-(8 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_qpack_header_block_ref))
        .min_cnt;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_shrink_dtable(
    mut encoder: *mut nghttp3_qpack_encoder,
) {
    let mut dtable: *mut nghttp3_ringbuf = &raw mut (*encoder).ctx.dtable;
    let mut mem: *const nghttp3_mem = (*encoder).ctx.mem;
    let mut min_cnt: uint64_t = UINT64_MAX as uint64_t;
    let mut len: size_t = 0;
    let mut ent: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    if (*encoder).ctx.dtable_size <= (*encoder).ctx.max_dtable_capacity {
        return;
    }
    if nghttp3_pq_empty(&raw mut (*encoder).min_cnts) == 0 {
        min_cnt = nghttp3_qpack_encoder_get_min_cnt(encoder);
    }
    while (*encoder).ctx.dtable_size > (*encoder).ctx.max_dtable_capacity {
        len = nghttp3_ringbuf_len(dtable);
        ent = *(nghttp3_ringbuf_get(dtable, len.wrapping_sub(1 as size_t))
            as *mut *mut nghttp3_qpack_entry);
        if (*ent).absidx.wrapping_add(1 as uint64_t) == min_cnt {
            return;
        }
        (*encoder).ctx.dtable_size = (*encoder)
            .ctx
            .dtable_size
            .wrapping_sub(table_space((*(*ent).nv.name).len, (*(*ent).nv.value).len));
        nghttp3_ringbuf_pop_back(dtable);
        qpack_map_remove(&raw mut (*encoder).dtable_map, ent);
        nghttp3_qpack_entry_free(ent);
        nghttp3_mem_free(mem, ent as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn qpack_encoder_add_stream_ref(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut stream_id: int64_t,
    mut stream: *mut nghttp3_qpack_stream,
    mut max_cnt: uint64_t,
    mut min_cnt: uint64_t,
) -> ::core::ffi::c_int {
    let mut r#ref: *mut nghttp3_qpack_header_block_ref = ::core::ptr::null_mut::<
        nghttp3_qpack_header_block_ref,
    >();
    let mut mem: *const nghttp3_mem = (*encoder).ctx.mem;
    let mut prev_max_cnt: uint64_t = 0 as uint64_t;
    let mut rv: ::core::ffi::c_int = 0;
    if stream.is_null() {
        rv = nghttp3_qpack_stream_new(&raw mut stream, stream_id, mem);
        if rv != 0 as ::core::ffi::c_int {
            '_c2rust_label: {
                if rv == -901 as ::core::ffi::c_int {} else {
                    __assert_fail(
                        b"rv == NGHTTP3_ERR_NOMEM\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1036 as ::core::ffi::c_uint,
                        b"int qpack_encoder_add_stream_ref(nghttp3_qpack_encoder *, int64_t, nghttp3_qpack_stream *, uint64_t, uint64_t)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            return rv;
        }
        rv = nghttp3_map_insert(
            &raw mut (*encoder).streams,
            (*stream).stream_id as nghttp3_map_key_type,
            stream as *mut ::core::ffi::c_void,
        );
        if rv != 0 as ::core::ffi::c_int {
            '_c2rust_label_0: {
                if rv == -901 as ::core::ffi::c_int {} else {
                    __assert_fail(
                        b"rv == NGHTTP3_ERR_NOMEM\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1042 as ::core::ffi::c_uint,
                        b"int qpack_encoder_add_stream_ref(nghttp3_qpack_encoder *, int64_t, nghttp3_qpack_stream *, uint64_t, uint64_t)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            nghttp3_qpack_stream_del(stream, mem);
            return rv;
        }
    } else {
        prev_max_cnt = nghttp3_qpack_stream_get_max_cnt(stream);
        if nghttp3_qpack_encoder_stream_is_blocked(encoder, stream) != 0
            && max_cnt > prev_max_cnt
        {
            nghttp3_qpack_encoder_unblock_stream(encoder, stream);
        }
    }
    rv = nghttp3_qpack_header_block_ref_new(&raw mut r#ref, max_cnt, min_cnt, mem);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    rv = nghttp3_qpack_stream_add_ref(stream, r#ref);
    if rv != 0 as ::core::ffi::c_int {
        nghttp3_qpack_header_block_ref_del(r#ref, mem);
        return rv;
    }
    if max_cnt > prev_max_cnt
        && nghttp3_qpack_encoder_stream_is_blocked(encoder, stream) != 0
    {
        rv = nghttp3_qpack_encoder_block_stream(encoder, stream);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
    }
    return nghttp3_pq_push(&raw mut (*encoder).min_cnts, &raw mut (*r#ref).min_cnts_pe);
}
unsafe extern "C" fn qpack_encoder_remove_stream(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut stream: *mut nghttp3_qpack_stream,
) {
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    let mut r#ref: *mut nghttp3_qpack_header_block_ref = ::core::ptr::null_mut::<
        nghttp3_qpack_header_block_ref,
    >();
    nghttp3_map_remove(
        &raw mut (*encoder).streams,
        (*stream).stream_id as nghttp3_map_key_type,
    );
    len = nghttp3_ringbuf_len(&raw mut (*stream).refs);
    i = 0 as size_t;
    while i < len {
        r#ref = *(nghttp3_ringbuf_get(&raw mut (*stream).refs, i)
            as *mut *mut nghttp3_qpack_header_block_ref);
        '_c2rust_label: {
            if (*r#ref).min_cnts_pe.index != 18446744073709551615 as size_t {} else {
                __assert_fail(
                    b"ref->min_cnts_pe.index != NGHTTP3_PQ_BAD_INDEX\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1089 as ::core::ffi::c_uint,
                    b"void qpack_encoder_remove_stream(nghttp3_qpack_encoder *, nghttp3_qpack_stream *)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        nghttp3_pq_remove(&raw mut (*encoder).min_cnts, &raw mut (*r#ref).min_cnts_pe);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn reserve_buf(
    mut buf: *mut nghttp3_buf,
    mut extra_size: size_t,
    mut mem: *const nghttp3_mem,
) -> ::core::ffi::c_int {
    let mut left: size_t = nghttp3_buf_left(buf);
    let mut n: size_t = 32 as size_t;
    if left >= extra_size {
        return 0 as ::core::ffi::c_int;
    }
    n = nghttp3_max_unsigned_long_int(
        n as ::core::ffi::c_ulong,
        (nghttp3_buf_cap(buf) as ::core::ffi::c_ulong)
            .wrapping_add(extra_size as ::core::ffi::c_ulong)
            .wrapping_sub(left as ::core::ffi::c_ulong),
    ) as size_t;
    if n > ((1 as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int) as size_t {
        return NGHTTP3_ERR_NOMEM;
    }
    n = ((1 as ::core::ffi::c_uint)
        << 32 as ::core::ffi::c_int
            - (n as uint32_t).wrapping_sub(1 as uint32_t).leading_zeros() as i32)
        as size_t;
    return nghttp3_buf_reserve(buf, n, mem);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_encode(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut pbuf: *mut nghttp3_buf,
    mut rbuf: *mut nghttp3_buf,
    mut ebuf: *mut nghttp3_buf,
    mut stream_id: int64_t,
    mut nva: *const nghttp3_nv,
    mut nvlen: size_t,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    let mut max_cnt: uint64_t = 0 as uint64_t;
    let mut min_cnt: uint64_t = UINT64_MAX as uint64_t;
    let mut base: uint64_t = 0;
    let mut rv: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut allow_blocking: ::core::ffi::c_int = 0;
    let mut blocked_stream: ::core::ffi::c_int = 0;
    let mut stream: *mut nghttp3_qpack_stream = ::core::ptr::null_mut::<
        nghttp3_qpack_stream,
    >();
    '_c2rust_label: {
        if stream_id >= 0 as int64_t {} else {
            __assert_fail(
                b"stream_id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1151 as ::core::ffi::c_uint,
                b"int nghttp3_qpack_encoder_encode(nghttp3_qpack_encoder *, nghttp3_buf *, nghttp3_buf *, nghttp3_buf *, int64_t, const nghttp3_nv *, size_t)\0"
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
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1152 as ::core::ffi::c_uint,
                b"int nghttp3_qpack_encoder_encode(nghttp3_qpack_encoder *, nghttp3_buf *, nghttp3_buf *, nghttp3_buf *, int64_t, const nghttp3_nv *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if (*encoder).ctx.bad != 0 {
        return NGHTTP3_ERR_QPACK_FATAL;
    }
    rv = nghttp3_qpack_encoder_process_dtable_update(encoder, ebuf);
    '_fail: {
        if rv == 0 as ::core::ffi::c_int {
            base = (*encoder).ctx.next_absidx;
            stream = nghttp3_qpack_encoder_find_stream(encoder, stream_id);
            blocked_stream = (!stream.is_null()
                && nghttp3_qpack_encoder_stream_is_blocked(encoder, stream) != 0)
                as ::core::ffi::c_int;
            allow_blocking = (blocked_stream != 0
                || (*encoder).ctx.max_blocked_streams
                    > nghttp3_ksl_len(&raw mut (*encoder).blocked_streams))
                as ::core::ffi::c_int;
            i = 0 as size_t;
            while i < nvlen {
                rv = nghttp3_qpack_encoder_encode_nv(
                    encoder,
                    &raw mut max_cnt,
                    &raw mut min_cnt,
                    rbuf,
                    ebuf,
                    nva.offset(i as isize),
                    base,
                    allow_blocking,
                );
                if rv != 0 as ::core::ffi::c_int {
                    break '_fail;
                }
                i = i.wrapping_add(1);
            }
            nghttp3_qpack_encoder_write_field_section_prefix(
                encoder,
                pbuf,
                max_cnt,
                base,
            );
            (*encoder).uninterrupted_decoderlen = 0 as size_t;
            if max_cnt == 0 {
                return 0 as ::core::ffi::c_int;
            }
            rv = qpack_encoder_add_stream_ref(
                encoder,
                stream_id,
                stream,
                max_cnt,
                min_cnt,
            );
            if rv == 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    (*encoder).ctx.bad = 1 as uint8_t;
    return rv;
}
unsafe extern "C" fn qpack_write_number(
    mut rbuf: *mut nghttp3_buf,
    mut fb: uint8_t,
    mut num: uint64_t,
    mut prefix: size_t,
    mut mem: *const nghttp3_mem,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    let mut len: size_t = nghttp3_qpack_put_varint_len(num, prefix);
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    rv = reserve_buf(rbuf, len, mem);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    p = (*rbuf).last;
    *p = fb;
    p = nghttp3_qpack_put_varint(p, num, prefix);
    '_c2rust_label: {
        if p.offset_from((*rbuf).last) as size_t == len {} else {
            __assert_fail(
                b"(size_t)(p - rbuf->last) == len\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1233 as ::core::ffi::c_uint,
                b"int qpack_write_number(nghttp3_buf *, uint8_t, uint64_t, size_t, const nghttp3_mem *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    (*rbuf).last = p;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_process_dtable_update(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut ebuf: *mut nghttp3_buf,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    nghttp3_qpack_encoder_shrink_dtable(encoder);
    if (*encoder).ctx.max_dtable_capacity < (*encoder).ctx.dtable_size
        || (*encoder).flags as ::core::ffi::c_uint
            & NGHTTP3_QPACK_ENCODER_FLAG_PENDING_SET_DTABLE_CAP == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*encoder).min_dtable_update < (*encoder).last_max_dtable_update {
        rv = nghttp3_qpack_encoder_write_set_dtable_cap(
            encoder,
            ebuf,
            (*encoder).min_dtable_update,
        );
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
    }
    rv = nghttp3_qpack_encoder_write_set_dtable_cap(
        encoder,
        ebuf,
        (*encoder).last_max_dtable_update,
    );
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    (*encoder).flags = ((*encoder).flags as ::core::ffi::c_int
        & !NGHTTP3_QPACK_ENCODER_FLAG_PENDING_SET_DTABLE_CAP as uint8_t
            as ::core::ffi::c_int) as uint8_t;
    (*encoder).min_dtable_update = SIZE_MAX as size_t;
    (*encoder).ctx.max_dtable_capacity = (*encoder).last_max_dtable_update;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_write_set_dtable_cap(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut ebuf: *mut nghttp3_buf,
    mut cap: size_t,
) -> ::core::ffi::c_int {
    return qpack_write_number(
        ebuf,
        0x20 as uint8_t,
        cap as uint64_t,
        5 as size_t,
        (*encoder).ctx.mem,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_find_stream(
    mut encoder: *const nghttp3_qpack_encoder,
    mut stream_id: int64_t,
) -> *mut nghttp3_qpack_stream {
    return nghttp3_map_find(
        &raw const (*encoder).streams,
        stream_id as nghttp3_map_key_type,
    ) as *mut nghttp3_qpack_stream;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_stream_is_blocked(
    mut encoder: *const nghttp3_qpack_encoder,
    mut stream: *const nghttp3_qpack_stream,
) -> ::core::ffi::c_int {
    return (!stream.is_null()
        && (*encoder).krcnt < nghttp3_qpack_stream_get_max_cnt(stream))
        as ::core::ffi::c_int;
}
unsafe extern "C" fn qpack_hash_name(mut nv: *const nghttp3_nv) -> uint32_t {
    let mut h: uint32_t = 2166136261 as uint32_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < (*nv).namelen {
        h ^= *(*nv).name.offset(i as isize) as uint32_t;
        h = h
            .wrapping_add(
                (h << 1 as ::core::ffi::c_int)
                    .wrapping_add(h << 4 as ::core::ffi::c_int)
                    .wrapping_add(h << 7 as ::core::ffi::c_int)
                    .wrapping_add(h << 8 as ::core::ffi::c_int)
                    .wrapping_add(h << 24 as ::core::ffi::c_int),
            );
        i = i.wrapping_add(1);
    }
    return h;
}
unsafe extern "C" fn qpack_encoder_decide_indexing_mode(
    mut encoder: *const nghttp3_qpack_encoder,
    mut nv: *const nghttp3_nv,
    mut token: int32_t,
) -> nghttp3_qpack_indexing_mode {
    if (*nv).flags as ::core::ffi::c_uint & NGHTTP3_NV_FLAG_NEVER_INDEX != 0 {
        return nghttp3_qpack_indexing_mode::NGHTTP3_QPACK_INDEXING_MODE_NEVER;
    }
    match token {
        45 => return nghttp3_qpack_indexing_mode::NGHTTP3_QPACK_INDEXING_MODE_NEVER,
        68 => {
            if (*nv).valuelen < 20 as size_t {
                return nghttp3_qpack_indexing_mode::NGHTTP3_QPACK_INDEXING_MODE_NEVER;
            }
        }
        -1 => {
            match (*encoder).indexing_strat {
                nghttp3_qpack_indexing_strat::NGHTTP3_QPACK_INDEXING_STRAT_EAGER => {}
                nghttp3_qpack_indexing_strat::NGHTTP3_QPACK_INDEXING_STRAT_NONE => {
                    if (*nv).flags as ::core::ffi::c_uint & NGHTTP3_NV_FLAG_TRY_INDEX
                        == 0
                    {
                        return nghttp3_qpack_indexing_mode::NGHTTP3_QPACK_INDEXING_MODE_LITERAL;
                    }
                }
                _ => {
                    nghttp3_unreachable_fail(
                        b"nghttp3_qpack.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1333 as ::core::ffi::c_int,
                        b"qpack_encoder_decide_indexing_mode\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            }
        }
        8 | 43 | 55 | 71 | 74 | 75 | 79 | 85 => {
            if (*nv).flags as ::core::ffi::c_uint & NGHTTP3_NV_FLAG_TRY_INDEX == 0 {
                return nghttp3_qpack_indexing_mode::NGHTTP3_QPACK_INDEXING_MODE_LITERAL;
            }
        }
        1000 | 1006 | 1007 | 1008 => {}
        _ => {
            if (*nv).flags as ::core::ffi::c_uint & NGHTTP3_NV_FLAG_TRY_INDEX == 0 {
                if token >= 1000 as int32_t {
                    return nghttp3_qpack_indexing_mode::NGHTTP3_QPACK_INDEXING_MODE_LITERAL;
                }
            }
        }
    }
    if table_space((*nv).namelen, (*nv).valuelen)
        > (*encoder)
            .ctx
            .max_dtable_capacity
            .wrapping_mul(3 as size_t)
            .wrapping_div(4 as size_t)
    {
        return nghttp3_qpack_indexing_mode::NGHTTP3_QPACK_INDEXING_MODE_LITERAL;
    }
    return nghttp3_qpack_indexing_mode::NGHTTP3_QPACK_INDEXING_MODE_STORE;
}
unsafe extern "C" fn qpack_encoder_can_index(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut need: size_t,
    mut min_cnt: uint64_t,
) -> ::core::ffi::c_int {
    let mut avail: size_t = 0 as size_t;
    let mut len: size_t = 0;
    let mut gmin_cnt: uint64_t = 0;
    let mut min_ent: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    let mut last_ent: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    let mut dtable: *mut nghttp3_ringbuf = &raw mut (*encoder).ctx.dtable;
    if (*encoder).ctx.max_dtable_capacity > (*encoder).ctx.dtable_size {
        avail = (*encoder)
            .ctx
            .max_dtable_capacity
            .wrapping_sub((*encoder).ctx.dtable_size);
        if need <= avail {
            return 1 as ::core::ffi::c_int;
        }
    }
    if nghttp3_pq_empty(&raw mut (*encoder).min_cnts) == 0 {
        gmin_cnt = nghttp3_qpack_encoder_get_min_cnt(encoder);
        min_cnt = nghttp3_min_unsigned_long_int(
            min_cnt as ::core::ffi::c_ulong,
            gmin_cnt as ::core::ffi::c_ulong,
        ) as uint64_t;
    }
    if min_cnt == UINT64_MAX as uint64_t {
        return ((*encoder).ctx.max_dtable_capacity >= need) as ::core::ffi::c_int;
    }
    min_ent = nghttp3_qpack_context_dtable_get(
        &raw mut (*encoder).ctx,
        min_cnt.wrapping_sub(1 as uint64_t),
    );
    len = nghttp3_ringbuf_len(&raw mut (*encoder).ctx.dtable);
    '_c2rust_label: {
        if len != 0 {} else {
            __assert_fail(
                b"len\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1405 as ::core::ffi::c_uint,
                b"int qpack_encoder_can_index(nghttp3_qpack_encoder *, size_t, uint64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    last_ent = *(nghttp3_ringbuf_get(dtable, len.wrapping_sub(1 as size_t))
        as *mut *mut nghttp3_qpack_entry);
    if min_ent == last_ent {
        return 0 as ::core::ffi::c_int;
    }
    return (avail.wrapping_add((*min_ent).sum).wrapping_sub((*last_ent).sum) >= need)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn qpack_encoder_can_index_nv(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut nv: *const nghttp3_nv,
    mut min_cnt: uint64_t,
) -> ::core::ffi::c_int {
    return qpack_encoder_can_index(
        encoder,
        table_space((*nv).namelen, (*nv).valuelen),
        min_cnt,
    );
}
unsafe extern "C" fn qpack_encoder_can_index_duplicate(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut absidx: uint64_t,
    mut min_cnt: uint64_t,
) -> ::core::ffi::c_int {
    let mut ent: *mut nghttp3_qpack_entry = nghttp3_qpack_context_dtable_get(
        &raw mut (*encoder).ctx,
        absidx,
    );
    return qpack_encoder_can_index(
        encoder,
        table_space((*(*ent).nv.name).len, (*(*ent).nv.value).len),
        min_cnt,
    );
}
unsafe extern "C" fn qpack_context_check_draining(
    mut ctx: *mut nghttp3_qpack_context,
    mut absidx: uint64_t,
) -> ::core::ffi::c_int {
    let safe: size_t = (*ctx)
        .max_dtable_capacity
        .wrapping_sub(
            nghttp3_min_unsigned_long_int(
                512 as ::core::ffi::c_ulong,
                ((*ctx).max_dtable_capacity as ::core::ffi::c_ulong)
                    .wrapping_mul(1 as ::core::ffi::c_ulong)
                    .wrapping_div(8 as ::core::ffi::c_ulong),
            ) as size_t,
        );
    let mut ent: *mut nghttp3_qpack_entry = nghttp3_qpack_context_dtable_get(
        ctx,
        absidx,
    );
    return ((*ctx).dtable_sum.wrapping_sub((*ent).sum) > safe) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_encode_nv(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut pmax_cnt: *mut uint64_t,
    mut pmin_cnt: *mut uint64_t,
    mut rbuf: *mut nghttp3_buf,
    mut ebuf: *mut nghttp3_buf,
    mut nv: *const nghttp3_nv,
    mut base: uint64_t,
    mut allow_blocking: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut hash: uint32_t = 0 as uint32_t;
    let mut token: int32_t = 0;
    let mut indexing_mode: nghttp3_qpack_indexing_mode = nghttp3_qpack_indexing_mode::NGHTTP3_QPACK_INDEXING_MODE_LITERAL;
    let mut sres: nghttp3_qpack_lookup_result = nghttp3_qpack_lookup_result {
        index: -1 as nghttp3_ssize,
        name_value_match: 0,
        pb_index: -1 as nghttp3_ssize,
    };
    let mut dres: nghttp3_qpack_lookup_result = nghttp3_qpack_lookup_result {
        index: -1 as nghttp3_ssize,
        name_value_match: 0,
        pb_index: -1 as nghttp3_ssize,
    };
    let mut new_ent: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    let mut static_entry: ::core::ffi::c_int = 0;
    let mut just_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rv: ::core::ffi::c_int = 0;
    token = qpack_lookup_token((*nv).name, (*nv).namelen);
    static_entry = (token != -1 as int32_t
        && (token as size_t)
            < ::core::mem::size_of::<[nghttp3_qpack_static_entry; 99]>()
                .wrapping_div(::core::mem::size_of::<nghttp3_qpack_static_entry>()))
        as ::core::ffi::c_int;
    indexing_mode = qpack_encoder_decide_indexing_mode(encoder, nv, token);
    if static_entry != 0 {
        sres = nghttp3_qpack_lookup_stable(nv, token, indexing_mode);
        if sres.index != -1 as nghttp3_ssize && sres.name_value_match != 0 {
            return nghttp3_qpack_encoder_write_static_indexed(
                encoder,
                rbuf,
                sres.index as uint64_t,
            );
        }
    }
    if static_entry != 0 {
        hash = token_stable[token as usize].hash;
    } else {
        match token {
            1000 => {
                hash = 2952701295 as ::core::ffi::c_uint as uint32_t;
            }
            1006 => {
                hash = 1011170994 as ::core::ffi::c_uint as uint32_t;
            }
            1007 => {
                hash = 1128642621 as ::core::ffi::c_uint as uint32_t;
            }
            1008 => {
                hash = 2498028297 as ::core::ffi::c_uint as uint32_t;
            }
            _ => {
                hash = qpack_hash_name(nv);
            }
        }
    }
    if nghttp3_map_size(&raw mut (*encoder).streams)
        < NGHTTP3_QPACK_MAX_QPACK_STREAMS as size_t
    {
        dres = nghttp3_qpack_encoder_lookup_dtable(
            encoder,
            nv,
            token,
            hash,
            indexing_mode,
            (*encoder).krcnt,
            allow_blocking,
        );
        just_index = (indexing_mode.0
            == nghttp3_qpack_indexing_mode::NGHTTP3_QPACK_INDEXING_MODE_STORE.0
            && dres.pb_index == -1 as nghttp3_ssize) as ::core::ffi::c_int;
    }
    if dres.index != -1 as nghttp3_ssize && dres.name_value_match != 0 {
        if allow_blocking != 0
            && qpack_context_check_draining(
                &raw mut (*encoder).ctx,
                dres.index as uint64_t,
            ) != 0
            && qpack_encoder_can_index_duplicate(
                encoder,
                dres.index as uint64_t,
                *pmin_cnt,
            ) != 0
        {
            rv = nghttp3_qpack_encoder_write_duplicate_insert(
                encoder,
                ebuf,
                dres.index as uint64_t,
            );
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
            rv = nghttp3_qpack_encoder_dtable_duplicate_add(
                encoder,
                dres.index as uint64_t,
            );
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
            new_ent = nghttp3_qpack_context_dtable_top(&raw mut (*encoder).ctx);
            dres.index = (*new_ent).absidx as nghttp3_ssize;
        }
        *pmax_cnt = nghttp3_max_unsigned_long_int(
            *pmax_cnt as ::core::ffi::c_ulong,
            (dres.index + 1 as nghttp3_ssize) as ::core::ffi::c_ulong,
        ) as uint64_t;
        *pmin_cnt = nghttp3_min_unsigned_long_int(
            *pmin_cnt as ::core::ffi::c_ulong,
            (dres.index + 1 as nghttp3_ssize) as ::core::ffi::c_ulong,
        ) as uint64_t;
        return nghttp3_qpack_encoder_write_dynamic_indexed(
            encoder,
            rbuf,
            dres.index as uint64_t,
            base,
        );
    }
    if sres.index != -1 as nghttp3_ssize {
        if just_index != 0 && qpack_encoder_can_index_nv(encoder, nv, *pmin_cnt) != 0 {
            rv = nghttp3_qpack_encoder_write_static_insert(
                encoder,
                ebuf,
                sres.index as uint64_t,
                nv,
            );
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
            rv = nghttp3_qpack_encoder_dtable_static_add(
                encoder,
                sres.index as uint64_t,
                nv,
                hash,
            );
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
            if allow_blocking != 0 {
                new_ent = nghttp3_qpack_context_dtable_top(&raw mut (*encoder).ctx);
                *pmax_cnt = nghttp3_max_unsigned_long_int(
                    *pmax_cnt as ::core::ffi::c_ulong,
                    ((*new_ent).absidx as ::core::ffi::c_ulong)
                        .wrapping_add(1 as ::core::ffi::c_ulong),
                ) as uint64_t;
                *pmin_cnt = nghttp3_min_unsigned_long_int(
                    *pmin_cnt as ::core::ffi::c_ulong,
                    ((*new_ent).absidx as ::core::ffi::c_ulong)
                        .wrapping_add(1 as ::core::ffi::c_ulong),
                ) as uint64_t;
                return nghttp3_qpack_encoder_write_dynamic_indexed(
                    encoder,
                    rbuf,
                    (*new_ent).absidx,
                    base,
                );
            }
        }
        return nghttp3_qpack_encoder_write_static_indexed_name(
            encoder,
            rbuf,
            sres.index as uint64_t,
            nv,
        );
    }
    if dres.index != -1 as nghttp3_ssize {
        if just_index != 0
            && qpack_encoder_can_index_nv(
                encoder,
                nv,
                if allow_blocking != 0 {
                    *pmin_cnt
                } else {
                    nghttp3_min_unsigned_long_int(
                        (dres.index as ::core::ffi::c_ulong)
                            .wrapping_add(1 as ::core::ffi::c_ulong),
                        *pmin_cnt as ::core::ffi::c_ulong,
                    ) as uint64_t
                },
            ) != 0
        {
            rv = nghttp3_qpack_encoder_write_dynamic_insert(
                encoder,
                ebuf,
                dres.index as uint64_t,
                nv,
            );
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
            if allow_blocking == 0 {
                *pmin_cnt = nghttp3_min_unsigned_long_int(
                    *pmin_cnt as ::core::ffi::c_ulong,
                    (dres.index as ::core::ffi::c_ulong)
                        .wrapping_add(1 as ::core::ffi::c_ulong),
                ) as uint64_t;
            }
            rv = nghttp3_qpack_encoder_dtable_dynamic_add(
                encoder,
                dres.index as uint64_t,
                nv,
                hash,
            );
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
            if allow_blocking != 0 {
                new_ent = nghttp3_qpack_context_dtable_top(&raw mut (*encoder).ctx);
                *pmax_cnt = nghttp3_max_unsigned_long_int(
                    *pmax_cnt as ::core::ffi::c_ulong,
                    ((*new_ent).absidx as ::core::ffi::c_ulong)
                        .wrapping_add(1 as ::core::ffi::c_ulong),
                ) as uint64_t;
                *pmin_cnt = nghttp3_min_unsigned_long_int(
                    *pmin_cnt as ::core::ffi::c_ulong,
                    ((*new_ent).absidx as ::core::ffi::c_ulong)
                        .wrapping_add(1 as ::core::ffi::c_ulong),
                ) as uint64_t;
                return nghttp3_qpack_encoder_write_dynamic_indexed(
                    encoder,
                    rbuf,
                    (*new_ent).absidx,
                    base,
                );
            }
        }
        *pmax_cnt = nghttp3_max_unsigned_long_int(
            *pmax_cnt as ::core::ffi::c_ulong,
            (dres.index + 1 as nghttp3_ssize) as ::core::ffi::c_ulong,
        ) as uint64_t;
        *pmin_cnt = nghttp3_min_unsigned_long_int(
            *pmin_cnt as ::core::ffi::c_ulong,
            (dres.index + 1 as nghttp3_ssize) as ::core::ffi::c_ulong,
        ) as uint64_t;
        return nghttp3_qpack_encoder_write_dynamic_indexed_name(
            encoder,
            rbuf,
            dres.index as uint64_t,
            base,
            nv,
        );
    }
    if just_index != 0 && qpack_encoder_can_index_nv(encoder, nv, *pmin_cnt) != 0 {
        rv = nghttp3_qpack_encoder_dtable_literal_add(encoder, nv, token, hash);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
        rv = nghttp3_qpack_encoder_write_literal_insert(encoder, ebuf, nv);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
        if allow_blocking != 0 {
            new_ent = nghttp3_qpack_context_dtable_top(&raw mut (*encoder).ctx);
            *pmax_cnt = nghttp3_max_unsigned_long_int(
                *pmax_cnt as ::core::ffi::c_ulong,
                ((*new_ent).absidx as ::core::ffi::c_ulong)
                    .wrapping_add(1 as ::core::ffi::c_ulong),
            ) as uint64_t;
            *pmin_cnt = nghttp3_min_unsigned_long_int(
                *pmin_cnt as ::core::ffi::c_ulong,
                ((*new_ent).absidx as ::core::ffi::c_ulong)
                    .wrapping_add(1 as ::core::ffi::c_ulong),
            ) as uint64_t;
            return nghttp3_qpack_encoder_write_dynamic_indexed(
                encoder,
                rbuf,
                (*new_ent).absidx,
                base,
            );
        }
    }
    return nghttp3_qpack_encoder_write_literal(encoder, rbuf, nv);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_lookup_stable(
    mut nv: *const nghttp3_nv,
    mut token: int32_t,
    mut indexing_mode: nghttp3_qpack_indexing_mode,
) -> nghttp3_qpack_lookup_result {
    let mut res: nghttp3_qpack_lookup_result = nghttp3_qpack_lookup_result {
        index: token_stable[token as usize].absidx as nghttp3_ssize,
        name_value_match: 0,
        pb_index: -1 as nghttp3_ssize,
    };
    let mut ent: *mut nghttp3_qpack_static_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_static_entry,
    >();
    let mut hdr: *mut nghttp3_qpack_static_header = ::core::ptr::null_mut::<
        nghttp3_qpack_static_header,
    >();
    let mut i: size_t = 0;
    '_c2rust_label: {
        if token >= 0 as int32_t {} else {
            __assert_fail(
                b"token >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1641 as ::core::ffi::c_uint,
                b"nghttp3_qpack_lookup_result nghttp3_qpack_lookup_stable(const nghttp3_nv *, int32_t, nghttp3_qpack_indexing_mode)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if indexing_mode.0
        == nghttp3_qpack_indexing_mode::NGHTTP3_QPACK_INDEXING_MODE_NEVER.0
    {
        return res;
    }
    i = token as size_t;
    while i
        < ::core::mem::size_of::<[nghttp3_qpack_static_entry; 99]>()
            .wrapping_div(::core::mem::size_of::<nghttp3_qpack_static_entry>())
        && token_stable[i].token == token
    {
        ent = (&raw mut token_stable as *mut nghttp3_qpack_static_entry)
            .offset(i as isize);
        hdr = (&raw mut stable as *mut nghttp3_qpack_static_header)
            .offset((*ent).absidx as isize);
        if (*hdr).value.len == (*nv).valuelen
            && memeq(
                (*hdr).value.base as *const ::core::ffi::c_void,
                (*nv).value as *const ::core::ffi::c_void,
                (*nv).valuelen,
            ) != 0
        {
            res.index = (*ent).absidx as nghttp3_ssize;
            res.name_value_match = 1 as ::core::ffi::c_int;
            return res;
        }
        i = i.wrapping_add(1);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_lookup_dtable(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut nv: *const nghttp3_nv,
    mut token: int32_t,
    mut hash: uint32_t,
    mut indexing_mode: nghttp3_qpack_indexing_mode,
    mut krcnt: uint64_t,
    mut allow_blocking: ::core::ffi::c_int,
) -> nghttp3_qpack_lookup_result {
    let mut res: nghttp3_qpack_lookup_result = nghttp3_qpack_lookup_result {
        index: -1 as nghttp3_ssize,
        name_value_match: 0,
        pb_index: -1 as nghttp3_ssize,
    };
    let mut exact_match: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut r#match: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    let mut pb_match: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    encoder_qpack_map_find(
        encoder,
        &raw mut exact_match,
        &raw mut r#match,
        &raw mut pb_match,
        nv,
        token,
        hash,
        krcnt,
        allow_blocking,
        (indexing_mode.0
            == nghttp3_qpack_indexing_mode::NGHTTP3_QPACK_INDEXING_MODE_NEVER.0)
            as ::core::ffi::c_int,
    );
    if !r#match.is_null() {
        res.index = (*r#match).absidx as nghttp3_ssize;
        res.name_value_match = exact_match;
    }
    if !pb_match.is_null() {
        res.pb_index = (*pb_match).absidx as nghttp3_ssize;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_header_block_ref_new(
    mut pref: *mut *mut nghttp3_qpack_header_block_ref,
    mut max_cnt: uint64_t,
    mut min_cnt: uint64_t,
    mut mem: *const nghttp3_mem,
) -> ::core::ffi::c_int {
    let mut r#ref: *mut nghttp3_qpack_header_block_ref = nghttp3_mem_malloc(
        mem,
        ::core::mem::size_of::<nghttp3_qpack_header_block_ref>(),
    ) as *mut nghttp3_qpack_header_block_ref;
    if r#ref.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    (*r#ref).max_cnts_pe.index = NGHTTP3_PQ_BAD_INDEX as size_t;
    (*r#ref).min_cnts_pe.index = NGHTTP3_PQ_BAD_INDEX as size_t;
    (*r#ref).max_cnt = max_cnt;
    (*r#ref).min_cnt = min_cnt;
    *pref = r#ref;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_header_block_ref_del(
    mut r#ref: *mut nghttp3_qpack_header_block_ref,
    mut mem: *const nghttp3_mem,
) {
    nghttp3_mem_free(mem, r#ref as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn ref_max_cnt_greater(
    mut lhsx: *const nghttp3_pq_entry,
    mut rhsx: *const nghttp3_pq_entry,
) -> ::core::ffi::c_int {
    let mut lhs: *const nghttp3_qpack_header_block_ref = (lhsx
        as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_qpack_header_block_ref;
    let mut rhs: *const nghttp3_qpack_header_block_ref = (rhsx
        as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_qpack_header_block_ref;
    return ((*lhs).max_cnt > (*rhs).max_cnt) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_stream_new(
    mut pstream: *mut *mut nghttp3_qpack_stream,
    mut stream_id: int64_t,
    mut mem: *const nghttp3_mem,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_qpack_stream = ::core::ptr::null_mut::<
        nghttp3_qpack_stream,
    >();
    stream = nghttp3_mem_malloc(mem, ::core::mem::size_of::<nghttp3_qpack_stream>())
        as *mut nghttp3_qpack_stream;
    if stream.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    nghttp3_ringbuf_init(
        &raw mut (*stream).refs,
        0 as size_t,
        ::core::mem::size_of::<*mut nghttp3_qpack_header_block_ref>(),
        mem,
    );
    nghttp3_pq_init(
        &raw mut (*stream).max_cnts,
        Some(
            ref_max_cnt_greater
                as unsafe extern "C" fn(
                    *const nghttp3_pq_entry,
                    *const nghttp3_pq_entry,
                ) -> ::core::ffi::c_int,
        ),
        mem,
    );
    (*stream).stream_id = stream_id;
    *pstream = stream;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_stream_del(
    mut stream: *mut nghttp3_qpack_stream,
    mut mem: *const nghttp3_mem,
) {
    let mut r#ref: *mut nghttp3_qpack_header_block_ref = ::core::ptr::null_mut::<
        nghttp3_qpack_header_block_ref,
    >();
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    if stream.is_null() {
        return;
    }
    nghttp3_pq_free(&raw mut (*stream).max_cnts);
    len = nghttp3_ringbuf_len(&raw mut (*stream).refs);
    i = 0 as size_t;
    while i < len {
        r#ref = *(nghttp3_ringbuf_get(&raw mut (*stream).refs, i)
            as *mut *mut nghttp3_qpack_header_block_ref);
        nghttp3_qpack_header_block_ref_del(r#ref, mem);
        i = i.wrapping_add(1);
    }
    nghttp3_ringbuf_free(&raw mut (*stream).refs);
    nghttp3_mem_free(mem, stream as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_stream_get_max_cnt(
    mut stream: *const nghttp3_qpack_stream,
) -> uint64_t {
    let mut r#ref: *mut nghttp3_qpack_header_block_ref = ::core::ptr::null_mut::<
        nghttp3_qpack_header_block_ref,
    >();
    if nghttp3_pq_empty(&raw const (*stream).max_cnts) != 0 {
        return 0 as uint64_t;
    }
    r#ref = (nghttp3_pq_top(&raw const (*stream).max_cnts) as *mut ::core::ffi::c_char)
        .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
        as *mut nghttp3_qpack_header_block_ref;
    return (*r#ref).max_cnt;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_stream_add_ref(
    mut stream: *mut nghttp3_qpack_stream,
    mut r#ref: *mut nghttp3_qpack_header_block_ref,
) -> ::core::ffi::c_int {
    let mut dest: *mut *mut nghttp3_qpack_header_block_ref = ::core::ptr::null_mut::<
        *mut nghttp3_qpack_header_block_ref,
    >();
    let mut rv: ::core::ffi::c_int = 0;
    if nghttp3_ringbuf_full(&raw mut (*stream).refs) != 0 {
        rv = nghttp3_ringbuf_reserve(
            &raw mut (*stream).refs,
            nghttp3_max_unsigned_long_int(
                4 as ::core::ffi::c_ulong,
                (nghttp3_ringbuf_len(&raw mut (*stream).refs) as ::core::ffi::c_ulong)
                    .wrapping_mul(2 as ::core::ffi::c_ulong),
            ) as size_t,
        );
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
    }
    rv = nghttp3_pq_push(&raw mut (*stream).max_cnts, &raw mut (*r#ref).max_cnts_pe);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    dest = nghttp3_ringbuf_push_back(&raw mut (*stream).refs)
        as *mut *mut nghttp3_qpack_header_block_ref;
    *dest = r#ref;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_stream_pop_ref(
    mut stream: *mut nghttp3_qpack_stream,
) {
    let mut r#ref: *mut nghttp3_qpack_header_block_ref = ::core::ptr::null_mut::<
        nghttp3_qpack_header_block_ref,
    >();
    '_c2rust_label: {
        if nghttp3_ringbuf_len(&raw mut (*stream).refs) != 0 {} else {
            __assert_fail(
                b"nghttp3_ringbuf_len(&stream->refs)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1805 as ::core::ffi::c_uint,
                b"void nghttp3_qpack_stream_pop_ref(nghttp3_qpack_stream *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    r#ref = *(nghttp3_ringbuf_get(&raw mut (*stream).refs, 0 as size_t)
        as *mut *mut nghttp3_qpack_header_block_ref);
    '_c2rust_label_0: {
        if (*r#ref).max_cnts_pe.index != 18446744073709551615 as size_t {} else {
            __assert_fail(
                b"ref->max_cnts_pe.index != NGHTTP3_PQ_BAD_INDEX\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1810 as ::core::ffi::c_uint,
                b"void nghttp3_qpack_stream_pop_ref(nghttp3_qpack_stream *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    nghttp3_pq_remove(&raw mut (*stream).max_cnts, &raw mut (*r#ref).max_cnts_pe);
    nghttp3_ringbuf_pop_front(&raw mut (*stream).refs);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_write_static_indexed(
    mut encoder: *const nghttp3_qpack_encoder,
    mut rbuf: *mut nghttp3_buf,
    mut absidx: uint64_t,
) -> ::core::ffi::c_int {
    return qpack_write_number(
        rbuf,
        0xc0 as uint8_t,
        absidx,
        6 as size_t,
        (*encoder).ctx.mem,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_write_dynamic_indexed(
    mut encoder: *const nghttp3_qpack_encoder,
    mut rbuf: *mut nghttp3_buf,
    mut absidx: uint64_t,
    mut base: uint64_t,
) -> ::core::ffi::c_int {
    if absidx < base {
        return qpack_write_number(
            rbuf,
            0x80 as uint8_t,
            base.wrapping_sub(absidx).wrapping_sub(1 as uint64_t),
            6 as size_t,
            (*encoder).ctx.mem,
        );
    }
    return qpack_write_number(
        rbuf,
        0x10 as uint8_t,
        absidx.wrapping_sub(base),
        4 as size_t,
        (*encoder).ctx.mem,
    );
}
unsafe extern "C" fn qpack_encoder_write_indexed_name(
    mut encoder: *const nghttp3_qpack_encoder,
    mut buf: *mut nghttp3_buf,
    mut fb: uint8_t,
    mut nameidx: uint64_t,
    mut prefix: size_t,
    mut nv: *const nghttp3_nv,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    let mut len: size_t = nghttp3_qpack_put_varint_len(nameidx, prefix);
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut hlen: size_t = 0;
    let mut h: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    hlen = nghttp3_qpack_huffman_encode_count((*nv).value, (*nv).valuelen);
    if hlen < (*nv).valuelen {
        h = 1 as ::core::ffi::c_int;
        len = len
            .wrapping_add(
                nghttp3_qpack_put_varint_len(hlen as uint64_t, 7 as size_t)
                    .wrapping_add(hlen),
            );
    } else {
        len = len
            .wrapping_add(
                nghttp3_qpack_put_varint_len((*nv).valuelen as uint64_t, 7 as size_t)
                    .wrapping_add((*nv).valuelen),
            );
    }
    rv = reserve_buf(buf, len, (*encoder).ctx.mem);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    p = (*buf).last;
    *p = fb;
    p = nghttp3_qpack_put_varint(p, nameidx, prefix);
    if h != 0 {
        *p = 0x80 as uint8_t;
        p = nghttp3_qpack_put_varint(p, hlen as uint64_t, 7 as size_t);
        p = nghttp3_qpack_huffman_encode(p, (*nv).value, (*nv).valuelen);
    } else {
        *p = 0 as uint8_t;
        p = nghttp3_qpack_put_varint(p, (*nv).valuelen as uint64_t, 7 as size_t);
        if (*nv).valuelen != 0 {
            p = nghttp3_cpymem(p, (*nv).value, (*nv).valuelen);
        }
    }
    '_c2rust_label: {
        if p.offset_from((*buf).last) as size_t == len {} else {
            __assert_fail(
                b"(size_t)(p - buf->last) == len\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1891 as ::core::ffi::c_uint,
                b"int qpack_encoder_write_indexed_name(const nghttp3_qpack_encoder *, nghttp3_buf *, uint8_t, uint64_t, size_t, const nghttp3_nv *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    (*buf).last = p;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_write_static_indexed_name(
    mut encoder: *const nghttp3_qpack_encoder,
    mut rbuf: *mut nghttp3_buf,
    mut absidx: uint64_t,
    mut nv: *const nghttp3_nv,
) -> ::core::ffi::c_int {
    let mut fb: uint8_t = (0x50 as ::core::ffi::c_uint
        | if (*nv).flags as ::core::ffi::c_uint & NGHTTP3_NV_FLAG_NEVER_INDEX != 0 {
            0x20 as ::core::ffi::c_uint
        } else {
            0 as ::core::ffi::c_uint
        }) as uint8_t;
    return qpack_encoder_write_indexed_name(encoder, rbuf, fb, absidx, 4 as size_t, nv);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_write_dynamic_indexed_name(
    mut encoder: *const nghttp3_qpack_encoder,
    mut rbuf: *mut nghttp3_buf,
    mut absidx: uint64_t,
    mut base: uint64_t,
    mut nv: *const nghttp3_nv,
) -> ::core::ffi::c_int {
    let mut fb: uint8_t = 0;
    if absidx < base {
        fb = (0x40 as ::core::ffi::c_uint
            | if (*nv).flags as ::core::ffi::c_uint & NGHTTP3_NV_FLAG_NEVER_INDEX != 0 {
                0x20 as ::core::ffi::c_uint
            } else {
                0 as ::core::ffi::c_uint
            }) as uint8_t;
        return qpack_encoder_write_indexed_name(
            encoder,
            rbuf,
            fb,
            base.wrapping_sub(absidx).wrapping_sub(1 as uint64_t),
            4 as size_t,
            nv,
        );
    }
    fb = (if (*nv).flags as ::core::ffi::c_uint & NGHTTP3_NV_FLAG_NEVER_INDEX != 0 {
        0x8 as ::core::ffi::c_uint
    } else {
        0 as ::core::ffi::c_uint
    }) as uint8_t;
    return qpack_encoder_write_indexed_name(
        encoder,
        rbuf,
        fb,
        absidx.wrapping_sub(base),
        3 as size_t,
        nv,
    );
}
unsafe extern "C" fn qpack_encoder_write_literal(
    mut encoder: *const nghttp3_qpack_encoder,
    mut buf: *mut nghttp3_buf,
    mut fb: uint8_t,
    mut prefix: size_t,
    mut nv: *const nghttp3_nv,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    let mut len: size_t = 0;
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut nhlen: size_t = 0;
    let mut vhlen: size_t = 0;
    let mut nh: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut vh: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    nhlen = nghttp3_qpack_huffman_encode_count((*nv).name, (*nv).namelen);
    if nhlen < (*nv).namelen {
        nh = 1 as ::core::ffi::c_int;
        len = nghttp3_qpack_put_varint_len(nhlen as uint64_t, prefix)
            .wrapping_add(nhlen);
    } else {
        len = nghttp3_qpack_put_varint_len((*nv).namelen as uint64_t, prefix)
            .wrapping_add((*nv).namelen);
    }
    vhlen = nghttp3_qpack_huffman_encode_count((*nv).value, (*nv).valuelen);
    if vhlen < (*nv).valuelen {
        vh = 1 as ::core::ffi::c_int;
        len = len
            .wrapping_add(
                nghttp3_qpack_put_varint_len(vhlen as uint64_t, 7 as size_t)
                    .wrapping_add(vhlen),
            );
    } else {
        len = len
            .wrapping_add(
                nghttp3_qpack_put_varint_len((*nv).valuelen as uint64_t, 7 as size_t)
                    .wrapping_add((*nv).valuelen),
            );
    }
    rv = reserve_buf(buf, len, (*encoder).ctx.mem);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    p = (*buf).last;
    *p = fb;
    if nh != 0 {
        *p = (*p as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << prefix) as uint8_t as ::core::ffi::c_int)
            as uint8_t;
        p = nghttp3_qpack_put_varint(p, nhlen as uint64_t, prefix);
        p = nghttp3_qpack_huffman_encode(p, (*nv).name, (*nv).namelen);
    } else {
        p = nghttp3_qpack_put_varint(p, (*nv).namelen as uint64_t, prefix);
        if (*nv).namelen != 0 {
            p = nghttp3_cpymem(p, (*nv).name, (*nv).namelen);
        }
    }
    *p = 0 as uint8_t;
    if vh != 0 {
        *p = (*p as ::core::ffi::c_uint | 0x80 as ::core::ffi::c_uint) as uint8_t;
        p = nghttp3_qpack_put_varint(p, vhlen as uint64_t, 7 as size_t);
        p = nghttp3_qpack_huffman_encode(p, (*nv).value, (*nv).valuelen);
    } else {
        p = nghttp3_qpack_put_varint(p, (*nv).valuelen as uint64_t, 7 as size_t);
        if (*nv).valuelen != 0 {
            p = nghttp3_cpymem(p, (*nv).value, (*nv).valuelen);
        }
    }
    '_c2rust_label: {
        if p.offset_from((*buf).last) as size_t == len {} else {
            __assert_fail(
                b"(size_t)(p - buf->last) == len\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2001 as ::core::ffi::c_uint,
                b"int qpack_encoder_write_literal(const nghttp3_qpack_encoder *, nghttp3_buf *, uint8_t, size_t, const nghttp3_nv *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    (*buf).last = p;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_write_literal(
    mut encoder: *const nghttp3_qpack_encoder,
    mut rbuf: *mut nghttp3_buf,
    mut nv: *const nghttp3_nv,
) -> ::core::ffi::c_int {
    let mut fb: uint8_t = (0x20 as ::core::ffi::c_uint
        | if (*nv).flags as ::core::ffi::c_uint & NGHTTP3_NV_FLAG_NEVER_INDEX != 0 {
            0x10 as ::core::ffi::c_uint
        } else {
            0 as ::core::ffi::c_uint
        }) as uint8_t;
    return qpack_encoder_write_literal(encoder, rbuf, fb, 3 as size_t, nv);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_write_static_insert(
    mut encoder: *const nghttp3_qpack_encoder,
    mut ebuf: *mut nghttp3_buf,
    mut absidx: uint64_t,
    mut nv: *const nghttp3_nv,
) -> ::core::ffi::c_int {
    return qpack_encoder_write_indexed_name(
        encoder,
        ebuf,
        0xc0 as uint8_t,
        absidx,
        6 as size_t,
        nv,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_write_dynamic_insert(
    mut encoder: *const nghttp3_qpack_encoder,
    mut ebuf: *mut nghttp3_buf,
    mut absidx: uint64_t,
    mut nv: *const nghttp3_nv,
) -> ::core::ffi::c_int {
    return qpack_encoder_write_indexed_name(
        encoder,
        ebuf,
        0x80 as uint8_t,
        (*encoder).ctx.next_absidx.wrapping_sub(absidx).wrapping_sub(1 as uint64_t),
        6 as size_t,
        nv,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_write_duplicate_insert(
    mut encoder: *const nghttp3_qpack_encoder,
    mut ebuf: *mut nghttp3_buf,
    mut absidx: uint64_t,
) -> ::core::ffi::c_int {
    let mut idx: uint64_t = (*encoder)
        .ctx
        .next_absidx
        .wrapping_sub(absidx)
        .wrapping_sub(1 as uint64_t);
    let mut len: size_t = nghttp3_qpack_put_varint_len(idx, 5 as size_t);
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut rv: ::core::ffi::c_int = 0;
    rv = reserve_buf(ebuf, len, (*encoder).ctx.mem);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    p = (*ebuf).last;
    *p = 0 as uint8_t;
    p = nghttp3_qpack_put_varint(p, idx, 5 as size_t);
    '_c2rust_label: {
        if p.offset_from((*ebuf).last) as size_t == len {} else {
            __assert_fail(
                b"(size_t)(p - ebuf->last) == len\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2057 as ::core::ffi::c_uint,
                b"int nghttp3_qpack_encoder_write_duplicate_insert(const nghttp3_qpack_encoder *, nghttp3_buf *, uint64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    (*ebuf).last = p;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_write_literal_insert(
    mut encoder: *const nghttp3_qpack_encoder,
    mut ebuf: *mut nghttp3_buf,
    mut nv: *const nghttp3_nv,
) -> ::core::ffi::c_int {
    return qpack_encoder_write_literal(encoder, ebuf, 0x40 as uint8_t, 5 as size_t, nv);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_context_dtable_add(
    mut ctx: *mut nghttp3_qpack_context,
    mut qnv: *mut nghttp3_qpack_nv,
    mut dtable_map: *mut nghttp3_qpack_map,
    mut hash: uint32_t,
) -> ::core::ffi::c_int {
    let mut new_ent: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    let mut p: *mut *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        *mut nghttp3_qpack_entry,
    >();
    let mut ent: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    let mut mem: *const nghttp3_mem = (*ctx).mem;
    let mut space: size_t = 0;
    let mut i: size_t = 0;
    let mut rv: ::core::ffi::c_int = 0;
    space = table_space((*(*qnv).name).len, (*(*qnv).value).len);
    '_c2rust_label: {
        if space <= (*ctx).max_dtable_capacity {} else {
            __assert_fail(
                b"space <= ctx->max_dtable_capacity\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2083 as ::core::ffi::c_uint,
                b"int nghttp3_qpack_context_dtable_add(nghttp3_qpack_context *, nghttp3_qpack_nv *, nghttp3_qpack_map *, uint32_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    while (*ctx).dtable_size.wrapping_add(space) > (*ctx).max_dtable_capacity {
        i = nghttp3_ringbuf_len(&raw mut (*ctx).dtable);
        '_c2rust_label_0: {
            if i != 0 {} else {
                __assert_fail(
                    b"i\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2087 as ::core::ffi::c_uint,
                    b"int nghttp3_qpack_context_dtable_add(nghttp3_qpack_context *, nghttp3_qpack_nv *, nghttp3_qpack_map *, uint32_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        ent = *(nghttp3_ringbuf_get(&raw mut (*ctx).dtable, i.wrapping_sub(1 as size_t))
            as *mut *mut nghttp3_qpack_entry);
        (*ctx).dtable_size = (*ctx)
            .dtable_size
            .wrapping_sub(table_space((*(*ent).nv.name).len, (*(*ent).nv.value).len));
        nghttp3_ringbuf_pop_back(&raw mut (*ctx).dtable);
        if !dtable_map.is_null() {
            qpack_map_remove(dtable_map, ent);
        }
        nghttp3_qpack_entry_free(ent);
        nghttp3_mem_free(mem, ent as *mut ::core::ffi::c_void);
    }
    new_ent = nghttp3_mem_malloc(mem, ::core::mem::size_of::<nghttp3_qpack_entry>())
        as *mut nghttp3_qpack_entry;
    if new_ent.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    let c2rust_fresh2 = (*ctx).next_absidx;
    (*ctx).next_absidx = (*ctx).next_absidx.wrapping_add(1);
    nghttp3_qpack_entry_init(new_ent, qnv, (*ctx).dtable_sum, c2rust_fresh2, hash);
    if nghttp3_ringbuf_full(&raw mut (*ctx).dtable) != 0 {
        rv = nghttp3_ringbuf_reserve(
            &raw mut (*ctx).dtable,
            nghttp3_max_unsigned_long_int(
                128 as ::core::ffi::c_ulong,
                (nghttp3_ringbuf_len(&raw mut (*ctx).dtable) as ::core::ffi::c_ulong)
                    .wrapping_mul(2 as ::core::ffi::c_ulong),
            ) as size_t,
        );
        if rv != 0 as ::core::ffi::c_int {
            nghttp3_qpack_entry_free(new_ent);
            nghttp3_mem_free(mem, new_ent as *mut ::core::ffi::c_void);
            return rv;
        }
    }
    p = nghttp3_ringbuf_push_front(&raw mut (*ctx).dtable)
        as *mut *mut nghttp3_qpack_entry;
    *p = new_ent;
    if !dtable_map.is_null() {
        qpack_map_insert(dtable_map, new_ent);
    }
    (*ctx).dtable_size = (*ctx).dtable_size.wrapping_add(space);
    (*ctx).dtable_sum = (*ctx).dtable_sum.wrapping_add(space);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_dtable_static_add(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut absidx: uint64_t,
    mut nv: *const nghttp3_nv,
    mut hash: uint32_t,
) -> ::core::ffi::c_int {
    let mut shd: *const nghttp3_qpack_static_header = ::core::ptr::null::<
        nghttp3_qpack_static_header,
    >();
    let mut qnv: nghttp3_qpack_nv = nghttp3_qpack_nv {
        name: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        value: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        token: 0,
        flags: 0,
    };
    let mut mem: *const nghttp3_mem = (*encoder).ctx.mem;
    let mut rv: ::core::ffi::c_int = 0;
    rv = nghttp3_rcbuf_new2(&raw mut qnv.value, (*nv).value, (*nv).valuelen, mem);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    '_c2rust_label: {
        if ::core::mem::size_of::<[nghttp3_qpack_static_header; 99]>()
            .wrapping_div(::core::mem::size_of::<nghttp3_qpack_static_header>())
            > absidx as usize
        {} else {
            __assert_fail(
                b"nghttp3_arraylen(stable) > absidx\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2150 as ::core::ffi::c_uint,
                b"int nghttp3_qpack_encoder_dtable_static_add(nghttp3_qpack_encoder *, uint64_t, const nghttp3_nv *, uint32_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    shd = (&raw mut stable as *mut nghttp3_qpack_static_header).offset(absidx as isize);
    qnv.name = &raw const (*shd).name as *mut nghttp3_rcbuf;
    qnv.token = (*shd).token;
    qnv.flags = NGHTTP3_NV_FLAG_NONE as uint8_t;
    rv = nghttp3_qpack_context_dtable_add(
        &raw mut (*encoder).ctx,
        &raw mut qnv,
        &raw mut (*encoder).dtable_map,
        hash,
    );
    nghttp3_rcbuf_decref(qnv.value);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_dtable_dynamic_add(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut absidx: uint64_t,
    mut nv: *const nghttp3_nv,
    mut hash: uint32_t,
) -> ::core::ffi::c_int {
    let mut qnv: nghttp3_qpack_nv = nghttp3_qpack_nv {
        name: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        value: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        token: 0,
        flags: 0,
    };
    let mut ent: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    let mut mem: *const nghttp3_mem = (*encoder).ctx.mem;
    let mut rv: ::core::ffi::c_int = 0;
    rv = nghttp3_rcbuf_new2(&raw mut qnv.value, (*nv).value, (*nv).valuelen, mem);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    ent = nghttp3_qpack_context_dtable_get(&raw mut (*encoder).ctx, absidx);
    qnv.name = (*ent).nv.name;
    qnv.token = (*ent).nv.token;
    qnv.flags = NGHTTP3_NV_FLAG_NONE as uint8_t;
    nghttp3_rcbuf_incref(qnv.name);
    rv = nghttp3_qpack_context_dtable_add(
        &raw mut (*encoder).ctx,
        &raw mut qnv,
        &raw mut (*encoder).dtable_map,
        hash,
    );
    nghttp3_rcbuf_decref(qnv.value);
    nghttp3_rcbuf_decref(qnv.name);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_dtable_duplicate_add(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut absidx: uint64_t,
) -> ::core::ffi::c_int {
    let mut qnv: nghttp3_qpack_nv = nghttp3_qpack_nv {
        name: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        value: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        token: 0,
        flags: 0,
    };
    let mut ent: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    let mut rv: ::core::ffi::c_int = 0;
    ent = nghttp3_qpack_context_dtable_get(&raw mut (*encoder).ctx, absidx);
    qnv = (*ent).nv;
    nghttp3_rcbuf_incref(qnv.name);
    nghttp3_rcbuf_incref(qnv.value);
    rv = nghttp3_qpack_context_dtable_add(
        &raw mut (*encoder).ctx,
        &raw mut qnv,
        &raw mut (*encoder).dtable_map,
        (*ent).hash,
    );
    nghttp3_rcbuf_decref(qnv.name);
    nghttp3_rcbuf_decref(qnv.value);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_dtable_literal_add(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut nv: *const nghttp3_nv,
    mut token: int32_t,
    mut hash: uint32_t,
) -> ::core::ffi::c_int {
    let mut qnv: nghttp3_qpack_nv = nghttp3_qpack_nv {
        name: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        value: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        token: 0,
        flags: 0,
    };
    let mut mem: *const nghttp3_mem = (*encoder).ctx.mem;
    let mut rv: ::core::ffi::c_int = 0;
    rv = nghttp3_rcbuf_new2(&raw mut qnv.name, (*nv).name, (*nv).namelen, mem);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    rv = nghttp3_rcbuf_new2(&raw mut qnv.value, (*nv).value, (*nv).valuelen, mem);
    if rv != 0 as ::core::ffi::c_int {
        nghttp3_rcbuf_decref(qnv.name);
        return rv;
    }
    qnv.token = token;
    qnv.flags = NGHTTP3_NV_FLAG_NONE as uint8_t;
    rv = nghttp3_qpack_context_dtable_add(
        &raw mut (*encoder).ctx,
        &raw mut qnv,
        &raw mut (*encoder).dtable_map,
        hash,
    );
    nghttp3_rcbuf_decref(qnv.value);
    nghttp3_rcbuf_decref(qnv.name);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_context_dtable_get(
    mut ctx: *mut nghttp3_qpack_context,
    mut absidx: uint64_t,
) -> *mut nghttp3_qpack_entry {
    let mut relidx: size_t = 0;
    '_c2rust_label: {
        if (*ctx).next_absidx > absidx {} else {
            __assert_fail(
                b"ctx->next_absidx > absidx\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2252 as ::core::ffi::c_uint,
                b"nghttp3_qpack_entry *nghttp3_qpack_context_dtable_get(nghttp3_qpack_context *, uint64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*ctx).next_absidx.wrapping_sub(absidx).wrapping_sub(1 as uint64_t)
            < nghttp3_ringbuf_len(&raw mut (*ctx).dtable) as uint64_t
        {} else {
            __assert_fail(
                b"ctx->next_absidx - absidx - 1 < nghttp3_ringbuf_len(&ctx->dtable)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2253 as ::core::ffi::c_uint,
                b"nghttp3_qpack_entry *nghttp3_qpack_context_dtable_get(nghttp3_qpack_context *, uint64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    relidx = (*ctx).next_absidx.wrapping_sub(absidx).wrapping_sub(1 as uint64_t)
        as size_t;
    return *(nghttp3_ringbuf_get(&raw mut (*ctx).dtable, relidx)
        as *mut *mut nghttp3_qpack_entry);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_context_dtable_top(
    mut ctx: *mut nghttp3_qpack_context,
) -> *mut nghttp3_qpack_entry {
    '_c2rust_label: {
        if nghttp3_ringbuf_len(&raw mut (*ctx).dtable) != 0 {} else {
            __assert_fail(
                b"nghttp3_ringbuf_len(&ctx->dtable)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2262 as ::core::ffi::c_uint,
                b"nghttp3_qpack_entry *nghttp3_qpack_context_dtable_top(nghttp3_qpack_context *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    return *(nghttp3_ringbuf_get(&raw mut (*ctx).dtable, 0 as size_t)
        as *mut *mut nghttp3_qpack_entry);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_entry_init(
    mut ent: *mut nghttp3_qpack_entry,
    mut qnv: *mut nghttp3_qpack_nv,
    mut sum: size_t,
    mut absidx: uint64_t,
    mut hash: uint32_t,
) {
    (*ent).nv = *qnv;
    (*ent).map_next = ::core::ptr::null_mut::<nghttp3_qpack_entry>();
    (*ent).sum = sum;
    (*ent).absidx = absidx;
    (*ent).hash = hash;
    nghttp3_rcbuf_incref((*ent).nv.name);
    nghttp3_rcbuf_incref((*ent).nv.value);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_entry_free(mut ent: *mut nghttp3_qpack_entry) {
    nghttp3_rcbuf_decref((*ent).nv.value);
    nghttp3_rcbuf_decref((*ent).nv.name);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_block_stream(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut stream: *mut nghttp3_qpack_stream,
) -> ::core::ffi::c_int {
    let mut bsk: nghttp3_blocked_streams_key = nghttp3_blocked_streams_key {
        max_cnt: (*((nghttp3_pq_top(&raw mut (*stream).max_cnts)
            as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
            as *mut nghttp3_qpack_header_block_ref))
            .max_cnt,
        id: (*stream).stream_id as uint64_t,
    };
    return nghttp3_ksl_insert(
        &raw mut (*encoder).blocked_streams,
        ::core::ptr::null_mut::<nghttp3_ksl_it>(),
        &raw mut bsk as *const ::core::ffi::c_void,
        stream as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_unblock_stream(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut stream: *mut nghttp3_qpack_stream,
) {
    let mut bsk: nghttp3_blocked_streams_key = nghttp3_blocked_streams_key {
        max_cnt: (*((nghttp3_pq_top(&raw mut (*stream).max_cnts)
            as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut ::core::ffi::c_void
            as *mut nghttp3_qpack_header_block_ref))
            .max_cnt,
        id: (*stream).stream_id as uint64_t,
    };
    let mut it: nghttp3_ksl_it = nghttp3_ksl_it {
        blk: ::core::ptr::null_mut::<nghttp3_ksl_blk>(),
        i: 0,
    };
    it = nghttp3_ksl_lower_bound(
        &raw mut (*encoder).blocked_streams,
        &raw mut bsk as *const ::core::ffi::c_void,
    );
    '_c2rust_label: {
        if nghttp3_ksl_it_end(&raw mut it) == 0 {} else {
            __assert_fail(
                b"!nghttp3_ksl_it_end(&it)\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2308 as ::core::ffi::c_uint,
                b"void nghttp3_qpack_encoder_unblock_stream(nghttp3_qpack_encoder *, nghttp3_qpack_stream *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if nghttp3_ksl_it_get(&raw mut it) == stream as *mut ::core::ffi::c_void
        {} else {
            __assert_fail(
                b"nghttp3_ksl_it_get(&it) == stream\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2309 as ::core::ffi::c_uint,
                b"void nghttp3_qpack_encoder_unblock_stream(nghttp3_qpack_encoder *, nghttp3_qpack_stream *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    nghttp3_ksl_remove_hint(
        &raw mut (*encoder).blocked_streams,
        ::core::ptr::null_mut::<nghttp3_ksl_it>(),
        &raw mut it,
        &raw mut bsk as *const ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_unblock(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut max_cnt: uint64_t,
) {
    let mut bsk: nghttp3_blocked_streams_key = nghttp3_blocked_streams_key {
        max_cnt: max_cnt,
        id: 0,
    };
    let mut it: nghttp3_ksl_it = nghttp3_ksl_it {
        blk: ::core::ptr::null_mut::<nghttp3_ksl_blk>(),
        i: 0,
    };
    it = nghttp3_ksl_lower_bound(
        &raw mut (*encoder).blocked_streams,
        &raw mut bsk as *const ::core::ffi::c_void,
    );
    while nghttp3_ksl_it_end(&raw mut it) == 0 {
        bsk = *(nghttp3_ksl_it_key(&raw mut it) as *mut nghttp3_blocked_streams_key);
        nghttp3_ksl_remove_hint(
            &raw mut (*encoder).blocked_streams,
            &raw mut it,
            &raw mut it,
            &raw mut bsk as *const ::core::ffi::c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_ack_header(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut nghttp3_qpack_stream = nghttp3_qpack_encoder_find_stream(
        encoder,
        stream_id,
    );
    let mut mem: *const nghttp3_mem = (*encoder).ctx.mem;
    let mut r#ref: *mut nghttp3_qpack_header_block_ref = ::core::ptr::null_mut::<
        nghttp3_qpack_header_block_ref,
    >();
    if stream.is_null() {
        return NGHTTP3_ERR_QPACK_DECODER_STREAM_ERROR;
    }
    '_c2rust_label: {
        if nghttp3_ringbuf_len(&raw mut (*stream).refs) != 0 {} else {
            __assert_fail(
                b"nghttp3_ringbuf_len(&stream->refs)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2340 as ::core::ffi::c_uint,
                b"int nghttp3_qpack_encoder_ack_header(nghttp3_qpack_encoder *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    r#ref = *(nghttp3_ringbuf_get(&raw mut (*stream).refs, 0 as size_t)
        as *mut *mut nghttp3_qpack_header_block_ref);
    if (*encoder).krcnt < (*r#ref).max_cnt {
        (*encoder).krcnt = (*r#ref).max_cnt;
        nghttp3_qpack_encoder_unblock(encoder, (*r#ref).max_cnt);
    }
    nghttp3_qpack_stream_pop_ref(stream);
    '_c2rust_label_0: {
        if (*r#ref).min_cnts_pe.index != 18446744073709551615 as size_t {} else {
            __assert_fail(
                b"ref->min_cnts_pe.index != NGHTTP3_PQ_BAD_INDEX\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2357 as ::core::ffi::c_uint,
                b"int nghttp3_qpack_encoder_ack_header(nghttp3_qpack_encoder *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    nghttp3_pq_remove(&raw mut (*encoder).min_cnts, &raw mut (*r#ref).min_cnts_pe);
    nghttp3_qpack_header_block_ref_del(r#ref, mem);
    if nghttp3_ringbuf_len(&raw mut (*stream).refs) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    qpack_encoder_remove_stream(encoder, stream);
    nghttp3_qpack_stream_del(stream, mem);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_add_icnt(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut n: uint64_t,
) -> ::core::ffi::c_int {
    if n == 0 as uint64_t
        || (*encoder).ctx.next_absidx.wrapping_sub((*encoder).krcnt) < n
    {
        return NGHTTP3_ERR_QPACK_DECODER_STREAM_ERROR;
    }
    (*encoder).krcnt = (*encoder).krcnt.wrapping_add(n);
    nghttp3_qpack_encoder_unblock(encoder, (*encoder).krcnt);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_ack_everything(
    mut encoder: *mut nghttp3_qpack_encoder,
) {
    (*encoder).krcnt = (*encoder).ctx.next_absidx;
    nghttp3_ksl_clear(&raw mut (*encoder).blocked_streams);
    nghttp3_pq_clear(&raw mut (*encoder).min_cnts);
    nghttp3_map_each(
        &raw mut (*encoder).streams,
        Some(
            map_stream_free
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        (*encoder).ctx.mem as *mut ::core::ffi::c_void,
    );
    nghttp3_map_clear(&raw mut (*encoder).streams);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_cancel_stream(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut stream_id: int64_t,
) {
    let mut stream: *mut nghttp3_qpack_stream = nghttp3_qpack_encoder_find_stream(
        encoder,
        stream_id,
    );
    let mut mem: *const nghttp3_mem = (*encoder).ctx.mem;
    if stream.is_null() {
        return;
    }
    if nghttp3_qpack_encoder_stream_is_blocked(encoder, stream) != 0 {
        nghttp3_qpack_encoder_unblock_stream(encoder, stream);
    }
    qpack_encoder_remove_stream(encoder, stream);
    nghttp3_qpack_stream_del(stream, mem);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_get_num_blocked_streams(
    mut encoder: *mut nghttp3_qpack_encoder,
) -> size_t {
    return nghttp3_qpack_encoder_get_num_blocked_streams2(encoder);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_get_num_blocked_streams2(
    mut encoder: *const nghttp3_qpack_encoder,
) -> size_t {
    return nghttp3_ksl_len(&raw const (*encoder).blocked_streams);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_write_field_section_prefix(
    mut encoder: *const nghttp3_qpack_encoder,
    mut pbuf: *mut nghttp3_buf,
    mut ricnt: uint64_t,
    mut base: uint64_t,
) -> ::core::ffi::c_int {
    let mut max_ents: size_t = (*encoder)
        .ctx
        .hard_max_dtable_capacity
        .wrapping_div(NGHTTP3_QPACK_ENTRY_OVERHEAD as size_t);
    let mut encricnt: uint64_t = if ricnt == 0 as uint64_t {
        0 as uint64_t
    } else {
        ricnt
            .wrapping_rem((2 as uint64_t).wrapping_mul(max_ents as uint64_t))
            .wrapping_add(1 as uint64_t)
    };
    let mut sign: ::core::ffi::c_int = (base < ricnt) as ::core::ffi::c_int;
    let mut delta_base: uint64_t = if sign != 0 {
        ricnt.wrapping_sub(base).wrapping_sub(1 as uint64_t)
    } else {
        base.wrapping_sub(ricnt)
    };
    let mut len: size_t = nghttp3_qpack_put_varint_len(encricnt, 8 as size_t)
        .wrapping_add(nghttp3_qpack_put_varint_len(delta_base, 7 as size_t));
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut rv: ::core::ffi::c_int = 0;
    rv = reserve_buf(pbuf, len, (*encoder).ctx.mem);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    p = (*pbuf).last;
    p = nghttp3_qpack_put_varint(p, encricnt, 8 as size_t);
    if sign != 0 {
        *p = 0x80 as uint8_t;
    } else {
        *p = 0 as uint8_t;
    }
    p = nghttp3_qpack_put_varint(p, delta_base, 7 as size_t);
    '_c2rust_label: {
        if p.offset_from((*pbuf).last) as size_t == len {} else {
            __assert_fail(
                b"(size_t)(p - pbuf->last) == len\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2455 as ::core::ffi::c_uint,
                b"int nghttp3_qpack_encoder_write_field_section_prefix(const nghttp3_qpack_encoder *, nghttp3_buf *, uint64_t, uint64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    (*pbuf).last = p;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn qpack_read_varint(
    mut fin: *mut ::core::ffi::c_int,
    mut rstate: *mut nghttp3_qpack_read_state,
    mut begin: *const uint8_t,
    mut end: *const uint8_t,
) -> nghttp3_ssize {
    let mut k: uint64_t = (((1 as ::core::ffi::c_int) << (*rstate).prefix)
        - 1 as ::core::ffi::c_int) as uint8_t as uint64_t;
    let mut n: uint64_t = (*rstate).left;
    let mut add: uint64_t = 0;
    let mut p: *const uint8_t = begin;
    let mut shift: size_t = (*rstate).shift;
    (*rstate).shift = 0 as size_t;
    *fin = 0 as ::core::ffi::c_int;
    if n == 0 as uint64_t {
        if *p as uint64_t & k != k {
            (*rstate).left = *p as uint64_t & k;
            *fin = 1 as ::core::ffi::c_int;
            return 1 as nghttp3_ssize;
        }
        n = k;
        p = p.offset(1);
        if p == end {
            (*rstate).left = n;
            return p.offset_from(begin);
        }
    }
    while p != end {
        add = (*p as ::core::ffi::c_uint & 0x7f as ::core::ffi::c_uint) as uint64_t;
        if shift > 62 as size_t {
            return NGHTTP3_ERR_QPACK_FATAL as nghttp3_ssize;
        }
        if NGHTTP3_QPACK_INT_MAX >> shift < add as ::core::ffi::c_ulonglong {
            return NGHTTP3_ERR_QPACK_FATAL as nghttp3_ssize;
        }
        add <<= shift;
        if NGHTTP3_QPACK_INT_MAX.wrapping_sub(add as ::core::ffi::c_ulonglong)
            < n as ::core::ffi::c_ulonglong
        {
            return NGHTTP3_ERR_QPACK_FATAL as nghttp3_ssize;
        }
        n = n.wrapping_add(add);
        if *p as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            break;
        }
        p = p.offset(1);
        shift = shift.wrapping_add(7 as size_t);
    }
    (*rstate).shift = shift;
    if p == end {
        (*rstate).left = n;
        return p.offset_from(begin);
    }
    (*rstate).left = n;
    *fin = 1 as ::core::ffi::c_int;
    return p.offset(1 as ::core::ffi::c_int as isize).offset_from(begin);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_read_decoder(
    mut encoder: *mut nghttp3_qpack_encoder,
    mut src: *const uint8_t,
    mut srclen: size_t,
) -> nghttp3_ssize {
    let mut p: *const uint8_t = src;
    let mut end: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut rv: ::core::ffi::c_int = 0;
    let mut nread: nghttp3_ssize = 0;
    let mut rfin: ::core::ffi::c_int = 0;
    if (*encoder).ctx.bad != 0 {
        return NGHTTP3_ERR_QPACK_FATAL as nghttp3_ssize;
    }
    if srclen == 0 as size_t {
        return 0 as nghttp3_ssize;
    }
    (*encoder).uninterrupted_decoderlen = (*encoder)
        .uninterrupted_decoderlen
        .wrapping_add(srclen);
    if (*encoder).uninterrupted_decoderlen > NGHTTP3_QPACK_MAX_DECODERLEN as size_t {
        return NGHTTP3_ERR_QPACK_DECODER_STREAM_ERROR as nghttp3_ssize;
    }
    end = src.offset(srclen as isize);
    '_fail: {
        while p != end {
            match (*encoder).state {
                nghttp3_qpack_decoder_stream_state::NGHTTP3_QPACK_DS_STATE_OPCODE => {
                    match *p as ::core::ffi::c_int >> 6 as ::core::ffi::c_int {
                        0 => {
                            (*encoder).opcode = nghttp3_qpack_decoder_stream_opcode::NGHTTP3_QPACK_DS_OPCODE_ICNT_INCREMENT;
                            (*encoder).rstate.prefix = 6 as size_t;
                        }
                        0x1 => {
                            (*encoder).opcode = nghttp3_qpack_decoder_stream_opcode::NGHTTP3_QPACK_DS_OPCODE_STREAM_CANCEL;
                            (*encoder).rstate.prefix = 6 as size_t;
                        }
                        _ => {
                            (*encoder).opcode = nghttp3_qpack_decoder_stream_opcode::NGHTTP3_QPACK_DS_OPCODE_SECTION_ACK;
                            (*encoder).rstate.prefix = 7 as size_t;
                        }
                    }
                    (*encoder).state = nghttp3_qpack_decoder_stream_state::NGHTTP3_QPACK_DS_STATE_READ_NUMBER;
                }
                nghttp3_qpack_decoder_stream_state::NGHTTP3_QPACK_DS_STATE_READ_NUMBER => {}
                _ => {
                    nghttp3_unreachable_fail(
                        b"nghttp3_qpack.c\0".as_ptr() as *const ::core::ffi::c_char,
                        2631 as ::core::ffi::c_int,
                        b"nghttp3_qpack_encoder_read_decoder\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            }
            nread = qpack_read_varint(&raw mut rfin, &raw mut (*encoder).rstate, p, end);
            if nread < 0 as nghttp3_ssize {
                '_c2rust_label: {
                    if nread == -108 as nghttp3_ssize {} else {
                        __assert_fail(
                            b"nread == NGHTTP3_ERR_QPACK_FATAL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            2594 as ::core::ffi::c_uint,
                            b"nghttp3_ssize nghttp3_qpack_encoder_read_decoder(nghttp3_qpack_encoder *, const uint8_t *, size_t)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                rv = NGHTTP3_ERR_QPACK_DECODER_STREAM_ERROR;
                break '_fail;
            } else {
                p = p.offset(nread as isize);
                if rfin == 0 {
                    return p.offset_from(src);
                }
                match (*encoder).opcode {
                    nghttp3_qpack_decoder_stream_opcode::NGHTTP3_QPACK_DS_OPCODE_ICNT_INCREMENT => {
                        rv = nghttp3_qpack_encoder_add_icnt(
                            encoder,
                            (*encoder).rstate.left,
                        );
                        if rv != 0 as ::core::ffi::c_int {
                            break '_fail;
                        }
                    }
                    nghttp3_qpack_decoder_stream_opcode::NGHTTP3_QPACK_DS_OPCODE_SECTION_ACK => {
                        rv = nghttp3_qpack_encoder_ack_header(
                            encoder,
                            (*encoder).rstate.left as int64_t,
                        );
                        if rv != 0 as ::core::ffi::c_int {
                            break '_fail;
                        }
                    }
                    nghttp3_qpack_decoder_stream_opcode::NGHTTP3_QPACK_DS_OPCODE_STREAM_CANCEL => {
                        nghttp3_qpack_encoder_cancel_stream(
                            encoder,
                            (*encoder).rstate.left as int64_t,
                        );
                    }
                    _ => {
                        nghttp3_unreachable_fail(
                            b"nghttp3_qpack.c\0".as_ptr() as *const ::core::ffi::c_char,
                            2624 as ::core::ffi::c_int,
                            b"nghttp3_qpack_encoder_read_decoder\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
                (*encoder).state = nghttp3_qpack_decoder_stream_state::NGHTTP3_QPACK_DS_STATE_OPCODE;
                nghttp3_qpack_read_state_reset(&raw mut (*encoder).rstate);
            }
        }
        return p.offset_from(src);
    }
    (*encoder).ctx.bad = 1 as uint8_t;
    return rv as nghttp3_ssize;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_put_varint_len(
    mut n: uint64_t,
    mut prefix: size_t,
) -> size_t {
    let mut k: size_t = (((1 as ::core::ffi::c_int) << prefix) - 1 as ::core::ffi::c_int)
        as size_t;
    let mut len: size_t = 0 as size_t;
    if n < k as uint64_t {
        return 1 as size_t;
    }
    n = (n as ::core::ffi::c_ulong).wrapping_sub(k as ::core::ffi::c_ulong) as uint64_t;
    len = len.wrapping_add(1);
    while n >= 128 as uint64_t {
        n >>= 7 as ::core::ffi::c_int;
        len = len.wrapping_add(1);
    }
    return len.wrapping_add(1 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_put_varint(
    mut buf: *mut uint8_t,
    mut n: uint64_t,
    mut prefix: size_t,
) -> *mut uint8_t {
    let mut k: size_t = (((1 as ::core::ffi::c_int) << prefix) - 1 as ::core::ffi::c_int)
        as size_t;
    *buf = (*buf as size_t & !k) as uint8_t;
    if n < k as uint64_t {
        *buf = (*buf as uint64_t | n) as uint8_t;
        return buf.offset(1 as ::core::ffi::c_int as isize);
    }
    *buf = (*buf as size_t | k) as uint8_t;
    buf = buf.offset(1);
    n = (n as ::core::ffi::c_ulong).wrapping_sub(k as ::core::ffi::c_ulong) as uint64_t;
    while n >= 128 as uint64_t {
        let c2rust_fresh0 = buf;
        buf = buf.offset(1);
        *c2rust_fresh0 = (0x80 as uint64_t | n & 0x7f as uint64_t) as uint8_t;
        n >>= 7 as ::core::ffi::c_int;
    }
    let c2rust_fresh1 = buf;
    buf = buf.offset(1);
    *c2rust_fresh1 = n as uint8_t;
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_read_state_free(
    mut rstate: *mut nghttp3_qpack_read_state,
) {
    nghttp3_rcbuf_decref((*rstate).value);
    nghttp3_rcbuf_decref((*rstate).name);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_read_state_reset(
    mut rstate: *mut nghttp3_qpack_read_state,
) {
    (*rstate).name = ::core::ptr::null_mut::<nghttp3_rcbuf>();
    (*rstate).value = ::core::ptr::null_mut::<nghttp3_rcbuf>();
    nghttp3_buf_init(&raw mut (*rstate).namebuf);
    nghttp3_buf_init(&raw mut (*rstate).valuebuf);
    (*rstate).left = 0 as uint64_t;
    (*rstate).prefix = 0 as size_t;
    (*rstate).shift = 0 as size_t;
    (*rstate).absidx = 0 as uint64_t;
    (*rstate).never = 0 as ::core::ffi::c_int;
    (*rstate).dynamic = 0 as ::core::ffi::c_int;
    (*rstate).huffman_encoded = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_init(
    mut decoder: *mut nghttp3_qpack_decoder,
    mut hard_max_dtable_capacity: size_t,
    mut max_blocked_streams: size_t,
    mut mem: *const nghttp3_mem,
) {
    qpack_context_init(
        &raw mut (*decoder).ctx,
        hard_max_dtable_capacity,
        max_blocked_streams,
        mem,
    );
    (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_OPCODE;
    (*decoder).opcode = nghttp3_qpack_encoder_stream_opcode(
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*decoder).written_icnt = 0 as uint64_t;
    (*decoder).max_concurrent_streams = 0 as size_t;
    (*decoder).uninterrupted_encoderlen = 0 as size_t;
    nghttp3_qpack_read_state_reset(&raw mut (*decoder).rstate);
    nghttp3_buf_init(&raw mut (*decoder).dbuf);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_free(
    mut decoder: *mut nghttp3_qpack_decoder,
) {
    nghttp3_buf_free(&raw mut (*decoder).dbuf, (*decoder).ctx.mem);
    nghttp3_qpack_read_state_free(&raw mut (*decoder).rstate);
    qpack_context_free(&raw mut (*decoder).ctx);
}
unsafe extern "C" fn qpack_read_huffman_string(
    mut rstate: *mut nghttp3_qpack_read_state,
    mut dest: *mut nghttp3_buf,
    mut begin: *const uint8_t,
    mut end: *const uint8_t,
) -> nghttp3_ssize {
    let mut nwrite: nghttp3_ssize = 0;
    let mut len: size_t = end.offset_from(begin) as size_t;
    let mut fin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if len >= (*rstate).left as size_t {
        len = (*rstate).left as size_t;
        fin = 1 as ::core::ffi::c_int;
    }
    nwrite = nghttp3_qpack_huffman_decode(
        &raw mut (*rstate).huffman_ctx,
        (*dest).last,
        begin,
        len,
        fin,
    );
    if nwrite < 0 as nghttp3_ssize {
        return nwrite;
    }
    if nghttp3_qpack_huffman_decode_failure_state(&raw mut (*rstate).huffman_ctx) != 0 {
        return NGHTTP3_ERR_QPACK_FATAL as nghttp3_ssize;
    }
    (*dest).last = (*dest).last.offset(nwrite as isize);
    (*rstate).left = ((*rstate).left as ::core::ffi::c_ulong)
        .wrapping_sub(len as ::core::ffi::c_ulong) as uint64_t;
    return len as nghttp3_ssize;
}
unsafe extern "C" fn qpack_read_string(
    mut rstate: *mut nghttp3_qpack_read_state,
    mut dest: *mut nghttp3_buf,
    mut begin: *const uint8_t,
    mut end: *const uint8_t,
) -> nghttp3_ssize {
    let mut len: size_t = end.offset_from(begin) as size_t;
    let mut n: size_t = nghttp3_min_unsigned_long_int(
        len as ::core::ffi::c_ulong,
        (*rstate).left as ::core::ffi::c_ulong,
    ) as size_t;
    (*dest).last = nghttp3_cpymem((*dest).last, begin, n);
    (*rstate).left = ((*rstate).left as ::core::ffi::c_ulong)
        .wrapping_sub(n as ::core::ffi::c_ulong) as uint64_t;
    return n as nghttp3_ssize;
}
unsafe extern "C" fn qpack_decoder_validate_index(
    mut decoder: *const nghttp3_qpack_decoder,
    mut rstate: *const nghttp3_qpack_read_state,
) -> ::core::ffi::c_int {
    if (*rstate).dynamic != 0 {
        return if (*rstate).absidx < (*decoder).ctx.next_absidx
            && (*decoder)
                .ctx
                .next_absidx
                .wrapping_sub((*rstate).absidx)
                .wrapping_sub(1 as uint64_t)
                < nghttp3_ringbuf_len(&raw const (*decoder).ctx.dtable) as uint64_t
        {
            0 as ::core::ffi::c_int
        } else {
            NGHTTP3_ERR_QPACK_FATAL
        };
    }
    return if (*rstate).absidx
        < (::core::mem::size_of::<[nghttp3_qpack_static_header; 99]>() as uint64_t)
            .wrapping_div(
                ::core::mem::size_of::<nghttp3_qpack_static_header>() as uint64_t,
            )
    {
        0 as ::core::ffi::c_int
    } else {
        NGHTTP3_ERR_QPACK_FATAL
    };
}
unsafe extern "C" fn qpack_read_state_check_huffman(
    mut rstate: *mut nghttp3_qpack_read_state,
    b: uint8_t,
) {
    (*rstate).huffman_encoded = (b as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << (*rstate).prefix != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn qpack_read_state_terminate_name(
    mut rstate: *mut nghttp3_qpack_read_state,
) {
    *(*rstate).namebuf.last = '\0' as uint8_t;
    (*(*rstate).name).len = nghttp3_buf_len(&raw mut (*rstate).namebuf);
}
unsafe extern "C" fn qpack_read_state_terminate_value(
    mut rstate: *mut nghttp3_qpack_read_state,
) {
    *(*rstate).valuebuf.last = '\0' as uint8_t;
    (*(*rstate).value).len = nghttp3_buf_len(&raw mut (*rstate).valuebuf);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_read_encoder(
    mut decoder: *mut nghttp3_qpack_decoder,
    mut src: *const uint8_t,
    mut srclen: size_t,
) -> nghttp3_ssize {
    let mut p: *const uint8_t = src;
    let mut end: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut rv: ::core::ffi::c_int = 0;
    let mut busy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mem: *const nghttp3_mem = (*decoder).ctx.mem;
    let mut nread: nghttp3_ssize = 0;
    let mut huff_declen: size_t = 0;
    let mut rfin: ::core::ffi::c_int = 0;
    if (*decoder).ctx.bad != 0 {
        return NGHTTP3_ERR_QPACK_FATAL as nghttp3_ssize;
    }
    if srclen == 0 as size_t {
        return 0 as nghttp3_ssize;
    }
    (*decoder).uninterrupted_encoderlen = (*decoder)
        .uninterrupted_encoderlen
        .wrapping_add(srclen);
    if (*decoder).uninterrupted_encoderlen > NGHTTP3_QPACK_MAX_ENCODERLEN as size_t {
        return NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR as nghttp3_ssize;
    }
    end = src.offset(srclen as isize);
    '_fail: {
        's_740: while p != end || busy != 0 {
            busy = 0 as ::core::ffi::c_int;
            'c_19268: {
                match (*decoder).state {
                    nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_OPCODE => {
                        match *p as ::core::ffi::c_int >> 5 as ::core::ffi::c_int {
                            0 => {
                                (*decoder).opcode = nghttp3_qpack_encoder_stream_opcode::NGHTTP3_QPACK_ES_OPCODE_DUPLICATE;
                                (*decoder).rstate.dynamic = 1 as ::core::ffi::c_int;
                                (*decoder).rstate.prefix = 5 as size_t;
                                (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_INDEX;
                            }
                            0x1 => {
                                (*decoder).opcode = nghttp3_qpack_encoder_stream_opcode::NGHTTP3_QPACK_ES_OPCODE_SET_DTABLE_CAP;
                                (*decoder).rstate.prefix = 5 as size_t;
                                (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_INDEX;
                            }
                            0x2 | 0x3 => {
                                (*decoder).opcode = nghttp3_qpack_encoder_stream_opcode::NGHTTP3_QPACK_ES_OPCODE_INSERT;
                                (*decoder).rstate.dynamic = 0 as ::core::ffi::c_int;
                                (*decoder).rstate.prefix = 5 as size_t;
                                (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_CHECK_NAME_HUFFMAN;
                            }
                            _ => {
                                (*decoder).opcode = nghttp3_qpack_encoder_stream_opcode::NGHTTP3_QPACK_ES_OPCODE_INSERT_INDEXED;
                                (*decoder).rstate.dynamic = (*p as ::core::ffi::c_uint
                                    & 0x40 as ::core::ffi::c_uint == 0) as ::core::ffi::c_int;
                                (*decoder).rstate.prefix = 6 as size_t;
                                (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_INDEX;
                            }
                        }
                        continue 's_740;
                    }
                    nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_INDEX => {
                        nread = qpack_read_varint(
                            &raw mut rfin,
                            &raw mut (*decoder).rstate,
                            p,
                            end,
                        );
                        if nread < 0 as nghttp3_ssize {
                            '_c2rust_label: {
                                if -108 as nghttp3_ssize == nread {} else {
                                    __assert_fail(
                                        b"NGHTTP3_ERR_QPACK_FATAL == nread\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        2880 as ::core::ffi::c_uint,
                                        b"nghttp3_ssize nghttp3_qpack_decoder_read_encoder(nghttp3_qpack_decoder *, const uint8_t *, size_t)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            rv = NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR;
                            break '_fail;
                        } else {
                            p = p.offset(nread as isize);
                            if rfin == 0 {
                                return p.offset_from(src);
                            }
                            if (*decoder).opcode.0
                                == nghttp3_qpack_encoder_stream_opcode::NGHTTP3_QPACK_ES_OPCODE_SET_DTABLE_CAP
                                    .0
                            {
                                rv = nghttp3_qpack_decoder_set_max_dtable_capacity(
                                    decoder,
                                    (*decoder).rstate.left as size_t,
                                );
                                if rv != 0 as ::core::ffi::c_int {
                                    rv = NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR;
                                    break '_fail;
                                } else {
                                    (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_OPCODE;
                                    nghttp3_qpack_read_state_reset(&raw mut (*decoder).rstate);
                                    continue 's_740;
                                }
                            } else {
                                rv = nghttp3_qpack_decoder_rel2abs(
                                    decoder,
                                    &raw mut (*decoder).rstate,
                                );
                                if rv < 0 as ::core::ffi::c_int {
                                    break '_fail;
                                }
                                match (*decoder).opcode {
                                    nghttp3_qpack_encoder_stream_opcode::NGHTTP3_QPACK_ES_OPCODE_DUPLICATE => {
                                        rv = nghttp3_qpack_decoder_dtable_duplicate_add(decoder);
                                        if rv != 0 as ::core::ffi::c_int {
                                            break '_fail;
                                        }
                                        (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_OPCODE;
                                        nghttp3_qpack_read_state_reset(&raw mut (*decoder).rstate);
                                        continue 's_740;
                                    }
                                    nghttp3_qpack_encoder_stream_opcode::NGHTTP3_QPACK_ES_OPCODE_INSERT_INDEXED => {
                                        (*decoder).rstate.prefix = 7 as size_t;
                                        (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_CHECK_VALUE_HUFFMAN;
                                        continue 's_740;
                                    }
                                    _ => {
                                        nghttp3_unreachable_fail(
                                            b"nghttp3_qpack.c\0".as_ptr() as *const ::core::ffi::c_char,
                                            2934 as ::core::ffi::c_int,
                                            b"nghttp3_qpack_decoder_read_encoder\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                }
                            }
                        }
                    }
                    nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_CHECK_NAME_HUFFMAN => {
                        qpack_read_state_check_huffman(&raw mut (*decoder).rstate, *p);
                        (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_NAMELEN;
                        (*decoder).rstate.left = 0 as uint64_t;
                        (*decoder).rstate.shift = 0 as size_t;
                        break 'c_19268;
                    }
                    nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_NAMELEN => {
                        break 'c_19268;
                    }
                    nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_NAME_HUFFMAN => {
                        nread = qpack_read_huffman_string(
                            &raw mut (*decoder).rstate,
                            &raw mut (*decoder).rstate.namebuf,
                            p,
                            end,
                        );
                        if nread < 0 as nghttp3_ssize {
                            '_c2rust_label_1: {
                                if -108 as nghttp3_ssize == nread {} else {
                                    __assert_fail(
                                        b"NGHTTP3_ERR_QPACK_FATAL == nread\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        2991 as ::core::ffi::c_uint,
                                        b"nghttp3_ssize nghttp3_qpack_decoder_read_encoder(nghttp3_qpack_decoder *, const uint8_t *, size_t)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            rv = NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR;
                            break '_fail;
                        } else {
                            p = p.offset(nread as isize);
                            if (*decoder).rstate.left != 0 {
                                return p.offset_from(src);
                            }
                            qpack_read_state_terminate_name(&raw mut (*decoder).rstate);
                            (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_CHECK_VALUE_HUFFMAN;
                            (*decoder).rstate.prefix = 7 as size_t;
                            continue 's_740;
                        }
                    }
                    nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_NAME => {
                        nread = qpack_read_string(
                            &raw mut (*decoder).rstate,
                            &raw mut (*decoder).rstate.namebuf,
                            p,
                            end,
                        );
                        if nread < 0 as nghttp3_ssize {
                            rv = nread as ::core::ffi::c_int;
                            break '_fail;
                        } else {
                            p = p.offset(nread as isize);
                            if (*decoder).rstate.left != 0 {
                                return p.offset_from(src);
                            }
                            qpack_read_state_terminate_name(&raw mut (*decoder).rstate);
                            (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_CHECK_VALUE_HUFFMAN;
                            (*decoder).rstate.prefix = 7 as size_t;
                            continue 's_740;
                        }
                    }
                    nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_CHECK_VALUE_HUFFMAN => {
                        qpack_read_state_check_huffman(&raw mut (*decoder).rstate, *p);
                        (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_VALUELEN;
                        (*decoder).rstate.left = 0 as uint64_t;
                        (*decoder).rstate.shift = 0 as size_t;
                    }
                    nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_VALUELEN => {}
                    nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_VALUE_HUFFMAN => {
                        nread = qpack_read_huffman_string(
                            &raw mut (*decoder).rstate,
                            &raw mut (*decoder).rstate.valuebuf,
                            p,
                            end,
                        );
                        if nread < 0 as nghttp3_ssize {
                            '_c2rust_label_3: {
                                if -108 as nghttp3_ssize == nread {} else {
                                    __assert_fail(
                                        b"NGHTTP3_ERR_QPACK_FATAL == nread\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        3082 as ::core::ffi::c_uint,
                                        b"nghttp3_ssize nghttp3_qpack_decoder_read_encoder(nghttp3_qpack_decoder *, const uint8_t *, size_t)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            rv = NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR;
                            break '_fail;
                        } else {
                            p = p.offset(nread as isize);
                            if (*decoder).rstate.left != 0 {
                                return p.offset_from(src);
                            }
                            qpack_read_state_terminate_value(&raw mut (*decoder).rstate);
                            match (*decoder).opcode {
                                nghttp3_qpack_encoder_stream_opcode::NGHTTP3_QPACK_ES_OPCODE_INSERT_INDEXED => {
                                    rv = nghttp3_qpack_decoder_dtable_indexed_add(decoder);
                                }
                                nghttp3_qpack_encoder_stream_opcode::NGHTTP3_QPACK_ES_OPCODE_INSERT => {
                                    rv = nghttp3_qpack_decoder_dtable_literal_add(decoder);
                                }
                                _ => {
                                    nghttp3_unreachable_fail(
                                        b"nghttp3_qpack.c\0".as_ptr() as *const ::core::ffi::c_char,
                                        3103 as ::core::ffi::c_int,
                                        b"nghttp3_qpack_decoder_read_encoder\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                }
                            }
                            if rv != 0 as ::core::ffi::c_int {
                                break '_fail;
                            }
                            (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_OPCODE;
                            nghttp3_qpack_read_state_reset(&raw mut (*decoder).rstate);
                            continue 's_740;
                        }
                    }
                    nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_VALUE => {
                        nread = qpack_read_string(
                            &raw mut (*decoder).rstate,
                            &raw mut (*decoder).rstate.valuebuf,
                            p,
                            end,
                        );
                        if nread < 0 as nghttp3_ssize {
                            rv = nread as ::core::ffi::c_int;
                            break '_fail;
                        } else {
                            p = p.offset(nread as isize);
                            if (*decoder).rstate.left != 0 {
                                return p.offset_from(src);
                            }
                            qpack_read_state_terminate_value(&raw mut (*decoder).rstate);
                            match (*decoder).opcode {
                                nghttp3_qpack_encoder_stream_opcode::NGHTTP3_QPACK_ES_OPCODE_INSERT_INDEXED => {
                                    rv = nghttp3_qpack_decoder_dtable_indexed_add(decoder);
                                }
                                nghttp3_qpack_encoder_stream_opcode::NGHTTP3_QPACK_ES_OPCODE_INSERT => {
                                    rv = nghttp3_qpack_decoder_dtable_literal_add(decoder);
                                }
                                _ => {
                                    nghttp3_unreachable_fail(
                                        b"nghttp3_qpack.c\0".as_ptr() as *const ::core::ffi::c_char,
                                        3136 as ::core::ffi::c_int,
                                        b"nghttp3_qpack_decoder_read_encoder\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                }
                            }
                            if rv != 0 as ::core::ffi::c_int {
                                break '_fail;
                            }
                            (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_OPCODE;
                            nghttp3_qpack_read_state_reset(&raw mut (*decoder).rstate);
                            continue 's_740;
                        }
                    }
                    _ => {
                        continue 's_740;
                    }
                }
                nread = qpack_read_varint(
                    &raw mut rfin,
                    &raw mut (*decoder).rstate,
                    p,
                    end,
                );
                if nread < 0 as nghttp3_ssize {
                    '_c2rust_label_2: {
                        if -108 as nghttp3_ssize == nread {} else {
                            __assert_fail(
                                b"NGHTTP3_ERR_QPACK_FATAL == nread\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                3035 as ::core::ffi::c_uint,
                                b"nghttp3_ssize nghttp3_qpack_decoder_read_encoder(nghttp3_qpack_decoder *, const uint8_t *, size_t)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    rv = NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR;
                    break '_fail;
                } else {
                    p = p.offset(nread as isize);
                    if rfin == 0 {
                        return p.offset_from(src);
                    }
                    if (*decoder).rstate.left > NGHTTP3_QPACK_MAX_VALUELEN as uint64_t {
                        rv = NGHTTP3_ERR_QPACK_HEADER_TOO_LARGE;
                        break '_fail;
                    } else {
                        if (*decoder).rstate.huffman_encoded != 0 {
                            huff_declen = nghttp3_qpack_huffman_estimate_decode_length(
                                (*decoder).rstate.left as size_t,
                            );
                            if huff_declen > NGHTTP3_QPACK_MAX_VALUELEN as size_t {
                                rv = NGHTTP3_ERR_QPACK_HEADER_TOO_LARGE;
                                break '_fail;
                            } else {
                                (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_VALUE_HUFFMAN;
                                nghttp3_qpack_huffman_decode_context_init(
                                    &raw mut (*decoder).rstate.huffman_ctx,
                                );
                                rv = nghttp3_rcbuf_new(
                                    &raw mut (*decoder).rstate.value,
                                    huff_declen.wrapping_add(1 as size_t),
                                    mem,
                                );
                            }
                        } else {
                            (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_VALUE;
                            rv = nghttp3_rcbuf_new(
                                &raw mut (*decoder).rstate.value,
                                ((*decoder).rstate.left as size_t)
                                    .wrapping_add(1 as size_t),
                                mem,
                            );
                        }
                        if rv != 0 as ::core::ffi::c_int {
                            break '_fail;
                        }
                        nghttp3_buf_wrap_init(
                            &raw mut (*decoder).rstate.valuebuf,
                            (*(*decoder).rstate.value).base,
                            (*(*decoder).rstate.value).len,
                        );
                        busy = 1 as ::core::ffi::c_int;
                        continue 's_740;
                    }
                }
            }
            nread = qpack_read_varint(&raw mut rfin, &raw mut (*decoder).rstate, p, end);
            if nread < 0 as nghttp3_ssize {
                '_c2rust_label_0: {
                    if -108 as nghttp3_ssize == nread {} else {
                        __assert_fail(
                            b"NGHTTP3_ERR_QPACK_FATAL == nread\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            2947 as ::core::ffi::c_uint,
                            b"nghttp3_ssize nghttp3_qpack_decoder_read_encoder(nghttp3_qpack_decoder *, const uint8_t *, size_t)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                rv = NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR;
                break '_fail;
            } else {
                p = p.offset(nread as isize);
                if rfin == 0 {
                    return p.offset_from(src);
                }
                if (*decoder).rstate.left > NGHTTP3_QPACK_MAX_NAMELEN as uint64_t {
                    rv = NGHTTP3_ERR_QPACK_HEADER_TOO_LARGE;
                    break '_fail;
                } else {
                    if (*decoder).rstate.huffman_encoded != 0 {
                        huff_declen = nghttp3_qpack_huffman_estimate_decode_length(
                            (*decoder).rstate.left as size_t,
                        );
                        if huff_declen > NGHTTP3_QPACK_MAX_NAMELEN as size_t {
                            rv = NGHTTP3_ERR_QPACK_HEADER_TOO_LARGE;
                            break '_fail;
                        } else {
                            (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_NAME_HUFFMAN;
                            nghttp3_qpack_huffman_decode_context_init(
                                &raw mut (*decoder).rstate.huffman_ctx,
                            );
                            rv = nghttp3_rcbuf_new(
                                &raw mut (*decoder).rstate.name,
                                huff_declen.wrapping_add(1 as size_t),
                                mem,
                            );
                        }
                    } else {
                        (*decoder).state = nghttp3_qpack_encoder_stream_state::NGHTTP3_QPACK_ES_STATE_READ_NAME;
                        rv = nghttp3_rcbuf_new(
                            &raw mut (*decoder).rstate.name,
                            ((*decoder).rstate.left as size_t).wrapping_add(1 as size_t),
                            mem,
                        );
                    }
                    if rv != 0 as ::core::ffi::c_int {
                        break '_fail;
                    }
                    nghttp3_buf_wrap_init(
                        &raw mut (*decoder).rstate.namebuf,
                        (*(*decoder).rstate.name).base,
                        (*(*decoder).rstate.name).len,
                    );
                }
            }
        }
        return p.offset_from(src);
    }
    (*decoder).ctx.bad = 1 as uint8_t;
    return rv as nghttp3_ssize;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_set_max_dtable_capacity(
    mut decoder: *mut nghttp3_qpack_decoder,
    mut max_dtable_capacity: size_t,
) -> ::core::ffi::c_int {
    let mut ent: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    let mut i: size_t = 0;
    let mut ctx: *mut nghttp3_qpack_context = &raw mut (*decoder).ctx;
    let mut mem: *const nghttp3_mem = (*ctx).mem;
    if max_dtable_capacity > (*decoder).ctx.hard_max_dtable_capacity {
        return NGHTTP3_ERR_INVALID_ARGUMENT;
    }
    (*ctx).max_dtable_capacity = max_dtable_capacity;
    while (*ctx).dtable_size > max_dtable_capacity {
        i = nghttp3_ringbuf_len(&raw mut (*ctx).dtable);
        '_c2rust_label: {
            if i != 0 {} else {
                __assert_fail(
                    b"i\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    3170 as ::core::ffi::c_uint,
                    b"int nghttp3_qpack_decoder_set_max_dtable_capacity(nghttp3_qpack_decoder *, size_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        ent = *(nghttp3_ringbuf_get(&raw mut (*ctx).dtable, i.wrapping_sub(1 as size_t))
            as *mut *mut nghttp3_qpack_entry);
        (*ctx).dtable_size = (*ctx)
            .dtable_size
            .wrapping_sub(table_space((*(*ent).nv.name).len, (*(*ent).nv.value).len));
        nghttp3_ringbuf_pop_back(&raw mut (*ctx).dtable);
        nghttp3_qpack_entry_free(ent);
        nghttp3_mem_free(mem, ent as *mut ::core::ffi::c_void);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_dtable_indexed_add(
    mut decoder: *mut nghttp3_qpack_decoder,
) -> ::core::ffi::c_int {
    if (*decoder).rstate.dynamic != 0 {
        return nghttp3_qpack_decoder_dtable_dynamic_add(decoder);
    }
    return nghttp3_qpack_decoder_dtable_static_add(decoder);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_dtable_static_add(
    mut decoder: *mut nghttp3_qpack_decoder,
) -> ::core::ffi::c_int {
    let mut qnv: nghttp3_qpack_nv = nghttp3_qpack_nv {
        name: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        value: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        token: 0,
        flags: 0,
    };
    let mut rv: ::core::ffi::c_int = 0;
    let mut shd: *const nghttp3_qpack_static_header = ::core::ptr::null::<
        nghttp3_qpack_static_header,
    >();
    shd = (&raw mut stable as *mut nghttp3_qpack_static_header)
        .offset((*decoder).rstate.absidx as isize);
    if table_space((*shd).name.len, (*(*decoder).rstate.value).len)
        > (*decoder).ctx.max_dtable_capacity
    {
        return NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR;
    }
    qnv.name = &raw const (*shd).name as *mut nghttp3_rcbuf;
    qnv.value = (*decoder).rstate.value;
    qnv.token = (*shd).token;
    qnv.flags = NGHTTP3_NV_FLAG_NONE as uint8_t;
    rv = nghttp3_qpack_context_dtable_add(
        &raw mut (*decoder).ctx,
        &raw mut qnv,
        ::core::ptr::null_mut::<nghttp3_qpack_map>(),
        0 as uint32_t,
    );
    nghttp3_rcbuf_decref(qnv.value);
    (*decoder).rstate.value = ::core::ptr::null_mut::<nghttp3_rcbuf>();
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_dtable_dynamic_add(
    mut decoder: *mut nghttp3_qpack_decoder,
) -> ::core::ffi::c_int {
    let mut qnv: nghttp3_qpack_nv = nghttp3_qpack_nv {
        name: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        value: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        token: 0,
        flags: 0,
    };
    let mut rv: ::core::ffi::c_int = 0;
    let mut ent: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    ent = nghttp3_qpack_context_dtable_get(
        &raw mut (*decoder).ctx,
        (*decoder).rstate.absidx,
    );
    if table_space((*(*ent).nv.name).len, (*(*decoder).rstate.value).len)
        > (*decoder).ctx.max_dtable_capacity
    {
        return NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR;
    }
    qnv.name = (*ent).nv.name;
    qnv.value = (*decoder).rstate.value;
    qnv.token = (*ent).nv.token;
    qnv.flags = NGHTTP3_NV_FLAG_NONE as uint8_t;
    nghttp3_rcbuf_incref(qnv.name);
    rv = nghttp3_qpack_context_dtable_add(
        &raw mut (*decoder).ctx,
        &raw mut qnv,
        ::core::ptr::null_mut::<nghttp3_qpack_map>(),
        0 as uint32_t,
    );
    nghttp3_rcbuf_decref(qnv.value);
    (*decoder).rstate.value = ::core::ptr::null_mut::<nghttp3_rcbuf>();
    nghttp3_rcbuf_decref(qnv.name);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_dtable_duplicate_add(
    mut decoder: *mut nghttp3_qpack_decoder,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    let mut ent: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    let mut qnv: nghttp3_qpack_nv = nghttp3_qpack_nv {
        name: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        value: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        token: 0,
        flags: 0,
    };
    ent = nghttp3_qpack_context_dtable_get(
        &raw mut (*decoder).ctx,
        (*decoder).rstate.absidx,
    );
    if table_space((*(*ent).nv.name).len, (*(*ent).nv.value).len)
        > (*decoder).ctx.max_dtable_capacity
    {
        return NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR;
    }
    qnv = (*ent).nv;
    nghttp3_rcbuf_incref(qnv.name);
    nghttp3_rcbuf_incref(qnv.value);
    rv = nghttp3_qpack_context_dtable_add(
        &raw mut (*decoder).ctx,
        &raw mut qnv,
        ::core::ptr::null_mut::<nghttp3_qpack_map>(),
        0 as uint32_t,
    );
    nghttp3_rcbuf_decref(qnv.value);
    nghttp3_rcbuf_decref(qnv.name);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_dtable_literal_add(
    mut decoder: *mut nghttp3_qpack_decoder,
) -> ::core::ffi::c_int {
    let mut qnv: nghttp3_qpack_nv = nghttp3_qpack_nv {
        name: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        value: ::core::ptr::null_mut::<nghttp3_rcbuf>(),
        token: 0,
        flags: 0,
    };
    let mut rv: ::core::ffi::c_int = 0;
    if table_space((*(*decoder).rstate.name).len, (*(*decoder).rstate.value).len)
        > (*decoder).ctx.max_dtable_capacity
    {
        return NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR;
    }
    qnv.name = (*decoder).rstate.name;
    qnv.value = (*decoder).rstate.value;
    qnv.token = qpack_lookup_token((*qnv.name).base, (*qnv.name).len);
    qnv.flags = NGHTTP3_NV_FLAG_NONE as uint8_t;
    rv = nghttp3_qpack_context_dtable_add(
        &raw mut (*decoder).ctx,
        &raw mut qnv,
        ::core::ptr::null_mut::<nghttp3_qpack_map>(),
        0 as uint32_t,
    );
    nghttp3_rcbuf_decref(qnv.value);
    (*decoder).rstate.value = ::core::ptr::null_mut::<nghttp3_rcbuf>();
    nghttp3_rcbuf_decref(qnv.name);
    (*decoder).rstate.name = ::core::ptr::null_mut::<nghttp3_rcbuf>();
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_set_max_concurrent_streams(
    mut decoder: *mut nghttp3_qpack_decoder,
    mut max_concurrent_streams: size_t,
) {
    (*decoder).max_concurrent_streams = nghttp3_max_unsigned_long_int(
        (*decoder).max_concurrent_streams as ::core::ffi::c_ulong,
        max_concurrent_streams as ::core::ffi::c_ulong,
    ) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_stream_context_init(
    mut sctx: *mut nghttp3_qpack_stream_context,
    mut stream_id: int64_t,
    mut mem: *const nghttp3_mem,
) {
    nghttp3_qpack_read_state_reset(&raw mut (*sctx).rstate);
    (*sctx).mem = mem;
    (*sctx).rstate.prefix = 8 as size_t;
    (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_RICNT;
    (*sctx).opcode = nghttp3_qpack_request_stream_opcode(
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*sctx).stream_id = stream_id;
    (*sctx).ricnt = 0 as uint64_t;
    (*sctx).dbase_sign = 0 as ::core::ffi::c_int;
    (*sctx).base = 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_stream_context_free(
    mut sctx: *mut nghttp3_qpack_stream_context,
) {
    nghttp3_qpack_read_state_free(&raw mut (*sctx).rstate);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_stream_context_reset(
    mut sctx: *mut nghttp3_qpack_stream_context,
) {
    nghttp3_qpack_stream_context_init(sctx, (*sctx).stream_id, (*sctx).mem);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_stream_context_get_ricnt(
    mut sctx: *mut nghttp3_qpack_stream_context,
) -> uint64_t {
    return nghttp3_qpack_stream_context_get_ricnt2(sctx);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_stream_context_get_ricnt2(
    mut sctx: *const nghttp3_qpack_stream_context,
) -> uint64_t {
    return (*sctx).ricnt;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_read_request(
    mut decoder: *mut nghttp3_qpack_decoder,
    mut sctx: *mut nghttp3_qpack_stream_context,
    mut nv: *mut nghttp3_qpack_nv,
    mut pflags: *mut uint8_t,
    mut src: *const uint8_t,
    mut srclen: size_t,
    mut fin: ::core::ffi::c_int,
) -> nghttp3_ssize {
    let mut p: *const uint8_t = src;
    let mut end: *const uint8_t = if !src.is_null() {
        src.offset(srclen as isize)
    } else {
        src
    };
    let mut rv: ::core::ffi::c_int = 0;
    let mut busy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nread: nghttp3_ssize = 0;
    let mut rfin: ::core::ffi::c_int = 0;
    let mut mem: *const nghttp3_mem = (*decoder).ctx.mem;
    let mut huff_declen: size_t = 0;
    if (*decoder).ctx.bad != 0 {
        return NGHTTP3_ERR_QPACK_FATAL as nghttp3_ssize;
    }
    *pflags = NGHTTP3_QPACK_DECODE_FLAG_NONE as uint8_t;
    '_fail: {
        '_almost_ok: while p != end || busy != 0 {
            busy = 0 as ::core::ffi::c_int;
            'c_22206: {
                'c_22231: {
                    match (*sctx).state {
                        nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_RICNT => {
                            nread = qpack_read_varint(
                                &raw mut rfin,
                                &raw mut (*sctx).rstate,
                                p,
                                end,
                            );
                            if nread < 0 as nghttp3_ssize {
                                '_c2rust_label: {
                                    if -108 as nghttp3_ssize == nread {} else {
                                        __assert_fail(
                                            b"NGHTTP3_ERR_QPACK_FATAL == nread\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            3368 as ::core::ffi::c_uint,
                                            b"nghttp3_ssize nghttp3_qpack_decoder_read_request(nghttp3_qpack_decoder *, nghttp3_qpack_stream_context *, nghttp3_qpack_nv *, uint8_t *, const uint8_t *, size_t, int)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                        );
                                    }
                                };
                                rv = NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
                                break '_fail;
                            } else {
                                p = p.offset(nread as isize);
                                if rfin == 0 {
                                    break '_almost_ok;
                                }
                                rv = nghttp3_qpack_decoder_reconstruct_ricnt(
                                    decoder,
                                    &raw mut (*sctx).ricnt,
                                    (*sctx).rstate.left,
                                );
                                if rv != 0 as ::core::ffi::c_int {
                                    break '_fail;
                                }
                                (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_DBASE_SIGN;
                                continue '_almost_ok;
                            }
                        }
                        nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_DBASE_SIGN => {
                            if *p as ::core::ffi::c_uint & 0x80 as ::core::ffi::c_uint
                                != 0
                            {
                                (*sctx).dbase_sign = 1 as ::core::ffi::c_int;
                            }
                            (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_DBASE;
                            (*sctx).rstate.left = 0 as uint64_t;
                            (*sctx).rstate.prefix = 7 as size_t;
                            (*sctx).rstate.shift = 0 as size_t;
                            break 'c_22206;
                        }
                        nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_DBASE => {
                            break 'c_22206;
                        }
                        nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_OPCODE => {
                            '_c2rust_label_1: {
                                if (*sctx).rstate.left == 0 as uint64_t {} else {
                                    __assert_fail(
                                        b"sctx->rstate.left == 0\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        3436 as ::core::ffi::c_uint,
                                        b"nghttp3_ssize nghttp3_qpack_decoder_read_request(nghttp3_qpack_decoder *, nghttp3_qpack_stream_context *, nghttp3_qpack_nv *, uint8_t *, const uint8_t *, size_t, int)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            '_c2rust_label_2: {
                                if (*sctx).rstate.shift == 0 as size_t {} else {
                                    __assert_fail(
                                        b"sctx->rstate.shift == 0\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        3437 as ::core::ffi::c_uint,
                                        b"nghttp3_ssize nghttp3_qpack_decoder_read_request(nghttp3_qpack_decoder *, nghttp3_qpack_stream_context *, nghttp3_qpack_nv *, uint8_t *, const uint8_t *, size_t, int)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            match *p as ::core::ffi::c_int >> 4 as ::core::ffi::c_int {
                                0 => {
                                    (*sctx).opcode = nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_INDEXED_NAME_PB;
                                    (*sctx).rstate.never = (*p as ::core::ffi::c_uint
                                        & 0x8 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                                    (*sctx).rstate.dynamic = 1 as ::core::ffi::c_int;
                                    (*sctx).rstate.prefix = 3 as size_t;
                                    (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_INDEX;
                                }
                                0x1 => {
                                    (*sctx).opcode = nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_INDEXED_PB;
                                    (*sctx).rstate.dynamic = 1 as ::core::ffi::c_int;
                                    (*sctx).rstate.prefix = 4 as size_t;
                                    (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_INDEX;
                                }
                                0x2 | 0x3 => {
                                    (*sctx).opcode = nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_LITERAL;
                                    (*sctx).rstate.never = (*p as ::core::ffi::c_uint
                                        & 0x10 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                                    (*sctx).rstate.dynamic = 0 as ::core::ffi::c_int;
                                    (*sctx).rstate.prefix = 3 as size_t;
                                    (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_CHECK_NAME_HUFFMAN;
                                }
                                0x4 | 0x5 | 0x6 | 0x7 => {
                                    (*sctx).opcode = nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_INDEXED_NAME;
                                    (*sctx).rstate.never = (*p as ::core::ffi::c_uint
                                        & 0x20 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                                    (*sctx).rstate.dynamic = (*p as ::core::ffi::c_uint
                                        & 0x10 as ::core::ffi::c_uint == 0) as ::core::ffi::c_int;
                                    (*sctx).rstate.prefix = 4 as size_t;
                                    (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_INDEX;
                                }
                                _ => {
                                    (*sctx).opcode = nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_INDEXED;
                                    (*sctx).rstate.dynamic = (*p as ::core::ffi::c_uint
                                        & 0x40 as ::core::ffi::c_uint == 0) as ::core::ffi::c_int;
                                    (*sctx).rstate.prefix = 6 as size_t;
                                    (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_INDEX;
                                }
                            }
                            continue '_almost_ok;
                        }
                        nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_INDEX => {
                            nread = qpack_read_varint(
                                &raw mut rfin,
                                &raw mut (*sctx).rstate,
                                p,
                                end,
                            );
                            if nread < 0 as nghttp3_ssize {
                                '_c2rust_label_3: {
                                    if -108 as nghttp3_ssize == nread {} else {
                                        __assert_fail(
                                            b"NGHTTP3_ERR_QPACK_FATAL == nread\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            3489 as ::core::ffi::c_uint,
                                            b"nghttp3_ssize nghttp3_qpack_decoder_read_request(nghttp3_qpack_decoder *, nghttp3_qpack_stream_context *, nghttp3_qpack_nv *, uint8_t *, const uint8_t *, size_t, int)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                        );
                                    }
                                };
                                rv = NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
                                break '_fail;
                            } else {
                                p = p.offset(nread as isize);
                                if rfin == 0 {
                                    break '_almost_ok;
                                }
                                match (*sctx).opcode {
                                    nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_INDEXED => {
                                        rv = nghttp3_qpack_decoder_brel2abs(decoder, sctx);
                                        if rv != 0 as ::core::ffi::c_int {
                                            break '_fail;
                                        }
                                        nghttp3_qpack_decoder_emit_indexed(decoder, sctx, nv);
                                        *pflags = (*pflags as ::core::ffi::c_uint
                                            | NGHTTP3_QPACK_DECODE_FLAG_EMIT) as uint8_t;
                                        (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_OPCODE;
                                        nghttp3_qpack_read_state_reset(&raw mut (*sctx).rstate);
                                        return p.offset_from(src);
                                    }
                                    nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_INDEXED_PB => {
                                        rv = nghttp3_qpack_decoder_pbrel2abs(decoder, sctx);
                                        if rv != 0 as ::core::ffi::c_int {
                                            break '_fail;
                                        }
                                        nghttp3_qpack_decoder_emit_indexed(decoder, sctx, nv);
                                        *pflags = (*pflags as ::core::ffi::c_uint
                                            | NGHTTP3_QPACK_DECODE_FLAG_EMIT) as uint8_t;
                                        (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_OPCODE;
                                        nghttp3_qpack_read_state_reset(&raw mut (*sctx).rstate);
                                        return p.offset_from(src);
                                    }
                                    nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_INDEXED_NAME => {
                                        rv = nghttp3_qpack_decoder_brel2abs(decoder, sctx);
                                        if rv != 0 as ::core::ffi::c_int {
                                            break '_fail;
                                        }
                                        (*sctx).rstate.prefix = 7 as size_t;
                                        (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_CHECK_VALUE_HUFFMAN;
                                        continue '_almost_ok;
                                    }
                                    nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_INDEXED_NAME_PB => {
                                        rv = nghttp3_qpack_decoder_pbrel2abs(decoder, sctx);
                                        if rv != 0 as ::core::ffi::c_int {
                                            break '_fail;
                                        }
                                        (*sctx).rstate.prefix = 7 as size_t;
                                        (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_CHECK_VALUE_HUFFMAN;
                                        continue '_almost_ok;
                                    }
                                    _ => {
                                        nghttp3_unreachable_fail(
                                            b"nghttp3_qpack.c\0".as_ptr() as *const ::core::ffi::c_char,
                                            3542 as ::core::ffi::c_int,
                                            b"nghttp3_qpack_decoder_read_request\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                }
                            }
                        }
                        nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_CHECK_NAME_HUFFMAN => {
                            qpack_read_state_check_huffman(&raw mut (*sctx).rstate, *p);
                            (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_NAMELEN;
                            (*sctx).rstate.left = 0 as uint64_t;
                            (*sctx).rstate.shift = 0 as size_t;
                            break 'c_22231;
                        }
                        nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_NAMELEN => {
                            break 'c_22231;
                        }
                        nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_NAME_HUFFMAN => {
                            nread = qpack_read_huffman_string(
                                &raw mut (*sctx).rstate,
                                &raw mut (*sctx).rstate.namebuf,
                                p,
                                end,
                            );
                            if nread < 0 as nghttp3_ssize {
                                '_c2rust_label_5: {
                                    if -108 as nghttp3_ssize == nread {} else {
                                        __assert_fail(
                                            b"NGHTTP3_ERR_QPACK_FATAL == nread\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            3597 as ::core::ffi::c_uint,
                                            b"nghttp3_ssize nghttp3_qpack_decoder_read_request(nghttp3_qpack_decoder *, nghttp3_qpack_stream_context *, nghttp3_qpack_nv *, uint8_t *, const uint8_t *, size_t, int)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                        );
                                    }
                                };
                                rv = NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
                                break '_fail;
                            } else {
                                p = p.offset(nread as isize);
                                if (*sctx).rstate.left != 0 {
                                    break '_almost_ok;
                                }
                                qpack_read_state_terminate_name(&raw mut (*sctx).rstate);
                                (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_CHECK_VALUE_HUFFMAN;
                                (*sctx).rstate.prefix = 7 as size_t;
                                continue '_almost_ok;
                            }
                        }
                        nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_NAME => {
                            nread = qpack_read_string(
                                &raw mut (*sctx).rstate,
                                &raw mut (*sctx).rstate.namebuf,
                                p,
                                end,
                            );
                            if nread < 0 as nghttp3_ssize {
                                rv = nread as ::core::ffi::c_int;
                                break '_fail;
                            } else {
                                p = p.offset(nread as isize);
                                if (*sctx).rstate.left != 0 {
                                    break '_almost_ok;
                                }
                                qpack_read_state_terminate_name(&raw mut (*sctx).rstate);
                                (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_CHECK_VALUE_HUFFMAN;
                                (*sctx).rstate.prefix = 7 as size_t;
                                continue '_almost_ok;
                            }
                        }
                        nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_CHECK_VALUE_HUFFMAN => {
                            qpack_read_state_check_huffman(&raw mut (*sctx).rstate, *p);
                            (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_VALUELEN;
                            (*sctx).rstate.left = 0 as uint64_t;
                            (*sctx).rstate.shift = 0 as size_t;
                        }
                        nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_VALUELEN => {}
                        nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_VALUE_HUFFMAN => {
                            nread = qpack_read_huffman_string(
                                &raw mut (*sctx).rstate,
                                &raw mut (*sctx).rstate.valuebuf,
                                p,
                                end,
                            );
                            if nread < 0 as nghttp3_ssize {
                                '_c2rust_label_7: {
                                    if -108 as nghttp3_ssize == nread {} else {
                                        __assert_fail(
                                            b"NGHTTP3_ERR_QPACK_FATAL == nread\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            3686 as ::core::ffi::c_uint,
                                            b"nghttp3_ssize nghttp3_qpack_decoder_read_request(nghttp3_qpack_decoder *, nghttp3_qpack_stream_context *, nghttp3_qpack_nv *, uint8_t *, const uint8_t *, size_t, int)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                        );
                                    }
                                };
                                rv = NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
                                break '_fail;
                            } else {
                                p = p.offset(nread as isize);
                                if (*sctx).rstate.left != 0 {
                                    break '_almost_ok;
                                }
                                qpack_read_state_terminate_value(&raw mut (*sctx).rstate);
                                match (*sctx).opcode {
                                    nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_INDEXED_NAME
                                    | nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_INDEXED_NAME_PB => {
                                        rv = nghttp3_qpack_decoder_emit_indexed_name(
                                            decoder,
                                            sctx,
                                            nv,
                                        );
                                        if rv != 0 as ::core::ffi::c_int {
                                            break '_fail;
                                        }
                                    }
                                    nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_LITERAL => {
                                        nghttp3_qpack_decoder_emit_literal(decoder, sctx, nv);
                                    }
                                    _ => {
                                        nghttp3_unreachable_fail(
                                            b"nghttp3_qpack.c\0".as_ptr() as *const ::core::ffi::c_char,
                                            3712 as ::core::ffi::c_int,
                                            b"nghttp3_qpack_decoder_read_request\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                }
                                *pflags = (*pflags as ::core::ffi::c_uint
                                    | NGHTTP3_QPACK_DECODE_FLAG_EMIT) as uint8_t;
                                (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_OPCODE;
                                nghttp3_qpack_read_state_reset(&raw mut (*sctx).rstate);
                                return p.offset_from(src);
                            }
                        }
                        nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_VALUE => {
                            nread = qpack_read_string(
                                &raw mut (*sctx).rstate,
                                &raw mut (*sctx).rstate.valuebuf,
                                p,
                                end,
                            );
                            if nread < 0 as nghttp3_ssize {
                                rv = nread as ::core::ffi::c_int;
                                break '_fail;
                            } else {
                                p = p.offset(nread as isize);
                                if (*sctx).rstate.left != 0 {
                                    break '_almost_ok;
                                }
                                qpack_read_state_terminate_value(&raw mut (*sctx).rstate);
                                match (*sctx).opcode {
                                    nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_INDEXED_NAME
                                    | nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_INDEXED_NAME_PB => {
                                        rv = nghttp3_qpack_decoder_emit_indexed_name(
                                            decoder,
                                            sctx,
                                            nv,
                                        );
                                        if rv != 0 as ::core::ffi::c_int {
                                            break '_fail;
                                        }
                                    }
                                    nghttp3_qpack_request_stream_opcode::NGHTTP3_QPACK_RS_OPCODE_LITERAL => {
                                        nghttp3_qpack_decoder_emit_literal(decoder, sctx, nv);
                                    }
                                    _ => {
                                        nghttp3_unreachable_fail(
                                            b"nghttp3_qpack.c\0".as_ptr() as *const ::core::ffi::c_char,
                                            3749 as ::core::ffi::c_int,
                                            b"nghttp3_qpack_decoder_read_request\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                }
                                *pflags = (*pflags as ::core::ffi::c_uint
                                    | NGHTTP3_QPACK_DECODE_FLAG_EMIT) as uint8_t;
                                (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_OPCODE;
                                nghttp3_qpack_read_state_reset(&raw mut (*sctx).rstate);
                                return p.offset_from(src);
                            }
                        }
                        nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_BLOCKED => {
                            if (*sctx).ricnt > (*decoder).ctx.next_absidx {
                                *pflags = (*pflags as ::core::ffi::c_uint
                                    | NGHTTP3_QPACK_DECODE_FLAG_BLOCKED) as uint8_t;
                                return p.offset_from(src);
                            }
                            (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_OPCODE;
                            nghttp3_qpack_read_state_reset(&raw mut (*sctx).rstate);
                            continue '_almost_ok;
                        }
                        _ => {
                            continue '_almost_ok;
                        }
                    }
                    nread = qpack_read_varint(
                        &raw mut rfin,
                        &raw mut (*sctx).rstate,
                        p,
                        end,
                    );
                    if nread < 0 as nghttp3_ssize {
                        '_c2rust_label_6: {
                            if -108 as nghttp3_ssize == nread {} else {
                                __assert_fail(
                                    b"NGHTTP3_ERR_QPACK_FATAL == nread\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    3640 as ::core::ffi::c_uint,
                                    b"nghttp3_ssize nghttp3_qpack_decoder_read_request(nghttp3_qpack_decoder *, nghttp3_qpack_stream_context *, nghttp3_qpack_nv *, uint8_t *, const uint8_t *, size_t, int)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                );
                            }
                        };
                        rv = NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
                        break '_fail;
                    } else {
                        p = p.offset(nread as isize);
                        if rfin == 0 {
                            break '_almost_ok;
                        }
                        if (*sctx).rstate.left > NGHTTP3_QPACK_MAX_VALUELEN as uint64_t {
                            rv = NGHTTP3_ERR_QPACK_HEADER_TOO_LARGE;
                            break '_fail;
                        } else {
                            if (*sctx).rstate.huffman_encoded != 0 {
                                huff_declen = nghttp3_qpack_huffman_estimate_decode_length(
                                    (*sctx).rstate.left as size_t,
                                );
                                if huff_declen > NGHTTP3_QPACK_MAX_VALUELEN as size_t {
                                    rv = NGHTTP3_ERR_QPACK_HEADER_TOO_LARGE;
                                    break '_fail;
                                } else {
                                    (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_VALUE_HUFFMAN;
                                    nghttp3_qpack_huffman_decode_context_init(
                                        &raw mut (*sctx).rstate.huffman_ctx,
                                    );
                                    rv = nghttp3_rcbuf_new(
                                        &raw mut (*sctx).rstate.value,
                                        huff_declen.wrapping_add(1 as size_t),
                                        mem,
                                    );
                                }
                            } else {
                                (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_VALUE;
                                rv = nghttp3_rcbuf_new(
                                    &raw mut (*sctx).rstate.value,
                                    ((*sctx).rstate.left as size_t).wrapping_add(1 as size_t),
                                    mem,
                                );
                            }
                            if rv != 0 as ::core::ffi::c_int {
                                break '_fail;
                            }
                            nghttp3_buf_wrap_init(
                                &raw mut (*sctx).rstate.valuebuf,
                                (*(*sctx).rstate.value).base,
                                (*(*sctx).rstate.value).len,
                            );
                            busy = 1 as ::core::ffi::c_int;
                            continue '_almost_ok;
                        }
                    }
                }
                nread = qpack_read_varint(
                    &raw mut rfin,
                    &raw mut (*sctx).rstate,
                    p,
                    end,
                );
                if nread < 0 as nghttp3_ssize {
                    '_c2rust_label_4: {
                        if -108 as nghttp3_ssize == nread {} else {
                            __assert_fail(
                                b"NGHTTP3_ERR_QPACK_FATAL == nread\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                3554 as ::core::ffi::c_uint,
                                b"nghttp3_ssize nghttp3_qpack_decoder_read_request(nghttp3_qpack_decoder *, nghttp3_qpack_stream_context *, nghttp3_qpack_nv *, uint8_t *, const uint8_t *, size_t, int)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    rv = NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
                    break '_fail;
                } else {
                    p = p.offset(nread as isize);
                    if rfin == 0 {
                        break '_almost_ok;
                    }
                    if (*sctx).rstate.left > NGHTTP3_QPACK_MAX_NAMELEN as uint64_t {
                        rv = NGHTTP3_ERR_QPACK_HEADER_TOO_LARGE;
                        break '_fail;
                    } else {
                        if (*sctx).rstate.huffman_encoded != 0 {
                            huff_declen = nghttp3_qpack_huffman_estimate_decode_length(
                                (*sctx).rstate.left as size_t,
                            );
                            if huff_declen > NGHTTP3_QPACK_MAX_NAMELEN as size_t {
                                rv = NGHTTP3_ERR_QPACK_HEADER_TOO_LARGE;
                                break '_fail;
                            } else {
                                (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_NAME_HUFFMAN;
                                nghttp3_qpack_huffman_decode_context_init(
                                    &raw mut (*sctx).rstate.huffman_ctx,
                                );
                                rv = nghttp3_rcbuf_new(
                                    &raw mut (*sctx).rstate.name,
                                    huff_declen.wrapping_add(1 as size_t),
                                    mem,
                                );
                            }
                        } else {
                            (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_READ_NAME;
                            rv = nghttp3_rcbuf_new(
                                &raw mut (*sctx).rstate.name,
                                ((*sctx).rstate.left as size_t).wrapping_add(1 as size_t),
                                mem,
                            );
                        }
                        if rv != 0 as ::core::ffi::c_int {
                            break '_fail;
                        }
                        nghttp3_buf_wrap_init(
                            &raw mut (*sctx).rstate.namebuf,
                            (*(*sctx).rstate.name).base,
                            (*(*sctx).rstate.name).len,
                        );
                        continue '_almost_ok;
                    }
                }
            }
            nread = qpack_read_varint(&raw mut rfin, &raw mut (*sctx).rstate, p, end);
            if nread < 0 as nghttp3_ssize {
                '_c2rust_label_0: {
                    if -108 as nghttp3_ssize == nread {} else {
                        __assert_fail(
                            b"NGHTTP3_ERR_QPACK_FATAL == nread\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3399 as ::core::ffi::c_uint,
                            b"nghttp3_ssize nghttp3_qpack_decoder_read_request(nghttp3_qpack_decoder *, nghttp3_qpack_stream_context *, nghttp3_qpack_nv *, uint8_t *, const uint8_t *, size_t, int)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                rv = NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
                break '_fail;
            } else {
                p = p.offset(nread as isize);
                if rfin == 0 {
                    break;
                }
                if (*sctx).dbase_sign != 0 {
                    if (*sctx).ricnt <= (*sctx).rstate.left {
                        rv = NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
                        break '_fail;
                    } else {
                        (*sctx).base = (*sctx)
                            .ricnt
                            .wrapping_sub((*sctx).rstate.left)
                            .wrapping_sub(1 as uint64_t);
                    }
                } else {
                    (*sctx).base = (*sctx).ricnt.wrapping_add((*sctx).rstate.left);
                }
                if (*sctx).ricnt > (*decoder).ctx.next_absidx {
                    (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_BLOCKED;
                    *pflags = (*pflags as ::core::ffi::c_uint
                        | NGHTTP3_QPACK_DECODE_FLAG_BLOCKED) as uint8_t;
                    return p.offset_from(src);
                }
                (*sctx).state = nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_OPCODE;
                (*sctx).rstate.left = 0 as uint64_t;
                (*sctx).rstate.shift = 0 as size_t;
            }
        }
        if fin != 0 {
            if (*sctx).state.0
                != nghttp3_qpack_request_stream_state::NGHTTP3_QPACK_RS_STATE_OPCODE.0
            {
                rv = NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
                break '_fail;
            } else {
                *pflags = (*pflags as ::core::ffi::c_uint
                    | NGHTTP3_QPACK_DECODE_FLAG_FINAL) as uint8_t;
                if (*sctx).ricnt != 0 {
                    rv = nghttp3_qpack_decoder_write_section_ack(decoder, sctx);
                    if rv != 0 as ::core::ffi::c_int {
                        break '_fail;
                    }
                }
                (*decoder).uninterrupted_encoderlen = 0 as size_t;
            }
        }
        return p.offset_from(src);
    }
    (*decoder).ctx.bad = 1 as uint8_t;
    return rv as nghttp3_ssize;
}
unsafe extern "C" fn qpack_decoder_dbuf_overflow(
    mut decoder: *const nghttp3_qpack_decoder,
) -> ::core::ffi::c_int {
    let mut limit: size_t = nghttp3_max_unsigned_long_int(
        (*decoder).max_concurrent_streams as ::core::ffi::c_ulong,
        100 as ::core::ffi::c_ulong,
    ) as size_t;
    return (nghttp3_buf_len(&raw const (*decoder).dbuf)
        > limit.wrapping_mul(2 as size_t).wrapping_mul(10 as size_t))
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_write_section_ack(
    mut decoder: *mut nghttp3_qpack_decoder,
    mut sctx: *const nghttp3_qpack_stream_context,
) -> ::core::ffi::c_int {
    let mut dbuf: *mut nghttp3_buf = &raw mut (*decoder).dbuf;
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut rv: ::core::ffi::c_int = 0;
    if qpack_decoder_dbuf_overflow(decoder) != 0 {
        return NGHTTP3_ERR_QPACK_FATAL;
    }
    rv = reserve_buf(
        dbuf,
        nghttp3_qpack_put_varint_len((*sctx).stream_id as uint64_t, 7 as size_t),
        (*decoder).ctx.mem,
    );
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    p = (*dbuf).last;
    *p = 0x80 as uint8_t;
    (*dbuf).last = nghttp3_qpack_put_varint(
        p,
        (*sctx).stream_id as uint64_t,
        7 as size_t,
    );
    if (*decoder).written_icnt < (*sctx).ricnt {
        (*decoder).written_icnt = (*sctx).ricnt;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_get_decoder_streamlen(
    mut decoder: *mut nghttp3_qpack_decoder,
) -> size_t {
    return nghttp3_qpack_decoder_get_decoder_streamlen2(decoder);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_get_decoder_streamlen2(
    mut decoder: *const nghttp3_qpack_decoder,
) -> size_t {
    let mut n: uint64_t = 0;
    let mut len: size_t = 0 as size_t;
    if (*decoder).written_icnt < (*decoder).ctx.next_absidx {
        n = (*decoder).ctx.next_absidx.wrapping_sub((*decoder).written_icnt);
        len = nghttp3_qpack_put_varint_len(n, 6 as size_t);
    }
    return nghttp3_buf_len(&raw const (*decoder).dbuf).wrapping_add(len);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_write_decoder(
    mut decoder: *mut nghttp3_qpack_decoder,
    mut dbuf: *mut nghttp3_buf,
) {
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut n: uint64_t = 0 as uint64_t;
    let mut len: size_t = 0 as size_t;
    if (*decoder).written_icnt < (*decoder).ctx.next_absidx {
        n = (*decoder).ctx.next_absidx.wrapping_sub((*decoder).written_icnt);
        len = nghttp3_qpack_put_varint_len(n, 6 as size_t);
    }
    '_c2rust_label: {
        if nghttp3_buf_left(dbuf)
            >= nghttp3_buf_len(&raw mut (*decoder).dbuf).wrapping_add(len)
        {} else {
            __assert_fail(
                b"nghttp3_buf_left(dbuf) >= nghttp3_buf_len(&decoder->dbuf) + len\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                3860 as ::core::ffi::c_uint,
                b"void nghttp3_qpack_decoder_write_decoder(nghttp3_qpack_decoder *, nghttp3_buf *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if nghttp3_buf_len(&raw mut (*decoder).dbuf) != 0 {
        (*dbuf).last = nghttp3_cpymem(
            (*dbuf).last,
            (*decoder).dbuf.pos,
            nghttp3_buf_len(&raw mut (*decoder).dbuf),
        );
    }
    if n != 0 {
        p = (*dbuf).last;
        *p = 0 as uint8_t;
        (*dbuf).last = nghttp3_qpack_put_varint(p, n, 6 as size_t);
        (*decoder).written_icnt = (*decoder).ctx.next_absidx;
    }
    nghttp3_buf_reset(&raw mut (*decoder).dbuf);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_cancel_stream(
    mut decoder: *mut nghttp3_qpack_decoder,
    mut stream_id: int64_t,
) -> ::core::ffi::c_int {
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut rv: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if stream_id >= 0 as int64_t {} else {
            __assert_fail(
                b"stream_id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                3883 as ::core::ffi::c_uint,
                b"int nghttp3_qpack_decoder_cancel_stream(nghttp3_qpack_decoder *, int64_t)\0"
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
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                3884 as ::core::ffi::c_uint,
                b"int nghttp3_qpack_decoder_cancel_stream(nghttp3_qpack_decoder *, int64_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if qpack_decoder_dbuf_overflow(decoder) != 0 {
        return NGHTTP3_ERR_QPACK_FATAL;
    }
    rv = reserve_buf(
        &raw mut (*decoder).dbuf,
        nghttp3_qpack_put_varint_len(stream_id as uint64_t, 6 as size_t),
        (*decoder).ctx.mem,
    );
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    p = (*decoder).dbuf.last;
    *p = 0x40 as uint8_t;
    (*decoder).dbuf.last = nghttp3_qpack_put_varint(
        p,
        stream_id as uint64_t,
        6 as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_reconstruct_ricnt(
    mut decoder: *const nghttp3_qpack_decoder,
    mut dest: *mut uint64_t,
    mut encricnt: uint64_t,
) -> ::core::ffi::c_int {
    let mut max_ents: uint64_t = 0;
    let mut full: uint64_t = 0;
    let mut max: uint64_t = 0;
    let mut max_wrapped: uint64_t = 0;
    let mut ricnt: uint64_t = 0;
    if encricnt == 0 as uint64_t {
        *dest = 0 as uint64_t;
        return 0 as ::core::ffi::c_int;
    }
    max_ents = (*decoder)
        .ctx
        .hard_max_dtable_capacity
        .wrapping_div(NGHTTP3_QPACK_ENTRY_OVERHEAD as size_t) as uint64_t;
    full = (2 as uint64_t).wrapping_mul(max_ents);
    if encricnt > full {
        return NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
    }
    max = (*decoder).ctx.next_absidx.wrapping_add(max_ents);
    max_wrapped = max.wrapping_div(full).wrapping_mul(full);
    ricnt = max_wrapped.wrapping_add(encricnt).wrapping_sub(1 as uint64_t);
    if ricnt > max {
        if ricnt <= full {
            return NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
        }
        ricnt = ricnt.wrapping_sub(full);
    }
    if ricnt == 0 as uint64_t {
        return NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
    }
    *dest = ricnt;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_rel2abs(
    mut decoder: *const nghttp3_qpack_decoder,
    mut rstate: *mut nghttp3_qpack_read_state,
) -> ::core::ffi::c_int {
    if (*rstate).dynamic != 0 {
        if (*decoder).ctx.next_absidx < (*rstate).left.wrapping_add(1 as uint64_t) {
            return NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR;
        }
        (*rstate).absidx = (*decoder)
            .ctx
            .next_absidx
            .wrapping_sub((*rstate).left)
            .wrapping_sub(1 as uint64_t);
    } else {
        (*rstate).absidx = (*rstate).left;
    }
    if qpack_decoder_validate_index(decoder, rstate) != 0 as ::core::ffi::c_int {
        return NGHTTP3_ERR_QPACK_ENCODER_STREAM_ERROR;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_brel2abs(
    mut decoder: *const nghttp3_qpack_decoder,
    mut sctx: *mut nghttp3_qpack_stream_context,
) -> ::core::ffi::c_int {
    let mut rstate: *mut nghttp3_qpack_read_state = &raw mut (*sctx).rstate;
    if (*rstate).dynamic != 0 {
        if (*sctx).base < (*rstate).left.wrapping_add(1 as uint64_t) {
            return NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
        }
        (*rstate).absidx = (*sctx)
            .base
            .wrapping_sub((*rstate).left)
            .wrapping_sub(1 as uint64_t);
        if (*rstate).absidx >= (*sctx).ricnt {
            return NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
        }
    } else {
        (*rstate).absidx = (*rstate).left;
    }
    if qpack_decoder_validate_index(decoder, rstate) != 0 as ::core::ffi::c_int {
        return NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_pbrel2abs(
    mut decoder: *const nghttp3_qpack_decoder,
    mut sctx: *mut nghttp3_qpack_stream_context,
) -> ::core::ffi::c_int {
    let mut rstate: *mut nghttp3_qpack_read_state = &raw mut (*sctx).rstate;
    '_c2rust_label: {
        if (*rstate).dynamic != 0 {} else {
            __assert_fail(
                b"rstate->dynamic\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                3994 as ::core::ffi::c_uint,
                b"int nghttp3_qpack_decoder_pbrel2abs(const nghttp3_qpack_decoder *, nghttp3_qpack_stream_context *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    (*rstate).absidx = (*rstate).left.wrapping_add((*sctx).base);
    if (*rstate).absidx >= (*sctx).ricnt {
        return NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
    }
    if qpack_decoder_validate_index(decoder, rstate) != 0 as ::core::ffi::c_int {
        return NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn qpack_decoder_emit_static_indexed(
    mut decoder: *const nghttp3_qpack_decoder,
    mut sctx: *const nghttp3_qpack_stream_context,
    mut nv: *mut nghttp3_qpack_nv,
) {
    let mut shd: *const nghttp3_qpack_static_header = (&raw mut stable
        as *mut nghttp3_qpack_static_header)
        .offset((*sctx).rstate.absidx as isize);
    (*nv).name = &raw const (*shd).name as *mut nghttp3_rcbuf;
    (*nv).value = &raw const (*shd).value as *mut nghttp3_rcbuf;
    (*nv).token = (*shd).token;
    (*nv).flags = NGHTTP3_NV_FLAG_NONE as uint8_t;
}
unsafe extern "C" fn qpack_decoder_emit_dynamic_indexed(
    mut decoder: *mut nghttp3_qpack_decoder,
    mut sctx: *const nghttp3_qpack_stream_context,
    mut nv: *mut nghttp3_qpack_nv,
) {
    let mut ent: *mut nghttp3_qpack_entry = nghttp3_qpack_context_dtable_get(
        &raw mut (*decoder).ctx,
        (*sctx).rstate.absidx,
    );
    *nv = (*ent).nv;
    nghttp3_rcbuf_incref((*nv).name);
    nghttp3_rcbuf_incref((*nv).value);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_emit_indexed(
    mut decoder: *mut nghttp3_qpack_decoder,
    mut sctx: *const nghttp3_qpack_stream_context,
    mut nv: *mut nghttp3_qpack_nv,
) {
    if (*sctx).rstate.dynamic != 0 {
        qpack_decoder_emit_dynamic_indexed(decoder, sctx, nv);
    } else {
        qpack_decoder_emit_static_indexed(decoder, sctx, nv);
    };
}
unsafe extern "C" fn qpack_decoder_emit_static_indexed_name(
    mut decoder: *const nghttp3_qpack_decoder,
    mut sctx: *mut nghttp3_qpack_stream_context,
    mut nv: *mut nghttp3_qpack_nv,
) {
    let mut shd: *const nghttp3_qpack_static_header = (&raw mut stable
        as *mut nghttp3_qpack_static_header)
        .offset((*sctx).rstate.absidx as isize);
    (*nv).name = &raw const (*shd).name as *mut nghttp3_rcbuf;
    (*nv).value = (*sctx).rstate.value;
    (*nv).token = (*shd).token;
    (*nv).flags = (if (*sctx).rstate.never != 0 {
        NGHTTP3_NV_FLAG_NEVER_INDEX
    } else {
        NGHTTP3_NV_FLAG_NONE
    }) as uint8_t;
    (*sctx).rstate.value = ::core::ptr::null_mut::<nghttp3_rcbuf>();
}
unsafe extern "C" fn qpack_decoder_emit_dynamic_indexed_name(
    mut decoder: *mut nghttp3_qpack_decoder,
    mut sctx: *mut nghttp3_qpack_stream_context,
    mut nv: *mut nghttp3_qpack_nv,
) -> ::core::ffi::c_int {
    let mut ent: *mut nghttp3_qpack_entry = ::core::ptr::null_mut::<
        nghttp3_qpack_entry,
    >();
    if qpack_decoder_validate_index(decoder, &raw mut (*sctx).rstate)
        != 0 as ::core::ffi::c_int
    {
        return NGHTTP3_ERR_QPACK_DECOMPRESSION_FAILED;
    }
    ent = nghttp3_qpack_context_dtable_get(
        &raw mut (*decoder).ctx,
        (*sctx).rstate.absidx,
    );
    (*nv).name = (*ent).nv.name;
    (*nv).value = (*sctx).rstate.value;
    (*nv).token = (*ent).nv.token;
    (*nv).flags = (if (*sctx).rstate.never != 0 {
        NGHTTP3_NV_FLAG_NEVER_INDEX
    } else {
        NGHTTP3_NV_FLAG_NONE
    }) as uint8_t;
    nghttp3_rcbuf_incref((*nv).name);
    (*sctx).rstate.value = ::core::ptr::null_mut::<nghttp3_rcbuf>();
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_emit_indexed_name(
    mut decoder: *mut nghttp3_qpack_decoder,
    mut sctx: *mut nghttp3_qpack_stream_context,
    mut nv: *mut nghttp3_qpack_nv,
) -> ::core::ffi::c_int {
    if (*sctx).rstate.dynamic != 0 {
        return qpack_decoder_emit_dynamic_indexed_name(decoder, sctx, nv);
    }
    qpack_decoder_emit_static_indexed_name(decoder, sctx, nv);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_emit_literal(
    mut decoder: *const nghttp3_qpack_decoder,
    mut sctx: *mut nghttp3_qpack_stream_context,
    mut nv: *mut nghttp3_qpack_nv,
) {
    (*nv).name = (*sctx).rstate.name;
    (*nv).value = (*sctx).rstate.value;
    (*nv).token = qpack_lookup_token((*(*nv).name).base, (*(*nv).name).len);
    (*nv).flags = (if (*sctx).rstate.never != 0 {
        NGHTTP3_NV_FLAG_NEVER_INDEX
    } else {
        NGHTTP3_NV_FLAG_NONE
    }) as uint8_t;
    (*sctx).rstate.name = ::core::ptr::null_mut::<nghttp3_rcbuf>();
    (*sctx).rstate.value = ::core::ptr::null_mut::<nghttp3_rcbuf>();
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_new2(
    mut pencoder: *mut *mut nghttp3_qpack_encoder,
    mut hard_max_dtable_capacity: size_t,
    mut seed: uint64_t,
    mut mem: *const nghttp3_mem,
) -> ::core::ffi::c_int {
    let mut p: *mut nghttp3_qpack_encoder = ::core::ptr::null_mut::<
        nghttp3_qpack_encoder,
    >();
    p = nghttp3_mem_malloc(mem, ::core::mem::size_of::<nghttp3_qpack_encoder>())
        as *mut nghttp3_qpack_encoder;
    if p.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    nghttp3_qpack_encoder_init(p, hard_max_dtable_capacity, seed, mem);
    *pencoder = p;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_new(
    mut pencoder: *mut *mut nghttp3_qpack_encoder,
    mut hard_max_dtable_capacity: size_t,
    mut mem: *const nghttp3_mem,
) -> ::core::ffi::c_int {
    return nghttp3_qpack_encoder_new2(
        pencoder,
        hard_max_dtable_capacity,
        0 as uint64_t,
        mem,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_encoder_del(
    mut encoder: *mut nghttp3_qpack_encoder,
) {
    let mut mem: *const nghttp3_mem = ::core::ptr::null::<nghttp3_mem>();
    if encoder.is_null() {
        return;
    }
    mem = (*encoder).ctx.mem;
    nghttp3_qpack_encoder_free(encoder);
    nghttp3_mem_free(mem, encoder as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_stream_context_new(
    mut psctx: *mut *mut nghttp3_qpack_stream_context,
    mut stream_id: int64_t,
    mut mem: *const nghttp3_mem,
) -> ::core::ffi::c_int {
    let mut p: *mut nghttp3_qpack_stream_context = ::core::ptr::null_mut::<
        nghttp3_qpack_stream_context,
    >();
    '_c2rust_label: {
        if stream_id >= 0 as int64_t {} else {
            __assert_fail(
                b"stream_id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                4168 as ::core::ffi::c_uint,
                b"int nghttp3_qpack_stream_context_new(nghttp3_qpack_stream_context **, int64_t, const nghttp3_mem *)\0"
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
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_qpack.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                4169 as ::core::ffi::c_uint,
                b"int nghttp3_qpack_stream_context_new(nghttp3_qpack_stream_context **, int64_t, const nghttp3_mem *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    p = nghttp3_mem_malloc(mem, ::core::mem::size_of::<nghttp3_qpack_stream_context>())
        as *mut nghttp3_qpack_stream_context;
    if p.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    nghttp3_qpack_stream_context_init(p, stream_id, mem);
    *psctx = p;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_stream_context_del(
    mut sctx: *mut nghttp3_qpack_stream_context,
) {
    let mut mem: *const nghttp3_mem = ::core::ptr::null::<nghttp3_mem>();
    if sctx.is_null() {
        return;
    }
    mem = (*sctx).mem;
    nghttp3_qpack_stream_context_free(sctx);
    nghttp3_mem_free(mem, sctx as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_new(
    mut pdecoder: *mut *mut nghttp3_qpack_decoder,
    mut hard_max_dtable_capacity: size_t,
    mut max_blocked_streams: size_t,
    mut mem: *const nghttp3_mem,
) -> ::core::ffi::c_int {
    let mut p: *mut nghttp3_qpack_decoder = ::core::ptr::null_mut::<
        nghttp3_qpack_decoder,
    >();
    p = nghttp3_mem_malloc(mem, ::core::mem::size_of::<nghttp3_qpack_decoder>())
        as *mut nghttp3_qpack_decoder;
    if p.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    nghttp3_qpack_decoder_init(p, hard_max_dtable_capacity, max_blocked_streams, mem);
    *pdecoder = p;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_del(
    mut decoder: *mut nghttp3_qpack_decoder,
) {
    let mut mem: *const nghttp3_mem = ::core::ptr::null::<nghttp3_mem>();
    if decoder.is_null() {
        return;
    }
    mem = (*decoder).ctx.mem;
    nghttp3_qpack_decoder_free(decoder);
    nghttp3_mem_free(mem, decoder as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_decoder_get_icnt(
    mut decoder: *const nghttp3_qpack_decoder,
) -> uint64_t {
    return (*decoder).ctx.next_absidx;
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    stable = [
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":authority\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 11]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__AUTHORITY.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":path\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 6]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"/\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 2]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__PATH.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"age\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"0\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 2]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_AGE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-disposition\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 20]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_DISPOSITION.0
                as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-length\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 15]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"0\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 2]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_LENGTH.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"cookie\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 7]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_COOKIE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"date\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 5]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_DATE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"etag\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 5]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ETAG.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"if-modified-since\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 18]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_IF_MODIFIED_SINCE.0
                as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"if-none-match\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_IF_NONE_MATCH.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"last-modified\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_LAST_MODIFIED.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"link\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 5]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_LINK.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"location\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 9]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_LOCATION.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"referer\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_REFERER.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"set-cookie\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 11]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_SET_COOKIE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":method\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"CONNECT\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":method\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"DELETE\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 7]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":method\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"GET\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":method\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"HEAD\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 5]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":method\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"OPTIONS\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":method\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"POST\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 5]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":method\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"PUT\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__METHOD.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":scheme\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"http\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 5]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__SCHEME.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":scheme\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"https\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 6]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__SCHEME.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":status\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"103\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":status\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"200\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":status\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"304\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":status\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"404\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":status\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"503\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"accept\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 7]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"*/*\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCEPT.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"accept\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 7]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"application/dns-message\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 24]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCEPT.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"accept-encoding\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"gzip, deflate, br\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 18]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCEPT_ENCODING.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"accept-ranges\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"bytes\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 6]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCEPT_RANGES.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"access-control-allow-headers\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 29]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"cache-control\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_HEADERS
                .0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"access-control-allow-headers\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 29]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-type\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_HEADERS
                .0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"access-control-allow-origin\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 28]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"*\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 2]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_ORIGIN.0
                as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"cache-control\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"max-age=0\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 10]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CACHE_CONTROL.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"cache-control\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"max-age=2592000\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CACHE_CONTROL.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"cache-control\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"max-age=604800\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 15]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CACHE_CONTROL.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"cache-control\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"no-cache\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 9]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CACHE_CONTROL.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"cache-control\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"no-store\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 9]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CACHE_CONTROL.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"cache-control\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"public, max-age=31536000\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 25]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CACHE_CONTROL.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-encoding\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 17]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"br\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 3]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_ENCODING.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-encoding\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 17]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"gzip\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 5]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_ENCODING.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-type\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"application/dns-message\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 24]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-type\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"application/javascript\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 23]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-type\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"application/json\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 17]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-type\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"application/x-www-form-urlencoded\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 34]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-type\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"image/gif\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 10]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-type\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 11]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-type\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"image/png\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 10]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-type\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"text/css\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 9]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-type\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"text/html; charset=utf-8\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 25]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-type\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"text/plain\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 11]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-type\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"text/plain;charset=utf-8\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 25]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_TYPE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"range\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 6]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"bytes=0-\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 9]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_RANGE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"strict-transport-security\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 26]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"max-age=31536000\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 17]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_STRICT_TRANSPORT_SECURITY.0
                as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"strict-transport-security\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 26]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"max-age=31536000; includesubdomains\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 36]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_STRICT_TRANSPORT_SECURITY.0
                as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"strict-transport-security\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 26]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"max-age=31536000; includesubdomains; preload\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 45]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_STRICT_TRANSPORT_SECURITY.0
                as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"vary\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 5]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"accept-encoding\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_VARY.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"vary\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 5]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"origin\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 7]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_VARY.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"x-content-type-options\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 23]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"nosniff\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_X_CONTENT_TYPE_OPTIONS.0
                as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"x-xss-protection\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 17]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"1; mode=block\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_X_XSS_PROTECTION.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":status\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"100\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":status\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"204\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":status\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"206\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":status\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"302\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":status\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"400\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":status\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"403\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":status\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"421\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":status\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"425\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b":status\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"500\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN__STATUS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"accept-language\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCEPT_LANGUAGE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"access-control-allow-credentials\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 33]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"FALSE\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 6]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_CREDENTIALS
                .0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"access-control-allow-credentials\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 33]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"TRUE\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 5]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_CREDENTIALS
                .0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"access-control-allow-headers\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 29]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"*\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 2]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_HEADERS
                .0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"access-control-allow-methods\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 29]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"get\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_METHODS
                .0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"access-control-allow-methods\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 29]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"get, post, options\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 19]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_METHODS
                .0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"access-control-allow-methods\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 29]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"options\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_ALLOW_METHODS
                .0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"access-control-expose-headers\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 30]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-length\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 15]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_EXPOSE_HEADERS
                .0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"access-control-request-headers\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 31]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-type\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_REQUEST_HEADERS
                .0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"access-control-request-method\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 30]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"get\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 4]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_REQUEST_METHOD
                .0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"access-control-request-method\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 30]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"post\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 5]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ACCESS_CONTROL_REQUEST_METHOD
                .0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"alt-svc\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"clear\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 6]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ALT_SVC.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"authorization\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 14]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_AUTHORIZATION.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"content-security-policy\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 24]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"script-src 'none'; object-src 'none'; base-uri 'none'\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 54]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_CONTENT_SECURITY_POLICY.0
                as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"early-data\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 11]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"1\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 2]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_EARLY_DATA.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"expect-ct\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 10]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_EXPECT_CT.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"forwarded\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 10]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_FORWARDED.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"if-range\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 9]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_IF_RANGE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"origin\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 7]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_ORIGIN.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"purpose\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"prefetch\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 9]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_PURPOSE.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"server\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 7]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_SERVER.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"timing-allow-origin\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 20]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"*\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 2]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_TIMING_ALLOW_ORIGIN.0
                as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"upgrade-insecure-requests\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 26]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"1\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 2]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_UPGRADE_INSECURE_REQUESTS.0
                as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"user-agent\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 11]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_USER_AGENT.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"x-forwarded-for\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 1]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_X_FORWARDED_FOR.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"x-frame-options\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"deny\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 5]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_X_FRAME_OPTIONS.0 as int32_t,
        },
        nghttp3_qpack_static_header {
            name: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"x-frame-options\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            value: nghttp3_rcbuf {
                mem: ::core::ptr::null::<nghttp3_mem>(),
                base: b"sameorigin\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut uint8_t,
                len: ::core::mem::size_of::<[::core::ffi::c_char; 11]>()
                    .wrapping_sub(1 as size_t),
                r#ref: -1 as int32_t,
            },
            token: nghttp3_qpack_token::NGHTTP3_QPACK_TOKEN_X_FRAME_OPTIONS.0 as int32_t,
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
