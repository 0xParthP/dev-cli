use anyhow::Result;
use dev_cli::tui::{
    state::{AppState, InputMode, SettingsTabFocus},
    widgets::settings,
};
use ratatui::{Terminal, backend::TestBackend};
use std::path::PathBuf;

#[test]
fn test_render_settings_list() -> Result<()> {
    let mut state = AppState::new();
    state.settings_tab_focus = SettingsTabFocus::List;
    state.project_roots = vec![PathBuf::from("/projects/rust"), PathBuf::from("/projects/go")];
    state.selected_index = 0;
    state.ignore_patterns = vec!["node_modules".into(), "target".into()];
    state.auto_refresh_on_launch = true;
    state.settings_status_message = Some("✓ Settings updated".into());

    let backend = TestBackend::new(120, 35);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| settings::render(f, f.area(), &state))?;

    // Error status message, empty ignore patterns, disabled auto refresh
    state.settings_status_message = Some("✗ Error saving setting".into());
    state.ignore_patterns = vec![];
    state.auto_refresh_on_launch = false;
    terminal.draw(|f| settings::render(f, f.area(), &state))?;

    // Empty project roots
    state.project_roots = vec![];
    state.settings_status_message = None;
    terminal.draw(|f| settings::render(f, f.area(), &state))?;

    Ok(())
}

#[test]
fn test_render_settings_add_root_split_view() -> Result<()> {
    let mut state = AppState::new();
    state.settings_tab_focus = SettingsTabFocus::AddRoot;
    state.project_roots = vec![PathBuf::from("/projects/demo")];
    state.input_mode = InputMode::Editing;
    state.settings_add_root = "/projects/new".into();
    state.settings_add_root_cursor = 5;
    state.settings_status_message = Some("✗ Path does not exist".into());

    let backend = TestBackend::new(120, 35);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| settings::render(f, f.area(), &state))?;

    // Non-editing mode
    state.input_mode = InputMode::Normal;
    state.settings_status_message = None;
    terminal.draw(|f| settings::render(f, f.area(), &state))?;

    Ok(())
}
