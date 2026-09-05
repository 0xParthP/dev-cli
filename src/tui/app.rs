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

/// Main application loop.
///
/// This is generic over the backend and the event handler so it can be tested
/// with Ratatui's `TestBackend` without entering raw mode.
pub fn run_loop<B, F>(terminal: &mut Terminal<B>, mut handle_events: F) -> Result<()>
where
    B: Backend,
    B::Error: std::error::Error + Send + Sync + 'static,
    F: FnMut(&mut AppState) -> Result<()>,
{
    let mut state = AppState::new();

    while !state.should_quit {
        terminal.draw(|frame| ui::render(frame, &state))?;
        handle_events(&mut state)?;
    }

    Ok(())
}

/// Restore the user's terminal after exiting the TUI.
fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
