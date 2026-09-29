//! Clone repository modal widget.

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::{
    tui::{state::AppState, theme::Palette},
    utils::path::display_path,
};

/// Renders the clone modal dialog if active.
pub fn render(frame: &mut Frame, state: &AppState, palette: Palette) {
    if !state.show_clone_modal {
        return;
    }

    let area = centered_rect(65, 14, frame.area());

    // Clear background for modal
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(Span::styled(
            " 📥 Clone Repository ",
            Style::default().fg(palette.primary).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(palette.border));

    frame.render_widget(block, area);

    // Inner layout
    let inner_area = area.inner(ratatui::layout::Margin {
        vertical: 1,
        horizontal: 2,
    });

    let chunks = Layout::vertical([
        Constraint::Length(3), // URL input
        Constraint::Length(3), // Target Root Selector
        Constraint::Length(3), // Custom Name input
        Constraint::Min(1),    // Status / Hints
    ])
    .split(inner_area);

    // Field 0: Repository URL
    let url_style = if state.clone_field == 0 {
        Style::default().fg(palette.primary)
    } else {
        Style::default().fg(palette.muted)
    };
    let url_block = Block::default()
        .title(" Git Repository URL ")
        .borders(Borders::ALL)
        .border_style(url_style);

    let mut url_text = state.clone_url.clone();
    if state.clone_field == 0 {
        url_text.insert(state.clone_url_cursor.min(url_text.len()), '│');
    }
    frame.render_widget(
        Paragraph::new(url_text).block(url_block).style(Style::default().fg(palette.text)),
        chunks[0],
    );

    // Field 1: Target Root Selector
    let root_style = if state.clone_field == 1 {
        Style::default().fg(palette.primary)
    } else {
        Style::default().fg(palette.muted)
    };
    let root_block = Block::default()
        .title(" Target Root (← / → to select) ")
        .borders(Borders::ALL)
        .border_style(root_style);

    let root_display = if state.project_roots.is_empty() {
        "No project roots configured".to_string()
    } else {
        let current_root =
            &state.project_roots[state.clone_root_index.min(state.project_roots.len() - 1)];
        format!(
            "[{}/{}]  📁 {}",
            state.clone_root_index + 1,
            state.project_roots.len(),
            display_path(current_root)
        )
    };

    frame.render_widget(
        Paragraph::new(root_display).block(root_block).style(Style::default().fg(palette.text)),
        chunks[1],
    );

    // Field 2: Custom Name (Optional)
    let name_style = if state.clone_field == 2 {
        Style::default().fg(palette.primary)
    } else {
        Style::default().fg(palette.muted)
    };
    let name_block = Block::default()
        .title(" Custom Folder Name (Optional) ")
        .borders(Borders::ALL)
        .border_style(name_style);

    let mut name_text = state.clone_name.clone();
    if state.clone_field == 2 {
        name_text.insert(state.clone_name_cursor.min(name_text.len()), '│');
    }
    frame.render_widget(
        Paragraph::new(name_text).block(name_block).style(Style::default().fg(palette.text)),
        chunks[2],
    );

    // Bottom Status / Controls
    let footer_text = if let Some(ref status) = state.clone_status_message {
        let color = if status.starts_with('✓') {
            palette.success
        } else {
            palette.danger
        };
        Line::from(Span::styled(status.clone(), Style::default().fg(color)))
    } else {
        Line::from(vec![
            Span::styled("Tab", Style::default().fg(palette.primary)),
            Span::raw(" Next Field   "),
            Span::styled("↵ Submit", Style::default().fg(palette.success)),
            Span::raw("   "),
            Span::styled("Esc Cancel", Style::default().fg(palette.danger)),
        ])
    };

    frame.render_widget(Paragraph::new(footer_text).alignment(Alignment::Center), chunks[3]);
}

/// Helper function to center a Rect popup modal.
fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(height),
        Constraint::Fill(1),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(width),
        Constraint::Fill(1),
    ])
    .split(popup_layout[1])[1]
}
