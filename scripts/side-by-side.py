#!/usr/bin/env python3
import argparse, html, json, subprocess, sys
from pathlib import Path

parser = argparse.ArgumentParser(description="Render Bun oracle and Rust preview output side by side")
parser.add_argument("file")
parser.add_argument("--width", type=int, default=96)
parser.add_argument("--height", type=int, default=32)
parser.add_argument("--out", type=Path, default=Path("proofs/side-by-side.html"))
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
file_path = Path(args.file)
if not file_path.is_absolute():
    file_path = root / file_path

bun = subprocess.check_output([
    "bun", "run", str(root / "scripts/bun-oracle.ts"), "preview-lines", str(file_path), str(args.width), str(args.height)
], cwd=root, text=True)
rust = subprocess.check_output([
    str(root / "target/release/md-editor-rust"), "preview-lines", str(file_path), "--width", str(args.width), "--height", str(args.height)
], cwd=root, text=True)

bun_lines = json.loads(bun)["lines"]
rust_lines = json.loads(rust)["lines"]
max_len = max(len(bun_lines), len(rust_lines))
rows = []
diffs = 0
for i in range(max_len):
    left = bun_lines[i] if i < len(bun_lines) else ""
    right = rust_lines[i] if i < len(rust_lines) else ""
    same = left == right
    if not same:
        diffs += 1
    rows.append((i + 1, left, right, same))

args.out.parent.mkdir(parents=True, exist_ok=True)
text_out = args.out.with_suffix(".txt")
with text_out.open("w") as f:
    f.write(f"Side-by-side oracle: {file_path}\nwidth={args.width} height={args.height} diff_rows={diffs}/{max_len}\n\n")
    f.write(f"{'#':>3} | {'Bun oracle':<{args.width}} | Rust\n")
    f.write("-" * (args.width * 2 + 12) + "\n")
    for n, left, right, same in rows:
        mark = " " if same else "!"
        f.write(f"{n:>3}{mark}| {left:<{args.width}} | {right}\n")

html_rows = "\n".join(
    f"<tr class='{'' if same else 'diff'}'><td>{n}</td><td><pre>{html.escape(left)}</pre></td><td><pre>{html.escape(right)}</pre></td></tr>"
    for n, left, right, same in rows
)
args.out.write_text(f"""<!doctype html>
<html><head><meta charset='utf-8'><title>md-editor parity side-by-side</title>
<style>
body {{ background:#111318; color:#d8dee8; font-family: Inter, system-ui, sans-serif; margin: 24px; }}
h1 {{ color:#4ec9b0; }}
.meta {{ color:#8a95a7; margin-bottom:16px; }}
table {{ border-collapse: collapse; width: 100%; table-layout: fixed; }}
th {{ color:#9cdcfe; text-align:left; border-bottom:1px solid #3a4759; padding:8px; }}
td {{ vertical-align:top; border-bottom:1px solid #222b37; padding:4px 8px; }}
td:first-child {{ width:48px; color:#8a95a7; }}
pre {{ margin:0; white-space:pre-wrap; font-family:'JetBrains Mono', 'Fira Code', monospace; font-size:13px; line-height:1.35; }}
.diff {{ background:rgba(206,111,124,.13); }}
.legend {{ color:#c9a86a; }}
</style></head><body>
<h1>Markdown Editor Parity Side-by-Side</h1>
<div class='meta'>Fixture: <code>{html.escape(str(file_path))}</code><br>Width: {args.width}, Height: {args.height}, Diff rows: <span class='legend'>{diffs}/{max_len}</span></div>
<table><thead><tr><th>#</th><th>Bun/OpenTUI oracle</th><th>Rust/Ratatui</th></tr></thead><tbody>
{html_rows}
</tbody></table>
</body></html>
""")
print(json.dumps({"html": str(args.out), "text": str(text_out), "diffRows": diffs, "rows": max_len}, indent=2))
