//! # agent-policy
//!
//! Schema-first generator for coding-agent repo policies and compatibility files.
//!
//! See the [README](https://github.com/CameronBrooks11/agent-policy) for usage.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod commands;
pub mod error;
pub mod load;
pub mod model;
pub mod render;
pub(crate) mod util;
