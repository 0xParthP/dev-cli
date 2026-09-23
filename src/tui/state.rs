//! Application state for the dashboard.

use std::{collections::HashMap, path::PathBuf};

use crate::{
    config::Config,
    models::{ide::Ide, project::Project, recent_project::RecentProject},
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

    /// Configured default IDE.
    pub default_ide: Ide,

    /// Installed IDEs available on the system.
    pub installed_ides: Vec<Ide>,

    /// Per-project IDE selection overrides.
    pub ide_overrides: HashMap<PathBuf, Ide>,

    /// Project and IDE pending launch after TUI exit.
    pub pending_launch: Option<(Ide, Project)>,
}

impl Default for AppState {
    fn default() -> Self {
        let default_ide = Config::load().map(|config| config.default_ide).unwrap_or(Ide::Vscode);

        let detected: Vec<Ide> =
            crate::ide::detect::detect_ides().into_iter().map(|i| i.ide).collect();

        let installed_ides = if detected.is_empty() {
            vec![
                Ide::Cursor,
                Ide::Vscode,
                Ide::Claude,
                Ide::Terminal,
                Ide::Idea,
                Ide::Rider,
                Ide::Zed,
            ]
        } else {
            detected
        };

        Self {
            projects: Vec::new(),
            project_tree: Vec::new(),
            recent_projects: Vec::new(),
            search_query: String::new(),
            selected_index: 0,
            should_quit: false,
            active_tab: Tab::Projects,
            default_ide,
            installed_ides,
            ide_overrides: HashMap::new(),
            pending_launch: None,
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

    /// Rescan projects from disk, reload recent history, and clamp current selection.
    pub fn refresh(&mut self) -> anyhow::Result<()> {
        let projects = crate::tui::data::load_projects()?;
        self.set_projects(projects);
        self.load_recent_projects();
        self.clamp_selection();
        Ok(())
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Get the target IDE for a project path (override or default).
    pub fn get_project_ide(&self, path: &std::path::Path) -> Ide {
        let ide = self.ide_overrides.get(path).copied().unwrap_or(self.default_ide);
        if !self.installed_ides.is_empty() && !self.installed_ides.contains(&ide) {
            self.installed_ides[0]
        } else {
            ide
        }
    }

    /// Cycle through available installed IDEs for the currently selected project.
    pub fn cycle_selected_ide(&mut self) {
        if self.active_tab != Tab::Projects || self.installed_ides.is_empty() {
            return;
        }

        if let Some(project) = self.selected_project() {
            let current_ide = self.get_project_ide(&project.path);
            let next_ide = match self.installed_ides.iter().position(|&i| i == current_ide) {
                Some(idx) => self.installed_ides[(idx + 1) % self.installed_ides.len()],
                None => self.installed_ides[0],
            };
            self.ide_overrides.insert(project.path.clone(), next_ide);
        }
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
