//! Pre-flight screen.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::{
    app::{App, Screen},
    installer::preflight,
    theme::styles,
    ui::widgets,
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(area);

    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            "Checking this host for install prerequisites…",
            styles::text(),
        ),
    ]));
    frame.render_widget(header, v[0]);

    if app.preflight.is_empty() {
        let p = Paragraph::new(Line::from(Span::styled(
            "  (press Enter to run checks)",
            styles::muted(),
        )));
        frame.render_widget(p, v[1]);
    } else {
        widgets::checklist::render(frame, v[1], &app.preflight, app.spinner_tick);
    }

    let can_proceed = preflight::all_ok(&app.preflight);
    let hint = if app.preflight.is_empty() {
        Line::from(vec![
            Span::styled("Enter", styles::primary_bold()),
            Span::styled(" — run checks · ", styles::muted()),
            Span::styled("Esc", styles::primary_bold()),
            Span::styled(" — back", styles::muted()),
        ])
    } else if can_proceed {
        Line::from(vec![
            Span::styled("Enter", styles::primary_bold()),
            Span::styled(" — continue · ", styles::muted()),
            Span::styled("r", styles::primary_bold()),
            Span::styled(" — re-run · ", styles::muted()),
            Span::styled("Esc", styles::primary_bold()),
            Span::styled(" — back", styles::muted()),
        ])
    } else {
        Line::from(vec![
            Span::styled("Blocked by failed checks. ", styles::error()),
            Span::styled("r", styles::primary_bold()),
            Span::styled(" — re-run · ", styles::muted()),
            Span::styled("Esc", styles::primary_bold()),
            Span::styled(" — back", styles::muted()),
        ])
    };
    frame.render_widget(Paragraph::new(hint), v[2]);
}

pub async fn handle(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.screen = Screen::Welcome,
        KeyCode::Char('r') | KeyCode::Enter if app.preflight.is_empty() => {
            app.preflight =
                preflight::run_all_checks(&app.config.config_dir, &app.config.storage_dir);
        }
        KeyCode::Char('r') => {
            app.preflight.clear();
            app.preflight =
                preflight::run_all_checks(&app.config.config_dir, &app.config.storage_dir);
        }
        KeyCode::Enter => {
            if preflight::all_ok(&app.preflight) {
                app.screen = Screen::SourceSelect;
            }
        }
        _ => {}
    }
}
