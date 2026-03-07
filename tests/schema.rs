// Schema loading and normalization tests — Phase 1

use agent_policy::{load, load::load_str, model::normalize};
use camino::Utf8Path;

#[test]
fn minimal_valid_config_loads() {
    let yaml = r#"
schema_version: "1"
project:
  name: test-project
"#;
    let result = load_str(yaml);
    assert!(
        result.is_ok(),
        "minimal valid config should load: {result:?}"
    );
}

#[test]
fn missing_project_name_fails_validation() {
    let yaml = r#"
schema_version: "1"
project:
  summary: no name here
"#;
    let result = load_str(yaml);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("validation failed") || err.contains("name"),
        "error should mention validation: {err}"
    );
}

#[test]
fn unknown_top_level_key_fails() {
    let yaml = r#"
project:
  name: test
unknown_key: true
"#;
    let result = load_str(yaml);
    assert!(result.is_err(), "unknown top-level key should be rejected");
}

#[test]
fn unknown_output_target_id_fails_normalization() {
    // The JSON Schema enforces the enum of valid target IDs, so load_str
    // rejects unknown targets before normalize() even runs.
    let yaml = r#"
schema_version: "1"
project:
  name: test
outputs:
  - agents-md
  - not-a-real-target
"#;
    let result = load_str(yaml);
    assert!(result.is_err(), "unknown output target should be rejected");
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("not-a-real-target"),
        "error should name the bad target: {err}"
    );
}

#[test]
fn empty_outputs_array_fails_normalization() {
    // JSON Schema allows an empty array; normalize() must catch it.
    let yaml = r#"
schema_version: "1"
project:
  name: test
outputs: []
"#;
    let raw = load_str(yaml).expect("should pass schema validation");
    let result = normalize(raw);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("No outputs") || err.contains("outputs"),
        "error should mention outputs: {err}"
    );
}

#[test]
fn full_config_normalizes_correctly() {
    let yaml = r#"
schema_version: "1"
project:
  name: website
  summary: Marketing website.
commands:
  install: npm install
  test: npm test
paths:
  editable:
    - src/**
  protected:
    - .github/**
roles:
  docs_agent:
    editable:
      - docs/**
    forbidden:
      - src/**
constraints:
  forbid_secrets: true
outputs:
  - agents-md
  - claude-md
"#;
    let raw = load_str(yaml).expect("should parse");
    let policy = normalize(raw).expect("should normalize");
    assert_eq!(policy.project.name, "website");
    assert_eq!(
        policy.project.summary.as_deref(),
        Some("Marketing website.")
    );
    assert!(policy.outputs.agents_md);
    assert!(policy.outputs.claude_md);
    assert!(!policy.outputs.cursor_rules);
    assert!(policy.constraints.forbid_secrets);
    assert!(!policy.constraints.require_tests_for_code_changes);
    assert_eq!(policy.paths.editable, vec!["src/**"]);
    assert_eq!(policy.paths.protected, vec![".github/**"]);
    let role = policy
        .roles
        .get("docs_agent")
        .expect("docs_agent role should exist");
    assert_eq!(role.editable, vec!["docs/**"]);
    assert_eq!(role.forbidden, vec!["src/**"]);
}

#[test]
fn outputs_defaults_to_agents_md_when_omitted() {
    let yaml = r#"
schema_version: "1"
project:
  name: bare
"#;
    let raw = load_str(yaml).expect("should parse");
    let policy = normalize(raw).expect("should normalize");
    assert!(
        policy.outputs.agents_md,
        "agents-md should be on by default"
    );
    assert!(!policy.outputs.claude_md);
    assert!(!policy.outputs.cursor_rules);
}

#[test]
fn invalid_role_name_fails_normalization() {
    let yaml = r#"
schema_version: "1"
project:
  name: test
roles:
  "my bad role name":
    editable: []
"#;
    let raw = load_str(yaml).expect("yaml parses fine");
    let result = normalize(raw);
    assert!(result.is_err(), "invalid role name should be rejected");
}

#[test]
fn missing_schema_version_fails_validation() {
    let yaml = r#"
project:
  name: test
"#;
    let result = load_str(yaml);
    assert!(result.is_err(), "missing schema_version should be rejected");
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("schema_version"),
        "error should mention schema_version: {err}"
    );
}

#[test]
fn examples_minimal_loads() {
    let path = Utf8Path::new("examples/minimal/agent-policy.yaml");
    let result = load(path);
    assert!(result.is_ok(), "examples/minimal should load: {result:?}");
}
