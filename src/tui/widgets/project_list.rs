//! Project list widget.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::{
    tui::{
        state::AppState,
        theme,
        widgets::list::{Notice, render_centered_notice, render_list},
    },
    utils::path::display_path,
};

/// Render the widget onto the given frame and area.
pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    // Route to the correct tab.
    let projects = state.filtered_projects();
    let title = format!(" Projects ({}) ", projects.len());

    if projects.is_empty() {
        render_centered_notice(
            frame,
            area,
            Notice {
                title: &title,
                icon: "📂",
                icon_color: theme::MUTED,
                heading: "No projects found",
                subtext: "Try another search.",
            },
        );
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
