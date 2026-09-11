//! Placeholder widget for unimplemented tabs.

use ratatui::{Frame, layout::Rect};

use crate::tui::{
    state::Tab,
    theme,
    widgets::list::{Notice, render_centered_notice},
};

/// Render a coming soon placeholder for unimplemented tabs.
pub fn render(frame: &mut Frame, area: Rect, tab: Tab) {
    let title = match tab {
        Tab::Ide => " IDE ",
        Tab::Settings => " Settings ",
        _ => " Coming Soon ",
    };

    render_centered_notice(
        frame,
        area,
        Notice {
            title,
            icon: "🚧",
            icon_color: theme::WARNING,
            heading: "Coming Soon",
            subtext: "This tab will be implemented in a later milestone.",
        },
    );
}
