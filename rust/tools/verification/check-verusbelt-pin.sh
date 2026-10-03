#!/usr/bin/env bash
set -euo pipefail

REV="${VERUSBELT_REV:-b9b14b2cb47b6a6307611578782adc3481627aac}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
OUT="${VERUSBELT_HOME:-$ROOT/target/verus-belt}"

rm -rf "$OUT"
git clone --quiet --filter=blob:none https://github.com/verus-lang/verus-belt.git "$OUT"
git -C "$OUT" checkout --quiet "$REV"

test "$(git -C "$OUT" rev-parse HEAD)" = "$REV"
test -f "$OUT/src/lambda_verus/typing/soundness.v"
test -f "$OUT/src/lambda_verus/lifetime/lifetime_full.v"

echo "Pinned VerusBelt semantic foundation: $REV"
