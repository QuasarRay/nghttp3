//! Runtime-independent safe protocol core for the nghttp3 Rust reimplementation.
//!
//! This crate must remain independent of Tokio and transport runtimes. It is the
//! destination for verified replacements of C2Rust-generated components.

#![forbid(unsafe_code)]

pub mod owned_input;
pub mod priority_update;
pub mod qpack;
pub mod qpack_decoder;
pub mod qpack_stream;
pub mod ringbuf;
pub mod settings;
pub mod structured;
pub mod varint;
