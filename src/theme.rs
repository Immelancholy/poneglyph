use ratatui::style::{Color, Modifier, Style};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ThemeTokens {
    pub slate: SlateTokens,
    pub semantic: SemanticTokens,
    pub syntax: SyntaxTokens,
}

#[derive(Clone, Debug, Serialize)]
pub struct SlateTokens {
    pub bg0: String,
    pub bg1: String,
    pub bg2: String,
    pub panel: String,
    #[serde(rename = "panelElevated")]
    pub panel_elevated: String,
    #[serde(rename = "borderSoft")]
    pub border_soft: String,
    #[serde(rename = "borderStrong")]
    pub border_strong: String,
    pub text: String,
    #[serde(rename = "textMuted")]
    pub text_muted: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct SemanticTokens {
    pub success: String,
    pub warn: String,
    pub error: String,
    pub info: String,
    pub accent: String,
    #[serde(rename = "modeLog")]
    pub mode_log: String,
    #[serde(rename = "modeEdit")]
    pub mode_edit: String,
    #[serde(rename = "modePreview")]
    pub mode_preview: String,
    #[serde(rename = "modeFiles")]
    pub mode_files: String,
    #[serde(rename = "modeLeader")]
    pub mode_leader: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct SyntaxTokens {
    pub heading1: String,
    pub heading2: String,
    pub heading3: String,
    pub heading4: String,
    pub heading5: String,
    pub heading6: String,
    #[serde(rename = "headingMarker")]
    pub heading_marker: String,
    pub bold: String,
    pub italic: String,
    #[serde(rename = "boldItalic")]
    pub bold_italic: String,
    pub code: String,
    #[serde(rename = "codeBg")]
    pub code_bg: String,
    pub strikethrough: String,
    #[serde(rename = "listMarker")]
    pub list_marker: String,
    pub blockquote: String,
    #[serde(rename = "blockquoteMarker")]
    pub blockquote_marker: String,
    pub link: String,
    #[serde(rename = "linkText")]
    pub link_text: String,
    #[serde(rename = "linkUrl")]
    pub link_url: String,
    pub image: String,
    #[serde(rename = "codeBlockBorder")]
    pub code_block_border: String,
    #[serde(rename = "codeBlockLang")]
    pub code_block_lang: String,
    #[serde(rename = "horizontalRule")]
    pub horizontal_rule: String,
    pub marker: String,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub bg: Color,
    pub panel: Color,
    pub panel_elevated: Color,
    pub bg2: Color,
    pub border: Color,
    pub border_strong: Color,
    pub text: Color,
    pub text_muted: Color,
    pub info: Color,
    pub accent: Color,
    pub success: Color,
    pub warn: Color,
    #[allow(dead_code)]
    pub error: Color,
    pub heading1: Color,
    pub heading2: Color,
    pub heading3: Color,
    pub heading4: Color,
    pub heading5: Color,
    pub heading6: Color,
    pub heading_marker: Color,
    pub bold: Color,
    pub italic: Color,
    pub bold_italic: Color,
    pub code: Color,
    pub code_bg: Color,
    pub quote: Color,
    pub quote_marker: Color,
    pub link: Color,
    pub image: Color,
    pub strikethrough: Color,
    pub hr: Color,
}

impl Theme {
    pub fn slate() -> Self {
        Self {
            bg: Color::Rgb(0x11, 0x13, 0x18),
            panel: Color::Rgb(0x1a, 0x20, 0x29),
            panel_elevated: Color::Rgb(0x22, 0x2b, 0x37),
            bg2: Color::Rgb(0x1d, 0x25, 0x30),
            border: Color::Rgb(0x2a, 0x34, 0x42),
            border_strong: Color::Rgb(0x3a, 0x47, 0x59),
            text: Color::Rgb(0xd8, 0xde, 0xe8),
            text_muted: Color::Rgb(0x8a, 0x95, 0xa7),
            info: Color::Rgb(0x7f, 0x9f, 0xbf),
            accent: Color::Rgb(0x6f, 0x89, 0xad),
            success: Color::Rgb(0x74, 0xb8, 0x8c),
            warn: Color::Rgb(0xc9, 0xa8, 0x6a),
            error: Color::Rgb(0xce, 0x6f, 0x7c),
            heading1: Color::Rgb(0x4e, 0xc9, 0xb0),
            heading2: Color::Rgb(0x4f, 0xc1, 0xff),
            heading3: Color::Rgb(0x56, 0x9c, 0xd6),
            heading4: Color::Rgb(0x9c, 0xdc, 0xfe),
            heading5: Color::Rgb(0xce, 0x91, 0x78),
            heading6: Color::Rgb(0xb5, 0xce, 0xa8),
            heading_marker: Color::Rgb(0x80, 0x80, 0x80),
            bold: Color::Rgb(0xce, 0x91, 0x78),
            italic: Color::Rgb(0x56, 0x9c, 0xd6),
            bold_italic: Color::Rgb(0xc5, 0x86, 0xc0),
            code: Color::Rgb(0xdc, 0xdc, 0xaa),
            code_bg: Color::Rgb(0x2d, 0x2d, 0x2d),
            quote: Color::Rgb(0x6a, 0x99, 0x55),
            quote_marker: Color::Rgb(0x80, 0x80, 0x80),
            link: Color::Rgb(0x4f, 0xc1, 0xff),
            image: Color::Rgb(0xc5, 0x86, 0xc0),
            strikethrough: Color::Rgb(0x6a, 0x99, 0x55),
            hr: Color::Rgb(0x3a, 0x47, 0x59),
        }
    }

    pub fn tokens(&self) -> ThemeTokens {
        ThemeTokens {
            slate: SlateTokens {
                bg0: color_hex(self.bg),
                bg1: color_hex(Color::Rgb(0x16, 0x1b, 0x22)),
                bg2: color_hex(self.bg2),
                panel: color_hex(self.panel),
                panel_elevated: color_hex(self.panel_elevated),
                border_soft: color_hex(self.border),
                border_strong: color_hex(self.border_strong),
                text: color_hex(self.text),
                text_muted: color_hex(self.text_muted),
            },
            semantic: SemanticTokens {
                success: color_hex(self.success),
                warn: color_hex(self.warn),
                error: color_hex(self.error),
                info: color_hex(self.info),
                accent: color_hex(self.accent),
                mode_log: color_hex(self.accent),
                mode_edit: color_hex(self.success),
                mode_preview: color_hex(self.warn),
                mode_files: color_hex(self.info),
                mode_leader: color_hex(self.accent),
            },
            syntax: SyntaxTokens {
                heading1: color_hex(self.heading1),
                heading2: color_hex(self.heading2),
                heading3: color_hex(self.heading3),
                heading4: color_hex(self.heading4),
                heading5: color_hex(self.heading5),
                heading6: color_hex(self.heading6),
                heading_marker: color_hex(self.heading_marker),
                bold: color_hex(self.bold),
                italic: color_hex(self.italic),
                bold_italic: color_hex(self.bold_italic),
                code: color_hex(self.code),
                code_bg: color_hex(self.code_bg),
                strikethrough: color_hex(self.strikethrough),
                list_marker: color_hex(self.warn),
                blockquote: color_hex(self.quote),
                blockquote_marker: color_hex(self.quote_marker),
                link: color_hex(self.link),
                link_text: color_hex(self.heading4),
                link_url: color_hex(self.heading5),
                image: color_hex(self.image),
                code_block_border: color_hex(self.heading_marker),
                code_block_lang: color_hex(self.code),
                horizontal_rule: color_hex(self.hr),
                marker: color_hex(self.heading_marker),
            },
        }
    }

    pub fn base(&self) -> Style {
        Style::default().fg(self.text).bg(self.bg)
    }

    pub fn dim(&self) -> Style {
        self.base().fg(self.text_muted)
    }

    pub fn panel(&self) -> Style {
        Style::default().fg(self.text).bg(self.panel)
    }

    pub fn elevated(&self) -> Style {
        Style::default().fg(self.text).bg(self.panel_elevated)
    }

    pub fn badge(&self, color: Color) -> Style {
        Style::default()
            .fg(self.bg)
            .bg(color)
            .add_modifier(Modifier::BOLD)
    }
}

fn color_hex(color: Color) -> String {
    match color {
        Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
        _ => "#000000".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slate_tokens_match_bun_default_values() {
        let tokens = Theme::slate().tokens();
        assert_eq!(tokens.slate.bg0, "#111318");
        assert_eq!(tokens.slate.panel_elevated, "#222b37");
        assert_eq!(tokens.semantic.mode_preview, "#c9a86a");
        assert_eq!(tokens.syntax.heading1, "#4ec9b0");
        assert_eq!(tokens.syntax.code_bg, "#2d2d2d");
    }
}
