# Contributing to dev-cli

Thanks for your interest in contributing! This guide covers the workflow, standards, and how to add new commands.

## Getting Started

### Prerequisites

- **Rust 1.88 or later** — install from [rustup.rs](https://rustup.rs)
- **Git** — install from [git-scm.com](https://git-scm.com)
- **Windows, macOS, or Linux** — all platforms supported

### Clone and Build

```bash
git clone https://github.com/yourusername/dev-cli.git
cd dev-cli
cargo build
cargo test
```

## Development Workflow

### Branch Naming

```
feature/<feature-name>     # New feature
fix/<bug-name>          # Bug fix
docs/<doc-name>         # Documentation
refactor/<area>         # Code refactor
chore/<maintenance>     # Maintenance
```

### Pre-commit Checklist

Before committing, run:

```bash
cargo fmt && cargo clippy && cargo test && cargo doc --no-deps
```

CI enforces all four. Format with rustfmt, address every clippy warning, keep tests green, and ensure rustdoc builds without warnings.

### Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types: `feature`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`. Subject is lowercase, imperative mood, max 50 characters, no trailing period.

## Code Standards

- Format with `cargo fmt` — never adjust formatting by hand.
- Follow the naming conventions in [docs/style-guide.md](docs/style-guide.md).
- Every public item must have rustdoc (`///`) including `# Errors` and `# Example` sections.
- Every file must start with a module-level `//!` comment.
- Never use `unwrap()` in production code. Use `?` with `.context()` for error propagation. `expect()` is fine only for invariants that genuinely cannot fail.
- Single files should stay under 500 lines (target 200–300). Split into submodules when they grow.

## Testing

Write tests alongside the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_default_uses_vscode() {
        assert_eq!(Config::default().default_ide, Ide::Vscode);
    }
}
```

Add integration tests in `tests/`, one file per command (`tests/config.rs`, `tests/project.rs`, `tests/launcher.rs`, etc.). Use `assert_cmd` to spawn the CLI as a subprocess and `predicates` for assertions.

For full guidance, see [docs/testing.md](docs/testing.md).

## Documentation

Every code change must update docs. This is part of "done."

| Code change | Update |
|-------------|--------|
| Add a public function | Rustdoc with `# Examples` |
| Add a new command | `README.md`, `CHANGELOG.md`, relevant `docs/` guide, rustdoc |
| Modify config | `docs/configuration.md`, `Config` rustdoc |
| Add or rename a module | `docs/project-structure.md`, `ARCHITECTURE.md` |

For the full maintenance contract, see [.claude/DOCUMENTATION-MAINTENANCE.md](.claude/DOCUMENTATION-MAINTENANCE.md).

## Adding a New Command

Walkthrough for adding a hypothetical `dev sync` command.

### 1. Define the CLI

In `src/cli.rs`, add a variant and an args struct:

```rust
#[derive(Subcommand)]
pub enum Commands {
    Project(ProjectCommand),
    Config(ConfigCommand),
    Ide(IdeCommand),
    Install,
    Open(OpenArgs),
    Sync(SyncCommand),  // ← new
}

#[derive(Args)]
pub struct SyncCommand {
    #[arg(long)]
    pub all: bool,
}
```

### 2. Implement the Handler

Create `src/commands/sync.rs`:

```rust
//! Handle the `dev sync` command.

use anyhow::Result;
use crate::cli::SyncCommand;

/// Execute the sync command.
pub fn execute(cmd: SyncCommand) -> Result<()> {
    if cmd.all {
        sync_all()
    } else {
        sync_default()
    }
}

fn sync_all() -> Result<()> { /* ... */ Ok(()) }
fn sync_default() -> Result<()> { /* ... */ Ok(()) }
```

Export it from `src/commands/mod.rs`:

```rust
pub mod config;
pub mod ide;
pub mod install;
pub mod project;
pub mod sync;  // ← new
```

### 3. Wire It Up

In `src/main.rs`, add a match arm:

```rust
match cli.command {
    // ...
    Commands::Sync(cmd) => commands::sync::execute(cmd)?,
}
```

### 4. Add Tests

Create `tests/sync.rs`:

```rust
use assert_cmd::Command;

#[test]
fn sync_runs() {
    Command::cargo_bin("dev")
        .unwrap()
        .arg("sync")
        .assert()
        .success();
}
```

### 5. Update Docs and Verify

- Add the command to `README.md`'s command reference.
- Add an entry to `CHANGELOG.md`.
- Add or update a `docs/` guide explaining the command.
- Run `cargo fmt && cargo clippy && cargo test && cargo doc --no-deps`.

## Pull Requests

1. Fetch and rebase on the latest `main`.
2. Run the full check suite (see above).
3. Push and open a PR with a clear title and description. Reference any issues it closes.
4. Address review feedback with new commits — don't force-push or rewrite history.
5. Maintainers merge after approval.

## Questions?

- 📖 [ARCHITECTURE.md](ARCHITECTURE.md) for system design
- 📖 [docs/](docs/) for detailed guides
- 💬 Open a GitHub discussion
- 🐛 Report bugs with reproduction steps
