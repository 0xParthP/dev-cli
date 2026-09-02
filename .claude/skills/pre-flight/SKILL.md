---
name: pre-flight
description: Runs the required project checks (fmt, clippy, test, doc) strictly before committing.
disable-model-invocation: true
---

# `pre-flight` Check

The user is running the strict pre-flight code validation checks for `dev-cli`.

Execute the following verification string. If any step fails, investigate the issue, fix the code, and re-run until all checks pass cleanly.

### Required Checks Protocol:
1. Format: `cargo fmt`
2. Lint: `cargo clippy --workspace --all-targets -- -D warnings` (Strict! Deny warnings)
3. Test: `cargo test --workspace`
4. Doc: `cargo doc --no-deps --workspace`

**Instructions for you:**
Do **not** prompt the user to run these. You should use the `Bash` tool to execute `cargo fmt && cargo clippy -- -D warnings && cargo test && cargo doc --no-deps` directly in the background.

If everything passes with exit code 0, print a green success message: "✅ All pre-flight checks passed successfully! You are clear to commit."
If anything fails, Read the failing file, Edit the bug away (usually resolving an unused import, formatting issue, or a missing `.context()` via anyhow), and retry.
