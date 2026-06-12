#!/usr/bin/env bash
set -euo pipefail
if [ $# -lt 4 ]; then
  echo "usage: $0 <name> <duration-sec> <out-dir> -- <command...>" >&2
  exit 2
fi
name="$1"; dur="$2"; out="$3"; shift 3
[ "${1:-}" = "--" ] && shift
mkdir -p "$out"
echo "ms,total_rss_kb,process_count" > "$out/$name.csv"
("$@") >"$out/$name.stdout" 2>"$out/$name.stderr" & root=$!
start=$(date +%s%3N)
end=$(( $(date +%s) + dur ))
children(){ pgrep -P "$1" 2>/dev/null || true; }
desc(){
  local q=("$1") p c
  while [ ${#q[@]} -gt 0 ]; do
    p="${q[0]}"; q=("${q[@]:1}"); echo "$p"
    while read -r c; do [ -n "$c" ] && q+=("$c"); done < <(children "$p")
  done
}
while kill -0 "$root" 2>/dev/null; do
  total=0; count=0
  while read -r pid; do
    [ -r "/proc/$pid/status" ] || continue
    rss=$(awk '/^VmRSS:/ {print $2}' "/proc/$pid/status" 2>/dev/null || echo 0)
    total=$((total + ${rss:-0})); count=$((count+1))
  done < <(desc "$root")
  echo "$(( $(date +%s%3N)-start )),$total,$count" >> "$out/$name.csv"
  if [ $(date +%s) -ge $end ]; then
    pkill -TERM -P "$root" 2>/dev/null || true
    kill -TERM "$root" 2>/dev/null || true
    sleep .2
    pkill -KILL -P "$root" 2>/dev/null || true
    kill -KILL "$root" 2>/dev/null || true
    break
  fi
  sleep .1
done
wait "$root" 2>/dev/null || true
awk -F, 'NR>1{sum+=$2;n++;if($2>p)p=$2;vals[n]=$2}END{steady=vals[int(n*.7)]; printf "peak_rss_mb=%.2f\nsteady_rss_mb=%.2f\navg_rss_mb=%.2f\nsamples=%d\n", p/1024, steady/1024, (sum/n)/1024, n}' "$out/$name.csv" > "$out/$name.summary"
cat "$out/$name.summary"
