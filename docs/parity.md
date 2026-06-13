# poneglyph parity notes

Oracle: the archived Bun/OpenTUI/Solid project at `/home/shamanic/dev/md-editor`.

`poneglyph` is now the Rust/Ratatui successor. The old Bun app remains useful as an oracle for behavior and rendering checks, but the Rust app is the publishing target.

## Parity goals

- Preserve the same product shape: header, main editor/preview pane, outline/files sidebar, footer.
- Default to preview mode.
- Keep direct mode keys (`Ctrl+E`, `Ctrl+V`, `Ctrl+F`) and legacy `Ctrl+X` compatibility.
- Support opening markdown files from CLI.
- Support preview scrolling, edit mode cursor movement, text insertion/deletion, save, undo/redo.
- Parse markdown headings/lists/quotes/code blocks/inline styling into styled terminal spans.
- Provide fixture tests for markdown classification, outline extraction, wrapping, editing, and file stats.
- Provide benchmark scripts comparing RSS against the Bun oracle.

## Accepted Rust enhancements

These are intentional divergences, not parity bugs:

- Boxed table rendering instead of raw pipe-table markdown.
- Compact square theme swatches in the theme picker.
- Richer nested list bullets.
- Stronger contrast guards for low-contrast themes.

## Remaining post-rename checks

- Publishing/release packaging.
- Config schema documentation hardening.
- Optional mouse support.
- Optional search.
- Mermaid strategy.
