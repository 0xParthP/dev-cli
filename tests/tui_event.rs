use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use serial_test::serial;

use dev_cli::{
    config::Config,
    models::ide::Ide,
    tui::{
        actions,
        event::{handle_events_with, handle_key, handle_key_with_launcher},
        state::{AppState, Tab},
    },
};

mod common;
use common::factories::fake_project as project;

use temp_env::with_var;
use tempfile::TempDir;

fn with_temp_config<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let dir = TempDir::new().unwrap();

    with_var("DEVCLI_CONFIG_DIR", Some(dir.path()), f)
}

fn key(code: KeyCode) -> Event {
    Event::Key(KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    })
}

#[test]
fn does_nothing_when_no_event_available() -> Result<()> {
    let mut state = AppState::new();

    handle_events_with(&mut state, |_| Ok(false), || unreachable!())?;

    assert!(!state.should_quit);
    Ok(())
}

#[test]
fn quits_on_escape() -> Result<()> {
    let mut state = AppState::new();

    handle_events_with(&mut state, |_| Ok(true), || Ok(key(KeyCode::Esc)))?;

    assert!(state.should_quit);
    Ok(())
}

#[test]
fn ignores_other_keys() -> Result<()> {
    let mut state = AppState::new();

    handle_events_with(&mut state, |_| Ok(true), || Ok(key(KeyCode::Enter)))?;

    assert!(!state.should_quit);
    Ok(())
}

#[test]
fn ignores_resize_events() -> Result<()> {
    let mut state = AppState::new();

    handle_events_with(&mut state, |_| Ok(true), || Ok(Event::Resize(100, 40)))?;

    assert!(!state.should_quit);
    Ok(())
}

#[test]
fn esc_key_quits() {
    let mut state = AppState::new();

    handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE), &mut state);

    assert!(state.should_quit);
}

#[test]
fn other_keys_do_nothing() {
    let mut state = AppState::new();

    handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE), &mut state);

    assert!(!state.should_quit);
}

/// ===== Tests for milestone 4.2.3 =====

#[test]
fn down_key_moves_selection() {
    let mut state = AppState::new();
    state.projects = vec![project("alpha"), project("beta")];

    handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE), &mut state);

    assert_eq!(state.selected_index, 1);
}

#[test]
fn up_key_moves_selection() {
    let mut state = AppState::new();
    state.projects = vec![project("alpha"), project("beta")];
    state.selected_index = 1;

    handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE), &mut state);

    assert_eq!(state.selected_index, 0);
}

#[test]
fn typing_adds_character() {
    let mut state = AppState::new();

    handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE), &mut state);
    handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE), &mut state);

    assert_eq!(state.search_query, "ab");
}

#[test]
fn backspace_removes_character() {
    let mut state = AppState::new();
    state.search_query = "cursor".into();

    handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE), &mut state);

    assert_eq!(state.search_query, "curso");
}

#[test]
fn enter_key_does_nothing_without_projects() {
    let mut state = AppState::default();

    handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE), &mut state);

    assert!(!state.should_quit);
}

#[test]
fn open_project_calls_launcher() -> Result<()> {
    with_temp_config(|| -> Result<()> {
        // Give the test its own isolated config.
        Config {
            projects_root: vec![std::env::temp_dir()],
            default_ide: Ide::Vscode,
            recent_projects: Vec::new(),
        }
        .save()?;

        let project = project("demo");

        let mut launched = None;

        actions::open_project_with(&project, |ide, path| {
            launched = Some((ide, path.to_path_buf()));
            Ok(())
        })?;

        let (ide, path) = launched.expect("launcher should be called");

        assert_eq!(ide, Ide::Vscode);
        assert!(path.ends_with("demo"));

        Ok(())
    })
}

#[test]
fn ignores_mouse_events() -> Result<()> {
    let mut state = AppState::new();

    handle_events_with(
        &mut state,
        |_| Ok(true),
        || {
            Ok(Event::Mouse(crossterm::event::MouseEvent {
                kind: crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left),
                column: 10,
                row: 5,
                modifiers: KeyModifiers::NONE,
            }))
        },
    )?;

    assert!(!state.should_quit);

    Ok(())
}

#[test]
fn open_project_returns_launcher_error() -> Result<()> {
    let project = project("demo");

    let result = dev_cli::tui::actions::open_project_with(&project, |_ide, _path| {
        Err(anyhow::anyhow!("launch failed"))
    });

    assert!(result.is_err());

    Ok(())
}

#[test]
fn enter_does_nothing_when_filtered_list_is_empty() {
    let mut state = AppState {
        projects: vec![project("cursor")],
        search_query: "weather".into(),
        ..Default::default()
    };

    handle_key_with_launcher(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE), &mut state, |_| {
        panic!("launcher should not be called");
    });

    assert!(!state.should_quit);
}

#[test]
fn enter_does_not_quit_when_open_project_fails() {
    let mut state = AppState { projects: vec![project("demo")], ..Default::default() };

    handle_key_with_launcher(
        KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        &mut state,
        |_project| Err(anyhow::anyhow!("launch failed")),
    );

    assert!(!state.should_quit);
}

#[test]
fn poll_false_does_not_read_event() -> Result<()> {
    let mut state = AppState::new();

    handle_events_with(&mut state, |_| Ok(false), || panic!("read should not be called"))?;

    assert!(!state.should_quit);

    Ok(())
}

#[test]
fn right_arrow_switches_tabs() {
    let mut state = AppState::new();
    state.active_tab = Tab::Recent;

    handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE), &mut state);
    assert_eq!(state.active_tab, Tab::Projects);

    handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE), &mut state);
    assert_eq!(state.active_tab, Tab::Ide);

    handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE), &mut state);
    assert_eq!(state.active_tab, Tab::Settings);
}

#[test]
fn left_arrow_switches_tabs() {
    let mut state = AppState::new();
    state.active_tab = Tab::Settings;

    handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE), &mut state);
    assert_eq!(state.active_tab, Tab::Ide);

    handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE), &mut state);
    assert_eq!(state.active_tab, Tab::Projects);

    handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE), &mut state);
    assert_eq!(state.active_tab, Tab::Recent);
}

#[test]
fn left_arrow_does_not_go_before_projects() {
    let mut state = AppState::new();

    handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE), &mut state);

    assert_eq!(state.active_tab, Tab::Recent);
}

#[test]
fn right_arrow_does_not_go_past_settings() {
    let mut state = AppState::new();
    state.active_tab = Tab::Settings;

    handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE), &mut state);

    assert_eq!(state.active_tab, Tab::Settings);
}

#[test]
fn enter_key_on_recent_tab_launches_project() {
    let mut state = AppState::new();
    state.active_tab = Tab::Recent;
    state.recent_projects = vec![dev_cli::models::recent_project::RecentProject {
        name: "recent_app".into(),
        path: std::path::PathBuf::from("/tmp/recent_app"),
        last_opened: 100,
    }];

    let launched = std::sync::atomic::AtomicBool::new(false);
    handle_key_with_launcher(
        KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        &mut state,
        |project| {
            launched.store(true, std::sync::atomic::Ordering::Relaxed);
            assert_eq!(project.name, "recent_app");
            Ok(())
        },
    );

    assert!(launched.load(std::sync::atomic::Ordering::Relaxed));
    assert!(state.should_quit);
}

#[test]
fn enter_key_on_recent_tab_does_not_quit_on_error() {
    let mut state = AppState::new();
    state.active_tab = Tab::Recent;
    state.recent_projects = vec![dev_cli::models::recent_project::RecentProject {
        name: "recent_app".into(),
        path: std::path::PathBuf::from("/tmp/recent_app"),
        last_opened: 100,
    }];

    handle_key_with_launcher(
        KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        &mut state,
        |_project| Err(anyhow::anyhow!("failed to launch")),
    );

    assert!(!state.should_quit);
}

#[test]
#[serial]
fn enter_key_on_recent_tab_reupdates_recent_project_in_config() {
    with_temp_config(|| {
        let mut config = Config::default();
        let old_project = project("old_app");
        let target_project = project("target_app");
        config.add_recent_project(&target_project);
        config.add_recent_project(&old_project);
        config.save().unwrap();

        let mut state = AppState::new();
        state.active_tab = Tab::Recent;
        state.load_recent_projects();
        assert_eq!(state.recent_projects[0].name, "old_app");
        assert_eq!(state.recent_projects[1].name, "target_app");

        state.selected_index = 1;

        handle_key_with_launcher(
            KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
            &mut state,
            |p| actions::open_project_with(p, |_, _| Ok(())),
        );

        let updated_config = Config::load().unwrap();
        assert_eq!(updated_config.recent_projects[0].name, "target_app");
    });
}
