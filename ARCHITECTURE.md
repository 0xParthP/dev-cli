# Architecture

`dev-cli` is a small, single-binary CLI written in Rust. The binary (`src/main.rs`) is intentionally thin — it parses the CLI and delegates to the `dev_cli` library crate (`src/lib.rs`). The architecture is layered so each module has one job and tests can target each layer in isolation.

---

## Layered Architecture

```
┌─────────────────────────────────────┐
│     CLI Layer (src/cli.rs)          │
│  Clap-based argument parsing        │
└────────────┬────────────────────────┘
             ↓
┌─────────────────────────────────────┐
│  Commands Layer (src/commands/*)    │
│  Command handlers & orchestration   │
└────────────┬────────────────────────┘
             ↓
┌─────────────────────────────────────┐
│  Services Layer (src/*.rs)          │
│  Business logic & system interaction │
└────────────┬────────────────────────┘
             ↓
┌─────────────────────────────────────┐
│  Models Layer (src/models/*)        │
│  Data structures (Ide, Project)     │
└─────────────────────────────────────┘
```

Dependencies flow downward only. A model never imports a service, and a service never reaches into a command handler.

### CLI Layer — `src/cli.rs`

Parses user input and validates arguments with Clap's derive macros. Defines the `Cli` top-level struct and the `Commands` enum, plus per-command args structs (`ProjectCommand`, `ConfigCommand`, `OpenArgs`, etc.). No business logic here.

### Commands Layer — `src/commands/`

One module per top-level command (`project`, `config`, `ide`). Each module exposes an `execute(...)` function that takes the parsed args struct and returns `anyhow::Result<()>`. Commands orchestrate services, format terminal output, and surface user-facing errors. The `Install` flow is handled by `cargo xtask install`.

### Services Layer — `src/config.rs`, `src/ide/`, `src/onboarding.rs`, `src/scanner.rs`, `src/startup.rs`, `src/utils/`

Holds business logic: configuration persistence, IDE detection, IDE launching, repository scanning, first-run onboarding, and cross-cutting helpers. Pure of CLI concerns — services can be exercised directly from tests via the `dev_cli` library crate.

### Models Layer — `src/models/`

Data structures shared by the layers above. `Ide` is the enum of supported IDEs (with `ValueEnum` so it can be parsed from CLI strings); `Project` holds a project name and path.

---

## Module Organization

```
src/
├── main.rs              # Thin binary: parse, dispatch
├── lib.rs               # Library crate root (re-exports for tests)
├── cli.rs               # Cli, Commands, *Command, *Args
├── commands/            # One file per command group
│   ├── project.rs
│   ├── config.rs
│   └── ide.rs
├── config.rs            # Config load/save/schema
├── onboarding.rs        # First-run interactive wizard
├── startup.rs           # Startup orchestration (onboarding gate)
├── ide/                 # IDE detection & launch
│   ├── detect.rs
│   ├── launcher.rs
│   └── registry.rs
├── models/              # Shared data types
│   ├── ide.rs
│   └── project.rs
├── scanner.rs           # Repository discovery
└── utils/               # Cross-cutting helpers
    └── path.rs          # display_path, etc.
```

The `xtask/` workspace member holds the developer tooling (`cargo xtask ci`,
`cargo xtask install`, `cargo xtask coverage`, …).

A new subcommand typically gets:

1. A new variant on `Commands` plus an args struct in `cli.rs`.
2. A new file under `commands/`, registered in `commands/mod.rs`.
3. A new entry in `src/lib.rs` so the library crate re-exports it.
4. A match arm in `main.rs`.
5. Integration tests in `tests/`, one file per command.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full walkthrough.

---

## Command Dispatch

```
cargo run -- config show
   ↓
Cli::parse()         (cli.rs)
   ↓
onboarding::ensure_onboarded()  (startup flow)
   ↓
match Commands { ... }
   ↓
commands::config::execute(ConfigCommand)  (commands/config.rs)
   ↓
Config::load()       (config.rs → filesystem)
   ↓
println!(...)        (formatted terminal output)
```

The dispatcher in `main.rs` is a single `match` on `Commands`. Each arm
calls the corresponding `commands::<group>::execute(...)` and propagates
the `Result`. The `onboarding::ensure_onboarded()` call runs **before** the
match so the wizard can populate `config.toml` on first run.

---

## Data Flow: `dev open <name>`

1. Clap parses the command into `Commands::Open(OpenArgs)`.
2. `main.rs` calls `onboarding::ensure_onboarded()` (no-op on subsequent runs).
3. `main.rs` calls `commands::project::open_shortcut(args)`.
4. The command calls `scanner::discover_projects(&config.projects_root)` to
   find the requested project by name among the configured roots.
5. The command passes the resolved path to `ide::launcher::launch`, which
   spawns the IDE process (returning once the IDE has accepted the path).
6. The command prints a confirmation or error to the terminal.

---

## IDE Detection Pipeline

Detection runs in stages and merges results, deduplicating by executable path:

1. **PATH lookup** — use the `which` crate to find `code`, `cursor`, `claude`, `wt`, and other known CLI shims. Fast and works for any user-installed CLI.
2. **Common install paths** — check the standard locations on the current platform (e.g. `%ProgramFiles%\Microsoft VS Code\bin\code.cmd`, `%LocalAppData%\Programs\Cursor\Cursor.exe`, `~/.local/bin/claude.exe`).

PATH lookup is fast and catches most user setups. The second stage is a safety net for GUI installers that don't add themselves to PATH, and lets us expand to platform-specific locations (macOS app bundles, Linux `.deb` installs) without rewriting the first stage.

---

## Configuration Lifecycle

`Config::load()` is the only entry point for reading the user config.

- If `config.toml` exists, read and parse it. A parse error is **not** fatal
  in the current build — it logs a clear message on stderr and rewrites the
  file with defaults so the next run starts fresh.
- If the file is missing, return `Config::default()` and persist it so the
  next run finds a config file.
- If both stdin and stdout are attached to a TTY **and** no config file
  exists, the **onboarding wizard** runs first and writes the config from
  user input.

```
App start
  ↓
onboarding::ensure_onboarded()
  ↓
  is TTY + no config? ── yes ──→ run wizard
  ↓ no
Config::load()
  ↓
file exists? ── no ──→ write defaults, return Config::default()
  ↓ yes
read & parse TOML
  ↓ ok
Config
  ↓ parse error
log to stderr, rewrite with defaults, return defaults
```

The default config has one `projects_root = ["~/Projects"]` and
`default_ide = "vscode"`. The path is resolved through the `directories`
crate (`%LocalAppData%\dev-cli\config\config.toml` on Windows,
`~/.config/dev-cli/config.toml` elsewhere). Tests point the loader at a
temporary directory via the `DEVCLI_CONFIG_DIR` env var.

---

## Type Glossary

| Type | Defined in | Role |
|------|-----------|------|
| `Cli` | `cli.rs` | Top-level parser struct |
| `Commands` | `cli.rs` | Enum of all top-level commands |
| `Config` | `config.rs` | User configuration (TOML) |
| `Ide` | `models/ide.rs` | Supported IDEs (ValueEnum) |
| `Project` | `models/project.rs` | Name + path |
| `InstalledIde` | `ide/registry.rs` | Detected IDE with executable path |
| `OpenArgs`, `ConfigCommand`, ... | `cli.rs` | Per-command parsed args |

---

## Design Decisions

**Clap derive macros** — declarative, compile-time validated, free `--help` and `--version`. We avoid the builder API because every command here is straightforward and the derive form is easier to read.

**TOML configuration** — human-editable, well-typed via Serde, familiar to anyone who's seen `Cargo.toml`. JSON would work but isn't as friendly in a text editor.

**No cached IDE paths in config** — re-detect on every run. Detection is fast (tens of milliseconds) and executables move or get uninstalled. Caching creates stale-config bugs without saving real time.

**Layered architecture** — each layer is independently testable. Adding a TUI or HTTP front-end later means writing a new top layer; the services stay put.

**Rust, single static binary** — fast startup (a CLI that takes a second to launch is dead on arrival), strong type system, easy distribution.

---

## Error Handling

The codebase returns `anyhow::Result<T>` from every fallible function and uses `.context("...")` to add a layer of meaning as the error propagates. `main()` returns `Result<()>` and lets `anyhow`'s default formatting render the full chain. There is no `unwrap()` in production code.

```rust
fn do_something() -> Result<()> {
    let config = Config::load()
        .context("Could not load configuration")?;
    Ok(())
}
```

---

## Testing

- **All tests live in `tests/`.** There are no `#[cfg(test)] mod tests`
  blocks in `src/`. The `dev_cli` library crate exposes the surface those
  tests need, and they `use dev_cli::…` directly.
- Tests are one file per command or service (`tests/config.rs`,
  `tests/project.rs`, `tests/launcher.rs`, `tests/onboarding.rs`, …). They
  use `assert_cmd` to spawn the compiled binary and `predicates` to assert
  on output.
- Tests that touch process-wide state (env vars like `DEVCLI_CONFIG_DIR`,
  `DEVCLI_TEST_EXECUTABLE`, `DEVCLI_SKIP_ONBOARDING`) are marked with
  `#[serial_test::serial]` so the runner never interleaves them.
- `cargo xtask ci` runs the full suite and enforces an **80% line-coverage
  minimum**.

For full coverage rules and patterns, see [docs/testing.md](docs/testing.md).

---

## Performance

| Operation | Typical time |
|-----------|--------------|
| CLI startup | < 50ms |
| Config load | 1–5ms |
| IDE detection | 10–100ms |
| Project open | 200–500ms (dominated by the IDE process itself) |

We don't micro-optimize. The single biggest cost is the IDE startup time, which we can't control. Everything else is dominated by I/O we can't avoid.

---

## Compatibility

| | |
|---|---|
| **Platforms** | Windows (primary), macOS, Linux |
| **Rust** | 1.88+ (edition 2024) |
| **IDEs** | VS Code, Cursor, Claude Code, Windows Terminal; others added via the `Ide` enum |
| **Shells** | Any — the binary is a normal executable |
| **CI matrix** | Ubuntu, Windows, macOS (latest) |

---

## See Also

- [CONTRIBUTING.md](CONTRIBUTING.md) — Workflow, standards, and how to add a new command
- [docs/project-structure.md](docs/project-structure.md) — File-by-file walkthrough
- [docs/cli-design.md](docs/cli-design.md) — Clap parser details
- [docs/rust-for-dev-cli.md](docs/rust-for-dev-cli.md) — Patterns used in the codebase
