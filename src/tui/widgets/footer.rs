//! Footer widget.

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::{
    state::{AppState, IdeTabFocus, InputMode, SettingsTabFocus, Tab},
    theme::Palette,
};

/// Render the widget onto the given frame and area.
pub fn render(frame: &mut Frame, area: Rect, state: &AppState, palette: Palette) {
    let line = build_footer_line(state, palette);

    frame.render_widget(
        Paragraph::new(line)
            .alignment(Alignment::Center)
            .style(Style::default().add_modifier(Modifier::BOLD)),
        area,
    );
}

/// Build the footer line based on the active tab and input mode.
fn build_footer_line(state: &AppState, palette: Palette) -> Line<'static> {
    if state.is_clone_form_active() {
        return Line::from(vec![
            Span::styled("Tab", Style::default().fg(palette.primary)),
            Span::raw(" Next Field  "),
            Span::styled("↵ Submit", Style::default().fg(palette.success)),
            Span::raw("  "),
            Span::styled("Esc Cancel", Style::default().fg(palette.danger)),
        ]);
    }

    if state.input_mode == InputMode::Editing {
        return match state.active_tab {
            Tab::Ide => Line::from(vec![
                Span::styled("Tab", Style::default().fg(palette.primary)),
                Span::raw(" Next Field  "),
                Span::styled("↵ Submit", Style::default().fg(palette.success)),
                Span::raw("  "),
                Span::styled("Esc Cancel", Style::default().fg(palette.danger)),
            ]),
            Tab::Settings => Line::from(vec![
                Span::styled("↵ Submit", Style::default().fg(palette.success)),
                Span::raw("  "),
                Span::styled("Esc Cancel", Style::default().fg(palette.danger)),
            ]),
            _ => Line::from(""),
        };
    }

    match state.active_tab {
        Tab::Projects => Line::from(vec![
            Span::styled("↵ Enter", Style::default().fg(palette.success)),
            Span::raw("  "),
            Span::styled("↑↓ Navigate", Style::default().fg(palette.primary)),
            Span::raw("  "),
            Span::styled("Tab IDE", Style::default().fg(palette.info)),
            Span::raw("  "),
            Span::styled("/ Search", Style::default().fg(palette.warning)),
            Span::raw("  "),
            Span::styled("F1 Refresh", Style::default().fg(palette.purple)),
            Span::raw("  "),
            Span::styled("F2 Clone", Style::default().fg(palette.success)),
            Span::raw("  "),
            Span::styled("Esc Quit", Style::default().fg(palette.danger)),
        ]),
        Tab::Recent => Line::from(vec![
            Span::styled("↵ Enter", Style::default().fg(palette.success)),
            Span::raw("  "),
            Span::styled("↑↓ Navigate", Style::default().fg(palette.primary)),
            Span::raw("  "),
            Span::styled("Tab IDE", Style::default().fg(palette.info)),
            Span::raw("  "),
            Span::styled("Esc Quit", Style::default().fg(palette.danger)),
        ]),

        Tab::Ide => match state.ide_tab_focus {
            IdeTabFocus::List => Line::from(vec![
                Span::styled("↑↓ Navigate", Style::default().fg(palette.primary)),
                Span::raw("  "),
                Span::styled("a Add", Style::default().fg(palette.success)),
                Span::raw("  "),
                Span::styled("d Delete", Style::default().fg(palette.danger)),
                Span::raw("  "),
                Span::styled("s Default", Style::default().fg(palette.warning)),
                Span::raw("  "),
                Span::styled("Esc Quit", Style::default().fg(palette.danger)),
            ]),
            IdeTabFocus::ConfirmDelete => Line::from(vec![
                Span::styled("y Yes Delete", Style::default().fg(palette.danger)),
                Span::raw("  "),
                Span::styled("n / Esc Cancel", Style::default().fg(palette.primary)),
            ]),
            IdeTabFocus::AddForm => Line::from(""),
        },
        Tab::Settings => {
            if state.settings_tab_focus == SettingsTabFocus::List {
                Line::from(vec![
                    Span::styled("↑↓ Nav", Style::default().fg(palette.primary)),
                    Span::raw("  "),
                    Span::styled("a Add Root", Style::default().fg(palette.success)),
                    Span::raw("  "),
                    Span::styled("d Delete", Style::default().fg(palette.danger)),
                    Span::raw("  "),
                    Span::styled("t Theme", Style::default().fg(palette.purple)),
                    Span::raw("  "),
                    Span::styled("m Depth", Style::default().fg(palette.warning)),
                    Span::raw("  "),
                    Span::styled("l Limit", Style::default().fg(palette.info)),
                    Span::raw("  "),
                    Span::styled("r AutoRef", Style::default().fg(palette.success)),
                    Span::raw("  "),
                    Span::styled("X Reset", Style::default().fg(palette.warning)),
                    Span::raw("  "),
                    Span::styled("Esc Quit", Style::default().fg(palette.danger)),
                ])
            } else {
                Line::from("")
            }
        }
    }
}
