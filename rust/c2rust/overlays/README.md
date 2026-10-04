# C2Rust safe overlays

Files in this directory replace complete C2Rust-generated translation units at
artifact-generation time.

They are **ABI edges**, not the proof-friendly implementation layer:

- unavoidable raw-pointer operations needed to preserve the C ABI stay here;
- protocol/domain semantics delegate into verbatim copies from
  `rust/nghttp3-core`;
- every activated overlay must be recorded in
  `rust/porting/replacements.tsv`;
- the generated artifact is compile-checked after overlays are applied.

## nghttp3_conv.rs

The first overlay replaces C2Rust's translation of `lib/nghttp3_conv.c`.

QUIC variable-integer encode/decode/length operations delegate to the verified
`nghttp3-core/src/varint.rs` implementation copied into the artifact as
`safe/varint.rs`.

Endian writes and stream-ID ordinal conversion are kept as small ABI-compatible
glue. Their presence does not expand the formal-verification claim recorded for
the `quic-varint` ledger row.
