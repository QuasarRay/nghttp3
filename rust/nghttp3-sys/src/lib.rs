//! Raw FFI bindings to nghttp3.
//!
//! The bindings are generated from the repository's public `nghttp3.h`
//! at build time and the bundled C library is linked statically.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    improper_ctypes
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
