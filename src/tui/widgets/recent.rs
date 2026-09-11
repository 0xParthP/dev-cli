//! Recent projects widget.

use std::time::{SystemTime, UNIX_EPOCH};

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    models::recent_project::RecentProject,
    tui::{
        state::AppState,
        theme,
        widgets::list::{Notice, render_centered_notice, render_list},
    },
    utils::path::display_path,
};

fn format_age(timestamp: u64) -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();

    let diff = now.saturating_sub(timestamp);

    match diff {
        0..=59 => "Opened just now".into(),
        60..=3599 => format!("Opened {}m ago", diff / 60),
        3600..=86399 => format!("Opened {}h ago", diff / 3600),
        86400..=172799 => "Opened yesterday".into(),
        _ => format!("Opened {}d ago", diff / 86400),
    }
}

/// Render the widget onto the given frame and area.
pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let recent_projects: &[RecentProject] = &state.recent_projects;

    let title = format!(" Recent Projects ({}) ", recent_projects.len());

    if recent_projects.is_empty() {
        render_centered_notice(
            frame,
            area,
            Notice {
                title: &title,
                icon: "🕘",
                icon_color: theme::MUTED,
                heading: "No recent projects",
                subtext: "Open a project from the Projects tab.",
            },
        );
        return;
    }

    let inner_width = area.width.saturating_sub(4) as usize;

    let items: Vec<ListItem> = recent_projects
        .iter()
        .map(|project| {
            let age = format_age(project.last_opened);
            let left = format!("📁 {}", project.name);

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

    let selected = state.selected_index.min(recent_projects.len().saturating_sub(1));

    render_list(frame, area, title, items, Some(selected));
}
