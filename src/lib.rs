//! Localearn provides local-first study tooling with a CLI, generation pipeline, and TUI shell.

pub mod app;
pub mod cli;
pub mod error;
pub mod fs;
pub mod generation;
pub mod providers;
pub mod study_set;
pub mod tui;

pub use app::Application;
pub use error::{Result, StudyError};
