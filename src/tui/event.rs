//! Keyboard event handling.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};

use super::state::AppState;

/// Production event handler.
pub fn handle_events(state: &mut AppState) -> Result<()> {
    handle_events_with(state, event::poll, event::read)
}

/// Generic event handler used by tests.
pub fn handle_events_with<P, R>(state: &mut AppState, mut poll: P, mut read: R) -> Result<()>
where
    P: FnMut(std::time::Duration) -> std::io::Result<bool>,
    R: FnMut() -> std::io::Result<Event>,
{
    if poll(std::time::Duration::from_millis(16))?
        && let Event::Key(key) = read()?
    {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => state.quit(),
            _ => {}
        }
    }

    Ok(())
}
