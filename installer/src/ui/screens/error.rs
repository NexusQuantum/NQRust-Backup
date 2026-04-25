//! Error screen.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::{
    app::App,
    theme::{styles, symbols},
    ui::widgets,
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(area);

    let msg = app
        .error_message
        .clone()
        .unwrap_or_else(|| "Unknown error".to_string());
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(format!("{} ", symbols::CROSS), styles::error()),
            Span::styled("Install failed", styles::error()),
        ]),
        Line::from(""),
        Line::from(Span::styled(msg, styles::text())),
    ])
    .wrap(Wrap { trim: false });
    frame.render_widget(header, v[0]);

    widgets::log_viewer::render(frame, v[1], &app.log);

    let hint = Line::from(vec![
        Span::styled("q", styles::primary_bold()),
        Span::styled(" — quit and inspect logs on the host", styles::muted()),
    ]);
    frame.render_widget(Paragraph::new(hint), v[2]);
}

pub async fn handle(app: &mut App, key: KeyEvent) {
    if matches!(key.code, KeyCode::Char('q') | KeyCode::Enter | KeyCode::Esc) {
        app.should_quit = true;
    }
}
