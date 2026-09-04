//! Core library for dev-cli.
//!
//! This library exposes all functionality used by the `dev` binary and by
//! integration tests in the `tests/` directory.

pub mod cli;
pub mod commands;
pub mod config;
pub mod ide;
pub mod models;
pub mod onboarding;
pub mod scanner;
pub mod utils;
