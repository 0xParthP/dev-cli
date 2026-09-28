use crate::common::factories::fake_project as project;
use common::temp_config::with_temp_config;
use dev_cli::{models::ide::Ide, tui::state::AppState};
use serial_test::serial;

mod common;

#[test]
fn app_state_starts_empty() {
    let state = AppState::new();

    assert!(state.projects.is_empty());
    assert!(state.search_query.is_empty());
    assert_eq!(state.selected_index, 0);
    assert!(!state.should_quit);
}

#[test]
fn quit_sets_should_quit() {
    let mut state = AppState::new();

    state.quit();

    assert!(state.should_quit);
}

#[test]
fn visible_items_returns_all_when_query_empty() {
    let mut state = AppState::new();

    state.set_projects(vec![project("cursor"), project("weather"), project("notes")]);

    let items = state.visible_items();

    assert_eq!(items.len(), 4);
}

#[test]
fn visible_items_filters_case_insensitively() {
    let mut state = AppState::new();

    state.set_projects(vec![project("Cursor"), project("cursor-theme"), project("weather")]);

    state.search_query = "CURSOR".into();

    let items = state.visible_items();

    assert_eq!(items.len(), 3);
    assert_eq!(items[1].name, "Cursor");
    assert_eq!(items[2].name, "cursor-theme");
}

#[test]
fn move_down_stops_at_last_project() {
    let mut state = AppState::new();

    state.set_projects(vec![project("a"), project("b")]);

    state.move_down();
    state.move_down();
    state.move_down();

    assert_eq!(state.selected_index, 2);
}

#[test]
fn move_up_stops_at_zero() {
    let mut state = AppState::new();

    state.set_projects(vec![project("a"), project("b")]);
    state.selected_index = 1;

    state.move_up();
    state.move_up();

    assert_eq!(state.selected_index, 0);
}

#[test]
fn push_char_appends_to_search_and_resets_selection() {
    let mut state = AppState::new();

    state.selected_index = 5;

    state.push_char('c');
    state.push_char('u');

    assert_eq!(state.search_query, "cu");
    assert_eq!(state.selected_index, 0);
}

#[test]
fn pop_char_removes_last_character() {
    let mut state = AppState::new();

    state.search_query = "cursor".into();
    state.search_cursor = state.search_query.chars().count();

    state.pop_char();
    state.pop_char();

    assert_eq!(state.search_query, "curs");
}

#[test]
fn clamp_selection_resets_when_filtered_list_is_empty() {
    let mut state = AppState::new();

    state.set_projects(vec![project("cursor")]);
    state.search_query = "weather".into();
    state.selected_index = 5;

    state.clamp_selection();

    assert_eq!(state.selected_index, 0);
}

#[test]
fn clamp_selection_moves_selection_to_last_item() {
    let mut state = AppState::new();

    state.set_projects(vec![project("cursor"), project("weather"), project("notes")]);

    state.selected_index = 10;

    state.clamp_selection();

    assert_eq!(state.selected_index, 3);
}

#[test]
fn search_filters_projects() {
    let mut state = AppState::new();

    state.set_projects(vec![project("cursor"), project("weather-app"), project("cursor-theme")]);

    state.search_query = "cursor".into();

    let items = state.visible_items();

    assert_eq!(items.len(), 3);
}

#[test]
fn move_down_on_recent_tab_stops_at_end() {
    use dev_cli::models::recent_project::RecentProject;
    use dev_cli::tui::state::Tab;
    use std::path::PathBuf;

    let mut state = AppState::new();
    state.active_tab = Tab::Recent;

    state.recent_projects = vec![
        RecentProject { name: "a".into(), path: PathBuf::from("/a"), last_opened: 0, ide: None },
        RecentProject { name: "b".into(), path: PathBuf::from("/b"), last_opened: 0, ide: None },
    ];

    state.move_down();
    state.move_down();
    state.move_down();

    assert_eq!(state.selected_index, 1);
}

#[test]
fn typing_resets_selection() {
    let mut state = AppState::new();

    state.selected_index = 4;

    state.push_char('a');

    assert_eq!(state.selected_index, 0);
    assert_eq!(state.search_query, "a");
}

#[test]
fn refresh_reloads_projects_and_recents() {
    let mut state = AppState::new();
    state.selected_index = 100;
    assert!(state.refresh().is_ok());
    assert!(state.selected_index < state.visible_items().len().max(1));
}

#[test]
fn cycle_selected_ide_updates_override() {
    use dev_cli::models::ide::IdeSelection;
    let mut state = AppState::new();
    state.default_ide = IdeSelection::BuiltIn(Ide::Vscode);
    state.custom_ides = Vec::new();
    state.installed_ides = vec![Ide::Vscode, Ide::Cursor, Ide::Claude];
    let p = project("demo");
    state.set_projects(vec![p.clone()]);
    state.selected_index = 1; // Highlight demo project under root

    assert_eq!(state.get_project_ide(&p.path), IdeSelection::BuiltIn(Ide::Vscode));
    state.cycle_selected_ide();
    assert_eq!(state.get_project_ide(&p.path), IdeSelection::BuiltIn(Ide::Cursor));
    state.cycle_selected_ide();
    assert_eq!(state.get_project_ide(&p.path), IdeSelection::BuiltIn(Ide::Claude));
    state.cycle_selected_ide();
    assert_eq!(state.get_project_ide(&p.path), IdeSelection::BuiltIn(Ide::Vscode));
}

#[test]
fn cycle_selected_ide_skips_uninstalled_ides() {
    use dev_cli::models::ide::IdeSelection;
    let mut state = AppState::new();
    state.default_ide = IdeSelection::BuiltIn(Ide::Vscode);
    state.custom_ides = Vec::new();
    state.installed_ides = vec![Ide::Vscode, Ide::Terminal];
    let p = project("demo");
    state.set_projects(vec![p.clone()]);
    state.selected_index = 1;

    assert_eq!(state.get_project_ide(&p.path), IdeSelection::BuiltIn(Ide::Vscode));

    state.cycle_selected_ide();
    assert_eq!(state.get_project_ide(&p.path), IdeSelection::BuiltIn(Ide::Terminal));

    state.cycle_selected_ide();
    assert_eq!(state.get_project_ide(&p.path), IdeSelection::BuiltIn(Ide::Vscode));
}

#[test]
fn get_project_ide_falls_back_to_first_installed_if_default_uninstalled() {
    use dev_cli::models::ide::IdeSelection;
    let mut state = AppState::new();
    state.custom_ides = Vec::new();
    state.default_ide = IdeSelection::BuiltIn(Ide::Idea);
    state.installed_ides = vec![Ide::Cursor, Ide::Claude];

    let p = project("demo");
    assert_eq!(state.get_project_ide(&p.path), IdeSelection::BuiltIn(Ide::Cursor));
}

#[test]
fn cycle_selected_ide_on_recent_tab() {
    use dev_cli::models::ide::IdeSelection;
    use dev_cli::models::recent_project::RecentProject;
    use dev_cli::tui::state::Tab;
    use std::path::PathBuf;

    let mut state = AppState::new();
    state.custom_ides = Vec::new();
    state.active_tab = Tab::Recent;
    state.installed_ides = vec![Ide::Vscode, Ide::Claude];
    let path = PathBuf::from("/tmp/recent_demo");
    state.recent_projects = vec![RecentProject {
        name: "recent_demo".into(),
        path: path.clone(),
        last_opened: 100,
        ide: Some(IdeSelection::BuiltIn(Ide::Vscode)),
    }];

    assert_eq!(
        state.get_recent_project_ide(&state.recent_projects[0]),
        IdeSelection::BuiltIn(Ide::Vscode)
    );

    state.cycle_selected_ide();
    assert_eq!(
        state.get_recent_project_ide(&state.recent_projects[0]),
        IdeSelection::BuiltIn(Ide::Claude)
    );

    state.cycle_selected_ide();
    assert_eq!(
        state.get_recent_project_ide(&state.recent_projects[0]),
        IdeSelection::BuiltIn(Ide::Vscode)
    );
}

#[test]
fn projects_tab_uses_default_ide_not_recent_ide() {
    use dev_cli::models::ide::IdeSelection;
    use dev_cli::models::recent_project::RecentProject;
    use std::path::PathBuf;

    let mut state = AppState::new();
    state.default_ide = IdeSelection::BuiltIn(Ide::Vscode);
    state.installed_ides = vec![Ide::Vscode, Ide::Claude];

    let path = PathBuf::from("/tmp/shared_demo");
    let recent = RecentProject {
        name: "shared_demo".into(),
        path: path.clone(),
        last_opened: 100,
        ide: Some(IdeSelection::BuiltIn(Ide::Claude)),
    };
    state.recent_projects = vec![recent.clone()];

    // Recent tab uses recent.ide (Claude)
    assert_eq!(state.get_recent_project_ide(&recent), IdeSelection::BuiltIn(Ide::Claude));

    // Projects tab uses default_ide (VS Code), ignoring recent.ide
    assert_eq!(state.get_project_ide(&path), IdeSelection::BuiltIn(Ide::Vscode));
}

#[test]
#[serial]
fn test_app_state_cycling_and_toggles() {
    with_temp_config(|| {
        let mut state = AppState::new();

        // Theme cycling
        let initial_theme = state.theme.clone();
        state.cycle_theme();
        assert_ne!(state.theme, initial_theme);
        assert!(state.settings_status_message.is_some());

        // Max depth cycling
        let initial_depth = state.max_depth;
        state.cycle_max_depth();
        assert_ne!(state.max_depth, initial_depth);

        // Recent limit cycling
        let initial_limit = state.recent_projects_limit;
        state.cycle_recent_limit();
        assert_ne!(state.recent_projects_limit, initial_limit);

        // Auto-refresh toggle
        let initial_auto = state.auto_refresh_on_launch;
        state.toggle_auto_refresh();
        assert_eq!(state.auto_refresh_on_launch, !initial_auto);

        // Reset defaults
        state.reset_config_defaults();
        assert!(state.settings_status_message.is_some());
    });
}

#[test]
fn test_search_cursor_and_character_edits() {
    let mut state = AppState::new();

    state.push_char('a');
    state.push_char('b');
    state.push_char('c');
    assert_eq!(state.search_query, "abc");
    assert_eq!(state.search_cursor, 3);

    state.move_search_cursor_left();
    assert_eq!(state.search_cursor, 2);

    state.move_search_cursor_home();
    assert_eq!(state.search_cursor, 0);

    state.move_search_cursor_right();
    assert_eq!(state.search_cursor, 1);

    state.delete_char(); // deletes 'b' at cursor index 1
    assert_eq!(state.search_query, "ac");

    state.move_search_cursor_end();
    assert_eq!(state.search_cursor, 2);

    state.clear_search();
    assert_eq!(state.search_query, "");
    assert_eq!(state.search_cursor, 0);
}

#[test]
fn test_editing_mode_field_manipulation() {
    use dev_cli::tui::state::{IdeTabFocus, InputMode, Tab};

    let mut state = AppState::new();
    state.active_tab = Tab::Ide;
    state.ide_tab_focus = IdeTabFocus::AddForm;
    state.input_mode = InputMode::Editing;

    for field in 0..3 {
        state.ide_form_field = field;
        assert_eq!(state.active_ide_form_field().to_string(), "");
    }

    state.ide_form_field = 0;
    state.editing_insert_char('X');
    state.editing_insert_char('Y');
    state.editing_insert_char('Z');
    assert_eq!(state.ide_form_name, "XYZ");
    assert_eq!(state.ide_form_name_cursor, 3);

    state.editing_move_cursor_left();
    assert_eq!(state.ide_form_name_cursor, 2);

    state.editing_move_cursor_home();
    assert_eq!(state.ide_form_name_cursor, 0);

    state.editing_move_cursor_right();
    assert_eq!(state.ide_form_name_cursor, 1);

    state.editing_delete(); // deletes 'Y'
    assert_eq!(state.ide_form_name, "XZ");

    state.editing_backspace(); // deletes 'X'
    assert_eq!(state.ide_form_name, "Z");

    state.editing_move_cursor_end();
    assert_eq!(state.ide_form_name_cursor, 1);

    state.cycle_ide_form_field();
    assert_eq!(state.ide_form_field, 1);
}

#[test]
#[serial]
fn test_ide_tab_custom_ide_management() {
    with_temp_config(|| {
        use dev_cli::models::custom_ide::CustomIde;
        use dev_cli::tui::state::{IdeTabFocus, Tab};
        use std::path::PathBuf;

        let mut state = AppState::new();
        state.active_tab = Tab::Ide;
        state.custom_ides = vec![CustomIde {
            id: "custom-one".into(),
            display_name: "Custom One".into(),
            executable: PathBuf::from("/bin/custom-one"),
            args_template: None,
            verified: true,
        }];

        let items = state.ide_tab_items();
        assert!(!items.is_empty());

        // Select installed vs custom
        state.selected_index = 0;
        state.set_default_selected_ide();
        assert!(state.ide_status_message.is_some());

        let installed_len = state.installed_ides.len();
        state.selected_index = installed_len; // point to custom-one
        state.set_default_selected_ide();
        assert!(state.ide_status_message.is_some());

        // Request delete
        state.request_delete_selected_custom_ide();
        assert_eq!(state.ide_tab_focus, IdeTabFocus::ConfirmDelete);

        // Cancel delete
        state.cancel_delete_custom_ide();
        assert_eq!(state.ide_tab_focus, IdeTabFocus::List);

        // Remove custom IDE
        state.request_delete_selected_custom_ide();
        state.remove_selected_custom_ide();
        assert!(state.custom_ides.is_empty());

        // Submit IDE form validation errors
        state.ide_form_name = "".into();
        assert!(state.submit_ide_form().is_err());

        state.ide_form_name = "Custom Two".into();
        state.ide_form_path = "".into();
        assert!(state.submit_ide_form().is_err());

        state.ide_form_path = "/nonexistent/path/to/exe".into();
        assert!(state.submit_ide_form().is_err());
    });
}

#[test]
#[serial]
fn test_settings_tab_root_management() {
    with_temp_config(|| {
        use dev_cli::tui::state::{SettingsTabFocus, Tab};

        let mut state = AppState::new();
        state.active_tab = Tab::Settings;
        state.settings_tab_focus = SettingsTabFocus::AddRoot;

        // submit empty
        state.settings_add_root = "".into();
        assert!(state.submit_add_root().is_err());

        // submit invalid path
        state.settings_add_root = "/nonexistent/dir/12345".into();
        assert!(state.submit_add_root().is_err());

        // add valid root
        let temp_dir = std::env::temp_dir();
        state.settings_add_root = temp_dir.to_string_lossy().to_string();
        assert!(state.submit_add_root().is_ok());
        assert_eq!(state.settings_item_count(), state.project_roots.len());

        // remove root
        state.selected_index = state.project_roots.len() - 1;
        state.remove_selected_root();
    });
}

#[test]
fn test_tree_toggle_and_scroll_offset() {
    use dev_cli::tui::state::Tab;
    use dev_cli::tui::tree::TreeNode;

    let mut state = AppState::new();
    state.active_tab = Tab::Projects;
    state.set_projects(vec![project("proj1")]);

    state.selected_index = 0; // Root folder
    state.toggle_selected();
    if let TreeNode::Folder { is_expanded, .. } = &state.project_tree[0] {
        assert!(!is_expanded);
    }

    assert_eq!(state.scroll_offset(10), 0);
    state.selected_index = 15;
    assert_eq!(state.scroll_offset(10), 6);
}
