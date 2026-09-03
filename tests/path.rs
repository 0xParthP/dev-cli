use dev_cli::utils::path::display_path;

use std::path::PathBuf;

#[test]
fn display_path_returns_string() {
    let path = PathBuf::from("C:/Projects/demo");

    let output = display_path(&path);

    assert!(output.contains("Projects"));
}

#[test]
fn display_path_handles_spaces() {
    let path = PathBuf::from("/tmp/My Projects/demo");

    let formatted = dev_cli::utils::path::display_path(&path);

    assert!(formatted.contains("My Projects"));
}

#[test]
fn display_path_handles_empty_path() {
    let path = std::path::PathBuf::new();
    let formatted = dev_cli::utils::path::display_path(&path);

    assert_eq!(formatted, "");
}
