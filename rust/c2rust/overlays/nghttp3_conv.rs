//! C ABI overlay for lib/nghttp3_conv.c.
//!
//! Pointer operations are intentionally isolated here. QUIC variable-integer
//! semantics delegate to the verbatim copied, verified safe_varint module.

use crate::safe_varint;
use std::ptr;
use std::slice;

#[no_mangle]
pub unsafe extern "C" fn nghttp3_get_uvarint(dest: *mut u64, p: *const u8) -> *const u8 {
    let len = 1_usize << ((*p >> 6) as usize);
    let input = slice::from_raw_parts(p, len);
    let (value, consumed) = safe_varint::decode(input)
        .expect("C ABI precondition provides a complete QUIC variable integer");
    *dest = value;
    p.add(consumed)
}

#[no_mangle]
pub unsafe extern "C" fn nghttp3_get_uvarintlen(p: *const u8) -> usize {
    1_usize << ((*p >> 6) as usize)
}

#[no_mangle]
pub unsafe extern "C" fn nghttp3_get_varint(dest: *mut i64, p: *const u8) -> *const u8 {
    let mut value = 0_u64;
    let next = nghttp3_get_uvarint(&mut value, p);
    *dest = value as i64;
    next
}

#[no_mangle]
pub unsafe extern "C" fn nghttp3_put_uint64be(p: *mut u8, n: u64) -> *mut u8 {
    let bytes = n.to_be_bytes();
    ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len());
    p.add(bytes.len())
}

#[no_mangle]
pub unsafe extern "C" fn nghttp3_put_uint32be(p: *mut u8, n: u32) -> *mut u8 {
    let bytes = n.to_be_bytes();
    ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len());
    p.add(bytes.len())
}

#[no_mangle]
pub unsafe extern "C" fn nghttp3_put_uint16be(p: *mut u8, n: u16) -> *mut u8 {
    let bytes = n.to_be_bytes();
    ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len());
    p.add(bytes.len())
}

#[no_mangle]
pub unsafe extern "C" fn nghttp3_put_uvarint(p: *mut u8, n: u64) -> *mut u8 {
    assert!(n <= safe_varint::MAX);
    let encoded = safe_varint::encode(n).expect("value checked against QUIC varint maximum");
    let bytes = encoded.as_slice();
    ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len());
    p.add(bytes.len())
}

#[no_mangle]
pub unsafe extern "C" fn nghttp3_put_uvarintlen(n: u64) -> usize {
    assert!(n <= safe_varint::MAX);
    safe_varint::encoded_len(n).expect("value checked against QUIC varint maximum")
}

#[no_mangle]
pub unsafe extern "C" fn nghttp3_ord_stream_id(stream_id: i64) -> u64 {
    ((stream_id >> 2) as u64).wrapping_add(1)
}
