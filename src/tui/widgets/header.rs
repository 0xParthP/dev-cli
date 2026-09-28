//! Dashboard header.

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::theme::Palette;

/// Render the widget onto the given frame and area.
pub fn render(frame: &mut Frame, area: Rect, palette: Palette) {
    let title = Line::from(vec![
        Span::raw("🚀 "),
        Span::styled("dev-cli", Style::default().fg(palette.primary).add_modifier(Modifier::BOLD)),
    ]);

    frame.render_widget(Paragraph::new(title).alignment(Alignment::Center), area);
}
