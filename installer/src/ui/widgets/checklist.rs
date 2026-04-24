//! Checklist widget — renders a slice of PreflightCheck with status glyphs.

use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{List, ListItem},
    Frame,
};

use crate::{
    app::{CheckStatus, PreflightCheck},
    theme::{styles, symbols},
};

pub fn render(frame: &mut Frame, area: Rect, items: &[PreflightCheck], spinner_tick: usize) {
    let rows: Vec<ListItem> = items
        .iter()
        .map(|c| {
            let (sym, style) = match c.status {
                CheckStatus::Pending => (symbols::PENDING, styles::muted()),
                CheckStatus::Running => {
                    let s = symbols::SPINNER[spinner_tick % symbols::SPINNER.len()];
                    (s, styles::info())
                }
                CheckStatus::Ok => (symbols::CHECK, styles::success()),
                CheckStatus::Warn => (symbols::ARROW_RIGHT, styles::warning()),
                CheckStatus::Fail => (symbols::CROSS, styles::error()),
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!(" {sym} "), style),
                Span::styled(format!("{:30}", c.name), styles::text()),
                Span::styled(&c.detail, styles::muted()),
            ]))
        })
        .collect();

    frame.render_widget(List::new(rows), area);
}
