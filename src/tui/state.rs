//! Application state for the dashboard.

use crate::models::project::Project;

#[derive(Debug, Default)]
pub struct AppState {
    /// Whether the dashboard should exit.
    pub should_quit: bool,

    /// All discovered projects.
    pub projects: Vec<Project>,

    /// Current search text.
    pub search_query: String,

    /// Currently selected project in the filtered list.
    pub selected_index: usize,
}

impl AppState {
    /// Create an empty dashboard state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Quit the dashboard.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Replace the project list.
    pub fn set_projects(&mut self, projects: Vec<Project>) {
        self.projects = projects;
        self.selected_index = 0;
    }

    /// Append a typed character.
    pub fn push_char(&mut self, c: char) {
        self.search_query.push(c);
        self.selected_index = 0;
    }

    /// Remove the previous character.
    pub fn backspace(&mut self) {
        self.search_query.pop();
        self.selected_index = 0;
    }

    /// Move selection down.
    pub fn select_next(&mut self) {
        let max = self.filtered_projects().len();

        if max > 0 {
            self.selected_index = (self.selected_index + 1).min(max - 1);
        }
    }

    /// Move selection up.
    pub fn select_previous(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    /// Projects matching the current search query.
    pub fn filtered_projects(&self) -> Vec<&Project> {
        if self.search_query.is_empty() {
            return self.projects.iter().collect();
        }

        let query = self.search_query.to_lowercase();

        self.projects
            .iter()
            .filter(|project| {
                project.name.to_lowercase().contains(&query)
                    || project.path.to_string_lossy().to_lowercase().contains(&query)
            })
            .collect()
    }

    /// Currently selected project.
    pub fn selected_project(&self) -> Option<&Project> {
        self.filtered_projects().get(self.selected_index).copied()
    }
}
