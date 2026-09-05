use anyhow::Result;
use ratatui::{Terminal, backend::TestBackend};

use dev_cli::tui::{app::run_loop_with_state, state::AppState, ui};

#[test]
fn dashboard_renders() {
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|frame| ui::render(frame, &AppState::new())).unwrap();
}

#[test]
fn app_state_quit_sets_flag() {
    let mut state = AppState::new();

    state.quit();

    assert!(state.should_quit);
}

#[test]
fn run_loop_exits_after_first_tick() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();

    let mut ticks = 0;

    run_loop_with_state(&mut terminal, &mut state, |state| {
        ticks += 1;
        state.quit();
        Ok(())
    })?;

    assert_eq!(ticks, 1);
    assert!(state.should_quit);

    Ok(())
}

#[test]
fn run_loop_handles_multiple_iterations() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();

    let mut ticks = 0;

    run_loop_with_state(&mut terminal, &mut state, |state| {
        ticks += 1;

        if ticks == 3 {
            state.quit();
        }

        Ok(())
    })?;

    assert_eq!(ticks, 3);

    Ok(())
}

#[test]
fn run_loop_skips_when_state_already_quit() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();
    state.quit();

    let mut called = false;

    run_loop_with_state(&mut terminal, &mut state, |_| {
        called = true;
        Ok(())
    })?;

    assert!(!called);

    Ok(())
}
