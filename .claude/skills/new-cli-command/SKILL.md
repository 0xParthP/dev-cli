---
name: new-cli-command
description: Scaffolds a new dev-cli command following the 7-step architecture checklist in CLAUDE.md
---

# `new-cli-command`

You are generating a new CLI command for `dev-cli`. The user will provide the name of the command and its purpose.
Your task is to implement the command by following the **exact 7-step checklist** outlined for this project.

## Requirements

1. **Define CLI arguments in `src/cli.rs`**
   - Add a new variant to the `pub enum Commands` enum.
   - Create a struct `[CommandName]Command` with `#[derive(Args)]`.
   - Add rustdoc comments (`///`) to the struct and enum variant.

2. **Create command handler in `src/commands/[command_name].rs`**
   - Implement `pub fn execute(cmd: [CommandName]Command) -> Result<()>` returning `anyhow::Result`.
   - Call the services layer; do not implement business logic inside this file.
   - Use `anyhow::Context` strictly. Do not use `unwrap()`.

3. **Export handler in `src/commands/mod.rs`**
   - Add `pub mod [command_name];`

4. **Dispatch in main in `src/main.rs`**
   - Update the `match` block in `src/main.rs` to route the new command variant to `commands::[command_name]::execute(cmd)?`.

5. **Add tests in `tests/commands_[command_name].rs`**
   - Create the file.
   - Use `assert_cmd` and `predicates` to test the happy path and an error case.

6. **Update Documentation**
   - Run a quick review against `doc/`, `README.md` and `CHANGELOG.md` to note the new feature.

7. **Run Checks**
   - Remind the user to run the `pre-flight` skill, or execute:
     `cargo fmt && cargo clippy -- -D warnings && cargo test && cargo doc --no-deps`

Always ensure you search the codebase for similar commands to match their style and output formatting verbatim.
