use dev_cli::{
    models::project::Project,
    tui::tree::{TreeNode, build_project_tree, flatten_filtered_tree, flatten_tree},
};

#[test]
fn test_build_project_tree_subdirectories() {
    let root = std::env::temp_dir().join("projects");

    let p1 = Project::new(root.join("Project 1"), root.clone());
    let p2 = Project::new(root.join("SubFolder").join("Project 2"), root.clone());
    let p3 = Project::new(root.join("Project 3"), root.clone());
    let p4 = Project::new(root.join("SubFolder").join("Project 4"), root.clone());

    let tree = build_project_tree(vec![p1, p2, p3, p4]);

    assert_eq!(tree.len(), 1); // "projects" folder
    if let TreeNode::Folder { name, children, .. } = &tree[0] {
        assert_eq!(name, "projects");
        assert_eq!(children.len(), 3); // "Project 1", "Project 3", and "SubFolder"

        let child_names: Vec<&str> = children.iter().map(|c| c.name()).collect();
        assert_eq!(child_names, vec!["Project 1", "Project 3", "SubFolder"]);

        if let TreeNode::Folder { name: sub_name, children: sub_children, .. } = &children[2] {
            assert_eq!(sub_name, "SubFolder");
            assert_eq!(sub_children.len(), 2);
            let sub_names: Vec<&str> = sub_children.iter().map(|c| c.name()).collect();
            assert_eq!(sub_names, vec!["Project 2", "Project 4"]);
        } else {
            panic!("Expected folder for SubFolder");
        }
    } else {
        panic!("Expected root folder");
    }
}

#[test]
fn test_flatten_tree_depths() {
    let root = std::env::temp_dir().join("projects");

    let p1 = Project::new(root.join("Project 1"), root.clone());
    let p2 = Project::new(root.join("SubFolder").join("Project 2"), root.clone());

    let tree = build_project_tree(vec![p1, p2]);
    let mut display_nodes = Vec::new();
    flatten_tree(&tree, 0, &mut display_nodes);

    assert_eq!(display_nodes.len(), 4);
    assert_eq!(display_nodes[0].name, "projects");
    assert_eq!(display_nodes[0].depth, 0);
    assert!(display_nodes[0].is_folder);

    assert_eq!(display_nodes[1].name, "Project 1");
    assert_eq!(display_nodes[1].depth, 1);
    assert!(!display_nodes[1].is_folder);

    assert_eq!(display_nodes[2].name, "SubFolder");
    assert_eq!(display_nodes[2].depth, 1);
    assert!(display_nodes[2].is_folder);

    assert_eq!(display_nodes[3].name, "Project 2");
    assert_eq!(display_nodes[3].depth, 2);
    assert!(!display_nodes[3].is_folder);
}

#[test]
fn test_flatten_filtered_tree_preserves_hierarchy() {
    let root = std::env::temp_dir().join("projects");

    let p1 = Project::new(root.join("App 1"), root.clone());
    let p2 = Project::new(root.join("SubFolder").join("App 2"), root.clone());
    let p3 = Project::new(root.join("Other Project"), root.clone());

    let tree = build_project_tree(vec![p1, p2, p3]);

    let mut display_nodes = Vec::new();
    flatten_filtered_tree(&tree, "app", 0, &mut display_nodes);

    // Should include projects (depth 0, folder), App 1 (depth 1, project), SubFolder (depth 1, folder), App 2 (depth 2, project)
    // "Other Project" should be excluded!
    let names: Vec<&str> = display_nodes.iter().map(|n| n.name).collect();
    assert_eq!(names, vec!["projects", "App 1", "SubFolder", "App 2"]);

    let depths: Vec<usize> = display_nodes.iter().map(|n| n.depth).collect();
    assert_eq!(depths, vec![0, 1, 1, 2]);
}
