//! Placeholder project list.

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, area: Rect) {
    let widget = Paragraph::new("Projects will appear here in Phase 4.2.\n\nUse q or Esc to quit.")
        .block(Block::default().borders(Borders::ALL).title("Projects"));

    frame.render_widget(widget, area);
}
