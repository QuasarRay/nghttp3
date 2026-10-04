# QPACK decoder transient-ownership verification provenance

This layer captures the safety invariant from historical commit
`ecfae7acf1813f83843b78b607a6abd1fd47be86` ("Fix null dereference").

The original C decoder stored reference-counted `name` and `value` pointers in
its read state. After attempting to add a decoded entry to the dynamic table,
the transient reference was decremented. The historical bug left those pointers
published in `rstate` on allocation failure, so later cleanup/processing could
observe a released object.

The Rust replacement uses `Option<T>` ownership:

- indexed/static/dynamic inserts remove the transient value with `Option::take`
  before invoking the insertion callback;
- literal inserts verify both fields exist and then take both name and value
  before invoking the callback;
- success and insertion failure therefore have the same postcondition: consumed
  transient fields are absent from decoder state;
- missing-field errors are reported before consuming an available counterpart.

RFC 9204 remains authoritative for QPACK protocol semantics. The exact
transient-ownership postcondition is an nghttp3 implementation-safety
compatibility invariant mined from repository history.

Kani quantifies over insertion success/failure and symbolic field bytes. Lambars
generates permanent failure-path regressions from the same predicates. Verus
records the state-transition postcondition independently of reference-count or
allocator implementation.
