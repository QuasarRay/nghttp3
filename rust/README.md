# Rust bindings for nghttp3

This directory contains two crates:

- `nghttp3-sys`: complete raw FFI generated from `lib/includes/nghttp3/nghttp3.h` with bindgen. The build script compiles and statically links the nghttp3 sources from this repository.
- `nghttp3`: safe wrappers for connection ownership, settings, callbacks, headers, request/response submission, stream I/O, priority parsing, and common utility functions. The raw crate remains available as `nghttp3::sys` for API surface that does not yet have a high-level wrapper.

## Build

A C11 compiler, CMake, Clang/libclang, and a stable Rust toolchain are required.

```sh
cargo test --manifest-path rust/Cargo.toml --workspace
```

The safe wrapper deliberately copies outgoing `writev` segments into Rust-owned buffers. This avoids exposing nghttp3's internal buffer lifetimes across asynchronous QUIC transports. Call `Connection::add_write_offset` with the number of bytes accepted by the QUIC stack.

Submitted request/response bodies are owned by the connection until the stream-close callback, so pointers handed to nghttp3 remain stable through acknowledgement.
