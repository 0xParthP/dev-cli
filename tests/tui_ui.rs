use anyhow::Result;
use dev_cli::tui::{
    state::{AppState, Tab},
    ui,
};
use ratatui::{Terminal, backend::TestBackend};

#[test]
fn dashboard_renders() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

    let state = AppState::new();

    terminal.draw(|frame| ui::render(frame, &state))?;

    let text =
        terminal.backend().buffer().content().iter().map(|cell| cell.symbol()).collect::<String>();

    assert!(text.contains("dev-cli"));
    Ok(())
}

#[test]
fn dashboard_renders_footer() -> Result<()> {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

    let state = AppState::new();

    terminal.draw(|frame| ui::render(frame, &state))?;

    let text =
        terminal.backend().buffer().content().iter().map(|cell| cell.symbol()).collect::<String>();

    assert!(text.contains("Q"));
    assert!(text.contains("Quit"));
    assert!(text.contains("Search"));
    assert!(text.contains("Enter"));

    Ok(())
}

#[test]
fn dashboard_renders_on_small_terminal() -> Result<()> {
    let backend = TestBackend::new(30, 10);
    let mut terminal = Terminal::new(backend)?;

    let state = AppState::new();

    terminal.draw(|frame| ui::render(frame, &state))?;

    let text =
        terminal.backend().buffer().content().iter().map(|cell| cell.symbol()).collect::<String>();

    println!("{text}");
    assert!(text.contains("dev-cli"));

    Ok(())
}

#[test]
fn dashboard_renders_tabs() -> Result<()> {
    let backend = TestBackend::new(80, 20);
    let mut terminal = Terminal::new(backend)?;

    let state = AppState::new();

    terminal.draw(|frame| ui::render(frame, &state))?;

    let rendered: String =
        terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

    assert!(rendered.contains("Projects"));
    assert!(rendered.contains("Recent"));
    assert!(rendered.contains("IDE"));
    assert!(rendered.contains("Settings"));

    Ok(())
}

#[test]
fn dashboard_opens_on_recent_tab() -> Result<()> {
    let backend = TestBackend::new(80, 20);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::new();
    state.active_tab = Tab::Recent;

    terminal.draw(|frame| ui::render(frame, &state))?;

    let rendered: String =
        terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect();

    assert!(rendered.contains("Recent Projects"));
    assert!(rendered.contains("No recent projects"));

    Ok(())
}
