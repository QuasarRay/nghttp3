# PRIORITY_UPDATE failure-path ownership provenance

This layer captures historical commit
`9bf7d87690171390a93a25326331e1682504223e` ("Fix memory leak on the failure path").

The C implementation copied PRIORITY_UPDATE payload bytes into a separately
allocated buffer before trying to reserve a control-stream frame. When frame
reservation failed, that buffer had to be explicitly freed before returning.

The Rust API separates ownership states:

- `PendingPriorityUpdate` owns the copied payload before queue reservation;
- reservation success consumes the pending value and produces
  `QueuedPriorityUpdate`;
- reservation failure returns `EnqueueError<E>`, which contains the complete
  pending value and therefore retains exactly one owner;
- if the caller discards that error, normal Rust drop semantics release the
  payload automatically.

RFC 9218 defines Priority semantics and RFC 9114 defines HTTP/3 framing context.
The exact failure-path ownership rule is an nghttp3 implementation-safety
invariant mined from repository history.

Kani proves both success and failure preserve stream ID and symbolic payload
bytes. Lambars generates the permanent historical failure regression. Verus
records that both transitions preserve exactly one abstract payload owner.
