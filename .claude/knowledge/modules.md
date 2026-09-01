# Modules

> Condensed map of every Rust module and its boundary contract. Mirrors `.claude/AGENTS.md` "Directory Responsibilities"; kept here so AI agents get the same rules without the full prose.

## Layered Boundaries

| Layer | Path | May import from | Must NOT import |
|-------|------|-----------------|-----------------|
| CLI | `src/cli.rs`, `src/main.rs` | `commands/*`, `models` | services logic, fs ops |
| Commands | `src/commands/*.rs` | services, models, `cli` | heavy business logic, direct fs/process |
| Services | `src/config.rs`, `src/ide/*`, `src/installer.rs`, `src/scanner.rs` | models, external crates | `commands/*` |
| Models | `src/models/*` | only serde/clap/derive crates | services, commands, fs |

## Per-Module Contract

### `src/main.rs` (~35 lines)
- Init tracing-subscriber, `Cli::parse()`, match-dispatch, `Ok(())`.
- **No** business logic, fs I/O, or config work.

### `src/cli.rs` (~140 lines)
- Clap derive structs/enums: `Cli`, `Commands` (Project/Config/Ide/Install/Open), `OpenArgs`, `ProjectCommand`/`ProjectSubcommand`, `ConfigCommand`/`ConfigSubcommand`, `IdeCommand`/`IdeSubcommand`, `InstallCommand`.
- `Ide` imported for `ValueEnum` parsing of `--ide`.

### `src/commands/project.rs` (~70 lines)
- `execute(ProjectCommand)`, `open_shortcut(OpenArgs)`, private `open`, `list_projects`.
- Loads config, calls scanner + launcher, formats terminal output.

### `src/commands/config.rs` (~70 lines)
- `execute(ConfigCommand)` → `init` / `show` / `set-default-ide`.
- Uses only `Config`.

### `src/commands/ide.rs` (~50 lines)
- `execute(IdeCommand)` → `list()` → `detect_ides()`, prints name + path.

### `src/commands/install.rs` (~12 lines)
- `execute(_cmd)` → `installer::install()`.

### `src/config.rs` (~150 lines)
- `Config { projects_root: Vec<PathBuf>, default_ide: Ide }` with serde.
- `Default` → `~/Projects` + `Ide::Vscode`.
- `path()` (env override `DEVCLI_CONFIG_DIR` → else `ProjectDirs`), `load()`, `save()`.

### `src/scanner.rs` (~100 lines)
- `discover_projects(&[PathBuf]) -> Result<Vec<Project>>` — ignore-crate walk, dedupe by canonical path, sorted by name.
- `IGNORED_DIRS`: `.git, target, node_modules, .venv, venv, build, dist, .idea, .vscode`.
- `is_git_repo`: `path.join(".git").is_dir()`.

### `src/ide/`
- `detect.rs` — `detect_ides()`, `detect_cli` (which), `detect_common_windows_locations`.
- `launcher.rs` — `launch(Ide, &Path)`; per-IDE arg mapping; `DEVCLI_TEST_EXECUTABLE` override.
- `registry.rs` — `InstalledIde { ide, display_name, executable }`, `InstalledIde::new`.

### `src/installer.rs` (~65 lines)
- `binary_install_dir()` — `DEVCLI_INSTALL_DIR` else `~/.local/bin`.
- `install()` — copy current exe to `dev[.exe]`, ensure config exists, print PATH hint.

### `src/utils/path.rs`
- `display_path(&Path) -> String` — strips `\\?\` canonical prefix on Windows.

### `src/models/`
- `ide.rs` — `enum Ide { Cursor, Vscode, Claude, Terminal, Idea, Rider, Zed }` (Copy/Clone/Eq, serde, ValueEnum).
- `project.rs` — `Project { name, path, root, git_dir }`, `Project::new(path, root)`.

### `xtask/src/main.rs`
- Not part of the shipped binary. `cargo xtask ci` runs fmt/clippy/security/tests/coverage gates; `coverage`/`coverage-summary` alias `cargo llvm-cov`.
