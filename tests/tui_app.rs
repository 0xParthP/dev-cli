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
