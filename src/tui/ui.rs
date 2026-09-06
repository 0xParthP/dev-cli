//! Rendering for the dashboard.

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::Stylize,
    widgets::Block,
};

use crate::tui::widgets::{footer, header, project_list, search};

use super::{state::AppState, theme};

pub fn render(frame: &mut Frame, state: &AppState) {
    // Background colour for the whole terminal.
    frame.render_widget(Block::default().bg(theme::BACKGROUND), frame.area());

    let chunks = Layout::vertical([
        Constraint::Length(2), // Header
        Constraint::Length(3), // Search bar
        Constraint::Min(1),    // Project list
        Constraint::Length(1), // Footer
    ])
    .split(frame.area());

    header::render(frame, chunks[0]);
    search::render(frame, chunks[1], state);
    project_list::render(frame, chunks[2], state);
    footer::render(frame, chunks[3]);
}
