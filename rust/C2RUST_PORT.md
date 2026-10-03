# Native Rust reimplementation plan

This repository keeps the existing `rust/nghttp3-sys` + `rust/nghttp3` crates as the FFI/differential oracle while a native Rust implementation is developed in stacked pull requests.

## Invariants

1. The C implementation under `lib/` remains the behavioral oracle until a native slice is independently verified.
2. The raw C2Rust snapshot is generated, not hand-edited.
3. Refactoring happens in later crates/branches so the original machine translation remains available for differential testing.
4. No Tokio dependency is introduced into the native HTTP/3 core. Runtime integration is kept outside the protocol implementation so the eventual ioxide-rs/GenHTTP-rs stack can supply its own executor/reactor.
5. Verification work is split by purpose:
   - Kani: bounded safety/correctness proofs and regression harnesses.
   - Verus: functional/state-machine invariants and refinement proofs.
   - VerusBelt: semantic foundation for the Verus proof-oriented Rust subset; it is not treated as a drop-in executable checker for arbitrary Rust.
6. Authoritative protocol inputs are IETF/RFC Editor/IANA artifacts. nghttp3 behavior and history are secondary differential/regression oracles.

## Stack

- **01-c2rust-bootstrap**: reproducible C2Rust translation pipeline and immutable generated baseline.
- **02-spec-and-regressions**: authoritative-source manifest, history-mined regression corpus, Kani harness framework.
- **03-verus-core**: proof-friendly pure protocol model and refinement boundary.
- **04-safe-core**: incrementally replace generated unsafe slices with safe idiomatic Rust.
- **05-lambars-metaprogramming**: compress repetitive protocol/state machinery with Lambars while preserving proofs.
- **06-ioxide-adapter**: executor-neutral transport API and ioxide-rs integration surface.

Each PR is based on the previous stack branch until merged. This keeps every intermediate oracle available and prevents later refactors from destroying the C2Rust baseline.

## C2Rust baseline

Run:

```sh
./scripts/c2rust-transpile.sh
```

The script:

- makes an isolated copy of the repository,
- configures a CMake compilation database for **libnghttp3 only**,
- uses Clang and disables optimization-oriented CMake build types,
- runs C2Rust against the compilation database,
- copies only generated Rust/build metadata into `rust/c2rust-generated/`,
- writes provenance and SHA-256 manifests.

The GitHub workflow pins the C2Rust source revision so regeneration does not silently change with upstream C2Rust development.
