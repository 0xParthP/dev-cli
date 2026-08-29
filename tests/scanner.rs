mod common;

use common::temp_project::{create_git_repo, create_workspace};

use dev_cli::scanner::discover_projects;

#[test]
fn empty_directory_returns_no_projects() {
    let dir = create_workspace();

    let projects = discover_projects(&[dir.path().into()]);

    assert!(projects.is_empty());
}

#[test]
fn placeholder_scanner_returns_empty_even_when_git_exists() {
    let (dir, _) = create_git_repo("MyRepo");

    let projects = discover_projects(&[dir.path().into()]);

    assert!(projects.is_empty());
}
