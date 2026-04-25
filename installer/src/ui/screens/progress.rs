//! Install-in-progress screen: phase list + streaming log.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::{
    app::{App, CheckStatus, Screen},
    theme::styles,
    ui::widgets,
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    // Sync phase statuses from the shared executor status
    // (Note: we can't mutate app here since this fn takes &App. We snapshot instead.)
    let phases_with_status: Vec<_> = match &app.exec_status {
        Some(arc) => {
            let s = arc.lock().unwrap();
            app.phases
                .iter()
                .zip(s.iter())
                .map(|((p, _), st)| (p.clone(), *st))
                .collect()
        }
        None => app.phases.clone(),
    };

    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(area);

    let done = phases_with_status
        .iter()
        .filter(|(_, s)| matches!(s, CheckStatus::Ok))
        .count();
    let total = phases_with_status.len();
    let header = Paragraph::new(Line::from(vec![Span::styled(
        format!("Installing… phases complete: {done}/{total}"),
        styles::text(),
    )]));
    frame.render_widget(header, v[0]);

    let split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(v[1]);

    let current = phases_with_status
        .iter()
        .position(|(_, s)| matches!(s, CheckStatus::Running))
        .unwrap_or(0);
    widgets::phase_progress::render(
        frame,
        split[0],
        &phases_with_status,
        current,
        app.spinner_tick,
        &app.log,
    );
    widgets::log_viewer::render(frame, split[1], &app.log);

    let any_fail = phases_with_status
        .iter()
        .any(|(_, s)| matches!(s, CheckStatus::Fail));
    let all_ok = total > 0
        && phases_with_status
            .iter()
            .all(|(_, s)| matches!(s, CheckStatus::Ok));

    let hint = if any_fail {
        Line::from(vec![
            Span::styled("Install failed. ", styles::error()),
            Span::styled("Enter", styles::primary_bold()),
            Span::styled(" — view error", styles::muted()),
        ])
    } else if all_ok {
        Line::from(vec![
            Span::styled("Install phases complete. ", styles::success()),
            Span::styled("Enter", styles::primary_bold()),
            Span::styled(" — verify", styles::muted()),
        ])
    } else {
        Line::from(vec![Span::styled(
            "Please wait… logs stream on the right.",
            styles::muted(),
        )])
    };
    frame.render_widget(Paragraph::new(hint), v[2]);
}

pub async fn handle(app: &mut App, key: KeyEvent) {
    // Pull latest status into app.phases so next render is correct
    if let Some(arc) = app.exec_status.clone() {
        let s = arc.lock().unwrap();
        for (i, ph) in app.phases.iter_mut().enumerate() {
            if let Some(st) = s.get(i) {
                ph.1 = *st;
            }
        }
    }

    let any_fail = app
        .phases
        .iter()
        .any(|(_, s)| matches!(s, CheckStatus::Fail));
    let all_ok =
        !app.phases.is_empty() && app.phases.iter().all(|(_, s)| matches!(s, CheckStatus::Ok));

    match key.code {
        KeyCode::Enter if any_fail => {
            app.error_message =
                Some("Install failed. See log above for the first failing phase.".into());
            app.executor_running = false;
            app.screen = Screen::Error;
        }
        KeyCode::Enter if all_ok => {
            app.executor_running = false;
            app.screen = Screen::Verify;
        }
        _ => {}
    }
}
