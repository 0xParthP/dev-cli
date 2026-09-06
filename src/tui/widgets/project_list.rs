//! Project list widget.

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::{
    tui::{
        state::{AppState, Tab},
        theme,
    },
    utils::path::display_path,
};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    // Only the Projects tab is implemented for now.
    if state.active_tab != Tab::Projects {
        let title = match state.active_tab {
            Tab::Recent => " Recent ",
            Tab::Ide => " IDE ",
            Tab::Settings => " Settings ",
            Tab::Projects => unreachable!(),
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
                "This tab will be implemented in Milestone 4.3.",
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
        return;
    }

    let projects = state.filtered_projects();
    let title = format!(" Projects ({}) ", projects.len());

    // Empty state.
    if projects.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("📂", Style::default().fg(theme::MUTED))),
            Line::from(""),
            Line::from(Span::styled(
                "No projects found",
                Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled("Try another search.", Style::default().fg(theme::MUTED))),
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

    // Each project is rendered as a two-line "card".
    let items: Vec<ListItem> = projects
        .iter()
        .map(|project| {
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled("📁 ", Style::default().fg(theme::WARNING)),
                    Span::styled(
                        &project.name,
                        Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
                    ),
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

    if !projects.is_empty() {
        list_state.select(Some(state.selected_index));
    }

    frame.render_stateful_widget(list, area, &mut list_state);
}
