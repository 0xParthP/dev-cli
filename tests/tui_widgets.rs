use anyhow::Result;
use ratatui::{Terminal, backend::TestBackend};

use dev_cli::tui::{
    state::AppState,
    widgets::{footer, header, project_list, search},
};

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
