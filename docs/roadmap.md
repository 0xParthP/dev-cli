# Roadmap

The future direction and planned sprints for `dev-cli`.

---

## Current Status

**Sprint:** 1.7 - Documentation Pass  
**Version:** 0.1.0  
**Release Date:** TBD

---

## Sprint Overview

```
Sprint 1.0-1.6   ████████████████████ Complete
Sprint 1.7 (Now) ██████                In Progress
Sprint 2         ░░░░░░░░░░░░░░░░░░░░ Planned
Sprint 3-8       ░░░░░░░░░░░░░░░░░░░░ Planned
```

---

## Sprint 1.0 - 1.6: Foundation ✅

**Status:** Complete

### Accomplishments

- ✅ CLI framework with Clap
- ✅ Configuration system (TOML, Serde)
- ✅ IDE detection (multi-stage)
- ✅ Project launching
- ✅ Global installation
- ✅ Integration tests
- ✅ Error handling with anyhow

### Delivered Features

- `dev project list` — List project directories
- `dev project open <NAME>` — Open project
- `dev open <NAME>` — Shorthand
- `dev config show` — Display config
- `dev config init` — Initialize config
- `dev config set-default-ide` — Set default
- `dev ide list` — List detected IDEs
- `dev install` — Install globally

### Supported IDEs

- VS Code
- Cursor
- Claude Code
- Windows Terminal

### Rust Learning Concepts Covered

- Modules and organization
- Structs and enums
- Pattern matching
- Traits and derives
- Error handling (Result, anyhow)
- Ownership and borrowing

---

## Sprint 1.7: Documentation Pass 🚀

**Status:** In Progress  
**Duration:** ~1 week

### Goals

Transform the repository into a polished, well-documented open-source project.

### Deliverables

- ✅ Professional README.md
- ✅ ARCHITECTURE.md with diagrams
- ✅ CONTRIBUTING.md for developers
- ✅ CHANGELOG.md with version history
- ✅ CLAUDE.md for AI assistants
- ✅ AGENTS.md for agent configuration
- ✅ Complete docs/ directory (9 guides)
- 🔄 mdBook project documentation
- 🔄 Rustdoc comments for all public APIs
- 🔄 .claude folder configuration files

### Documentation Structure

```
Root Documentation:
- README.md — Project overview
- ARCHITECTURE.md — System design
- CONTRIBUTING.md — Developer guide
- CHANGELOG.md — Version history
- CLAUDE.md — AI instructions
- AGENTS.md — Agent rules

User Guides (docs/):
- getting-started.md — Installation & setup
- project-structure.md — File reference
- rust-for-dev-cli.md — Rust tutorial
- cli-design.md — CLI parser explanation
- configuration.md — Config file format
- ide-system.md — IDE detection algorithm
- testing.md — Testing philosophy
- style-guide.md — Code standards
- roadmap.md — This file

mdBook:
- Complete narrative guide
- Chapters 1-6: From bootstrap to architecture
- Designed for learning
```

### Learning Objectives

- Understanding how documentation serves different audiences
- Writing for different skill levels
- Organizing information clearly
- Creating effective diagrams (Mermaid)

---

## Sprint 2: Automatic Repository Discovery 🔄

**Status:** Planned  
**Estimated Duration:** 2 weeks  
**Target Release:** v0.2.0

### Goals

Automatically discover Git repositories without manual configuration.

### Features

#### Automatic Project Scanning

```
Projects discovered:
  C:\Users\parth\Projects\
  ├── MyProject (.git) ← Auto-discovered!
  ├── OtherProject (.git) ← Auto-discovered!
  └── tools/
      └── cli-tool (.git) ← Recursive search

User doesn't add anything to config!
```

#### Enhanced `dev project list`

```bash
$ dev project list
Discovered Projects
📁 C:\Users\parth\Projects\MyProject
📁 C:\Users\parth\Projects\OtherProject
📁 C:\Users\parth\Projects\tools\cli-tool

Configured Project Roots
📁 C:\Users\parth\Projects
```

### Implementation Details

**New Module:** `src/scanner.rs`

```rust
pub fn discover_projects(roots: &[PathBuf]) -> Result<Vec<Project>> {
    let mut projects = Vec::new();
    
    for root in roots {
        discover_recursive(root, &mut projects)?;
    }
    
    Ok(projects)
}

fn discover_recursive(dir: &Path, projects: &mut Vec<Project>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        
        if path.join(".git").exists() {
            projects.push(Project {
                name: path.file_name()?.to_string_lossy().to_string(),
                path,
            });
        } else if path.is_dir() {
            discover_recursive(&path, projects)?;
        }
    }
    
    Ok(())
}
```

**Features:**
- Recursive directory scanning
- `.git` detection
- Ignore patterns (from `.gitignore`)
- Caching results (optional)
- Watch for changes (optional)

### Rust Concepts Introduced

- `fs::read_dir()` — Directory iteration
- Walking the filesystem
- Error handling with custom types
- Performance optimization

---

## Sprint 3: Git Integration 🔄

**Status:** Planned  
**Estimated Duration:** 2 weeks  
**Target Release:** v0.3.0

### Goals

Show Git information for each project.

### Features

#### Enhanced Project List

```bash
$ dev project list
MyProject
  Branch: main
  Status: Clean ✓
  Recent: "feat: add logging"

OtherProject
  Branch: feature/new-feature
  Status: Uncommitted changes (3 files)
  Recent: "fix: handle edge case"
```

#### Filter by Branch

```bash
$ dev open MyProject --branch main
# Only shows projects on 'main' branch

$ dev project list --branch feature
# Show only projects with 'feature' branches
```

#### Git Status Display

```bash
$ dev status
MyProject — main, 2 commits ahead
OtherProject — develop, uncommitted changes
tools/cli-tool — (detached HEAD)
```

### Implementation

**New Module:** `src/git/`

```rust
// src/git/mod.rs
pub mod branch;
pub mod status;

pub struct GitInfo {
    pub current_branch: String,
    pub status: GitStatus,
    pub recent_commit: String,
}

pub enum GitStatus {
    Clean,
    UncommittedChanges(usize),
    Untracked(usize),
}
```

### Rust Concepts Introduced

- Process spawning with output capture
- Parsing command output
- Working with Git commands
- Error handling with git operations

---

## Sprint 4: Interactive Dashboard (TUI) 🔄

**Status:** Planned  
**Estimated Duration:** 3 weeks  
**Target Release:** v0.4.0

### Goals

Add an interactive terminal UI for browsing and launching projects.

### Features

#### Interactive Mode

```bash
$ dev dashboard
```

```
┌─────────────────────────────────────┐
│   dev-cli Dashboard                 │
├─────────────────────────────────────┤
│  ↓ MyProject (main)                 │
│    OtherProject (develop)           │
│    tools/cli-tool (main)            │
├─────────────────────────────────────┤
│  Press ↑↓ to navigate               │
│  Press Enter to open                │
│  Press q to quit                    │
└─────────────────────────────────────┘
```

#### Features

- Arrow keys to navigate
- Enter to open project
- Type to search/filter
- Show Git branch and status
- Preview project details
- Configure default IDE per project

### Implementation

**New Modules:**
- `src/tui/` — Terminal UI layer
- `src/events/` — Event handling
- Use `crossterm` or `termion` for terminal control

### Rust Concepts Introduced

- Terminal control and ANSI codes
- Event loops
- State management
- User input handling

---

## Sprint 5: Project Templates 🔄

**Status:** Planned  
**Target Release:** v0.5.0

### Goals

Quick project scaffolding with templates.

### Features

```bash
$ dev new --template rust-cli my-project
Created my-project with Rust CLI template

$ dev new --template web-fullstack my-app
Created my-app with full-stack web template

$ dev templates list
Available templates:
  rust-cli — CLI application template
  rust-lib — Library template
  web-fullstack — Web app template
  python-ml — Machine learning template
```

### Implementation

- Store templates in `~/.local/share/dev-cli/templates/`
- Git-based template downloading
- Customizable variables
- Post-creation hooks

---

## Sprint 6: Remote Synchronization 🔄

**Status:** Planned  
**Target Release:** v0.6.0

### Goals

Sync project list with remote sources.

### Features

```bash
$ dev sync github --user myusername
Synced 42 repositories from GitHub

$ dev sync gitlab --group mygroup
Synced 15 repositories from GitLab

$ dev sync --all
Syncing from: GitHub, GitLab, Gitea...
```

### Implementation

- OAuth authentication
- API integration (GitHub, GitLab, Gitea)
- Automatic sync on schedule
- Cached state

---

## Sprint 7: CI/CD Integration 🔄

**Status:** Planned  
**Target Release:** v0.7.0

### Goals

Show CI/CD pipeline information for projects.

### Features

```bash
$ dev status
MyProject
  CI Status: Passing ✓
  Latest build: 2 min ago
  Coverage: 82%

OtherProject
  CI Status: Failing ✗
  Failed: tests/integration_tests.rs
```

### Implementation

- GitHub Actions integration
- GitLab CI support
- Custom webhook support
- Real-time status updates

---

## Sprint 8: Workspace Management 🔄

**Status:** Planned  
**Target Release:** v0.8.0

### Goals

Manage workspace groups and multi-project development.

### Features

```bash
$ dev workspace create fullstack
Created workspace 'fullstack'

$ dev workspace add fullstack MyProject
$ dev workspace add fullstack MyAPI
$ dev workspace add fullstack MyDB

$ dev workspace open fullstack
Opens all 3 projects in separate IDE windows

$ dev workspace list
fullstack — 3 projects
  MyProject (main)
  MyAPI (develop)
  MyDB (release)
```

### Implementation

- Workspace configuration files
- Batch operations
- IDE support for multi-project opening

---

## Version Timeline

| Version | Sprint | Status | Date |
|---------|--------|--------|------|
| 0.1.0 | 1.0-1.6 | ✅ Complete | Early 2024 |
| 0.2.0 | 2 | 🔄 Planned | Mid 2024 |
| 0.3.0 | 3 | 🔄 Planned | Late 2024 |
| 0.4.0 | 4 | 🔄 Planned | Early 2025 |
| 0.5.0 | 5 | 🔄 Planned | Mid 2025 |
| 1.0.0 | 6-8 | 🔄 Planned | Late 2025 |

---

## Release Strategy

### Pre-1.0 (0.x.y)

- **Minor** version (0.x) for features
- **Patch** version (0.x.y) for bug fixes
- **BREAKING** changes possible in minor releases
- Documented in CHANGELOG.md

### Post-1.0

- Follow [Semantic Versioning](https://semver.org/)
- BREAKING changes only in major versions
- Deprecation warnings for planned removals

---

## Ongoing Initiatives

### Code Quality

- Increase test coverage to 80%+
- Reduce cyclomatic complexity
- Improve error messages
- Performance benchmarking

### Documentation

- Keep guides updated with features
- Add video tutorials
- Create architecture decision records
- Maintain API docs with examples

### Community

- Accept community contributions
- Respond to issues and PRs quickly
- Create contribution guide
- Build ecosystem around `dev-cli`

---

## Known Limitations (Pre-1.0)

1. **Windows-first** — macOS/Linux support basic
2. **Limited IDE support** — 4 IDEs currently, expanding in future
3. **No caching** — Fresh detection every run (by design currently)
4. **Minimal logging** — Will add as features grow
5. **No plugin system** — Planned for future

---

## Principles Guiding Development

### 1. Simplicity First

Keep core functionality simple. Build complexity incrementally.

### 2. Rust as Learning Vehicle

Every feature should teach Rust concepts clearly.

### 3. Open Source Ready

From day one, design for community contribution.

### 4. Documentation-Driven

Document before coding. Make concepts clear.

### 5. User-Focused

Solve real developer problems in elegant ways.

---

## Getting Involved

Want to help shape the roadmap?

1. **Report issues** — Let us know what's missing
2. **Suggest features** — GitHub discussions
3. **Contribute code** — See [CONTRIBUTING.md](../CONTRIBUTING.md)
4. **Improve docs** — Documentation PRs always welcome
5. **Spread the word** — Star on GitHub, share with friends

---

## FAQ

**Q: When will feature X be released?**  
A: See the timeline above. Exact dates depend on community contributions.

**Q: Can I request a feature?**  
A: Yes! Open a GitHub discussion or issue.

**Q: Will there be a GUI?**  
A: Possibly in a separate application. CLI will remain the primary interface.

**Q: When will v1.0 be released?**  
A: Target: Late 2025, pending feature completion and community feedback.

**Q: Is this project still active?**  
A: Yes! See [CHANGELOG.md](../CHANGELOG.md) for recent updates.

---

## See Also

- [CHANGELOG.md](../CHANGELOG.md) — Completed features
- [CONTRIBUTING.md](../CONTRIBUTING.md) — How to contribute
- [ARCHITECTURE.md](../ARCHITECTURE.md) — System design
- [GitHub Issues](https://github.com/yourusername/dev-cli/issues) — Feature requests
