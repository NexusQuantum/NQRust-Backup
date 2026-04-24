//! Phase progress widget — shows the list of install phases with current one highlighted.

use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::{
    app::{CheckStatus, LogRing},
    installer::executor::Phase,
    theme::{styles, symbols},
};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    phases: &[(Phase, CheckStatus)],
    current: usize,
    spinner_tick: usize,
    _log: &LogRing,
) {
    let rows: Vec<ListItem> = phases
        .iter()
        .enumerate()
        .map(|(i, (phase, status))| {
            let (sym, style) = if i == current && *status == CheckStatus::Running {
                (
                    symbols::SPINNER[spinner_tick % symbols::SPINNER.len()],
                    styles::info(),
                )
            } else {
                match status {
                    CheckStatus::Pending => (symbols::PENDING, styles::muted()),
                    CheckStatus::Running => (
                        symbols::SPINNER[spinner_tick % symbols::SPINNER.len()],
                        styles::info(),
                    ),
                    CheckStatus::Ok => (symbols::CHECK, styles::success()),
                    CheckStatus::Warn => (symbols::ARROW_RIGHT, styles::warning()),
                    CheckStatus::Fail => (symbols::CROSS, styles::error()),
                }
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!(" {:>2}. ", phase.ordinal), styles::muted()),
                Span::styled(format!("{sym} "), style),
                Span::styled(&phase.name, styles::text()),
            ]))
        })
        .collect();

    let list = List::new(rows).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(styles::border())
            .title(" Phases ")
            .title_style(styles::muted()),
    );
    frame.render_widget(list, area);
}
