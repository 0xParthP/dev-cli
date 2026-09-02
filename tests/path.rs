use dev_cli::utils::path::display_path;

use std::path::PathBuf;

#[test]
fn display_path_returns_string() {
    let path = PathBuf::from("C:/Projects/demo");

    let output = display_path(&path);

    assert!(output.contains("Projects"));
}
