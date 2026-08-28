//! Core data structures and types.
//!
//! This module contains all data models used throughout dev-cli.
//! Models are serializable and represent domain concepts.
//!
//! # Types
//!
//! - [`ide::Ide`] — Supported IDE type enum
//! - [`project::Project`] — Discovered Git repository

pub mod ide;
pub mod project;
