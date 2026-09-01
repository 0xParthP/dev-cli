---
name: maintenance
description: How to keep .claude/ knowledge and memory files current as code evolves
metadata:
  type: reference
---

# Maintenance

This document is the single source of truth for **when and how** to update each
`.claude/knowledge/*.md` and `.claude/memory/*.md` file. Run through the
relevant subsection when making a code change.

> Pair this with the agents under `.claude/agents/` and the skills under
> `.claude/skills/` — they read these files.

---

## Knowledge files (`.claude/knowledge/`)

| File | Update when… | Co-update with… |
|------|--------------|------------------|
| `architecture.md` | new layer, new command category, or any new module that crosses a layer boundary | `modules.md`, `architecture-diagrams.md` |
| `modules.md` | module added/renamed/deleted, or its public surface changes | `architecture.md`, `dependency-map.md` |
| `api.md` | new CLI command, new env-var hook, or library API change | `modules.md` |
| `build-system.md` | `Cargo.toml` workspace member, cargo alias, xtask subcommand, or CI workflow changes | `development-workflow.md` |
| `testing.md` | new test file, new isolation env var, or coverage gate change | none |
| `conventions.md` | new project-wide rule added (naming, error handling, doc style) | `.claude/CLAUDE.md` |
| `dependency-map.md` | crate added/removed/version-pinned, or its role changes | `build-system.md` |
| `development-workflow.md` | PR/CI/local workflow step changes | `build-system.md` |
| `architecture-diagrams.md` | any of the above; also any new CI workflow | `architecture.md` |
| `MAINTENANCE.md` (this file) | when the maintenance rules themselves change | — |

## Memory files (`.claude/memory/``)

| File | Add an entry when… |
|------|--------------------|
| `decisions.md` | an architectural or technical decision is made (with rejected alternatives) |
| `implementation-notes.md` | you discover a non-obvious workaround, gotcha, or pattern (e.g. the env-var Mutex) |
| `roadmap.md` | a phase / sprint is completed or added |
| `progress.md` | a meaningful chunk of work is finished (sprint boundary, bootstrap milestone) |
| `known-bugs.md` | a bug is confirmed, fixed, or newly suspected |
| `refactors.md` | a refactor decision is made (include what was rejected and why) |

## Per-change checklist

When you make a code change, ask:

1. **Did I add/rename/delete a module?** → update `modules.md`, `architecture.md`, `architecture-diagrams.md`.
2. **Did I add/remove a `Cargo.toml` dependency?** → update `dependency-map.md`; consider an entry in `refactors.md`.
3. **Did I add a new command or change the CLI surface?** → update `api.md`, `README.md`, `CHANGELOG.md`.
4. **Did I add a new env-var test hook?** → update `testing.md`, `api.md`.
5. **Did I make an architectural decision?** → add to `decisions.md` (with rejected alternatives).
6. **Did I fix a bug?** → move it in `known-bugs.md` from suspected → fixed with date.
7. **Did I refactor?** → add to `refactors.md`; if it touched a layer boundary, also update `architecture.md`.
8. **Before merging:** run `/review-pr` skill and the agents under `.claude/agents/`.

## Triggering this on demand

The `explain-architecture` skill can render a fresh summary of the current
`.claude/knowledge/` state, which is a good way to spot drift.
