use dev_cli::tui::state::AppState;

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
