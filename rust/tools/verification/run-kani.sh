#!/usr/bin/env bash
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
OUT="${KANI_COUNTEREXAMPLE_DIR:-$ROOT/target/kani-counterexamples}"
mkdir -p "$OUT"

set +e
cargo kani "$@" 2>&1 | tee "$OUT/verification.log"
status=${PIPESTATUS[0]}
set -e

if [[ "$status" -eq 0 ]]; then
  rm -f "$OUT/concrete-playback.log"
  exit 0
fi

# Kani concrete playback turns failing assertion counterexamples into ordinary
# Rust unit-test bodies. Keep the generated source text as a CI artifact; after
# triage, promote every relevant counterexample into the normal regression
# suite next to its proof harness.
set +e
cargo kani "$@" -Z concrete-playback --concrete-playback=print   2>&1 | tee "$OUT/concrete-playback.log"
playback_status=${PIPESTATUS[0]}
set -e

printf 'verification_exit=%s\nplayback_exit=%s\n'   "$status" "$playback_status" > "$OUT/status.txt"

exit "$status"
