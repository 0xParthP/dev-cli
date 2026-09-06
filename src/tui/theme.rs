//! Shared colour palette for the dashboard.

use ratatui::style::Color;

/// Main application background.
pub const BACKGROUND: Color = Color::Rgb(12, 18, 28);

/// Primary accent colour.
pub const PRIMARY: Color = Color::Rgb(0, 190, 255);

/// Success / positive actions.
pub const SUCCESS: Color = Color::Rgb(72, 199, 116);

/// Accent icons / search shortcut.
pub const WARNING: Color = Color::Rgb(255, 190, 60);

/// Quit / destructive actions.
pub const DANGER: Color = Color::Rgb(255, 95, 95);

/// Main text colour.
pub const TEXT: Color = Color::Rgb(235, 240, 250);

/// Secondary text.
pub const MUTED: Color = Color::Rgb(120, 130, 150);

/// Widget borders.
pub const BORDER: Color = Color::Rgb(45, 70, 95);

/// Selected row background.
pub const HIGHLIGHT_BG: Color = Color::Rgb(0, 120, 180);
