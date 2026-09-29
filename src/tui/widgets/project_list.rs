//! Project list widget — shows project tree or clone form split-view.

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, ListItem, Paragraph},
};

use crate::{
    tui::{
        state::{AppState, InputMode, ProjectsTabFocus},
        theme::Palette,
        widgets::list::{Notice, render_centered_notice, render_field_spans, render_list},
    },
    utils::path::display_path,
};
use unicode_width::UnicodeWidthStr;

/// Render the widget onto the given frame and area.
pub fn render(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    match state.projects_tab_focus {
        ProjectsTabFocus::List => render_project_list(frame, area, state, palette),
        ProjectsTabFocus::CloneForm => render_split_view(frame, area, state, palette),
    }
}

/// Render the project tree list.
fn render_project_list(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    let items = state.visible_items();
    let count = state.filtered_projects().len();
    let title = format!(" Projects ({count}) ");

    if items.is_empty() {
        render_centered_notice(
            frame,
            area,
            Notice {
                title: &title,
                icon: "📂",
                icon_color: palette.muted,
                heading: "No projects found",
                subtext: "Try another search.",
            },
            palette,
        );
        return;
    }

    let available_width = area.width.saturating_sub(4) as usize;

    let list_items: Vec<ListItem> = items
        .iter()
        .map(|node| {
            let indent = "  ".repeat(node.depth);

            let (icon, icon_color, path_padding) = if node.is_folder {
                let icon_str = if node.is_expanded { "[-] 📂 " } else { "[+] 📁 " };
                (icon_str, palette.warning, "       ")
            } else {
                ("📦 ", palette.info, "   ")
            };

            let path_str = display_path(node.path);

            let mut line2_spans = vec![
                Span::raw(indent.clone()),
                Span::raw(path_padding),
                Span::styled(path_str.clone(), Style::default().fg(palette.muted)),
            ];

            if !node.is_folder {
                let ide = state.get_project_ide(node.path);
                let badge = format!("[{}]", ide.display_name(&state.custom_ides));
                let left_len = UnicodeWidthStr::width(indent.as_str())
                    + UnicodeWidthStr::width(path_padding)
                    + UnicodeWidthStr::width(path_str.as_str());
                let badge_len = UnicodeWidthStr::width(badge.as_str());

                let spaces = if available_width > left_len + badge_len {
                    available_width - left_len - badge_len
                } else {
                    2
                };

                line2_spans.push(Span::raw(" ".repeat(spaces)));
                line2_spans.push(Span::styled(
                    badge,
                    Style::default().fg(ide.color(&state.custom_ides)).add_modifier(Modifier::BOLD),
                ));
            }

            ListItem::new(vec![
                Line::from(vec![
                    Span::raw(indent),
                    Span::styled(icon, Style::default().fg(icon_color)),
                    Span::styled(
                        node.name,
                        Style::default().fg(palette.text).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(line2_spans),
                Line::default(),
            ])
        })
        .collect();

    render_list(frame, area, title, list_items, Some(state.selected_index), palette);
}

/// Render the split view: project list (top) + clone form (bottom).
fn render_split_view(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    let chunks =
        Layout::vertical([Constraint::Percentage(55), Constraint::Percentage(45)]).split(area);

    render_project_list(frame, chunks[0], state, palette);
    render_clone_form(frame, chunks[1], state, palette);
}

/// Render the Clone Repository form (matches add-root / add-custom-IDE style).
fn render_clone_form(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    let is_cloning = state.clone_task.is_some();
    let has_logs = !state.clone_log_lines.is_empty();

    // When cloning is active or logs are present, split the form vertically:
    // top half = input fields, bottom half = git log output.
    let (fields_area, log_area) = if is_cloning || has_logs {
        let chunks = Layout::vertical([Constraint::Min(8), Constraint::Min(4)]).split(area);
        (chunks[0], Some(chunks[1]))
    } else {
        (area, None)
    };

    render_clone_fields(frame, fields_area, state, palette);

    if let Some(log_rect) = log_area {
        render_clone_log(frame, log_rect, state, palette);
    }
}

/// Render the input fields section of the clone form.
fn render_clone_fields(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    let is_editing = state.input_mode == InputMode::Editing;
    let is_cloning = state.clone_task.is_some();

    // Field 0: Repository URL
    let url_active = state.clone_field == 0 && !is_cloning;
    let mut line_url = vec![Span::styled("   URL:   ", Style::default().fg(palette.info))];
    line_url.extend(render_field_spans(
        &state.clone_url,
        state.clone_url_cursor,
        url_active,
        is_editing && !is_cloning,
        palette,
    ));

    // Field 1: Target Root Selector (not a text field — use arrow keys)
    let root_active = state.clone_field == 1 && !is_cloning;
    let root_label_style = if root_active && is_editing {
        Style::default().fg(palette.primary).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(palette.info)
    };
    let root_value = if state.project_roots.is_empty() {
        "No roots configured".to_string()
    } else {
        let idx = state.clone_root_index.min(state.project_roots.len() - 1);
        format!(
            "← [{}/{}] {} →",
            idx + 1,
            state.project_roots.len(),
            display_path(&state.project_roots[idx])
        )
    };
    let root_value_style = if root_active && is_editing {
        Style::default().fg(palette.text)
    } else {
        Style::default().fg(palette.muted)
    };
    let line_root = vec![
        Span::styled("   Root:  ", root_label_style),
        Span::styled(root_value, root_value_style),
    ];

    // Field 2: Custom Name (optional)
    let name_active = state.clone_field == 2 && !is_cloning;
    let mut line_name = vec![Span::styled("   Name:  ", Style::default().fg(palette.info))];
    line_name.extend(render_field_spans(
        &state.clone_name,
        state.clone_name_cursor,
        name_active,
        is_editing && !is_cloning,
        palette,
    ));
    if state.clone_name.is_empty() && !(name_active && is_editing) {
        line_name.push(Span::styled("(inferred from URL)", Style::default().fg(palette.muted)));
    }

    let mut lines =
        vec![Line::from(""), Line::from(line_url), Line::from(line_root), Line::from(line_name)];

    if is_cloning {
        // While cloning, show a "cloning…" notice instead of the action buttons.
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("          "),
            Span::styled(
                "⏳ Cloning in progress…",
                Style::default().fg(palette.warning).add_modifier(Modifier::BOLD),
            ),
            Span::raw("    "),
            Span::styled("Esc", Style::default().fg(palette.danger)),
            Span::raw(" Cancel"),
        ]));
    } else {
        // Idle: show normal form buttons.
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("          "),
            Span::styled("Tab", Style::default().fg(palette.primary)),
            Span::raw(" Next Field    "),
            Span::styled("Enter", Style::default().fg(palette.success)),
            Span::raw(" Submit    "),
            Span::styled("Esc", Style::default().fg(palette.danger)),
            Span::raw(" Cancel"),
        ]));

        if let Some(ref msg) = state.clone_status_message {
            let color = if msg.starts_with('✓') {
                palette.success
            } else if msg.starts_with('⏳') {
                palette.warning
            } else {
                palette.danger
            };
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(format!("   {msg}"), Style::default().fg(color))));
        }
    }

    let border_color = if is_cloning {
        palette.warning
    } else if is_editing {
        palette.primary
    } else {
        palette.border
    };

    let title = if is_cloning { " Clone Repository — Cloning… " } else { " Clone Repository " };

    let form = Paragraph::new(lines).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    );

    frame.render_widget(form, area);
}

/// Render the streaming git log output panel.
fn render_clone_log(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    // Show the most recent lines that fit in the area.
    let inner_height = area.height.saturating_sub(2) as usize; // minus border rows
    let total = state.clone_log_lines.len();
    let start = total.saturating_sub(inner_height);

    let log_lines: Vec<Line> = state
        .clone_log_lines
        .iter()
        .skip(start)
        .map(|l| {
            let color = if l.starts_with('✓') || l.starts_with("✓") {
                palette.success
            } else if l.starts_with('✗') {
                palette.danger
            } else {
                palette.muted
            };
            Line::from(Span::styled(format!("  {l}"), Style::default().fg(color)))
        })
        .collect();

    let is_cloning = state.clone_task.is_some();
    let log_widget = Paragraph::new(log_lines).block(
        Block::default()
            .title(if is_cloning { " Git Output — Live " } else { " Git Output " })
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if is_cloning {
                palette.warning
            } else {
                palette.success
            })),
    );

    frame.render_widget(log_widget, area);
}
