#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
OUT="${1:-$ROOT/target/regression-mining/candidates.tsv}"
mkdir -p "$(dirname "$OUT")"

printf 'commit\tdate\tcategory\tsubject\tfiles\n' > "$OUT"

git -C "$ROOT" log --all --no-merges   --regexp-ignore-case --extended-regexp   --grep='(fix|bug|crash|overflow|underflow|use.?after.?free|double.?free|null.?deref|memory.?corrupt|leak|fuzz|asan|ubsan)'   --format='%H%x09%aI%x09%s' |
while IFS=$'\t' read -r sha date subject; do
  files="$(git -C "$ROOT" diff-tree --no-commit-id --name-only -r "$sha" |
    grep -E '^(lib|tests|fuzz)/' || true)"
  [[ -n "$files" ]] || continue

  lower="$(printf '%s' "$subject" | tr '[:upper:]' '[:lower:]')"
  case "$lower" in
    *double*free*|*use*after*free*) category="ownership" ;;
    *memory*corrupt*|*asan*|*ubsan*|*crash*) category="memory-safety" ;;
    *overflow*|*underflow*) category="arithmetic" ;;
    *null*deref*) category="null-deref" ;;
    *leak*) category="resource-leak" ;;
    *fuzz*) category="fuzz" ;;
    *) category="bug-fix" ;;
  esac

  joined="$(printf '%s\n' "$files" | paste -sd, -)"
  printf '%s\t%s\t%s\t%s\t%s\n' "$sha" "$date" "$category" "$subject" "$joined" >> "$BODY"
done

{
  printf 'commit\tdate\tcategory\tsubject\tfiles\n'
  sort -t\t' -k2,2 -k1,1 "$BODY"
} > "$OUT"
echo "Historical regression candidates: $OUT"
