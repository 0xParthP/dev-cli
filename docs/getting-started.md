# Getting Started with dev-cli

Welcome to `dev-cli`! This guide will help you install, configure, and use the tool effectively.

---

## Table of Contents

1. [Installation](#installation)
2. [First-Run Onboarding](#first-run-onboarding)
3. [Configuration](#configuration)
4. [First Steps](#first-steps)
5. [Common Tasks](#common-tasks)
6. [Troubleshooting](#troubleshooting)
7. [Platform-Specific Notes](#platform-specific-notes)
8. [Next Steps](#next-steps)

---

## Installation

### Requirements

- **Windows, macOS, or Linux**
- **Rust 1.88+** (only if building from source; edition 2024)
- **`cargo-llvm-cov`** (only required for the local coverage gate)
- **Git** (for development)

### Option 1: Build from Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/0xParthP/dev-cli.git
cd dev-cli

# Build the release binary
cargo build --release

# Binary located at: target/release/dev (or target/release/dev.exe on Windows)
```

### Option 2: Download Pre-built Binary

Download the binary for your operating system and add it to your PATH.

### Verify Installation

```bash
dev --version
dev --help
```

`dev --help` shows a one-line usage header followed by the subcommand list and the global options — pure `clap`-generated output, no business logic in the binary.

---

## First-Run Onboarding

When you run **any** `dev` command for the first time in an interactive terminal, the **onboarding wizard** launches. It asks you to:

1. Choose one or more directories that contain your Git projects.
2. Pick the IDE you want to launch by default.

The wizard writes the answers to `config.toml` and is then skipped on subsequent runs. In CI, tests, or any non-interactive shell the wizard is bypassed and a default config is written instead. You can force-bypass the wizard for a single run with `DEVCLI_SKIP_ONBOARDING=1`.

Re-run the same setup at any time with:

```bash
dev config init        # writes defaults if missing
dev config show        # prints the active configuration
```

---

## Configuration

### Where the Config File Lives

- **Windows:** `C:\Users\{YourName}\AppData\Roaming\dev-cli\config\config.toml`
- **macOS:** `~/Library/Application Support/dev-cli/config/config.toml`
- **Linux:** `~/.config/dev-cli/config.toml`

### File Format

```toml
projects_root = [
    "C:/Users/parth/Projects",
    "C:/Users/parth/Work",
    "C:/Users/parth/Side",
]

default_ide = "cursor"
```

### Edit the Config

Three options:

1. **Edit the file directly.** It is a plain TOML file; save and run `dev config show` to confirm.
2. **Use the CLI.** `dev config set-default-ide cursor` writes the change for you.
3. **Reset.** Delete the file and let `dev` recreate it. A missing file is non-fatal — `Config::load()` writes defaults. A parse error is also non-fatal: a message is logged on stderr and a fresh default config is written in its place.

---

## First Steps

### 1. List Configured Project Roots and Discovered Repositories

```bash
dev project list
```

Shows the configured `projects_root` directories **and** the Git repositories the scanner discovered under them.

```
Configured Project Roots
📁 C:\Users\parth\Projects
📁 C:\Users\parth\Work

Discovered Git Repositories
• dev-cli (...Projects\dev-cli)
• blog (...Projects\blog)
```

### 2. Detect Installed IDEs

```bash
dev ide list
```

Lists every IDE detected on your system with its full path.

```
Installed IDEs:
✓ VS Code (C:\Program Files\Microsoft VS Code\bin\code.cmd)
✓ Cursor (C:\Program Files\Cursor\Cursor.exe)
✓ Claude Code (C:\Users\parth\.local\bin\claude.exe)
✓ Windows Terminal (wt)
```

### 3. Open Your First Project

```bash
# Opens the first repository named "dev-cli" under your projects_root
dev open dev-cli
```

This launches the project in your default IDE.

### 4. Open in a Specific IDE

```bash
dev open dev-cli --ide cursor
```

**Available IDE identifiers** (parsed by `Ide`'s `ValueEnum`):

- `vscode` — VS Code
- `cursor` — Cursor
- `claude` — Claude Code
- `terminal` — Windows Terminal
- `idea` — IntelliJ IDEA
- `rider` — JetBrains Rider
- `zed` — Zed Editor

> **Note:** VS Code, Cursor, Claude Code, and Windows Terminal have launch arms in the IDE launcher today. `idea`, `rider`, and `zed` are recognised by the enum so the CLI accepts them, but launching them currently surfaces an "unsupported IDE" error.

---

## Common Tasks

### Configure Your Project Directories

Edit `config.toml` to point to your actual project folders:

```toml
projects_root = [
    "C:/Users/parth/Projects",
    "C:/Users/parth/Work",
]
```

Now `dev` searches these directories for projects.

### Set Your Default IDE

```bash
dev config set-default-ide cursor
```

All `dev open <project>` commands now use Cursor by default.

### Open Without Specifying an IDE

```bash
dev open dev-cli
```

Uses your configured default IDE.

### Override the IDE for One Project

```bash
dev open dev-cli --ide vscode
```

Uses VS Code for this project only; the default is unchanged.

### View Your Current Configuration

```bash
dev config show
```

Displays the active configuration.

### Run the Full Local CI Check

Before opening a PR, mirror what CI will run:

```bash
cargo xtask ci
```

This runs `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and the 80% line-coverage gate. See [docs/xtask.md](xtask.md).

---

## Troubleshooting

### "Project not found" Error

```bash
dev open UnknownProject
# Error: Project 'UnknownProject' not found.
```

**Solution:**

1. Check the name is spelled correctly.
2. Verify the project is in one of your `projects_root` directories and contains a `.git` entry — the scanner only finds Git repositories.
3. Run `dev project list` to see configured roots and discovered repos.
4. Add the directory to `config.toml` if needed.

### IDE Not Detected

```bash
dev ide list
# Shows fewer IDEs than expected
```

**Solution:**

1. Confirm the IDE is actually installed.
2. Try restarting your terminal/shell so `PATH` updates are picked up.
3. The IDE may be in a non-standard location — install its CLI shim and put it on `PATH`. The detector tries `which` first, then platform-standard install locations.

### Configuration File Issues

```bash
dev config show
# Error: Configuration file is not valid TOML
```

**Solution:**

1. Check the syntax with an online TOML validator.
2. As a last resort, delete the file and let `dev` recreate it. A parse error is **not** fatal — `dev` logs a clear message on stderr and writes a fresh config.
3. Check permissions on the config directory.

### Slow Performance

If `dev` is noticeably slow:

1. **IDE detection slow:** Expected if `PATH` is very long. Detection runs on every invocation by design (no cache).
2. **Config load slow:** Shouldn't happen (< 10 ms).
3. **Project open slow:** Mostly waiting for the IDE to start (out of our control).
4. **Scanner slow on huge trees:** Honours `.gitignore` via the `ignore` crate, so `target/`, `node_modules/`, etc. are skipped automatically.

---

## Platform-Specific Notes

### Windows

**IDEs:**

- VS Code is auto-detected from the standard install location.
- Cursor is auto-detected from the standard install location.
- Windows Terminal is detected via `PATH`.
- For custom locations, install the IDE's CLI shim and add it to `PATH`.

**PowerShell Note:**
If using PowerShell, you may need to allow execution of scripts:
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### macOS

**Installation:**
After building, copy the binary to `PATH`:
```bash
cp target/release/dev /usr/local/bin/
```

**IDEs:**

- Applications are auto-detected from `/Applications`.
- CLI tools from `PATH`.

### Linux

**Installation:**
```bash
cp target/release/dev ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"  # Add to ~/.bashrc or .zshrc
```

**Permissions:**
```bash
chmod +x ~/.local/bin/dev
```

**IDEs:**

- Detected from standard locations.
- Add custom locations via `PATH` or `config.toml`.

---

## Next Steps

1. **Explore the full documentation:** See [docs/](../docs/) directory.
2. **Understand the architecture:** Read [ARCHITECTURE.md](../ARCHITECTURE.md).
3. **Configure for your workflow:** Edit `config.toml`.
4. **Create aliases (optional):**
   ```bash
   # Add to .bashrc or .zshrc
   alias do="dev open"
   ```
5. **Learn about upcoming features:** Read [docs/roadmap.md](../docs/roadmap.md).
6. **Contribute:** Read [CONTRIBUTING.md](../CONTRIBUTING.md).

---

## Getting Help

- 📖 **Guides:** Read the [docs/](../docs/) directory
- 🏗️ **Architecture:** Check [ARCHITECTURE.md](../ARCHITECTURE.md)
- 🤝 **Contributing:** See [CONTRIBUTING.md](../CONTRIBUTING.md)
- 🐛 **Report Issues:** Open a GitHub issue
- 💬 **Discuss:** Start a GitHub discussion

---

**Happy project management! 🚀**
