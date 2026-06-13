#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
ORACLE=${ORACLE:-/home/shamanic/dev/md-editor}
FIXTURE=${1:-$ROOT/fixtures/small.md}
DUR=${DUR:-5}
STAMP=$(date +%Y%m%d-%H%M%S)
OUT="$ROOT/bench/results/$STAMP"
mkdir -p "$OUT"

cargo build --release --manifest-path "$ROOT/Cargo.toml" >/tmp/poneglyph-build.log

RUST_BIN="$ROOT/target/release/poneglyph"
ORACLE_PROD="$ORACLE/dist/tui/tui/main.js"
ORACLE_PRELOAD="$ORACLE/scripts/preload-solid-client.ts"

{
  echo "# poneglyph vs Bun md-editor oracle"
  echo
  echo "- fixture: $FIXTURE"
  echo "- oracle: $ORACLE"
  echo "- rust binary: $RUST_BIN"
  echo "- date: $(date -Iseconds)"
  echo
} > "$OUT/summary.md"

"$ROOT/bench/profile_rss.sh" rust "$DUR" "$OUT" -- script -q -e -c "timeout -s INT $((DUR-1))s '$RUST_BIN' '$FIXTURE'" /dev/null | sed 's/^/rust /'
cat "$OUT/rust.summary" >> "$OUT/summary.md"

if [ -f "$ORACLE_PROD" ]; then
  "$ROOT/bench/profile_rss.sh" bun-oracle "$DUR" "$OUT" -- script -q -e -c "timeout -s INT $((DUR-1))s bun --preload '$ORACLE_PRELOAD' '$ORACLE_PROD' '$FIXTURE'" /dev/null | sed 's/^/bun /'
  {
    echo
    echo "## Bun oracle"
    cat "$OUT/bun-oracle.summary"
  } >> "$OUT/summary.md"
else
  echo "Oracle dist missing at $ORACLE_PROD; run bun run build in oracle if you want prod comparison." | tee -a "$OUT/summary.md"
fi

echo
cat "$OUT/summary.md"
echo "Wrote $OUT"
