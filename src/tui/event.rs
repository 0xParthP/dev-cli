//! Keyboard event handling for the TUI.

use std::time::Duration;

use super::{actions, state::AppState};
use crate::tui::state::Tab;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub fn handle_events(state: &mut AppState) -> Result<()> {
    handle_events_with(state, event::poll, event::read)
}

pub fn handle_events_with<P, R>(state: &mut AppState, poll: P, read: R) -> Result<()>
where
    P: Fn(Duration) -> Result<bool, std::io::Error>,
    R: Fn() -> Result<Event, std::io::Error>,
{
    if poll(Duration::from_millis(50))?
        && let Event::Key(key) = read()?
    {
        // Only react to actual key presses.
        if key.kind == KeyEventKind::Press {
            handle_key(key, state);
        }
    }

    Ok(())
}

pub fn handle_key(key: KeyEvent, state: &mut AppState) {
    handle_key_with_launcher(key, state, actions::open_project)
}

pub fn handle_key_with_launcher<L>(key: KeyEvent, state: &mut AppState, launcher: L)
where
    L: Fn(&crate::models::project::Project) -> Result<()>,
{
    match key.code {
        KeyCode::Esc => state.quit(),

        KeyCode::Down => state.move_down(),
        KeyCode::Up => state.move_up(),

        KeyCode::Backspace => {
            state.pop_char();
            state.clamp_selection();
        }

        KeyCode::Char(c) => {
            state.push_char(c);
            state.clamp_selection();
        }

        KeyCode::Enter => {
            if let Some(project) = state.selected_project()
                && launcher(project).is_ok()
            {
                state.quit();
            }
        }

        KeyCode::Left => {
            state.active_tab = match state.active_tab {
                Tab::Projects => Tab::Projects,
                Tab::Recent => Tab::Projects,
                Tab::Ide => Tab::Recent,
                Tab::Settings => Tab::Ide,
            };
        }

        KeyCode::Right => {
            state.active_tab = match state.active_tab {
                Tab::Projects => Tab::Recent,
                Tab::Recent => Tab::Ide,
                Tab::Ide => Tab::Settings,
                Tab::Settings => Tab::Settings,
            };
        }

        _ => {}
    }
}
