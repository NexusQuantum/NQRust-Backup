//! TUI entry point and screen dispatch.

pub mod screens;
pub mod widgets;

use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};

use crate::app::{App, Screen};
use crate::theme::styles;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Outer layout: body + status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    draw_body(frame, app, chunks[0]);
    widgets::status_bar::render(frame, app, chunks[1]);
}

fn draw_body(frame: &mut Frame, app: &App, area: Rect) {
    // Panel border
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(styles::border())
        .title(format!(" NQRustBackup Installer · {} ", app.screen.title()))
        .title_style(styles::primary_bold());
    let inner = block.inner(area);
    frame.render_widget(block, area);

    match app.screen {
        Screen::Welcome => screens::welcome::render(frame, app, inner),
        Screen::Preflight => screens::preflight::render(frame, app, inner),
        Screen::SourceSelect => screens::source_select::render(frame, app, inner),
        Screen::ProfileSelect => screens::profile_select::render(frame, app, inner),
        Screen::Config => screens::config_screen::render(frame, app, inner),
        Screen::Progress => screens::progress::render(frame, app, inner),
        Screen::Verify => screens::verify::render(frame, app, inner),
        Screen::Complete => screens::complete::render(frame, app, inner),
        Screen::Error => screens::error::render(frame, app, inner),
    }
}

pub async fn handle_key(app: &mut App, key: KeyEvent) {
    match app.screen {
        Screen::Welcome => screens::welcome::handle(app, key).await,
        Screen::Preflight => screens::preflight::handle(app, key).await,
        Screen::SourceSelect => screens::source_select::handle(app, key).await,
        Screen::ProfileSelect => screens::profile_select::handle(app, key).await,
        Screen::Config => screens::config_screen::handle(app, key).await,
        Screen::Progress => screens::progress::handle(app, key).await,
        Screen::Verify => screens::verify::handle(app, key).await,
        Screen::Complete => screens::complete::handle(app, key).await,
        Screen::Error => screens::error::handle(app, key).await,
    }
}
