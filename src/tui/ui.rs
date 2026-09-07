//! Rendering for the dashboard.

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::Stylize,
    widgets::Block,
};

use crate::tui::widgets::{footer, header, project_list, search, tabs};

use super::{
    state::{AppState, Tab},
    theme,
};

pub fn render(frame: &mut Frame, state: &AppState) {
    frame.render_widget(Block::default().bg(theme::BACKGROUND), frame.area());

    let chunks = Layout::vertical([
        Constraint::Length(2), // Header
        Constraint::Length(2), // Tabs
        Constraint::Length(3), // Search (only used on Projects tab)
        Constraint::Min(1),    // Content
        Constraint::Length(1), // Footer
    ])
    .split(frame.area());

    header::render(frame, chunks[0]);
    tabs::render(frame, chunks[1], state);

    match state.active_tab {
        Tab::Projects => {
            search::render(frame, chunks[2], state);
            project_list::render(frame, chunks[3], state);
        }

        _ => {
            // Leave the search area blank for other tabs.
            frame.render_widget(Block::default(), chunks[2]);
            project_list::render(frame, chunks[3], state);
        }
    }

    footer::render(frame, chunks[4]);
}
