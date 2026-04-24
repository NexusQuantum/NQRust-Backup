//! Install source selection.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
    Frame,
};

use crate::{
    app::{App, InstallSource, Screen},
    theme::{styles, symbols},
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(1), Constraint::Length(2)])
        .split(area);

    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "Choose where to get the NQRustBackup binaries from:",
            styles::text(),
        ))),
        v[0],
    );

    let items: Vec<ListItem> = InstallSource::ALL
        .iter()
        .enumerate()
        .map(|(i, src)| {
            let marker = if i == app.source_idx {
                symbols::ARROW_RIGHT
            } else {
                " "
            };
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(format!(" {marker} "), styles::primary_bold()),
                    Span::styled(
                        src.name(),
                        if i == app.source_idx {
                            styles::primary_bold()
                        } else {
                            styles::text()
                        },
                    ),
                ]),
                Line::from(Span::styled(
                    format!("     {}", src.description()),
                    styles::muted(),
                )),
                Line::from(""),
            ])
        })
        .collect();
    frame.render_widget(List::new(items), v[1]);

    let hint = Line::from(vec![
        Span::styled("↑↓", styles::primary_bold()),
        Span::styled(" move · ", styles::muted()),
        Span::styled("Enter", styles::primary_bold()),
        Span::styled(" select · ", styles::muted()),
        Span::styled("Esc", styles::primary_bold()),
        Span::styled(" back", styles::muted()),
    ]);
    frame.render_widget(Paragraph::new(hint), v[2]);
}

pub async fn handle(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Up => {
            if app.source_idx > 0 {
                app.source_idx -= 1;
            }
        }
        KeyCode::Down => {
            if app.source_idx + 1 < InstallSource::ALL.len() {
                app.source_idx += 1;
            }
        }
        KeyCode::Enter => {
            app.config.source = app.selected_source();
            app.screen = Screen::ProfileSelect;
        }
        KeyCode::Esc => app.screen = Screen::Preflight,
        _ => {}
    }
}
