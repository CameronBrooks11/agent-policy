//! Command-line interface definition.

use clap::{Parser, Subcommand};

/// Schema-first generator for coding-agent repo policies.
#[derive(Parser)]
#[command(
    name = "agent-policy",
    version,
    about = "Schema-first generator for coding-agent repo policies.",
    long_about = "Generates AGENTS.md, CLAUDE.md, and .cursor/rules from a canonical agent-policy.yaml.\n\nSee https://github.com/CameronBrooks11/agent-policy for documentation."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Agent-policy subcommands.
#[derive(Subcommand)]
pub enum Command {
    /// Write a starter agent-policy.yaml to the current directory.
    Init {
        /// Overwrite existing agent-policy.yaml if present.
        #[arg(long)]
        force: bool,
    },

    /// Generate all enabled output files from agent-policy.yaml.
    Generate {
        /// Path to agent-policy.yaml.
        #[arg(long, short, default_value = "agent-policy.yaml")]
        config: camino::Utf8PathBuf,
    },

    /// Check that committed generated files match the current policy.
    ///
    /// Exits non-zero if any generated file is stale or missing.
    Check {
        /// Path to agent-policy.yaml.
        #[arg(long, short, default_value = "agent-policy.yaml")]
        config: camino::Utf8PathBuf,
    },

    /// List all supported output targets and their output paths.
    #[command(name = "list-targets")]
    ListTargets,
}
