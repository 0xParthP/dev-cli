//! Dashboard header.

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::theme;

pub fn render(frame: &mut Frame, area: Rect) {
    let top = Line::from(vec![
        Span::styled("🚀 ", Style::default().fg(theme::PRIMARY)),
        Span::styled("dev-cli", Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD)),
    ]);

    let bottom =
        Line::from(Span::styled("Fast project launcher", Style::default().fg(theme::MUTED)));

    let header = Paragraph::new(vec![top, bottom]).alignment(Alignment::Left);

    frame.render_widget(header, area);
}
