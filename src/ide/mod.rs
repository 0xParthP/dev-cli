//! IDE detection and launching system.
//!
//! Three-part system for finding and launching IDEs:
//!
//! 1. **Detection** — Automatically find installed IDEs on the system
//! 2. **Registry** — Store information about detected IDEs
//! 3. **Launching** — Spawn external IDE processes
//!
//! # Example
//!
//! ```no_run
//! # use anyhow::Result;
//! # fn example() -> Result<()> {
//! use dev_cli::ide::detect::detect_ides;
//! use dev_cli::ide::launcher;
//! use dev_cli::models::ide::Ide;
//! use std::path::Path;
//!
//! // Detect IDEs
//! let ides = detect_ides();
//! println!("Found {} IDEs", ides.len());
//!
//! // Launch an IDE
//! launcher::launch(Ide::Cursor, Path::new("./my-project"))?;
//! # Ok(())
//! # }
//! ```

pub mod detect;
pub mod launcher;
pub mod registry;
