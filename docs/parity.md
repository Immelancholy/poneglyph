# md-editor Rust parity plan

Oracle: `/home/shamanic/dev/md-editor`.

First slice goals:

- Preserve the same product shape: header, main editor/preview pane, outline/files sidebar, footer.
- Default to preview mode.
- Use leader commands similar to md-editor (`Ctrl+X`, then command key).
- Support opening markdown files from CLI.
- Support preview scrolling, edit mode cursor movement, basic text insertion/deletion, save.
- Parse markdown headings/lists/quotes/code blocks/inline styling into styled terminal spans.
- Provide fixture tests for markdown classification, outline extraction, wrapping, editing, and file stats.
- Provide a benchmark script that can compare process RSS against the Bun md-editor oracle.

Non-goals for first slice:

- Full Mermaid rendering.
- Full theme picker.
- Mouse parity.
- Exact OpenTUI rendering implementation details.
