# Rust bindings for nghttp3

This directory contains two crates:

- `nghttp3-sys`: generated raw FFI for the exported `nghttp3_*` ABI, public types, and constants from `lib/includes/nghttp3/nghttp3.h`. It also reproduces the header's public function-like versioning macros as Rust forwarding functions. The build script compiles and statically links the nghttp3 sources from this repository.
- `nghttp3`: safe wrappers for connection ownership, settings, callbacks, headers, request/response submission, stream I/O, standalone QPACK encoder/decoder state, reference-counted QPACK fields, priority parsing, varints, errors, and common utility functions. The raw crate remains available as `nghttp3::sys` for low-level or debug-only APIs.

## Build

A C11 compiler, CMake, Clang/libclang, and a stable Rust toolchain are required.

```sh
cargo test --manifest-path rust/Cargo.toml --workspace
```

The safe wrapper deliberately copies outgoing `writev` segments into Rust-owned buffers. This avoids exposing nghttp3's internal buffer lifetimes across asynchronous QUIC transports. Call `Connection::add_write_offset` with no more than the number of bytes accepted from the preceding `Connection::next_write` result.

Submitted request/response bodies are boxed so pointers handed to nghttp3 stay stable. Body storage is retained until `acked_stream_data` reports the application-owned bytes acknowledged; if a stream closes before that acknowledgement, the storage is conservatively retained until the connection is dropped.

The safe standalone QPACK wrappers always use nghttp3's default allocator. That lets decoded `RcBuffer` values own their C references independently of the decoder object's lifetime without exposing allocator-lifetime requirements in safe Rust.
