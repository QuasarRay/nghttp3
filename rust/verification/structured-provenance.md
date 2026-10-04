# Structured Fields / Priority verification provenance

This slice preserves the historical `aed3107f9104eae77d97ed8093caa8f3b7ef64d4`
parser-boundary fix while moving parser state into safe Rust slices.

- RFC 9218 is authoritative for HTTP Priority members `u` and `i`.
- RFC 9651 is the current Structured Fields authority.
- RFC 8941 is retained as the historical Structured Fields authority that was
  current when the 2021 regression was fixed.
- The original C commit and its tests are a compatibility/regression oracle,
  not a replacement for RFC semantics.
- `nghttp3::parse_priority_oracle` is intentionally a narrow synchronous FFI
  boundary used only for differential verification.
- The safe core never exposes raw pointers and never reads after terminal `=`.
- Advanced Structured Fields constructs not needed by this RFC 9218 slice are
  rejected and can be added in later stack layers with independent proofs.
