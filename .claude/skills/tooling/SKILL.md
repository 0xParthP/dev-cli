---
name: tooling
description: Before writing any new helper, or hand-rolled utility, ask "does an existing tool, package, dependency, crate, std utility, or in-repo helper already do this?" Trigger whenever you're about to implement something that's plausibly a solved problem — serialisation, config loading, env isolation, file globbing, parsing, retries, etc.
---

# `tooling` — the "don't reinvent" rule

The instinct this skill enforces:

> **Before you write a new function, helper, mutex, polling loop, regex, or boilerplate, check whether an existing tool already does the job.**

This applies to **everything** in `dev-cli` — tests, source code, CI, even docs. The win is always the same: less code for us to maintain, fewer bugs we introduce, and a codebase that anyone familiar with the ecosystem can read at speed.

## The three-tier checklist

Run through these in order before writing new code:

1. **Std library / built-in** — Rust's `std`, `core`, or a tool already in our `Cargo.toml`.
2. **An in-repo helper** — `tests/common/`, `xtask/`, anything under `src/` that's already exposed (`#[doc(hidden)] pub` for tests).
3. **A widely-used crate** — `serial_test`, `assert_cmd`, `predicates`, `tempfile`, `anyhow`, `clap`, `which`, etc. Adding a small, well-maintained dev-dependency is cheaper than maintaining a home-grown equivalent.

Only if all three say "no" should you write fresh code. And in that case, leave a comment explaining *why* the standard solution doesn't apply — that note is gold for the next maintainer.

## Quick wins people miss

| Common need | What people write | What to use instead |
|-------------|-------------------|---------------------|
| Serialise a test that mutates process-global state | `static MTX: Mutex<()>; let _g = MTX.lock();` | `serial_test::serial` (`#[serial]`) |
| Spawn the binary from a test | `std::process::Command` + manual exit-code parsing | `assert_cmd::Command` + `predicates` |
| Hermetic temp directory | `env::temp_dir().join(uuid)` | `tempfile::TempDir` |
| Path-independent filesystem assertions | `assert!(p.exists()); println!("{p:?}")` | `predicates::path::exists()` |
| Per-platform config dir | `if cfg!(windows) { ... } else { ... }` | `directories::BaseDirs` (already in `Cargo.toml`) |
| Look up an executable | Manual `PATH` walk | `which::which` (already in `Cargo.toml`) |
| Pretty errors with context | String formatting into the error | `anyhow::Context::context(...)` + `?` |
| Parse CLI args | Hand-rolled `argv` parser | `clap` derive (already used everywhere) |
| Single-test fixture that's reused | A module-level `static` mutated by every test | `rstest::fixture` or a `tests/common/` helper |
| Wall-clock waits | `std::thread::sleep` | Schedule via `Monitor` or refactor the SUT to accept a clock |
| Walk a directory respecting `.gitignore` | `std::fs::read_dir` recursive | `ignore::WalkBuilder` (already in `Cargo.toml`) |
| Find files matching a glob | Manual `read_dir` + `name().ends_with` | `globset` or `ignore::overrides` |
| Parse a small DSL | Custom string splitter | `nom`, `pest`, or even a single `regex` |
| TOML read/write | `serde_json`-style hand parsing | `toml` (already in `Cargo.toml`) |
| Logging that respects `RUST_LOG` | `println!` | `tracing` / `tracing-subscriber` |

The right-hand column is almost always shorter, faster, and tested by thousands of users.

## The "first instinct" trap

When you reach for the obvious implementation, pause and ask:

- *Is this a wheel?* — if you can name a crate that already does it, **use the crate**.
- *Is this a wheel that *we* already have?* — check `Cargo.toml` and `tests/common/`.
- *Is this a wheel that the project rejected for a reason?* — read `.claude/knowledge/dependency-map.md` and the relevant module's rustdoc. If the project consciously chose not to depend on something, don't add it without discussion.

**Concrete example from this repo:** the recent refactor replaced five `static Mutex<()>` blocks with `#[serial]` because `serial_test` is a 200-line crate that solves exactly this problem. The mutexes worked; they were just unnecessary.

## Architecture + tooling go together

`dev-cli` uses a strict 4-layer architecture (CLI → Commands → Services → Models). Before adding code, locate the right layer:

```
src/cli.rs            (clap parse)
src/main.rs           (dispatch)
src/commands/*        (orchestrate services, format output)
src/{config,scanner,installer,ide}  (business logic)
src/models/*          (data)
tests/                (integration)
```

- **Logic goes in the lowest layer that owns it.** A "find this in PATH" helper belongs in `ide/detect.rs`, not duplicated inside a command handler.
- **No upward imports.** A model must not import a service; a service must not import a command.
- **Cross-cutting concerns (env vars, paths, OS quirks) live in a single module.** Don't sprinkle `cfg!(windows)` checks across command handlers.
- **Reuse `tests/common/`** before writing a new fixture. If a new helper is justified, add it there so the next test can use it.

If you're not sure where a new function belongs, read `.claude/knowledge/architecture.md` and the relevant section of `ARCHITECTURE.md` first — they exist to answer exactly that question.

## When a new crate is the right call

1. Search crates.io and the existing dependency list for a maintained option.
2. Confirm it has zero (or negligible) transitive dependencies beyond what we already pull in.
3. Add it under `[dev-dependencies]` if it's test-only, or `[dependencies]` if the binary needs it.
4. Comment in `Cargo.toml` explaining *why* this crate (so the next person doesn't have to re-derive the reasoning).
5. Update `.claude/knowledge/dependency-map.md` with a row.

Avoid:

- Crates with 100+ transitive dependencies for a 50-line job.
- Crates last updated 2+ years ago.
- Multiple crates that overlap (pick one).
- Re-implementing what `std` already provides (e.g. don't pull in `itertools` to do `chain`).

## Self-check before merging

- [ ] Did I search std + existing dependencies + `tests/common/` for an existing solution?
- [ ] If I wrote something new, is there a comment explaining why no existing tool fit?
- [ ] Does the new code live in the correct layer per `ARCHITECTURE.md`?
- [ ] If I added a crate, is it documented in `Cargo.toml` and the dependency map?
- [ ] Does it pass `cargo fmt`, `cargo clippy --all-targets`, and `cargo test`?

## Summary

- **Use the crate, use the std, use the in-repo helper.**
- When you can't, write the smallest possible thing — and leave a note about why.
- Respect the architecture: each new function has a "natural home" in one of the four layers.
- Reuse > duplicate > reinvent.
