//! IDE management tab widget.

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, ListItem, Paragraph},
};

use crate::models::custom_ide::CustomIde;
use crate::tui::{
    state::{AppState, IdeTabFocus, InputMode},
    theme::Palette,
    widgets::list::{Notice, render_centered_notice, render_list_with_symbol},
};

/// Render the IDE management tab.
pub fn render(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    match state.ide_tab_focus {
        IdeTabFocus::List | IdeTabFocus::ConfirmDelete => {
            render_ide_list(frame, area, state, palette)
        }
        IdeTabFocus::AddForm => render_split_view(frame, area, state, palette),
    }
}

/// Render the IDE list split into Installed IDEs (top) and Custom IDEs (bottom).
fn render_ide_list(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    let installed_count = state.installed_ides.len();
    let custom_count = state.custom_ides.len();
    let total_count = installed_count + custom_count;

    if total_count == 0 {
        render_centered_notice(
            frame,
            area,
            Notice {
                title: " Installed & Custom IDEs ",
                icon: "🔧",
                icon_color: palette.muted,
                heading: "No IDEs detected",
                subtext: "Press 'a' to add a custom IDE.",
            },
            palette,
        );
        return;
    }

    let (list_area, status_area) = if let Some(ref msg) = state.ide_status_message {
        let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(area);
        (chunks[0], Some((chunks[1], msg)))
    } else {
        (area, None)
    };

    let split_chunks =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(list_area);

    let installed_items: Vec<ListItem> = state
        .installed_ides
        .iter()
        .enumerate()
        .map(|(idx, &ide)| {
            let is_default = state.default_ide == crate::models::ide::IdeSelection::BuiltIn(ide);
            let is_selected = state.selected_index == idx;
            render_installed_entry(
                ide,
                is_default,
                is_selected,
                split_chunks[0].width.saturating_sub(4) as usize,
                palette,
            )
        })
        .collect();

    let selected_top =
        if state.selected_index < installed_count { Some(state.selected_index) } else { None };

    render_list_with_symbol(
        frame,
        split_chunks[0],
        format!(" Installed IDEs ({}) ", installed_count),
        installed_items,
        selected_top,
        "",
        palette,
    );

    let custom_items = build_custom_ide_items(
        state,
        installed_count,
        split_chunks[1].width.saturating_sub(4) as usize,
        true,
        palette,
    );

    let selected_bottom = if state.selected_index >= installed_count && custom_count > 0 {
        Some(state.selected_index - installed_count)
    } else {
        None
    };

    render_list_with_symbol(
        frame,
        split_chunks[1],
        format!(" Custom IDEs ({}) ", custom_count),
        custom_items,
        selected_bottom,
        "",
        palette,
    );

    if let Some((area, msg)) = status_area {
        let color = if msg.starts_with('✓') {
            palette.success
        } else if msg.starts_with("Are you sure") {
            palette.warning
        } else {
            palette.danger
        };
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(format!(" {msg}"), Style::default().fg(color)))),
            area,
        );
    }
}

/// Render the split view: IDE list split on top, form on bottom.
fn render_split_view(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    let chunks =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);

    let top_chunks =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(chunks[0]);

    let installed_items = state
        .installed_ides
        .iter()
        .map(|&ide| {
            let is_default = state.default_ide == crate::models::ide::IdeSelection::BuiltIn(ide);
            render_installed_entry(
                ide,
                is_default,
                false,
                top_chunks[0].width.saturating_sub(4) as usize,
                palette,
            )
        })
        .collect();
    render_list_with_symbol(
        frame,
        top_chunks[0],
        format!(" Installed IDEs ({}) ", state.installed_ides.len()),
        installed_items,
        None,
        "",
        palette,
    );

    let custom_items = build_custom_ide_items(
        state,
        0,
        top_chunks[1].width.saturating_sub(4) as usize,
        false,
        palette,
    );
    render_list_with_symbol(
        frame,
        top_chunks[1],
        format!(" Custom IDEs ({}) ", state.custom_ides.len()),
        custom_items,
        None,
        "",
        palette,
    );

    // Bottom half: Add form
    render_add_form(frame, chunks[1], state, palette);
}

/// Helper to build list items for custom IDEs.
fn build_custom_ide_items(
    state: &AppState,
    installed_offset: usize,
    width: usize,
    interactive: bool,
    palette: Palette,
) -> Vec<ListItem<'static>> {
    if state.custom_ides.is_empty() {
        vec![ListItem::new(vec![
            Line::from(Span::styled(
                "  No custom IDEs configured. Press 'a' to add.",
                Style::default().fg(palette.muted),
            )),
            Line::default(),
        ])]
    } else {
        state
            .custom_ides
            .iter()
            .enumerate()
            .map(|(idx, custom)| {
                let is_default = state.default_ide
                    == crate::models::ide::IdeSelection::Custom(custom.id.clone());
                let is_selected = interactive && (state.selected_index == installed_offset + idx);
                render_custom_entry(custom, is_default, is_selected, width, palette)
            })
            .collect()
    }
}

#[allow(clippy::too_many_arguments)]
#[rustfmt::skip]
fn render_entry_row(
    left_name: &str, status_text: &str, status_color: ratatui::style::Color, extra_tag: Option<&str>,
    is_default: bool, is_selected: bool, available_width: usize, palette: Palette,
) -> Vec<Span<'static>> {
    let prefix = if is_selected { "❯ " } else { "  " };
    let tag_len = extra_tag.map_or(0, |t| unicode_width::UnicodeWidthStr::width(t) + 2);
    let right_len = unicode_width::UnicodeWidthStr::width(status_text) + tag_len + if is_default { 11 } else { 0 };
    let left_len = unicode_width::UnicodeWidthStr::width(left_name);
    let spacing = available_width.saturating_sub(2 + left_len + right_len);

    let p_style = if is_selected { Style::default().fg(palette.primary).add_modifier(Modifier::BOLD) } else { Style::default().fg(palette.muted) };
    let l_style = Style::default().fg(palette.text).add_modifier(Modifier::BOLD);

    let mut spans = vec![
        Span::styled(prefix.to_string(), p_style),
        Span::styled(left_name.to_string(), l_style),
        Span::raw(" ".repeat(spacing)),
    ];

    if is_default {
        spans.push(Span::styled("★ DEFAULT  ".to_string(), Style::default().fg(palette.primary).add_modifier(Modifier::BOLD)));
    }
    spans.push(Span::styled(status_text.to_string(), Style::default().fg(status_color)));
    if let Some(tag) = extra_tag {
        spans.push(Span::styled(format!("  {}", tag), Style::default().fg(palette.muted)));
    }
    spans
}

#[rustfmt::skip]
fn render_installed_entry(
    ide: crate::models::ide::Ide, is_default: bool, is_selected: bool, available_width: usize, palette: Palette,
) -> ListItem<'static> {
    let spans = render_entry_row(ide.display_name(), "✓ Detected", palette.success, None, is_default, is_selected, available_width, palette);
    ListItem::new(vec![Line::from(spans), Line::default()])
}

/// Render a custom IDE entry.
#[rustfmt::skip]
fn render_custom_entry(
    custom: &CustomIde, is_default: bool, is_selected: bool, available_width: usize, palette: Palette,
) -> ListItem<'static> {
    let (status_text, status_color) = if custom.verified { ("✓ Verified", palette.success) } else { ("✗ Unverified", palette.warning) };
    let spans = render_entry_row(&custom.display_name, status_text, status_color, Some("(custom)"), is_default, is_selected, available_width, palette);
    ListItem::new(vec![
        Line::from(spans),
        Line::from(vec![Span::raw("    "), Span::styled(custom.executable.display().to_string(), Style::default().fg(palette.muted))]),
        Line::default(),
    ])
}

/// Render the "Add Custom IDE" form.
fn render_add_form(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    let is_editing = state.input_mode == InputMode::Editing;

    let name_active = state.ide_form_field == 0;
    let path_active = state.ide_form_field == 1;
    let args_active = state.ide_form_field == 2;

    let mut line_name = vec![Span::styled("   Name:  ", Style::default().fg(palette.info))];
    line_name.extend(crate::tui::widgets::list::render_field_spans(
        &state.ide_form_name,
        state.ide_form_name_cursor,
        name_active,
        is_editing,
        palette,
    ));

    let mut line_path = vec![Span::styled("   Path:  ", Style::default().fg(palette.info))];
    line_path.extend(crate::tui::widgets::list::render_field_spans(
        &state.ide_form_path,
        state.ide_form_path_cursor,
        path_active,
        is_editing,
        palette,
    ));

    let mut line_args = vec![Span::styled("   Args:  ", Style::default().fg(palette.info))];
    line_args.extend(crate::tui::widgets::list::render_field_spans(
        &state.ide_form_args,
        state.ide_form_args_cursor,
        args_active,
        is_editing,
        palette,
    ));

    let mut lines =
        vec![Line::from(""), Line::from(line_name), Line::from(line_path), Line::from(line_args)];
    crate::tui::widgets::list::append_form_footer(
        &mut lines,
        state.ide_status_message.as_deref(),
        palette,
    );

    let form = Paragraph::new(lines).block(
        Block::default().title(" Add Custom IDE ").borders(Borders::ALL).border_style(
            Style::default().fg(if is_editing { palette.primary } else { palette.border }),
        ),
    );

    frame.render_widget(form, area);
}
