# Roadmap

Where the project is and where it's going.

## Status

| Sprint | Focus | Status |
|--------|-------|--------|
| 1.0–1.6 | Foundation: CLI, config, IDE detection, launching, install | Done |
| 1.7 | Repository scanner + documentation | In progress |
| 2 | Git integration: branches, status, recent commit | Planned |
| 3 | Interactive TUI dashboard | Planned |
| 4+ | Templates, remote sync, CI status, workspaces | Exploratory |

## What's Shipped

- `dev project list` / `dev project open <name>` / `dev open <name>` shorthand
- `dev config show` / `init` / `set-default-ide`
- `dev ide list`
- `dev install`
- TOML configuration with platform-aware paths
- IDE detection via PATH + common install locations
- `dev` installer that places the binary on PATH

Supported IDEs: VS Code, Cursor, Claude Code, Windows Terminal. Adding a new IDE means adding a variant to the `Ide` enum and a detection rule.

## In Progress

### Repository Scanner (Sprint 1.7)

`src/scanner.rs` will walk each `projects_root`, recognise a directory as a project by the presence of `.git`, and produce a list of `Project` values. Honours `.gitignore` so `node_modules`, `target`, etc. are not traversed. The `dev project list` output will switch to showing the discovered projects instead of just the configured roots.

This unblocks the Git integration in Sprint 2 and gives the TUI (Sprint 3) something to browse.

## Planned

### Git Integration (Sprint 2)

For each discovered project, surface:

- Current branch
- Dirty/clean state
- Most recent commit (subject + relative time)

Implementation: spawn `git` as a subprocess per project, parse its output, cache aggressively. Adds `src/git/` (branch, status, log).

### Interactive TUI (Sprint 3)

A `dev dashboard` command that lists projects with arrow-key navigation, fuzzy filter, and Enter to open in the configured IDE. Built on `crossterm` (or `ratatui` once we compare).

The TUI sits above the services layer, so it can call the same `scanner` and `launcher` functions the CLI uses.

### Later

These are sketched but not scheduled:

- **Project templates** — `dev new --template rust-cli my-app`
- **Remote sync** — pull project lists from GitHub/GitLab
- **CI status** — surface the latest build for each project
- **Workspaces** — group projects and open them in one go

## Versioning

Pre-1.0: minor bumps add features, patches fix bugs, breaking changes are noted in `CHANGELOG.md`. Post-1.0 we follow semver strictly.

## Known Limitations

- Windows is the primary target. macOS and Linux work for the basics but the IDE install paths and the installer are Windows-leaning.
- We support a small set of IDEs by name. Anything not in the `Ide` enum won't be detected.
- No cached IDE paths — every run re-detects. This is by design; detection is fast and IDEs move.
- No plugin system. The detection logic lives in code; if you need a new IDE, open a PR.

## Principles

A few rules of thumb that guide the design:

- **Single binary, fast startup.** A CLI that takes a second to launch is dead on arrival.
- **Layered, not over-engineered.** Services stay independent of commands so we can add a TUI or HTTP front-end without rewriting them.
- **Document what's in the code, not what might be.** Roadmap items move when they ship, not before.
- **Real problems first.** Every feature should map to a thing a developer actually does daily.

## Getting Involved

- Open an issue to report a bug or request a feature.
- Read [CONTRIBUTING.md](../CONTRIBUTING.md) before opening a PR.
- The `help wanted` label on GitHub marks issues we think are good first contributions.

## See Also

- [CHANGELOG.md](../CHANGELOG.md) — what shipped and when
- [CONTRIBUTING.md](../CONTRIBUTING.md) — workflow and standards
- [ARCHITECTURE.md](../ARCHITECTURE.md) — the layers these features plug into
