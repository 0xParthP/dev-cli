//! Rendering for the dashboard.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    prelude::Stylize,
    widgets::Block,
};

use super::{state::AppState, theme, widgets};

pub fn render(frame: &mut Frame, _state: &AppState) {
    frame.render_widget(Block::default().bg(theme::BACKGROUND), frame.area());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .split(frame.area());

    widgets::header::render(frame, layout[0]);
    widgets::search::render(frame, layout[1]);
    widgets::project_list::render(frame, layout[2]);
    widgets::footer::render(frame, layout[3]);
}
