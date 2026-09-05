use crossterm::event::KeyCode;
use dev_cli::tui::{event::handle_key, state::AppState};

#[test]
fn pressing_q_quits() {
    let mut state = AppState::new();

    handle_key(&mut state, KeyCode::Char('q'));

    assert!(state.should_quit);
}

#[test]
fn pressing_escape_quits() {
    let mut state = AppState::new();

    handle_key(&mut state, KeyCode::Esc);

    assert!(state.should_quit);
}

#[test]
fn unrelated_keys_do_not_quit() {
    let mut state = AppState::new();

    handle_key(&mut state, KeyCode::Char('a'));
    handle_key(&mut state, KeyCode::Enter);
    handle_key(&mut state, KeyCode::Up);

    assert!(!state.should_quit);
}
