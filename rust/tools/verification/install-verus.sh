#!/usr/bin/env bash
set -euo pipefail

VERUS_VERSION="${VERUS_VERSION:-0.2026.09.27.3cf1832}"
RUST_TOOLCHAIN="${VERUS_RUST_TOOLCHAIN:-1.98.1}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
OUT="${VERUS_HOME:-$ROOT/target/verus}"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

rustup toolchain install "$RUST_TOOLCHAIN" --profile minimal

# The exact asset name is part of the pinned Verus release contract. Avoid the
# unauthenticated GitHub releases API here: shared CI runners can hit its rate
# limit even though the immutable release asset itself is still available.
asset_url="https://github.com/verus-lang/verus/releases/download/release/$VERUS_VERSION/verus-$VERUS_VERSION-x86-linux.zip"

curl --fail --silent --show-error --location "$asset_url" -o "$TMP/verus.zip"
unzip -q "$TMP/verus.zip" -d "$TMP/unpacked"

verus_bin="$(find "$TMP/unpacked" -type f -name verus -perm -111 | head -n1)"
if [[ -z "$verus_bin" ]]; then
  echo "Verus executable not found in release archive" >&2
  exit 1
fi

rm -rf "$OUT"
mkdir -p "$OUT"
cp -a "$(dirname "$verus_bin")/." "$OUT/"

"$OUT/verus" --version
