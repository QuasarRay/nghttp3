#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST="${SPEC_SOURCE_MANIFEST:-$ROOT/verification/spec-sources.toml}"
OUT="${SPEC_SNAPSHOT_DIR:-$ROOT/verification/spec-snapshots}"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

command -v curl >/dev/null
command -v sha256sum >/dev/null
command -v awk >/dev/null

rm -rf "$OUT"
mkdir -p "$OUT"

# The manifest is deliberately constrained to one id/url/format triple per
# [[source]] block. Parse only those exact keys; reject incomplete blocks.
awk -F'"' '
  /^\[\[source\]\]/ {
    if (id != "" || url != "" || fmt != "") {
      if (id == "" || url == "" || fmt == "") exit 2;
      print id "\t" url "\t" fmt;
    }
    id=url=fmt="";
    next;
  }
  /^id = "/ { id=$2; next }
  /^url = "/ { url=$2; next }
  /^format = "/ { fmt=$2; next }
  END {
    if (id == "" || url == "" || fmt == "") exit 2;
    print id "\t" url "\t" fmt;
  }
' "$MANIFEST" > "$TMP/sources.tsv"

while IFS=$'\t' read -r id url fmt; do
  case "$fmt" in
    xml|csv) ;;
    *) echo "unsupported authoritative format: $fmt" >&2; exit 1 ;;
  esac

  target="$OUT/$id.$fmt"
  echo "snapshot: $id <- $url"
  curl     --fail     --silent     --show-error     --location     --proto '=https'     --tlsv1.2     --retry 4     --retry-all-errors     --connect-timeout 20     --max-time 120     -H 'Accept: application/xml,text/xml,text/csv,text/plain,*/*'     "$url"     -o "$target"

  test -s "$target"
done < "$TMP/sources.tsv"

cp "$MANIFEST" "$OUT/SOURCES.toml"

(
  cd "$OUT"
  find . -type f ! -name SHA256SUMS -print0 |
    sort -z |
    xargs -0 sha256sum > SHA256SUMS
)

{
  printf 'snapshot_format=1\n'
  printf 'source_count=%s\n' "$(wc -l < "$TMP/sources.tsv" | tr -d ' ')"
  printf 'generated_utc=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
} > "$OUT/PROVENANCE.txt"

echo "snapshotted $(wc -l < "$TMP/sources.tsv" | tr -d ' ') authoritative artifacts"
