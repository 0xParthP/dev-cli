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

fn make_palette(
    name: &'static str,
    colors: (Color, Color, Color, Color, Color, Color, Color, Color, Color, Color, Color),
) -> Palette {
    let (
        primary,
        background,
        text,
        muted,
        border,
        success,
        warning,
        danger,
        info,
        purple,
        highlight_bg,
    ) = colors;
    Palette {
        name,
        primary,
        background,
        text,
        muted,
        border,
        success,
        warning,
        danger,
        info,
        purple,
        highlight_bg,
    }
}

pub fn get_palette(theme_name: &str) -> Palette {
    match theme_name.to_lowercase().as_str() {
        "cyberpunk" => make_palette(
            "Cyberpunk",
            (
                Color::Rgb(255, 0, 127),
                Color::Rgb(10, 5, 20),
                Color::Rgb(240, 240, 255),
                Color::Rgb(140, 100, 180),
                Color::Rgb(200, 0, 255),
                Color::Rgb(0, 255, 180),
                Color::Rgb(255, 200, 0),
                Color::Rgb(255, 50, 80),
                Color::Rgb(0, 220, 255),
                Color::Rgb(210, 80, 255),
                Color::Rgb(120, 0, 150),
            ),
        ),
        "catppuccin" => make_palette(
            "Catppuccin",
            (
                Color::Rgb(137, 180, 250),
                Color::Rgb(30, 30, 46),
                Color::Rgb(205, 214, 244),
                Color::Rgb(147, 153, 178),
                Color::Rgb(88, 91, 112),
                Color::Rgb(166, 227, 161),
                Color::Rgb(249, 226, 175),
                Color::Rgb(243, 139, 168),
                Color::Rgb(148, 226, 213),
                Color::Rgb(203, 166, 247),
                Color::Rgb(69, 71, 90),
            ),
        ),
        "monokai" => make_palette(
            "Monokai",
            (
                Color::Rgb(166, 226, 46),
                Color::Rgb(39, 40, 34),
                Color::Rgb(248, 248, 242),
                Color::Rgb(117, 113, 94),
                Color::Rgb(73, 72, 62),
                Color::Rgb(166, 226, 46),
                Color::Rgb(230, 219, 116),
                Color::Rgb(249, 38, 114),
                Color::Rgb(102, 217, 239),
                Color::Rgb(174, 129, 255),
                Color::Rgb(73, 72, 62),
            ),
        ),
        "high-contrast" => make_palette(
            "High Contrast",
            (
                Color::Rgb(255, 255, 0),
                Color::Rgb(0, 0, 0),
                Color::Rgb(255, 255, 255),
                Color::Rgb(180, 180, 180),
                Color::Rgb(255, 255, 255),
                Color::Rgb(0, 255, 0),
                Color::Rgb(255, 255, 0),
                Color::Rgb(255, 0, 0),
                Color::Rgb(0, 255, 255),
                Color::Rgb(255, 0, 255),
                Color::Rgb(50, 50, 50),
            ),
        ),
        _ => make_palette(
            "Neon Purple",
            (
                PRIMARY,
                BACKGROUND,
                TEXT,
                MUTED,
                BORDER,
                SUCCESS,
                WARNING,
                DANGER,
                INFO,
                PURPLE,
                HIGHLIGHT_BG,
            ),
        ),
    }
}
