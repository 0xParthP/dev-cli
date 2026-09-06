//! Search widget.

use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::{state::AppState, theme};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let placeholder =
        if state.search_query.is_empty() { "Search projects..." } else { &state.search_query };

    frame.render_widget(
        Paragraph::new(placeholder).block(
            Block::default()
                .title(" Search ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme::PRIMARY)),
        ),
        area,
    );
}
