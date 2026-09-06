//! Footer widget.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::theme;

pub fn render(frame: &mut Frame, area: Rect) {
    let footer = Line::from(vec![
        Span::styled("⏎ Enter", Style::default().fg(theme::SUCCESS).add_modifier(Modifier::BOLD)),
        Span::styled(" Open   ", Style::default().fg(theme::TEXT)),
        Span::styled("↑↓", Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD)),
        Span::styled(" Navigate   ", Style::default().fg(theme::TEXT)),
        Span::styled("/", Style::default().fg(theme::WARNING).add_modifier(Modifier::BOLD)),
        Span::styled(" Search   ", Style::default().fg(theme::TEXT)),
        Span::styled("Q", Style::default().fg(theme::DANGER).add_modifier(Modifier::BOLD)),
        Span::styled(" Quit", Style::default().fg(theme::TEXT)),
    ]);

    let widget = Paragraph::new(footer).style(Style::default().bg(theme::SURFACE));

    frame.render_widget(widget, area);
}
