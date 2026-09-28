use dev_cli::tui::theme::get_palette;

#[test]
fn test_all_theme_palettes() {
    let themes = [
        "neon",
        "cyberpunk",
        "catppuccin",
        "monokai",
        "high-contrast",
        "unknown_theme",
        "CYBERPUNK",
    ];

    for theme in themes {
        let palette = get_palette(theme);
        assert!(!palette.name.is_empty());
    }
}
