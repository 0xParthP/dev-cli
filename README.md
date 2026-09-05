# dev-cli

<div align="center">

![Rust](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-green)
![Status](https://img.shields.io/badge/status-Active%20Development-blue)
[![CI](https://github.com/0xParthP/dev-cli/actions/workflows/ci.yml/badge.svg)](...)
[![Coverage](https://github.com/0xParthP/dev-cli/actions/workflows/coverage.yml/badge.svg)](...)
[![Security](https://github.com/0xParthP/dev-cli/actions/workflows/security.yml/badge.svg)](...)
[![Release](https://github.com/0xParthP/dev-cli/actions/workflows/release.yml/badge.svg)](...)

**A modern developer project manager written in Rust**

Fast. Simple. Extensible.

[Getting Started](#quick-start) • [Documentation](docs/getting-started.md) • [Architecture](ARCHITECTURE.md) • [Contributing](CONTRIBUTING.md)

</div>

---

## Overview

`dev-cli` is a command-line tool that helps you manage, discover, and launch your Git repositories with speed and simplicity. Instead of manually navigating to project directories, `dev-cli` maintains a configuration of your project roots and automatically opens them in your favorite IDE.

**Key features:**
- ⚡ **Fast project discovery** — automatically find Git repositories in configured directories
- 🎯 **IDE detection** — detects VS Code, Cursor, Claude Code, Windows Terminal, and more
- 🚀 **One-command launch** — open any project in any IDE instantly
- ⚙️ **Configuration management** — TOML-based configuration for projects and defaults
- 🧙 **First-run onboarding** — interactive wizard collects project roots and default IDE on first launch
- 🔄 **Cross-platform** — built and tested on Windows, macOS, and Linux

---

## Feature Comparison

| Feature | Status | Notes |
|---------|--------|-------|
| Project discovery | ✅ Active | Configured roots + automatic Git-repo scanning |
| IDE detection | ✅ Active | 7 IDEs across PATH and standard install paths |
| Quick project launch | ✅ Active | `dev open <project>` |
| Configuration management | ✅ Active | TOML config with serde defaults |
| First-run onboarding | ✅ Active | Interactive wizard on first launch |
| Automatic repository scanning | ✅ Active | Honours `.gitignore` via the `ignore` crate |
| Git integration | 🔄 Planned | Branch + status per project |
| Project templates | 🔄 Planned | Quick scaffolding |
| Dashboard mode | 🔄 Planned | Interactive TUI |

---

## Screenshots

```
$ dev project list
Configured Project Roots
📁 C:\Users\parth\Projects
📁 C:\Users\parth\Work

Discovered Git Repositories
• dev-cli (C:\Users\parth\Projects\dev-cli)
• blog (C:\Users\parth\Projects\blog)

$ dev open dev-cli --ide cursor
Opened C:\Users\parth\Projects\dev-cli

$ dev ide list
Installed IDEs:
✓ VS Code (C:\Program Files\Microsoft VS Code\bin\code.cmd)
✓ Cursor (C:\Program Files\Cursor\Cursor.exe)
✓ Claude Code (C:\Users\parth\.local\bin\claude.exe)
```

---

## Quick Start

### Installation

#### From Release

1. Download the latest binary for your operating system
2. Store the binary securely and add it to your PATH

### First Steps

The first time you run any `dev` command, an **onboarding wizard** walks you through:

1. Choosing the directory (or directories) that hold your Git projects.
2. Picking the IDE to launch by default.

The wizard writes a `config.toml` so subsequent runs are immediate. You can re-run the same setup with:

```bash
dev config init        # write defaults if missing
dev config show        # print the active configuration
```

3. **List installed IDEs:**
   ```bash
   dev ide list
   ```

4. **Discover projects under the configured roots:**
   ```bash
   dev project list
   ```

5. **Open a project:**
   ```bash
   dev open my-project
   dev open my-project --ide cursor
   ```

---

## Command Reference

### `dev project`

Manage and launch projects.

| Command | Description |
|---------|-------------|
| `dev project list` | List configured project roots and discovered Git repos |
| `dev project open <NAME>` | Open a project in the default IDE |
| `dev project open <NAME> --ide <IDE>` | Open a project in a specific IDE |
| `dev open <NAME>` | Shorthand for `dev project open` |

**Examples:**
```bash
dev project list
dev open dev-cli
dev open dev-cli --ide vscode
```

### `dev config`

Manage development configuration.

| Command | Description |
|---------|-------------|
| `dev config show` | Display current configuration |
| `dev config init` | Write the default configuration if missing |
| `dev config set-default-ide <IDE>` | Set default IDE for launching projects |

**Examples:**
```bash
dev config show
dev config init
dev config set-default-ide cursor
```

### `dev ide`

Discover installed development environments.

| Command | Description |
|---------|-------------|
| `dev ide list` | List all detected installed IDEs |

**Examples:**
```bash
dev ide list
```

## Building from Source

### Requirements
- **Rust 1.88+ (edition 2024)** — install from [rustup.rs](https://rustup.rs)
- **Git**
- **Windows, macOS, or Linux**

### Build Steps

```bash
# Clone the repository
git clone https://github.com/0xParthP/dev-cli.git
cd dev-cli

# Build debug binary
cargo build

# Build release binary (optimized)
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

The compiled binary will be in `target/release/dev` (or `target/debug/dev.exe` on Windows).

### Pre-commit / CI Check

The same checks CI runs are bundled into `cargo xtask ci`. It runs `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and enforces an **80% line-coverage** minimum. Install the helper once and use it before every commit:

```bash
cargo install cargo-llvm-cov   # coverage backend
cargo xtask ci                 # runs the full suite
```

### Useful Cargo Commands

```bash
# Run the CLI directly
cargo run -- <COMMAND>

# Run a specific test
cargo test test_name

# Generate and view documentation
cargo doc --no-deps --open

# Check code without building
cargo check

# View project dependencies
cargo tree

# Open the HTML coverage report
cargo coverage

# Print a one-line coverage summary
cargo coverage-summary
```

---

## Project Structure

```
dev-cli/
├── src/
│   ├── main.rs           # Application entry point (thin binary)
│   ├── lib.rs            # Library crate root (shared by tests)
│   ├── cli.rs            # CLI argument parsing (Clap)
│   ├── config.rs         # Configuration management
│   ├── onboarding.rs     # First-run interactive wizard
│   ├── startup.rs        # Startup orchestration
│   ├── scanner.rs        # Repository discovery
│   ├── commands/         # Command implementations
│   ├── ide/              # IDE detection and launching
│   ├── models/           # Data models
│   └── utils/            # Shared helpers (path display)
├── tests/                # Integration tests (one file per command)
├── docs/                 # User and contributor documentation
├── xtask/                # Dev tooling (cargo xtask)
├── Cargo.toml            # Workspace manifest
├── README.md             # This file
├── ARCHITECTURE.md       # Architecture and design
├── CONTRIBUTING.md       # Contributor guide
├── CHANGELOG.md          # Version history
└── .claude/              # Agent instructions (CLAUDE.md, AGENTS.md)
```

For detailed project structure documentation, see [docs/project-structure.md](docs/project-structure.md).

---

## Documentation

Complete documentation is available in the `docs/` directory:

- **[Getting Started](docs/getting-started.md)** — Setup and first steps
- **[Project Structure](docs/project-structure.md)** — Complete file reference
- **[Architecture](ARCHITECTURE.md)** — System design and module organization
- **[Rust for dev-cli](docs/rust-for-dev-cli.md)** — Learn Rust through this project
- **[CLI Design](docs/cli-design.md)** — How the CLI parser works
- **[Configuration](docs/configuration.md)** — Configuration file format and schema
- **[IDE System](docs/ide-system.md)** — IDE detection algorithm
- **[Testing](docs/testing.md)** — Testing philosophy and practices
- **[Style Guide](docs/style-guide.md)** — Coding standards
- **[Roadmap](docs/roadmap.md)** — Future direction

### Rust Documentation

Generate and view the auto-generated Rust documentation:

```bash
cargo doc --no-deps --open
```

This opens the rustdoc site showing all APIs with examples.

---

## Architecture

`dev-cli` follows a layered architecture:

See [ARCHITECTURE.md](ARCHITECTURE.md) for more details.

---

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

### Quick Development

1. **Fork and clone** the repository
2. **Run `git config core.hooksPath .githooks` to configure pre-commit hook**
3. **Create a feature branch** — `git checkout -b feature/my-feature` (branch names must match `^(feature|fix|docs|refactor|chore)/<kebab-case>$`)
4. **Make changes** and **add tests** under `tests/` (no tests inside `src/`)
5. **Run the full check** — `cargo xtask ci`
6. **Commit with a clear message** — follow conventional commits
7. **Push and create a PR**

### Code Quality

- **Format:** `cargo fmt` (enforced via CI)
- **Lint:** `cargo clippy -- -D warnings` (enforced via CI)
- **Tests:** All tests must pass (enforced via CI)
- **Coverage:** Lines ≥ 80% via `cargo xtask ci` (enforced via CI)
- **Docs:** Public APIs must have rustdoc comments, modules must have `//!` headers
- **No `unwrap()`** in production code — use `?` with `.context(...)`

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) file for details.

---

## Support

- **Documentation:** See the [docs/](docs/) directory
- **Architecture:** Read [ARCHITECTURE.md](ARCHITECTURE.md)
- **Issues:** Report bugs on GitHub
- **Discussions:** Open a GitHub discussion for questions

---

**Built with ❤️ in Rust**