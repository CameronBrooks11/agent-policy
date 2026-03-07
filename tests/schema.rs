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
    let (policy, _warnings) = normalize(raw).expect("should normalize");
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
    let (policy, _warnings) = normalize(raw).expect("should normalize");
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

// ---- auto-generated paths behaviour ----

#[test]
fn generated_paths_auto_injected() {
    // When paths.generated is omitted entirely, enabled output targets
    // should automatically populate it.
    let yaml = r#"
schema_version: "1"
project:
  name: test
outputs:
  - agents-md
  - claude-md
"#;
    let raw = load_str(yaml).expect("should parse");
    let (policy, warnings) = normalize(raw).expect("should normalize");
    assert!(
        policy.paths.generated.contains(&"AGENTS.md".to_owned()),
        "AGENTS.md should be auto-injected: {:?}",
        policy.paths.generated
    );
    assert!(
        policy.paths.generated.contains(&"CLAUDE.md".to_owned()),
        "CLAUDE.md should be auto-injected: {:?}",
        policy.paths.generated
    );
    assert!(warnings.is_empty(), "no warnings expected: {warnings:?}");
}

#[test]
fn generated_paths_redundant_warns() {
    // When the user lists a path that is already implied by outputs,
    // a warning should be emitted and the path should appear exactly once.
    let yaml = r#"
schema_version: "1"
project:
  name: test
paths:
  generated:
    - AGENTS.md
outputs:
  - agents-md
"#;
    let raw = load_str(yaml).expect("should parse");
    let (policy, warnings) = normalize(raw).expect("should normalize");
    // Path present exactly once
    let count = policy
        .paths
        .generated
        .iter()
        .filter(|p| *p == "AGENTS.md")
        .count();
    assert_eq!(count, 1, "AGENTS.md should appear exactly once");
    // At least one warning about it
    assert!(
        !warnings.is_empty(),
        "expected a warning about redundant AGENTS.md"
    );
    assert!(
        warnings.iter().any(|w| w.contains("AGENTS.md")),
        "warning should mention AGENTS.md: {warnings:?}"
    );
    assert!(
        warnings
            .iter()
            .any(|w| w.contains("implied") || w.contains("remove") || w.contains("unnecessary")),
        "warning should say it can be removed: {warnings:?}"
    );
}

#[test]
fn generated_paths_user_extras_preserved() {
    // User-supplied paths that are NOT implied by outputs should be kept
    // without any warning.
    let yaml = r#"
schema_version: "1"
project:
  name: test
paths:
  generated:
    - site/**
    - dist/**
outputs:
  - agents-md
"#;
    let raw = load_str(yaml).expect("should parse");
    let (policy, warnings) = normalize(raw).expect("should normalize");
    assert!(
        policy.paths.generated.contains(&"AGENTS.md".to_owned()),
        "AGENTS.md auto-injected: {:?}",
        policy.paths.generated
    );
    assert!(
        policy.paths.generated.contains(&"site/**".to_owned()),
        "site/** preserved: {:?}",
        policy.paths.generated
    );
    assert!(
        policy.paths.generated.contains(&"dist/**".to_owned()),
        "dist/** preserved: {:?}",
        policy.paths.generated
    );
    assert!(warnings.is_empty(), "no warnings expected: {warnings:?}");
}
