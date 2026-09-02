mod common;

use common::temp_project::TempProject;
use dev_cli::scanner::discover_projects;
use std::path::PathBuf;

#[test]
fn discovers_repository() {
    let workspace = TempProject::new("workspace");
    workspace.create_git_repo("portfolio");

    let projects = discover_projects(&[workspace.root()]).unwrap();

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "portfolio");
}

#[test]
fn discovers_multiple_repositories() {
    let workspace = TempProject::new("workspace");

    workspace.create_git_repo("portfolio");
    workspace.create_git_repo("omniroute");

    let mut names: Vec<String> =
        discover_projects(&[workspace.root()]).unwrap().into_iter().map(|p| p.name).collect();

    names.sort();

    assert_eq!(names, vec!["omniroute", "portfolio"]);
}

#[test]
fn duplicate_roots_are_removed() {
    let workspace = TempProject::new("workspace");
    workspace.create_git_repo("portfolio");

    let roots: Vec<PathBuf> = vec![workspace.root(), workspace.root()];

    let projects = discover_projects(&roots).unwrap();

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "portfolio");
}
