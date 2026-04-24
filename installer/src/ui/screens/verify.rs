//! Verification screen — re-runs targeted checks after install.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::{
    app::{App, CheckStatus, Screen},
    installer::verify,
    theme::styles,
    ui::widgets,
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(1), Constraint::Length(2)])
        .split(area);

    let header = Paragraph::new(Line::from(Span::styled(
        "Verifying the installation…",
        styles::text(),
    )));
    frame.render_widget(header, v[0]);

    if app.verify.is_empty() {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "  (press Enter to run verify checks)",
                styles::muted(),
            ))),
            v[1],
        );
    } else {
        widgets::checklist::render(frame, v[1], &app.verify, app.spinner_tick);
    }

    let all_ok = !app.verify.is_empty()
        && app.verify.iter().all(|c| c.status == CheckStatus::Ok);
    let hint = if app.verify.is_empty() {
        Line::from(vec![
            Span::styled("Enter", styles::primary_bold()),
            Span::styled(" — run verify", styles::muted()),
        ])
    } else if all_ok {
        Line::from(vec![
            Span::styled("Enter", styles::primary_bold()),
            Span::styled(" — finish", styles::muted()),
        ])
    } else {
        Line::from(vec![
            Span::styled("Some checks are warnings / fails. ", styles::warning()),
            Span::styled("Enter", styles::primary_bold()),
            Span::styled(" — finish anyway · ", styles::muted()),
            Span::styled("r", styles::primary_bold()),
            Span::styled(" — re-run", styles::muted()),
        ])
    };
    frame.render_widget(Paragraph::new(hint), v[2]);
}

pub async fn handle(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('r') => {
            app.verify.clear();
            app.verify = verify::run(&app.config, &app.log).await.unwrap_or_default();
        }
        KeyCode::Enter if app.verify.is_empty() => {
            app.verify = verify::run(&app.config, &app.log).await.unwrap_or_default();
        }
        KeyCode::Enter => {
            if app.config.profile.installs_webui() {
                app.complete_url = Some(format!(
                    "http://{}:{}/bareos-webui/",
                    app.config.director_address, app.config.webui_port
                ));
            }
            app.screen = Screen::Complete;
        }
        _ => {}
    }
}
