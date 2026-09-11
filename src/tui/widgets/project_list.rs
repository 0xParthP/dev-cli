//! Project list widget.

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, ListItem, Paragraph},
};

use crate::{
    tui::{state::AppState, theme, widgets::list::render_list},
    utils::path::display_path,
};

/// Render the widget onto the given frame and area.
pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    // Route to the correct tab.
    let projects = state.filtered_projects();
    let title = format!(" Projects ({}) ", projects.len());

    if projects.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("📂", Style::default().fg(theme::MUTED))),
            Line::from(""),
            Line::from(Span::styled(
                "No projects found",
                Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled("Try another search.", Style::default().fg(theme::MUTED))),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme::BORDER)),
        );

        frame.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = projects
        .iter()
        .map(|project| {
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled("📁 ", Style::default().fg(theme::WARNING)),
                    Span::styled(
                        &project.name,
                        Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("   "),
                    Span::styled(display_path(&project.path), Style::default().fg(theme::MUTED)),
                ]),
                Line::default(),
            ])
        })
        .collect();

    render_list(frame, area, title, items, Some(state.selected_index));
}
