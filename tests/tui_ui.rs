use anyhow::Result;
use dev_cli::tui::{state::AppState, ui};
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
    assert!(text.contains("0 projects"));

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

    assert!(text.contains("dev-cli"));

    Ok(())
}
