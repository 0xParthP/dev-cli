mod common;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use dev_cli::tui::{
    event::handle_key,
    state::{AppState, InputMode, Tab},
};
use serial_test::serial;

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
#[serial]
fn open_and_close_clone_form() {
    let mut state = AppState::new();
    assert!(!state.is_clone_form_active());
    assert_eq!(state.input_mode, InputMode::Normal);

    state.open_clone_modal();
    assert!(state.is_clone_form_active());
    assert_eq!(state.input_mode, InputMode::Editing);

    handle_key(key(KeyCode::Esc), &mut state);
    assert!(!state.is_clone_form_active());
    assert_eq!(state.input_mode, InputMode::Normal);
}

#[test]
#[serial]
fn clone_form_fields_and_navigation() {
    let mut state = AppState::new();
    state.open_clone_modal();

    // Field 0: URL
    assert_eq!(state.clone_field, 0);
    handle_key(key(KeyCode::Char('h')), &mut state);
    handle_key(key(KeyCode::Char('t')), &mut state);
    assert_eq!(state.clone_url, "ht");

    // Cycle to Field 1: Target Root Selector via Tab
    handle_key(key(KeyCode::Tab), &mut state);
    assert_eq!(state.clone_field, 1);

    // Cycle to Field 2: Custom Name via Tab
    handle_key(key(KeyCode::Tab), &mut state);
    assert_eq!(state.clone_field, 2);
    handle_key(key(KeyCode::Char('m')), &mut state);
    handle_key(key(KeyCode::Char('y')), &mut state);
    assert_eq!(state.clone_name, "my");

    // Cycle backwards to Field 1 via BackTab
    handle_key(key(KeyCode::BackTab), &mut state);
    assert_eq!(state.clone_field, 1);

    // Cycle backwards to Field 0 via BackTab
    handle_key(key(KeyCode::BackTab), &mut state);
    assert_eq!(state.clone_field, 0);
}

#[test]
#[serial]
fn clone_form_submit_empty_url_fails() {
    let mut state = AppState::new();
    state.open_clone_modal();

    handle_key(key(KeyCode::Enter), &mut state);

    assert!(state.is_clone_form_active());
    assert!(state.clone_status_message.is_some());
    assert!(state.clone_status_message.unwrap().contains("URL cannot be empty"));
}

#[test]
#[serial]
fn clone_shortcut_is_f2_on_projects_tab() {
    let mut state = AppState::new();

    // On Recent tab: F2 does not open clone form
    state.active_tab = Tab::Recent;
    handle_key(key(KeyCode::F(2)), &mut state);
    assert!(!state.is_clone_form_active());

    // On Projects tab: F2 opens clone form
    state.active_tab = Tab::Projects;
    handle_key(key(KeyCode::F(2)), &mut state);
    assert!(state.is_clone_form_active());
}

#[test]
#[serial]
fn c_key_goes_to_search_not_clone() {
    let mut state = AppState::new();
    state.active_tab = Tab::Projects;

    // Pressing 'c' should feed into the search bar, not open clone form
    handle_key(key(KeyCode::Char('c')), &mut state);
    assert!(!state.is_clone_form_active());
    assert_eq!(state.search_query, "c");
}

#[test]
#[serial]
fn clone_form_closes_automatically_on_successful_clone() {
    use dev_cli::commands::clone::CloneChannels;
    use std::path::PathBuf;
    use std::sync::mpsc;

    let mut state = AppState::new();
    state.open_clone_modal();
    assert!(state.is_clone_form_active());

    let (log_tx, log_rx) = mpsc::channel();
    let (done_tx, done_rx) = mpsc::channel();
    state.clone_task = Some(CloneChannels { log_rx, done_rx });

    log_tx.send("Cloning...".to_string()).unwrap();
    done_tx.send(Ok(PathBuf::from("/tmp/myrepo"))).unwrap();

    state.tick_clone_task();

    assert!(!state.is_clone_form_active());
    assert_eq!(state.input_mode, InputMode::Normal);
}
