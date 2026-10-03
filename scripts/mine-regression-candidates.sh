#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="${REGRESSION_CANDIDATES_OUT:-$ROOT/verification/history-candidates.tsv}"

cd "$ROOT"

# Complete history is required. CI checks out with fetch-depth: 0.
if git rev-parse --is-shallow-repository | grep -qx true; then
  echo "refusing to mine a shallow clone" >&2
  exit 1
fi

pattern='fix|bug|regression|crash|segfault|asan|ubsan|overflow|underflow|leak|double[- ]free|use[- ]after|out[- ]of[- ]bounds|oob|invalid|assert|fuzz|undefined behavior|\bub\b|error handling|security'

{
  printf 'commit\tdate\tsubject\n'
  git log --all --reverse --date=iso-strict --format='%H%x09%cI%x09%s' |
    grep -Eai "$pattern" || true
} > "$OUT"

echo "mined $(( $(wc -l < "$OUT") - 1 )) history candidates into $OUT"
