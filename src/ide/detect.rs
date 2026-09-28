//! IDE detection algorithm.

use directories::BaseDirs;
use std::path::{Path, PathBuf};
use which::which;

use crate::{ide::registry::InstalledIde, models::ide::Ide};

pub fn detect_ides() -> Vec<InstalledIde> {
    let mut found = Vec::new();

    detect_cli(&mut found, Ide::Antigravity, "Antigravity", "antigravity");
    detect_cli(&mut found, Ide::AntigravityCli, "Antigravity CLI", "agy");
    if !found.iter().any(|i| matches!(i.ide, Ide::AntigravityCli)) {
        detect_cli(&mut found, Ide::AntigravityCli, "Antigravity CLI", "antigravity");
    }
    detect_cli(&mut found, Ide::Cursor, "Cursor", "cursor");
    detect_cli(&mut found, Ide::Vscode, "VS Code", "code");
    detect_cli(&mut found, Ide::Windsurf, "Windsurf", "windsurf");
    detect_cli(&mut found, Ide::Claude, "Claude Code", "claude");
    detect_cli(&mut found, Ide::Sublime, "Sublime Text", "subl");
    detect_cli(&mut found, Ide::Neovim, "Neovim", "nvim");
    detect_cli(&mut found, Ide::Zed, "Zed", "zed");
    detect_cli(&mut found, Ide::Idea, "IntelliJ IDEA", "idea");
    detect_cli(&mut found, Ide::PyCharm, "PyCharm", "pycharm");
    detect_cli(&mut found, Ide::WebStorm, "WebStorm", "webstorm");
    detect_cli(&mut found, Ide::CLion, "CLion", "clion");
    detect_cli(&mut found, Ide::RustRover, "RustRover", "rustrover");
    detect_cli(&mut found, Ide::GoLand, "GoLand", "goland");
    detect_cli(&mut found, Ide::Fleet, "Fleet", "fleet");
    detect_cli(&mut found, Ide::Rider, "Rider", "rider");
    detect_cli(&mut found, Ide::AndroidStudio, "Android Studio", "studio");
    detect_cli(&mut found, Ide::VisualStudio, "Visual Studio", "devenv");
    detect_cli(&mut found, Ide::Helix, "Helix", "hx");

    // Cross-platform Terminal detection (Windows, macOS, Linux)
    detect_terminal(&mut found);

    detect_common_windows_locations(&mut found);
    detect_common_macos_locations(&mut found);
    detect_common_linux_locations(&mut found);

    found.sort_by_key(|a| a.display_name.to_lowercase());

    found
}

fn detect_terminal(list: &mut Vec<InstalledIde>) {
    if cfg!(windows) {
        if let Ok(path) = which("wt") {
            list.push(InstalledIde::new(Ide::Terminal, "Terminal", path));
        } else if let Ok(cmd) = which("cmd") {
            list.push(InstalledIde::new(Ide::Terminal, "Terminal", cmd));
        } else if let Ok(powershell) = which("powershell") {
            list.push(InstalledIde::new(Ide::Terminal, "Terminal", powershell));
        }
    } else if cfg!(target_os = "macos") {
        if let Ok(open_path) = which("open") {
            list.push(InstalledIde::new(Ide::Terminal, "Terminal", open_path));
        } else {
            let open_bin = PathBuf::from("/usr/bin/open");
            if open_bin.exists() {
                list.push(InstalledIde::new(Ide::Terminal, "Terminal", open_bin));
            } else if let Ok(sh) = which("zsh").or_else(|_| which("bash")) {
                list.push(InstalledIde::new(Ide::Terminal, "Terminal", sh));
            }
        }
    } else {
        let linux_terminals = [
            "gnome-terminal",
            "konsole",
            "alacritty",
            "kitty",
            "wezterm",
            "xterm",
            "x-terminal-emulator",
        ];
        let mut term_found = false;
        for t in &linux_terminals {
            if let Ok(path) = which(t) {
                list.push(InstalledIde::new(Ide::Terminal, "Terminal", path));
                term_found = true;
                break;
            }
        }
        if !term_found && let Ok(sh) = which("bash").or_else(|_| which("sh")) {
            list.push(InstalledIde::new(Ide::Terminal, "Terminal", sh));
        }
    }
}

/// Check if IDE is available as command-line tool in PATH.
fn detect_cli(list: &mut Vec<InstalledIde>, ide: Ide, name: &str, cmd: &str) {
    if let Ok(path) = which(cmd) {
        list.push(InstalledIde::new(ide, name, path));
    }
}

/// Check common Windows installation directories.
fn detect_common_windows_locations(list: &mut Vec<InstalledIde>) {
    let home = match BaseDirs::new() {
        Some(dirs) => dirs.home_dir().to_path_buf(),
        None => return,
    };
    detect_common_windows_locations_in(list, &home);
}

/// Check common macOS installation directories.
fn detect_common_macos_locations(list: &mut Vec<InstalledIde>) {
    let home = match BaseDirs::new() {
        Some(dirs) => dirs.home_dir().to_path_buf(),
        None => return,
    };
    detect_common_macos_locations_in(list, &home);
}

/// Check common Linux installation directories.
fn detect_common_linux_locations(list: &mut Vec<InstalledIde>) {
    let home = match BaseDirs::new() {
        Some(dirs) => dirs.home_dir().to_path_buf(),
        None => return,
    };
    detect_common_linux_locations_in(list, &home);
}

/// Check common macOS installation directories.
#[doc(hidden)]
pub fn detect_common_macos_locations_in(list: &mut Vec<InstalledIde>, home: &Path) {
    let app_dirs = [PathBuf::from("/Applications"), home.join("Applications")];

    let macos_apps = [
        (Ide::Vscode, "VS Code", "Visual Studio Code.app/Contents/Resources/app/bin/code"),
        (Ide::Cursor, "Cursor", "Cursor.app/Contents/Resources/app/bin/cursor"),
        (Ide::Antigravity, "Antigravity", "Antigravity.app/Contents/Resources/app/bin/antigravity"),
        (Ide::Antigravity, "Antigravity", "Antigravity.app/Contents/Resources/app/bin/agy"),
        (Ide::Windsurf, "Windsurf", "Windsurf.app/Contents/Resources/app/bin/windsurf"),
        (Ide::Sublime, "Sublime Text", "Sublime Text.app/Contents/SharedSupport/bin/subl"),
        (Ide::Zed, "Zed", "Zed.app/Contents/MacOS/zed"),
        (Ide::Idea, "IntelliJ IDEA", "IntelliJ IDEA.app/Contents/MacOS/idea"),
        (Ide::Idea, "IntelliJ IDEA", "IntelliJ IDEA CE.app/Contents/MacOS/idea"),
        (Ide::PyCharm, "PyCharm", "PyCharm.app/Contents/MacOS/pycharm"),
        (Ide::PyCharm, "PyCharm", "PyCharm CE.app/Contents/MacOS/pycharm"),
        (Ide::WebStorm, "WebStorm", "WebStorm.app/Contents/MacOS/webstorm"),
        (Ide::CLion, "CLion", "CLion.app/Contents/MacOS/clion"),
        (Ide::RustRover, "RustRover", "RustRover.app/Contents/MacOS/rustrover"),
        (Ide::GoLand, "GoLand", "GoLand.app/Contents/MacOS/goland"),
        (Ide::Fleet, "Fleet", "Fleet.app/Contents/MacOS/Fleet"),
        (Ide::Rider, "Rider", "Rider.app/Contents/MacOS/rider"),
        (Ide::AndroidStudio, "Android Studio", "Android Studio.app/Contents/MacOS/studio"),
    ];

    for (ide, name, rel_path) in &macos_apps {
        if list.iter().any(|i| i.ide == *ide) {
            continue;
        }
        for dir in &app_dirs {
            let candidate = dir.join(rel_path);
            if candidate.exists() {
                list.push(InstalledIde::new(*ide, name, candidate));
                break;
            }
        }
    }

    let macos_cli_tools = [
        (
            Ide::AntigravityCli,
            "Antigravity CLI",
            vec![
                home.join(".antigravity/bin/agy"),
                home.join(".antigravity/bin/antigravity"),
                home.join(".local/bin/agy"),
                home.join(".local/bin/antigravity"),
                PathBuf::from("/opt/homebrew/bin/agy"),
                PathBuf::from("/opt/homebrew/bin/antigravity"),
                PathBuf::from("/usr/local/bin/agy"),
                PathBuf::from("/usr/local/bin/antigravity"),
            ],
        ),
        (
            Ide::Claude,
            "Claude Code",
            vec![
                home.join(".local/bin/claude"),
                PathBuf::from("/opt/homebrew/bin/claude"),
                PathBuf::from("/usr/local/bin/claude"),
            ],
        ),
        (
            Ide::Neovim,
            "Neovim",
            vec![
                home.join(".local/bin/nvim"),
                PathBuf::from("/opt/homebrew/bin/nvim"),
                PathBuf::from("/usr/local/bin/nvim"),
            ],
        ),
        (
            Ide::Helix,
            "Helix",
            vec![
                home.join(".local/bin/hx"),
                PathBuf::from("/opt/homebrew/bin/hx"),
                PathBuf::from("/usr/local/bin/hx"),
            ],
        ),
    ];

    for (ide, name, paths) in &macos_cli_tools {
        detect_first_existing(list, *ide, name, paths);
    }
}

/// Check common Linux installation directories.
#[doc(hidden)]
pub fn detect_common_linux_locations_in(list: &mut Vec<InstalledIde>, home: &Path) {
    let linux_bins = [
        (
            Ide::AntigravityCli,
            "Antigravity CLI",
            vec![
                home.join(".antigravity/bin/agy"),
                home.join(".antigravity/bin/antigravity"),
                home.join(".local/bin/agy"),
                home.join(".local/bin/antigravity"),
                PathBuf::from("/usr/bin/agy"),
                PathBuf::from("/usr/bin/antigravity"),
                PathBuf::from("/usr/local/bin/agy"),
                PathBuf::from("/usr/local/bin/antigravity"),
                PathBuf::from("/snap/bin/agy"),
                PathBuf::from("/snap/bin/antigravity"),
            ],
        ),
        (
            Ide::Vscode,
            "VS Code",
            vec![
                PathBuf::from("/snap/bin/code"),
                PathBuf::from("/usr/bin/code"),
                PathBuf::from("/usr/local/bin/code"),
                home.join(".local/bin/code"),
            ],
        ),
        (
            Ide::Cursor,
            "Cursor",
            vec![
                PathBuf::from("/usr/bin/cursor"),
                PathBuf::from("/usr/local/bin/cursor"),
                PathBuf::from("/snap/bin/cursor"),
                home.join(".local/bin/cursor"),
                home.join(".local/bin/Cursor.AppImage"),
            ],
        ),
        (
            Ide::Claude,
            "Claude Code",
            vec![
                PathBuf::from("/usr/bin/claude"),
                PathBuf::from("/usr/local/bin/claude"),
                home.join(".local/bin/claude"),
            ],
        ),
        (
            Ide::Windsurf,
            "Windsurf",
            vec![
                PathBuf::from("/usr/bin/windsurf"),
                PathBuf::from("/usr/local/bin/windsurf"),
                home.join(".local/bin/windsurf"),
            ],
        ),
        (
            Ide::Sublime,
            "Sublime Text",
            vec![
                PathBuf::from("/snap/bin/subl"),
                PathBuf::from("/usr/bin/subl"),
                PathBuf::from("/usr/local/bin/subl"),
            ],
        ),
        (
            Ide::Neovim,
            "Neovim",
            vec![
                PathBuf::from("/usr/bin/nvim"),
                PathBuf::from("/usr/local/bin/nvim"),
                home.join(".local/bin/nvim"),
                PathBuf::from("/snap/bin/nvim"),
            ],
        ),
        (
            Ide::Helix,
            "Helix",
            vec![
                PathBuf::from("/usr/bin/hx"),
                PathBuf::from("/usr/local/bin/hx"),
                home.join(".local/bin/hx"),
                PathBuf::from("/snap/bin/hx"),
            ],
        ),
        (
            Ide::Zed,
            "Zed",
            vec![
                PathBuf::from("/usr/bin/zed"),
                PathBuf::from("/usr/local/bin/zed"),
                home.join(".local/bin/zed"),
                home.join(".local/bin/zed-cli"),
            ],
        ),
    ];

    for (ide, name, paths) in &linux_bins {
        detect_first_existing(list, *ide, name, paths);
    }

    let jb_tools = [
        (Ide::Idea, "IntelliJ IDEA", "idea", &["IDEA-ULT", "IDEA-C"][..]),
        (Ide::PyCharm, "PyCharm", "pycharm", &["PyCharm-P", "PyCharm-C"][..]),
        (Ide::WebStorm, "WebStorm", "webstorm", &["WebStorm"][..]),
        (Ide::CLion, "CLion", "clion", &["CLion"][..]),
        (Ide::RustRover, "RustRover", "rustrover", &["RustRover"][..]),
        (Ide::GoLand, "GoLand", "goland", &["GoLand"][..]),
        (Ide::Rider, "Rider", "rider", &["Rider"][..]),
    ];

    let jb_toolbox_apps = home.join(".local/share/JetBrains/Toolbox/apps");
    scan_jetbrains_toolbox(list, &jb_toolbox_apps, &jb_tools);
}

/// Check common Windows installation directories under a given home directory.
#[doc(hidden)]
pub fn detect_common_windows_locations_in(list: &mut Vec<InstalledIde>, home: &Path) {
    let is_real_home = BaseDirs::new().map(|b| b.home_dir() == home).unwrap_or(false);

    let mut pf_roots = vec![home.join("Program Files"), home.join("Program Files (x86)")];

    if is_real_home {
        let pf = std::env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".to_string());
        let pf86 = std::env::var("ProgramFiles(x86)")
            .unwrap_or_else(|_| r"C:\Program Files (x86)".to_string());
        pf_roots.push(PathBuf::from(&pf));
        pf_roots.push(PathBuf::from(&pf86));
    }

    // Visual Studio (2022, 2019, 2017 across Community, Professional, Enterprise, Preview)
    let vs_editions = ["Community", "Professional", "Enterprise", "Preview", "BuildTools"];
    let vs_years = ["2022", "2019", "2017"];
    for root in &pf_roots {
        for year in &vs_years {
            for edition in &vs_editions {
                let devenv = root.join(format!(
                    "Microsoft Visual Studio/{year}/{edition}/Common7/IDE/devenv.exe"
                ));
                if devenv.exists() && !list.iter().any(|i| matches!(i.ide, Ide::VisualStudio)) {
                    list.push(InstalledIde::new(Ide::VisualStudio, "Visual Studio", devenv));
                    break;
                }
            }
        }
    }

    // Antigravity (GUI IDE)
    detect_first_existing(
        list,
        Ide::Antigravity,
        "Antigravity",
        &[
            home.join("AppData/Local/Programs/Antigravity/Antigravity.exe"),
            pf_roots[0].join("Antigravity/Antigravity.exe"),
        ],
    );

    // Antigravity CLI
    detect_first_existing(
        list,
        Ide::AntigravityCli,
        "Antigravity CLI",
        &[
            home.join("AppData/Local/Programs/Antigravity/bin/agy.cmd"),
            home.join(".antigravity/bin/agy.exe"),
            home.join(".antigravity/bin/antigravity.exe"),
        ],
    );

    // VS Code: Standard Windows User & System installation paths
    detect_first_existing(
        list,
        Ide::Vscode,
        "VS Code",
        &[
            home.join("AppData/Local/Programs/Microsoft VS Code/bin/code.cmd"),
            home.join("AppData/Local/Programs/Microsoft VS Code/Code.exe"),
            pf_roots[0].join("Microsoft VS Code/bin/code.cmd"),
            pf_roots[0].join("Microsoft VS Code/Code.exe"),
        ],
    );

    // Cursor: Standard Windows installation path
    detect_first_existing(
        list,
        Ide::Cursor,
        "Cursor",
        &[
            home.join("AppData/Local/Programs/Cursor/Cursor.exe"),
            pf_roots[0].join("Cursor/Cursor.exe"),
        ],
    );

    // Windsurf: Standard Windows installation path
    detect_first_existing(
        list,
        Ide::Windsurf,
        "Windsurf",
        &[
            home.join("AppData/Local/Programs/Windsurf/Windsurf.exe"),
            home.join("AppData/Local/Programs/Windsurf/bin/windsurf.cmd"),
        ],
    );

    // Claude Code: ~/.local/bin location
    detect_first_existing(list, Ide::Claude, "Claude Code", &[home.join(".local/bin/claude.exe")]);

    // Zed: Standard Windows AppData installation path
    detect_first_existing(
        list,
        Ide::Zed,
        "Zed",
        &[home.join("AppData/Local/Programs/Zed/Zed.exe")],
    );

    // Fleet: Standard Windows AppData installation path
    detect_first_existing(
        list,
        Ide::Fleet,
        "Fleet",
        &[home.join("AppData/Local/Programs/Fleet/Fleet.exe")],
    );

    // Sublime Text
    let st_candidates = [
        pf_roots[0].join("Sublime Text/sublime_text.exe"),
        pf_roots[0].join("Sublime Text 3/sublime_text.exe"),
        pf_roots[1].join("Sublime Text/sublime_text.exe"),
        pf_roots[1].join("Sublime Text 3/sublime_text.exe"),
    ];
    detect_first_existing(list, Ide::Sublime, "Sublime Text", &st_candidates);

    // Android Studio
    let studio_candidates = [
        pf_roots[0].join("Android/Android Studio/bin/studio64.exe"),
        pf_roots[1].join("Android/Android Studio/bin/studio64.exe"),
    ];
    detect_first_existing(list, Ide::AndroidStudio, "Android Studio", &studio_candidates);

    // Neovim
    let nvim_candidates =
        [pf_roots[0].join("Neovim/bin/nvim.exe"), pf_roots[1].join("Neovim/bin/nvim.exe")];
    detect_first_existing(list, Ide::Neovim, "Neovim", &nvim_candidates);

    // JetBrains IDEs (IntelliJ IDEA, PyCharm, WebStorm, CLion, RustRover, GoLand, Rider)
    let jb_tools = [
        (Ide::Idea, "IntelliJ IDEA", "idea64.exe", &["IDEA-ULT", "IDEA-C"][..]),
        (Ide::PyCharm, "PyCharm", "pycharm64.exe", &["PyCharm-P", "PyCharm-C"][..]),
        (Ide::WebStorm, "WebStorm", "webstorm64.exe", &["WebStorm"][..]),
        (Ide::CLion, "CLion", "clion64.exe", &["CLion"][..]),
        (Ide::RustRover, "RustRover", "rustrover64.exe", &["RustRover"][..]),
        (Ide::GoLand, "GoLand", "goland64.exe", &["GoLand"][..]),
        (Ide::Rider, "Rider", "rider64.exe", &["Rider"][..]),
    ];

    scan_jetbrains_program_files(list, &pf_roots, &jb_tools);

    let local_appdata = home.join("AppData/Local");
    let jb_toolbox_apps = local_appdata.join("JetBrains/Toolbox/apps");
    scan_jetbrains_toolbox(list, &jb_toolbox_apps, &jb_tools);
}

fn detect_first_existing(
    list: &mut Vec<InstalledIde>,
    ide: Ide,
    name: &str,
    candidates: &[PathBuf],
) {
    if list.iter().any(|i| i.ide == ide) {
        return;
    }
    for candidate in candidates {
        if candidate.exists() {
            list.push(InstalledIde::new(ide, name, candidate.clone()));
            break;
        }
    }
}

fn scan_jetbrains_toolbox(
    list: &mut Vec<InstalledIde>,
    jb_toolbox_apps: &Path,
    jb_tools: &[(Ide, &str, &str, &[&str])],
) {
    if !jb_toolbox_apps.is_dir() {
        return;
    }
    for (ide, name, exe_name, tb_names) in jb_tools {
        if list.iter().any(|i| i.ide == *ide) {
            continue;
        }
        for tb_name in *tb_names {
            let tool_dir = jb_toolbox_apps.join(tb_name);
            if tool_dir.is_dir()
                && let Ok(channels) = std::fs::read_dir(&tool_dir)
            {
                for channel in channels.flatten() {
                    if let Ok(builds) = std::fs::read_dir(channel.path()) {
                        for build in builds.flatten() {
                            let candidate = build.path().join("bin").join(exe_name);
                            if candidate.exists() {
                                list.push(InstalledIde::new(*ide, name, candidate));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn scan_jetbrains_program_files(
    list: &mut Vec<InstalledIde>,
    pf_roots: &[PathBuf],
    jb_tools: &[(Ide, &str, &str, &[&str])],
) {
    for (ide, name, exe_name, _) in jb_tools {
        if list.iter().any(|i| i.ide == *ide) {
            continue;
        }
        for root in pf_roots {
            let jb_dir = root.join("JetBrains");
            if jb_dir.is_dir()
                && let Ok(entries) = std::fs::read_dir(&jb_dir)
            {
                for entry in entries.flatten() {
                    let candidate = entry.path().join("bin").join(exe_name);
                    if candidate.exists() {
                        list.push(InstalledIde::new(*ide, name, candidate));
                        break;
                    }
                }
            }
        }
    }
}

/// Check whether a file path points to a valid executable.
///
/// On Windows, checks that the file exists and has a recognised
/// executable extension (`.exe`, `.cmd`, `.bat`, `.com`).
pub fn verify_executable(path: &Path) -> bool {
    if !path.exists() || !path.is_file() {
        return false;
    }

    if cfg!(windows) {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| {
                let lower = ext.to_lowercase();
                lower == "exe" || lower == "cmd" || lower == "bat" || lower == "com"
            })
            .unwrap_or(false)
    } else {
        // On Unix, any existing file is accepted. A more
        // thorough check could inspect the execute permission
        // bits, but that isn't needed for the current scope.
        true
    }
}
