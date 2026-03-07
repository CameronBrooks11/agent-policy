// JSON Schema validation — implemented in Phase 1

use crate::error::{Error, Result};
use std::sync::OnceLock;

const SCHEMA_JSON: &str = include_str!("../../agent-policy.schema.json");

/// Return a reference to the compiled JSON Schema validator.
///
/// The validator is compiled exactly once (on first call) and cached for the
/// lifetime of the process. Compiling a JSON Schema is expensive; calling
/// `validator_for` on every `validate()` invocation would be measurable
/// overhead in test suites and CI pipelines that load many configs.
#[allow(clippy::expect_used)] // panics are on invariants about bundled binary content
fn compiled_validator() -> &'static jsonschema::Validator {
    static VALIDATOR: OnceLock<jsonschema::Validator> = OnceLock::new();
    VALIDATOR.get_or_init(|| {
        let schema: serde_json::Value =
            serde_json::from_str(SCHEMA_JSON).expect("bundled schema is always valid JSON");
        jsonschema::validator_for(&schema).expect("bundled schema always compiles")
    })
}

/// Validate a parsed YAML document against the bundled JSON Schema.
///
/// The input must be a `serde_json::Value` representation of the raw policy.
/// Convert via `serde_json::to_value(&raw_policy)`.
///
/// # Errors
///
/// Returns [`crate::Error::Schema`] if the document violates the bundled JSON Schema.
pub fn validate(doc: &serde_json::Value) -> Result<()> {
    let validator = compiled_validator();

    let errors: Vec<String> = validator
        .iter_errors(doc)
        .map(|e| format!("  - {e}"))
        .collect();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(Error::Schema(errors.join("\n")))
    }
}
