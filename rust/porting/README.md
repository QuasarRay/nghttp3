# Rust porting replacement ledger

`replacements.tsv` is the machine-readable inventory connecting the C/C++
reference tree, the frozen C2Rust translation, and verified safe Rust
replacements.

It exists to prevent two failure modes:

1. **duplicate work** — reimplementing a subsystem that already has a verified
   safe replacement; and
2. **false completion** — treating a safe parallel module as if it had already
   replaced the corresponding C2Rust-generated implementation.

## Columns

- `id`: stable replacement identifier.
- `c_source`: original authoritative compatibility-oracle source file.
- `c2rust_module`: path inside the self-contained generated artifact, or `-`
  when the source is intentionally outside C2Rust's production C input.
- `safe_module`: verified hand/refactor destination in `nghttp3-core`.
- `scope`: exact function/semantic slice already replaced.
- `coverage`: whether the row covers a whole source module or only a slice.
- `verification`: independent checks currently attached.
- `authority`: protocol/history sources governing the slice.
- `history`: historical regression commit when applicable.
- `integration`: whether the safe implementation has actually been wired into
  the generated/runtime implementation.

## Critical rule

`not-wired` means exactly that: the safe module exists and is verified in
parallel, but the C2Rust-generated implementation is still the generated
baseline. A row must not be promoted to an integrated state until the generated
crate or higher runtime actually dispatches through the safe replacement and
differential/formal checks remain green.

## Validation

After C2Rust generation:

```sh
bash rust/tools/c2rust/check-replacements.sh
```

The checker verifies that:

- every original source file still exists;
- every declared C2Rust module exists inside the self-contained generated
  artifact;
- every declared safe replacement exists;
- replacement IDs are unique.

It emits `target/porting-coverage.tsv` for CI/artifact inspection.
