# Getting Started

Welcome to "Building dev-cli in Rust"! This guide teaches you Rust through building a real, production-quality CLI tool.

## What You'll Learn

By working through this guide, you'll gain hands-on experience with:

- **Rust fundamentals:** Variables, functions, error handling
- **Ownership model:** Understanding Rust's unique memory safety
- **Modules and organization:** Building well-structured code
- **Type system:** Enums, structs, traits
- **CLI development:** Parsing arguments, displaying output
- **Configuration management:** Reading and writing files
- **Testing:** Writing and running tests
- **Real-world patterns:** What actual production code looks like

## What We're Building

The **dev-cli** project is a command-line tool that helps developers manage their Git repositories and launch them in IDEs.

```bash
$ dev project list
Configured Project Roots:
📁 C:\Users\you\Projects

$ dev ide list
Installed IDEs:
✓ VS Code
✓ Cursor

$ dev open MyProject --ide cursor
# Opens MyProject in Cursor IDE
```

It's small enough to understand completely, yet complex enough to demonstrate real Rust patterns.

## Prerequisites

- **Basic programming knowledge:** Familiarity with another language helps
- **Computer access:** Windows, macOS, or Linux with Rust installed
- **Text editor:** VS Code, Vim, or your favorite editor
- **About 4-6 hours:** This guide includes hands-on coding

## How to Use This Guide

Each chapter builds on previous concepts. You can:

1. **Read actively:** Type examples into your own project
2. **Experiment:** Modify code and see what breaks
3. **Reference:** Return to chapters when you need clarification
4. **Practice:** Exercises at the end of each chapter

## A Word About Rust

Rust has a reputation for being difficult to learn. This is often because:

- The compiler is very strict
- Error messages can be long
- The borrow checker enforces ownership rules

But here's the secret: **These are features, not bugs.**

They prevent entire categories of bugs at compile time. Yes, you'll fight the compiler. But once your code compiles, it usually works correctly.

As you progress through this guide, the compiler will become your ally rather than your adversary.

## Installing Rust

If you haven't already:

```bash
# Download and install from https://rustup.rs/
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:

```bash
rustc --version
cargo --version
```

## Next Steps

Ready to start? Head to [What We'll Build](02-project-overview.md) to understand the project architecture.

## Need Help?

- **Rust Book:** https://doc.rust-lang.org/book/
- **Official Docs:** https://doc.rust-lang.org/
- **Community:** https://www.rust-lang.org/community
- **Questions:** Check our [Troubleshooting](C-troubleshooting.md) section
