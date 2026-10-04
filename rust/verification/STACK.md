# Verified Rust port stack

This is the recovery/checkpoint document for the incremental nghttp3 Rust
reimplementation. Each layer is intentionally reviewable and mergeable in
order. Lower layers are treated as immutable foundations except for narrowly
scoped CI/proof fixes, which are propagated upward without replacing later work.

## Merge order

| PR | Branch | Purpose |
| --- | --- | --- |
| #3 | `rust-port/01-c2rust-baseline` | Reproducible C2Rust/native/C-oracle baseline and authoritative spec snapshot |
| #4 | `rust-port/02-verification-varint` | Safe QUIC varints, C differential oracle, Kani, Verus, VerusBelt |
| #5 | `rust-port/03-history-regressions` | Repository-history mining and permanent regression manifest |
| #8 | `rust-port/04-lambars-contracts` | Lambars-generated runtime/Kani contract layer; explicit no-Tokio rule |
| #9 | `rust-port/05-settings-semantics` | Safe owned versioned settings semantics and C differential defaults |
| #10 | `rust-port/06-safe-ringbuf` | Unsafe-free ring buffer; 2019 memory-corruption regression |
| #11 | `rust-port/07-safe-priority-parser` | Safe RFC 9218/Structured Fields slice; 2021 stack-overflow regression |
| #12 | `rust-port/08-qpack-growth` | Checked QPACK buffer-growth arithmetic; 2024 overflow regression |
| #13 | `rust-port/09-qpack-decoder-ownership` | Explicit transient decoder ownership; 2025 null-dereference/OOM regression |
| #14 | `rust-port/10-qpack-publication-order` | Registration-before-publication; 2025 double-free/OOM regression |
| #15 | `rust-port/11-priority-update-ownership` | Owned PRIORITY_UPDATE failure path; 2026 leak regression |
| #16 | `rust-port/12-owned-qpack-input` | Owned retained QPACK request input; 2026 ASAN lifetime regression |

## Verification roots

- Protocol/reference inputs live in `rust/spec/sources.toml` and are fetched as
  machine-readable RFC/IANA XML/CSV snapshots.
- C remains a differential compatibility oracle while safe replacements are
  introduced incrementally.
- Kani is the executable bounded/model-checking regression layer.
- Verus is the functional/state-invariant proof layer.
- VerusBelt is pinned as the semantic foundation for future unsafe/low-level
  reasoning.
- Lambars generates repeated runtime/Kani proof shapes from shared predicates.
- Historical commits are tracked in
  `rust/verification/history/regressions.toml`.

## Runtime architecture rule

The protocol core and verification crates remain runtime-independent. Do not
introduce Tokio into the ioxide integration path. Lambars' async feature remains
disabled. Transport-driving code must eventually sit behind a completion-driven
interface compatible with the shared-nothing/per-thread design of ioxide and a
modular GenHTTP-style host/handler boundary.

## Refactoring rule

Do not replace the C2Rust baseline wholesale after safe work begins. Migrate one
semantic slice at a time:

1. identify authoritative RFC/IANA behavior and relevant historical bugs;
2. preserve a narrow C differential oracle where compatibility evidence helps;
3. implement an unsafe-free/runtime-free Rust slice;
4. add concrete historical regression tests;
5. add Kani symbolic proofs;
6. add Verus state/functional proofs;
7. express repeated obligations through Lambars;
8. only then replace callers and retire the corresponding generated/C-backed
   path.

## Next slices

Highest-value remaining work is deeper protocol coverage rather than more
infrastructure:

1. integrate the safe ownership/publication/input primitives into a real QPACK
   encoder/decoder state-machine slice;
2. reimplement QPACK integer/string/Huffman decoding with RFC 9204 oracles and
   differential C tests;
3. reimplement HTTP/3 frame parsing and stream state transitions from RFC 9114;
4. progressively move connection logic behind runtime-independent transport
   traits suitable for ioxide;
5. retain C2Rust/C as an oracle until each corresponding safe slice has
   differential tests and formal obligations.

Every new slice should be another stacked PR based on the exact current head of
the preceding branch.
