//! Core data structures and types.
//!
//! This module contains all data models used throughout dev-cli.
//! Models are serializable and represent domain concepts.
//!
//! # Types
//!
//! - [`ide::Ide`] — Supported IDE type enum
//! - [`project::Project`] — Discovered Git repository
//! - [`custom_ide::CustomIde`] — User-configured custom IDE

pub mod custom_ide;
pub mod ide;
pub mod project;
pub mod recent_project;
