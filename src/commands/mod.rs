//! Command implementations.
//!
//! This module contains the logic for each CLI command.
//! Each submodule implements one or more related commands.
//!
//! # Commands
//!
//! - [`config`] — Configuration management (`dev config`)
//! - [`ide`] — IDE management (`dev ide`)
//! - [`project`] — Project management (`dev project`)

pub mod config;
pub mod ide;
pub mod project;
