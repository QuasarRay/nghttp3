#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
LEDGER="${REPLACEMENT_LEDGER:-$ROOT/rust/porting/replacements.tsv}"
GENERATED="${C2RUST_OUT_DIR:-$ROOT/rust/c2rust/generated}"
REPORT="${PORTING_REPORT:-$ROOT/target/porting-coverage.tsv}"

mkdir -p "$(dirname "$REPORT")"

header="$(head -n1 "$LEDGER")"
expected=$'id\tc_source\tc2rust_module\tsafe_module\tscope\tcoverage\tverification\tauthority\thistory\tintegration'
if [[ "$header" != "$expected" ]]; then
  echo "Unexpected replacement-ledger header" >&2
  exit 1
fi

duplicates="$(
  tail -n +2 "$LEDGER" |
    cut -f1 |
    sort |
    uniq -d
)"
if [[ -n "$duplicates" ]]; then
  echo "Duplicate replacement IDs:" >&2
  printf '%s\n' "$duplicates" >&2
  exit 1
fi

printf '%s\t%s\n' "$header" "validation" > "$REPORT"

while IFS=$'\t' read -r id c_source c2rust_module safe_module scope coverage verification authority history integration; do
  [[ -n "$id" ]] || continue

  if [[ ! -f "$ROOT/$c_source" ]]; then
    echo "[$id] missing reference source: $c_source" >&2
    exit 1
  fi

  if [[ ! -f "$ROOT/$safe_module" ]]; then
    echo "[$id] missing safe replacement: $safe_module" >&2
    exit 1
  fi

  generated_status="not-applicable"
  if [[ "$c2rust_module" != "-" ]]; then
    if [[ ! -f "$GENERATED/$c2rust_module" ]]; then
      echo "[$id] missing generated C2Rust module: $c2rust_module" >&2
      exit 1
    fi
    generated_status="present"
  fi

  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n'     "$id" "$c_source" "$c2rust_module" "$safe_module" "$scope" "$coverage"     "$verification" "$authority" "$history" "$integration" "$generated_status"     >> "$REPORT"
done < <(tail -n +2 "$LEDGER")

echo "Validated replacement ledger: $REPORT"
