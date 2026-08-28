# Documentation Maintenance Guidelines

**For all AI agents, LLMs, and contributors working on dev-cli**

---

## 🔴 CRITICAL PRINCIPLE: Documentation-First Development

> **Every code change MUST include corresponding documentation updates. No exceptions.**

This is not optional. Documentation updates are part of the **Definition of Done**. A change is NOT complete until documentation is updated.

---

## Rule 1: Code Changes Require Documentation Updates

### The Principle

```
Code Change → Update Related Docs → Update Rustdoc → Commit Together
```

**Never commit code changes without updating documentation.**

### What This Means

When you modify code, you MUST also update:

1. **Rustdoc comments** if public APIs change
2. **Module-level documentation** if module behavior changes
3. **Architecture documentation** if structure changes
4. **User guides** if user-facing behavior changes
5. **CHANGELOG.md** with the changes
6. **Examples in docs/** if examples are affected

---

## Rule 2: Types of Changes & Required Documentation

### Adding a Public Function

```rust
// ❌ INCOMPLETE
pub fn new_function(x: i32) -> Result<String> {
    // implementation
}

// ✅ COMPLETE: Includes rustdoc
/// Brief description of what this does.
///
/// # Arguments
/// * `x` - What x represents
///
/// # Returns
/// Description of return value
///
/// # Errors
/// When this can fail and why
///
/// # Example
/// ```
/// let result = new_function(42)?;
/// ```
pub fn new_function(x: i32) -> Result<String> {
    // implementation
}
```

**Also update:** Module rustdoc if function is important

### Adding a New Public Type

```rust
// ❌ INCOMPLETE
pub struct NewType {
    pub field: String,
}

// ✅ COMPLETE
/// Represents a [concept explanation].
///
/// # Fields
/// - `field` - What this field stores
///
/// # Example
/// ```
/// let obj = NewType { field: "value".into() };
/// ```
pub struct NewType {
    /// Description of field purpose
    pub field: String,
}
```

**Also update:**
- Module rustdoc
- Relevant user guide in docs/
- Architecture documentation if it's a core type

### Adding a New Module

```
Create:
1. src/new_module.rs with module-level //! documentation
2. Add pub mod new_module; to parent mod.rs
3. Add documentation in docs/new_module.md
4. Update docs/project-structure.md
5. Update ARCHITECTURE.md if it's significant
6. Update README.md if user-facing
```

### Changing Existing Behavior

```
Update:
1. Rustdoc if function signature or behavior changes
2. docs/project-structure.md for architecture changes
3. Relevant user guide if user-facing behavior changes
4. ARCHITECTURE.md if affects system design
5. CHANGELOG.md with breaking change notice
6. Examples if they're now incorrect
```

### Adding a New Command

**Complete checklist:**

- [ ] Update `src/cli.rs` with new command definition
- [ ] Add rustdoc to all CLI structs/enums
- [ ] Create `src/commands/new_cmd.rs` with module rustdoc
- [ ] Add rustdoc to public functions
- [ ] Create `tests/cli_new_cmd.rs` integration tests
- [ ] Update README.md command reference
- [ ] Create/update relevant docs/ guide
- [ ] Update ARCHITECTURE.md if it's significant
- [ ] Update CHANGELOG.md (Unreleased section)
- [ ] Verify `cargo doc --no-deps` passes
- [ ] Run full quality check: `cargo fmt && cargo clippy && cargo test && cargo doc --no-deps`

### Changing Configuration Structure

```
Update:
1. docs/configuration.md with new schema
2. Rustdoc on Config struct
3. Example config in getting-started.md
4. Migration guide if breaking change
5. CHANGELOG.md
```

### Fixing a Bug

```
Update:
1. Rustdoc if behavior description was unclear
2. CHANGELOG.md with bug fix note
3. Add test case to prevent regression
4. Update docs if bug affected documented behavior
```

---

## Rule 3: Documentation Locations & Responsibilities

| Change Type | Files to Update |
|-----------|-----------------|
| Public function | Rustdoc + module docs |
| New command | README.md, ARCHITECTURE.md, relevant docs/ guide |
| Config change | docs/configuration.md, Config rustdoc |
| Error handling | Function rustdoc (# Errors section) |
| Architecture change | ARCHITECTURE.md, docs/project-structure.md |
| User-facing behavior | Relevant docs/ guide + README.md |
| Internal refactor | Module rustdoc, code comments |

---

## Rule 4: Verification Checklist

**Before every commit, verify:**

- [ ] All public APIs have rustdoc (run `cargo doc --no-deps`)
- [ ] Module-level `//!` documentation present
- [ ] Related documentation files updated
- [ ] CHANGELOG.md updated (Unreleased section)
- [ ] Examples are accurate
- [ ] No `cargo doc` warnings
- [ ] Code passes all quality checks:
  ```bash
  cargo fmt && cargo clippy && cargo test && cargo doc --no-deps
  ```

---

## Rule 5: When Documentation Lags Behind Code

If you discover code exists but is undocumented:

**THIS IS A BUG.** Treat it as seriously as a code bug:

1. **Create documentation immediately** (not in future sprint)
2. **Update CHANGELOG.md** noting "Added missing documentation for X"
3. **Add tests if missing** (code without tests + docs is untested)
4. **Fix code if documentation reveals it's broken** (docs expose bad design)

**Example:**

```
User: "I need to understand how IDE detection works"
Agent response: "I notice detect_ides() lacks comprehensive documentation"
Action: Immediately add detailed rustdoc + docs/ide-system.md section
Never: "I'll document it later"
```

---

## Rule 6: For AI Agents / LLMs Specifically

### Your Workflow

**When Claude Code or other AI agents work on this codebase:**

1. **Read the docs first** — Understand current state
2. **Make code changes** — Implement feature/fix
3. **Update documentation simultaneously** — Before testing
4. **Verify consistency** — Does doc match code?
5. **Run quality check** — `cargo doc --no-deps` must pass
6. **Review changes** — Code + docs changed together

### Red Flags (Do Not Commit)

❌ Code change but rustdoc not updated  
❌ New public API without examples  
❌ Behavior change but CHANGELOG.md not updated  
❌ `cargo doc` produces warnings  
❌ README.md or architecture docs out of sync  
❌ New feature but no user guide  

### Documentation-First Mindset

When asked to:

- **"Add feature X"** → First write rustdoc, then implement
- **"Fix bug Y"** → First understand why docs were unclear
- **"Refactor module Z"** → First update architecture docs
- **"Optimize performance"** → First document trade-offs in comments

---

## Rule 7: Documentation Review

### When Reviewing Changes

Every review must verify:

```
Feature/Fix: [description]
✅ Code implementation complete
✅ Rustdoc complete with # Examples
✅ Module-level docs updated
✅ README.md updated if user-facing
✅ CHANGELOG.md updated
✅ User guides updated
✅ ARCHITECTURE.md updated if needed
✅ cargo doc --no-deps passes
✅ All quality checks pass
```

If ANY documentation is missing → **Send back for documentation updates.**

---

## Rule 8: Documentation Debt

Tracking documentation debt:

```markdown
// In code
// TODO-DOC: Explain why we detect IDEs at runtime instead of caching
// See: docs/ide-system.md#why-not-cache-paths-in-config

// In CHANGELOG.md
## Documentation Improvements
- Clarified IDE detection algorithm in docs/ide-system.md
```

---

## The Contract with AI Agents

> **If you work on dev-cli, you commit to:**
>
> 1. **Every code change includes documentation**
> 2. **Documentation is complete and accurate**
> 3. **`cargo doc --no-deps` passes without warnings**
> 4. **User guides stay synchronized with code**
> 5. **Architecture documentation reflects reality**
> 6. **Examples in docs actually work**

If you cannot maintain this standard, **do not make the code change.**

---

## Quick Reference: Documentation Checklist

```bash
# After making any code change, run this:

# 1. Update rustdoc if public APIs changed
# 2. Update module docs if behavior changed
# 3. Update relevant docs/ guide if user-facing
# 4. Update CHANGELOG.md
# 5. Run this command:

cargo fmt && \
cargo clippy && \
cargo test && \
cargo doc --no-deps && \
echo "✅ All checks passed. Documentation complete."

# If this passes, your changes are ready.
# If it fails, fix the issues and try again.
```

---

## Examples of Correct Workflow

### ✅ Example 1: Adding a New IDE

```
1. Add IDE to models/ide.rs enum
   ├─ Add rustdoc for variant
   └─ Verify implements ValueEnum

2. Update ide/detect.rs
   ├─ Add detection logic
   └─ Add comprehensive rustdoc

3. Update docs/ide-system.md
   ├─ Add to supported IDEs table
   ├─ Update detection algorithm if needed
   └─ Add troubleshooting if needed

4. Update README.md
   ├─ Add to feature table

5. Update CHANGELOG.md
   ├─ Add to Unreleased section

6. Verify and commit
   └─ cargo fmt && cargo clippy && cargo test && cargo doc --no-deps
```

### ✅ Example 2: Fixing a Bug

```
1. Write test that reproduces bug
2. Fix the bug in code
3. Check if docs explain the behavior
   ├─ If not, document the expected behavior
   ├─ If docs are wrong, fix them
4. Update rustdoc if clarification needed
5. Update CHANGELOG.md with bug fix note
6. Verify: cargo fmt && cargo clippy && cargo test && cargo doc --no-deps
```

### ✅ Example 3: Refactoring

```
1. Plan refactoring, check docs first
2. Update architecture docs if needed
3. Refactor code with improved rustdoc
4. Update module-level docs
5. Update CHANGELOG.md with "Improved X documentation"
6. Verify: cargo fmt && cargo clippy && cargo test && cargo doc --no-deps
```

---

## For Future AI Models Reading This

**This is non-negotiable.**

When you load dev-cli into Claude Code or any AI development tool:

- **Always** update documentation with code changes
- **Never** defer documentation to "later"
- **Every** public API needs comprehensive rustdoc
- **Every** user-facing change needs guide updates
- **Every** architectural change needs ARCHITECTURE.md updates

The quality of this codebase depends on documentation being kept current. If you skip this step, you break the quality contract.

---

**Questions about what needs documenting? Ask. But never commit without it.**
