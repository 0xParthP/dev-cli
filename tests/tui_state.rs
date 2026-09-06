use dev_cli::{models::project::Project, tui::state::AppState};
use std::path::PathBuf;

fn project(name: &str) -> Project {
    let root = PathBuf::from(format!("/projects/{name}"));

    Project {
        name: name.into(),
        path: root.clone(),
        root: root.clone(),
        git_dir: root.join(".git"),
    }
}

#[test]
fn app_state_starts_running() {
    let state = AppState::new();

    assert!(!state.should_quit);
}

#[test]
fn quit_sets_flag() {
    let mut state = AppState::new();

    state.quit();

    assert!(state.should_quit);
}

#[test]
fn quit_is_idempotent() {
    let mut state = AppState::new();

    state.quit();
    state.quit();

    assert!(state.should_quit);
}

#[test]
fn empty_state_has_no_projects() {
    let state = AppState::new();

    assert!(state.projects.is_empty());
    assert!(state.search_query.is_empty());
    assert_eq!(state.selected_index, 0);
}

#[test]
fn set_projects_replaces_project_list() {
    let mut state = AppState::new();

    state.set_projects(vec![project("dev-cli"), project("portfolio")]);

    assert_eq!(state.projects.len(), 2);
    assert_eq!(state.selected_index, 0);
}

#[test]
fn push_char_updates_search_query() {
    let mut state = AppState::new();

    state.push_char('d');
    state.push_char('e');
    state.push_char('v');

    assert_eq!(state.search_query, "dev");
}

#[test]
fn backspace_removes_last_character() {
    let mut state = AppState::new();

    state.push_char('d');
    state.push_char('e');
    state.push_char('v');

    state.backspace();

    assert_eq!(state.search_query, "de");
}

#[test]
fn filtering_is_case_insensitive() {
    let mut state = AppState::new();

    state.set_projects(vec![project("Dev-CLI"), project("Portfolio"), project("Backend")]);

    state.search_query = "dev".into();

    let filtered = state.filtered_projects();

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].name, "Dev-CLI");
}

#[test]
fn filtering_returns_all_when_query_empty() {
    let mut state = AppState::new();

    state.set_projects(vec![project("One"), project("Two")]);

    assert_eq!(state.filtered_projects().len(), 2);
}

#[test]
fn select_next_stops_at_end() {
    let mut state = AppState::new();

    state.set_projects(vec![project("One"), project("Two")]);

    state.select_next();
    state.select_next();
    state.select_next();

    assert_eq!(state.selected_index, 1);
}

#[test]
fn select_previous_saturates_at_zero() {
    let mut state = AppState::new();

    state.set_projects(vec![project("One"), project("Two")]);

    state.select_previous();

    assert_eq!(state.selected_index, 0);
}

#[test]
fn selected_project_returns_current_selection() {
    let mut state = AppState::new();

    state.set_projects(vec![project("One"), project("Two")]);

    state.select_next();

    assert_eq!(state.selected_project().unwrap().name, "Two");
}
