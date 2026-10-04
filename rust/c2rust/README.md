# Frozen C2Rust baseline

This directory is the landing zone for the reproducible machine-generated Rust implementation baseline.

Do not hand-edit generated source. The generator writes a **self-contained crate** to `rust/c2rust/generated/` and emits `MANIFEST.txt` containing the nghttp3 source commit, C2Rust revision, compiler versions, translated-module count, and hashes for the complete preserved tree.

The generated baseline is intentionally separate from:

- `rust/nghttp3-sys`: FFI compatibility oracle
- `rust/nghttp3`: existing safe C-backed wrapper
- `rust/nghttp3-core`: incrementally verified safe replacements
- `rust/nghttp3-contracts`: Lambars-generated verification contracts

Regenerate with:

```sh
rust/tools/c2rust/transpile.sh
```

The script:

1. creates a CMake compilation database with `ENABLE_LIB_ONLY=ON`;
2. restricts translation to production `lib/*.c` translation units;
3. runs the pinned C2Rust revision;
4. copies every generated Rust module into `generated/source/lib/**`;
5. rewrites `c2rust-lib.rs` so it references only those preserved copies;
6. removes C2Rust's temporary adjacent `lib/*.rs` outputs;
7. fails if any external `../../../lib/` path remains;
8. hashes the entire self-contained generated crate.

CI additionally compiles that exact artifact with the nightly toolchain emitted by C2Rust. This prevents a workflow from reporting success when only the root wrapper was preserved while translated modules remained ephemeral in the checkout.

The generated crate is a **translation baseline, not a specification**. Safe replacements are admitted incrementally only after differential and/or formal verification against the relevant protocol/history contracts.
