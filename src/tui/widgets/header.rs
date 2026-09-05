//! Header widget.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::theme;

pub fn render(frame: &mut Frame, area: Rect) {
    let text = "🚀 dev-cli\nModern Git Project Manager";

    let widget = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Dashboard"))
        .style(Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD));

    frame.render_widget(widget, area);
}
