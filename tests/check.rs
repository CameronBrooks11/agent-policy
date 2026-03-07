// check command integration tests — Phase 3

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn agent_policy() -> Command {
    #[allow(deprecated)]
    let cmd = Command::cargo_bin("agent-policy").unwrap();
    cmd
}

fn setup_dir_with_generated(yaml: &str) -> TempDir {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("agent-policy.yaml"), yaml).unwrap();
    agent_policy()
        .arg("generate")
        .current_dir(dir.path())
        .assert()
        .success();
    dir
}

#[test]
fn check_passes_when_files_match() {
    let yaml = "project:\n  name: test\noutputs:\n  - agents-md\n";
    let dir = setup_dir_with_generated(yaml);
    agent_policy()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(contains("up to date"));
}

#[test]
fn check_fails_when_generated_file_missing() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("agent-policy.yaml"),
        "project:\n  name: test\noutputs:\n  - agents-md\n",
    )
    .unwrap();
    // Do NOT run generate — AGENTS.md is missing
    agent_policy()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(contains("missing"));
}

#[test]
fn check_fails_when_file_is_stale() {
    let yaml = "project:\n  name: original\noutputs:\n  - agents-md\n";
    let dir = setup_dir_with_generated(yaml);

    // Change the policy without regenerating
    std::fs::write(
        dir.path().join("agent-policy.yaml"),
        "project:\n  name: changed\noutputs:\n  - agents-md\n",
    )
    .unwrap();

    agent_policy()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(contains("stale"));
}

#[test]
fn check_diff_output_goes_to_stderr() {
    let yaml = "project:\n  name: original\noutputs:\n  - agents-md\n";
    let dir = setup_dir_with_generated(yaml);

    std::fs::write(
        dir.path().join("agent-policy.yaml"),
        "project:\n  name: different\noutputs:\n  - agents-md\n",
    )
    .unwrap();

    // Stdout should be empty on failure; diff is on stderr
    agent_policy()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stdout("")
        .stderr(contains("---"));
}

#[test]
fn check_run_generate_check_roundtrip() {
    let yaml = r#"
project:
  name: website
  summary: Test site.
commands:
  test: npm test
paths:
  editable:
    - src/**
  protected:
    - .github/**
constraints:
  forbid_secrets: true
outputs:
  - agents-md
  - claude-md
"#;
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("agent-policy.yaml"), yaml).unwrap();

    agent_policy()
        .arg("generate")
        .current_dir(dir.path())
        .assert()
        .success();

    agent_policy()
        .arg("check")
        .current_dir(dir.path())
        .assert()
        .success();
}
