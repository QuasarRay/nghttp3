# Retained QPACK input ownership provenance

This layer captures the lifetime failure fixed by
`fce4985891606e973a97c4d64fc2247df4f8daca` ("fuzz: Fix ASAN failure").

The historical fuzz request retained a shallow `nghttp3_buf` view whose backing
bytes came from a temporary fuzz chunk. A blocked request could therefore
outlive the chunk and later decode through dangling storage. The fix changed the
request to own a byte vector and derive a span from that owned storage.

The Rust replacement avoids a self-referential span entirely:

- `OwnedInput` stores the backing bytes in `Box<[u8]>`;
- decoder progress is represented only by an integer offset;
- `remaining()` derives a slice from the owner on each call;
- `consume` refuses to advance past the owned allocation and leaves state
  unchanged on failure.

RFC 9204 remains authoritative for QPACK protocol semantics. The backing-storage
lifetime rule is an implementation-safety invariant mined from ASAN/fuzz
history.

Kani proves symbolic suffix preservation and failed over-consumption behavior.
Lambars generates permanent ownership/bounds regressions. Verus records the
offset/length invariant independently of the concrete allocation type.
