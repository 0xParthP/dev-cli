# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## Unreleased

### Added
- Comprehensive onboarding test suite in `tests/onboarding.rs` covering the
  first-start wizard's non-interactive paths and the `is_interactive_terminal`
  helper. New `serial_test` dependency keeps env-driven tests deterministic.
- `src/startup.rs` extracted from `main.rs` so `onboarding::ensure_onboarded`
  can be reused and tested in isolation.
- `Config::exists()` helper used by the startup flow to skip the wizard on
  subsequent runs.

### Changed
- Reorganized the top-level documentation pass around the real on-disk layout
  (`lib.rs` exposes the library, `tests/` is the only home for tests,
  `xtask/` is the developer entry point, etc.).
- Updated every `docs/*.md` to reflect the current code (install command,
  onboarding wizard, scanner integration, 80% coverage gate, branch naming,
  `cargo xtask ci`).

### Fixed
- Branch-name pre-commit hook and `branch-name.yml` workflow now reject
  names that don't match `^(feature|fix|docs|refactor|chore)/[a-z0-9-]+$`
  (kebab-case). The previous pattern allowed spaces and other characters.
- `Config::load` no longer panics when the config file is missing during
  CI runs — it writes the defaults and returns, and on a parse error it
  recreates a valid file with an explanatory message on stderr.

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

## [0.2.0] - 2026

### Added
- First-run onboarding wizard (`src/onboarding.rs`). When invoked on a TTY
  with no `config.toml` present, it walks the user through choosing
  `projects_root` directories and a default IDE, then writes the config.
  CI/test runs opt out via `DEVCLI_SKIP_ONBOARDING=1`.
- Repository scanner (`src/scanner.rs`) using the `ignore` crate — `dev
  project list` now reports the Git repos it discovers under the configured
  roots, honouring `.gitignore` and pruning at the `.git` boundary.
- Library crate (`src/lib.rs`) so integration tests can `use dev_cli::…`.
- `src/startup.rs` orchestrating onboarding and config loading.
- `src/utils/path.rs` with `display_path` for friendlier Windows path
  output.
- `cargo xtask ci` as the canonical pre-commit / CI entry point (format,
  clippy, test, coverage, security). `cargo xtask install` mirrors the
  install command for developers.
- 80% line-coverage gate enforced in `cargo xtask ci`.
- Branch-name policy enforced in both the local pre-commit hook and the
  `branch-name.yml` workflow: names must match
  `^(feature|fix|docs|refactor|chore)/[a-z0-9-]+$`.
- Sonar workflow for code-quality analysis.

### Changed
- Help menu restructured: `dev --help` now prints a one-line "usage" header,
  a list of all subcommands, and the global options in that order.
- `dev project list` shows the discovered repositories in addition to the
  configured project roots.
- Project paths are rendered through `display_path` for cleaner Windows
  output (e.g. collapsing `C:\Users\parth\Projects\…` to `…\Projects\…`).
- All production code follows the no-`unwrap` rule; tests use `serial_test`
  to remove env-var races.

### Fixed
- `Config::load` no longer panics when the config file is missing or
  unparseable — missing files are created with defaults; parse errors
  surface a clear message and write a fresh config.
- Pre-commit hook no longer overwrites files via `git add -A` after
  `cargo fmt`; only the formatted files are added.

--- 

## Known Issues

None currently known. Please report issues on GitHub.

---

## Planned Features

| Feature | Sprint | Status |
|---------|--------|--------|
| Automatic repository scanning | 2 | ✅ Shipped |
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
