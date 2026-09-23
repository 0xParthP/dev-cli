//! Project list widget.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::{
    tui::{
        state::AppState,
        theme,
        widgets::list::{Notice, render_centered_notice, render_list},
    },
    utils::path::display_path,
};
use unicode_width::UnicodeWidthStr;

/// Render the widget onto the given frame and area.
pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    // Route to the correct tab.
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
                icon_color: theme::MUTED,
                heading: "No projects found",
                subtext: "Try another search.",
            },
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
                (icon_str, theme::WARNING, "       ")
            } else {
                ("📦 ", theme::INFO, "   ")
            };

            let path_str = display_path(node.path);

            let mut line2_spans = vec![
                Span::raw(indent.clone()),
                Span::raw(path_padding),
                Span::styled(path_str.clone(), Style::default().fg(theme::MUTED)),
            ];

            if !node.is_folder {
                let ide = state.get_project_ide(node.path);
                let badge = format!("[{}]", ide.display_name());
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
                    Style::default().fg(ide.color()).add_modifier(Modifier::BOLD),
                ));
            }

            ListItem::new(vec![
                Line::from(vec![
                    Span::raw(indent),
                    Span::styled(icon, Style::default().fg(icon_color)),
                    Span::styled(
                        node.name,
                        Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(line2_spans),
                Line::default(),
            ])
        })
        .collect();

    render_list(frame, area, title, list_items, Some(state.selected_index));
}
