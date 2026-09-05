//! Footer widget.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::Paragraph,
};

use crate::tui::theme;

pub fn render(frame: &mut Frame, area: Rect) {
    let widget = Paragraph::new("Enter Open    / Search    Q Quit")
        .style(Style::default().fg(theme::SUCCESS).add_modifier(Modifier::BOLD));

    frame.render_widget(widget, area);
}
