use anyhow::{Result, anyhow};
use temp_env::with_var;
use tempfile::TempDir;

use dev_cli::{config::Config, models::ide::Ide, tui::actions::open_project_with};

mod common;
use common::factories::fake_project as project;

fn with_temp_config<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let dir = TempDir::new().unwrap();
    with_var("DEVCLI_CONFIG_DIR", Some(dir.path()), f)
}

#[test]
fn open_project_uses_default_ide() -> Result<()> {
    with_temp_config(|| -> Result<()> {
        Config { projects_root: vec![], default_ide: Ide::Vscode, recent_projects: Vec::new() }
            .save()?;

        let project = project("demo");

        let mut called = false;

        open_project_with(&project, |ide, path| {
            called = true;
            assert_eq!(ide, Ide::Vscode);
            assert_eq!(path, &project.path);
            Ok(())
        })?;

        assert!(called);
        Ok(())
    })
}

#[test]
fn open_project_propagates_launcher_error() -> Result<()> {
    with_temp_config(|| -> Result<()> {
        Config { projects_root: vec![], default_ide: Ide::Vscode, recent_projects: Vec::new() }
            .save()?;

        let project = project("demo");

        let result = open_project_with(&project, |_, _| Err(anyhow!("launch failed")));

        assert!(result.is_err());
        Ok(())
    })
}

#[test]
#[serial_test::serial]
fn open_path_launches_and_updates_recents() {
    with_temp_config(|| {
        let temp = tempfile::TempDir::new().unwrap();
        let path = temp.path();

        let bin_dir = temp.path().join("bin");
        std::fs::create_dir_all(&bin_dir).unwrap();
        let exe = if cfg!(windows) { "code.exe" } else { "code" };
        let exe_path = bin_dir.join(exe);
        if cfg!(windows) {
            let comspec = std::env::var("COMSPEC")
                .unwrap_or_else(|_| r"C:\Windows\System32\cmd.exe".to_string());
            std::fs::copy(comspec, &exe_path).unwrap();
        } else {
            std::fs::write(&exe_path, "#!/bin/sh\nexit 0\n").unwrap();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&exe_path).unwrap().permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&exe_path, perms).unwrap();
            }
        }

        let path_sep = if cfg!(windows) { ";" } else { ":" };
        let path_var = match std::env::var("PATH") {
            Ok(p) => format!("{}{}{}", bin_dir.display(), path_sep, p),
            Err(_) => bin_dir.display().to_string(),
        };

        temp_env::with_var("PATH", Some(path_var), || {
            let res = dev_cli::tui::actions::open_path(path);
            assert!(res.is_ok());

            let config = Config::load().unwrap();
            assert_eq!(config.recent_projects.len(), 1);
            assert_eq!(config.recent_projects[0].path, path);
        });
    });
}
