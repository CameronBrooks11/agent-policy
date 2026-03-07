// Load and validate pipeline — implemented in Phase 1

pub mod schema;
pub mod yaml;

use crate::{error::Result, model::policy::RawPolicy};

/// Load and validate an `agent-policy.yaml` from a string.
///
/// Parses YAML, validates against the bundled JSON Schema, and returns the
/// raw policy struct on success. The caller is responsible for normalization.
///
/// # Errors
///
/// Returns [`crate::Error::Yaml`] on YAML parse failure or [`crate::Error::Schema`] if the
/// document does not conform to the bundled JSON Schema.
///
/// # Panics
///
/// Panics if `RawPolicy` cannot be serialized to a JSON value, which is an
/// internal invariant that holds as long as all field types implement `Serialize`.
#[allow(clippy::expect_used)] // RawPolicy derives Serialize; to_value is infallible for these types
pub fn load_str(input: &str) -> Result<RawPolicy> {
    let raw = yaml::parse(input)?;
    let doc = serde_json::to_value(&raw)
        .expect("RawPolicy is always serializable to JSON");
    schema::validate(&doc)?;
    Ok(raw)
}

/// Load and validate an `agent-policy.yaml` from a file path.
///
/// # Errors
///
/// Returns [`crate::Error::Io`] if the file cannot be read, [`crate::Error::Yaml`] on YAML
/// parse failure, or [`crate::Error::Schema`] if the document does not conform to
/// the bundled JSON Schema.
pub fn load_file(path: &camino::Utf8Path) -> Result<RawPolicy> {
    let content = std::fs::read_to_string(path).map_err(|e| crate::error::Error::Io {
        path: path.as_std_path().to_owned(),
        source: e,
    })?;
    load_str(&content)
}
