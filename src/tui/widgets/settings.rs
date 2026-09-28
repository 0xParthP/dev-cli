//! Settings tab widget.

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, ListItem, Paragraph, Wrap},
};

use crate::{
    tui::{
        state::{AppState, InputMode, SettingsTabFocus},
        theme::Palette,
        widgets::list::render_list,
    },
    utils::path::display_path,
};

/// Render the Settings tab.
pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let palette = state.palette();
    match state.settings_tab_focus {
        SettingsTabFocus::List => render_settings_list(frame, area, state, palette),
        SettingsTabFocus::AddRoot => render_split_view(frame, area, state, palette),
    }
}

/// Helper to build list items for project roots.
fn build_root_list_items(state: &AppState, palette: Palette) -> Vec<ListItem<'static>> {
    state
        .project_roots
        .iter()
        .map(|root| {
            let path_str = display_path(root);
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled("📁  ", Style::default().fg(palette.warning)),
                    Span::styled(
                        path_str,
                        Style::default().fg(palette.text).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::default(),
            ])
        })
        .collect()
}

/// Render the settings list with preferences cards.
fn render_settings_list(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    let (main_area, status_area) = if let Some(ref msg) = state.settings_status_message {
        let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(area);
        (chunks[0], Some((chunks[1], msg)))
    } else {
        (area, None)
    };

    let vertical_chunks =
        Layout::vertical([Constraint::Percentage(45), Constraint::Percentage(55)]).split(main_area);

    // Top: Project Roots list
    let title = format!(" Project Roots ({}) ", state.project_roots.len());
    let list_items = build_root_list_items(state, palette);

    let selected = if state.project_roots.is_empty() {
        None
    } else {
        Some(state.selected_index.min(state.project_roots.len().saturating_sub(1)))
    };

    render_list(frame, vertical_chunks[0], title, list_items, selected, palette);

    // Bottom: Preferences (left) + Diagnostics (right)
    let bottom_chunks =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(vertical_chunks[1]);

    render_preferences_card(frame, bottom_chunks[0], state, palette);
    render_diagnostics_card(frame, bottom_chunks[1], state, palette);

    if let Some((area, msg)) = status_area {
        let color = if msg.starts_with('✓') { palette.success } else { palette.danger };
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(format!(" {msg}"), Style::default().fg(color)))),
            area,
        );
    }
}

/// Render preferences & search settings card.
fn render_preferences_card(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    let ignores_str = if state.ignore_patterns.is_empty() {
        "None".to_string()
    } else {
        state.ignore_patterns.join(", ")
    };

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  [t] Theme Palette:     ", Style::default().fg(palette.info)),
            Span::styled(
                palette.name,
                Style::default().fg(palette.primary).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  [m] Max Scan Depth:    ", Style::default().fg(palette.info)),
            Span::styled(format!("{} levels", state.max_depth), Style::default().fg(palette.text)),
        ]),
        Line::from(vec![
            Span::styled("  [l] Recents Limit:     ", Style::default().fg(palette.info)),
            Span::styled(
                format!("{} items", state.recent_projects_limit),
                Style::default().fg(palette.text),
            ),
        ]),
        Line::from(vec![
            Span::styled("  [r] Auto Refresh:      ", Style::default().fg(palette.info)),
            Span::styled(
                if state.auto_refresh_on_launch { "Enabled" } else { "Disabled" },
                Style::default().fg(if state.auto_refresh_on_launch {
                    palette.success
                } else {
                    palette.muted
                }),
            ),
        ]),
        Line::from(vec![Span::styled("  Ignored Directories:", Style::default().fg(palette.info))]),
        Line::from(vec![Span::styled(
            format!("  {}", ignores_str),
            Style::default().fg(palette.muted),
        )]),
    ];

    let block = Block::default()
        .title(" Preferences & Scanner ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(palette.border));

    frame.render_widget(Paragraph::new(lines).block(block).wrap(Wrap { trim: true }), area);
}

/// Render system & config diagnostics card.
fn render_diagnostics_card(frame: &mut Frame, area: Rect, _state: &AppState, palette: Palette) {
    let config_path = crate::config::Config::path()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    let os_info = format!("{} ({})", std::env::consts::OS, std::env::consts::ARCH);

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Version:        ", Style::default().fg(palette.info)),
            Span::styled(
                format!("dev-cli v{}", env!("CARGO_PKG_VERSION")),
                Style::default().fg(palette.text).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Platform:       ", Style::default().fg(palette.info)),
            Span::styled(os_info, Style::default().fg(palette.text)),
        ]),
        Line::from(vec![Span::styled("  Config File:", Style::default().fg(palette.info))]),
        Line::from(vec![Span::styled(
            format!("  {}", config_path),
            Style::default().fg(palette.muted),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  [X] Reset Settings to Defaults",
            Style::default().fg(palette.danger).add_modifier(Modifier::BOLD),
        )]),
    ];

    let block = Block::default()
        .title(" Diagnostics & System ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(palette.border));

    frame.render_widget(Paragraph::new(lines).block(block).wrap(Wrap { trim: true }), area);
}

/// Render the split view: list on top, add form on bottom.
fn render_split_view(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    let chunks =
        Layout::vertical([Constraint::Percentage(60), Constraint::Percentage(40)]).split(area);

    // Top: project roots list (non-interactive)
    let title = format!(" Project Roots ({}) ", state.project_roots.len());
    let list_items = build_root_list_items(state, palette);
    render_list(frame, chunks[0], title, list_items, None, palette);

    // Bottom: add form
    render_add_root_form(frame, chunks[1], state, palette);
}

/// Render the "Add Project Root" form.
fn render_add_root_form(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    let is_editing = state.input_mode == InputMode::Editing;

    let mut line_path_spans = vec![Span::styled("   Path:  ", Style::default().fg(palette.info))];

    if is_editing {
        let (before, cursor_char, after) = crate::tui::widgets::list::split_at_cursor(
            &state.settings_add_root,
            state.settings_add_root_cursor,
        );
        line_path_spans.push(Span::styled(before, Style::default().fg(palette.text)));
        line_path_spans.push(Span::styled(
            cursor_char,
            Style::default().bg(palette.primary).fg(palette.background),
        ));
        line_path_spans.push(Span::styled(after, Style::default().fg(palette.text)));
    } else {
        line_path_spans.push(Span::styled(
            state.settings_add_root.clone(),
            Style::default().fg(palette.muted),
        ));
    }

    let mut lines = vec![Line::from(""), Line::from(line_path_spans)];
    crate::tui::widgets::list::append_form_footer(
        &mut lines,
        state.settings_status_message.as_deref(),
        palette,
    );

    let form = Paragraph::new(lines).block(
        Block::default().title(" Add Project Root ").borders(Borders::ALL).border_style(
            Style::default().fg(if is_editing { palette.primary } else { palette.border }),
        ),
    );

    frame.render_widget(form, area);
}
