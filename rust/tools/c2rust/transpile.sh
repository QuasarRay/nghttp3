#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
BUILD="${C2RUST_BUILD_DIR:-$ROOT/target/c2rust-c-build}"
OUT="${C2RUST_OUT_DIR:-$ROOT/rust/c2rust/generated}"
C2RUST_REV="${C2RUST_REV:-92e37649e5c278746697e42d8427722221046546}"

command -v cmake >/dev/null
command -v jq >/dev/null
command -v cargo >/dev/null
command -v clang >/dev/null

if ! command -v c2rust >/dev/null; then
  cargo install --locked --git https://github.com/immunant/c2rust.git --rev "$C2RUST_REV" c2rust
fi

rm -rf "$BUILD" "$OUT"
mkdir -p "$BUILD" "$OUT"

cmake -S "$ROOT" -B "$BUILD"   -DCMAKE_BUILD_TYPE=None   -DCMAKE_C_COMPILER=clang   -DCMAKE_EXPORT_COMPILE_COMMANDS=ON   -DENABLE_LIB_ONLY=ON   -DENABLE_STATIC_LIB=ON   -DENABLE_SHARED_LIB=OFF   -DBUILD_TESTING=OFF

cmake --build "$BUILD" --target nghttp3_static -j"$(getconf _NPROCESSORS_ONLN 2>/dev/null || echo 2)"

# C2Rust requires a compilation database named exactly compile_commands.json.
# Keep only production library translation units; tests/examples are verification
# inputs, not part of the generated implementation.
jq --arg root "$ROOT/lib/"   '[.[] | select(.file | startswith($root)) | select(.file | endswith(".c"))]'   "$BUILD/compile_commands.json" > "$OUT/compile_commands.json"

(
  cd "$OUT"
  c2rust transpile --emit-build-files compile_commands.json
)

{
  echo "nghttp3_commit=$(git -C "$ROOT" rev-parse HEAD)"
  echo "c2rust_rev=$C2RUST_REV"
  echo "rustc=$(rustc --version)"
  echo "clang=$(clang --version | head -n1)"
  echo "cmake=$(cmake --version | head -n1)"
  echo "compile_commands_sha256=$(sha256sum "$OUT/compile_commands.json" | cut -d' ' -f1)"
  find "$OUT" -type f ! -name MANIFEST.txt -print0 |
    sort -z |
    xargs -0 sha256sum
} > "$OUT/MANIFEST.txt"

echo "C2Rust baseline written to $OUT"
