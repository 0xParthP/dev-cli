//! Footer widget.

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::theme;

pub fn render(frame: &mut Frame, area: Rect) {
    let line = Line::from(vec![
        Span::styled("↵ Enter", Style::default().fg(theme::SUCCESS)),
        Span::raw("  "),
        Span::styled("↑↓ Navigate", Style::default().fg(theme::PRIMARY)),
        Span::raw("  "),
        Span::styled("/ Search", Style::default().fg(theme::WARNING)),
        Span::raw("  "),
        Span::styled("Q Quit", Style::default().fg(theme::DANGER)),
    ]);

    frame.render_widget(
        Paragraph::new(line)
            .alignment(Alignment::Center)
            .style(Style::default().add_modifier(Modifier::BOLD)),
        area,
    );
}
