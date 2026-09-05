//! Search box widget.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::theme;

pub fn render(frame: &mut Frame, area: Rect) {
    let widget = Paragraph::new("> Search coming in Phase 4.3")
        .block(Block::default().borders(Borders::ALL).title("Search"))
        .style(Style::default().fg(theme::MUTED).add_modifier(Modifier::ITALIC));

    frame.render_widget(widget, area);
}
