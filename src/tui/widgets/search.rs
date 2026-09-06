//! Search widget.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::{state::AppState, theme};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let text = if state.search_query.is_empty() {
        "Search projects...".into()
    } else {
        state.search_query.clone()
    };

    let widget = Paragraph::new(text)
        .block(Block::default().title(" Search ").borders(Borders::ALL))
        .style(Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD));

    frame.render_widget(widget, area);
}
