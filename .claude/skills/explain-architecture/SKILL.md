---
name: explain-architecture
description: Explains dev-cli's 4-layer architecture, command dispatch flow, and module map. Triggered when the user asks "how does this project work", "explain the architecture", or "what does module X do".
---

# `explain-architecture`

You are explaining the `dev-cli` project structure. Anchor every answer in the canonical sources: `.claude/knowledge/architecture.md` and `.claude/knowledge/modules.md` (read them if not already in context).

## Architecture Summary

`dev-cli` is a Windows-first Rust CLI (Clap) that discovers Git repos under configured roots and launches them in a detected IDE. It follows a strict 4-layer architecture with **no upward dependencies**:

```
main.rs ──► commands/* ──► services ──► models/*
  │             │            │            ▲
  └── cli.rs ───┘            └──► models ──┘
```

1. **CLI** (`src/cli.rs`, `src/main.rs`) — Clap parsing + dispatch. No business logic.
2. **Commands** (`src/commands/`) — orchestrate services, format output, user-facing errors.
3. **Services** (`src/config.rs`, `src/ide/`, `src/installer.rs`, `src/scanner.rs`) — business logic + system interaction.
4. **Models** (`src/models/`) — `Ide` enum, `Project` struct. No service dependencies.

## Command Dispatch Flow

Trace any command end-to-end, e.g. `dev config show`:

1. `Cli::parse()` (clap) parses args in `src/cli.rs`.
2. `src/main.rs` matches the `Commands` variant.
3. The matching `commands::*::execute(Command)` handler runs (e.g. `commands::config::execute`).
4. The handler calls a service (e.g. `Config::load()`), formats output, returns `anyhow::Result<()>`; `?` propagates errors to `main`.

## Module Map

Use the tables in `.claude/knowledge/modules.md` (per-layer import rules and per-module contract) and `.claude/knowledge/architecture.md` (module responsibilities and key types). Read the actual `src/` file when the user asks about a specific module rather than answering from memory.

## What to Do

- State the layer the module belongs to and which layers it may/must not import from.
- For a command: name its `execute` handler, the services it calls, and the data flow.
- If the user asks about a data flow (`dev open <name>`, IDE detection, config lifecycle), walk it step-by-step as shown in `.claude/knowledge/architecture.md`.
