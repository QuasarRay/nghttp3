#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
OUT="${SPEC_OUT_DIR:-$ROOT/target/http3-spec-snapshot}"
mkdir -p "$OUT"

fetch() {
  local id="$1" url="$2"
  curl --fail --location --silent --show-error "$url" -o "$OUT/$id.xml"
}

fetch rfc9000 https://www.rfc-editor.org/rfc/rfc9000.xml
fetch rfc9114 https://www.rfc-editor.org/rfc/rfc9114.xml
fetch rfc9204 https://www.rfc-editor.org/rfc/rfc9204.xml
fetch rfc9218 https://www.rfc-editor.org/rfc/rfc9218.xml
fetch rfc9220 https://www.rfc-editor.org/rfc/rfc9220.xml
fetch rfc9297 https://www.rfc-editor.org/rfc/rfc9297.xml
fetch rfc9412 https://www.rfc-editor.org/rfc/rfc9412.xml
fetch iana-http3 https://www.iana.org/assignments/http3-parameters/http3-parameters.xml
fetch iana-quic https://www.iana.org/assignments/quic/quic.xml

curl --fail --location --silent --show-error \
  https://www.iana.org/assignments/http3-parameters/http3-parameters-frame-types.csv \
  -o "$OUT/iana-http3-frame-types.csv"
curl --fail --location --silent --show-error \
  https://www.iana.org/assignments/http3-parameters/http3-parameters-settings.csv \
  -o "$OUT/iana-http3-settings.csv"
curl --fail --location --silent --show-error \
  https://www.iana.org/assignments/http3-parameters/http3-parameters-error-codes.csv \
  -o "$OUT/iana-http3-error-codes.csv"
curl --fail --location --silent --show-error \
  https://www.iana.org/assignments/http3-parameters/http3-parameters-stream-types.csv \
  -o "$OUT/iana-http3-stream-types.csv"

(
  cd "$OUT"
  sha256sum *.xml *.csv | sort > SHA256SUMS
)

echo "Authoritative source snapshot written to $OUT"
