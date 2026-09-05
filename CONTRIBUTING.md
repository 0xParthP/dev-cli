# Contributing to dev-cli

Thanks for your interest in contributing! This guide covers the workflow, standards, and how to add new commands.

## Getting Started

### Prerequisites

- **Rust 1.88 or later** (edition 2024) — install from [rustup.rs](https://rustup.rs)
- **Git** — install from [git-scm.com](https://git-scm.com)
- **Windows, macOS, or Linux** — all platforms supported
- **`cargo-llvm-cov`** — required to run the coverage gate locally
  (`cargo install cargo-llvm-cov`)

### Clone and Build

```bash
git clone https://github.com/0xParthP/dev-cli.git
cd dev-cli
cargo build
cargo test
```

## Development Workflow

### Branch Naming

The pre-commit hook and the `branch-name.yml` workflow both reject names
that don't match this pattern. Keep names lowercase, kebab-cased, and
prefixed with one of the allowed categories:

```
feature/<kebab-case>   # New feature
fix/<kebab-case>       # Bug fix
docs/<kebab-case>      # Documentation
refactor/<kebab-case>  # Code refactor
chore/<kebab-case>     # Maintenance
```

Examples: `feature/repository-scanner`, `fix/install-paths`,
`docs/update-config-guide`. `main` is exempt from the rule.

### Pre-commit Checklist

Before committing, run:

```bash
cargo fmt
cargo xtask ci
```

The `xtask ci` command runs the formatter, `cargo fmt --check`, `cargo clippy -- -D warnings`,
`cargo test`, and the 80% line-coverage gate. CI runs the same suite, so a
green local run is a green PR. If you want to run the steps individually:

```bash
cargo fmt-check
cargo lint
cargo test-all
cargo coverage-summary
cargo doc --no-deps
```

### Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`.
Subject is lowercase, imperative mood, max 50 characters, no trailing
period. The previous style used `feature/<name>` in branch prefixes; in
commit-message scopes we use `feat` (matching the Conventional Commits
convention).

## Code Standards

- Format with `cargo fmt` — never adjust formatting by hand.
- Follow the naming conventions in [docs/style-guide.md](docs/style-guide.md).
- Every public item must have rustdoc (`///`) including `# Errors` and
  `# Example` sections.
- Every file must start with a module-level `//!` comment.
- Never use `unwrap()` in production code. Use `?` with `.context()` for
  error propagation. `expect()` is fine only for invariants that genuinely
  cannot fail.
- Single files should stay under 500 lines (target 200–300). Split into
  submodules when they grow.

## Testing

Tests live in **one** place: `tests/`. Do not add `#[cfg(test)] mod tests`
blocks inside `src/` — the test suite is fully external and exercises the
public surface of the `dev_cli` library crate. A typical round of changes
adds one integration test file per new command or service, plus per-test
helpers under `tests/common/`.

When a test sets environment variables (`DEVCLI_CONFIG_DIR`,
`DEVCLI_TEST_EXECUTABLE`, `DEVCLI_SKIP_ONBOARDING`, …) it must run
**serially** with the rest of the suite. Mark it with
`#[serial_test::serial]` so the test runner doesn't interleave env-var
mutations across threads:

```rust
use serial_test::serial;

#[test]
#[serial]
fn onboard_writes_default_config() {
    std::env::set_var("DEVCLI_CONFIG_DIR", /* temp dir */);
    // ...
}
```

For full guidance on the suite layout and conventions, see
[docs/testing.md](docs/testing.md).

## Documentation

Every code change must update docs. This is part of "done."

| Code change | Update |
|-------------|--------|
| Add a public function | Rustdoc with `# Examples` |
| Add a new command | `README.md`, `CHANGELOG.md`, relevant `docs/` guide, rustdoc |
| Modify config | `docs/configuration.md`, `Config` rustdoc |
| Add or rename a module | `docs/project-structure.md`, `ARCHITECTURE.md` |

For the full maintenance contract, see
[.claude/DOCUMENTATION-MAINTENANCE.md](.claude/DOCUMENTATION-MAINTENANCE.md).

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

The library crate is what integration tests use, so it must re-export the
new `cli::*` types and the new `commands::sync` module from `src/lib.rs`.

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

If the test mutates env vars, mark it with `#[serial]` so it runs alone.

### 5. Update Docs and Verify

- Add the command to `README.md`'s command reference.
- Add an entry to `CHANGELOG.md`.
- Add or update a `docs/` guide explaining the command.
- Run `cargo xtask ci`.

## Pull Requests

1. Fetch and rebase on the latest `main`.
2. Run `cargo xtask ci` and confirm the coverage gate passes.
3. Push and open a PR with a clear title and description. Reference any
   issues it closes.
4. Address review feedback with new commits — don't force-push or rewrite
   history.
5. Maintainers merge after approval.

## Questions?

- 📖 [ARCHITECTURE.md](ARCHITECTURE.md) for system design
- 📖 [docs/](docs/) for detailed guides
- 💬 Open a GitHub discussion
- 🐛 Report bugs with reproduction steps
