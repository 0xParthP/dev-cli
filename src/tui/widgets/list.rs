//! Shared helpers for list-based TUI widgets.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::tui::theme;

/// Render a styled selectable list used throughout the dashboard.
pub fn render_list(
    frame: &mut Frame,
    area: Rect,
    title: String,
    items: Vec<ListItem>,
    selected: Option<usize>,
) {
    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme::BORDER)),
        )
        .highlight_style(
            Style::default().bg(theme::HIGHLIGHT_BG).fg(theme::TEXT).add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("❯ ");

    let mut state = ListState::default();
    state.select(selected);

    frame.render_stateful_widget(list, area, &mut state);
}
