//! Application state for the dashboard.

use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
};

use crate::{
    config::Config,
    ide::detect::verify_executable,
    models::{custom_ide::CustomIde, ide::Ide, project::Project, recent_project::RecentProject},
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

/// Keyboard input routing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Normal mode — arrow keys navigate, chars go to search.
    Normal,
    /// Editing a text field (IDE tab or Settings tab form).
    Editing,
}

/// Which element has focus on the IDE tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdeTabFocus {
    /// Browsing the list of IDEs.
    List,
    /// Filling in the "Add IDE" form.
    AddForm,
    /// Confirming deletion of a custom IDE.
    ConfirmDelete,
}

/// Which element has focus on the Settings tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTabFocus {
    /// Browsing settings list.
    List,
    /// Adding a new project root.
    AddRoot,
}

/// Which element has focus on the Projects tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectsTabFocus {
    /// Browsing the project list.
    List,
    /// Filling in the Clone form.
    CloneForm,
}

/// An entry in the IDE tab list — either a built-in detected IDE or a custom one.
#[derive(Debug, Clone)]
pub enum IdeListEntry {
    BuiltIn { ide: Ide, is_default: bool },
    Custom { custom: CustomIde, is_default: bool },
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

    /// Character cursor position in search query.
    pub search_cursor: usize,

    /// Currently selected row in the active list.
    pub selected_index: usize,

    /// Whether the dashboard should exit.
    pub should_quit: bool,

    /// Currently active tab.
    pub active_tab: Tab,

    /// Configured default IDE.
    pub default_ide: crate::models::ide::IdeSelection,

    /// Installed IDEs available on the system.
    pub installed_ides: Vec<Ide>,

    /// Per-project IDE selection overrides.
    pub ide_overrides: HashMap<PathBuf, crate::models::ide::IdeSelection>,

    /// Project and IDE pending launch after TUI exit.
    pub pending_launch: Option<(crate::models::ide::IdeSelection, Project)>,

    // --- Input mode ---
    /// Current input routing mode.
    pub input_mode: InputMode,

    // --- IDE tab state ---
    /// IDE tab focus area.
    pub ide_tab_focus: IdeTabFocus,

    /// User-configured custom IDEs loaded from config.
    pub custom_ides: Vec<CustomIde>,

    /// IDE add-form: display name.
    pub ide_form_name: String,
    pub ide_form_name_cursor: usize,

    /// IDE add-form: executable path.
    pub ide_form_path: String,
    pub ide_form_path_cursor: usize,

    /// IDE add-form: argument template.
    pub ide_form_args: String,
    pub ide_form_args_cursor: usize,

    /// IDE add-form: which field is active (0=name, 1=path, 2=args).
    pub ide_form_field: usize,

    /// IDE tab status message (e.g. "✓ Verified" or "✗ Not found").
    pub ide_status_message: Option<String>,

    // --- Settings tab state ---
    /// Settings tab focus area.
    pub settings_tab_focus: SettingsTabFocus,

    /// Settings tab: new root path input.
    pub settings_add_root: String,
    pub settings_add_root_cursor: usize,

    /// Project root directories (loaded from config).
    pub project_roots: Vec<PathBuf>,

    /// Settings tab status message.
    pub settings_status_message: Option<String>,

    // --- Clone form state (Projects tab) ---
    pub projects_tab_focus: ProjectsTabFocus,
    pub clone_url: String,
    pub clone_url_cursor: usize,
    pub clone_root_index: usize,
    pub clone_name: String,
    pub clone_name_cursor: usize,
    pub clone_field: usize,
    pub clone_status_message: Option<String>,
    /// Streaming log lines received from the background git process.
    pub clone_log_lines: VecDeque<String>,
    /// Background clone task channels (None when idle).
    pub clone_task: Option<crate::commands::clone::CloneChannels>,

    /// Maximum search depth for repository scanner.
    pub max_depth: usize,

    /// Extra directory names ignored during scanning.
    pub ignore_patterns: Vec<String>,

    /// Active TUI theme palette name.
    pub theme: String,

    /// Maximum recent projects to retain.
    pub recent_projects_limit: usize,

    /// Auto refresh projects on launch.
    pub auto_refresh_on_launch: bool,
}

impl Default for AppState {
    fn default() -> Self {
        let config = Config::load().unwrap_or_default();
        let default_ide = config.default_ide;
        let mut custom_ides = config.custom_ides.clone();
        custom_ides.sort_by_key(|a| a.display_name.to_lowercase());
        let project_roots = config.projects_root.clone();

        let detected: Vec<Ide> =
            crate::ide::detect::detect_ides().into_iter().map(|i| i.ide).collect();

        let mut installed_ides = if detected.is_empty() {
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
        installed_ides.sort_by_key(|a| a.display_name().to_lowercase());

        Self {
            projects: Vec::new(),
            project_tree: Vec::new(),
            recent_projects: Vec::new(),
            search_query: String::new(),
            search_cursor: 0,
            selected_index: 0,
            should_quit: false,
            active_tab: Tab::Projects,
            default_ide,
            installed_ides,
            ide_overrides: HashMap::new(),
            pending_launch: None,

            input_mode: InputMode::Normal,

            ide_tab_focus: IdeTabFocus::List,
            custom_ides,
            ide_form_name: String::new(),
            ide_form_name_cursor: 0,
            ide_form_path: String::new(),
            ide_form_path_cursor: 0,
            ide_form_args: String::new(),
            ide_form_args_cursor: 0,
            ide_form_field: 0,
            ide_status_message: None,

            settings_tab_focus: SettingsTabFocus::List,
            settings_add_root: String::new(),
            settings_add_root_cursor: 0,
            project_roots,
            settings_status_message: None,

            projects_tab_focus: ProjectsTabFocus::List,
            clone_url: String::new(),
            clone_url_cursor: 0,
            clone_root_index: 0,
            clone_name: String::new(),
            clone_name_cursor: 0,
            clone_field: 0,
            clone_status_message: None,
            clone_log_lines: VecDeque::new(),
            clone_task: None,

            max_depth: config.max_depth,
            ignore_patterns: config.ignore_patterns,
            theme: config.theme,
            recent_projects_limit: config.recent_projects_limit,
            auto_refresh_on_launch: config.auto_refresh_on_launch,
        }
    }
}

impl AppState {
    /// Creates the runtime application state.
    /// The dashboard opens on the Recent tab.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the active theme palette.
    pub fn palette(&self) -> crate::tui::theme::Palette {
        crate::tui::theme::get_palette(&self.theme)
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
        self.reload_config_state();
        self.clamp_selection();
        Ok(())
    }

    /// Reload config-derived state (custom IDEs, project roots, default IDE, settings).
    pub fn reload_config_state(&mut self) {
        if let Ok(config) = Config::load() {
            let mut custom_ides = config.custom_ides;
            custom_ides.sort_by_key(|a| a.display_name.to_lowercase());
            self.custom_ides = custom_ides;
            self.project_roots = config.projects_root;
            self.default_ide = config.default_ide;
            self.max_depth = config.max_depth;
            self.ignore_patterns = config.ignore_patterns;
            self.theme = config.theme;
            self.recent_projects_limit = config.recent_projects_limit;
            self.auto_refresh_on_launch = config.auto_refresh_on_launch;
        }
    }

    pub fn cycle_theme(&mut self) {
        let themes = ["neon", "cyberpunk", "catppuccin", "monokai", "high-contrast"];
        let current_idx =
            themes.iter().position(|&t| t.eq_ignore_ascii_case(&self.theme)).unwrap_or(0);
        let next_idx = (current_idx + 1) % themes.len();
        self.theme = themes[next_idx].to_string();

        if let Ok(mut config) = Config::load() {
            config.theme = self.theme.clone();
            let _ = config.save();
        }
        let palette_name = crate::tui::theme::get_palette(&self.theme).name;
        self.settings_status_message = Some(format!("✓ Theme set to {}", palette_name));
    }

    pub fn cycle_max_depth(&mut self) {
        let depths = [2, 3, 4, 5, 6, 8];
        let current_idx = depths.iter().position(|&d| d == self.max_depth).unwrap_or(2);
        let next_idx = (current_idx + 1) % depths.len();
        self.max_depth = depths[next_idx];

        if let Ok(mut config) = Config::load() {
            config.max_depth = self.max_depth;
            let _ = config.save();
        }
        let _ = self.refresh();
        self.settings_status_message = Some(format!("✓ Max scan depth set to {}", self.max_depth));
    }

    pub fn cycle_recent_limit(&mut self) {
        let limits = [5, 10, 15, 20, 25];
        let current_idx = limits.iter().position(|&l| l == self.recent_projects_limit).unwrap_or(1);
        let next_idx = (current_idx + 1) % limits.len();
        self.recent_projects_limit = limits[next_idx];

        if let Ok(mut config) = Config::load() {
            config.recent_projects_limit = self.recent_projects_limit;
            let _ = config.save();
        }
        self.settings_status_message =
            Some(format!("✓ Recent projects limit set to {}", self.recent_projects_limit));
    }

    pub fn toggle_auto_refresh(&mut self) {
        self.auto_refresh_on_launch = !self.auto_refresh_on_launch;

        if let Ok(mut config) = Config::load() {
            config.auto_refresh_on_launch = self.auto_refresh_on_launch;
            let _ = config.save();
        }
        let status = if self.auto_refresh_on_launch { "Enabled" } else { "Disabled" };
        self.settings_status_message = Some(format!("✓ Auto-refresh on launch {}", status));
    }

    pub fn reset_config_defaults(&mut self) {
        let default_config = Config::default();
        if default_config.save().is_ok() {
            self.reload_config_state();
            let _ = self.refresh();
            self.settings_status_message = Some("✓ Settings reset to defaults".to_string());
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Returns all available IDE selections (detected built-in + user custom IDEs).
    pub fn available_ide_selections(&self) -> Vec<crate::models::ide::IdeSelection> {
        let mut selections: Vec<crate::models::ide::IdeSelection> = self
            .installed_ides
            .iter()
            .map(|&ide| crate::models::ide::IdeSelection::BuiltIn(ide))
            .collect();

        for custom in &self.custom_ides {
            selections.push(crate::models::ide::IdeSelection::Custom(custom.id.clone()));
        }

        selections
    }

    /// Get the target IDE for a project path (override or default).
    pub fn get_project_ide(&self, path: &std::path::Path) -> crate::models::ide::IdeSelection {
        let ide = self.ide_overrides.get(path).cloned().unwrap_or_else(|| self.default_ide.clone());
        self.ensure_available_ide(ide)
    }

    /// Get the target IDE for a recent project (override, recent default, or config default).
    pub fn get_recent_project_ide(
        &self,
        recent: &RecentProject,
    ) -> crate::models::ide::IdeSelection {
        let ide = if let Some(override_ide) = self.ide_overrides.get(&recent.path) {
            override_ide.clone()
        } else {
            recent.ide.clone().unwrap_or_else(|| self.default_ide.clone())
        };
        self.ensure_available_ide(ide)
    }

    fn ensure_available_ide(
        &self,
        ide: crate::models::ide::IdeSelection,
    ) -> crate::models::ide::IdeSelection {
        let available = self.available_ide_selections();
        if !available.is_empty() && !available.contains(&ide) { available[0].clone() } else { ide }
    }

    /// Cycle through available installed built-in and custom IDEs for the currently selected project (Projects or Recent tab).
    pub fn cycle_selected_ide(&mut self) {
        let available = self.available_ide_selections();
        if available.is_empty() {
            return;
        }

        let (path, current_ide) = match self.active_tab {
            Tab::Projects => {
                let Some(project) = self.selected_project() else { return };
                (project.path.clone(), self.get_project_ide(&project.path))
            }
            Tab::Recent => {
                let Some(recent) = self.selected_recent_project() else { return };
                (recent.path.clone(), self.get_recent_project_ide(recent))
            }
            _ => return,
        };

        let next_ide = match available.iter().position(|i| i == &current_ide) {
            Some(idx) => available[(idx + 1) % available.len()].clone(),
            None => available[0].clone(),
        };

        self.ide_overrides.insert(path, next_ide);
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
            Tab::Ide => self.ide_tab_items().len(),
            Tab::Settings => self.settings_item_count(),
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
        let char_idx = self.search_cursor;
        let byte_idx = self
            .search_query
            .char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap_or(self.search_query.len());
        self.search_query.insert(byte_idx, c);
        self.search_cursor += 1;
        self.selected_index = 0;
    }

    pub fn pop_char(&mut self) {
        if self.search_cursor > 0 {
            let remove_idx = self.search_cursor - 1;
            if let Some((byte_idx, _)) = self.search_query.char_indices().nth(remove_idx) {
                self.search_query.remove(byte_idx);
                self.search_cursor -= 1;
            }
        }
        self.selected_index = 0;
    }

    pub fn delete_char(&mut self) {
        let char_idx = self.search_cursor;
        if let Some((byte_idx, _)) = self.search_query.char_indices().nth(char_idx) {
            self.search_query.remove(byte_idx);
        }
        self.selected_index = 0;
    }

    pub fn move_search_cursor_left(&mut self) {
        self.search_cursor = self.search_cursor.saturating_sub(1);
    }

    pub fn move_search_cursor_right(&mut self) {
        let max_len = self.search_query.chars().count();
        if self.search_cursor < max_len {
            self.search_cursor += 1;
        }
    }

    pub fn move_search_cursor_home(&mut self) {
        self.search_cursor = 0;
    }

    pub fn move_search_cursor_end(&mut self) {
        self.search_cursor = self.search_query.chars().count();
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.search_cursor = 0;
        self.selected_index = 0;
    }

    pub fn active_editing_field_and_cursor(&mut self) -> Option<(&mut String, &mut usize)> {
        if self.input_mode != InputMode::Editing {
            return None;
        }
        match self.active_tab {
            Tab::Ide => match self.ide_form_field {
                0 => Some((&mut self.ide_form_name, &mut self.ide_form_name_cursor)),
                1 => Some((&mut self.ide_form_path, &mut self.ide_form_path_cursor)),
                2 => Some((&mut self.ide_form_args, &mut self.ide_form_args_cursor)),
                _ => Some((&mut self.ide_form_name, &mut self.ide_form_name_cursor)),
            },
            Tab::Settings => {
                Some((&mut self.settings_add_root, &mut self.settings_add_root_cursor))
            }
            _ => None,
        }
    }

    pub fn editing_insert_char(&mut self, c: char) {
        if let Some((field, cursor)) = self.active_editing_field_and_cursor() {
            let char_idx = *cursor;
            let byte_idx =
                field.char_indices().nth(char_idx).map(|(i, _)| i).unwrap_or(field.len());
            field.insert(byte_idx, c);
            *cursor += 1;
        }
    }

    pub fn editing_backspace(&mut self) {
        if let Some((field, cursor)) = self.active_editing_field_and_cursor()
            && *cursor > 0
        {
            let remove_idx = *cursor - 1;
            if let Some((byte_idx, _)) = field.char_indices().nth(remove_idx) {
                field.remove(byte_idx);
                *cursor -= 1;
            }
        }
    }

    pub fn editing_delete(&mut self) {
        if let Some((field, cursor)) = self.active_editing_field_and_cursor() {
            let char_idx = *cursor;
            if let Some((byte_idx, _)) = field.char_indices().nth(char_idx) {
                field.remove(byte_idx);
            }
        }
    }

    pub fn editing_move_cursor_left(&mut self) {
        if let Some((_, cursor)) = self.active_editing_field_and_cursor() {
            *cursor = cursor.saturating_sub(1);
        }
    }

    pub fn editing_move_cursor_right(&mut self) {
        if let Some((field, cursor)) = self.active_editing_field_and_cursor() {
            let max_len = field.chars().count();
            if *cursor < max_len {
                *cursor += 1;
            }
        }
    }

    pub fn editing_move_cursor_home(&mut self) {
        if let Some((_, cursor)) = self.active_editing_field_and_cursor() {
            *cursor = 0;
        }
    }

    pub fn editing_move_cursor_end(&mut self) {
        if let Some((field, cursor)) = self.active_editing_field_and_cursor() {
            *cursor = field.chars().count();
        }
    }

    pub fn clamp_selection(&mut self) {
        let len = match self.active_tab {
            Tab::Projects => self.visible_items().len(),
            Tab::Recent => self.recent_projects.len(),
            Tab::Ide => self.ide_tab_items().len(),
            Tab::Settings => self.settings_item_count(),
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

    // ──────────────────────────────────────────────────────
    // IDE Tab
    // ──────────────────────────────────────────────────────

    /// Build the merged list of detected built-in + custom IDEs.
    pub fn ide_tab_items(&self) -> Vec<IdeListEntry> {
        let mut entries: Vec<IdeListEntry> = self
            .installed_ides
            .iter()
            .map(|&ide| IdeListEntry::BuiltIn {
                ide,
                is_default: self.default_ide == crate::models::ide::IdeSelection::BuiltIn(ide),
            })
            .collect();

        for custom in &self.custom_ides {
            entries.push(IdeListEntry::Custom {
                custom: custom.clone(),
                is_default: self.default_ide
                    == crate::models::ide::IdeSelection::Custom(custom.id.clone()),
            });
        }

        entries
    }

    /// Set the currently selected built-in or custom IDE as the global default.
    pub fn set_default_selected_ide(&mut self) {
        let items = self.ide_tab_items();
        match items.get(self.selected_index) {
            Some(IdeListEntry::BuiltIn { ide, .. }) => {
                let selection = crate::models::ide::IdeSelection::BuiltIn(*ide);
                self.default_ide = selection.clone();

                if let Ok(mut config) = Config::load() {
                    config.default_ide = selection;
                    let _ = config.save();
                }

                self.ide_status_message = Some(format!("✓ {} set as default", ide.display_name()));
            }
            Some(IdeListEntry::Custom { custom, .. }) => {
                let selection = crate::models::ide::IdeSelection::Custom(custom.id.clone());
                self.default_ide = selection.clone();

                if let Ok(mut config) = Config::load() {
                    config.default_ide = selection;
                    let _ = config.save();
                }

                self.ide_status_message = Some(format!("✓ {} set as default", custom.display_name));
            }
            None => {}
        }
    }

    /// Request deletion of the currently selected custom IDE (triggers confirmation prompt).
    pub fn request_delete_selected_custom_ide(&mut self) {
        let items = self.ide_tab_items();
        if let Some(IdeListEntry::Custom { custom, .. }) = items.get(self.selected_index) {
            self.ide_tab_focus = IdeTabFocus::ConfirmDelete;
            self.ide_status_message =
                Some(format!("Are you sure you wish to delete '{}'?", custom.display_name));
        }
    }

    /// Cancel custom IDE deletion request.
    pub fn cancel_delete_custom_ide(&mut self) {
        self.ide_tab_focus = IdeTabFocus::List;
        self.ide_status_message = None;
    }

    /// Remove the currently selected custom IDE.
    pub fn remove_selected_custom_ide(&mut self) {
        let items = self.ide_tab_items();
        if let Some(IdeListEntry::Custom { custom, .. }) = items.get(self.selected_index) {
            let id = custom.id.clone();
            let name = custom.display_name.clone();
            self.custom_ides.retain(|c| c.id != id);

            if self.default_ide == crate::models::ide::IdeSelection::Custom(id.clone()) {
                self.default_ide = self
                    .available_ide_selections()
                    .first()
                    .cloned()
                    .unwrap_or(crate::models::ide::IdeSelection::BuiltIn(Ide::Vscode));
            }

            if let Ok(mut config) = Config::load() {
                config.remove_custom_ide(&id);
                config.default_ide = self.default_ide.clone();
                let _ = config.save();
            }

            self.ide_tab_focus = IdeTabFocus::List;
            self.clamp_selection();
            self.ide_status_message = Some(format!("✓ {} removed", name));
        } else {
            self.ide_tab_focus = IdeTabFocus::List;
        }
    }

    /// Submit the IDE add form — validate, verify executable, and persist a new custom IDE.
    pub fn submit_ide_form(&mut self) -> Result<(), String> {
        let name = self.ide_form_name.trim().to_string();
        let path = self.ide_form_path.trim().to_string();
        let args = self.ide_form_args.trim().to_string();

        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if path.is_empty() {
            return Err("Path cannot be empty".to_string());
        }

        let executable = PathBuf::from(&path);
        if !verify_executable(&executable) {
            return Err(format!("Executable path '{}' does not exist or is invalid", path));
        }

        let id = name.to_lowercase().replace(' ', "-");

        if self.custom_ides.iter().any(|c| c.id == id) {
            return Err(format!("IDE '{}' already exists", name));
        }

        let custom = CustomIde {
            id,
            display_name: name,
            executable,
            args_template: if args.is_empty() { None } else { Some(args) },
            verified: true,
        };

        self.custom_ides.push(custom.clone());
        self.custom_ides.sort_by_key(|a| a.display_name.to_lowercase());

        if let Ok(mut config) = Config::load() {
            config.add_custom_ide(custom);
            let _ = config.save();
        }

        // Reset form
        self.ide_form_name.clear();
        self.ide_form_name_cursor = 0;
        self.ide_form_path.clear();
        self.ide_form_path_cursor = 0;
        self.ide_form_args.clear();
        self.ide_form_args_cursor = 0;
        self.ide_form_field = 0;
        self.ide_tab_focus = IdeTabFocus::List;
        self.input_mode = InputMode::Normal;

        self.ide_status_message = Some("✓ Added and verified".to_string());

        Ok(())
    }

    /// Get the active IDE form field's mutable reference.
    pub fn active_ide_form_field(&mut self) -> &mut String {
        match self.ide_form_field {
            0 => &mut self.ide_form_name,
            1 => &mut self.ide_form_path,
            2 => &mut self.ide_form_args,
            _ => &mut self.ide_form_name,
        }
    }

    /// Cycle the IDE form's active field.
    pub fn cycle_ide_form_field(&mut self) {
        self.ide_form_field = (self.ide_form_field + 1) % 3;
    }

    // ──────────────────────────────────────────────────────
    // Settings Tab
    // ──────────────────────────────────────────────────────

    /// Number of items on the Settings tab list.
    pub fn settings_item_count(&self) -> usize {
        self.project_roots.len()
    }

    /// Submit the "Add Root" form — validate and persist.
    pub fn submit_add_root(&mut self) -> Result<(), String> {
        let path_str = self.settings_add_root.trim().to_string();
        if path_str.is_empty() {
            return Err("Path cannot be empty".to_string());
        }

        let path = PathBuf::from(&path_str);

        if !path.exists() || !path.is_dir() {
            return Err("Path does not exist or is not a directory".to_string());
        }

        if self.project_roots.contains(&path) {
            return Err("Root already exists".to_string());
        }

        self.project_roots.push(path.clone());

        if let Ok(mut config) = Config::load() {
            config.add_project_root(path);
            let _ = config.save();
        }

        self.settings_add_root.clear();
        self.settings_add_root_cursor = 0;
        self.settings_tab_focus = SettingsTabFocus::List;
        self.input_mode = InputMode::Normal;

        self.settings_status_message = Some("✓ Root added".to_string());

        // Refresh projects after adding a root.
        let _ = self.refresh();

        Ok(())
    }

    /// Remove the currently selected project root.
    pub fn remove_selected_root(&mut self) {
        if self.selected_index < self.project_roots.len() {
            let removed = self.project_roots.remove(self.selected_index);

            if let Ok(mut config) = Config::load() {
                config.projects_root = self.project_roots.clone();
                let _ = config.save();
            }

            self.clamp_selection();
            self.settings_status_message = Some(format!("✓ Removed {}", removed.display()));

            // Refresh projects after removing a root.
            let _ = self.refresh();
        }
    }

    // ──────────────────────────────────────────────────────
    // Clone Form Methods (Projects tab)
    // ──────────────────────────────────────────────────────

    /// Opens the clone form on the Projects tab.
    pub fn open_clone_modal(&mut self) {
        self.projects_tab_focus = ProjectsTabFocus::CloneForm;
        self.clone_url.clear();
        self.clone_url_cursor = 0;
        self.clone_root_index = 0;
        self.clone_name.clear();
        self.clone_name_cursor = 0;
        self.clone_field = 0;
        self.clone_status_message = None;
        self.clone_log_lines.clear();
        self.clone_task = None;
        self.input_mode = InputMode::Editing;
    }

    /// Closes the clone form and restores normal input mode.
    pub fn close_clone_modal(&mut self) {
        self.projects_tab_focus = ProjectsTabFocus::List;
        self.input_mode = InputMode::Normal;
    }

    /// Returns true if the clone form is active.
    pub fn is_clone_form_active(&self) -> bool {
        self.projects_tab_focus == ProjectsTabFocus::CloneForm
    }

    /// Cycle active field in clone modal (0 = URL, 1 = Root Selector, 2 = Custom Name).
    pub fn cycle_clone_field(&mut self) {
        self.clone_field = (self.clone_field + 1) % 3;
    }

    /// Cycle active field backwards in clone modal.
    pub fn cycle_clone_field_backwards(&mut self) {
        self.clone_field = if self.clone_field == 0 { 2 } else { self.clone_field - 1 };
    }

    /// Push character into active clone input field.
    pub fn clone_push_char(&mut self, c: char) {
        match self.clone_field {
            0 => {
                self.clone_url.insert(self.clone_url_cursor, c);
                self.clone_url_cursor += 1;
            }
            2 => {
                self.clone_name.insert(self.clone_name_cursor, c);
                self.clone_name_cursor += 1;
            }
            _ => {}
        }
    }

    /// Backspace character from active clone input field.
    pub fn clone_pop_char(&mut self) {
        match self.clone_field {
            0 if self.clone_url_cursor > 0 => {
                self.clone_url_cursor -= 1;
                self.clone_url.remove(self.clone_url_cursor);
            }
            2 if self.clone_name_cursor > 0 => {
                self.clone_name_cursor -= 1;
                self.clone_name.remove(self.clone_name_cursor);
            }
            _ => {}
        }
    }

    /// Delete character at cursor from active clone input field.
    pub fn clone_delete_char(&mut self) {
        match self.clone_field {
            0 if self.clone_url_cursor < self.clone_url.len() => {
                self.clone_url.remove(self.clone_url_cursor);
            }
            2 if self.clone_name_cursor < self.clone_name.len() => {
                self.clone_name.remove(self.clone_name_cursor);
            }
            _ => {}
        }
    }

    /// Move cursor left in active clone input field / select previous root.
    pub fn clone_move_left(&mut self) {
        match self.clone_field {
            0 if self.clone_url_cursor > 0 => {
                self.clone_url_cursor -= 1;
            }
            1 if self.clone_root_index > 0 => {
                self.clone_root_index -= 1;
            }
            2 if self.clone_name_cursor > 0 => {
                self.clone_name_cursor -= 1;
            }
            _ => {}
        }
    }

    /// Move cursor right in active clone input field / select next root.
    pub fn clone_move_right(&mut self) {
        match self.clone_field {
            0 if self.clone_url_cursor < self.clone_url.len() => {
                self.clone_url_cursor += 1;
            }
            1 if !self.project_roots.is_empty()
                && self.clone_root_index + 1 < self.project_roots.len() =>
            {
                self.clone_root_index += 1;
            }
            2 if self.clone_name_cursor < self.clone_name.len() => {
                self.clone_name_cursor += 1;
            }
            _ => {}
        }
    }

    /// Submit the clone form — validates inputs and spawns a background git thread.
    pub fn submit_clone(&mut self) -> Result<(), String> {
        let url = self.clone_url.trim().to_string();
        if url.is_empty() {
            return Err("URL cannot be empty".to_string());
        }

        if self.project_roots.is_empty() {
            return Err("No project roots configured".to_string());
        }

        // Already cloning — ignore duplicate submits.
        if self.clone_task.is_some() {
            return Ok(());
        }

        let root_index = self.clone_root_index.min(self.project_roots.len() - 1);
        let target_root = self.project_roots[root_index].clone();

        let custom_name = if self.clone_name.trim().is_empty() {
            None
        } else {
            Some(self.clone_name.trim().to_string())
        };

        self.clone_log_lines.clear();
        self.clone_status_message = Some("⏳ Cloning...".to_string());
        // Switch to Normal mode so the user can navigate away but keep form open.
        self.input_mode = InputMode::Normal;

        let channels =
            crate::commands::clone::clone_repository_streamed(url, target_root, custom_name);
        self.clone_task = Some(channels);
        Ok(())
    }

    /// Poll the background clone task channels and update state.
    /// Call this every tick from the event loop.
    pub fn tick_clone_task(&mut self) {
        // Keep at most 200 log lines.
        const MAX_LINES: usize = 200;

        if let Some(ref task) = self.clone_task {
            // Drain all pending log lines (non-blocking).
            while let Ok(line) = task.log_rx.try_recv() {
                if self.clone_log_lines.len() >= MAX_LINES {
                    self.clone_log_lines.pop_front();
                }
                self.clone_log_lines.push_back(line);
            }

            // Check for completion.
            match task.done_rx.try_recv() {
                Ok(Ok(cloned_path)) => {
                    let msg = format!("✓ Cloned into {}", cloned_path.display());
                    self.clone_status_message = Some(msg.clone());
                    self.clone_log_lines.push_back(format!("✓ Done → {}", cloned_path.display()));
                    self.clone_task = None;
                    // Immediately refresh the project tree.
                    let _ = self.refresh();
                    // Automatically close the clone split window.
                    self.close_clone_modal();
                }
                Ok(Err(err)) => {
                    self.clone_status_message = Some(format!("✗ {err}"));
                    self.clone_log_lines.push_back(format!("✗ Error: {err}"));
                    self.clone_task = None;
                    self.input_mode = InputMode::Editing;
                }
                Err(_) => {} // Still running.
            }
        }
    }
}
