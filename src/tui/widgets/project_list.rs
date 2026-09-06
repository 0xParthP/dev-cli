//! Project list widget.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::tui::{state::AppState, theme};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let projects = state.filtered_projects();

    let items: Vec<ListItem> = projects
        .iter()
        .enumerate()
        .map(|(index, project)| {
            let prefix = if index == state.selected_index { "▶ " } else { "  " };

            ListItem::new(format!("{prefix}{}", project.name))
        })
        .collect();

    let widget = List::new(items)
        .block(Block::default().title(" Projects ").borders(Borders::ALL))
        .highlight_style(Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD));

    frame.render_widget(widget, area);
}
