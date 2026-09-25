//! Ratatui dashboard.

pub mod actions;
pub mod app;
pub mod data;
pub mod event;
pub mod state;
pub mod theme;

pub mod tree;
pub mod ui;
pub mod widgets;

pub use app::run;
