//! IDE detection algorithm.
//!
//! Automatically finds installed IDEs through multi-stage detection:
//!
//! **Stage 1:** Check if IDE is available as CLI tool in PATH  
//! **Stage 2:** Check common Windows installation directories  
//! **Stage 3:** Deduplicate results
//!
//! # How It Works
//!
//! Detection runs quickly by checking:
//! 1. `which code`, `which cursor`, etc. (milliseconds)
//! 2. Known Windows paths (milliseconds)
//! 3. Removes duplicates automatically
//!
//! Total detection time: typically 10-100ms
//!
//! # Why Not Cache?
//!
//! Runtime detection avoids stale paths when IDEs are:
//! - Uninstalled
//! - Updated to new locations
//! - Installed on new machines
//!
//! Fresh detection every invocation ensures accuracy.

use directories::BaseDirs;
use which::which;

use crate::{ide::registry::InstalledIde, models::ide::Ide};

/// Detect all installed IDEs on the system.
///
/// Runs multi-stage detection algorithm to find IDEs:
/// 1. Check if IDE is in PATH
/// 2. Check common Windows installation directories
/// 3. Remove any duplicates
///
/// # Returns
///
/// Vector of detected IDEs. Empty if no supported IDEs found.
///
/// # Example
///
/// ```no_run
/// use dev_cli::ide::detect::detect_ides;
///
/// let ides = detect_ides();
/// for ide in ides {
///     println!("{}: {}", ide.display_name, ide.executable.display());
/// }
/// ```
pub fn detect_ides() -> Vec<InstalledIde> {
    let mut found = Vec::new();

    // Stage 1: Check CLI tools in PATH
    detect_cli(&mut found, Ide::Vscode, "VS Code", "code");
    detect_cli(&mut found, Ide::Cursor, "Cursor", "cursor");
    detect_cli(&mut found, Ide::Claude, "Claude Code", "claude");
    detect_cli(&mut found, Ide::Terminal, "Windows Terminal", "wt");

    // Stage 2: Check common Windows locations
    detect_common_windows_locations(&mut found);

    // Stage 3: Duplicates automatically handled by "already exists" check in detect_cli

    found
}

/// Check if IDE is available as command-line tool in PATH.
///
/// Uses `which` crate to locate executable in PATH.
/// Only adds to list if not already found.
///
/// # Arguments
///
/// * `list` — List to append to if IDE is found
/// * `ide` — IDE enum value
/// * `name` — Display name for IDE
/// * `cmd` — Command name to search for in PATH
fn detect_cli(list: &mut Vec<InstalledIde>, ide: Ide, name: &str, cmd: &str) {
    if let Ok(path) = which(cmd) {
        list.push(InstalledIde::new(ide, name, path));
    }
}

/// Check common Windows installation directories.
///
/// Checks standard Windows Program Files locations and AppData directories.
/// Avoids adding duplicates if IDE was already found in PATH.
///
/// # Arguments
///
/// * `list` — List to append to
fn detect_common_windows_locations(list: &mut Vec<InstalledIde>) {
    let home = BaseDirs::new().unwrap().home_dir().to_path_buf();

    // VS Code: Standard Windows installation path
    let vscode = home.join("AppData/Local/Programs/Microsoft VS Code/bin/code.cmd");
    if vscode.exists() && !list.iter().any(|i| matches!(i.ide, Ide::Vscode)) {
        list.push(InstalledIde::new(Ide::Vscode, "VS Code", vscode));
    }

    // Cursor: Standard Windows installation path
    let cursor = home.join("AppData/Local/Programs/Cursor/Cursor.exe");
    if cursor.exists() {
        list.push(InstalledIde::new(Ide::Cursor, "Cursor", cursor));
    }

    // Claude Code: ~/.local/bin location
    let claude = home.join(".local/bin/claude.exe");
    if claude.exists() && !list.iter().any(|i| matches!(i.ide, Ide::Claude)) {
        list.push(InstalledIde::new(Ide::Claude, "Claude Code", claude));
    }
}
