# QPACK publication-order verification provenance

This layer captures historical commit
`62743057dbe607f5d65ce65416efeaa2b792090e` ("Fix double free on out of memory").

The C bug published a header-block reference into the stream ring buffer before
attempting priority-queue insertion. If priority-queue insertion failed, cleanup
paths could disagree about ownership and free the same reference twice.

The fixed ordering is:

1. register the reference in the prerequisite priority queue;
2. return immediately on registration failure;
3. only then publish ownership in the stream reference collection.

`PublishedRefs<T>::register_then_publish` encodes exactly this sequence. The
publication collection is mutated only after the registration callback returns
success. On failure, ordinary Rust ownership drops the unpublished value and the
collection remains unchanged.

RFC 9204 remains authoritative for QPACK protocol semantics. Publication order
is an nghttp3 implementation-safety invariant mined from repository history.
Kani proves publication occurs iff registration succeeds and that a failed
second registration preserves all previously published references. Lambars
generates the historical OOM regression. Verus records the implication
`published => registered` independently of container implementation.
