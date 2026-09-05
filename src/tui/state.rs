//! Application state for the dashboard.

#[derive(Debug, Default)]
pub struct AppState {
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
}
