//! Rendering for the dashboard.

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::Stylize,
    widgets::Block,
};

use crate::tui::widgets::{footer, header, placeholder, project_list, recent, search, tabs};

use super::{
    state::{AppState, Tab},
    theme,
};

/// Render the widget onto the given frame and area.
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
        Tab::Recent => {
            frame.render_widget(Block::default(), chunks[2]);
            recent::render(frame, chunks[3], state);
        }
        Tab::Ide | Tab::Settings => {
            frame.render_widget(Block::default(), chunks[2]);
            placeholder::render(frame, chunks[3], state.active_tab);
        }
    }

    footer::render(frame, chunks[4]);
}
