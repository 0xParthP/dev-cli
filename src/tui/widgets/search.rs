//! Search widget.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::{state::AppState, theme};

/// Render the widget onto the given frame and area.
pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let text = if state.search_query.is_empty() {
        Line::from(vec![
            Span::styled("🔍 ", Style::default().fg(theme::PRIMARY)),
            Span::styled(
                "Search projects...",
                Style::default().fg(theme::MUTED).add_modifier(Modifier::ITALIC),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled("🔍 ", Style::default().fg(theme::PRIMARY)),
            Span::styled(&state.search_query, Style::default().fg(theme::TEXT)),
            Span::styled("█", Style::default().fg(theme::PRIMARY)),
        ])
    };

    let widget = Paragraph::new(text).style(Style::default().bg(theme::BACKGROUND)).block(
        Block::default()
            .title(" Search ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::BORDER)),
    );

    frame.render_widget(widget, area);
}
