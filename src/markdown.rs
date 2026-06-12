use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};
use serde::Serialize;

use crate::theme::Theme;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum LineKind {
    Heading(u8),
    Blockquote,
    UnorderedList,
    OrderedList,
    CodeBlock,
    HorizontalRule,
    Empty,
    Text,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ClassifiedLine {
    pub kind: LineKind,
    pub text: String,
    pub level: Option<u8>,
    pub language: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct OutlineItem {
    pub level: u8,
    pub text: String,
    pub line: usize,
}

pub fn classify_document(content: &str) -> Vec<ClassifiedLine> {
    let mut in_code = false;
    let mut lang = String::new();
    content
        .split('\n')
        .map(|line| {
            if line.starts_with("```") {
                if in_code {
                    in_code = false;
                    let old = std::mem::take(&mut lang);
                    return ClassifiedLine {
                        kind: LineKind::CodeBlock,
                        text: line.to_string(),
                        level: None,
                        language: Some(old),
                    };
                }
                lang = line.trim_start_matches("```").trim().to_string();
                in_code = true;
                return ClassifiedLine {
                    kind: LineKind::CodeBlock,
                    text: line.to_string(),
                    level: None,
                    language: if lang.is_empty() {
                        None
                    } else {
                        Some(lang.clone())
                    },
                };
            }
            if in_code {
                return ClassifiedLine {
                    kind: LineKind::CodeBlock,
                    text: line.to_string(),
                    level: None,
                    language: if lang.is_empty() {
                        None
                    } else {
                        Some(lang.clone())
                    },
                };
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return ClassifiedLine {
                    kind: LineKind::Empty,
                    text: line.to_string(),
                    level: None,
                    language: None,
                };
            }
            if is_hr(trimmed) {
                return ClassifiedLine {
                    kind: LineKind::HorizontalRule,
                    text: line.to_string(),
                    level: None,
                    language: None,
                };
            }
            if let Some((level, title)) = heading(line) {
                return ClassifiedLine {
                    kind: LineKind::Heading(level),
                    text: title.to_string(),
                    level: Some(level),
                    language: None,
                };
            }
            if trimmed.starts_with('>') {
                return ClassifiedLine {
                    kind: LineKind::Blockquote,
                    text: line.to_string(),
                    level: None,
                    language: None,
                };
            }
            if unordered(line) {
                return ClassifiedLine {
                    kind: LineKind::UnorderedList,
                    text: line.to_string(),
                    level: None,
                    language: None,
                };
            }
            if ordered(line) {
                return ClassifiedLine {
                    kind: LineKind::OrderedList,
                    text: line.to_string(),
                    level: None,
                    language: None,
                };
            }
            ClassifiedLine {
                kind: LineKind::Text,
                text: line.to_string(),
                level: None,
                language: None,
            }
        })
        .collect()
}

pub fn outline(content: &str) -> Vec<OutlineItem> {
    content
        .split('\n')
        .enumerate()
        .filter_map(|(line, text)| {
            heading(text).map(|(level, title)| OutlineItem {
                level,
                text: title.to_string(),
                line,
            })
        })
        .collect()
}

fn heading(line: &str) -> Option<(u8, &str)> {
    let bytes = line.as_bytes();
    let mut level = 0usize;
    while level < bytes.len() && bytes[level] == b'#' && level < 6 {
        level += 1;
    }
    if level == 0 || bytes.get(level) != Some(&b' ') {
        return None;
    }
    Some((level as u8, line[level + 1..].trim()))
}

fn is_hr(s: &str) -> bool {
    s.len() >= 3
        && (s.chars().all(|c| c == '-')
            || s.chars().all(|c| c == '*')
            || s.chars().all(|c| c == '_'))
}

fn unordered(line: &str) -> bool {
    let s = line.trim_start();
    matches!(s.as_bytes(), [b'-' | b'*' | b'+', b' ', ..])
}

fn ordered(line: &str) -> bool {
    let s = line.trim_start();
    let Some(dot) = s.find('.') else {
        return false;
    };
    dot > 0
        && s[..dot].chars().all(|c| c.is_ascii_digit())
        && s.as_bytes().get(dot + 1) == Some(&b' ')
}

pub fn render_preview_line<'a>(raw: &'a str, theme: &Theme) -> Line<'a> {
    if let Some((level, title)) = heading(raw) {
        let color = match level {
            1 => theme.heading1,
            2 => theme.heading2,
            _ => theme.heading3,
        };
        return Line::from(vec![Span::styled(
            title.to_string(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )]);
    }
    let trimmed = raw.trim_start();
    if trimmed.starts_with('>') {
        return Line::from(vec![
            Span::styled("▌ ", Style::default().fg(theme.text_muted)),
            Span::styled(
                trimmed.trim_start_matches('>').trim().to_string(),
                Style::default()
                    .fg(theme.text_muted)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]);
    }
    if unordered(raw) || ordered(raw) {
        let prefix_len = raw.find(' ').map(|i| i + 1).unwrap_or(0);
        let (prefix, rest) = raw.split_at(prefix_len.min(raw.len()));
        return Line::from(vec![
            Span::styled(prefix.to_string(), Style::default().fg(theme.warn)),
            Span::raw(rest.to_string()),
        ]);
    }
    if raw.starts_with("```") {
        return Line::from(Span::styled(
            raw.to_string(),
            Style::default().fg(theme.code).bg(theme.bg2),
        ));
    }
    render_inline(raw, theme)
}

pub fn render_editor_line<'a>(raw: &'a str, theme: &Theme) -> Line<'a> {
    render_preview_line(raw, theme)
}

fn render_inline<'a>(raw: &'a str, theme: &Theme) -> Line<'a> {
    let mut spans = Vec::new();
    let mut rest = raw;
    while !rest.is_empty() {
        if let Some(stripped) = rest.strip_prefix("**") {
            if let Some(end) = stripped.find("**") {
                let (text, after) = stripped.split_at(end);
                spans.push(Span::styled(
                    text.to_string(),
                    Style::default().fg(theme.warn).add_modifier(Modifier::BOLD),
                ));
                rest = &after[2..];
                continue;
            }
        }
        if let Some(stripped) = rest.strip_prefix('`') {
            if let Some(end) = stripped.find('`') {
                let (text, after) = stripped.split_at(end);
                spans.push(Span::styled(
                    text.to_string(),
                    Style::default().fg(theme.code).bg(theme.bg2),
                ));
                rest = &after[1..];
                continue;
            }
        }
        if let Some(ch) = rest.chars().next() {
            let len = ch.len_utf8();
            spans.push(Span::styled(
                ch.to_string(),
                Style::default().fg(theme.text),
            ));
            rest = &rest[len..];
        }
    }
    if spans.is_empty() {
        Line::from("")
    } else {
        Line::from(spans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_like_md_editor_core_shapes() {
        let doc = "# H1\n\n> quote\n- item\n1. one\n---\n```ts\nlet x = 1;\n```";
        let kinds: Vec<_> = classify_document(doc).into_iter().map(|l| l.kind).collect();
        assert_eq!(
            kinds,
            vec![
                LineKind::Heading(1),
                LineKind::Empty,
                LineKind::Blockquote,
                LineKind::UnorderedList,
                LineKind::OrderedList,
                LineKind::HorizontalRule,
                LineKind::CodeBlock,
                LineKind::CodeBlock,
                LineKind::CodeBlock
            ]
        );
    }

    #[test]
    fn extracts_outline() {
        let items = outline("# A\nbody\n### C");
        assert_eq!(
            items,
            vec![
                OutlineItem {
                    level: 1,
                    text: "A".into(),
                    line: 0
                },
                OutlineItem {
                    level: 3,
                    text: "C".into(),
                    line: 2
                }
            ]
        );
    }
}
