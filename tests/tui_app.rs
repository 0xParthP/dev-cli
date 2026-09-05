use anyhow::{Result, anyhow};
use dev_cli::tui::{
    app::{run_loop, tick},
    event::handle_events,
    state::AppState,
    ui,
};
use ratatui::{Terminal, backend::TestBackend};

#[test]
fn dashboard_draws_successfully() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|frame| ui::render(frame, &AppState::new())).unwrap();
}

#[test]
fn app_state_quits() {
    let mut state = AppState::new();
    state.quit();
    assert!(state.should_quit);
}

#[test]
fn handle_events_without_input_returns_ok() -> Result<()> {
    let mut state = AppState::new();

    handle_events(&mut state)?;

    assert!(!state.should_quit);
    Ok(())
}

#[test]
fn handle_events_multiple_times() -> Result<()> {
    let mut state = AppState::new();

    for _ in 0..5 {
        handle_events(&mut state)?;
    }

    assert!(!state.should_quit);
    Ok(())
}

#[test]
fn tick_delegates_to_event_handler() -> Result<()> {
    let mut state = AppState::new();

    tick(&mut state)?;

    assert!(!state.should_quit);

    Ok(())
}

#[test]
fn run_loop_exits_after_first_iteration() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

    run_loop(&mut terminal, |state| {
        state.quit();
        Ok(())
    })?;

    Ok(())
}

#[test]
fn run_loop_runs_multiple_iterations() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

    let mut ticks = 0;

    run_loop(&mut terminal, |state| {
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
fn run_loop_propagates_errors() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let err = run_loop(&mut terminal, |_| Err(anyhow!("boom")));

    assert!(err.is_err());
    assert!(err.unwrap_err().to_string().contains("boom"));
}
