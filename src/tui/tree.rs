use crate::models::project::Project;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeNode {
    Folder { name: String, path: PathBuf, is_expanded: bool, children: Vec<TreeNode> },
    Project(Project),
}

impl TreeNode {
    pub fn name(&self) -> &str {
        match self {
            TreeNode::Folder { name, .. } => name,
            TreeNode::Project(p) => &p.name,
        }
    }

    pub fn path(&self) -> &Path {
        match self {
            TreeNode::Folder { path, .. } => path,
            TreeNode::Project(p) => &p.path,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayNode<'a> {
    pub depth: usize,
    pub is_folder: bool,
    pub is_expanded: bool,
    pub name: &'a str,
    pub path: &'a Path,
    pub project: Option<&'a Project>,
}

use crate::utils::path::normalize_path;

pub fn build_project_tree(projects: Vec<Project>) -> Vec<TreeNode> {
    // Group projects by their normalized configured root.
    let mut root_groups: BTreeMap<PathBuf, Vec<Project>> = BTreeMap::new();
    for p in projects {
        let clean_root = normalize_path(p.root.clone());
        root_groups.entry(clean_root).or_default().push(p);
    }

    let mut tree = Vec::new();
    for (root_path, projs) in root_groups {
        let root_name = root_path.file_name().unwrap_or_default().to_string_lossy().into_owned();

        let mut root_children = build_subtree(&root_path, projs);
        root_children.sort_by(|a, b| a.name().cmp(b.name()));

        tree.push(TreeNode::Folder {
            name: root_name,
            path: root_path,
            is_expanded: true, // Default to expanded
            children: root_children,
        });
    }

    tree
}

fn build_subtree(base_path: &Path, projects: Vec<Project>) -> Vec<TreeNode> {
    let base_path_norm = normalize_path(base_path.to_path_buf());
    let mut tree = Vec::new();
    let mut folder_groups: BTreeMap<String, Vec<Project>> = BTreeMap::new();

    for p in projects {
        let p_path_norm = normalize_path(p.path.clone());

        if let Ok(rel) = p_path_norm.strip_prefix(&base_path_norm) {
            let components: Vec<_> = rel.components().collect();
            if components.len() > 1 {
                // If there are 2 or more components, the first component is an intermediate folder
                let folder_name = components[0].as_os_str().to_string_lossy().into_owned();
                folder_groups.entry(folder_name).or_default().push(p);
            } else {
                // It's a direct child project of base_path
                tree.push(TreeNode::Project(p));
            }
        } else {
            // Fallback if prefix stripping fails
            tree.push(TreeNode::Project(p));
        }
    }

    for (folder_name, projs) in folder_groups {
        let folder_path = base_path_norm.join(&folder_name);
        let mut children = build_subtree(&folder_path, projs);
        children.sort_by(|a, b| a.name().cmp(b.name()));

        tree.push(TreeNode::Folder {
            name: folder_name,
            path: folder_path,
            is_expanded: true,
            children,
        });
    }

    tree.sort_by(|a, b| a.name().cmp(b.name()));
    tree
}

/// Flattens a tree into a list of DisplayNodes, respecting expansion state.
pub fn flatten_tree<'a>(nodes: &'a [TreeNode], depth: usize, out: &mut Vec<DisplayNode<'a>>) {
    flatten_filtered_tree(nodes, "", depth, out);
}

/// Flattens a tree into a list of DisplayNodes, including only projects matching query (and their parent folders).
pub fn flatten_filtered_tree<'a>(
    nodes: &'a [TreeNode],
    query: &str,
    depth: usize,
    out: &mut Vec<DisplayNode<'a>>,
) -> bool {
    let mut has_matching_child = false;

    for node in nodes {
        match node {
            TreeNode::Folder { name, path, is_expanded, children } => {
                let start_idx = out.len();
                out.push(DisplayNode {
                    depth,
                    is_folder: true,
                    is_expanded: *is_expanded,
                    name,
                    path,
                    project: None,
                });

                if *is_expanded {
                    let child_has_match = flatten_filtered_tree(children, query, depth + 1, out);
                    if child_has_match {
                        has_matching_child = true;
                    } else {
                        out.truncate(start_idx);
                    }
                } else {
                    let has_match = has_matching_project(children, query);
                    if has_match {
                        has_matching_child = true;
                    } else {
                        out.truncate(start_idx);
                    }
                }
            }
            TreeNode::Project(p) => {
                if p.name.to_lowercase().contains(query) {
                    has_matching_child = true;
                    out.push(DisplayNode {
                        depth,
                        is_folder: false,
                        is_expanded: false,
                        name: &p.name,
                        path: &p.path,
                        project: Some(p),
                    });
                }
            }
        }
    }

    has_matching_child
}

/// Returns true if any project within nodes matches the search query.
pub fn has_matching_project(nodes: &[TreeNode], query: &str) -> bool {
    for node in nodes {
        match node {
            TreeNode::Folder { children, .. } => {
                if has_matching_project(children, query) {
                    return true;
                }
            }
            TreeNode::Project(p) => {
                if p.name.to_lowercase().contains(query) {
                    return true;
                }
            }
        }
    }
    false
}
