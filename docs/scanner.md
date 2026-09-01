# Repository Scanner

`src/scanner.rs` discovers Git repositories inside the configured `projects_root` directories.

## Responsibilities

- Walk each `projects_root` recursively.
- Skip directories matched by `.gitignore` (using the `ignore` crate) and well-known noise like `target/`, `node_modules/`, `.idea/`.
- Detect a directory as a project by the presence of a `.git` entry.
- Return a `Vec<Project>` sorted by path, deduplicated.

## Why the `ignore` Crate

`ignore` is the same library `ripgrep` uses. It applies a directory's `.gitignore` rules on the fly, which means we get the user's "ignore `target/`" decision for free instead of hard-coding skip lists. It also has a fast parallel walker that keeps big trees responsive.

The alternative — a manual `fs::read_dir` loop with our own skip list — would re-implement the rules and miss anything project-specific.

## Discovery Algorithm

1. Read each path from `config.projects_root`.
2. For each root, start a parallel walker.
3. When the walker enters a directory that has a `.git` entry, record a `Project` and prune descent (we don't list repos inside repos).
4. Apply `.gitignore` rules to skip matched subtrees.
5. Canonicalise each project path, deduplicate, sort.

`O(N)` in the number of visited directories, `O(R)` in the number of projects found. The walker is bounded by the filesystem, not by the algorithm.

## Output

```rust
pub struct Project {
    pub name: String,
    pub path: PathBuf,
}
```

`name` is the directory's file name; `path` is the canonicalised absolute path. Downstream commands (`dev project list`, `dev open <name>`, the planned TUI) consume this list directly.

## Future Work

- Git status (clean / dirty / untracked) per project — Sprint 2.
- Cache results with a short TTL so a refresh isn't necessary on every command. The cache will be invalidated when the user adds a project.
- Watch roots with `notify` and refresh the cache automatically.

## See Also

- [docs/roadmap.md](roadmap.md) — the Git integration sprint that builds on this
- [docs/project-structure.md](project-structure.md) — where `scanner.rs` fits in the layout
