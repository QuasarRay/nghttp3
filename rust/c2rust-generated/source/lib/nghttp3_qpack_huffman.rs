extern "C" {
    static huffman_sym_table: [nghttp3_qpack_huffman_sym; 0];
    static qpack_huffman_decode_table: [[nghttp3_qpack_huffman_decode_node; 16]; 0];
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type ptrdiff_t = isize;
pub type nghttp3_ssize = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_qpack_huffman_sym {
    pub nbits: uint32_t,
    pub code: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_qpack_huffman_decode_node {
    pub fstate: uint16_t,
    pub flags: uint8_t,
    pub sym: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_qpack_huffman_decode_context {
    pub fstate: uint16_t,
    pub flags: uint8_t,
}
pub const NGHTTP3_ERR_QPACK_FATAL: ::core::ffi::c_int = -108 as ::core::ffi::c_int;
pub const NGHTTP3_QPACK_HUFFMAN_FLAG_ACCEPTED: ::core::ffi::c_uint = 0x1
    as ::core::ffi::c_uint;
pub const NGHTTP3_QPACK_HUFFMAN_FLAG_SYM: ::core::ffi::c_uint = 0x2
    as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_huffman_encode_count(
    mut src: *const uint8_t,
    mut len: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut nbits: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < len {
        nbits = nbits
            .wrapping_add(
                (*(&raw const huffman_sym_table as *const nghttp3_qpack_huffman_sym)
                    .offset(*src.offset(i as isize) as isize))
                    .nbits as size_t,
            );
        i = i.wrapping_add(1);
    }
    return nbits.wrapping_add(7 as size_t).wrapping_div(8 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_huffman_encode(
    mut dest: *mut uint8_t,
    mut src: *const uint8_t,
    mut srclen: size_t,
) -> *mut uint8_t {
    let mut sym: *const nghttp3_qpack_huffman_sym = ::core::ptr::null::<
        nghttp3_qpack_huffman_sym,
    >();
    let mut end: *const uint8_t = src.offset(srclen as isize);
    let mut code: uint64_t = 0 as uint64_t;
    let mut nbits: size_t = 0 as size_t;
    let mut x: uint32_t = 0;
    while src != end {
        let c2rust_fresh0 = src;
        src = src.offset(1);
        sym = (&raw const huffman_sym_table as *const nghttp3_qpack_huffman_sym)
            .offset(*c2rust_fresh0 as isize);
        code |= ((*sym).code as uint64_t) << (32 as size_t).wrapping_sub(nbits);
        nbits = nbits.wrapping_add((*sym).nbits as size_t);
        if nbits < 32 as size_t {
            continue;
        }
        x = htonl((code >> 32 as ::core::ffi::c_int) as uint32_t);
        memcpy(
            dest as *mut ::core::ffi::c_void,
            &raw mut x as *const ::core::ffi::c_void,
            4 as size_t,
        );
        dest = dest.offset(4 as ::core::ffi::c_int as isize);
        code <<= 32 as ::core::ffi::c_int;
        nbits = nbits.wrapping_sub(32 as size_t);
    }
    while nbits >= 8 as size_t {
        let c2rust_fresh1 = dest;
        dest = dest.offset(1);
        *c2rust_fresh1 = (code >> 56 as ::core::ffi::c_int) as uint8_t;
        code <<= 8 as ::core::ffi::c_int;
        nbits = nbits.wrapping_sub(8 as size_t);
    }
    if nbits != 0 {
        let c2rust_fresh2 = dest;
        dest = dest.offset(1);
        *c2rust_fresh2 = ((code >> 56 as ::core::ffi::c_int) as uint8_t
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << (8 as size_t).wrapping_sub(nbits))
                - 1 as ::core::ffi::c_int) as uint8_t;
    }
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_huffman_decode_context_init(
    mut ctx: *mut nghttp3_qpack_huffman_decode_context,
) {
    *ctx = nghttp3_qpack_huffman_decode_context {
        fstate: 0,
        flags: NGHTTP3_QPACK_HUFFMAN_FLAG_ACCEPTED as uint8_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_huffman_decode(
    mut ctx: *mut nghttp3_qpack_huffman_decode_context,
    mut dest: *mut uint8_t,
    mut src: *const uint8_t,
    mut srclen: size_t,
    mut fin: ::core::ffi::c_int,
) -> nghttp3_ssize {
    let mut p: *mut uint8_t = dest;
    let mut end: *const uint8_t = src.offset(srclen as isize);
    let mut t: nghttp3_qpack_huffman_decode_node = nghttp3_qpack_huffman_decode_node {
        fstate: (*ctx).fstate,
        flags: (*ctx).flags,
        sym: 0,
    };
    let mut c: uint8_t = 0;
    while src != end {
        let c2rust_fresh3 = src;
        src = src.offset(1);
        c = *c2rust_fresh3;
        t = (*(&raw const qpack_huffman_decode_table
            as *const [nghttp3_qpack_huffman_decode_node; 16])
            .offset(
                t.fstate as isize,
            ))[(c as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as usize];
        if t.flags as ::core::ffi::c_uint & NGHTTP3_QPACK_HUFFMAN_FLAG_SYM != 0 {
            let c2rust_fresh4 = p;
            p = p.offset(1);
            *c2rust_fresh4 = t.sym;
        }
        t = (*(&raw const qpack_huffman_decode_table
            as *const [nghttp3_qpack_huffman_decode_node; 16])
            .offset(
                t.fstate as isize,
            ))[(c as ::core::ffi::c_uint & 0xf as ::core::ffi::c_uint) as usize];
        if t.flags as ::core::ffi::c_uint & NGHTTP3_QPACK_HUFFMAN_FLAG_SYM != 0 {
            let c2rust_fresh5 = p;
            p = p.offset(1);
            *c2rust_fresh5 = t.sym;
        }
    }
    (*ctx).fstate = t.fstate;
    (*ctx).flags = t.flags;
    if fin != 0
        && (*ctx).flags as ::core::ffi::c_uint & NGHTTP3_QPACK_HUFFMAN_FLAG_ACCEPTED == 0
    {
        return NGHTTP3_ERR_QPACK_FATAL as nghttp3_ssize;
    }
    return p.offset_from(dest);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_qpack_huffman_decode_failure_state(
    mut ctx: *const nghttp3_qpack_huffman_decode_context,
) -> ::core::ffi::c_int {
    return ((*ctx).fstate as ::core::ffi::c_uint == 0x100 as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
