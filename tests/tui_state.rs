use crate::common::factories::fake_project as project;
use dev_cli::tui::state::AppState;

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
fn filtered_projects_returns_all_when_query_empty() {
    let mut state = AppState::new();

    state.projects = vec![project("cursor"), project("weather"), project("notes")];

    let filtered = state.filtered_projects();

    assert_eq!(filtered.len(), 3);
}

#[test]
fn filtered_projects_filters_case_insensitively() {
    let mut state = AppState::new();

    state.projects = vec![project("Cursor"), project("cursor-theme"), project("weather")];

    state.search_query = "CURSOR".into();

    let filtered = state.filtered_projects();

    assert_eq!(filtered.len(), 2);
    assert_eq!(filtered[0].name, "Cursor");
    assert_eq!(filtered[1].name, "cursor-theme");
}

#[test]
fn move_down_stops_at_last_project() {
    let mut state = AppState::new();

    state.projects = vec![project("a"), project("b")];

    state.move_down();
    state.move_down();
    state.move_down();

    assert_eq!(state.selected_index, 1);
}

#[test]
fn move_up_stops_at_zero() {
    let mut state = AppState::new();

    state.projects = vec![project("a"), project("b")];
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

    state.projects = vec![project("cursor")];
    state.search_query = "weather".into();
    state.selected_index = 5;

    state.clamp_selection();

    assert_eq!(state.selected_index, 0);
}

#[test]
fn clamp_selection_moves_selection_to_last_item() {
    let mut state = AppState::new();

    state.projects = vec![project("cursor"), project("weather"), project("notes")];

    state.selected_index = 10;

    state.clamp_selection();

    assert_eq!(state.selected_index, 2);
}

#[test]
fn search_filters_projects() {
    let mut state = AppState::new();

    state.projects = vec![project("cursor"), project("weather-app"), project("cursor-theme")];

    state.search_query = "cursor".into();

    let filtered = state.filtered_projects();

    assert_eq!(filtered.len(), 2);
}

#[test]
fn move_down_on_recent_tab_stops_at_end() {
    use dev_cli::models::recent_project::RecentProject;
    use dev_cli::tui::state::Tab;
    use std::path::PathBuf;

    let mut state = AppState::new();
    state.active_tab = Tab::Recent;

    state.recent_projects = vec![
        RecentProject { name: "a".into(), path: PathBuf::from("/a"), last_opened: 0 },
        RecentProject { name: "b".into(), path: PathBuf::from("/b"), last_opened: 0 },
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
