//! Top navigation tabs.

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::Tabs,
};

use crate::tui::{
    state::{AppState, Tab},
    theme,
};

/// Render the widget onto the given frame and area.
pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let titles = [
        Line::from(" Recent "),
        Line::from(" Projects "),
        Line::from(" IDE "),
        Line::from(" Settings "),
    ];

    let selected = match state.active_tab {
        Tab::Recent => 0,
        Tab::Projects => 1,
        Tab::Ide => 2,
        Tab::Settings => 3,
    };

    let tabs = Tabs::new(titles)
        .select(selected)
        .style(Style::default().fg(theme::MUTED))
        .highlight_style(
            Style::default().fg(theme::BACKGROUND).bg(theme::PRIMARY).add_modifier(Modifier::BOLD),
        )
        .divider(" ");

    // Centre the tab bar horizontally.
    let centered = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(44), // Width of the four tabs.
        Constraint::Fill(1),
    ])
    .split(area);

    frame.render_widget(tabs, centered[1]);
}
