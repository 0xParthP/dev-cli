# API Reference

> Public surface of the `dev_cli` library crate and the `dev` binary. For exhaustive rustdoc, run `cargo doc --no-deps --open`.

## Binary — `dev`

| Command | Behavior |
|---------|----------|
| `dev project list` | Print configured roots + discovered repos |
| `dev project open <NAME> [--ide IDE]` | Open project in (default or specified) IDE |
| `dev open <NAME> [--ide IDE]` | Shorthand for `project open` |
| `dev config init` | Write default config to disk |
| `dev config show` | Print config with `{:#?}` |
| `dev config set-default-ide <IDE>` | Update and save `default_ide` |
| `dev ide list` | Print detected installed IDEs |
| `dev install` | Copy exe to install dir, init config |
| `dev --help` / `dev --version` | Clap help/version |

IDE values (ValueEnum): `cursor | vscode | claude | terminal | idea | rider | zed`.

## Library — `dev_cli`

### `dev_cli::cli`
- `struct Cli` — `command: Commands`.
- `enum Commands` — `Project(ProjectCommand)`, `Config(ConfigCommand)`, `Ide(IdeCommand)`, `Install(InstallCommand)`, `Open(OpenArgs)`.
- Subcommand/args structs mirror the binary usage table above.

### `dev_cli::config`
- `struct Config` — `projects_root: Vec<PathBuf>`, `default_ide: Ide`; `Serialize/Deserialize`.
- `Config::default()` — `~/Projects` root, `Ide::Vscode`. **Panics** if home dir missing.
- `Config::path() -> Result<PathBuf>` — `DEVCLI_CONFIG_DIR` override, else `ProjectDirs::from("", "", "dev-cli").config_dir()/config.toml`.
- `Config::load() -> Result<Config>` — create-defaults-and-save if missing.
- `Config::save(&self) -> Result<()>`.

### `dev_cli::scanner`
- `discover_projects(roots: &[PathBuf]) -> Result<Vec<Project>>` — ignores missing roots, dedupes canonical paths, sorts by name, skips `IGNORED_DIRS`.

### `dev_cli::ide`
- `detect::detect_ides() -> Vec<InstalledIde>` — PATH scan + Windows locations, deduped.
- `launcher::launch(ide: Ide, project: &Path) -> Result<()>` — **Errors** if IDE not detected or process spawn fails.
- `registry::InstalledIde { ide, display_name, executable }` + `InstalledIde::new(ide, &str, PathBuf)`.

### `dev_cli::installer`
- `binary_install_dir() -> Result<PathBuf>` — `DEVCLI_INSTALL_DIR` else `~/.local/bin`.
- `install() -> Result<()>`.

### `dev_cli::models`
- `ide::Ide` enum (7 variants; `ValueEnum`, serde, Copy/Clone/Eq/Debug).
- `project::Project { name, path, root, git_dir }` + `Project::new(path, root)`.

### `dev_cli::utils::path`
- `display_path(&Path) -> String` — strip `\\?\` prefix on Windows.

## Env-Var Test Hooks

| Var | Effect |
|-----|--------|
| `DEVCLI_CONFIG_DIR` | Redirect config file location (integration tests) |
| `DEVCLI_TEST_EXECUTABLE` | Bypass IDE detection in `launch()` (launcher tests) |
| `DEVCLI_INSTALL_DIR` | Redirect install destination (installer tests) |
