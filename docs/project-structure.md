# Project Structure

A walkthrough of the repository. For architectural reasoning, see [ARCHITECTURE.md](../ARCHITECTURE.md).

## Top-Level Layout

```
dev-cli/
├── .cargo/                # Cargo aliases (xtask, etc.)
├── .claude/               # AI assistant instructions (not user docs)
├── .github/               # CI workflows
├── .githooks/             # Local git hooks
├── docs/                  # User and developer guides
├── src/                   # Rust source
├── tests/                 # Integration tests
├── xtask/                 # Build / lint helpers (separate crate)
├── Cargo.toml
├── Cargo.lock
├── clippy.toml
├── rustfmt.toml
├── README.md
├── ARCHITECTURE.md
├── CHANGELOG.md
└── CONTRIBUTING.md
```

`.claude/` holds AI-assistant instructions and is not part of the user-facing docs.

## Source Code — `src/`

```
src/
├── main.rs            # Entry point; Cli::parse() then dispatch
├── cli.rs             # Cli, Commands, *Command, *Args
├── commands/          # One file per top-level command
│   ├── project.rs
│   ├── config.rs
│   ├── ide.rs
│   └── install.rs
├── config.rs          # Config load/save
├── ide/
│   ├── detect.rs      # PATH + standard install paths
│   ├── launcher.rs    # Process spawn
│   └── registry.rs    # InstalledIde
├── models/
│   ├── ide.rs         # Ide enum (ValueEnum)
│   └── project.rs     # Project
├── installer.rs       # `dev install` logic
└── scanner.rs         # Repository discovery
```

The split between `commands/`, `config.rs` / `ide/` / `installer.rs` / `scanner.rs`, and `models/` is the layered architecture. `models/` has no dependencies on the rest; everything else can depend on it.

### Entry Point — `main.rs`

Parses the CLI and matches on `Commands`. The match arm for each command is one line: `Commands::Foo(cmd) => commands::foo::execute(cmd)?`. The function returns `anyhow::Result<()>`.

### CLI Layer — `cli.rs`

Defines `Cli` (the top-level parser), `Commands` (the subcommand enum), and one args struct per subcommand. No logic; just structs with `#[derive(Parser)]`, `#[derive(Subcommand)]`, and `#[derive(Args)]`.

### Commands — `commands/`

Each file is a self-contained handler. `commands::project::execute(cmd)` orchestrates `Config::load`, the IDE launcher, and stdout formatting. Commands are thin — they delegate to the service layer.

### Services — `config.rs`, `ide/`, `installer.rs`, `scanner.rs`

The work happens here. `Config` reads/writes the user's TOML; `ide::detect` discovers installed IDEs; `ide::launcher` spawns them; `installer` handles `dev install`; `scanner` walks project roots for Git repos.

### Models — `models/`

Two small types: `Ide` (the supported set, with `ValueEnum` so Clap can parse `--ide cursor`) and `Project` (name + path). Anything in the codebase that needs to talk about an IDE or a project uses these.

## Tests — `tests/`

One file per top-level command. Each file uses `assert_cmd` to spawn the compiled `dev` binary and `predicates` to assert on its output. Helpers live in `tests/common/` (e.g. `temp_project` for spinning up a fake project tree).

Currently:

- `tests/config.rs` — `dev config` commands
- `tests/project.rs` — `dev project` and `dev open`
- `tests/launcher.rs` — IDE launching
- `tests/commands_install.rs`, `tests/commands_config.rs`, ... — newer per-command files
- `tests/install.rs`, `tests/main_cli.rs`, `tests/path.rs`, `tests/scanner.rs` — coverage of installer, top-level dispatch, path handling, scanner

See [testing.md](testing.md) for the patterns.

## Workspace Crate — `xtask/`

`xtask` is a small binary in its own crate that wraps common dev workflows (`cargo xtask lint`, `cargo xtask coverage`, ...). It's the recommended entry point for the checks the CI runs. See [xtask.md](xtask.md).

## Documentation — `docs/`

| File | Audience | Topic |
|------|----------|-------|
| [getting-started.md](getting-started.md) | Users | Install and first run |
| [configuration.md](configuration.md) | Users, devs | Config file format |
| [cli-design.md](cli-design.md) | Devs | Clap parser structure |
| [ide-system.md](ide-system.md) | Devs | IDE detection algorithm |
| [testing.md](testing.md) | Devs | Test patterns and rules |
| [style-guide.md](style-guide.md) | Devs | Code standards |
| [roadmap.md](roadmap.md) | Everyone | Where the project is going |
| [rust-for-dev-cli.md](rust-for-dev-cli.md) | Learners | Rust concepts used here |
| [scanner.md](scanner.md) | Devs | Repository discovery |

## Dependencies

The dependency list lives in `Cargo.toml` and is the source of truth. The current set:

- `clap` 4 — CLI parsing
- `serde`, `toml` — config persistence
- `anyhow` — error handling
- `directories` — platform paths (`%LocalAppData%` / `~/.config`)
- `which` — PATH lookup for IDEs
- `owo-colors`, `tracing` — output and logging
- `regex`, `ignore` — repo scanner

Dev: `assert_cmd`, `predicates`, `tempfile`, `assert_fs`.

## File Naming

- `*.rs` — Rust source
- `mod.rs` — module root
- `tests/<command>.rs` — integration tests for a command
- `*.toml` — Cargo or user config
- `*.md` — documentation

## See Also

- [ARCHITECTURE.md](../ARCHITECTURE.md) — Why the layers exist
- [CONTRIBUTING.md](../CONTRIBUTING.md) — How to add a new command or module
- [docs/testing.md](testing.md) — How the test files are organized
