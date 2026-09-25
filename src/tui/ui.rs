//! Rendering for the dashboard.

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::Stylize,
    widgets::Block,
};

use crate::tui::widgets::{footer, header, ide_list, project_list, recent, search, settings, tabs};

use super::state::{AppState, Tab};

/// Render the widget onto the given frame and area.
pub fn render(frame: &mut Frame, state: &AppState) {
    let palette = state.palette();
    frame.render_widget(Block::default().bg(palette.background), frame.area());

    let chunks = Layout::vertical([
        Constraint::Length(2), // Header
        Constraint::Length(2), // Tabs
        Constraint::Length(3), // Search (only used on Projects tab)
        Constraint::Min(1),    // Content
        Constraint::Length(1), // Footer
    ])
    .split(frame.area());

    header::render(frame, chunks[0], palette);
    tabs::render(frame, chunks[1], state, palette);

    match state.active_tab {
        Tab::Projects => {
            search::render(frame, chunks[2], state, palette);
            project_list::render(frame, chunks[3], state, palette);
        }
        Tab::Recent => {
            frame.render_widget(Block::default(), chunks[2]);
            recent::render(frame, chunks[3], state, palette);
        }
        Tab::Ide => {
            frame.render_widget(Block::default(), chunks[2]);
            ide_list::render(frame, chunks[3], state, palette);
        }
        Tab::Settings => {
            frame.render_widget(Block::default(), chunks[2]);
            settings::render(frame, chunks[3], state);
        }
    }

    footer::render(frame, chunks[4], state, palette);
}
