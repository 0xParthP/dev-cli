//! Project list widget.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::tui::{state::AppState, theme};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let items: Vec<ListItem> = state
        .filtered_projects()
        .iter()
        .map(|project| ListItem::new(project.name.clone()))
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Projects").borders(Borders::ALL))
        .highlight_style(Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD))
        .highlight_symbol("❯ ");

    let mut list_state = ListState::default();

    if !state.filtered_projects().is_empty() {
        list_state.select(Some(state.selected_index));
    }

    frame.render_stateful_widget(list, area, &mut list_state);
}
