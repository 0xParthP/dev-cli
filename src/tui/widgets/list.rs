//! Shared helpers for list-based TUI widgets.

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::tui::theme::Palette;

/// Render a styled selectable list used throughout the dashboard.
#[allow(clippy::too_many_arguments)]
pub fn render_list(
    frame: &mut Frame,
    area: Rect,
    title: String,
    items: Vec<ListItem>,
    selected: Option<usize>,
    palette: Palette,
) {
    render_list_with_symbol(frame, area, title, items, selected, "❯ ", palette);
}

/// Render a styled selectable list with a customizable highlight symbol.
#[allow(clippy::too_many_arguments)]
pub fn render_list_with_symbol(
    frame: &mut Frame,
    area: Rect,
    title: String,
    items: Vec<ListItem>,
    selected: Option<usize>,
    symbol: &str,
    palette: Palette,
) {
    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(palette.border)),
        )
        .highlight_style(Style::default().bg(palette.highlight_bg).add_modifier(Modifier::BOLD))
        .highlight_symbol(symbol);

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
pub fn render_centered_notice(frame: &mut Frame, area: Rect, notice: Notice<'_>, palette: Palette) {
    let widget = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(notice.icon, Style::default().fg(notice.icon_color))),
        Line::from(""),
        Line::from(Span::styled(
            notice.heading,
            Style::default().fg(palette.text).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(notice.subtext, Style::default().fg(palette.muted))),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .title(notice.title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette.border)),
    );

    frame.render_widget(widget, area);
}

/// Helper to split a string at a character cursor offset into:
/// `(before_str, cursor_char, after_str)`
pub fn split_at_cursor(s: &str, cursor: usize) -> (&str, &str, &str) {
    let char_count = s.chars().count();
    let cursor = cursor.min(char_count);
    let byte_idx = s.char_indices().nth(cursor).map(|(i, _)| i).unwrap_or(s.len());
    let before = &s[..byte_idx];
    if cursor < char_count {
        let next_byte_idx = s.char_indices().nth(cursor + 1).map(|(i, _)| i).unwrap_or(s.len());
        let cursor_char = &s[byte_idx..next_byte_idx];
        let after = &s[next_byte_idx..];
        (before, cursor_char, after)
    } else {
        (before, " ", "")
    }
}

/// Helper to append common Submit/Cancel action buttons and optional status error line to form lines.
pub fn append_form_footer<'a>(
    lines: &mut Vec<Line<'a>>,
    status_message: Option<&str>,
    palette: Palette,
) {
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("          "),
        Span::styled("Enter", Style::default().fg(palette.success)),
        Span::raw(" Submit    "),
        Span::styled("Esc", Style::default().fg(palette.danger)),
        Span::raw(" Cancel"),
    ]));

    if let Some(msg) = status_message
        && msg.starts_with('✗')
    {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("   {msg}"),
            Style::default().fg(palette.danger),
        )));
    }
}
