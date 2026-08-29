# Repository Scanner

The repository scanner is responsible for discovering Git repositories inside configured project roots.

## Responsibilities

- Walk project roots recursively.
- Ignore build directories.
- Detect `.git` repositories.
- Return `Vec<Project>`.

## Why use the `ignore` crate?

Explain traversal optimisations.

## Discovery Algorithm

1. Read roots from config.
2. Walk recursively.
3. Ignore excluded folders.
4. Detect `.git`.
5. Canonicalise path.
6. Deduplicate.
7. Sort alphabetically.

## Complexity

Time complexity: O(N)

Space complexity: O(R)