use dev_cli::tui::{state::AppState, ui::render};
use ratatui::{Terminal, backend::TestBackend};

fn buffer_string(terminal: &Terminal<TestBackend>) -> String {
    terminal.backend().buffer().content().iter().map(|cell| cell.symbol()).collect()
}

#[test]
fn dashboard_renders() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let state = AppState::new();

    terminal.draw(|frame| render(frame, &state)).unwrap();

    let text = buffer_string(&terminal);

    assert!(text.contains("dev-cli"));
    assert!(text.contains("Dashboard"));
    assert!(text.contains("Projects"));
    assert!(text.contains("Search"));
}

#[test]
fn dashboard_renders_footer() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|frame| render(frame, &AppState::new())).unwrap();

    let text = buffer_string(&terminal);

    assert!(text.contains("Enter"));
    assert!(text.contains("Quit"));
}

#[test]
fn dashboard_renders_on_small_terminal() {
    let backend = TestBackend::new(40, 12);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|frame| render(frame, &AppState::new())).unwrap();

    let text = buffer_string(&terminal);

    assert!(text.contains("Dashboard"));
}
