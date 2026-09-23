use crate::common::factories::fake_project as project;
use dev_cli::{models::ide::Ide, tui::state::AppState};

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
    let mut state = AppState::new();
    state.installed_ides = vec![Ide::Vscode, Ide::Cursor, Ide::Claude];
    let p = project("demo");
    state.set_projects(vec![p.clone()]);
    state.selected_index = 1; // Highlight demo project under root

    assert_eq!(state.get_project_ide(&p.path), Ide::Vscode);
    state.cycle_selected_ide();
    assert_eq!(state.get_project_ide(&p.path), Ide::Cursor);
    state.cycle_selected_ide();
    assert_eq!(state.get_project_ide(&p.path), Ide::Claude);
    state.cycle_selected_ide();
    assert_eq!(state.get_project_ide(&p.path), Ide::Vscode);
}

#[test]
fn cycle_selected_ide_skips_uninstalled_ides() {
    let mut state = AppState::new();
    state.installed_ides = vec![Ide::Vscode, Ide::Terminal];
    let p = project("demo");
    state.set_projects(vec![p.clone()]);
    state.selected_index = 1;

    assert_eq!(state.get_project_ide(&p.path), Ide::Vscode);

    state.cycle_selected_ide();
    assert_eq!(state.get_project_ide(&p.path), Ide::Terminal);

    state.cycle_selected_ide();
    assert_eq!(state.get_project_ide(&p.path), Ide::Vscode);
}

#[test]
fn get_project_ide_falls_back_to_first_installed_if_default_uninstalled() {
    let mut state = AppState::new();
    state.default_ide = Ide::Idea;
    state.installed_ides = vec![Ide::Cursor, Ide::Claude];

    let p = project("demo");
    assert_eq!(state.get_project_ide(&p.path), Ide::Cursor);
}

#[test]
fn cycle_selected_ide_on_recent_tab() {
    use dev_cli::models::recent_project::RecentProject;
    use dev_cli::tui::state::Tab;
    use std::path::PathBuf;

    let mut state = AppState::new();
    state.active_tab = Tab::Recent;
    state.installed_ides = vec![Ide::Vscode, Ide::Claude];
    let path = PathBuf::from("/tmp/recent_demo");
    state.recent_projects = vec![RecentProject {
        name: "recent_demo".into(),
        path: path.clone(),
        last_opened: 100,
        ide: Some(Ide::Vscode),
    }];

    assert_eq!(state.get_recent_project_ide(&state.recent_projects[0]), Ide::Vscode);

    state.cycle_selected_ide();
    assert_eq!(state.get_recent_project_ide(&state.recent_projects[0]), Ide::Claude);

    state.cycle_selected_ide();
    assert_eq!(state.get_recent_project_ide(&state.recent_projects[0]), Ide::Vscode);
}

#[test]
fn projects_tab_uses_default_ide_not_recent_ide() {
    use dev_cli::models::recent_project::RecentProject;
    use std::path::PathBuf;

    let mut state = AppState::new();
    state.default_ide = Ide::Vscode;
    state.installed_ides = vec![Ide::Vscode, Ide::Claude];

    let path = PathBuf::from("/tmp/shared_demo");
    let recent = RecentProject {
        name: "shared_demo".into(),
        path: path.clone(),
        last_opened: 100,
        ide: Some(Ide::Claude),
    };
    state.recent_projects = vec![recent.clone()];

    // Recent tab uses recent.ide (Claude)
    assert_eq!(state.get_recent_project_ide(&recent), Ide::Claude);

    // Projects tab uses default_ide (VS Code), ignoring recent.ide
    assert_eq!(state.get_project_ide(&path), Ide::Vscode);
}
