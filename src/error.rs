// Error types — implemented in Phase 1

use std::path::PathBuf;

/// All errors produced by agent-policy.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An I/O operation failed.
    #[error("I/O error reading '{path}': {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// The YAML file could not be parsed.
    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// The policy failed JSON Schema validation.
    #[error("Policy validation failed:\n{0}")]
    Schema(String),

    /// A glob pattern in the policy is invalid.
    #[error("Invalid glob pattern '{pattern}': {source}")]
    Glob {
        pattern: String,
        #[source]
        source: globset::Error,
    },

    /// A role name contains invalid characters.
    #[error("Invalid role name '{name}': use only lowercase letters, digits, and underscores")]
    InvalidRoleName { name: String },

    /// Template rendering failed.
    #[error("Render error for target '{target}': {source}")]
    Render {
        target: String,
        #[source]
        source: minijinja::Error,
    },

    /// Generated file content differs from the committed file.
    #[error("Stale generated file: {}", path.display())]
    CheckFailed { path: PathBuf },

    /// The outputs list is present but empty.
    #[error(
        "No outputs are enabled. Add at least one target ID to `outputs` (e.g. `outputs: [agents-md]`)."
    )]
    NoOutputs,

    /// An unrecognized target ID was specified in outputs.
    #[error(
        "Unknown output target '{id}'. Supported targets for this version: agents-md, claude-md, cursor-rules."
    )]
    UnknownTarget { id: String },
}

/// Convenience alias.
pub type Result<T> = std::result::Result<T, Error>;
