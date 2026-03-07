# Phase 3 — Integrity

**Goal:** `agent-policy check` detects when committed generated files are out of sync with the current policy. CI uses it to prevent drift. The tool is used to generate its own `AGENTS.md` (self-dogfooding). All checks are covered by automated tests.

**Depends on:** Phase 2 (`render_all()` and the full generation pipeline)  
**Unlocks:** Phase 4 (external real-world adoption, polish, path-scoped rules)

---

## Overview

`check` runs the exact same pipeline as `generate`, but instead of writing files to disk, it reads each committed file, compares it to what would be generated, and reports any differences. This is the command consuming repos run in CI.

```
agent-policy.yaml
    ↓ load + normalize + render_all()     (in memory)
Vec<RenderedOutput>
    ↓ for each output:
        read committed file from disk
        normalize line endings (→ \n)
        compare strings
    ↓
exit 0    if all match
exit 1    if any differ or any file is missing
          print unified diff to stderr for each mismatch
```

The render pipeline is **not duplicated** — `check` imports and calls the same `render_all()` function from Phase 2.

---

## New Dependency

```toml
[dependencies]
# Add to existing deps:
similar = "2"
```

---

## Diff Utility (`src/util/diff.rs`)

```rust
use similar::{ChangeTag, TextDiff};
use std::fmt::Write;

/// Format a human-readable unified diff between `old` and `new`.
///
/// `label` is used as the file path annotation in the diff header.
pub fn unified_diff(label: &str, old: &str, new: &str) -> String {
    let diff = TextDiff::from_lines(old, new);
    let mut out = String::new();

    writeln!(out, "--- {label} (committed)").unwrap();
    writeln!(out, "+++ {label} (generated)").unwrap();

    for group in diff.grouped_ops(3) {
        writeln!(out, "@@").unwrap();
        for op in group {
            for change in diff.iter_inline_changes(&op) {
                let prefix = match change.tag() {
                    ChangeTag::Delete => '-',
                    ChangeTag::Insert => '+',
                    ChangeTag::Equal  => ' ',
                };
                write!(out, "{prefix}").unwrap();
                for (_, s) in change.iter_strings_lossy() {
                    write!(out, "{s}").unwrap();
                }
                if change.missing_newline() {
                    writeln!(out).unwrap();
                }
            }
        }
    }
    out
}

/// Normalize line endings to `\n` before comparison.
///
/// This prevents false diff failures on Windows where files may be written
/// or checked out with `\r\n` endings.
pub fn normalize_line_endings(s: &str) -> String {
    s.replace("\r\n", "\n")
}
```

---

## `check` Command (`src/commands/check.rs`)

```rust
use crate::{
    error::{Error, Result},
    load, model::normalize, render,
    util::{diff, fs::read_if_exists},
};
use camino::Utf8Path;

/// Result of checking one output file.
enum FileCheck {
    Ok,
    Missing { path: String },
    Stale { path: String, diff: String },
}

pub fn run(config: &Utf8Path) -> Result<()> {
    let raw = load::load_file(config)?;
    let policy = normalize(raw)?;
    let outputs = render::render_all(&policy)?;

    let mut checks: Vec<FileCheck> = Vec::new();

    for output in &outputs {
        let generated = diff::normalize_line_endings(&output.content);
        let committed = read_if_exists(output.path.as_std_path())?;

        match committed {
            None => {
                checks.push(FileCheck::Missing {
                    path: output.path.to_string(),
                });
            }
            Some(committed_raw) => {
                let committed = diff::normalize_line_endings(&committed_raw);
                if committed != generated {
                    let d = diff::unified_diff(&output.path, &committed, &generated);
                    checks.push(FileCheck::Stale {
                        path: output.path.to_string(),
                        diff: d,
                    });
                } else {
                    checks.push(FileCheck::Ok);
                }
            }
        }
    }

    let failures: Vec<&FileCheck> = checks
        .iter()
        .filter(|c| !matches!(c, FileCheck::Ok))
        .collect();

    if failures.is_empty() {
        let count = checks.len();
        println!("✓ All {count} generated file(s) are up to date.");
        return Ok(());
    }

    eprintln!("Generated files are out of date:\n");
    for check in &failures {
        match check {
            FileCheck::Missing { path } => {
                eprintln!("  missing  {path}");
                eprintln!("  → Run: agent-policy generate\n");
            }
            FileCheck::Stale { path, diff: d } => {
                eprintln!("  stale    {path}");
                eprintln!("{d}");
            }
            FileCheck::Ok => unreachable!(),
        }
    }

    eprintln!("Run `agent-policy generate` to update.");
    // Return error to trigger non-zero exit in main.rs
    Err(Error::CheckFailed {
        path: failures
            .first()
            .map(|c| match c {
                FileCheck::Missing { path } | FileCheck::Stale { path, .. } => {
                    std::path::PathBuf::from(path)
                }
                FileCheck::Ok => unreachable!(),
            })
            .unwrap(),
    })
}
```

**Exit code behavior:**

| Situation            | Exit code | Output                                              |
| -------------------- | --------- | --------------------------------------------------- |
| All files match      | 0         | stdout: "✓ All N generated file(s) are up to date." |
| Any file missing     | 1         | stderr: file name + remediation hint                |
| Any file differs     | 1         | stderr: unified diff for each stale file            |
| Load/normalize error | 1         | stderr: error message                               |

Stdout is reserved for success messages. All failure output goes to stderr. This is the correct UNIX convention and ensures CI log parsers can distinguish output types.

---

## CI Update (`.github/workflows/ci.yml`)

Add a `policy-check` job after the `test` job. This job:

1. Builds the binary from source
2. Runs `agent-policy check` against this repo's own `agent-policy.yaml`

```yaml
policy-check:
  name: Policy check
  runs-on: ubuntu-latest
  needs: [check] # ensure it compiles before checking
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - uses: Swatinem/rust-cache@v2
    - name: Build agent-policy
      run: cargo build --release
    - name: Check generated files
      run: ./target/release/agent-policy check
```

This step fails if anyone edits `AGENTS.md` or other generated files by hand instead of through `agent-policy generate`.

---

## Self-Dogfooding: `agent-policy.yaml` for This Repo

In Phase 3, the tool's own repository adopts itself. Add `agent-policy.yaml` to the repo root:

```yaml
# agent-policy.yaml — policy for the agent-policy tool repo itself
project:
  name: agent-policy
  summary: Schema-first generator for coding-agent repo policies and compatibility files.

commands:
  test: cargo test
  lint: cargo clippy --all-targets -- -D warnings
  build: cargo build --release

paths:
  editable:
    - src/**
    - templates/**
    - tests/**
    - examples/**
  protected:
    - .github/workflows/**
    - agent-policy.schema.json
    - Cargo.toml
  generated:
    - AGENTS.md
    - CLAUDE.md

constraints:
  forbid_secrets: true
  require_tests_for_code_changes: true

outputs:
  agents_md: true
  claude_md: true
  cursor_rules: false
```

Then:

1. Run `agent-policy generate` to produce `AGENTS.md` and `CLAUDE.md`
2. Review the output — confirm it reads naturally
3. Commit `agent-policy.yaml`, `AGENTS.md`, and `CLAUDE.md`
4. CI now enforces that those files stay in sync

This is the proof that the tool works in a real repo. If something is broken or awkward, fix it now before Phase 4 external adoption.

---

## `check` Command Tests (`tests/check.rs`)

```rust
use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn agent_policy() -> Command {
    Command::cargo_bin("agent-policy").unwrap()
}

fn setup_dir_with_generated(yaml: &str) -> TempDir {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("agent-policy.yaml"), yaml).unwrap();
    // Generate first
    agent_policy()
        .arg("generate")
        .current_dir(dir.path())
        .assert()
        .success();
    dir
}

#[test]
fn check_passes_when_files_match() {
    let yaml = "project:\n  name: test\noutputs:\n  agents_md: true\n";
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
        "project:\n  name: test\noutputs:\n  agents_md: true\n",
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
    let yaml = "project:\n  name: original\noutputs:\n  agents_md: true\n";
    let dir = setup_dir_with_generated(yaml);

    // Change the policy without regenerating
    std::fs::write(
        dir.path().join("agent-policy.yaml"),
        "project:\n  name: changed\noutputs:\n  agents_md: true\n",
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
    let yaml = "project:\n  name: original\noutputs:\n  agents_md: true\n";
    let dir = setup_dir_with_generated(yaml);

    std::fs::write(
        dir.path().join("agent-policy.yaml"),
        "project:\n  name: different\noutputs:\n  agents_md: true\n",
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
    // Verify that generate then check always exits 0
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
  agents_md: true
  claude_md: true
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
```

Add `tests/check.rs` to the project. Register the test file by ensuring `tests/` is scanned automatically (it is, by default with Cargo's test discovery).

---

## README Updates

After Phase 3, the README can be completed with the full "Quick start" and "CI integration" sections.

### Consuming repo workflow (add to README)

````markdown
## Usage

### 1. Create a policy

```bash
agent-policy init
```
````

Edit the generated `agent-policy.yaml` for your project.

### 2. Generate output files

```bash
agent-policy generate
```

This writes `AGENTS.md` (and any other enabled outputs) to your repo. Commit them.

### 3. Check for drift in CI

Add to your CI workflow:

```yaml
- name: Check agent policy
  run: agent-policy check
```

This step exits non-zero if generated files are out of date with `agent-policy.yaml`.

```

---

## Exit Condition

Phase 3 is complete when all of the following are true:

- [ ] `agent-policy check` exits 0 when all generated files match the policy
- [ ] `agent-policy check` exits 1 when any file is missing, with a clear hint to run `generate`
- [ ] `agent-policy check` exits 1 when any file differs, printing a unified diff to stderr
- [ ] Diff output goes to stderr; success message goes to stdout
- [ ] Line endings are normalized before comparison (no false failures on Windows)
- [ ] All `check` tests in `tests/check.rs` pass
- [ ] The roundtrip test (`generate` → `check` → success) passes
- [ ] `agent-policy.yaml` is added to this repo's root
- [ ] `AGENTS.md` and `CLAUDE.md` are generated and committed for this repo
- [ ] The `policy-check` CI job is added to `.github/workflows/ci.yml` and passes
- [ ] README "Usage" and "CI integration" sections are complete and accurate
- [ ] `cargo test` passes with zero failures
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo doc --no-deps` passes with no warnings
```
