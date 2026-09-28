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

fn from_rgb_list(name: &'static str, colors: &[(u8, u8, u8); 11]) -> Palette {
    let c = |i: usize| Color::Rgb(colors[i].0, colors[i].1, colors[i].2);
    Palette {
        name,
        primary: c(0),
        background: c(1),
        text: c(2),
        muted: c(3),
        border: c(4),
        success: c(5),
        warning: c(6),
        danger: c(7),
        info: c(8),
        purple: c(9),
        highlight_bg: c(10),
    }
}

pub fn get_palette(theme_name: &str) -> Palette {
    match theme_name.to_lowercase().as_str() {
        "cyberpunk" => from_rgb_list(
            "Cyberpunk",
            &[
                (255, 0, 127),
                (10, 5, 20),
                (240, 240, 255),
                (140, 100, 180),
                (200, 0, 255),
                (0, 255, 180),
                (255, 200, 0),
                (255, 50, 80),
                (0, 220, 255),
                (210, 80, 255),
                (120, 0, 150),
            ],
        ),
        "catppuccin" => from_rgb_list(
            "Catppuccin",
            &[
                (137, 180, 250),
                (30, 30, 46),
                (205, 214, 244),
                (147, 153, 178),
                (88, 91, 112),
                (166, 227, 161),
                (249, 226, 175),
                (243, 139, 168),
                (148, 226, 213),
                (203, 166, 247),
                (69, 71, 90),
            ],
        ),
        "monokai" => from_rgb_list(
            "Monokai",
            &[
                (166, 226, 46),
                (39, 40, 34),
                (248, 248, 242),
                (117, 113, 94),
                (73, 72, 62),
                (166, 226, 46),
                (230, 219, 116),
                (249, 38, 114),
                (102, 217, 239),
                (174, 129, 255),
                (73, 72, 62),
            ],
        ),
        "high-contrast" => from_rgb_list(
            "High Contrast",
            &[
                (255, 255, 0),
                (0, 0, 0),
                (255, 255, 255),
                (180, 180, 180),
                (255, 255, 255),
                (0, 255, 0),
                (255, 255, 0),
                (255, 0, 0),
                (0, 255, 255),
                (255, 0, 255),
                (50, 50, 50),
            ],
        ),
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
