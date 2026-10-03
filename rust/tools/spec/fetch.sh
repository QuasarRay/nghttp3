#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
OUT="${SPEC_OUT_DIR:-$ROOT/target/http3-spec-snapshot}"
mkdir -p "$OUT"

fetch() {
  local id="$1" url="$2"
  curl --fail --location --silent --show-error "$url" -o "$OUT/$id.xml"
}

fetch rfc9114 https://www.rfc-editor.org/rfc/rfc9114.xml
fetch rfc9204 https://www.rfc-editor.org/rfc/rfc9204.xml
fetch rfc9218 https://www.rfc-editor.org/rfc/rfc9218.xml
fetch rfc9220 https://www.rfc-editor.org/rfc/rfc9220.xml
fetch rfc9297 https://www.rfc-editor.org/rfc/rfc9297.xml
fetch iana-http3 https://www.iana.org/assignments/http3-parameters/http3-parameters.xml

(
  cd "$OUT"
  sha256sum *.xml | sort > SHA256SUMS
)

echo "Authoritative source snapshot written to $OUT"
