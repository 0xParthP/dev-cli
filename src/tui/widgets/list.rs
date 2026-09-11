//! Shared helpers for list-based TUI widgets.

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
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

/// Parameters for rendering a centered notice block.
pub struct Notice<'a> {
    pub title: &'a str,
    pub icon: &'a str,
    pub icon_color: Color,
    pub heading: &'a str,
    pub subtext: &'a str,
}

/// Render a centered notice paragraph inside a bordered block.
pub fn render_centered_notice(frame: &mut Frame, area: Rect, notice: Notice<'_>) {
    let widget = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(notice.icon, Style::default().fg(notice.icon_color))),
        Line::from(""),
        Line::from(Span::styled(
            notice.heading,
            Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(notice.subtext, Style::default().fg(theme::MUTED))),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .title(notice.title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::BORDER)),
    );

    frame.render_widget(widget, area);
}
