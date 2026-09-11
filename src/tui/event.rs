//! Keyboard event handling for the TUI.

use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{models::project::Project, tui::state::Tab};

use super::{actions, state::AppState};

/// Poll and handle terminal events.
pub fn handle_events(state: &mut AppState) -> Result<()> {
    handle_events_with(state, event::poll, event::read)
}

/// Poll and handle terminal events with custom implementations.
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

/// Handle standard key events.
pub fn handle_key(key: KeyEvent, state: &mut AppState) {
    handle_key_with_launcher(key, state, actions::open_project)
}

/// Handle key events with a custom launcher dependency injection.
pub fn handle_key_with_launcher<L>(key: KeyEvent, state: &mut AppState, launcher: L)
where
    L: Fn(&Project) -> Result<()>,
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

        KeyCode::Enter => match state.active_tab {
            Tab::Projects => {
                if let Some(project) = state.selected_project()
                    && launcher(project).is_ok()
                {
                    state.quit();
                }
            }

            Tab::Recent => {
                if let Some(recent) = state.selected_recent_project() {
                    let project = Project::new(recent.path.clone(), recent.path.clone());
                    if launcher(&project).is_ok() {
                        state.quit();
                    }
                }
            }

            _ => {}
        },

        KeyCode::Left => {
            state.active_tab = match state.active_tab {
                Tab::Recent => Tab::Recent,
                Tab::Projects => Tab::Recent,
                Tab::Ide => Tab::Projects,
                Tab::Settings => Tab::Ide,
            };

            state.selected_index = 0;
            state.clamp_selection();
        }

        KeyCode::Right => {
            state.active_tab = match state.active_tab {
                Tab::Recent => Tab::Projects,
                Tab::Projects => Tab::Ide,
                Tab::Ide => Tab::Settings,
                Tab::Settings => Tab::Settings,
            };

            state.selected_index = 0;
            state.clamp_selection();
        }

        _ => {}
    }
}
