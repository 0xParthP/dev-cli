use dev_cli::tui::widgets::{footer, header, project_list, search};
use ratatui::{Terminal, backend::TestBackend, layout::Rect};

fn render_widget<F>(renderer: F) -> String
where
    F: FnOnce(&mut ratatui::Frame, Rect),
{
    let backend = TestBackend::new(60, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            renderer(frame, frame.area());
        })
        .unwrap();

    terminal.backend().buffer().content().iter().map(|c| c.symbol()).collect()
}

#[test]
fn header_contains_branding() {
    let text = render_widget(header::render);

    assert!(text.contains("dev-cli"));
    assert!(text.contains("Dashboard"));
}

#[test]
fn search_widget_contains_placeholder() {
    let text = render_widget(search::render);

    assert!(text.contains("Search"));
    assert!(text.contains("Phase"));
}

#[test]
fn project_widget_contains_placeholder() {
    let text = render_widget(project_list::render);

    assert!(text.contains("Projects"));
    assert!(text.contains("Use q"));
}

#[test]
fn footer_contains_shortcuts() {
    let text = render_widget(footer::render);

    assert!(text.contains("Enter"));
    assert!(text.contains("Quit"));
}
