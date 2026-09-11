//! Placeholder widget for unimplemented tabs.

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::{state::Tab, theme};

/// Render a coming soon placeholder for unimplemented tabs.
pub fn render(frame: &mut Frame, area: Rect, tab: Tab) {
    let title = match tab {
        Tab::Ide => " IDE ",
        Tab::Settings => " Settings ",
        _ => " Coming Soon ",
    };

    let placeholder = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled("🚧", Style::default().fg(theme::WARNING))),
        Line::from(""),
        Line::from(Span::styled(
            "Coming Soon",
            Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "This tab will be implemented in a later milestone.",
            Style::default().fg(theme::MUTED),
        )),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::BORDER)),
    );

    frame.render_widget(placeholder, area);
}
