# Control-frame failure-path ownership provenance

Historical nghttp3 commit
`9bf7d87690171390a93a25326331e1682504223e` fixed a memory leak in
`nghttp3_conn_set_client_stream_priority`.

The C implementation allocated an encoded control-frame buffer, then called the
fallible control-stream frame-queue insertion routine. On insertion failure, the
function returned without freeing that buffer. The fix added the missing free.

The Rust model uses an owned `PendingControlFrame`:

1. the pending frame uniquely owns its `Vec<u8>`;
2. a fallible admission callback runs while ownership remains pending;
3. failure returns `AdmissionFailure`, which still owns the frame and therefore
   releases it automatically if the caller returns/drops the error;
4. only successful admission moves the bytes into `ControlFrameQueue`.

No explicit failure-path free is required and there is no raw allocation to
forget.

This is an implementation resource-lifetime invariant rather than an HTTP/3
wire-protocol requirement. Protocol-level Priority semantics remain governed by
RFC 9218 and the relevant HTTP/3 standards.
