# Getting Started with dev-cli

Welcome to `dev-cli`! This guide will help you install, configure, and use the tool effectively.

---

## Table of Contents

1. [Installation](#installation)
2. [Initial Configuration](#initial-configuration)
3. [First Steps](#first-steps)
4. [Common Tasks](#common-tasks)
5. [Troubleshooting](#troubleshooting)
6. [Platform-Specific Notes](#platform-specific-notes)
7. [Next Steps](#next-steps)

---

## Installation

### Requirements

- **Windows, macOS, or Linux**
- **Rust 1.88+** (only if building from source)
- **Git** (for development)

### Option 1: Build from Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/yourusername/dev-cli.git
cd dev-cli

# Build release binary
cargo build --release

# Binary is at: target/release/dev (or target/release/dev.exe on Windows)
```

### Option 2: Download Pre-built Binary

(Coming soon — pre-built binaries will be available on GitHub releases)

### Option 3: Install Globally (Windows)

If you already have `dev` installed:

```bash
dev install
```

This copies the executable to `~/.local/bin/` and prints instructions for adding it to PATH.

### Verify Installation

```bash
dev --version
dev --help
```

---

## Initial Configuration

### Automatic Setup

When you run `dev` for the first time, it automatically creates a configuration file with defaults:

```bash
dev config show
```

**Output:**
```
projects_root = ["C:\\Users\\YourName\\Projects"]
default_ide = "vscode"
```

### Manual Setup (Optional)

To initialize with defaults explicitly:

```bash
dev config init
```

This creates the configuration file if it doesn't exist.

### Configuration Location

The configuration file is stored at a platform-specific location:

- **Windows:** `C:\Users\{YourName}\AppData\Local\dev-cli\config\config.toml`
- **macOS:** `~/.config/dev-cli/config.toml`
- **Linux:** `~/.config/dev-cli/config.toml`

You can edit this file directly:

```toml
# ~/.config/dev-cli/config.toml

projects_root = [
    "C:/Users/parth/Projects",
    "C:/Users/parth/Work",
    "C:/Users/parth/Side"
]

default_ide = "cursor"
```

---

## First Steps

### 1. List Configured Project Directories

```bash
dev project list
```

Shows all directories where `dev` will search for projects.

### 2. Detect Installed IDEs

```bash
dev ide list
```

Lists all IDEs detected on your system with their full paths.

**Sample output:**
```
Installed IDEs:
✓ VS Code (C:\Program Files\Microsoft VS Code\bin\code.cmd)
✓ Cursor (C:\Program Files\Cursor\Cursor.exe)
✓ Claude Code (C:\Users\parth\.local\bin\claude.exe)
✓ Windows Terminal (wt)
```

### 3. Open Your First Project

```bash
# Assuming you have a project called "MyProject"
# in your Projects directory

dev open MyProject
```

This opens `MyProject` in your default IDE.

### 4. Open in a Specific IDE

```bash
dev open MyProject --ide cursor
```

**Available IDE identifiers:**
- `vscode` — VS Code
- `cursor` — Cursor
- `claude` — Claude Code
- `terminal` — Windows Terminal
- `idea` — IntelliJ IDEA
- `rider` — JetBrains Rider
- `zed` — Zed Editor

---

## Common Tasks

### Configure Your Project Directories

Edit `config.toml` to point to your actual project folders:

```toml
projects_root = [
    "C:/Users/parth/Projects",
    "C:/Users/parth/Work"
]
```

Now `dev` will search these directories for projects.

### Set Your Default IDE

```bash
dev config set-default-ide cursor
```

Now all `dev open <project>` commands use Cursor by default.

### Open Projects Without Specifying IDE

```bash
dev open MyProject
```

Uses your configured default IDE.

### Use Different IDE for One Project

```bash
dev open MyProject --ide vscode
```

Uses VS Code for this one project, but doesn't change your default.

### View Your Current Configuration

```bash
dev config show
```

Displays the complete active configuration.

---

## Troubleshooting

### "Project not found" Error

```bash
dev open UnknownProject
# Error: Project 'UnknownProject' not found.
```

**Solution:**
1. Check spelled correctly
2. Verify project is in one of your `projects_root` directories
3. Run `dev project list` to see configured directories
4. Add the directory to config.toml if needed

### IDE Not Detected

```bash
dev ide list
# Shows fewer IDEs than expected
```

**Solution:**
1. Ensure IDE is actually installed
2. Try restarting terminal/shell
3. IDE may be in a non-standard location — add it manually to PATH or config (future feature)

### Can't Find dev Command

```bash
dev: command not found
```

**Solution:**
1. Did you build it? `cargo build --release`
2. Is it in PATH? Add `~/.local/bin` to PATH
3. Run `dev install` to install globally

### Configuration File Issues

```bash
dev config show
# Error: Configuration file is not valid TOML
```

**Solution:**
1. Check syntax: Use an online TOML validator
2. Reset config: Delete the config file and re-run `dev config init`
3. Check permissions: Ensure you can write to config directory

### Slow Performance

If `dev` is noticeably slow:

1. **IDE detection slow:** This is expected if scanning many PATHs
2. **Config load slow:** Shouldn't happen (< 10ms)
3. **Project open slow:** Mostly waiting for IDE to start (not our control)

---

## Platform-Specific Notes

### Windows

**PATH Configuration:**
After `dev install`, add `C:\Users\{YourName}\.local\bin` to your PATH:

1. Press `Win + X`, select "System"
2. Click "Advanced system settings"
3. Click "Environment Variables"
4. Under "User variables", select "Path" and click "Edit"
5. Click "New" and add `C:\Users\{YourName}\.local\bin`
6. Click "OK" and restart terminal

**IDEs:**
- VS Code is auto-detected from standard install location
- Cursor is auto-detected from standard install location
- Windows Terminal is detected via PATH
- For custom locations, edit config.toml

**PowerShell Note:**
If using PowerShell, you may need to allow execution of scripts:
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### macOS

**Installation:**
After building, copy binary to PATH:
```bash
cp target/release/dev /usr/local/bin/
```

Or use the built-in installer concept:
```bash
cargo run -- install
# (installer will place in ~/.local/bin)
```

**IDEs:**
- Applications are auto-detected from `/Applications`
- CLI tools from PATH

### Linux

**Installation:**
```bash
cp target/release/dev ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"  # Add to ~/.bashrc or ~/.zshrc
```

**Permissions:**
```bash
chmod +x ~/.local/bin/dev
```

**IDEs:**
- Detected from standard locations
- Add custom locations via PATH or config.toml

---

## Next Steps

1. **Explore the full documentation:** See [docs/](../docs/) directory
2. **Understand the architecture:** Read [ARCHITECTURE.md](../ARCHITECTURE.md)
3. **Configure for your workflow:** Edit config.toml
4. **Create aliases (optional):**
   ```bash
   # Add to .bashrc or .zshrc
   alias do="dev open"
   ```
5. **Learn about upcoming features:** Read [docs/roadmap.md](../docs/roadmap.md)

---

## Getting Help

- 📖 **Guides:** Read the [docs/](../docs/) directory
- 🏗️ **Architecture:** Check [ARCHITECTURE.md](../ARCHITECTURE.md)
- 🤝 **Contributing:** See [CONTRIBUTING.md](../CONTRIBUTING.md)
- 🐛 **Report Issues:** Open a GitHub issue
- 💬 **Discuss:** Start a GitHub discussion

---

**Happy project management! 🚀**
