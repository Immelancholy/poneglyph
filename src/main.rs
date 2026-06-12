mod app;
mod markdown;
mod theme;
mod ui;

use std::{io, path::PathBuf, time::Duration};

use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::{App, FocusPane, ViewMode};
use theme::Theme;

#[derive(Parser)]
#[command(
    name = "md-editor-rust",
    about = "Rust/Ratatui parity port spike of md-editor"
)]
struct Cli {
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Print markdown outline JSON for oracle tests
    Outline { file: PathBuf },
    /// Print document stats JSON for oracle tests
    Stats { file: PathBuf },
    /// Print markdown classification JSON for oracle tests
    Classify { file: PathBuf },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Outline { file }) => {
            let content = std::fs::read_to_string(file)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&markdown::outline(&content))?
            );
            return Ok(());
        }
        Some(Command::Stats { file }) => {
            let mut app = App::new(None)?;
            app.content = std::fs::read_to_string(file)?;
            println!("{}", serde_json::to_string_pretty(&app.stats())?);
            return Ok(());
        }
        Some(Command::Classify { file }) => {
            let content = std::fs::read_to_string(file)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&markdown::classify_document(&content))?
            );
            return Ok(());
        }
        None => {}
    }

    let mut terminal = setup_terminal()?;
    let result = run_app(&mut terminal, App::new(cli.file)?, Theme::slate());
    restore_terminal(&mut terminal)?;
    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut app: App,
    theme: Theme,
) -> Result<()> {
    loop {
        terminal.draw(|frame| ui::draw(frame, &app, &theme))?;
        if app.should_quit {
            return Ok(());
        }
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key(&mut app, key)?;
                }
            }
        }
    }
}

fn handle_key(app: &mut App, key: KeyEvent) -> Result<()> {
    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('c' | 'q'))
    {
        app.should_quit = true;
        return Ok(());
    }
    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('s')) {
        app.save()?;
        return Ok(());
    }
    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('z')) {
        app.undo();
        return Ok(());
    }
    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('y')) {
        app.redo();
        return Ok(());
    }
    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('x')) {
        app.leader = true;
        app.status = "Ctrl+X: e edit, p preview, f files, o outline, b/r sidebar, u undo, y redo, s save, q quit".into();
        return Ok(());
    }
    if app.leader {
        match key.code {
            KeyCode::Esc => {
                app.leader = false;
                app.status = "Cancelled".into();
            }
            KeyCode::Char(ch) => app.command(ch)?,
            _ => {}
        }
        return Ok(());
    }

    if app.show_help {
        match key.code {
            KeyCode::Esc | KeyCode::Char('h') => app.show_help = false,
            _ => {}
        }
        return Ok(());
    }

    if matches!(app.focus, FocusPane::Files) {
        return handle_files_key(app, key);
    }

    match app.mode {
        ViewMode::Preview => handle_preview_key(app, key),
        ViewMode::Edit => handle_edit_key(app, key),
    }
}

fn handle_preview_key(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => app.scroll_preview(-1),
        KeyCode::Down | KeyCode::Char('j') => app.scroll_preview(1),
        KeyCode::PageUp => app.scroll_preview(-10),
        KeyCode::PageDown => app.scroll_preview(10),
        KeyCode::Esc => app.focus = FocusPane::Editor,
        _ => {}
    }
    Ok(())
}

fn handle_edit_key(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Up => app.move_cursor(-1, 0),
        KeyCode::Down => app.move_cursor(1, 0),
        KeyCode::Left => app.move_cursor(0, -1),
        KeyCode::Right => app.move_cursor(0, 1),
        KeyCode::Esc => {
            app.mode = ViewMode::Preview;
            app.status = "View: preview".into();
        }
        KeyCode::Enter => app.newline(),
        KeyCode::Backspace => app.backspace(),
        KeyCode::Tab => {
            app.insert_char(' ');
            app.insert_char(' ');
        }
        KeyCode::Char(ch) if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT => {
            app.insert_char(ch)
        }
        _ => {}
    }
    Ok(())
}

fn handle_files_key(app: &mut App, key: KeyEvent) -> Result<()> {
    let len = app.file_entries().len().max(1);
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => app.selected_file = app.selected_file.saturating_sub(1),
        KeyCode::Down | KeyCode::Char('j') => {
            app.selected_file = (app.selected_file + 1).min(len - 1)
        }
        KeyCode::Enter | KeyCode::Right => app.open_selected_file()?,
        KeyCode::Left => {
            if let Some(parent) = app.file_browser_cwd.parent() {
                app.file_browser_cwd = parent.to_path_buf();
                app.selected_file = 0;
            }
        }
        KeyCode::Esc => app.focus = FocusPane::Editor,
        _ => {}
    }
    Ok(())
}
