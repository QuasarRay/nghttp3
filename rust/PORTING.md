# Rust reimplementation program

This tree preserves three independent artifacts:

1. **C reference** — the upstream nghttp3 C implementation and its history.
2. **Generated baseline** — reproducible C2Rust output. Generated code is not hand-edited.
3. **Verified/refactored Rust** — safe, idiomatic replacements introduced incrementally behind differential tests.

The merged `rust/nghttp3-sys` + `rust/nghttp3` crates remain the FFI oracle. They are not discarded when the independent Rust implementation appears.

## Invariants

- Every generated baseline records the exact nghttp3 commit and C2Rust revision.
- Generated output is regenerated, never manually repaired in place.
- A refactor must retain a differential path against the C implementation until its replacement has equivalent tests/proofs.
- Protocol semantics come from authoritative standards, not from accidental C behavior. The C implementation remains an executable compatibility oracle.
- HTTP/3/QPACK normative inputs are fetched from RFC Editor/IANA machine-readable sources.
- Runtime-independent protocol code must not gain a Tokio dependency. Future transport integration targets the ioxide/GenHTTP-style io_uring architecture.
- Unsafe C2Rust output is considered transitional. Unsafe regions are removed or isolated incrementally.
- Lambars may be used aggressively in the refactored layer, but never inside the frozen generated baseline.

## Stack

1. Reproducible C2Rust baseline and authoritative spec snapshots.
2. Kani + Verus + VerusBelt verification and regression-mining infrastructure.
3. Differential replacement of small subsystems, starting with pure/low-state code.
4. Lambars-driven compaction and metaprogramming after behavioral equivalence is established.
5. Connection/QPACK state-machine migration.
6. ioxide-rs integration without Tokio in the primary runtime path.

## First replacement candidates

Prefer components with compact state spaces and no transport scheduler coupling:

- varints / conversion helpers
- settings
- strings / byte helpers
- priority parsing
- QPACK static-table and Huffman primitives
- generic containers

Connection orchestration and stream scheduling remain later milestones because they carry the largest state space.
