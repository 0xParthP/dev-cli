//! Keyboard event handling for the TUI.

use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};

use super::state::AppState;

/// Production event handler.
pub fn handle_events(state: &mut AppState) -> Result<()> {
    handle_events_with(state, event::poll, event::read)
}

/// Generic event handler used by tests.
pub fn handle_events_with<P, R>(state: &mut AppState, poll: P, read: R) -> Result<()>
where
    P: Fn(Duration) -> Result<bool, std::io::Error>,
    R: Fn() -> Result<Event, std::io::Error>,
{
    if poll(Duration::from_millis(50))?
        && let Event::Key(key) = read()?
    {
        handle_key(key, state);
    }

    Ok(())
}

/// Handle a single keypress.
pub fn handle_key(key: KeyEvent, state: &mut AppState) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => state.quit(),
        _ => {}
    }
}
