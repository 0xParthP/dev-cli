use anyhow::{Result, bail};
use dev_cli::tui::{app::run_loop, state::AppState, ui};
use ratatui::{Terminal, backend::TestBackend};

#[test]
fn dashboard_draws_successfully() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|frame| ui::render(frame, &AppState::new())).unwrap();
}

#[test]
fn run_loop_draws_once_then_exits() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut ticks = 0;

    run_loop(&mut terminal, |state| -> Result<()> {
        ticks += 1;
        state.quit();
        Ok(())
    })
    .unwrap();

    assert_eq!(ticks, 1);
}

#[test]
fn run_loop_multiple_iterations() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut ticks = 0;

    run_loop(&mut terminal, |state| -> Result<()> {
        ticks += 1;

        if ticks == 3 {
            state.quit();
        }

        Ok(())
    })
    .unwrap();

    assert_eq!(ticks, 3);
}

#[test]
fn run_loop_propagates_errors() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let result = run_loop(&mut terminal, |_state| -> Result<()> {
        bail!("boom");
    });

    assert!(result.is_err());
}
