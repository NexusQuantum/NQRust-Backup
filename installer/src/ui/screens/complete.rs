//! Completion screen.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::{
    app::App,
    theme::{styles, symbols, PRODUCT_NAME},
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area);

    let banner = Paragraph::new(Line::from(vec![
        Span::styled(format!("{} ", symbols::CHECK), styles::success()),
        Span::styled(format!("{PRODUCT_NAME} installed"), styles::primary_bold()),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(banner, v[0]);

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled("Next steps:", styles::accent())),
        Line::from(""),
        Line::from(vec![
            Span::styled("  • ", styles::muted()),
            Span::styled("bconsole", styles::primary_bold()),
            Span::styled(" — open the admin console", styles::muted()),
        ]),
    ];
    if app.config.profile.installs_director() {
        lines.push(Line::from(vec![
            Span::styled("  • Run a test backup: ", styles::muted()),
            Span::styled(
                "echo -e 'label pool=EvalPool volume=vol-0001\\nrun job=BackupClient1 yes\\nwait\\nquit' | bconsole",
                styles::accent(),
            ),
        ]));
    }
    if let Some(url) = &app.complete_url {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("  • WebUI: ", styles::muted()),
            Span::styled(url.clone(), styles::accent()),
        ]));
        lines.push(Line::from(vec![
            Span::styled("    (log in with the ", styles::muted()),
            Span::styled("admin", styles::primary_bold()),
            Span::styled(
                " console profile — password in /etc/bareos/bareos-dir.d/console/admin.conf)",
                styles::muted(),
            ),
        ]));
    }
    frame.render_widget(Paragraph::new(lines), v[1]);

    let prompt = Paragraph::new(Line::from(vec![
        Span::styled("Press ", styles::muted()),
        Span::styled("Enter", styles::primary_bold()),
        Span::styled(" or ", styles::muted()),
        Span::styled("q", styles::primary_bold()),
        Span::styled(" to quit.", styles::muted()),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(prompt, v[3]);
}

pub async fn handle(app: &mut App, key: KeyEvent) {
    if matches!(key.code, KeyCode::Enter | KeyCode::Char('q')) {
        app.should_quit = true;
    }
}
