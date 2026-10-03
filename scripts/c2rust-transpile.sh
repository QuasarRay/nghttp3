#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
C2RUST_BIN="${C2RUST_BIN:-c2rust}"
OUT="${C2RUST_OUT:-$ROOT/rust/c2rust-generated}"
WORK="${C2RUST_WORK:-$(mktemp -d)}"
KEEP_WORK="${C2RUST_KEEP_WORK:-0}"

cleanup() {
  if [[ "$KEEP_WORK" != "1" ]]; then
    rm -rf "$WORK"
  else
    printf 'C2Rust worktree retained at %s\n' "$WORK" >&2
  fi
}
trap cleanup EXIT

command -v cmake >/dev/null
command -v clang >/dev/null
command -v rsync >/dev/null
command -v "$C2RUST_BIN" >/dev/null

SRC="$WORK/src"
BUILD="$WORK/build"

mkdir -p "$SRC"
rsync -a \
  --exclude='.git' \
  --exclude='build' \
  --exclude='rust/c2rust-generated' \
  "$ROOT/" "$SRC/"

cmake -S "$SRC" -B "$BUILD" \
  -DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
  -DCMAKE_BUILD_TYPE=None \
  -DCMAKE_C_COMPILER=clang \
  -DENABLE_LIB_ONLY=ON \
  -DBUILD_TESTING=OFF \
  -DENABLE_SHARED_LIB=OFF \
  -DENABLE_STATIC_LIB=ON

# Build once so generated config/version headers referenced by compile_commands exist.
cmake --build "$BUILD" --target nghttp3_static --parallel 2

# C2Rust emits translated files beside the copied C sources. Never run it on the
# real lib/ tree: the generated baseline must be reproducible and upstream C must
# stay byte-for-byte untouched.
(
  cd "$SRC"
  "$C2RUST_BIN" transpile --emit-build-files "$BUILD/compile_commands.json"
)

rm -rf "$OUT"
mkdir -p "$OUT/source"

# The C translation units in compile_commands live under lib/. The source tree
# already contains a separate hand-written rust/ workspace, so copying every
# *.rs file from the temporary tree would contaminate the machine-generated
# snapshot with pre-existing Rust. Upstream lib/ contains no Rust source; every
# *.rs file appearing there after transpilation is therefore C2Rust output.
while IFS= read -r -d '' file; do
  rel="${file#"$SRC/"}"
  mkdir -p "$OUT/source/$(dirname "$rel")"
  cp "$file" "$OUT/source/$rel"
done < <(find "$SRC/lib" -type f -name '*.rs' -print0)

# Preserve build metadata if C2Rust emits it either at repository or lib scope.
for file in Cargo.toml Cargo.lock build.rs; do
  if [[ -f "$SRC/$file" ]]; then
    cp "$SRC/$file" "$OUT/$file"
  fi
  if [[ -f "$SRC/lib/$file" ]]; then
    cp "$SRC/lib/$file" "$OUT/lib-$file"
  fi
done

source_commit="unknown"
if command -v git >/dev/null && git -C "$ROOT" rev-parse HEAD >/dev/null 2>&1; then
  source_commit="$(git -C "$ROOT" rev-parse HEAD)"
fi

c2rust_version="$("$C2RUST_BIN" --version 2>/dev/null || true)"

cat > "$OUT/PROVENANCE.txt" <<EOF
source_repository=https://github.com/QuasarRay/nghttp3
source_commit=$source_commit
generator=$c2rust_version
compiler=$(clang --version | head -n 1)
cmake=$(cmake --version | head -n 1)
configuration=ENABLE_LIB_ONLY=ON;BUILD_TESTING=OFF;ENABLE_SHARED_LIB=OFF;ENABLE_STATIC_LIB=ON;CMAKE_BUILD_TYPE=None
scope=lib/
EOF

(
  cd "$OUT"
  find . -type f ! -name SHA256SUMS -print0 |
    sort -z |
    xargs -0 sha256sum > SHA256SUMS
)

count="$(find "$OUT/source/lib" -type f -name '*.rs' | wc -l)"
if [[ "$count" -eq 0 ]]; then
  echo "C2Rust produced no Rust files under lib/" >&2
  exit 1
fi

printf 'Generated %s C-derived Rust files in %s\n' "$count" "$OUT"
