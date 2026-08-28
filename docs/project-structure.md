# Project Structure

Complete reference for every file and directory in `dev-cli`.

---

## Directory Tree

```
dev-cli/
├── .cargo/                 # Cargo configuration
├── .claude/                # AI assistant configuration
├── .git/                   # Git repository
├── .github/                # GitHub files (workflows, etc.)
├── .githooks/              # Custom git hooks
├── docs/                   # Documentation guides
├── src/                    # Rust source code
├── target/                 # Build artifacts (generated)
├── tests/                  # Integration tests
├── Cargo.lock              # Dependency lock file
├── Cargo.toml              # Project manifest
├── clippy.toml             # Clippy linter configuration
├── README.md               # Project overview
├── ARCHITECTURE.md         # System design documentation
├── AGENTS.md               # Agent rules and architecture
├── CHANGELOG.md            # Version history
├── CLAUDE.md               # AI assistant instructions
├── CONTRIBUTING.md         # Contributor guide
└── rustfmt.toml            # Code formatter configuration
```

---

## Root-Level Files

### Project Configuration

| File | Purpose | Editable? |
|------|---------|-----------|
| `Cargo.toml` | Project manifest, dependencies, metadata | ✅ Yes |
| `Cargo.lock` | Locked dependency versions | ❌ Generated |
| `clippy.toml` | Linter configuration | ✅ Yes |
| `rustfmt.toml` | Code formatter configuration | ✅ Yes |
| `.github/workflows/` | CI/CD pipelines | ✅ Yes |

### Documentation

| File | Purpose | Audience |
|------|---------|----------|
| `README.md` | Project overview, quick start | Everyone |
| `ARCHITECTURE.md` | System design, layers, modules | Developers |
| `CONTRIBUTING.md` | Development workflow, standards | Contributors |
| `CHANGELOG.md` | Version history, features | Users, developers |
| `CLAUDE.md` | AI assistant instructions | AI tools |
| `AGENTS.md` | Architecture rules, invariants | AI agents, reviewers |

### Configuration Files

| File | Purpose |
|------|---------|
| `clippy.toml` | Linting rules |
| `rustfmt.toml` | Formatting rules |
| `.gitignore` | Files to ignore in git |

---

## Source Code (`src/`)

### Entry Point

**File:** `src/main.rs`  
**Size:** ~50 lines  
**Purpose:** Application entry point; CLI dispatch

**Key Responsibilities:**
- Initialize logging
- Parse CLI arguments via Clap
- Dispatch to appropriate command
- Handle top-level errors

**Key Code:**
```rust
fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Project(cmd) => commands::project::execute(cmd)?,
        // ...
    }
    Ok(())
}
```

**Used by:** Everything (is the entry point)  
**Uses:** `cli`, `commands`

### CLI Layer

**File:** `src/cli.rs`  
**Size:** ~200 lines  
**Purpose:** Define command-line arguments (Clap-based)

**Key Types:**
- `Cli` — top-level command parser
- `Commands` — enum of all available commands
- `ProjectCommand`, `ConfigCommand`, `IdeCommand` — subcommand groups
- `OpenArgs`, `ProjectCommand` — command arguments
- `ProjectSubcommand`, `ConfigSubcommand`, `IdeSubcommand` — subcommand variants

**Key Derives:**
- `#[derive(Parser)]` — Clap parser derive
- `#[derive(Subcommand)]` — For command enums
- `#[derive(Args)]` — For argument structs

**Used by:** `main.rs`  
**Uses:** `models/ide.rs`, Clap crate

**Example:**
```rust
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Project(ProjectCommand),
    Config(ConfigCommand),
    // ...
}
```

### Configuration

**File:** `src/config.rs`  
**Size:** ~150 lines  
**Purpose:** Load, save, and manage user configuration

**Key Types:**
- `Config` — user configuration struct
  - `projects_root: Vec<PathBuf>` — directories to search for projects
  - `default_ide: Ide` — default IDE for opening projects

**Key Functions:**
- `Config::load() -> Result<Config>` — Load from disk (or create default)
- `Config::save(&self) -> Result<()>` — Save to disk
- `Config::path() -> Result<PathBuf>` — Get config file path (platform-aware)
- `Config::default()` — Create default configuration

**Configuration Location:**
- Windows: `C:\Users\{user}\AppData\Local\dev-cli\config\config.toml`
- macOS/Linux: `~/.config/dev-cli/config.toml`

**Used by:** `commands/project.rs`, `commands/config.rs`  
**Uses:** `serde`, `toml`, `directories`, `models`

### Models

**Directory:** `src/models/`  
**Purpose:** Define data structures used throughout the app

#### `src/models/mod.rs`
**Size:** < 10 lines  
**Purpose:** Module definition; re-export public types

```rust
pub mod ide;
pub mod project;
```

#### `src/models/ide.rs`
**Size:** ~15 lines  
**Purpose:** Define IDE enum

**Key Type:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum Ide {
    Cursor,
    Vscode,
    Claude,
    Terminal,
    Idea,
    Rider,
    Zed,
}
```

**Derives:**
- `ValueEnum` — Enables parsing from CLI strings
- `Serialize, Deserialize` — TOML serialization
- `Copy, Clone` — Cheap copying

**Used by:** `cli.rs`, `config.rs`, `commands/*`, `ide/*`

#### `src/models/project.rs`
**Size:** ~20 lines  
**Purpose:** Define Project type (placeholder)

**Key Type:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
}
```

**Used by:** (future) `scanner.rs`, `commands/project.rs`

### Commands

**Directory:** `src/commands/`  
**Purpose:** Implement command handlers

#### `src/commands/mod.rs`
**Size:** ~10 lines  
**Purpose:** Module definition

```rust
pub mod config;
pub mod ide;
pub mod install;
pub mod project;
```

#### `src/commands/project.rs`
**Size:** ~100 lines  
**Purpose:** Handle `dev project` and `dev open` commands

**Key Functions:**
- `execute(cmd: ProjectCommand) -> Result<()>` — Dispatcher
- `open(args: OpenArgs) -> Result<()>` — Implement `dev open`
- `list() -> Result<()>` — Implement `dev project list`
- `open_shortcut(args: OpenArgs) -> Result<()>` — Shorthand for `dev open`

**Handles:**
- `dev project list` — Display configured project roots
- `dev project open <NAME> [--ide IDE]` — Open project in IDE
- `dev open <NAME> [--ide IDE]` — Shorthand

**Used by:** `main.rs`  
**Uses:** `config`, `ide/launcher`, `cli::OpenArgs`

#### `src/commands/config.rs`
**Size:** ~100 lines  
**Purpose:** Handle `dev config` commands

**Key Functions:**
- `execute(cmd: ConfigCommand) -> Result<()>` — Dispatcher

**Handles:**
- `dev config show` — Display configuration
- `dev config init` — Initialize default config
- `dev config set-default-ide <IDE>` — Update default IDE

**Used by:** `main.rs`  
**Uses:** `config::Config`

#### `src/commands/ide.rs`
**Size:** ~50 lines  
**Purpose:** Handle `dev ide` commands

**Key Functions:**
- `execute(cmd: IdeCommand) -> Result<()>` — Dispatcher

**Handles:**
- `dev ide list` — List detected IDEs

**Used by:** `main.rs`  
**Uses:** `ide/detect`, `ide/registry`

#### `src/commands/install.rs`
**Size:** ~50 lines  
**Purpose:** Handle `dev install` command

**Key Functions:**
- `execute() -> Result<()>` — Implement installation

**Handles:**
- `dev install` — Copy executable to global location

**Used by:** `main.rs`  
**Uses:** `installer`, `config`

### IDE System

**Directory:** `src/ide/`  
**Purpose:** IDE detection and project launching

#### `src/ide/mod.rs`
**Size:** ~10 lines  
**Purpose:** Module definition

```rust
pub mod detect;
pub mod launcher;
pub mod registry;
```

#### `src/ide/detect.rs`
**Size:** ~100 lines  
**Purpose:** Discover installed IDEs

**Key Functions:**
- `detect_ides() -> Vec<InstalledIde>` — Detect all installed IDEs
- `detect_cli(list, ide, name, cmd)` — Check if CLI command in PATH
- `detect_common_windows_locations(list)` — Check standard install paths

**Algorithm:**
1. Check PATH for CLI commands (`code`, `cursor`, `claude`, `wt`)
2. Check common Windows installation directories
3. Return deduplicated list of found IDEs

**Used by:** `commands/ide.rs`  
**Uses:** `which`, `directories`, `ide/registry`

#### `src/ide/launcher.rs`
**Size:** ~100 lines  
**Purpose:** Spawn external IDE process

**Key Functions:**
- `launch(ide: Ide, path: &Path) -> Result<()>` — Launch IDE with project

**Behavior:**
- Spawns external process (does not wait)
- Handles platform-specific launch commands
- Maps IDE enum to executable name

**Used by:** `commands/project.rs`  
**Uses:** `std::process::Command`, `ide/registry`

#### `src/ide/registry.rs`
**Size:** ~50 lines  
**Purpose:** Store information about detected IDE

**Key Type:**
```rust
pub struct InstalledIde {
    pub ide: Ide,
    pub name: String,
    pub path: PathBuf,
}
```

**Key Functions:**
- `InstalledIde::new(ide, name, path) -> Self` — Create instance

**Used by:** `ide/detect.rs`, `commands/ide.rs`

### Installer

**File:** `src/installer.rs`  
**Size:** ~50 lines  
**Purpose:** Handle global installation (`dev install`)

**Key Functions:**
- `install() -> Result<()>` — Copy executable and setup PATH

**Behavior:**
- Copies current executable to `~/.local/bin/dev`
- Initializes config if needed
- Prints PATH addition instructions

**Used by:** `commands/install.rs`  
**Uses:** `directories`, `config::Config`

### Scanner

**File:** `src/scanner.rs`  
**Size:** ~10 lines  
**Purpose:** Placeholder for repository discovery (Sprint 2+)

**Current State:**
- Stub implementation
- Documents intended behavior
- Placeholder for future development

**Future Purpose:**
- Scan project roots for Git repositories
- Extract project metadata
- Cache results

**Used by:** (future) `commands/project.rs`  
**Uses:** (future) `ignore` crate

---

## Tests (`tests/`)

**Location:** `tests/` directory  
**Type:** Integration tests using Clap subprocess execution

### `tests/cli_config.rs`
**Size:** ~30 lines  
**Purpose:** Test `dev config` commands

**Tests:**
- `config_show_runs` — Verify `dev config show` succeeds

**Framework:** `assert_cmd`, `predicates`

### `tests/cli_ide.rs`
**Size:** ~30 lines  
**Purpose:** Test `dev ide` commands

**Tests:**
- `ide_list_runs` — Verify `dev ide list` succeeds

**Framework:** `assert_cmd`, `predicates`

### `tests/cli_open.rs`
**Size:** ~30 lines  
**Purpose:** Test `dev open` commands

**Tests:**
- `open_project_not_found` — Verify error handling

**Framework:** `assert_cmd`, `predicates`

---

## Documentation (`docs/`)

**Location:** `docs/` directory  
**Type:** Markdown guides and tutorials

### Guides

| File | Audience | Topic |
|------|----------|-------|
| `getting-started.md` | Users | Installation and setup |
| `project-structure.md` | Developers | File reference (this file) |
| `rust-for-dev-cli.md` | Learners | Rust concepts with examples |
| `cli-design.md` | Developers | CLI architecture and parsing |
| `configuration.md` | Users, Developers | Config file format |
| `ide-system.md` | Developers | IDE detection algorithm |
| `testing.md` | Developers | Testing philosophy |
| `style-guide.md` | Developers | Code standards |
| `roadmap.md` | Everyone | Future direction |

### mdBook

**Location:** `docs/book/`  
**Type:** Complete reference manual

- `book.toml` — mdBook configuration
- `src/SUMMARY.md` — Table of contents
- `src/01-*.md` through `src/06-*.md` — Chapters

---

## Dependencies

### Core Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `clap` | 4.5 | CLI argument parsing |
| `serde` | 1.0 | Serialization framework |
| `toml` | 0.9 | TOML format support |
| `directories` | 6 | Platform-aware paths |
| `anyhow` | 1.0 | Error handling |
| `which` | 8 | Find executables in PATH |
| `owo-colors` | 4 | Terminal colors |
| `regex` | 1.0 | Pattern matching |
| `ignore` | 0.4 | .gitignore support |
| `tracing` | 0.1 | Logging framework |

### Dev Dependencies

| Crate | Purpose |
|-------|---------|
| `assert_cmd` | CLI testing |
| `predicates` | Test assertions |
| `tempfile` | Temporary files |
| `assert_fs` | Filesystem testing |

---

## File Naming Conventions

| Pattern | Meaning | Example |
|---------|---------|---------|
| `.rs` | Rust source file | `main.rs`, `config.rs` |
| `/mod.rs` | Module definition | `commands/mod.rs` |
| `cli_*.rs` | Integration test | `tests/cli_config.rs` |
| `*.toml` | TOML configuration | `Cargo.toml`, `config.toml` |
| `.md` | Markdown documentation | `README.md`, `ARCHITECTURE.md` |

---

## Module Dependency Graph

```
                    main.rs
                      │
                    cli.rs
                      │
        ┌─────────────┼─────────────┐
        │             │             │
    commands/      (none)         ide/*
    project.rs                    /   \
    config.rs              detect.rs launcher.rs
    ide.rs           registry.rs
    install.rs


    config.rs  ←─────┐
                     ├─ models/ide.rs
    scanner.rs ←─────┤
                     ├─ models/project.rs
    installer.rs ←───┘
```

**Rule:** Arrows point downward only (no circular or upward dependencies).

---

## Future Structure (Sprint 2+)

```
src/
├── git/              # (new) Git integration
│   ├── mod.rs
│   ├── status.rs
│   └── branch.rs
│
├── dashboard/        # (new) TUI/interactive mode
│   ├── mod.rs
│   ├── ui.rs
│   └── events.rs
│
└── cache/            # (new) Caching layer
    ├── mod.rs
    └── store.rs
```

---

## Size Summary

| Component | Files | Lines |
|-----------|-------|-------|
| Source code | 12 | ~1000 |
| Tests | 3 | ~100 |
| Documentation | 10 | ~5000 |
| Total | ~25 | ~6000 |

---

## See Also

- [ARCHITECTURE.md](../ARCHITECTURE.md) — System design
- [docs/getting-started.md](../docs/getting-started.md) — User guide
- [CONTRIBUTING.md](../CONTRIBUTING.md) — Development workflow
