//! Scrolling log viewer — renders the LogRing's tail.

use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{
    app::{LogLevel, LogRing},
    theme::styles,
};

pub fn render(frame: &mut Frame, area: Rect, log: &LogRing) {
    let snap = log.snapshot();
    // Take only the lines that will fit in the area (area.height rows)
    let rows = area.height.saturating_sub(2) as usize;
    let start = snap.len().saturating_sub(rows);
    let lines: Vec<Line> = snap[start..]
        .iter()
        .map(|(ts, lvl, msg)| {
            let ts = ts.format("%H:%M:%S").to_string();
            let (tag, style) = match lvl {
                LogLevel::Info => ("INFO", styles::info()),
                LogLevel::Ok => ("OK  ", styles::success()),
                LogLevel::Warn => ("WARN", styles::warning()),
                LogLevel::Err => ("ERR ", styles::error()),
            };
            Line::from(vec![
                Span::styled(format!(" {ts} "), styles::muted()),
                Span::styled(format!("{tag} "), style),
                Span::styled(msg.clone(), styles::text()),
            ])
        })
        .collect();

    let para = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(styles::border())
                .title(" Log ")
                .title_style(styles::muted()),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(para, area);
}
