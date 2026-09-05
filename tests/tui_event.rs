use crossterm::event::KeyCode;
use dev_cli::tui::{event::handle_key, state::AppState};

#[test]
fn q_quits() {
    let mut state = AppState::new();
    handle_key(&mut state, KeyCode::Char('q'));
    assert!(state.should_quit);
}

#[test]
fn escape_quits() {
    let mut state = AppState::new();
    handle_key(&mut state, KeyCode::Esc);
    assert!(state.should_quit);
}

#[test]
fn other_keys_do_nothing() {
    let mut state = AppState::new();

    handle_key(&mut state, KeyCode::Enter);
    handle_key(&mut state, KeyCode::Left);
    handle_key(&mut state, KeyCode::Right);
    handle_key(&mut state, KeyCode::Char('a'));

    assert!(!state.should_quit);
}

#[test]
fn quit_remains_true() {
    let mut state = AppState::new();

    handle_key(&mut state, KeyCode::Char('q'));
    handle_key(&mut state, KeyCode::Enter);

    assert!(state.should_quit);
}
