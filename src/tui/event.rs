//! Keyboard event handling for the TUI.

use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{models::project::Project, tui::state::Tab};

use super::state::AppState;

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
    handle_key_with_launcher(key, state, |_ide, _project| Ok(()))
}

/// Handle key events with a custom launcher dependency injection.
pub fn handle_key_with_launcher<L>(key: KeyEvent, state: &mut AppState, launcher: L)
where
    L: Fn(crate::models::ide::Ide, &Project) -> Result<()>,
{
    match key.code {
        KeyCode::Esc => state.quit(),
        KeyCode::F(1) => {
            let _ = state.refresh();
        }
        KeyCode::Tab => {
            state.cycle_selected_ide();
        }

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
                let is_folder = state
                    .visible_items()
                    .get(state.selected_index)
                    .map(|n| n.is_folder)
                    .unwrap_or(false);

                if is_folder {
                    state.toggle_selected();
                } else if let Some(project) = state.selected_project() {
                    let ide = state.get_project_ide(&project.path);
                    if launcher(ide, project).is_ok() {
                        state.pending_launch = Some((ide, project.clone()));
                        state.quit();
                    }
                }
            }

            Tab::Recent => {
                if let Some(recent) = state.selected_recent_project() {
                    let mut project = Project::new(recent.path.clone(), recent.path.clone());
                    if !recent.name.is_empty() {
                        project.name = recent.name.clone();
                    }
                    let ide = state.get_project_ide(&project.path);
                    if launcher(ide, &project).is_ok() {
                        state.pending_launch = Some((ide, project));
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
