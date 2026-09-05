//! Ratatui application entrypoint.
use std::io::{Stdout, stdout};

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};

use super::{event, state::AppState, ui};

/// Launch the TUI in a real terminal.
pub fn run() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal, event::handle_events);

    restore_terminal(terminal)?;

    result
}

#[doc(hidden)]
pub fn run_loop<B, F>(terminal: &mut Terminal<B>, handle_events: F) -> Result<()>
where
    B: Backend,
    B::Error: std::error::Error + Send + Sync + 'static,
    F: FnMut(&mut AppState) -> Result<()>,
{
    let mut state = AppState::new();

    run_loop_with_state(terminal, &mut state, handle_events)
}

pub fn run_loop_with_state<B, F>(
    terminal: &mut Terminal<B>,
    state: &mut AppState,
    mut handle_events: F,
) -> Result<()>
where
    B: Backend,
    B::Error: std::error::Error + Send + Sync + 'static,
    F: FnMut(&mut AppState) -> Result<()>,
{
    while !state.should_quit {
        terminal.draw(|frame| ui::render(frame, state))?;
        handle_events(state)?;
    }

    Ok(())
}

#[doc(hidden)]
pub fn tick(state: &mut AppState) -> Result<()> {
    event::handle_events(state)
}

/// Restore the user's terminal after exiting the TUI.
fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
