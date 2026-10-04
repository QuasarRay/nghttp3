# QPACK stream-reference publication provenance

## Historical source

nghttp3 commit
`62743057dbe607f5d65ce65416efeaa2b792090e` fixed a double free on
out-of-memory.

Before that fix, `nghttp3_qpack_stream_add_ref`:

1. published the heap-owned reference into the stream ring buffer;
2. attempted the fallible priority-queue insertion;
3. returned an error when insertion failed.

The caller then deleted the reference, while the ring buffer still contained the
same raw pointer. Later cleanup could free it again.

The C fix reordered the operations: priority-queue insertion must succeed before
ring-buffer publication.

## Rust invariant

The safe model makes the ordering a type-level requirement:

`PendingReference<T>`
→ fallible registration
→ `RegisteredReference<T, Registration>`
→ publication into `StreamReferences<T, Registration>`.

`StreamReferences::publish` does not accept a pending reference. A failed
registration therefore returns the sole unpublished owner; there is no
published alias to become dangling.

A registration token is owned alongside the published value. Future QPACK
integration can use an RAII token/guard for priority-queue membership rather
than an intrusive raw-pointer entry.

## Authority boundary

This is an implementation ownership invariant, not an RFC 9204 protocol rule.
RFC 9204 remains authoritative for QPACK semantics; the historical commit is the
regression oracle for publication ordering and lifetime safety.
