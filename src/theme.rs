use std::{fs, path::PathBuf};

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
    pub list_marker: Color,
    pub link: Color,
    pub link_text: Color,
    pub link_url: Color,
    pub image: Color,
    pub code_block_border: Color,
    pub code_block_lang: Color,
    pub strikethrough: Color,
    pub hr: Color,
}

impl Theme {
    pub fn named(name: &str) -> Self {
        if let Some(theme) = Self::from_bundled_toml(name) {
            return theme;
        }
        match name {
            "ember" => Self::ember(),
            _ => Self::slate(),
        }
    }

    pub fn from_bundled_toml(name: &str) -> Option<Self> {
        for dir in theme_dirs() {
            let path = dir.join(format!("{name}.toml"));
            if let Ok(raw) = fs::read_to_string(&path) {
                return Some(Self::from_theme_toml(&raw));
            }
        }
        None
    }

    pub fn from_theme_toml(raw: &str) -> Self {
        let mut theme = Self::slate();
        let mut section = String::new();
        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                section = line.trim_matches(&['[', ']'][..]).to_string();
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let key = key.trim();
            let value = value.trim().trim_matches('"');
            let Some(color) = parse_hex_color(value) else {
                continue;
            };
            theme.apply_token(&section, key, color);
        }
        theme.ensure_readable_contrast();
        theme
    }

    fn ensure_readable_contrast(&mut self) {
        self.text = readable_on(self.text, self.bg);
        self.text = readable_on(self.text, self.panel);
        self.text = readable_on(self.text, self.panel_elevated);
        self.text_muted = readable_muted_on(self.text_muted, self.bg, self.text);
        self.text_muted = readable_muted_on(self.text_muted, self.panel, self.text);
        self.text_muted = readable_muted_on(self.text_muted, self.panel_elevated, self.text);
        self.link = readable_on(self.link, self.bg);
        self.link_text = readable_on(self.link_text, self.bg);
        self.link_url = readable_muted_on(self.link_url, self.bg, self.text);
        self.quote = readable_muted_on(self.quote, self.bg, self.text);
    }

    fn apply_token(&mut self, section: &str, key: &str, color: Color) {
        match (section, key) {
            ("slate", "bg0") => self.bg = color,
            ("slate", "bg2") => self.bg2 = color,
            ("slate", "panel") => self.panel = color,
            ("slate", "panelElevated") => self.panel_elevated = color,
            ("slate", "borderSoft") => self.border = color,
            ("slate", "borderStrong") => self.border_strong = color,
            ("slate", "text") => self.text = color,
            ("slate", "textMuted") => self.text_muted = color,
            ("semantic", "success" | "modeEdit") => self.success = color,
            ("semantic", "warn" | "modePreview") => self.warn = color,
            ("semantic", "error") => self.error = color,
            ("semantic", "info" | "modeFiles") => self.info = color,
            ("semantic", "accent" | "modeLog" | "modeLeader") => self.accent = color,
            ("syntax", "heading1") => self.heading1 = color,
            ("syntax", "heading2") => self.heading2 = color,
            ("syntax", "heading3") => self.heading3 = color,
            ("syntax", "heading4") => self.heading4 = color,
            ("syntax", "heading5") => self.heading5 = color,
            ("syntax", "heading6") => self.heading6 = color,
            ("syntax", "headingMarker" | "marker") => self.heading_marker = color,
            ("syntax", "codeBlockBorder") => self.code_block_border = color,
            ("syntax", "bold") => self.bold = color,
            ("syntax", "italic") => self.italic = color,
            ("syntax", "boldItalic") => self.bold_italic = color,
            ("syntax", "code") => self.code = color,
            ("syntax", "codeBlockLang") => self.code_block_lang = color,
            ("syntax", "codeBg") => self.code_bg = color,
            ("syntax", "strikethrough") => self.strikethrough = color,
            ("syntax", "listMarker") => self.list_marker = color,
            ("syntax", "blockquote") => self.quote = color,
            ("syntax", "blockquoteMarker") => self.quote_marker = color,
            ("syntax", "link") => self.link = color,
            ("syntax", "linkText") => self.link_text = color,
            ("syntax", "linkUrl") => self.link_url = color,
            ("syntax", "image") => self.image = color,
            ("syntax", "horizontalRule") => self.hr = color,
            _ => {}
        }
    }

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
            list_marker: Color::Rgb(0xce, 0x91, 0x78),
            link: Color::Rgb(0x4f, 0xc1, 0xff),
            link_text: Color::Rgb(0x9c, 0xdc, 0xfe),
            link_url: Color::Rgb(0xce, 0x91, 0x78),
            image: Color::Rgb(0xc5, 0x86, 0xc0),
            code_block_border: Color::Rgb(0x80, 0x80, 0x80),
            code_block_lang: Color::Rgb(0xdc, 0xdc, 0xaa),
            strikethrough: Color::Rgb(0x6a, 0x99, 0x55),
            hr: Color::Rgb(0x3a, 0x47, 0x59),
        }
    }

    pub fn ember() -> Self {
        Self {
            bg: Color::Rgb(0x16, 0x10, 0x0f),
            panel: Color::Rgb(0x22, 0x18, 0x15),
            panel_elevated: Color::Rgb(0x32, 0x22, 0x1d),
            bg2: Color::Rgb(0x2a, 0x1d, 0x19),
            border: Color::Rgb(0x4a, 0x32, 0x2a),
            border_strong: Color::Rgb(0x74, 0x49, 0x36),
            text: Color::Rgb(0xf4, 0xdd, 0xc8),
            text_muted: Color::Rgb(0xb8, 0x94, 0x7d),
            info: Color::Rgb(0xee, 0x9b, 0x72),
            accent: Color::Rgb(0xff, 0x7a, 0x45),
            success: Color::Rgb(0xd7, 0xa8, 0x6e),
            warn: Color::Rgb(0xff, 0xc8, 0x57),
            error: Color::Rgb(0xff, 0x5d, 0x73),
            heading1: Color::Rgb(0xff, 0x9f, 0x68),
            heading2: Color::Rgb(0xff, 0xc8, 0x57),
            heading3: Color::Rgb(0xf7, 0x8c, 0x6b),
            heading4: Color::Rgb(0xe0, 0xa4, 0x77),
            heading5: Color::Rgb(0xff, 0x6b, 0x6b),
            heading6: Color::Rgb(0xc9, 0x8b, 0x5f),
            heading_marker: Color::Rgb(0xa8, 0x5f, 0x3d),
            bold: Color::Rgb(0xff, 0xc8, 0x57),
            italic: Color::Rgb(0xee, 0x9b, 0x72),
            bold_italic: Color::Rgb(0xff, 0x7a, 0x90),
            code: Color::Rgb(0xff, 0xdf, 0x9e),
            code_bg: Color::Rgb(0x31, 0x20, 0x1a),
            quote: Color::Rgb(0xd7, 0xa8, 0x6e),
            quote_marker: Color::Rgb(0xff, 0x7a, 0x45),
            list_marker: Color::Rgb(0xff, 0xc8, 0x57),
            link: Color::Rgb(0xff, 0x9f, 0x68),
            link_text: Color::Rgb(0xf4, 0xdd, 0xc8),
            link_url: Color::Rgb(0xff, 0xc8, 0x57),
            image: Color::Rgb(0xff, 0x7a, 0x90),
            code_block_border: Color::Rgb(0xa8, 0x5f, 0x3d),
            code_block_lang: Color::Rgb(0xff, 0xdf, 0x9e),
            strikethrough: Color::Rgb(0xb8, 0x94, 0x7d),
            hr: Color::Rgb(0x74, 0x49, 0x36),
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
                list_marker: color_hex(self.list_marker),
                blockquote: color_hex(self.quote),
                blockquote_marker: color_hex(self.quote_marker),
                link: color_hex(self.link),
                link_text: color_hex(self.link_text),
                link_url: color_hex(self.link_url),
                image: color_hex(self.image),
                code_block_border: color_hex(self.code_block_border),
                code_block_lang: color_hex(self.code_block_lang),
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

fn theme_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        dirs.push(cwd.join("themes"));
        dirs.push(cwd.join("../md-editor/themes"));
    }
    if let Some(home) = std::env::var_os("HOME") {
        dirs.push(PathBuf::from(&home).join(".config/poneglyph/themes"));
        dirs.push(PathBuf::from(home).join(".config/md-editor/themes"));
    }
    dirs
}

fn readable_on(fg: Color, bg: Color) -> Color {
    if contrast_ratio(fg, bg).unwrap_or(21.0) < 4.5 {
        Color::Rgb(0xf8, 0xf8, 0xf2)
    } else {
        fg
    }
}

fn readable_muted_on(fg: Color, bg: Color, fallback: Color) -> Color {
    if contrast_ratio(fg, bg).unwrap_or(21.0) < 2.6 {
        fallback
    } else {
        fg
    }
}

fn contrast_ratio(a: Color, b: Color) -> Option<f32> {
    let (ar, ag, ab) = color_rgb(a)?;
    let (br, bg, bb) = color_rgb(b)?;
    let al = rel_luminance(ar, ag, ab);
    let bl = rel_luminance(br, bg, bb);
    let (light, dark) = if al > bl { (al, bl) } else { (bl, al) };
    Some((light + 0.05) / (dark + 0.05))
}

fn color_rgb(color: Color) -> Option<(u8, u8, u8)> {
    match color {
        Color::Rgb(r, g, b) => Some((r, g, b)),
        _ => None,
    }
}

fn rel_luminance(r: u8, g: u8, b: u8) -> f32 {
    fn channel(v: u8) -> f32 {
        let c = v as f32 / 255.0;
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b)
}

fn parse_hex_color(raw: &str) -> Option<Color> {
    let hex = raw.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
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
    fn parses_bundled_theme_toml_tokens() {
        let raw = "[slate]\nbg0 = \"#1a1b26\"\n[syntax]\nheading1 = \"#bb9af7\"\n";
        let theme = Theme::from_theme_toml(raw);
        assert_eq!(color_hex(theme.bg), "#1a1b26");
        assert_eq!(color_hex(theme.heading1), "#bb9af7");
    }

    #[test]
    fn low_contrast_theme_tokens_are_guarded() {
        let raw = "[slate]\nbg0 = \"#282a36\"\npanelElevated = \"#6272a4\"\ntextMuted = \"#6272a4\"\n[syntax]\nlink = \"#8be9fd\"\nlinkText = \"#6272a4\"\n";
        let theme = Theme::from_theme_toml(raw);
        assert_ne!(color_hex(theme.text_muted), "#6272a4");
        assert_eq!(color_hex(theme.link), "#8be9fd");
    }

    #[test]
    fn maps_richer_syntax_tokens_independently() {
        let raw = "[syntax]\nlistMarker = \"#e0af68\"\nlinkText = \"#9cdcfe\"\nlinkUrl = \"#ce9178\"\ncodeBlockBorder = \"#808080\"\ncodeBlockLang = \"#dcdcaa\"\n";
        let theme = Theme::from_theme_toml(raw);
        assert_eq!(color_hex(theme.list_marker), "#e0af68");
        assert_eq!(color_hex(theme.link_text), "#9cdcfe");
        assert_eq!(color_hex(theme.link_url), "#ce9178");
        assert_eq!(color_hex(theme.code_block_border), "#808080");
        assert_eq!(color_hex(theme.code_block_lang), "#dcdcaa");
    }

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
