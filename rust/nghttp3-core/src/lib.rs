//! Runtime-independent safe protocol core for the nghttp3 Rust reimplementation.
//!
//! This crate must remain independent of Tokio and transport runtimes. It is the
//! destination for verified replacements of C2Rust-generated components.

#![forbid(unsafe_code)]

pub mod priority;
pub mod qpack_buffer;
pub mod qpack_read_state;
pub mod qpack_reference;
pub mod ringbuf;
pub mod settings;
pub mod varint;
