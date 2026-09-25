//! Search widget.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::{state::AppState, theme::Palette};

/// Render the widget onto the given frame and area.
pub fn render(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    let text = if state.search_query.is_empty() {
        Line::from(vec![
            Span::styled("🔍 ", Style::default().fg(palette.primary)),
            Span::styled(" ", Style::default().bg(palette.primary).fg(palette.background)),
            Span::styled(
                "Search projects...",
                Style::default().fg(palette.muted).add_modifier(Modifier::ITALIC),
            ),
        ])
    } else {
        let (before, cursor_char, after) =
            crate::tui::widgets::list::split_at_cursor(&state.search_query, state.search_cursor);
        Line::from(vec![
            Span::styled("🔍 ", Style::default().fg(palette.primary)),
            Span::styled(before, Style::default().fg(palette.text)),
            Span::styled(cursor_char, Style::default().bg(palette.primary).fg(palette.background)),
            Span::styled(after, Style::default().fg(palette.text)),
        ])
    };

    let widget = Paragraph::new(text).style(Style::default().bg(palette.background)).block(
        Block::default()
            .title(" Search ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette.border)),
    );

    frame.render_widget(widget, area);
}
