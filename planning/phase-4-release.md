# Phase 4 — Hardening and Release

**Goal:** The tool is robust, well-documented, and published to crates.io as `v0.1.0`. Path-scoped Cursor rules are implemented. All documentation is complete. Seeking external adoption is a post-publish milestone tracked separately — it is not a blocker for tagging and publishing.

**Depends on:** Phase 3 (check command, self-dogfooding, CI)
**Unlocks:** future phases (governance, enforcement, ecosystem)

---

## Overview

Phase 4 has three parallel concerns:

1. **Feature completion** — path-scoped Cursor rules, `examples/website/` expansion
2. **Hardening** — integration testing, error message polish, edge case handling
3. **Publication readiness** — rustdoc, CHANGELOG, crates.io metadata, release workflow, `cargo publish`

All three must be complete before tagging `v0.1.0`. Do not tag and publish until every item in the exit condition is checked.

---

## Feature: Path-Scoped Cursor Rules

### Why

The single `default.mdc` generated in Phase 2 uses `alwaysApply: true` and applies globally. Cursor supports multiple `.mdc` files where each file's `globs` frontmatter field controls when it applies. This allows generating per-role rules that Cursor activates automatically based on which file is open — without requiring the always-on default to enumerate file patterns at all.

### Design

Update `cursor_rules::render` to return `Vec<RenderedOutput>` instead of `RenderedOutput`.

Output files:

- `.cursor/rules/default.mdc` — global rules, `globs: "**/*"`, applies always
- `.cursor/rules/{role_name}.mdc` — per-role rules, `globs` set to the role's editable patterns

Update `render_all()` in `src/render/mod.rs`:

```rust
pub fn render_all(policy: &Policy) -> Result<Vec<RenderedOutput>> {
    let mut outputs = Vec::new();
    if policy.outputs.agents_md {
        outputs.push(agents_md::render(policy)?);
    }
    if policy.outputs.claude_md {
        outputs.push(claude_md::render(policy)?);
    }
    if policy.outputs.cursor_rules {
        // cursor_rules returns Vec — extend rather than push
        outputs.extend(cursor_rules::render(policy)?);
    }
    Ok(outputs)
}
```

### Per-Role Template (`templates/cursor_role.mdc.j2`)

A second template for role-scoped rules:

```jinja2
---
description: "{{ project.name }} — {{ role_name }} rules"
globs: "{{ globs_pattern }}"
alwaysApply: false
---

# Role: `{{ role_name }}`

This rule applies when editing files matching: `{{ globs_pattern }}`

## Allowed paths for this role
{% for p in role.editable %}
- `{{ p }}`
{%- endfor %}

## Forbidden paths for this role
{% if role.forbidden %}
{%- for p in role.forbidden %}
- `{{ p }}`
{%- endfor %}
{%- else %}
No additional restrictions beyond global policy.
{%- endif %}

---

*Defined in `agent-policy.yaml` → roles → {{ role_name }}*
```

### Cursor Rules Renderer (`src/render/cursor_rules.rs`)

```rust
use crate::{
    error::{Error, Result},
    model::normalized::Policy,
    render::RenderedOutput,
};
use camino::Utf8PathBuf;
use minijinja::Environment;

const DEFAULT_TEMPLATE: &str = include_str!("../../templates/cursor_rule.mdc.j2");
const ROLE_TEMPLATE: &str = include_str!("../../templates/cursor_role.mdc.j2");

pub fn render(policy: &Policy) -> Result<Vec<RenderedOutput>> {
    let mut outputs = Vec::new();

    // Render the default (global) rule
    let mut env = Environment::new();
    env.add_template("default.mdc", DEFAULT_TEMPLATE)
        .map_err(|e| Error::Render { target: ".cursor/rules/default.mdc".to_owned(), source: e })?;
    let tmpl = env.get_template("default.mdc")
        .map_err(|e| Error::Render { target: ".cursor/rules/default.mdc".to_owned(), source: e })?;
    let content = tmpl
        .render(minijinja::context! {
            project => policy.project,
            commands => policy.commands,
            paths => policy.paths,
            constraints => policy.constraints,
        })
        .map_err(|e| Error::Render { target: ".cursor/rules/default.mdc".to_owned(), source: e })?;
    outputs.push(RenderedOutput {
        path: Utf8PathBuf::from(".cursor/rules/default.mdc"),
        content,
    });

    // Render per-role rules
    if !policy.roles.is_empty() {
        let mut role_env = Environment::new();
        role_env.add_template("role.mdc", ROLE_TEMPLATE)
            .map_err(|e| Error::Render { target: "cursor role".to_owned(), source: e })?;
        let role_tmpl = role_env.get_template("role.mdc")
            .map_err(|e| Error::Render { target: "cursor role".to_owned(), source: e })?;

        for (name, role) in &policy.roles {
            if role.editable.is_empty() {
                continue; // no point generating a role rule with no globs
            }
            // Build a globs pattern from role.editable (comma-separated for Cursor)
            let globs_pattern = role.editable.join(",");
            let target = format!(".cursor/rules/{name}.mdc");

            let content = role_tmpl
                .render(minijinja::context! {
                    project => policy.project,
                    role_name => name,
                    role => role,
                    globs_pattern => globs_pattern,
                })
                .map_err(|e| Error::Render { target: target.clone(), source: e })?;

            outputs.push(RenderedOutput {
                path: Utf8PathBuf::from(&target),
                content,
            });
        }
    }

    Ok(outputs)
}
```

**Note:** Cursor's `globs` field accepts a comma-separated list of patterns. Joining `role.editable` with a comma is correct for multi-pattern globs.

---

## Feature: `examples/website/`

Expand `examples/website/agent-policy.yaml` to a comprehensive real-world example with all features exercised:

```yaml
# examples/website/agent-policy.yaml
project:
  name: website
  summary: Public marketing website and documentation.

commands:
  install: npm install
  dev: npm run dev
  lint: npm run lint
  test: npm test
  build: npm run build

paths:
  editable:
    - src/**
    - public/**
    - docs/**
    - content/**
  protected:
    - .github/workflows/**
    - deployment.json
    - functions/**
    - package-lock.json
  generated:
    - AGENTS.md
    - CLAUDE.md
    - .cursor/rules/**

roles:
  docs_agent:
    editable:
      - docs/**
      - content/**
    forbidden:
      - src/**
      - functions/**
      - .github/**

  frontend_agent:
    editable:
      - src/**
      - public/**
    forbidden:
      - functions/**
      - deployment.json
      - .github/**

constraints:
  require_tests_for_code_changes: false
  forbid_secrets: true
  require_human_review_for_protected_paths: true

outputs:
  - agents-md
  - claude-md
  - cursor-rules
```

Add golden snapshot tests covering this example. This becomes the primary demo example in the README.

---

## Hardening: Error Message Quality

Work through these error scenarios and verify each one produces a useful, specific message:

| Scenario                         | Expected error                                                                     |
| -------------------------------- | ---------------------------------------------------------------------------------- |
| `agent-policy.yaml` not found    | `I/O error reading 'agent-policy.yaml': No such file or directory`                 |
| YAML syntax error                | `YAML parse error: ... at line N column M`                                         |
| Missing required `project.name`  | `Policy validation failed:\n  - "name" is required`                                |
| Unknown top-level key            | shows the unknown key clearly                                                      |
| Invalid glob pattern `[unclosed` | `Invalid glob pattern '[unclosed': ...`                                            |
| Invalid role name `my role`      | `Invalid role name 'my role': use only lowercase letters, digits, and underscores` |
| No outputs enabled               | `No outputs are enabled. Set at least one of ...`                                  |
| `check` finds stale file         | prints unified diff to stderr                                                      |
| `init` finds existing file       | `I/O error ... Already exists ... Use --force to overwrite.`                       |

For each scenario, manually test and improve any message that a real developer would struggle to act on.

---

## Hardening: `--config` Flag Behavior

Verify that `--config` / `-c` works correctly in both `generate` and `check`:

```bash
# Generate from a non-default config location
agent-policy generate --config infra/agent-policy.yaml

# Check in CI with non-default location
agent-policy check -c path/to/agent-policy.yaml
```

This is important for monorepos and non-standard setups.

---

## Documentation: Rustdoc

All public items must have `///` doc comments before publishing. Use `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` to enforce this.

Key items to document:

**`lib.rs` crate-level (`//!` block):**

````rust
//! # agent-policy
//!
//! Schema-first generator for coding-agent repo policies and compatibility files.
//!
//! ## Usage
//!
//! ```no_run
//! use agent_policy::load;
//! use camino::Utf8Path;
//!
//! let policy = agent_policy::load(Utf8Path::new("agent-policy.yaml"))
//!     .expect("failed to load policy");
//!
//! println!("Project: {}", policy.project.name);
//! ```
//!
//! ## Command-line interface
//!
//! See the [README](https://github.com/CameronBrooks11/agent-policy) for CLI documentation.
````

**Public structs and functions** — concise `///` comments on every pub item, focusing on what it is and any non-obvious behavior.

**`doc_tests`** — add at least one doc test for `load()` in `lib.rs` (marked `no_run` since it needs a file).

---

## Documentation: README Completion

By end of Phase 4, the README must be complete for a crates.io landing page reader:

````markdown
# agent-policy

Schema-first generator for coding-agent repo policies and compatibility files.

**Generates `AGENTS.md`, `CLAUDE.md`, and `.cursor/rules/` from a single `agent-policy.yaml`.**

[badges row]

## What it does

Keep one policy file. Generate all agent instruction files from it. Detect drift in CI.

## Install

### From crates.io

```bash
cargo install agent-policy
```
````

### Binary releases

Download pre-built binaries from [GitHub Releases](link).

## Quick start

```bash
# Create a starter policy
agent-policy init

# Edit agent-policy.yaml, then generate
agent-policy generate

# Add to CI to prevent drift
agent-policy check
```

## Commands

| Command                 | Description                                  |
| ----------------------- | -------------------------------------------- |
| `agent-policy init`     | Write a starter `agent-policy.yaml`          |
| `agent-policy generate` | Generate all enabled output files            |
| `agent-policy check`    | Verify committed files match policy (CI use) |

## agent-policy.yaml

[minimal and full examples, link to examples/]

## Generated outputs

| Target ID      | File                  | Default |
| -------------- | --------------------- | ------- |
| `agents-md`    | `AGENTS.md`           | Yes     |
| `claude-md`    | `CLAUDE.md`           | No      |
| `cursor-rules` | `.cursor/rules/*.mdc` | No      |

## CI integration

```yaml
- name: Check agent policy
  run: agent-policy check
```

## Non-goals

...

## License

Apache-2.0

````

---

## `CHANGELOG.md` — v0.1.0 Entry

Follow [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format:

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] — 2026-MM-DD

### Added

- `agent-policy init` — write a starter `agent-policy.yaml`
- `agent-policy generate` — generate `AGENTS.md`, `CLAUDE.md`, and `.cursor/rules/` from `agent-policy.yaml`
- `agent-policy check` — verify committed generated files match the current policy (exits non-zero on drift)
- JSON Schema (`agent-policy.schema.json`) validating all policy configuration
- Path-scoped `.cursor/rules/` generation (one file per declared agent role)
- Human-readable unified diff output from `check` on stale files
- Golden snapshot tests for all render targets
- CLI integration tests
- Self-dogfooding: `agent-policy` uses itself to manage its own `AGENTS.md` and `CLAUDE.md`

[0.1.0]: https://github.com/CameronBrooks11/agent-policy/releases/tag/v0.1.0
````

---

## Release Workflow

### Tool: `cargo-dist`

`cargo-dist` is the standard Rust tool for multi-platform binary distribution. It handles GitHub Releases, artifact naming, SHA256 checksums, and install scripts.

```bash
# Install cargo-dist
cargo install cargo-dist

# Initialize (run once in the repo root)
cargo dist init
```

This adds to `Cargo.toml`:

```toml
[workspace.metadata.dist]
cargo-dist-version = "0.x.x"
ci = ["github"]
installers = ["shell", "powershell"]
targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc",
]
```

And creates `.github/workflows/release.yml` that triggers on version tags.

### Release Process

1. Update `CHANGELOG.md` — move items from `[Unreleased]` to `[0.1.0]` with the date
2. **Bump version in `Cargo.toml` from `"0.0.0"` to `"0.1.0"`** (the crate starts at `0.0.0` per Phase 0 to avoid any ambiguity about what is "released"; this bump marks the first public release)
3. Run the pre-publish checklist (see below)
4. Commit: `git commit -m "chore: release v0.1.0"`
5. Tag: `git tag v0.1.0`
6. Push: `git push && git push --tags`
7. `cargo-dist` CI creates the GitHub Release automatically
8. Publish to crates.io: `cargo publish`

### crates.io Publication

Before publishing for the first time:

```bash
# Authenticate
cargo login

# Dry run — must pass with no errors
cargo publish --dry-run

# Actual publish
cargo publish
```

After publishing, the crate is available via:

```bash
cargo install agent-policy
```

---

## Pre-Publish Checklist

Work through this list in order before running `cargo publish`.

### Code quality

- [ ] `cargo test` passes with zero failures
- [ ] `cargo clippy --all-targets -- -D warnings` passes with zero warnings
- [ ] `cargo fmt --check` passes (code is formatted)
- [ ] `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` passes (all public items documented)

### Correctness

- [ ] `agent-policy init` creates a valid config that `generate` accepts
- [ ] `agent-policy generate --config examples/minimal/agent-policy.yaml` works from the repo root
- [ ] `agent-policy generate --config examples/website/agent-policy.yaml` works and produces all three target types
- [ ] `agent-policy check` passes on this repo (self-dogfooding CI step is green)

### Metadata

- [ ] `Cargo.toml` version is `"0.1.0"` (bumped from `"0.0.0"` during this release step)
- [ ] `Cargo.toml` has: `description`, `keywords` (5 max), `categories`, `repository`, `homepage`, `documentation`, `readme`, `license`
- [ ] `keywords` are all legal (lowercase, max 20 chars, max 5 entries)
- [ ] `categories` are valid crates.io category slugs
- [ ] `readme = "README.md"` and README.md exists

### Documentation

- [ ] README.md has: install instructions, quick start, all three commands documented, non-goals
- [ ] `CHANGELOG.md` has `[0.1.0]` section with release date filled in
- [ ] All public types and functions have `///` doc comments
- [ ] `lib.rs` has `//!` crate-level documentation with a working code example

### Release artifacts

- [ ] `cargo publish --dry-run` succeeds with no errors
- [ ] `cargo-dist` is configured and `.github/workflows/release.yml` exists
- [ ] Version tag `v0.1.0` matches the `version` in `Cargo.toml`

### Schema

- [ ] `agent-policy.schema.json` is the final v0.1 schema and matches what the Rust types accept
- [ ] The schema file is included in the published crate (it is a source file, not in `exclude`)
- [ ] A note in the README or docs states that the schema is versioned with the binary and is considered unstable until v1.0.0

---

## Exit Condition

Phase 4 is complete — and `v0.1.0` is ready to tag — when all **Publication Readiness** items below are checked. External adoption is a separate post-publish milestone tracked independently.

### Publication Readiness (required to tag v0.1.0)

- [ ] Path-scoped Cursor rules generate correctly: `default.mdc` plus one `.mdc` per role with non-empty `editable`
- [ ] `examples/website/agent-policy.yaml` is fully specified and its golden tests pass
- [ ] All error messages are clear and actionable (manually tested through the error scenario table)
- [ ] `--config` flag works correctly in `generate` and `check`
- [ ] All pre-publish checklist items are checked
- [ ] `cargo publish --dry-run` passes with no errors
- [ ] `v0.1.0` is tagged and GitHub Release is created by `cargo-dist` with binaries for all four targets
- [ ] `cargo publish` succeeds and the crate is live on crates.io
- [ ] `cargo install agent-policy` installs the binary correctly (verified on at least one platform)

### Post-Publish Adoption Milestone (tracked separately, not a v0.1.0 blocker)

These items are desirable but outside direct control. Track them as a follow-up milestone after publication:

- [ ] At least one external repo (not this one) has adopted `agent-policy` with a working `agent-policy check` CI step
- [ ] Feedback from the external adopter has been reviewed and any actionable items filed as issues
