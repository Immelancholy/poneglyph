use ratatui::style::{Color, Modifier, Style};

#[derive(Clone, Debug)]
pub struct Theme {
    pub bg: Color,
    pub panel: Color,
    pub bg2: Color,
    pub border: Color,
    pub border_strong: Color,
    pub text: Color,
    pub text_muted: Color,
    pub info: Color,
    pub success: Color,
    pub warn: Color,
    #[allow(dead_code)]
    pub error: Color,
    pub heading1: Color,
    pub heading2: Color,
    pub heading3: Color,
    pub code: Color,
}

impl Theme {
    pub fn slate() -> Self {
        Self {
            bg: Color::Rgb(15, 18, 25),
            panel: Color::Rgb(22, 27, 36),
            bg2: Color::Rgb(31, 37, 49),
            border: Color::Rgb(54, 64, 82),
            border_strong: Color::Rgb(92, 107, 135),
            text: Color::Rgb(211, 218, 232),
            text_muted: Color::Rgb(127, 140, 166),
            info: Color::Rgb(88, 166, 255),
            success: Color::Rgb(86, 211, 100),
            warn: Color::Rgb(240, 190, 96),
            error: Color::Rgb(255, 109, 109),
            heading1: Color::Rgb(86, 211, 100),
            heading2: Color::Rgb(88, 166, 255),
            heading3: Color::Rgb(240, 190, 96),
            code: Color::Rgb(255, 125, 125),
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

    pub fn badge(&self, color: Color) -> Style {
        Style::default()
            .fg(self.bg)
            .bg(color)
            .add_modifier(Modifier::BOLD)
    }
}
