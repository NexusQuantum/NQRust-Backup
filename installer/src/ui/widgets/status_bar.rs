//! Status line at the bottom of the TUI.

use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::{
    app::{App, TOTAL_STEPS},
    theme::{styles, PRODUCT_NAME, VERSION},
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let line = Line::from(vec![
        Span::styled(format!(" {} ", PRODUCT_NAME), styles::primary_bold()),
        Span::styled("│", styles::border()),
        Span::styled(format!(" {} ", app.screen.title()), styles::text()),
        Span::styled("│", styles::border()),
        Span::styled(
            format!(" Step {}/{} ", app.screen.step(), TOTAL_STEPS),
            styles::muted(),
        ),
        Span::styled("│", styles::border()),
        Span::styled(format!(" v{} ", VERSION), styles::muted()),
        Span::styled("│", styles::border()),
        Span::styled(" q=quit  ↑↓=move  Enter=select  Esc=back ", styles::muted()),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}
