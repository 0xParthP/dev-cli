use anyhow::Result;
use ratatui::{Terminal, backend::TestBackend, buffer::Buffer};

use dev_cli::tui::{
    state::AppState,
    widgets::{footer, header, project_list, search},
};

use dev_cli::models::project::Project;

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
    let state = AppState::new();

    terminal.draw(|frame| {
        header::render(frame, frame.area(), &state);
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

    state.projects = vec![project("alpha"), project("beta"), project("gamma")];

    // Select "beta".
    state.move_down();

    let buffer = render_widget(
        |frame| {
            project_list::render(frame, ratatui::layout::Rect::new(0, 0, 40, 10), &state);
        },
        40,
        10,
    );

    // Convert buffer into plain text.
    let text: String = buffer.content().iter().map(|cell| cell.symbol()).collect();

    assert!(text.contains("alpha"));
    assert!(text.contains("beta"));
    assert!(text.contains("gamma"));

    Ok(())
}
