//! Application state for the dashboard.

use crate::models::project::Project;

#[derive(Debug, Default)]
pub struct AppState {
    /// All discovered projects.
    pub projects: Vec<Project>,

    /// Current search query.
    pub search_query: String,

    /// Currently selected row in the filtered list.
    pub selected_index: usize,

    /// Whether the dashboard should exit.
    pub should_quit: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
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
        let len = self.filtered_projects().len();

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
        let len = self.filtered_projects().len();

        if len == 0 {
            self.selected_index = 0;
        } else if self.selected_index >= len {
            self.selected_index = len - 1;
        }
    }

    pub fn selected_project(&self) -> Option<&Project> {
        self.filtered_projects().get(self.selected_index).copied()
    }
}
