//! Application state for the dashboard.

use crate::{
    config::Config,
    models::{project::Project, recent_project::RecentProject},
    tui::tree::{DisplayNode, TreeNode, build_project_tree, flatten_filtered_tree, flatten_tree},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Different views available in the application.
pub enum Tab {
    Recent,
    Projects,
    Ide,
    Settings,
}

#[derive(Debug)]
/// Core state for the Ratatui application.
pub struct AppState {
    /// All discovered projects.
    pub projects: Vec<Project>,

    /// The structured tree of projects.
    pub project_tree: Vec<TreeNode>,

    /// Recently opened projects.
    pub recent_projects: Vec<RecentProject>,

    /// Current search query.
    pub search_query: String,

    /// Currently selected row in the active list.
    pub selected_index: usize,

    /// Whether the dashboard should exit.
    pub should_quit: bool,

    /// Currently active tab.
    pub active_tab: Tab,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
            project_tree: Vec::new(),
            recent_projects: Vec::new(),
            search_query: String::new(),
            selected_index: 0,
            should_quit: false,
            active_tab: Tab::Projects,
        }
    }
}

impl AppState {
    /// Creates the runtime application state.
    /// The dashboard opens on the Recent tab.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_projects(&mut self, projects: Vec<Project>) {
        self.projects = projects.clone();
        self.project_tree = build_project_tree(projects);
    }

    /// Load recent projects from the user's configuration.
    pub fn load_recent_projects(&mut self) {
        self.recent_projects =
            Config::load().map(|config| config.recent_projects).unwrap_or_default();
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn filtered_projects(&self) -> Vec<&Project> {
        if self.search_query.is_empty() {
            return self.projects.iter().collect();
        }

        let query = self.search_query.to_lowercase();

        self.projects
            .iter()
            .filter(|project| project.name.to_lowercase().contains(&query))
            .collect()
    }

    pub fn visible_items(&self) -> Vec<DisplayNode<'_>> {
        let mut out = Vec::new();
        if self.search_query.is_empty() {
            flatten_tree(&self.project_tree, 0, &mut out);
        } else {
            let query = self.search_query.to_lowercase();
            flatten_filtered_tree(&self.project_tree, &query, 0, &mut out);
        }
        out
    }

    pub fn toggle_selected(&mut self) {
        if self.active_tab != Tab::Projects {
            return;
        }

        if let Some(node) = self.visible_items().get(self.selected_index).filter(|n| n.is_folder) {
            let path = node.path.to_path_buf();
            Self::toggle_node(&mut self.project_tree, &path);
        }
    }

    fn toggle_node(nodes: &mut [TreeNode], target: &std::path::Path) -> bool {
        for node in nodes {
            if let TreeNode::Folder { path, is_expanded, children, .. } = node {
                if path == target {
                    *is_expanded = !*is_expanded;
                    return true;
                }
                if Self::toggle_node(children, target) {
                    return true;
                }
            }
        }
        false
    }

    pub fn move_down(&mut self) {
        let len = match self.active_tab {
            Tab::Projects => self.visible_items().len(),
            Tab::Recent => self.recent_projects.len(),
            _ => 0,
        };

        if len == 0 {
            self.selected_index = 0;
            return;
        }

        if self.selected_index + 1 < len {
            self.selected_index += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn push_char(&mut self, c: char) {
        self.search_query.push(c);
        self.selected_index = 0;
    }

    pub fn pop_char(&mut self) {
        self.search_query.pop();
        self.selected_index = 0;
    }

    pub fn clamp_selection(&mut self) {
        let len = match self.active_tab {
            Tab::Projects => self.visible_items().len(),
            Tab::Recent => self.recent_projects.len(),
            _ => 0,
        };

        if len == 0 {
            self.selected_index = 0;
        } else if self.selected_index >= len {
            self.selected_index = len - 1;
        }
    }

    pub fn selected_project(&self) -> Option<&Project> {
        self.visible_items().get(self.selected_index).and_then(|node| node.project)
    }

    pub fn selected_recent_project(&self) -> Option<&RecentProject> {
        self.recent_projects.get(self.selected_index)
    }

    pub fn scroll_offset(&self, visible_rows: usize) -> usize {
        if visible_rows == 0 {
            return 0;
        }

        self.selected_index.saturating_sub(visible_rows.saturating_sub(1))
    }
}
