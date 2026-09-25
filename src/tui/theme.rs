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

/// Info text.
pub const INFO: Color = Color::Rgb(125, 180, 255);

/// Secondary accent / purple actions.
pub const PURPLE: Color = Color::Rgb(180, 120, 255);

/// Palette configuration containing all UI colors.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub name: &'static str,
    pub primary: Color,
    pub background: Color,
    pub text: Color,
    pub muted: Color,
    pub border: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub info: Color,
    pub purple: Color,
    pub highlight_bg: Color,
}

pub fn get_palette(theme_name: &str) -> Palette {
    match theme_name.to_lowercase().as_str() {
        "cyberpunk" => Palette {
            name: "Cyberpunk",
            primary: Color::Rgb(255, 0, 127),
            background: Color::Rgb(10, 5, 20),
            text: Color::Rgb(240, 240, 255),
            muted: Color::Rgb(140, 100, 180),
            border: Color::Rgb(200, 0, 255),
            success: Color::Rgb(0, 255, 180),
            warning: Color::Rgb(255, 200, 0),
            danger: Color::Rgb(255, 50, 80),
            info: Color::Rgb(0, 220, 255),
            purple: Color::Rgb(210, 80, 255),
            highlight_bg: Color::Rgb(120, 0, 150),
        },
        "catppuccin" => Palette {
            name: "Catppuccin",
            primary: Color::Rgb(137, 180, 250),
            background: Color::Rgb(30, 30, 46),
            text: Color::Rgb(205, 214, 244),
            muted: Color::Rgb(147, 153, 178),
            border: Color::Rgb(88, 91, 112),
            success: Color::Rgb(166, 227, 161),
            warning: Color::Rgb(249, 226, 175),
            danger: Color::Rgb(243, 139, 168),
            info: Color::Rgb(148, 226, 213),
            purple: Color::Rgb(203, 166, 247),
            highlight_bg: Color::Rgb(69, 71, 90),
        },
        "monokai" => Palette {
            name: "Monokai",
            primary: Color::Rgb(166, 226, 46),
            background: Color::Rgb(39, 40, 34),
            text: Color::Rgb(248, 248, 242),
            muted: Color::Rgb(117, 113, 94),
            border: Color::Rgb(73, 72, 62),
            success: Color::Rgb(166, 226, 46),
            warning: Color::Rgb(230, 219, 116),
            danger: Color::Rgb(249, 38, 114),
            info: Color::Rgb(102, 217, 239),
            purple: Color::Rgb(174, 129, 255),
            highlight_bg: Color::Rgb(73, 72, 62),
        },
        "high-contrast" => Palette {
            name: "High Contrast",
            primary: Color::Rgb(255, 255, 0),
            background: Color::Rgb(0, 0, 0),
            text: Color::Rgb(255, 255, 255),
            muted: Color::Rgb(180, 180, 180),
            border: Color::Rgb(255, 255, 255),
            success: Color::Rgb(0, 255, 0),
            warning: Color::Rgb(255, 255, 0),
            danger: Color::Rgb(255, 0, 0),
            info: Color::Rgb(0, 255, 255),
            purple: Color::Rgb(255, 0, 255),
            highlight_bg: Color::Rgb(50, 50, 50),
        },
        _ => Palette {
            name: "Neon Purple",
            primary: PRIMARY,
            background: BACKGROUND,
            text: TEXT,
            muted: MUTED,
            border: BORDER,
            success: SUCCESS,
            warning: WARNING,
            danger: DANGER,
            info: INFO,
            purple: PURPLE,
            highlight_bg: HIGHLIGHT_BG,
        },
    }
}
