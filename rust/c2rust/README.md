# Frozen C2Rust baseline

This directory is the landing zone for reproducible machine-generated Rust.

Do not hand-edit generated source. The generator writes to `rust/c2rust/generated/` and emits `MANIFEST.txt` containing the nghttp3 source commit, C2Rust revision, compiler versions, and hashes of the compilation database and generated files.

The generated baseline is intentionally separate from:

- `rust/nghttp3-sys`: FFI oracle
- `rust/nghttp3`: existing safe wrapper
- future verified/refactored crates

Regenerate with:

```sh
rust/tools/c2rust/transpile.sh
```

The script uses the repository CMake build with `ENABLE_LIB_ONLY=ON` and an unoptimized compilation database, then restricts translation to `lib/*.c`.
