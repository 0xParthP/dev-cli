//! Keyboard event handling.

use super::state::AppState;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub fn handle_key(state: &mut AppState, key: KeyCode) {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => state.quit(),
        _ => {}
    }
}

pub fn handle_events(state: &mut AppState) -> anyhow::Result<()> {
    if !event::poll(Duration::from_millis(100))? {
        return Ok(());
    }

    if let Event::Key(key) = event::read()? {
        handle_key(state, key.code);
    }

    Ok(())
}
