//! Keyboard event handling.

use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};

use super::state::AppState;

/// Polls for keyboard input and updates application state.
pub fn handle_events(state: &mut AppState) -> Result<()> {
    if !event::poll(Duration::from_millis(100))? {
        return Ok(());
    }

    let Event::Key(key) = event::read()? else {
        return Ok(());
    };

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => state.quit(),
        _ => {}
    }

    Ok(())
}
