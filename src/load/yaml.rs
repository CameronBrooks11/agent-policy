// YAML parsing — implemented in Phase 1

use crate::{error::Result, model::policy::RawPolicy};

/// Parse a raw YAML string into a [`RawPolicy`].
///
/// # Errors
///
/// Returns [`crate::Error::Yaml`] if the input is not valid YAML or cannot be
/// deserialized into [`RawPolicy`].
pub fn parse(input: &str) -> Result<RawPolicy> {
    serde_yaml::from_str(input).map_err(crate::error::Error::Yaml)
}
