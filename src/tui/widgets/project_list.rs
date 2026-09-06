//! Project list widget.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::tui::{state::AppState, theme};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let items: Vec<ListItem> = state
        .filtered_projects()
        .iter()
        .enumerate()
        .map(|(index, project)| {
            let style = if index == state.selected_index {
                Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(project.name.clone()).style(style)
        })
        .collect();

    frame.render_widget(
        List::new(items).block(Block::default().title(" Projects ").borders(Borders::ALL)),
        area,
    );
}
