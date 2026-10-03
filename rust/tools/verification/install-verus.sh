#!/usr/bin/env bash
set -euo pipefail

VERUS_VERSION="${VERUS_VERSION:-0.2026.09.27.3cf1832}"
RUST_TOOLCHAIN="${VERUS_RUST_TOOLCHAIN:-1.98.1}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
OUT="${VERUS_HOME:-$ROOT/target/verus}"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

rustup toolchain install "$RUST_TOOLCHAIN" --profile minimal

release_json="$TMP/release.json"
curl --fail --silent --show-error --location   "https://api.github.com/repos/verus-lang/verus/releases/tags/release%2F$VERUS_VERSION"   -o "$release_json"

asset_url="$(jq -r '[.assets[] | select(.name | test("x86.*linux.*\\.zip$"))][0].browser_download_url // empty' "$release_json")"

if [[ -z "$asset_url" ]]; then
  echo "Unable to locate the pinned Verus Linux release asset" >&2
  exit 1
fi

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
