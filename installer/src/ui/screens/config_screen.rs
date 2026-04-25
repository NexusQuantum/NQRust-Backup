//! Config review screen — shows the resolved install config and planned phases.

use std::sync::{Arc, Mutex};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
    Frame,
};

use crate::{
    app::{App, CheckStatus, LogLevel, Screen},
    installer::executor::{build_tasks, planned_phases},
    theme::styles,
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(area);

    let cfg = &app.config;
    let lines = vec![
        Line::from(Span::styled("Review configuration:", styles::text())),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Source:         ", styles::muted()),
            Span::styled(cfg.source.name(), styles::primary_bold()),
        ]),
        Line::from(vec![
            Span::styled("  Profile:        ", styles::muted()),
            Span::styled(cfg.profile.name(), styles::primary_bold()),
        ]),
        Line::from(vec![
            Span::styled("  Config dir:     ", styles::muted()),
            Span::styled(cfg.config_dir.display().to_string(), styles::accent()),
        ]),
        Line::from(vec![
            Span::styled("  Storage dir:    ", styles::muted()),
            Span::styled(cfg.storage_dir.display().to_string(), styles::accent()),
        ]),
        Line::from(vec![
            Span::styled("  WebUI port:     ", styles::muted()),
            Span::styled(cfg.webui_port.to_string(), styles::accent()),
        ]),
        Line::from(vec![
            Span::styled("  Director addr:  ", styles::muted()),
            Span::styled(&cfg.director_address, styles::accent()),
        ]),
    ];
    frame.render_widget(Paragraph::new(lines), v[0]);

    let phases = planned_phases(cfg.source, cfg.profile);
    let items: Vec<ListItem> = phases
        .iter()
        .map(|p| {
            ListItem::new(Line::from(vec![
                Span::styled(format!("  {:>2}. ", p.ordinal), styles::muted()),
                Span::styled(&p.name, styles::text()),
            ]))
        })
        .collect();
    frame.render_widget(List::new(items), v[1]);

    let hint = Line::from(vec![
        Span::styled("Enter", styles::primary_bold()),
        Span::styled(" run install · ", styles::muted()),
        Span::styled("Esc", styles::primary_bold()),
        Span::styled(" back", styles::muted()),
    ]);
    frame.render_widget(Paragraph::new(hint), v[2]);
}

pub async fn handle(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.screen = Screen::ProfileSelect,
        KeyCode::Enter => {
            app.phases = planned_phases(app.config.source, app.config.profile)
                .into_iter()
                .map(|p| (p, CheckStatus::Pending))
                .collect();
            app.current_phase = 0;
            app.screen = Screen::Progress;
            start_executor(app);
        }
        _ => {}
    }
}

fn start_executor(app: &mut App) {
    let cfg = app.config.clone();
    let log = app.log.clone();
    let tasks = build_tasks(&cfg);

    let status = Arc::new(Mutex::new(vec![CheckStatus::Pending; tasks.len()]));
    app.exec_status = Some(Arc::clone(&status));
    app.executor_running = true;

    tokio::spawn(async move {
        for (i, task) in tasks.into_iter().enumerate() {
            {
                let mut s = status.lock().unwrap();
                s[i] = CheckStatus::Running;
            }
            let res = task.run(&cfg, &log).await;
            {
                let mut s = status.lock().unwrap();
                s[i] = if res.is_ok() {
                    CheckStatus::Ok
                } else {
                    CheckStatus::Fail
                };
            }
            if res.is_err() {
                log.push(LogLevel::Err, "executor stopping due to failure");
                return;
            }
        }
    });
}
