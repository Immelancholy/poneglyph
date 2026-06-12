use std::{fs, path::PathBuf};

use anyhow::Result;
use serde::Serialize;

use crate::app::CursorStyle;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct AppConfig {
    pub theme: Option<String>,
    pub cursor_style: Option<CursorStyle>,
    pub boxed_chrome: Option<bool>,
}

impl AppConfig {
    pub fn load() -> Self {
        let Some(path) = config_path_for_load() else {
            return Self::default();
        };
        fs::read_to_string(path)
            .ok()
            .map(|raw| parse_config(&raw))
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<PathBuf> {
        let path = config_path_for_save();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, self.to_toml())?;
        Ok(path)
    }

    pub fn to_toml(&self) -> String {
        let mut out = String::from("# md-editor-rust user preferences\n\n[ui]\n");
        if let Some(theme) = &self.theme {
            out.push_str(&format!("theme = \"{}\"\n", escape_toml_string(theme)));
        }
        if let Some(cursor_style) = &self.cursor_style {
            out.push_str(&format!(
                "cursorStyle = \"{}\"\n",
                cursor_style.as_config_str()
            ));
        }
        if let Some(boxed_chrome) = self.boxed_chrome {
            out.push_str(&format!("boxedChrome = {}\n", boxed_chrome));
        }
        out
    }
}

pub fn config_path_for_load() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("MD_EDITOR_RUST_CONFIG") {
        return Some(PathBuf::from(path));
    }
    if let Ok(cwd) = std::env::current_dir() {
        let local = cwd.join(".md-editor.toml");
        if local.exists() {
            return Some(local);
        }
    }
    let global = config_path_for_save();
    global.exists().then_some(global)
}

pub fn config_path_for_save() -> PathBuf {
    if let Some(path) = std::env::var_os("MD_EDITOR_RUST_CONFIG") {
        return PathBuf::from(path);
    }
    if let Ok(cwd) = std::env::current_dir() {
        let local = cwd.join(".md-editor.toml");
        if local.exists() {
            return local;
        }
    }
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("md-editor-rust/config.toml")
}

pub fn parse_config(raw: &str) -> AppConfig {
    let mut cfg = AppConfig::default();
    let mut section = String::new();
    for line in raw.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = line.trim_matches(&['[', ']'][..]).to_string();
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if !section.is_empty() && section != "ui" && section != "editor" && section != "theme" {
            continue;
        }
        let key = key.trim();
        let value = value.trim().trim_matches('"');
        match key {
            "theme" | "themeName" => cfg.theme = Some(value.to_string()),
            "cursorStyle" | "cursor_style" => {
                cfg.cursor_style = CursorStyle::from_config_str(value)
            }
            "boxedChrome" | "boxed_chrome" => cfg.boxed_chrome = parse_bool(value),
            _ => {}
        }
    }
    cfg
}

fn parse_bool(raw: &str) -> Option<bool> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "true" | "yes" | "1" | "boxed" => Some(true),
        "false" | "no" | "0" | "smooth" => Some(false),
        _ => None,
    }
}

fn escape_toml_string(raw: &str) -> String {
    raw.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ui_preferences() {
        let cfg = parse_config(
            r##"
            [ui]
            theme = "tokyo-night"
            cursorStyle = "underline"
            boxedChrome = false
            "##,
        );
        assert_eq!(cfg.theme.as_deref(), Some("tokyo-night"));
        assert_eq!(cfg.cursor_style, Some(CursorStyle::Underline));
        assert_eq!(cfg.boxed_chrome, Some(false));
    }

    #[test]
    fn writes_stable_toml() {
        let cfg = AppConfig {
            theme: Some("slate".into()),
            cursor_style: Some(CursorStyle::Bar),
            boxed_chrome: Some(true),
        };
        let toml = cfg.to_toml();
        assert!(toml.contains("theme = \"slate\""));
        assert!(toml.contains("cursorStyle = \"bar\""));
        assert!(toml.contains("boxedChrome = true"));
    }
}
