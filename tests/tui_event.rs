use std::path::PathBuf;

use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

use dev_cli::{
    models::project::Project,
    tui::{
        event::{handle_events_with, handle_key},
        state::AppState,
    },
};

fn project(name: &str) -> Project {
    let root = PathBuf::from("/tmp");
    let path = root.join(name);

    Project { name: name.into(), path: path.clone(), root, git_dir: path.join(".git") }
}

fn key(code: KeyCode) -> Event {
    Event::Key(KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    })
}

#[test]
fn does_nothing_when_no_event_available() -> Result<()> {
    let mut state = AppState::new();

    handle_events_with(&mut state, |_| Ok(false), || unreachable!())?;

    assert!(!state.should_quit);
    Ok(())
}

#[test]
fn quits_on_q() -> Result<()> {
    let mut state = AppState::new();

    handle_events_with(&mut state, |_| Ok(true), || Ok(key(KeyCode::Char('q'))))?;

    assert!(state.should_quit);
    Ok(())
}

#[test]
fn quits_on_escape() -> Result<()> {
    let mut state = AppState::new();

    handle_events_with(&mut state, |_| Ok(true), || Ok(key(KeyCode::Esc)))?;

    assert!(state.should_quit);
    Ok(())
}

#[test]
fn ignores_other_keys() -> Result<()> {
    let mut state = AppState::new();

    handle_events_with(&mut state, |_| Ok(true), || Ok(key(KeyCode::Enter)))?;

    assert!(!state.should_quit);
    Ok(())
}

#[test]
fn ignores_resize_events() -> Result<()> {
    let mut state = AppState::new();

    handle_events_with(&mut state, |_| Ok(true), || Ok(Event::Resize(100, 40)))?;

    assert!(!state.should_quit);
    Ok(())
}

#[test]
fn q_key_quits() {
    let mut state = AppState::new();

    handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE), &mut state);

    assert!(state.should_quit);
}

#[test]
fn esc_key_quits() {
    let mut state = AppState::new();

    handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE), &mut state);

    assert!(state.should_quit);
}

#[test]
fn other_keys_do_nothing() {
    let mut state = AppState::new();

    handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE), &mut state);

    assert!(!state.should_quit);
}

/// ===== Tests for milestone 4.2.3 =====

#[test]
fn down_key_moves_selection() {
    let mut state = AppState::new();
    state.projects = vec![project("alpha"), project("beta")];

    handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE), &mut state);

    assert_eq!(state.selected_index, 1);
}

#[test]
fn up_key_moves_selection() {
    let mut state = AppState::new();
    state.projects = vec![project("alpha"), project("beta")];
    state.selected_index = 1;

    handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE), &mut state);

    assert_eq!(state.selected_index, 0);
}

#[test]
fn typing_adds_character() {
    let mut state = AppState::new();

    handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE), &mut state);
    handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE), &mut state);

    assert_eq!(state.search_query, "ab");
}

#[test]
fn backspace_removes_character() {
    let mut state = AppState::new();
    state.search_query = "cursor".into();

    handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE), &mut state);

    assert_eq!(state.search_query, "curso");
}
