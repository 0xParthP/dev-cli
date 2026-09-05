use anyhow::{Result, anyhow};
use dev_cli::tui::{
    app::{run_loop, run_loop_with_state},
    state::AppState,
    ui,
};
use ratatui::{Terminal, backend::TestBackend};

#[test]
fn dashboard_draws_successfully() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| ui::render(f, &AppState::new()))?;
    Ok(())
}

#[test]
fn app_state_quits() {
    let mut state = AppState::new();
    state.quit();

    assert!(state.should_quit);
}

#[test]
fn run_loop_quits_after_first_tick() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

    let mut called = false;

    run_loop(&mut terminal, |state| {
        called = true;
        state.quit();
        Ok(())
    })?;

    assert!(called);
    Ok(())
}

#[test]
fn run_loop_multiple_iterations() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

    let mut ticks = 0;

    run_loop(&mut terminal, |state| {
        ticks += 1;

        if ticks == 5 {
            state.quit();
        }

        Ok(())
    })?;

    assert_eq!(ticks, 5);
    Ok(())
}

#[test]
fn run_loop_propagates_errors() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let err = run_loop(&mut terminal, |_| Err(anyhow!("boom"))).unwrap_err();

    assert_eq!(err.to_string(), "boom");
}

#[test]
fn run_app_exits_when_handler_requests_quit() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

    run_loop(&mut terminal, |state| {
        state.quit();
        Ok(())
    })?;

    Ok(())
}

#[test]
fn run_loop_immediately_exits_when_state_already_quit() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

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

#[test]
fn run_loop_draws_once_then_quits() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::new();
    let mut ticks = 0;

    run_loop_with_state(&mut terminal, &mut state, |state| {
        ticks += 1;
        state.quit();
        Ok(())
    })?;

    assert_eq!(ticks, 1);

    Ok(())
}

#[test]
fn run_loop_draws_multiple_frames() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

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
fn run_loop_propagates_event_errors() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();

    let err = run_loop_with_state(&mut terminal, &mut state, |_| Err(anyhow!("boom")));

    assert!(err.is_err());
}

#[test]
fn repeated_draw_calls_are_safe() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

    let state = AppState::new();

    for _ in 0..5 {
        terminal.draw(|frame| ui::render(frame, &state))?;
    }

    Ok(())
}
