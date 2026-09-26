use anyhow::Result;
use ratatui::{Terminal, backend::TestBackend};

use dev_cli::tui::{
    state::{AppState, IdeTabFocus, InputMode, SettingsTabFocus, Tab},
    theme::get_palette,
    widgets::footer,
};

#[test]
fn test_footer_render_all_states() -> Result<()> {
    let palette = get_palette("neon");

    let tabs = [Tab::Projects, Tab::Recent, Tab::Ide, Tab::Settings];

    for tab in tabs {
        let mut state = AppState::new();
        state.active_tab = tab;

        // Normal mode
        if tab == Tab::Ide {
            for focus in [IdeTabFocus::List, IdeTabFocus::ConfirmDelete, IdeTabFocus::AddForm] {
                state.ide_tab_focus = focus;
                let backend = TestBackend::new(120, 2);
                let mut terminal = Terminal::new(backend)?;
                terminal.draw(|f| footer::render(f, f.area(), &state, palette))?;
            }
        } else if tab == Tab::Settings {
            for focus in [SettingsTabFocus::List, SettingsTabFocus::AddRoot] {
                state.settings_tab_focus = focus;
                let backend = TestBackend::new(120, 2);
                let mut terminal = Terminal::new(backend)?;
                terminal.draw(|f| footer::render(f, f.area(), &state, palette))?;
            }
        } else {
            let backend = TestBackend::new(120, 2);
            let mut terminal = Terminal::new(backend)?;
            terminal.draw(|f| footer::render(f, f.area(), &state, palette))?;
        }

        // Editing mode
        state.input_mode = InputMode::Editing;
        let backend = TestBackend::new(120, 2);
        let mut terminal = Terminal::new(backend)?;
        terminal.draw(|f| footer::render(f, f.area(), &state, palette))?;
    }

    Ok(())
}
