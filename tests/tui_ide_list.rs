use anyhow::Result;
use dev_cli::{
    models::{custom_ide::CustomIde, ide::Ide, ide::IdeSelection},
    tui::{
        state::{AppState, IdeTabFocus, InputMode},
        theme::get_palette,
        widgets::ide_list,
    },
};
use ratatui::{Terminal, backend::TestBackend};
use std::path::PathBuf;

#[test]
fn test_render_ide_list_empty() -> Result<()> {
    let mut state = AppState::new();
    state.installed_ides = vec![];
    state.custom_ides = vec![];

    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| ide_list::render(f, f.area(), &state, get_palette("neon")))?;

    Ok(())
}

#[test]
fn test_render_ide_list_with_installed_and_custom() -> Result<()> {
    let mut state = AppState::new();
    state.installed_ides = vec![Ide::Vscode, Ide::Cursor];
    state.default_ide = IdeSelection::BuiltIn(Ide::Vscode);
    state.custom_ides = vec![
        CustomIde {
            id: "my-custom".into(),
            display_name: "My Custom IDE".into(),
            executable: PathBuf::from("/usr/bin/custom"),
            args_template: None,
            verified: true,
        },
        CustomIde {
            id: "unverified-ide".into(),
            display_name: "Unverified IDE".into(),
            executable: PathBuf::from("/usr/bin/unverified"),
            args_template: Some("--new-window".into()),
            verified: false,
        },
    ];

    // Select top (installed)
    state.selected_index = 0;
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| ide_list::render(f, f.area(), &state, get_palette("neon")))?;

    // Select bottom (custom)
    state.selected_index = 2;
    state.ide_status_message = Some("✓ Action successful".into());
    terminal.draw(|f| ide_list::render(f, f.area(), &state, get_palette("cyberpunk")))?;

    // Warning status message
    state.ide_status_message = Some("Are you sure you want to delete?".into());
    terminal.draw(|f| ide_list::render(f, f.area(), &state, get_palette("catppuccin")))?;

    // Error status message
    state.ide_status_message = Some("✗ Failed to delete".into());
    terminal.draw(|f| ide_list::render(f, f.area(), &state, get_palette("monokai")))?;

    Ok(())
}

#[test]
fn test_render_ide_list_empty_custom_list() -> Result<()> {
    let mut state = AppState::new();
    state.installed_ides = vec![Ide::Vscode];
    state.custom_ides = vec![];

    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| ide_list::render(f, f.area(), &state, get_palette("neon")))?;

    Ok(())
}

#[test]
fn test_render_ide_add_form_split_view() -> Result<()> {
    let mut state = AppState::new();
    state.ide_tab_focus = IdeTabFocus::AddForm;
    state.installed_ides = vec![Ide::Vscode];
    state.default_ide = IdeSelection::Custom("my-custom".into());
    state.custom_ides = vec![CustomIde {
        id: "my-custom".into(),
        display_name: "My Custom IDE".into(),
        executable: PathBuf::from("/usr/bin/custom"),
        args_template: None,
        verified: true,
    }];

    state.input_mode = InputMode::Editing;
    state.ide_form_name = "New Custom".into();
    state.ide_form_name_cursor = 3;
    state.ide_form_path = "/usr/bin/new".into();
    state.ide_form_path_cursor = 5;
    state.ide_form_args = "--flag".into();
    state.ide_form_args_cursor = 2;
    state.ide_status_message = Some("✗ Executable path does not exist".into());

    for field in 0..3 {
        state.ide_form_field = field;
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend)?;
        terminal.draw(|f| ide_list::render(f, f.area(), &state, get_palette("high-contrast")))?;
    }

    Ok(())
}
