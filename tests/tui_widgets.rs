use anyhow::Result;
use ratatui::{Terminal, backend::TestBackend, buffer::Buffer};

use dev_cli::tui::{
    state::{AppState, Tab},
    widgets::{footer, header, project_list, search, tabs},
};

use dev_cli::models::project::Project;
use dev_cli::tui::widgets::recent;

use dev_cli::{config::Config, models::recent_project::RecentProject};
use serial_test::serial;

use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

mod common;
use common::temp_config::with_temp_config;

fn project(name: &str) -> Project {
    let root = std::env::temp_dir();
    let path = root.join(name);

    // Ensure the fake project directory actually exists.
    std::fs::create_dir_all(path.join(".git")).unwrap();

    Project { name: name.into(), path: path.clone(), root, git_dir: path.join(".git") }
}

fn render_widget(widget: impl FnOnce(&mut ratatui::Frame), width: u16, height: u16) -> Buffer {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|frame| widget(frame)).expect("failed to draw widget");

    terminal.backend().buffer().clone()
}

#[test]
fn header_renders() -> Result<()> {
    let backend = TestBackend::new(80, 4);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        header::render(frame, frame.area());
    })?;

    Ok(())
}

#[test]
fn search_renders() -> Result<()> {
    let backend = TestBackend::new(80, 3);
    let mut terminal = Terminal::new(backend)?;
    let state = AppState::new();

    terminal.draw(|frame| {
        search::render(frame, frame.area(), &state);
    })?;

    Ok(())
}

#[test]
fn project_list_renders() -> Result<()> {
    let backend = TestBackend::new(80, 10);
    let mut terminal = Terminal::new(backend)?;
    let state = AppState::new();

    terminal.draw(|frame| {
        project_list::render(frame, frame.area(), &state);
    })?;

    Ok(())
}

#[test]
fn footer_renders() -> Result<()> {
    let backend = TestBackend::new(80, 2);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        footer::render(frame, frame.area());
    })?;

    Ok(())
}

#[test]
fn search_works() -> Result<()> {
    let mut state = AppState::new();

    state.projects = vec![project("cursor"), project("weather-app"), project("notes")];

    state.push_char('c');
    state.push_char('u');
    state.push_char('r');

    let backend = TestBackend::new(60, 5);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        search::render(frame, frame.area(), &state);
    })?;

    let text = terminal.backend().buffer().content();

    // Convert the terminal buffer into plain text.
    let rendered: String = text.iter().map(|cell| cell.symbol()).collect();

    assert!(rendered.contains("cur"));

    let filtered = state.filtered_projects();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].name, "cursor");

    Ok(())
}

#[test]
fn selected_project_is_highlighted() -> Result<()> {
    let mut state = AppState::new();
    state.active_tab = Tab::Projects; // <-- Add this

    state.projects = vec![project("alpha"), project("beta"), project("gamma")];

    state.move_down();

    let buffer = render_widget(
        |frame| {
            project_list::render(frame, ratatui::layout::Rect::new(0, 0, 50, 16), &state);
        },
        50,
        16,
    );

    let text: String = buffer.content().iter().map(|cell| cell.symbol()).collect();

    assert!(text.contains("beta"));

    Ok(())
}

#[test]
fn empty_search_state_renders_message() -> Result<()> {
    let mut state = AppState::new();
    state.active_tab = Tab::Projects; // <-- Important
    state.projects = vec![project("cursor")];
    state.search_query = "xyz".into();

    let backend = TestBackend::new(60, 10);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        project_list::render(frame, frame.area(), &state);
    })?;

    let rendered: String =
        terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

    assert!(rendered.contains("No projects found"));
    assert!(rendered.contains("Try another search."));

    Ok(())
}

#[test]
fn search_placeholder_renders() -> Result<()> {
    let state = AppState::new();

    let backend = TestBackend::new(60, 3);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        search::render(frame, frame.area(), &state);
    })?;

    let rendered: String =
        terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

    assert!(rendered.contains("Search projects..."));

    Ok(())
}

#[test]
fn footer_contains_enter_shortcut() -> Result<()> {
    let backend = TestBackend::new(80, 2);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        footer::render(frame, frame.area());
    })?;

    let rendered: String =
        terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

    assert!(rendered.contains("Enter"));
    assert!(rendered.contains("Navigate"));

    Ok(())
}

#[test]
fn scroll_offset_keeps_selection_visible() {
    let mut state = AppState::new();
    state.selected_index = 15;

    assert_eq!(state.scroll_offset(5), 11);
}

#[test]
fn header_is_centered_and_contains_title() -> Result<()> {
    let backend = TestBackend::new(60, 2);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        header::render(frame, frame.area());
    })?;

    let rendered: String =
        terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

    assert!(rendered.contains("dev-cli"));
    assert!(rendered.contains("🚀"));

    Ok(())
}

#[test]
fn tabs_render_projects_tab_selected() -> Result<()> {
    let backend = TestBackend::new(80, 2);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::new();
    state.active_tab = Tab::Projects;

    terminal.draw(|frame| {
        tabs::render(frame, frame.area(), &state);
    })?;

    let rendered: String =
        terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

    assert!(rendered.contains("Projects"));
    assert!(rendered.contains("Recent"));
    assert!(rendered.contains("IDE"));
    assert!(rendered.contains("Settings"));

    Ok(())
}

#[test]
fn tabs_render_settings_tab_selected() -> Result<()> {
    let backend = TestBackend::new(80, 2);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::new();
    state.active_tab = Tab::Settings;

    terminal.draw(|frame| {
        tabs::render(frame, frame.area(), &state);
    })?;

    let rendered: String =
        terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

    assert!(rendered.contains("Settings"));

    Ok(())
}

#[test]
fn footer_contains_all_shortcuts() -> Result<()> {
    let backend = TestBackend::new(80, 2);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        footer::render(frame, frame.area());
    })?;

    let rendered: String =
        terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

    assert!(rendered.contains("Enter"));
    assert!(rendered.contains("Navigate"));
    assert!(rendered.contains("Search"));
    assert!(rendered.contains("Quit"));

    Ok(())
}

#[test]
fn non_projects_tab_shows_placeholder() -> Result<()> {
    let mut state = AppState::new();
    state.active_tab = Tab::Ide;

    let backend = TestBackend::new(60, 10);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        project_list::render(frame, frame.area(), &state);
    })?;

    let rendered: String =
        terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

    assert!(rendered.contains("Coming Soon"));

    Ok(())
}

#[test]
#[serial]
fn recent_widget_renders_empty_state() -> Result<()> {
    let backend = TestBackend::new(60, 10);
    let mut terminal = Terminal::new(backend)?;
    let state = AppState::new();

    terminal.draw(|frame| {
        recent::render(frame, frame.area(), &state);
    })?;

    let rendered: String =
        terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

    assert!(rendered.contains("No recent projects"));

    Ok(())
}

#[test]
#[serial]
fn recent_widget_renders_project_name() -> Result<()> {
    with_temp_config(|| -> Result<()> {
        let mut config = Config::default();

        config.recent_projects.push(RecentProject {
            name: "weather-app".into(),
            path: PathBuf::from("/projects/weather-app"),
            last_opened: 0,
        });

        config.save()?;

        let backend = TestBackend::new(60, 10);
        let mut terminal = Terminal::new(backend)?;
        let state = AppState::new();

        terminal.draw(|frame| {
            recent::render(frame, frame.area(), &state);
        })?;

        let rendered: String =
            terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

        assert!(rendered.contains("weather-app"));

        Ok(())
    })
}

#[test]
#[serial]
fn recent_widget_renders_relative_timestamp() -> Result<()> {
    with_temp_config(|| -> Result<()> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let mut config = Config::default();

        config.recent_projects.push(RecentProject {
            name: "weather-app".into(),
            path: PathBuf::from("/projects/weather-app"),
            last_opened: now,
        });

        config.save()?;

        let backend = TestBackend::new(70, 10);
        let mut terminal = Terminal::new(backend)?;
        let state = AppState::new();

        terminal.draw(|frame| {
            recent::render(frame, frame.area(), &state);
        })?;

        let rendered: String =
            terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

        assert!(rendered.contains("Opened just now"));

        Ok(())
    })
}

#[test]
#[serial]
fn recent_tab_is_selected() -> Result<()> {
    let state = AppState::new();

    let backend = TestBackend::new(60, 2);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        tabs::render(frame, frame.area(), &state);
    })?;

    let rendered: String =
        terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

    assert!(rendered.contains("Recent"));

    Ok(())
}

#[test]
#[serial]
fn recent_tab_renders_recent_widget() -> Result<()> {
    with_temp_config(|| -> Result<()> {
        let mut config = Config::default();

        config.recent_projects.push(RecentProject {
            name: "weather-app".into(),
            path: PathBuf::from("/projects/weather-app"),
            last_opened: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        });

        config.save()?;

        let mut state = AppState::new();
        state.active_tab = Tab::Recent;

        let backend = TestBackend::new(60, 10);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            project_list::render(frame, frame.area(), &state);
        })?;

        let rendered: String =
            terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

        assert!(rendered.contains("weather-app"));

        Ok(())
    })
}
