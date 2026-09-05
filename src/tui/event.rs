use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
/// Keyboard event handling.
use std::time::Duration;

use super::state::AppState;

/// Handle a single key press.
pub fn handle_key(state: &mut AppState, key: KeyCode) {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => state.quit(),
        _ => {}
    }
}

/// Read one crossterm event.
///
/// Separated for testing.
fn read_event() -> Result<Option<Event>> {
    if !event::poll(Duration::from_millis(100))? {
        return Ok(None);
    }

    Ok(Some(event::read()?))
}

/// Poll terminal input.
pub fn handle_events(state: &mut AppState) -> Result<()> {
    if let Some(Event::Key(key)) = read_event()? {
        handle_key(state, key.code);
    }

    Ok(())
}
