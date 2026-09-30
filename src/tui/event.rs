//! Keyboard event handling for the TUI.

use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{models::project::Project, tui::state::Tab};

use super::state::{AppState, IdeTabFocus, InputMode, SettingsTabFocus};

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

    // Pump the background clone task channels every tick.
    state.tick_clone_task();

    Ok(())
}

/// Handle standard key events.
pub fn handle_key(key: KeyEvent, state: &mut AppState) {
    handle_key_with_launcher(key, state, |_ide, _project| Ok(()))
}

/// Handle key events with a custom launcher dependency injection.
pub fn handle_key_with_launcher<L>(key: KeyEvent, state: &mut AppState, launcher: L)
where
    L: Fn(crate::models::ide::IdeSelection, &Project) -> Result<()>,
{
    match state.input_mode {
        InputMode::Normal => handle_normal_key(key, state, launcher),
        InputMode::Editing => handle_editing_key(key, state),
    }
}

/// Handle keys in Normal mode (navigation, actions, search).
fn handle_normal_key<L>(key: KeyEvent, state: &mut AppState, launcher: L)
where
    L: Fn(crate::models::ide::IdeSelection, &Project) -> Result<()>,
{
    match key.code {
        KeyCode::Esc => {
            if state.is_clone_form_active() {
                state.close_clone_modal();
            } else if state.active_tab == Tab::Ide
                && state.ide_tab_focus == IdeTabFocus::ConfirmDelete
            {
                state.cancel_delete_custom_ide();
            } else if !state.search_query.is_empty() {
                state.clear_search();
                state.clamp_selection();
            } else {
                state.quit();
            }
        }
        KeyCode::F(1) => {
            let _ = state.refresh();
        }
        KeyCode::F(2) => {
            if state.active_tab == Tab::Projects {
                state.open_clone_modal();
            }
        }
        KeyCode::Tab => match state.active_tab {
            Tab::Projects | Tab::Recent => state.cycle_selected_ide(),
            _ => {}
        },

        KeyCode::Down => state.move_down(),
        KeyCode::Up => state.move_up(),

        KeyCode::Backspace => {
            if matches!(state.active_tab, Tab::Projects) {
                state.pop_char();
                state.clamp_selection();
            }
        }

        KeyCode::Delete => {
            if matches!(state.active_tab, Tab::Projects) {
                state.delete_char();
                state.clamp_selection();
            }
        }

        KeyCode::Enter => handle_enter(state, launcher),

        KeyCode::Left => {
            if matches!(state.active_tab, Tab::Projects) && !state.search_query.is_empty() {
                state.move_search_cursor_left();
            } else {
                state.active_tab = match state.active_tab {
                    Tab::Recent => Tab::Recent,
                    Tab::Projects => Tab::Recent,
                    Tab::Ide => Tab::Projects,
                    Tab::Settings => Tab::Ide,
                };

                state.selected_index = 0;
                state.clear_search();
                state.ide_status_message = None;
                state.settings_status_message = None;
                state.clamp_selection();
            }
        }

        KeyCode::Right => {
            if matches!(state.active_tab, Tab::Projects) && !state.search_query.is_empty() {
                state.move_search_cursor_right();
            } else {
                state.active_tab = match state.active_tab {
                    Tab::Recent => Tab::Projects,
                    Tab::Projects => Tab::Ide,
                    Tab::Ide => Tab::Settings,
                    Tab::Settings => Tab::Settings,
                };

                state.selected_index = 0;
                state.clear_search();
                state.ide_status_message = None;
                state.settings_status_message = None;
                state.clamp_selection();
            }
        }

        KeyCode::Home => {
            if matches!(state.active_tab, Tab::Projects) {
                state.move_search_cursor_home();
            }
        }

        KeyCode::End => {
            if matches!(state.active_tab, Tab::Projects) {
                state.move_search_cursor_end();
            }
        }

        KeyCode::Char(c) => match state.active_tab {
            Tab::Projects => {
                state.push_char(c);
                state.clamp_selection();
            }
            Tab::Recent => {}
            Tab::Ide => handle_ide_tab_char(c, state),
            Tab::Settings => handle_settings_tab_char(c, state),
        },

        _ => {}
    }
}

/// Handle Enter key based on active tab.
fn handle_enter<L>(state: &mut AppState, launcher: L)
where
    L: Fn(crate::models::ide::IdeSelection, &Project) -> Result<()>,
{
    match state.active_tab {
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
                if launcher(ide.clone(), project).is_ok() {
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
                let ide = state.get_recent_project_ide(recent);
                if launcher(ide.clone(), &project).is_ok() {
                    state.pending_launch = Some((ide, project));
                    state.quit();
                }
            }
        }

        Tab::Ide => {
            if state.ide_tab_focus == IdeTabFocus::ConfirmDelete {
                state.remove_selected_custom_ide();
            } else {
                state.set_default_selected_ide();
            }
        }

        Tab::Settings => {}
    }
}

/// Handle letter keys on the IDE tab (Normal mode).
fn handle_ide_tab_char(c: char, state: &mut AppState) {
    if state.ide_tab_focus == IdeTabFocus::ConfirmDelete {
        match c {
            'y' | 'Y' => state.remove_selected_custom_ide(),
            'n' | 'N' => state.cancel_delete_custom_ide(),
            _ => {}
        }
        return;
    }

    match c {
        'a' | 'A' => {
            state.ide_tab_focus = IdeTabFocus::AddForm;
            state.input_mode = InputMode::Editing;
            state.ide_form_name.clear();
            state.ide_form_name_cursor = 0;
            state.ide_form_path.clear();
            state.ide_form_path_cursor = 0;
            state.ide_form_args.clear();
            state.ide_form_args_cursor = 0;
            state.ide_form_field = 0;
            state.ide_status_message = None;
        }
        'd' | 'D' => state.request_delete_selected_custom_ide(),
        's' | 'S' => state.set_default_selected_ide(),
        _ => {}
    }
}

/// Handle letter keys on the Settings tab (Normal mode).
fn handle_settings_tab_char(c: char, state: &mut AppState) {
    match c {
        'a' | 'A' => {
            state.settings_tab_focus = SettingsTabFocus::AddRoot;
            state.input_mode = InputMode::Editing;
            state.settings_add_root.clear();
            state.settings_add_root_cursor = 0;
            state.settings_status_message = None;
        }
        'd' | 'D' => state.remove_selected_root(),
        't' | 'T' => state.cycle_theme(),
        'm' | 'M' => state.cycle_max_depth(),
        'l' | 'L' => state.cycle_recent_limit(),
        'r' | 'R' => state.toggle_auto_refresh(),
        'X' => state.reset_config_defaults(),
        _ => {}
    }
}

/// Handle keys in Editing mode (form input).
fn handle_editing_key(key: KeyEvent, state: &mut AppState) {
    if state.is_clone_form_active() {
        match key.code {
            KeyCode::Esc => state.close_clone_modal(),
            KeyCode::Enter => {
                if let Err(msg) = state.submit_clone() {
                    state.clone_status_message = Some(format!("✗ {msg}"));
                }
            }
            KeyCode::Tab | KeyCode::Down => state.cycle_clone_field(),
            KeyCode::BackTab | KeyCode::Up => state.cycle_clone_field_backwards(),
            KeyCode::Left => state.clone_move_left(),
            KeyCode::Right => state.clone_move_right(),
            KeyCode::Backspace => state.clone_pop_char(),
            KeyCode::Delete => state.clone_delete_char(),
            KeyCode::Char(c) => state.clone_push_char(c),
            _ => {}
        }
        return;
    }

    match key.code {
        KeyCode::Esc => {
            // Cancel editing and return to Normal mode.
            state.input_mode = InputMode::Normal;
            match state.active_tab {
                Tab::Ide => state.ide_tab_focus = IdeTabFocus::List,
                Tab::Settings => state.settings_tab_focus = SettingsTabFocus::List,
                _ => {}
            }
        }

        KeyCode::Enter => match state.active_tab {
            Tab::Ide => {
                if let Err(msg) = state.submit_ide_form() {
                    state.ide_status_message = Some(format!("✗ {msg}"));
                }
            }
            Tab::Settings => {
                if let Err(msg) = state.submit_add_root() {
                    state.settings_status_message = Some(format!("✗ {msg}"));
                }
            }
            _ => {}
        },

        KeyCode::Tab => {
            if matches!(state.active_tab, Tab::Ide) {
                state.cycle_ide_form_field();
            }
        }

        KeyCode::Left => state.editing_move_cursor_left(),
        KeyCode::Right => state.editing_move_cursor_right(),
        KeyCode::Home => state.editing_move_cursor_home(),
        KeyCode::End => state.editing_move_cursor_end(),
        KeyCode::Backspace => state.editing_backspace(),
        KeyCode::Delete => state.editing_delete(),

        KeyCode::Char(c) => state.editing_insert_char(c),

        _ => {}
    }
}
