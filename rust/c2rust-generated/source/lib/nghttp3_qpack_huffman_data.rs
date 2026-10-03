pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
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
#[no_mangle]
pub static mut huffman_sym_table: [nghttp3_qpack_huffman_sym; 257] = [
    nghttp3_qpack_huffman_sym {
        nbits: 13 as uint32_t,
        code: 0xffc00000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffb000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xfffffe20 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xfffffe30 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xfffffe40 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xfffffe50 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xfffffe60 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xfffffe70 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xfffffe80 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 24 as uint32_t,
        code: 0xffffea00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 30 as uint32_t,
        code: 0xfffffff0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xfffffe90 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xfffffea0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 30 as uint32_t,
        code: 0xfffffff4 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xfffffeb0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xfffffec0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xfffffed0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xfffffee0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xfffffef0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffff00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffff10 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffff20 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 30 as uint32_t,
        code: 0xfffffff8 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffff30 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffff40 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffff50 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffff60 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffff70 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffff80 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffff90 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffffa0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffffb0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x50000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 10 as uint32_t,
        code: 0xfe000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 10 as uint32_t,
        code: 0xfe400000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 12 as uint32_t,
        code: 0xffa00000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 13 as uint32_t,
        code: 0xffc80000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x54000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 8 as uint32_t,
        code: 0xf8000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 11 as uint32_t,
        code: 0xff400000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 10 as uint32_t,
        code: 0xfe800000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 10 as uint32_t,
        code: 0xfec00000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 8 as uint32_t,
        code: 0xf9000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 11 as uint32_t,
        code: 0xff600000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 8 as uint32_t,
        code: 0xfa000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x58000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x5c000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x60000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 5 as uint32_t,
        code: 0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 5 as uint32_t,
        code: 0x8000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 5 as uint32_t,
        code: 0x10000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x64000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x68000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x6c000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x70000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x74000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x78000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x7c000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xb8000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 8 as uint32_t,
        code: 0xfb000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 15 as uint32_t,
        code: 0xfff80000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x80000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 12 as uint32_t,
        code: 0xffb00000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 10 as uint32_t,
        code: 0xff000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 13 as uint32_t,
        code: 0xffd00000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x84000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xba000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xbc000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xbe000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xc0000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xc2000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xc4000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xc6000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xc8000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xca000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xcc000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xce000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xd0000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xd2000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xd4000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xd6000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xd8000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xda000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xdc000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xde000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xe0000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xe2000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xe4000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 8 as uint32_t,
        code: 0xfc000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xe6000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 8 as uint32_t,
        code: 0xfd000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 13 as uint32_t,
        code: 0xffd80000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 19 as uint32_t,
        code: 0xfffe0000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 13 as uint32_t,
        code: 0xffe00000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 14 as uint32_t,
        code: 0xfff00000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x88000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 15 as uint32_t,
        code: 0xfffa0000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 5 as uint32_t,
        code: 0x18000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x8c000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 5 as uint32_t,
        code: 0x20000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x90000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 5 as uint32_t,
        code: 0x28000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x94000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x98000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0x9c000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 5 as uint32_t,
        code: 0x30000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xe8000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xea000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0xa0000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0xa4000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0xa8000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 5 as uint32_t,
        code: 0x38000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0xac000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xec000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0xb0000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 5 as uint32_t,
        code: 0x40000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 5 as uint32_t,
        code: 0x48000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 6 as uint32_t,
        code: 0xb4000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xee000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xf0000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xf2000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xf4000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 7 as uint32_t,
        code: 0xf6000000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 15 as uint32_t,
        code: 0xfffc0000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 11 as uint32_t,
        code: 0xff800000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 14 as uint32_t,
        code: 0xfff40000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 13 as uint32_t,
        code: 0xffe80000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffffc0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 20 as uint32_t,
        code: 0xfffe6000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff4800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 20 as uint32_t,
        code: 0xfffe7000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 20 as uint32_t,
        code: 0xfffe8000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff4c00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff5000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff5400 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffb200 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff5800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffb400 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffb600 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffb800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffba00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffbc00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 24 as uint32_t,
        code: 0xffffeb00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffbe00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 24 as uint32_t,
        code: 0xffffec00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 24 as uint32_t,
        code: 0xffffed00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff5c00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffc000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 24 as uint32_t,
        code: 0xffffee00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffc200 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffc400 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffc600 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffc800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 21 as uint32_t,
        code: 0xfffee000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff6000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffca00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff6400 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffcc00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffce00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 24 as uint32_t,
        code: 0xffffef00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff6800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 21 as uint32_t,
        code: 0xfffee800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 20 as uint32_t,
        code: 0xfffe9000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff6c00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff7000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffd000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffd200 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 21 as uint32_t,
        code: 0xfffef000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffd400 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff7400 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff7800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 24 as uint32_t,
        code: 0xfffff000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 21 as uint32_t,
        code: 0xfffef800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff7c00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffd600 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffd800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 21 as uint32_t,
        code: 0xffff0000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 21 as uint32_t,
        code: 0xffff0800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff8000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 21 as uint32_t,
        code: 0xffff1000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffda00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff8400 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffdc00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffde00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 20 as uint32_t,
        code: 0xfffea000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff8800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff8c00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff9000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffe000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff9400 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff9800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffe200 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffff800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffff840 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 20 as uint32_t,
        code: 0xfffeb000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 19 as uint32_t,
        code: 0xfffe2000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffff9c00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffe400 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffffa000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 25 as uint32_t,
        code: 0xfffff600 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffff880 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffff8c0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffff900 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffbc0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffbe0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffff940 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 24 as uint32_t,
        code: 0xfffff100 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 25 as uint32_t,
        code: 0xfffff680 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 19 as uint32_t,
        code: 0xfffe4000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 21 as uint32_t,
        code: 0xffff1800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffff980 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffc00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffc20 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffff9c0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffc40 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 24 as uint32_t,
        code: 0xfffff200 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 21 as uint32_t,
        code: 0xffff2000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 21 as uint32_t,
        code: 0xffff2800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffffa00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffffa40 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffffd0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffc60 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffc80 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffca0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 20 as uint32_t,
        code: 0xfffec000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 24 as uint32_t,
        code: 0xfffff300 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 20 as uint32_t,
        code: 0xfffed000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 21 as uint32_t,
        code: 0xffff3000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffffa400 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 21 as uint32_t,
        code: 0xffff3800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 21 as uint32_t,
        code: 0xffff4000 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffe600 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffffa800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 22 as uint32_t,
        code: 0xffffac00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 25 as uint32_t,
        code: 0xfffff700 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 25 as uint32_t,
        code: 0xfffff780 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 24 as uint32_t,
        code: 0xfffff400 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 24 as uint32_t,
        code: 0xfffff500 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffffa80 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 23 as uint32_t,
        code: 0xffffe800 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffffac0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffcc0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffffb00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffffb40 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffce0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffd00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffd20 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffd40 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffd60 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 28 as uint32_t,
        code: 0xffffffe0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffd80 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffda0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffdc0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffde0 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 27 as uint32_t,
        code: 0xfffffe00 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 26 as uint32_t,
        code: 0xfffffb80 as uint32_t,
    },
    nghttp3_qpack_huffman_sym {
        nbits: 30 as uint32_t,
        code: 0xfffffffc as uint32_t,
    },
];
#[no_mangle]
pub static mut qpack_huffman_decode_table: [[nghttp3_qpack_huffman_decode_node; 16]; 257] = [
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x4 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x5 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x7 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x8 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xb as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xc as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x10 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x13 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x19 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1c as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x20 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x23 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2a as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x31 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x39 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x40 as uint16_t,
            flags: 0x1 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x31 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x61 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x65 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x74 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xd as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xe as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x11 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x12 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x14 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x15 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x31 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x31 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x61 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x61 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x65 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x65 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6f as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x31 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x31 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x31 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x31 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x61 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x61 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x61 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x61 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x30 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x31 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x31 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x31 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x31 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x31 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x31 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x31 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x31 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x32 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x61 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x61 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x61 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x61 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x61 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x61 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x61 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x61 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x65 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x65 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x65 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x65 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6f as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x63 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x65 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x65 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x65 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x65 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x65 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x65 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x65 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x65 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x69 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6f as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x74 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x74 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x25 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x33 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x35 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x37 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x39 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x74 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x74 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x74 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x74 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x25 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x25 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2e as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x73 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x74 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x74 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x74 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x74 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x74 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x74 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x74 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x74 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x25 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x25 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x25 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x25 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2e as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x20 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x25 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x25 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x25 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x25 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x25 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x25 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x25 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x25 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2e as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x33 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x33 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x35 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x35 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x37 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x37 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x39 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x39 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x33 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x33 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x33 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x33 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x35 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x35 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x35 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x35 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x33 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x33 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x33 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x33 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x33 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x33 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x33 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x33 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x34 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x35 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x35 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x35 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x35 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x35 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x35 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x35 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x35 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x37 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x37 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x37 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x37 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x39 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x39 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x39 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x39 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x36 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x37 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x37 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x37 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x37 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x37 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x37 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x37 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x37 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x38 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x39 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x39 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x39 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x39 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x39 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x39 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x39 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x39 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1a as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1b as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1d as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1e as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x21 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x22 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x24 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x25 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2b as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2e as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x32 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x35 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3a as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3d as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x41 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x44 as uint16_t,
            flags: 0x1 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x41 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x62 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x66 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x68 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x70 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x75 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x26 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x27 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x41 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x41 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x62 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x62 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x66 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x66 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x68 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x68 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x41 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x41 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x41 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x41 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x62 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x62 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x62 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x62 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x41 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x41 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x41 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x41 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x41 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x41 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x41 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x41 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x62 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x62 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x62 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x62 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x62 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x62 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x62 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x62 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x66 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x66 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x66 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x66 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x68 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x68 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x68 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x68 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x64 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x66 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x66 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x66 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x66 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x66 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x66 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x66 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x66 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x67 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x68 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x68 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x68 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x68 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x68 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x68 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x68 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x68 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x70 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x70 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x75 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x75 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x42 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x44 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x70 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x70 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x70 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x70 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6d as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x70 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x70 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x70 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x70 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x70 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x70 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x70 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x70 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x75 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x75 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x75 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x75 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x42 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x42 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x44 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x44 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x72 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x75 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x75 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x75 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x75 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x75 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x75 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x75 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x75 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x42 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x42 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x42 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x42 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x44 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x44 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x44 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x44 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x42 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x42 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x42 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x42 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x42 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x42 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x42 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x42 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x43 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x44 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x44 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x44 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x44 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x44 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x44 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x44 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x44 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2c as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2d as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2f as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x30 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x33 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x34 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x36 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x37 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3b as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3c as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3e as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3f as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x42 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x43 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x45 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x48 as uint16_t,
            flags: 0x1 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x46 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x48 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x50 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x52 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x54 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x46 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x46 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x48 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x48 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4c as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x46 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x46 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x46 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x46 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x48 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x48 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x48 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x48 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x45 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x46 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x46 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x46 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x46 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x46 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x46 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x46 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x46 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x47 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x48 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x48 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x48 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x48 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x48 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x48 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x48 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x48 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4c as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x49 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4a as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4c as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x50 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x50 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x52 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x52 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x54 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x54 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x50 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x50 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x50 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x50 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4e as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x50 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x50 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x50 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x50 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x50 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x50 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x50 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x50 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x52 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x52 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x52 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x52 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x54 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x54 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x54 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x54 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x51 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x52 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x52 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x52 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x52 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x52 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x52 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x52 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x52 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x53 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x54 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x54 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x54 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x54 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x54 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x54 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x54 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x54 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x56 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x59 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x76 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x78 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x46 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x47 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x49 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x4a as uint16_t,
            flags: 0x1 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x56 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x56 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x59 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x59 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x76 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x76 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x56 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x56 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x56 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x56 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x59 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x59 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x59 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x59 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x55 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x56 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x56 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x56 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x56 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x56 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x56 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x56 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x56 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x57 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x59 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x59 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x59 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x59 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x59 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x59 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x59 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x59 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x76 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x76 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x76 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x76 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6b as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x71 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x76 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x76 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x76 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x76 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x76 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x76 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x76 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x76 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x78 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x78 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x4b as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x4e as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x78 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x78 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x78 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x78 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7a as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x77 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x78 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x78 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x78 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x78 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x78 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x78 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x78 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x78 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x79 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7a as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x4c as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x4d as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x4f as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x51 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3b as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x26 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2a as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3b as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x22 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x29 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x50 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x52 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x54 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x58 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5a as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x22 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x22 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x29 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x29 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x53 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x55 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x58 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x22 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x22 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x22 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x22 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x29 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x29 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x29 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x29 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x21 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x22 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x22 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x22 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x22 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x22 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x22 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x22 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x22 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x28 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x29 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x29 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x29 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x29 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x29 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x29 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x29 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x29 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x56 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x57 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x59 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x5a as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2b as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x27 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2b as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x24 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x5b as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x5c as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3e as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x23 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3e as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x24 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x24 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x5d as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x5e as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x24 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x24 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x24 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x24 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5b as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x24 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x24 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x24 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x24 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x24 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x24 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x24 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x24 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x40 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5b as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x60 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x5f as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7e as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x60 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x60 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x60 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6e as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7d as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x60 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x60 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x60 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x60 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x61 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x65 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6f as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x85 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x60 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x60 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x60 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x60 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x60 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x60 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x60 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x60 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x62 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x63 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x66 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x69 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x70 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x77 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x86 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x99 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x64 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x67 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x68 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6a as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6b as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x71 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x74 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x78 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x7e as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x87 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x8e as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9a as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa9 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x82 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6c as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6d as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x82 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x82 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc3 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x82 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x82 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x82 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x82 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x80 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x82 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x82 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x82 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x82 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x82 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x82 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x82 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x82 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xac as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x83 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa2 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc2 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xac as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xac as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe2 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xac as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xac as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xac as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xac as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x99 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa1 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xac as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xac as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xac as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xac as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xac as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xac as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xac as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xac as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x72 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x73 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x75 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x76 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x79 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x7b as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x7f as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x82 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x88 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x8b as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x8f as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x92 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9b as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa2 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xaa as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xb4 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x7a as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x7c as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x7d as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x80 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x81 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x83 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x84 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb1 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd1 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd9 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe5 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x84 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x86 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x92 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xad as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x84 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x84 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x86 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x86 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x92 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x92 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x84 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x84 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x84 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x84 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x81 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x84 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x84 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x84 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x84 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x84 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x84 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x84 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x84 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x86 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x86 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x86 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x86 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x92 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x92 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x92 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x92 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x85 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x86 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x86 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x86 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x86 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x86 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x86 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x86 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x86 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x88 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x92 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x92 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x92 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x92 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x92 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x92 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x92 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x92 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xad as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xad as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9c as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa3 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xad as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xad as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xad as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xad as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa9 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xaa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xad as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xad as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xad as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xad as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xad as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xad as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xad as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xad as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x89 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x8a as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x8c as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x8d as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x90 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x91 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x93 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x96 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9c as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9f as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa3 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa6 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xab as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xae as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xb5 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xbe as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xba as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x94 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x95 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x97 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x98 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xba as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xba as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xba as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xba as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xba as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xba as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb5 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xba as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xba as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xba as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xba as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xba as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xba as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xba as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xba as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbd as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc4 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x87 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8f as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe4 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe9 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x87 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x87 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8f as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x87 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x87 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x87 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x87 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8a as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x87 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x87 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x87 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x87 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x87 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x87 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x87 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x87 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x89 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8a as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8f as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8c as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8f as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9d as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9e as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa0 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa1 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa4 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa5 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa7 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa8 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xac as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xad as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xaf as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xb1 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xb6 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xb9 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xbf as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xcf as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x95 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x97 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xae as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x95 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x95 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x97 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x97 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9e as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x95 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x95 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x95 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x95 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x97 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x97 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x97 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x97 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x93 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x95 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x95 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x95 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x95 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x95 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x95 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x95 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x95 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x96 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x97 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x97 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x97 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x97 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x97 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x97 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x97 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x97 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9e as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x98 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9b as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9e as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xae as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xae as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xae as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xae as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xae as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xae as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa6 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xae as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xae as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xae as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xae as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xae as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xae as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xae as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xae as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xaf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb4 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb7 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xb0 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xb2 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xb3 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xb7 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xb8 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xba as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xbb as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xc0 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xc7 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xd0 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xdf as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x91 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9f as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xbf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xbf as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe7 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x91 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x91 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9f as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xef as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8e as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8e as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x91 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x91 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x91 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x91 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9f as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x90 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x91 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x91 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x91 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x91 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x91 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x91 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x91 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x91 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x94 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x9f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x9f as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xce as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xed as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xbc as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xbd as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xc1 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xc4 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xc8 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xcb as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xd1 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xd8 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xe0 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xee as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xce as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xce as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xed as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xed as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xeb as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xce as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xce as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xce as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xce as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xab as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xce as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xce as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xce as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xce as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xce as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xce as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xce as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xce as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe1 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xed as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xed as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xed as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xed as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xeb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xeb as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xec as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xed as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xed as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xed as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xed as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xed as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xed as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xed as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xed as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xeb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xeb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xeb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xeb as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcf as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xea as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xeb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xeb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xeb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xeb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xeb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xeb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xeb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xeb as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xc2 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xc3 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xc5 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xc6 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xc9 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xca as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xcc as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xcd as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xd2 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xd5 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xd9 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xdc as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xe1 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xe7 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xef as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf6 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xce as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc1 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc9 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xca as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcd as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd5 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcc as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xda as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdb as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xee as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcc as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf3 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xff as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcc as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xcc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xcc as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xd3 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xd4 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xd6 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xd7 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xda as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xdb as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xdd as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xde as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xe2 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xe4 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xe8 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xeb as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf0 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf3 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf7 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xfa as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfd as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdd as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd4 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdd as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xde as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdf as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf1 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf4 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfd as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf6 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf8 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfd as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfb as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfd as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xe3 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xe5 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xe6 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xe9 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xea as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xec as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xed as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf1 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf2 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf4 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf5 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf8 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf9 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xfb as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xfc as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x10 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x12 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xfe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x2 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x3 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x3 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x4 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x5 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x5 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x6 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x10 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x10 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x12 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x12 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x8 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xb as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xb as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xe as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xe as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x10 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x10 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x10 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x10 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x12 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x12 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x12 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x12 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x10 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x10 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x10 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x10 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x10 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x10 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x10 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x10 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x11 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x12 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x12 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x12 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x12 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x12 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x12 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x12 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x12 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x14 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x17 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x19 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xfd as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x14 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x14 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x17 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x17 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x19 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x19 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1b as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x14 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x14 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x14 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x14 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x17 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x17 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x17 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x17 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x13 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x14 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x14 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x14 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x14 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x14 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x14 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x14 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x14 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x15 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x17 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x17 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x17 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x17 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x17 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x17 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x17 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x17 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x19 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x19 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x19 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x19 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1b as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x18 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x19 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x19 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x19 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x19 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x19 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x19 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x19 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x19 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1a as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1b as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1b as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xfe as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xff as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1f as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1c as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1d as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1d as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1e as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x1f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x1f as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x7f as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xdc as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xdc as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xf9 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x16 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x2 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x9 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x17 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x28 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xa as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0xd as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0xd as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x3 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x6 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xa as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0xf as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x18 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x1f as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x29 as uint16_t,
            flags: 0x2 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x38 as uint16_t,
            flags: 0x3 as uint8_t,
            sym: 0x16 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
    [
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
        nghttp3_qpack_huffman_decode_node {
            fstate: 0x100 as uint16_t,
            flags: 0 as uint8_t,
            sym: 0 as uint8_t,
        },
    ],
];
