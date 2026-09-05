# Project Structure

A walkthrough of the repository. For architectural reasoning, see [ARCHITECTURE.md](../ARCHITECTURE.md).

## Top-Level Layout

```
dev-cli/
├── .cargo/                # Cargo aliases (xtask, fmt-check, lint, ...)
├── .claude/               # AI assistant instructions (not user docs)
├── .github/               # CI workflows (ci, release, sonar, branch-name)
├── .githooks/             # Local git hooks (pre-commit)
├── docs/                  # User and developer guides
├── src/                   # Rust source (binary + library crate)
├── tests/                 # Integration tests (the only home for tests)
├── xtask/                 # Dev tooling (cargo xtask ci, install, coverage)
├── Cargo.toml             # Workspace manifest
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
├── main.rs            # Thin binary: parse, dispatch
├── lib.rs             # Library crate root (re-exports for tests)
├── cli.rs             # Cli, Commands, *Command, *Args
├── commands/          # One file per top-level command
│   ├── project.rs
│   ├── config.rs
│   └── ide.rs
├── config.rs          # Config load/save
├── onboarding.rs      # First-run interactive wizard
├── startup.rs         # ensure_onboarded() orchestration
├── ide/
│   ├── detect.rs      # PATH + standard install paths
│   ├── launcher.rs    # Process spawn
│   └── registry.rs    # InstalledIde
├── models/
│   ├── ide.rs         # Ide enum (ValueEnum)
│   └── project.rs     # Project
├── scanner.rs         # Repository discovery (via the `ignore` crate)
└── utils/             # Cross-cutting helpers
    └── path.rs        # display_path (Windows-friendly path rendering)
```

The split between `commands/`, the service modules (`config.rs`, `ide/`, `onboarding.rs`, `scanner.rs`, `startup.rs`, `utils/`), and `models/` is the layered architecture. `models/` has no dependencies on the rest; everything else can depend on it.

### Entry Point — `main.rs`

Parses the CLI, calls `onboarding::ensure_onboarded()` (no-op after the first run), then matches on `Commands`. The match arm for each command is one line: `Commands::Foo(cmd) => commands::foo::execute(cmd)?`. The function returns `anyhow::Result<()>`.

### Library Crate — `lib.rs`

`dev-cli` is a Cargo workspace with two members: the binary (`src/main.rs`) and a library crate (`src/lib.rs`) that re-exports `cli`, `commands`, `config`, `ide`, `models`, `onboarding`, `scanner`, and `utils`. The integration tests `use dev_cli::…` directly — there are no `#[cfg(test)] mod tests` blocks inside `src/`.

### CLI Layer — `cli.rs`

Defines `Cli` (the top-level parser), `Commands` (the subcommand enum), and one args struct per subcommand. No logic; just structs with `#[derive(Parser)]`, `#[derive(Subcommand)]`, and `#[derive(Args)]`.

### Commands — `commands/`

Each file is a self-contained handler. `commands::project::execute(cmd)` orchestrates `Config::load`, the IDE launcher, and stdout formatting. Commands are thin — they delegate to the service layer.

### Services — `config.rs`, `ide/`, `onboarding.rs`, `scanner.rs`, `startup.rs`, `utils/`

The work happens here. `Config` reads/writes the user's TOML; `ide::detect` discovers installed IDEs; `ide::launcher` spawns them; `onboarding` runs the first-start wizard; `scanner` walks project roots for Git repos (respecting `.gitignore` via the `ignore` crate); `startup::ensure_onboarded` orchestrates the wizard gate; `utils::path` provides `display_path` for Windows-friendly path rendering.

### Models — `models/`

Two small types: `Ide` (the supported set, with `ValueEnum` so Clap can parse `--ide cursor`) and `Project` (name + path). Anything in the codebase that needs to talk about an IDE or a project uses these.

## Tests — `tests/`

This is the **only** place tests live — no `#[cfg(test)] mod tests` inside `src/`. One file per top-level command or service. Each file uses `assert_cmd` to spawn the compiled `dev` binary and `predicates` to assert on its output. Helpers live in `tests/common/`.

Current layout:

- `tests/cli_config.rs`, `tests/cli_ide.rs`, `tests/cli_open.rs` — black-box CLI smoke tests
- `tests/commands_config.rs`, `tests/project_commands.rs` — per-command exercise
- `tests/config.rs`, `tests/scanner.rs`, `tests/launcher.rs` — service-level coverage
- `tests/ide_detect.rs` — IDE detection rules
- `tests/main_cli.rs` — top-level dispatch (help, version, errors)
- `tests/onboarding.rs` — wizard + helpers
- `tests/path.rs` — `display_path`
- `tests/project.rs` — project list / open behaviour

Tests that touch process-wide state (`DEVCLI_CONFIG_DIR`, `DEVCLI_TEST_EXECUTABLE`, `DEVCLI_SKIP_ONBOARDING`, …) are marked `#[serial_test::serial]` so the runner never interleaves them.

See [testing.md](testing.md) for the patterns.

## Workspace Crate — `xtask/`

`xtask` is a small binary in its own crate that wraps common dev workflows: `cargo xtask ci`, `cargo xtask install`, `cargo xtask coverage`, `cargo xtask coverage-summary`, `cargo xtask security`, … It's the recommended entry point for the checks CI runs. See [xtask.md](xtask.md).

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
| [xtask.md](xtask.md) | Devs | The `xtask` helper crate |

## Dependencies

The dependency list lives in `Cargo.toml` and is the source of truth. The current set:

- `clap` 4.5 — CLI parsing
- `serde`, `toml` 0.9 — config persistence
- `anyhow` 1.0 — error handling
- `directories` 6 — platform paths (`%AppData%` / `~/.config`)
- `which` 8 — PATH lookup for IDEs
- `owo-colors` 4 — terminal output
- `ignore` 0.4 — Gitignore-aware scanner
- `cliclack` 0.5, `console` 0.16 — onboarding wizard
- `tracing`, `tracing-subscriber` — logging
- `regex` (optional) — only via `cliclack` transitives today

Dev: `assert_cmd`, `predicates`, `tempfile`, `serial_test`.

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
- [docs/xtask.md](xtask.md) — The local CI helper
