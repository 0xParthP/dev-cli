# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## Unreleased

### Added
- Complete documentation suite (ARCHITECTURE.md, CONTRIBUTING.md, CLAUDE.md, AGENTS.md)
- Comprehensive docs/ guides covering project structure, CLI design, configuration, IDE system, testing, roadmap
- mdBook project documentation ("Building dev-cli in Rust")
- Rustdoc comments for all public APIs and modules
- Module-level documentation (`//!` comments) for all modules
- Code examples in rustdoc comments

### Changed
- Improved README.md with professional formatting, feature table, command reference
- Enhanced error messages for better user experience
- Restructured documentation for clarity

### Fixed
- (none yet for this sprint)

### Added (infrastructure bootstrap)
- Claude workspace: `.claude/knowledge/` (architecture, modules, testing, conventions, build-system, dependency-map, development-workflow, api, architecture-diagrams) and `.claude/memory/` (decisions, implementation-notes, roadmap, progress, known-bugs, refactors)
- Specialist agents under `.claude/agents/`: `architect`, `reviewer`, `rust`, `testing`, `performance`, `security`, `documentation`, `refactoring`, `release` (existing `rust-compliance-reviewer` preserved)
- Skills under `.claude/skills/`: `explain-architecture`, `review-pr`, `generate-tests` (existing `new-cli-command`, `pre-flight` preserved)
- PostCommit reminder hook in `.claude/settings.json` (existing fmt / unwrap-block hooks preserved)
- claude-mem memory corpus `dev-cli-corpus` built and primed for semantic recall

### Fixed (infrastructure bootstrap)
- Stale `edition 2021` / `MSRV 1.70` references in `ARCHITECTURE.md`, `CONTRIBUTING.md`, `README.md`, `docs/*.md`, `docs/book/src/*.md` updated to reflect actual `edition 2024` / `MSRV 1.88`
- `ARCHITECTURE.md` and `docs/project-structure.md` reconciled with the real module layout (`ide/detect.rs`, `ide/launcher.rs`, `ide/registry.rs`, `utils/path.rs` etc.)
- Flaky `tests/launcher.rs` race on shared `DEVCLI_TEST_EXECUTABLE` env var — serialised with a static `Mutex` (same pattern as `tests/install.rs`)
- Removed unused dependencies: `tracing`, `regex` (regular), `assert_fs` (dev-dep)

---

## [0.1.0] - 2026

### Added
- Core CLI application with Clap-based argument parsing
- Project management commands: `dev project list`, `dev project open`, `dev open`
- Configuration management: `dev config show`, `dev config init`, `dev config set-default-ide`
- IDE detection system supporting 7 IDE types (VS Code, Cursor, Claude Code, Windows Terminal, IntelliJ, Rider, Zed)
- Multi-stage IDE detection (PATH lookup, common Windows locations)
- IDE launching functionality with proper error handling
- Global installation command: `dev install`
- TOML-based configuration file format with serde
- Cross-platform configuration directory handling using `directories` crate
- Integration tests for CLI commands (cli_config.rs, cli_ide.rs, cli_open.rs)
- Colored output support with `owo-colors`
- Error handling with `anyhow` Result type

### Features
- **Commands:**
  - `dev project list` - List configured project root directories
  - `dev project open <NAME>` - Open project in default IDE
  - `dev open <NAME> --ide <IDE>` - Open project in specific IDE
  - `dev config show` - Display configuration
  - `dev config init` - Initialize default configuration
  - `dev config set-default-ide <IDE>` - Set default IDE
  - `dev ide list` - List detected installed IDEs
  - `dev install` - Install to system PATH

- **IDE Support:**
  - VS Code (code)
  - Cursor (cursor)
  - Claude Code (claude)
  - Windows Terminal (wt)
  - IntelliJ IDEA (planned)
  - JetBrains Rider (planned)
  - Zed Editor (planned)

- **Configuration:**
  - Project root directories (searchable paths)
  - Default IDE preference
  - TOML format at platform-specific location
  - Auto-creation of default config

### Architecture
- Layered architecture (CLI → Commands → Services → Models)
- Clear separation of concerns
- Reusable service modules
- Extensible command structure

### Testing
- Integration tests using `assert_cmd` and `predicates`
- Test coverage for main commands
- Temporary file handling for test isolation

---

## Sprint Progress

### Sprint 1.0 - 1.6 ✅
- Initial CLI architecture
- Configuration system
- IDE detection and launching
- Basic commands
- Integration tests
- Error handling

### Sprint 1.7 - Documentation Pass 🚀
- Complete ARCHITECTURE.md with diagrams
- Professional README.md
- CONTRIBUTING.md for developers
- CLAUDE.md for AI assistants
- AGENTS.md for agent configuration
- docs/ guides (9 comprehensive guides)
- mdBook project documentation
- Rustdoc comments for all public APIs
- Module documentation
- Code examples and explanations

### Sprint 2 - Repository Scanner 🔄
- Automatic .git repository discovery
- Project metadata extraction
- Caching system
- Watch mode for changes
- Update `dev project list` to use auto-discovered projects

### Sprint 3 - Git Integration 🔄
- Show current branch for each project
- Display uncommitted changes
- Recent commit history
- Filter projects by branch
- Git status in project list

### Sprint 4+ - Dashboard & TUI 🔄
- Interactive terminal UI
- Browse projects with keyboard navigation
- Preview project status in real-time
- Search and filter
- Open project with single keystroke

---

## Known Issues

None currently known. Please report issues on GitHub.

---

## Planned Features

| Feature | Sprint | Status |
|---------|--------|--------|
| Automatic repository scanning | 2 | 🔄 Planned |
| Git branch integration | 3 | 🔄 Planned |
| Project templates | 4 | 🔄 Planned |
| Interactive TUI dashboard | 5 | 🔄 Planned |
| Remote sync (GitHub, GitLab) | 6 | 🔄 Planned |
| CI/CD pipeline info | 7 | 🔄 Planned |
| Workspace management | 8 | 🔄 Planned |

---

## Version History

### Versioning Scheme

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** (0.X.Y) — Breaking changes
- **MINOR** (X.Y) — New features, backward compatible
- **PATCH** (X.Y.Z) — Bug fixes only

Example: `0.2.3`
- 0 = Major (pre-release phase)
- 2 = Minor (features added)
- 3 = Patch (bug fixes)

Current phase: **Pre-1.0** (0.x.y)

---

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

---

## See Also

- [README.md](README.md) — Project overview
- [ARCHITECTURE.md](ARCHITECTURE.md) — System design
- [CONTRIBUTING.md](CONTRIBUTING.md) — Development guide
- [docs/roadmap.md](docs/roadmap.md) — Detailed sprint plans
