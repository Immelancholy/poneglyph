# poneglyph

A tiny, beautiful terminal markdown editor for ancient texts and modern notes.

`poneglyph` is the Rust/Ratatui successor to the original Bun/OpenTUI/Solid `md-editor`. It keeps the markdown-first TUI workflow, but ships as a tiny native binary with dramatically lower memory use.

Current local release build:

| Artifact | Size / memory |
| --- | ---: |
| Release binary | ~924 KB |
| Runtime peak RSS | ~17 MB |
| Bun oracle peak RSS | ~97 MB |

## Features

- Preview-first markdown reading.
- In-place edit mode.
- Outline sidebar and file browser.
- Theme picker with bundled theme loading.
- Boxed or smooth chrome.
- Configurable cursor style and theme swatches.
- Rich preview rendering for headings, blockquotes, nested lists, code blocks, tables, links, and images.
- Save, undo/redo, Delete/Backspace editing, and file opening.
- Oracle/debug commands for parity and snapshot testing.

Accepted enhancements over the old Bun app:

- Boxed table rendering instead of raw pipe-table lines.
- Compact square theme swatches in the theme picker.
- Richer nested list bullets.

## Usage

```bash
cargo run -- fixtures/small.md
cargo run --release -- fixtures/large.md

# after release build
./target/release/poneglyph README.md
```

Direct mode keys:

- `Ctrl+E` edit mode
- `Ctrl+V` view mode
- `Ctrl+F` files mode
- `Ctrl+Q` quit anywhere
- `Ctrl+S` save anywhere
- `Ctrl+Z` / `Ctrl+Y` undo / redo

View mode commands after `Ctrl+V`:

- `o` outline
- `r` collapse/expand sidebar
- `t` theme picker
- `b` boxed/smooth chrome
- `c` cursor style

Edit mode:

- Type to insert text.
- Arrows/Home/End/PageUp/PageDown move.
- `Enter` newline.
- `Backspace` delete backward.
- `Delete` delete forward.
- `Esc` exits back to preview.

Legacy `Ctrl+X` commands remain available for compatibility.

## Configuration

Default config path:

```text
~/.config/poneglyph/config.toml
```

Project-local config is also supported:

```text
.poneglyph.toml
```

Example:

```toml
[ui]
theme = "tokyo-night"
cursorStyle = "block"      # brackets | block | bar | underline | box
boxedChrome = true
themeSwatches = "square"   # off | circle | square
themeSwatchSpacing = 0      # 0..8
```

You can override config path for testing:

```bash
PONEGLYPH_CONFIG=/tmp/poneglyph.toml poneglyph README.md
```

## Oracle/debug helpers

```bash
cargo run -- outline fixtures/large.md
cargo run -- stats fixtures/large.md
cargo run -- classify fixtures/small.md
cargo run -- preview-lines fixtures/small.md --width 96 --height 32
cargo run -- sidebar-lines fixtures/small.md --files
cargo run -- state-after-keys fixtures/small.md 'ctrl+e,right,delete'
```

Side-by-side parity helpers compare `poneglyph` against the old Bun oracle:

```bash
./scripts/side-by-side.py fixtures/preview-rich.md --out proofs/side-by-side-preview-rich.html
./scripts/structural-parity.py fixtures/small.md fixtures/preview-rich.md fixtures/parity-structural.md
```

## Benchmarks

Compare `poneglyph` against the original Bun production oracle:

```bash
./bench/compare.sh fixtures/small.md
./bench/compare.sh fixtures/large.md
```

Latest local result:

```text
poneglyph peak RSS: ~17 MB
Bun oracle peak RSS: ~97 MB
```
