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

    let list_items: Vec<ListItem> = items
        .iter()
        .map(|node| {
            let indent = "  ".repeat(node.depth);

            if node.is_folder {
                let icon = if node.is_expanded { "[-] 📂 " } else { "[+] 📁 " };
                ListItem::new(vec![
                    Line::from(vec![
                        Span::raw(indent.clone()),
                        Span::styled(icon, Style::default().fg(theme::WARNING)),
                        Span::styled(
                            node.name,
                            Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw(indent),
                        Span::raw("       "),
                        Span::styled(display_path(node.path), Style::default().fg(theme::MUTED)),
                    ]),
                    Line::default(),
                ])
            } else {
                ListItem::new(vec![
                    Line::from(vec![
                        Span::raw(indent.clone()),
                        Span::styled("📦 ", Style::default().fg(theme::INFO)),
                        Span::styled(
                            node.name,
                            Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw(indent),
                        Span::raw("   "),
                        Span::styled(display_path(node.path), Style::default().fg(theme::MUTED)),
                    ]),
                    Line::default(),
                ])
            }
        })
        .collect();

    render_list(frame, area, title, list_items, Some(state.selected_index));
}
