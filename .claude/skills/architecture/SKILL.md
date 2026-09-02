---
name: architecture
description: Place every new function, type, test, or command in the correct layer of dev-cli's 4-layer architecture (CLI → Commands → Services → Models) and respect the no-upward-imports rule. Trigger whenever you add a new module, move code, refactor, or feel tempted to reach across layers.
---

# `architecture`

`dev-cli` is intentionally layered. Every change should slot into one of the four layers without crossing the lines below. **Before adding new code, locate its natural home; if you can't, the design probably needs another look, not a new layer.**

> Source of truth: [`ARCHITECTURE.md`](../../../ARCHITECTURE.md) (top-level) and [`.claude/knowledge/architecture.md`](../../knowledge/architecture.md) (AI-facing summary). This skill is the operational rule; the docs are the reference.

## The four layers

```
┌─────────────────────────────────────┐
│  CLI Layer        src/cli.rs        │  parse argv (clap), no logic
├─────────────────────────────────────┤
│  Commands Layer   src/commands/*    │  orchestrate services, format output
├─────────────────────────────────────┤
│  Services Layer   src/config.rs     │  business logic + system interaction
│                   src/scanner.rs    │
│                   src/installer.rs  │
│                   src/ide/*         │
├─────────────────────────────────────┤
│  Models Layer     src/models/*      │  data: Ide, Project, …
└─────────────────────────────────────┘
```

Dependencies flow **downward only**. A model never imports a service; a service never imports a command handler; a command handler never imports `clap` (it consumes parsed args structs).

| Layer | Owns | Imports allowed | Forbidden |
|-------|------|-----------------|-----------|
| **CLI** (`src/cli.rs`) | `Cli`, `Commands`, per-command args structs | `clap`, models | anything that does I/O |
| **Commands** (`src/commands/*`) | `pub fn execute(...) -> anyhow::Result<()>` | services, models, `owo-colors`/`tracing` for output | other command modules, `clap` types |
| **Services** (`src/config.rs`, `src/scanner.rs`, `src/installer.rs`, `src/ide/*`, `src/utils/*`) | real work: parse TOML, scan FS, spawn processes, detect IDEs | `std`, `anyhow`, `directories`, `which`, `ignore`, models | command handlers, `clap` |
| **Models** (`src/models/*`) | `Ide`, `Project`, plain data | `std`, `serde` | services, commands, `clap` |

`src/main.rs` is the only file that touches all four layers (it parses the `Cli` and dispatches into commands). `src/lib.rs` re-exports modules for use by integration tests.

## Where does my new code go?

| If you're adding… | It belongs in… |
|-------------------|----------------|
| A new subcommand | `Commands` variant + args struct in `src/cli.rs`; `commands/<name>.rs` with `pub fn execute(args) -> Result<()>`; match arm in `src/main.rs`; integration test in `tests/<name>.rs` |
| A new IDE | `Ide` enum in `src/models/ide.rs`; detection in `src/ide/detect.rs`; launch mapping in `src/ide/launcher.rs` |
| A new env-var override (e.g. `DEVCLI_TEST_EXECUTABLE`) | Document in `.claude/knowledge/testing.md`; read in the service that owns the affected behaviour; never read from a command handler |
| A new config field | `Config` struct in `src/config.rs`; defaults in `Config::default`; integration test that round-trips it |
| A new helper used by multiple tests | `tests/common/<name>.rs`; add a row to `.claude/knowledge/testing.md` |
| A new utility for paths / strings / env | `src/utils/<name>.rs`; declare in `src/lib.rs` |
| A pure function with no I/O | Either the owning service or a free function near its caller; **not** a command handler |

If the new code reads from disk, the network, the env, or another process, it is **service-layer** code regardless of how small it feels. Don't hide it in a command handler.

## The "natural home" test

When you're not sure where a piece of logic belongs, ask in this order:

1. **Can I express this as data on a model?** (e.g. "given an `Ide`, return its CLI arg" → `impl Ide`).
2. **Does it need the file system, env, or a process?** → service layer.
3. **Does it need to format user-facing output?** → command layer.
4. **Does it need to parse argv?** → CLI layer.

If you can answer "yes" to more than one, the logic is in the wrong place — usually it should be split between a service (does the work) and a command (formats the result).

## Anti-patterns to reject in review

- **Command handler doing real I/O** (e.g. `commands::open` directly calls `std::fs::read_dir`). Move the I/O into a service and have the command call it.
- **Service importing `crate::commands`** to "reuse" a helper. Commands are presentation; they're not a utility library. If a service needs the same logic, extract it into a service-layer module.
- **Model importing a service** (e.g. `Project::discover()` calling `scanner::discover_projects`). Keep models inert; put the operation in a service that takes the model as input.
- **Cross-cutting env-var reads in many places.** Pick one service to own the env var; everyone else goes through that service.
- **`pub mod x` in `commands/` reaching into `commands::y`** to share helpers. Either extract the shared logic to a service or `pub(crate) mod shared` inside `commands/`.
- **Tests living in `src/` for anything that exercises a command or the binary.** Pure unit tests on a service are fine; everything else goes in `tests/`. (See `tests-only-in-tests-folder` memory.)

## Module size & shape

- Target **200–300 lines** per file; hard cap **500** before splitting.
- One responsibility per module. A module named `ide.rs` that does "detect + launch + spawn + tests" is a code smell.
- If a new function doesn't obviously fit the module's name, it doesn't belong there.

## Extending the architecture (rare)

Adding a fifth layer is almost always wrong. If the design pressure is real:

1. Write a short design note in `ARCHITECTURE.md` explaining why the existing layers can't hold it.
2. Update `.claude/knowledge/architecture.md` and the diagram in this skill.
3. Update the import matrix above.
4. Get a second pair of eyes on the change.

The default answer is **no**.

## Self-check before merging

- [ ] Does the new code live in the lowest layer that can own it?
- [ ] Are there any `use` statements that point *upward* in the layer diagram? (Models → Services, Services → Commands, Commands → CLI are all forbidden.)
- [ ] If I added a new module, is it declared in `src/lib.rs` and documented in `.claude/knowledge/modules.md`?
- [ ] If I changed a layer boundary (e.g. moved logic from commands to services), did I update both `ARCHITECTURE.md` and `.claude/knowledge/architecture.md`?
- [ ] Does the new code respect module size (target 200–300, hard cap 500 lines)?

## Quick mnemonic

> **"Where does this live?"** → walk down the layer diagram until you find the first layer that has the dependencies your code needs. That's home.
