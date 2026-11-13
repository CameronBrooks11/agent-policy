// CLI integration tests — Phase 2

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn agent_policy() -> Command {
    #[allow(deprecated)]
    let cmd = Command::cargo_bin("agent-policy").unwrap();
    cmd
}

#[test]
fn help_exits_zero() {
    agent_policy().arg("--help").assert().success();
}

#[test]
fn version_exits_zero() {
    agent_policy().arg("--version").assert().success();
}

#[test]
fn init_creates_config_file() {
    let dir = TempDir::new().unwrap();
    agent_policy()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();
    assert!(dir.path().join("agent-policy.yaml").exists());
}

#[test]
fn init_does_not_overwrite_without_force() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("agent-policy.yaml"),
        "project:\n  name: existing\n",
    )
    .unwrap();
    agent_policy()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .failure();
}

#[test]
fn init_force_overwrites() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("agent-policy.yaml"),
        "project:\n  name: existing\n",
    )
    .unwrap();
    agent_policy()
        .args(["init", "--force"])
        .current_dir(dir.path())
        .assert()
        .success();
}

/// Run `init` in a fresh directory, apply `edit` to the starter config, then
/// run `generate` and `lint`. Returns the directory and both commands'
/// combined stdout + stderr.
fn init_then_generate_and_lint(edit: impl Fn(String) -> String) -> (TempDir, String) {
    let dir = TempDir::new().unwrap();
    agent_policy()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();
    let config = dir.path().join("agent-policy.yaml");
    let yaml = std::fs::read_to_string(&config).unwrap();
    std::fs::write(&config, edit(yaml)).unwrap();

    let mut output = String::new();
    for cmd in ["generate", "lint"] {
        let out = agent_policy()
            .arg(cmd)
            .current_dir(dir.path())
            .assert()
            .success()
            .get_output()
            .clone();
        output.push_str(&String::from_utf8_lossy(&out.stdout));
        output.push_str(&String::from_utf8_lossy(&out.stderr));
    }
    (dir, output)
}

#[test]
fn init_config_generates_and_lints_without_warnings() {
    let (_dir, output) = init_then_generate_and_lint(|yaml| yaml);
    assert!(!output.to_lowercase().contains("warning"), "{output}");
}

#[test]
fn init_config_with_suggested_outputs_enabled_has_no_warnings() {
    // The template suggests uncommenting these targets.
    let (_dir, output) = init_then_generate_and_lint(|yaml| {
        yaml.replace("  # - claude-md ", "  - claude-md ")
            .replace("  # - cursor-rules ", "  - cursor-rules ")
    });
    assert!(
        output.contains("CLAUDE.md"),
        "claude-md was not enabled: {output}"
    );
    assert!(!output.to_lowercase().contains("warning"), "{output}");
}

#[test]
fn init_config_lists_no_agent_outputs_as_build_artifacts() {
    let (dir, _output) = init_then_generate_and_lint(|yaml| yaml);
    let agents_md = std::fs::read_to_string(dir.path().join("AGENTS.md")).unwrap();
    assert!(
        !agents_md.contains("Project Build Artifacts"),
        "{agents_md}"
    );
}

#[test]
fn generate_minimal_produces_agents_md() {
    let dir = TempDir::new().unwrap();
    std::fs::copy(
        "examples/minimal/agent-policy.yaml",
        dir.path().join("agent-policy.yaml"),
    )
    .unwrap();
    agent_policy()
        .arg("generate")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(contains("AGENTS.md"));
    assert!(dir.path().join("AGENTS.md").exists());
}

#[test]
fn generate_invalid_config_exits_nonzero_with_message() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("agent-policy.yaml"),
        "not_a_valid_key: oops\n",
    )
    .unwrap();
    agent_policy()
        .arg("generate")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(contains("error"));
}

#[test]
fn list_targets_runs_and_includes_all_ids() {
    agent_policy()
        .arg("list-targets")
        .assert()
        .success()
        .stdout(predicates::str::contains("agents-md"))
        .stdout(predicates::str::contains("gemini-md"))
        .stdout(predicates::str::contains("copilot-instructions"))
        .stdout(predicates::str::contains("AGENTS.md"))
        .stdout(predicates::str::contains(".github/copilot-instructions.md"));
}

#[test]
fn generate_missing_config_exits_nonzero() {
    let dir = TempDir::new().unwrap();
    agent_policy()
        .arg("generate")
        .current_dir(dir.path())
        .assert()
        .failure();
}

#[test]
fn lint_exits_zero_on_valid_config() {
    let dir = TempDir::new().unwrap();
    std::fs::copy(
        "examples/minimal/agent-policy.yaml",
        dir.path().join("agent-policy.yaml"),
    )
    .unwrap();
    agent_policy()
        .arg("lint")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(contains("Lint passed"));
}

#[test]
fn lint_exits_nonzero_on_path_conflicts() {
    let dir = TempDir::new().unwrap();
    let bad_config = r#"
schema_version: "1"
project:
  name: bad
paths:
  editable:
    - src/**
  protected:
    - src/**
"#;
    std::fs::write(dir.path().join("agent-policy.yaml"), bad_config).unwrap();
    agent_policy()
        .arg("lint")
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(contains("Path conflict"))
        .stderr(contains("Lint failed with errors"));
}

#[test]
fn lint_emits_warning_on_empty_role_scope() {
    let dir = TempDir::new().unwrap();
    let warning_config = r#"
schema_version: "1"
project:
  name: warn_mode
roles:
  empty_role:
    editable: []
"#;
    std::fs::write(dir.path().join("agent-policy.yaml"), warning_config).unwrap();
    agent_policy()
        .arg("lint")
        .current_dir(dir.path())
        .assert()
        .success() // Should succeed because it is only a warning!
        .stdout(contains("Lint passed"))
        .stdout(contains("Warning"));
}
