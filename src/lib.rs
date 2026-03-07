//! # agent-policy
//!
//! Schema-first generator for coding-agent repo policies and compatibility files.
//!
//! ## Usage
//!
//! ```no_run
//! use camino::Utf8Path;
//!
//! let policy = agent_policy::load(Utf8Path::new("agent-policy.yaml"))
//!     .expect("failed to load policy");
//!
//! println!("Project: {}", policy.project.name);
//! ```
//!
//! ## Command-line interface
//!
//! See the [README](https://github.com/CameronBrooks11/agent-policy) for CLI documentation.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod cli;
pub mod commands;
pub mod error;
pub mod load;
pub mod model;
pub mod render;
pub(crate) mod util;

pub use error::{Error, Result};
pub use model::normalized::Policy;

/// Load, validate, and normalize an `agent-policy.yaml` from a file path.
///
/// This is the main entry point for the entire load pipeline.
///
/// # Errors
///
/// Returns an [`Error`] if the file cannot be read, the YAML is invalid, the
/// document fails schema validation, or normalization encounters invalid values.
pub fn load(path: &camino::Utf8Path) -> Result<Policy> {
    let raw = load::load_file(path)?;
    model::normalize(raw)
}
