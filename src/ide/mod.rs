//! IDE detection and launching system.
//!
//! Three-part system for finding and launching IDEs:
//!
//! 1. **Detection** — Automatically find installed IDEs on the system
//! 2. **Registry** — Store information about detected IDEs
//! 3. **Launching** — Spawn external IDE processes

pub mod detect;
pub mod launcher;
pub mod registry;
