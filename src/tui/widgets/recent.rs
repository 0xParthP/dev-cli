//! Recent projects widget.

use std::time::{SystemTime, UNIX_EPOCH};
use unicode_width::UnicodeWidthStr;

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::{
    config::Config,
    tui::{state::AppState, theme},
    utils::path::display_path,
};

fn format_age(timestamp: u64) -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let diff = now.saturating_sub(timestamp);

    match diff {
        0..=59 => "Opened just now".into(),
        60..=3599 => format!("Opened {}m ago", diff / 60),
        3600..=86399 => format!("Opened {}h ago", diff / 3600),
        86400..=172799 => "Opened yesterday".into(),
        _ => format!("Opened {}d ago", diff / 86400),
    }
}

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let config = Config::load().unwrap_or_default();

    let title = format!(" Recent Projects ({}) ", config.recent_projects.len());

    if config.recent_projects.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("🕘", Style::default().fg(theme::MUTED))),
            Line::from(""),
            Line::from(Span::styled(
                "No recent projects",
                Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Open a project from the Projects tab.",
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

        frame.render_widget(empty, area);
        return;
    }

    // Width available inside the bordered list.
    let inner_width = area.width.saturating_sub(4) as usize;

    let items: Vec<ListItem> = config
        .recent_projects
        .iter()
        .map(|project| {
            let age = format_age(project.last_opened);

            let left = format!("📁 {}", project.name);

            // Use terminal display width instead of character count.
            let left_width = UnicodeWidthStr::width(left.as_str());
            let age_width = UnicodeWidthStr::width(age.as_str());

            let spacing = inner_width.saturating_sub(left_width + age_width);

            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        left,
                        Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" ".repeat(spacing)),
                    Span::styled(age, Style::default().fg(theme::INFO)),
                ]),
                Line::from(vec![
                    Span::raw("   "),
                    Span::styled(display_path(&project.path), Style::default().fg(theme::MUTED)),
                ]),
                Line::default(),
            ])
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme::BORDER)),
        )
        .highlight_style(
            Style::default().bg(theme::HIGHLIGHT_BG).fg(theme::TEXT).add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("❯ ");

    let mut list_state = ListState::default();

    // Keep the highlighted row in sync with AppState.
    let selected = state.selected_index.min(config.recent_projects.len().saturating_sub(1));

    list_state.select(Some(selected));

    frame.render_stateful_widget(list, area, &mut list_state);
}
