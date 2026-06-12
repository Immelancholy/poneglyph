#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { parseMarkdownDocument } from "../../md-editor/src/core/markdown-syntax";
import { UI_TOKENS } from "../../md-editor/src/core/ui-tokens";

type Segment = { text: string; bold?: boolean; italic?: boolean };
type RenderedLine = { segments: Segment[]; prefix?: string };

const command = process.argv[2];
const file = process.argv[3];
const widthArg = Number(process.argv[4] ?? "80");
const heightArg = Number(process.argv[5] ?? "24");

if (!command || !file) {
  console.error("usage: bun scripts/bun-oracle.ts <classify|outline|preview-lines> <file> [width] [height]");
  process.exit(2);
}

const content = readFileSync(file, "utf8");

if (command === "classify") {
  const colors = syntaxColors();
  const lines = parseMarkdownDocument(content, colors as any).map((line, index) => ({
    line: index,
    lineType: line.lineType,
    text: line.segments.map((seg) => seg.text).join(""),
    segments: line.segments.map((seg) => ({
      text: seg.text,
      bold: !!seg.bold,
      italic: !!seg.italic,
      underline: !!seg.underline,
      hasBg: !!seg.bg,
    })),
  }));
  console.log(JSON.stringify(lines, null, 2));
} else if (command === "outline") {
  const out: Array<{ level: number; text: string; line: number }> = [];
  const headingRegex = /^(#{1,6})\s+(.+)$/gm;
  let match: RegExpExecArray | null;
  while ((match = headingRegex.exec(content)) !== null) {
    const prefix = content.slice(0, match.index);
    const line = prefix.length === 0 ? 0 : prefix.split("\n").length - 1;
    out.push({ level: match[1].length, text: match[2], line });
  }
  console.log(JSON.stringify(out, null, 2));
} else if (command === "preview-lines") {
  const lines = renderOriginalPreviewLines(content, widthArg, heightArg);
  console.log(JSON.stringify({ width: widthArg, height: heightArg, lines }, null, 2));
} else {
  console.error(`unknown command: ${command}`);
  process.exit(2);
}

function syntaxColors() {
  return {
    ...UI_TOKENS.syntax,
    text: UI_TOKENS.slate.text,
  };
}

function renderOriginalPreviewLines(content: string, width: number, height: number): string[] {
  const rendered = renderMarkdownLikeOriginal(content, width);
  const availableWidth = Math.max(1, width - 2);
  const wrapped: string[] = [];
  for (const line of rendered) {
    const text = `${line.prefix ?? ""}${line.segments.map((seg) => seg.text).join("")}`;
    wrapped.push(...wrapPlain(text, availableWidth));
  }
  return wrapped.slice(0, height);
}

function renderMarkdownLikeOriginal(content: string, width: number): RenderedLine[] {
  const text = content || "# No content\n\nStart typing or open a file...";
  const lines: RenderedLine[] = [];
  const contentLines = text.split("\n");
  let inCodeBlock = false;
  let codeLanguage = "";

  for (let i = 0; i < contentLines.length; i++) {
    const line = contentLines[i] ?? "";
    const trimmedLine = line.trim();

    if (/^```mermaid\s*/i.test(line)) {
      lines.push({ segments: [{ text: "mermaid", bold: true }], prefix: "▌ " });
      i++;
      while (i < contentLines.length && contentLines[i]?.trim() !== "```") {
        lines.push({ segments: [{ text: contentLines[i] || " " }], prefix: "│ " });
        i++;
      }
      continue;
    }

    if (trimmedLine === "```" || (trimmedLine.startsWith("```") && trimmedLine.toLowerCase() !== "```mermaid")) {
      if (!inCodeBlock) {
        inCodeBlock = true;
        codeLanguage = line.slice(3).trim();
        if (codeLanguage) {
          lines.push({ segments: [{ text: codeLanguage, bold: true }], prefix: "▌ " });
        }
      } else {
        inCodeBlock = false;
        codeLanguage = "";
      }
      continue;
    }

    if (inCodeBlock) {
      lines.push({ segments: [{ text: line || " " }], prefix: "▌ " });
      continue;
    }

    if (/^-{3,}$/.test(line.trim()) || /^\*{3,}$/.test(line.trim())) {
      lines.push({ segments: [{ text: "─".repeat(Math.max(1, width - 4)) }] });
      continue;
    }

    const h1Match = line.match(/^#\s+(.+)$/);
    if (h1Match) {
      const title = h1Match[1].toUpperCase();
      lines.push({ segments: [{ text: "" }] });
      lines.push({ segments: [{ text: title, bold: true }] });
      lines.push({ segments: [{ text: "═".repeat(Math.min(title.length, width - 2)) }] });
      lines.push({ segments: [{ text: "" }] });
      continue;
    }

    const h2Match = line.match(/^##\s+(.+)$/);
    if (h2Match) {
      const title = h2Match[1];
      lines.push({ segments: [{ text: "" }] });
      lines.push({ segments: [{ text: title, bold: true }] });
      lines.push({ segments: [{ text: "─".repeat(Math.min(title.length, width - 2)) }] });
      continue;
    }

    const hxMatch = line.match(/^#{3,6}\s+(.+)$/);
    if (hxMatch) {
      lines.push({ segments: [{ text: "▸ " + hxMatch[1], bold: true }] });
      continue;
    }

    if (line.startsWith("> ")) {
      lines.push({ segments: parseInlineSegmentsPlain(line.slice(2)), prefix: "│ " });
      continue;
    }

    const ulMatch = line.match(/^(\s*)[-*+]\s+(.+)$/);
    if (ulMatch) {
      const indent = ulMatch[1].length;
      const content = ulMatch[2];
      const bullet = indent > 0 ? "  ◦" : "•";
      lines.push({ segments: parseInlineSegmentsPlain(" ".repeat(indent) + content), prefix: bullet + " " });
      continue;
    }

    const olMatch = line.match(/^(\s*)(\d+)\.\s+(.+)$/);
    if (olMatch) {
      const indent = olMatch[1].length;
      const num = olMatch[2];
      const body = olMatch[3];
      lines.push({ segments: parseInlineSegmentsPlain(" ".repeat(indent) + body), prefix: num + ". " });
      continue;
    }

    if (line.trim() === "") {
      lines.push({ segments: [{ text: "" }] });
      continue;
    }

    lines.push({ segments: parseInlineSegmentsPlain(line) });
  }
  return lines;
}

function parseInlineSegmentsPlain(line: string): Segment[] {
  // Mirrors PreviewView's display-level parser enough for side-by-side text parity.
  let remaining = line;
  const segments: Segment[] = [];
  const patterns = [
    { regex: /\*\*([^*]+)\*\*/, type: "bold" },
    { regex: /__([^_]+)__/, type: "bold" },
    { regex: /(?<!\*)\*([^*]+)\*(?!\*)/, type: "italic" },
    { regex: /(?<!_)_([^_]+)_(?!_)/, type: "italic" },
    { regex: /`([^`]+)`/, type: "code" },
    { regex: /\[([^\]]+)\]\(([^)]+)\)/, type: "link" },
  ];
  while (remaining.length > 0) {
    let best: { index: number; match: RegExpMatchArray; type: string } | null = null;
    for (const pattern of patterns) {
      const match = remaining.match(pattern.regex);
      if (match && (best === null || match.index! < best.index)) {
        best = { index: match.index!, match, type: pattern.type };
      }
    }
    if (!best) {
      segments.push({ text: remaining });
      break;
    }
    if (best.index > 0) segments.push({ text: remaining.slice(0, best.index) });
    segments.push({ text: best.match[1], bold: best.type === "bold", italic: best.type === "italic" });
    remaining = remaining.slice(best.index + best.match[0].length);
  }
  return segments.length ? segments : [{ text: " " }];
}

function wrapPlain(text: string, width: number): string[] {
  if (text.length <= width) return [text];
  const rows: string[] = [];
  let rest = text;
  while (rest.length > width) {
    rows.push(rest.slice(0, width));
    rest = rest.slice(width);
  }
  rows.push(rest);
  return rows;
}
