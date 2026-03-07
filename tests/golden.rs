// Golden output tests — Phase 2

use agent_policy::{load::load_str, model::normalize, render::render_all};

fn render_yaml(yaml: &str) -> Vec<(String, String)> {
    let raw = load_str(yaml).expect("parse failed");
    let policy = normalize(raw).expect("normalize failed");
    let outputs = render_all(&policy).expect("render failed");
    outputs
        .into_iter()
        .map(|o| (o.path.to_string(), o.content))
        .collect()
}

#[test]
fn golden_minimal() {
    let yaml = std::fs::read_to_string("examples/minimal/agent-policy.yaml").unwrap();
    for (path, content) in render_yaml(&yaml) {
        let name = format!("minimal__{path}");
        insta::assert_snapshot!(name, content);
    }
}

#[test]
fn golden_all_outputs_enabled() {
    let yaml = r#"
project:
  name: website
  summary: Marketing website.
commands:
  install: npm install
  test: npm test
  lint: npm run lint
paths:
  editable:
    - src/**
    - docs/**
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
  require_tests_for_code_changes: true
outputs:
  - agents-md
  - claude-md
  - cursor-rules
"#;
    for (path, content) in render_yaml(yaml) {
        let name = format!("all_outputs__{path}");
        insta::assert_snapshot!(name, content);
    }
}

#[test]
fn golden_no_commands_no_roles() {
    let yaml = r#"
project:
  name: bare-bones
outputs:
  - agents-md
"#;
    for (path, content) in render_yaml(yaml) {
        let name = format!("no_commands_no_roles__{path}");
        insta::assert_snapshot!(name, content);
    }
}
