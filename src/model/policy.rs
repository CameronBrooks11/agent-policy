// Raw policy types — directly deserialized from YAML.
// Do not use these types in renderers — use the normalized model instead.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Raw deserialized policy as it appears in agent-policy.yaml.
///
/// `Serialize` is required so `serde_json::to_value(&raw)` compiles in the load pipeline.
/// All `Option` fields use `skip_serializing_if` so `None` values are **omitted** from the
/// JSON rather than serialized as `null` — the JSON Schema uses plain type checks (not
/// `["T", "null"]`), so a `null` value would fail validation.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawPolicy {
    /// Declares the schema version. Use `"1"` for all new files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    pub project: RawProject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<RawCommands>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<RawPaths>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<IndexMap<String, RawRole>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<RawConstraints>,
    /// List of output target IDs.
    /// Valid values: `"agents-md"`, `"claude-md"`, `"cursor-rules"`, `"gemini-md"`, `"copilot-instructions"`.
    /// Defaults to `["agents-md"]` when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawProject {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawPaths {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawRole {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawConstraints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_tests_for_code_changes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbid_secrets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_human_review_for_protected_paths: Option<bool>,
}
