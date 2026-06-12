# md-editor-rust

Rust/Ratatui parity-port spike of the original Bun/OpenTUI/Solid [`md-editor`](../md-editor).

This is intentionally **not** a simplified rewrite. The goal is to preserve the shape and feel of Markdown Editor while testing whether a Rust/Ratatui architecture can keep the memory profile closer to Fresh.

## First slice parity

Implemented:

- Preview-first startup.
- `md-editor`-style app shell: header, main pane, outline/files sidebar, footer.
- CLI file open: `md-editor-rust README.md`.
- Preview scrolling.
- Edit mode with cursor movement, insertion, newline, backspace, tab.
- Save command.
- Undo/redo (`Ctrl+Z`/`Ctrl+Y` and `Ctrl+X u`/`Ctrl+X y`).
- Escape exits edit mode back to preview; `Ctrl+Q` quits anywhere.
- Outline extraction from markdown headings.
- File browser sidebar for `.md`, `.markdown`, `.mdx`.
- Help/sidebar collapse commands.
- Markdown styling for headings, blockquotes, lists, code fences, inline bold/code.
- JSON oracle commands for tests/fixtures: `outline`, `stats`, `classify`.
- RSS benchmark comparison against the Bun md-editor oracle.

Not implemented yet:

- Mermaid rendering.
- Theme picker / all bundled themes.
- Mouse parity.
- Exact OpenTUI visual details.
- Advanced markdown preview layout.

## Usage

```bash
cargo run -- fixtures/small.md
cargo run --release -- fixtures/large.md
```

Leader commands:

- `Ctrl+X e` edit
- `Ctrl+X p` preview
- `Ctrl+X f` files
- `Ctrl+X o` outline
- `Ctrl+X b` / `Ctrl+X r` collapse sidebar
- `Ctrl+X h` help
- `Ctrl+X u` undo
- `Ctrl+X y` redo
- `Ctrl+X s` save
- `Ctrl+X q` quit
- `Ctrl+Q` quit anywhere
- `Esc` exits edit mode to preview

## Oracle helpers

```bash
cargo run -- outline fixtures/large.md
cargo run -- stats fixtures/large.md
cargo run -- classify fixtures/small.md
```

## Benchmarks

Compare Rust port against the original Bun production build:

```bash
./bench/compare.sh fixtures/small.md
./bench/compare.sh fixtures/large.md
```

Current first-slice measurements on this machine:

| Fixture | Rust peak RSS | Bun oracle peak RSS |
| --- | ---: | ---: |
| `fixtures/small.md` | 16.54 MB | 97.03 MB |
| `fixtures/large.md` | 16.64 MB | 97.48 MB |

These numbers are a first-pass PTY/process-tree sample, not a rigorous final benchmark, but the direction is already very clear.
