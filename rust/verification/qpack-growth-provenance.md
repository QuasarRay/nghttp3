# QPACK buffer-growth verification provenance

This slice addresses historical commit
`8a8d45cb734eb8087773988cafee3a15906e7f52` ("Fix potential overflow").

- RFC 9204 remains authoritative for QPACK protocol behavior.
- The 2^31 allocation ceiling, minimum capacity 32, and power-of-two growth are
  nghttp3 implementation policy recovered from the preserved C source/history.
- The Rust implementation uses checked subtraction, checked addition, and
  `checked_next_power_of_two`; it never relies on shift width or wrapping.
- Kani proves successful growth is bounded, power-of-two, and sufficient for
  the requested free space, and that no-growth is an identity operation.
- Lambars generates concrete boundary and overflow regressions from shared
  predicates.
- The Verus model records the arithmetic obligation independently of allocator
  implementation details.
