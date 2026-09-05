use anyhow::{Result, anyhow};
use dev_cli::tui::{app::run_loop, state::AppState, ui};
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

#[test]
fn run_loop_with_existing_state_runs_until_quit() -> anyhow::Result<()> {
    use dev_cli::tui::app::run_loop_with_state;
    use ratatui::{Terminal, backend::TestBackend};

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
fn run_loop_draws_until_quit() -> anyhow::Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::new();
    let mut ticks = 0;

    dev_cli::tui::app::run_loop_with_state(&mut terminal, &mut state, |state| {
        ticks += 1;

        if ticks == 3 {
            state.quit();
        }

        Ok(())
    })?;

    assert_eq!(ticks, 3);
    assert!(state.should_quit);

    Ok(())
}

#[test]
fn run_loop_with_already_quit_state_returns_immediately() -> anyhow::Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::new();
    state.quit();

    let mut called = false;

    dev_cli::tui::app::run_loop_with_state(&mut terminal, &mut state, |_| {
        called = true;
        Ok(())
    })?;

    assert!(!called);

    Ok(())
}

#[test]
fn run_loop_propagates_event_errors() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = AppState::new();

    let err = dev_cli::tui::app::run_loop_with_state(&mut terminal, &mut state, |_| {
        Err(anyhow::anyhow!("boom"))
    })
    .unwrap_err();

    assert_eq!(err.to_string(), "boom");
}

#[cfg(windows)]
#[test]
fn handle_events_without_input_returns_ok() -> anyhow::Result<()> {
    use dev_cli::tui::event::handle_events;
    let mut state = AppState::new();
    handle_events(&mut state)?;
    assert!(!state.should_quit);
    Ok(())
}

#[cfg(windows)]
#[test]
fn handle_events_multiple_times() -> anyhow::Result<()> {
    use dev_cli::tui::event::handle_events;
    let mut state = AppState::new();
    for _ in 0..5 {
        handle_events(&mut state)?;
    }

    assert!(!state.should_quit);
    Ok(())
}
