//! Dashboard header.

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::{state::AppState, theme};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let title =
        Span::styled("dev-cli", Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD));

    let count = Span::styled(
        format!("{} projects", state.projects.len()),
        Style::default().fg(theme::MUTED),
    );

    let line = Line::from(vec![title, Span::raw(" "), count]);

    frame.render_widget(Paragraph::new(line).alignment(Alignment::Left), area);
}
