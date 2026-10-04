# Retained QPACK request-input lifetime provenance

Historical commit
`fce4985891606e973a97c4d64fc2247df4f8daca` fixed an ASAN failure in
`fuzz/fuzz_qpackdecoder.cc`.

The old fuzz `Request` copied an `nghttp3_buf` view whose pointers referred
to a temporary fuzz-data vector. A request retained because QPACK decoding was
blocked could outlive that vector. The fix copied bytes into storage owned by
the retained request and kept a span into that storage.

## Rust representation

`OwnedRequestInput` avoids a self-referential span entirely:

- one `Vec<u8>` owns the complete input;
- one integer offset denotes the unread region;
- `remaining()` derives a slice on demand;
- `advance()` checks bounds and leaves the cursor unchanged on failure.

The source buffer may be dropped immediately after
`copy_from_slice`; a blocked request remains self-contained.

## Authority boundary

This historical issue was in the upstream fuzz harness rather than the nghttp3
production library. It is retained because it identifies a lifetime invariant
that the Rust decoder API should enforce for blocked requests and because it
strengthens the differential/fuzzing infrastructure used to verify the port.
RFC 9204 remains the protocol authority.
