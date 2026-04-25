//! Welcome screen.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::{
    app::{App, Screen},
    theme::{styles, PRODUCT_NAME, VERSION},
};

pub fn render(frame: &mut Frame, _app: &App, area: Rect) {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(6),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area);

    let banner = Paragraph::new(Line::from(vec![
        Span::styled(PRODUCT_NAME, styles::primary_bold()),
        Span::styled(" Installer ", styles::text()),
        Span::styled(format!("v{VERSION}"), styles::muted()),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(banner, v[0]);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "A guided installer for the NQRustBackup server stack.",
            styles::secondary(),
        )),
        Line::from(Span::styled(
            "Director · Storage · FileDaemon · Catalog · WebUI",
            styles::accent(),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Targets Debian / Ubuntu. For other hosts, a ",
                styles::muted(),
            ),
            Span::styled("--dry-run", styles::info()),
            Span::styled(" flag is available on the CLI subcommand.", styles::muted()),
        ]),
    ];
    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), v[1]);

    let prompt = Paragraph::new(Line::from(vec![
        Span::styled("Press ", styles::muted()),
        Span::styled("Enter", styles::primary_bold()),
        Span::styled(" to begin, ", styles::muted()),
        Span::styled("q", styles::primary_bold()),
        Span::styled(" to quit.", styles::muted()),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(prompt, v[3]);
}

pub async fn handle(app: &mut App, key: KeyEvent) {
    if key.code == KeyCode::Enter {
        app.screen = Screen::Preflight;
    }
}
