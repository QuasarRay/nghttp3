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

cmake -S "$ROOT" -B "$BUILD" \
  -DCMAKE_BUILD_TYPE=None \
  -DCMAKE_C_COMPILER=clang \
  -DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
  -DENABLE_LIB_ONLY=ON \
  -DENABLE_STATIC_LIB=ON \
  -DENABLE_SHARED_LIB=OFF \
  -DBUILD_TESTING=OFF

cmake --build "$BUILD" --target nghttp3_static -j"$(getconf _NPROCESSORS_ONLN 2>/dev/null || echo 2)"

# C2Rust requires a compilation database named exactly compile_commands.json.
# Keep only production library translation units; tests/examples remain
# verification inputs rather than implementation modules.
jq --arg root "$ROOT/lib/" \
  '[.[] | select(.file | startswith($root)) | select(.file | endswith(".c"))]' \
  "$BUILD/compile_commands.json" > "$OUT/compile_commands.json"

(
  cd "$OUT"
  c2rust transpile --emit-build-files compile_commands.json
)

# C2Rust writes translated modules next to the original C translation units,
# while c2rust-lib.rs references them through ../../../lib/*.rs. Preserve those
# modules inside the artifact and rewrite the root module to be self-contained.
mapfile -t GENERATED_RS < <(
  jq -r '.[].file | sub("\\.c$"; ".rs")' "$OUT/compile_commands.json" | sort -u
)

for src in "${GENERATED_RS[@]}"; do
  if [[ ! -f "$src" ]]; then
    echo "Expected C2Rust output is missing: $src" >&2
    exit 1
  fi

  rel="${src#"$ROOT/"}"
  dst="$OUT/source/$rel"
  mkdir -p "$(dirname "$dst")"
  cp "$src" "$dst"
done

# The generated crate must resolve only files stored inside the artifact.
sed -i 's#../../../lib/#source/lib/#g' "$OUT/c2rust-lib.rs"

# Remove temporary adjacent outputs so the compile check below cannot
# accidentally succeed by reading files outside the preserved artifact.
for src in "${GENERATED_RS[@]}"; do
  rm -f "$src"
done

if grep -R --line-number --fixed-strings '../../../lib/' "$OUT"; then
  echo "Generated crate still contains repository-relative lib paths" >&2
  exit 1
fi

{
  echo "nghttp3_commit=$(git -C "$ROOT" rev-parse HEAD)"
  echo "c2rust_rev=$C2RUST_REV"
  echo "rustc=$(rustc --version)"
  echo "clang=$(clang --version | head -n1)"
  echo "cmake=$(cmake --version | head -n1)"
  echo "compile_commands_sha256=$(sha256sum "$OUT/compile_commands.json" | cut -d' ' -f1)"
  echo "translated_modules=${#GENERATED_RS[@]}"
  find "$OUT" -type f ! -name MANIFEST.txt -print0 |
    sort -z |
    xargs -0 sha256sum
} > "$OUT/MANIFEST.txt"

echo "Self-contained C2Rust baseline written to $OUT"
