use directories::BaseDirs;
use which::which;

use crate::{
    ide::registry::InstalledIde,
    models::ide::Ide,
};

pub fn detect_ides() -> Vec<InstalledIde> {
    let mut found = Vec::new();

    detect_cli(
        &mut found,
        Ide::Vscode,
        "VS Code",
        "code",
    );

    detect_cli(
        &mut found,
        Ide::Cursor,
        "Cursor",
        "cursor",
    );

    detect_cli(
        &mut found,
        Ide::Claude,
        "Claude Code",
        "claude",
    );

    detect_cli(
        &mut found,
        Ide::Terminal,
        "Windows Terminal",
        "wt",
    );

    detect_common_windows_locations(&mut found);

    found
}

fn detect_cli(
    list: &mut Vec<InstalledIde>,
    ide: Ide,
    name: &str,
    cmd: &str,
) {
    if let Ok(path) = which(cmd) {
        list.push(InstalledIde::new(
            ide,
            name,
            path,
        ));
    }
}

fn detect_common_windows_locations(
    list: &mut Vec<InstalledIde>,
) {
    let home = BaseDirs::new().unwrap().home_dir().to_path_buf();

    let vscode = home.join(
        "AppData/Local/Programs/Microsoft VS Code/bin/code.cmd",
    );

    if vscode.exists()
        && !list.iter().any(|i| matches!(i.ide, Ide::Vscode))
    {
        list.push(InstalledIde::new(
            Ide::Vscode,
            "VS Code",
            vscode,
        ));
    }

    let cursor = home.join(
        "AppData/Local/Programs/Cursor/Cursor.exe",
    );

    if cursor.exists() {
        list.push(InstalledIde::new(
            Ide::Cursor,
            "Cursor",
            cursor,
        ));
    }

    let claude = home.join(".local/bin/claude.exe");

    if claude.exists()
        && !list.iter().any(|i| matches!(i.ide, Ide::Claude))
    {
        list.push(InstalledIde::new(
            Ide::Claude,
            "Claude Code",
            claude,
        ));
    }
}