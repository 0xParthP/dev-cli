//! Dashboard header.

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::theme;

/// Render the widget onto the given frame and area.
pub fn render(frame: &mut Frame, area: Rect) {
    let title = Line::from(vec![
        Span::raw("🚀 "),
        Span::styled("dev-cli", Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD)),
    ]);

    frame.render_widget(Paragraph::new(title).alignment(Alignment::Center), area);
}
