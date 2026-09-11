//! Application state for the dashboard.

use crate::{
    config::Config,
    models::{project::Project, recent_project::RecentProject},
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

    pub fn move_down(&mut self) {
        let len = match self.active_tab {
            Tab::Projects => self.filtered_projects().len(),
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
            Tab::Projects => self.filtered_projects().len(),
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
        self.filtered_projects().get(self.selected_index).copied()
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
