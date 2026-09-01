# Architecture

`dev-cli` is a small, single-binary CLI written in Rust. The architecture is intentionally layered so each module has one job and tests can target each layer in isolation.

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

One module per top-level command (`project`, `config`, `ide`, `install`, plus subcommand files). Each module exposes an `execute(...)` function that takes the parsed args struct and returns `anyhow::Result<()>`. Commands orchestrate services, format terminal output, and surface user-facing errors.

### Services Layer — `src/config.rs`, `src/ide/`, `src/installer.rs`, `src/scanner.rs`

Holds business logic: configuration persistence, IDE detection, IDE launching, repository scanning, installation. Pure of CLI concerns — services can be exercised directly from tests.

### Models Layer — `src/models/`

Data structures shared by the layers above. `Ide` is the enum of supported IDEs (with `ValueEnum` so it can be parsed from CLI strings); `Project` holds a project name and path.

---

## Module Organization

```
src/
├── main.rs              # Entry point; dispatches to commands
├── cli.rs               # Cli, Commands, *Command, *Args
├── commands/            # One file per command group
│   ├── project.rs
│   ├── config.rs
│   ├── ide.rs
│   └── install.rs
├── config.rs            # Config load/save, schema
├── ide/                 # IDE detection & launch
│   ├── detect.rs
│   ├── launcher.rs
│   └── registry.rs
├── models/              # Shared data types
│   ├── ide.rs
│   └── project.rs
├── installer.rs         # Install command logic
└── scanner.rs           # Repository discovery
```

A new subcommand typically gets:

1. A new variant on `Commands` plus an args struct in `cli.rs`.
2. A new file under `commands/`, registered in `commands/mod.rs`.
3. A match arm in `main.rs`.
4. Integration tests in `tests/`, one file per command.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full walkthrough.

---

## Command Dispatch

```
cargo run -- config show
   ↓
Cli::parse()         (cli.rs)
   ↓
match Commands { ... }
   ↓
commands::config::execute(ConfigCommand)  (commands/config.rs)
   ↓
Config::load()       (config.rs → filesystem)
   ↓
println!(...)        (formatted terminal output)
```

The dispatcher in `main.rs` is a single `match` on `Commands`. Each arm calls the corresponding `commands::<group>::execute(...)` and propagates the `Result`.

---

## Data Flow: `dev open <name>`

1. Clap parses the command into `Commands::Open(OpenArgs)`.
2. `main.rs` calls `commands::project::execute(...)`.
3. The command loads the config, looks up the project under `projects_root`, and asks the IDE layer to launch it.
4. `ide::launcher` spawns the IDE process (returning once the IDE has accepted the path).
5. The command prints a confirmation or error to the terminal.

---

## IDE Detection Pipeline

Detection runs in stages and merges results, deduplicating by executable path:

1. **PATH lookup** — use the `which` crate to find `code`, `cursor`, `claude`, `wt`, and other known CLI shims. Fast and works for any user-installed CLI.
2. **Common install paths** — check the standard locations on the current platform (e.g. `%ProgramFiles%\Microsoft VS Code\bin\code.cmd`, `%LocalAppData%\Programs\Cursor\Cursor.exe`, `~/.local/bin/claude.exe`).

PATH lookup is fast and catches most user setups. The second stage is a safety net for GUI installers that don't add themselves to PATH, and lets us expand to platform-specific locations (macOS app bundles, Linux `.deb` installs) without rewriting the first stage.

---

## Configuration Lifecycle

`Config::load()` is the only entry point for reading the user config.

- If `config.toml` exists, read and parse it. A parse error is propagated to the user.
- If the file is missing, return `Config::default()` and persist it so the next run finds a config file.

```
App start
  ↓
Config::load()
  ↓
file exists? ── no ──→ write defaults, return Config::default()
  ↓ yes
read & parse TOML
  ↓
Config
```

The default config has one `projects_root = ["~/Projects"]` and `default_ide = "vscode"`. The path is resolved through the `directories` crate (`%LocalAppData%\dev-cli\config\config.toml` on Windows, `~/.config/dev-cli/config.toml` elsewhere).

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

- **Unit tests** live next to the code in `#[cfg(test)] mod tests` blocks.
- **Integration tests** live in `tests/`, one file per command (`tests/config.rs`, `tests/project.rs`, `tests/launcher.rs`, ...). They use `assert_cmd` to spawn the compiled binary and `predicates` to assert on output.

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

---

## See Also

- [CONTRIBUTING.md](CONTRIBUTING.md) — Workflow, standards, and how to add a new command
- [docs/project-structure.md](docs/project-structure.md) — File-by-file walkthrough
- [docs/cli-design.md](docs/cli-design.md) — Clap parser details
- [docs/rust-for-dev-cli.md](docs/rust-for-dev-cli.md) — Patterns used in the codebase
