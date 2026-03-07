# Phase 1 — Core Model

**Goal:** Load a valid `agent-policy.yaml`, validate it against the JSON Schema, and produce a stable internal normalized policy model ready for rendering. No output files are written yet. Invalid input must fail cleanly with actionable error messages.

**Depends on:** Phase 0 (project skeleton compiles)
**Unlocks:** Phase 2 (renderers consume the normalized model)

---

## Overview

This phase establishes the spine of the entire tool. Everything in Phase 2 and beyond consumes the normalized `Policy` struct produced here. Getting the model right — and keeping the load pipeline layered cleanly — is the most important design work in the project.

The pipeline is:

```
agent-policy.yaml  (file on disk)
       ↓ read bytes
raw YAML string
       ↓ serde_yaml::from_str
RawPolicy  (raw serde types)
       ↓ serde_json::to_value  →  jsonschema validate
validated raw document
       ↓ normalize()
Policy  (stable normalized model)
```

The raw types mirror the YAML structure exactly. The normalized model is what renderers and commands consume. The separation means YAML schema can evolve without breaking renderer logic.

---

## Dependencies to Add

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1"
jsonschema = "0.18"
anyhow = "1"
thiserror = "1"
camino = "1"
globset = "0.4"
indexmap = { version = "2", features = ["serde"] }
minijinja = { version = "2", features = ["loader"] }

[dev-dependencies]
insta = { version = "1", features = ["yaml"] }
```

Notes:

- `indexmap` preserves insertion order for role maps — important for deterministic output rendering.
- `insta` is used for snapshot tests in Phase 1 (normalized model snapshots) and Golden tests in Phase 2.
- `minijinja` is added in Phase 1 (not Phase 2) because `Error::Render { source: minijinja::Error }` in `src/error.rs` requires the type to be in scope. Completing `error.rs` in Phase 1 — before renderers exist — keeps it as the stable foundation the rest of the codebase builds on. Deferring it would mean `error.rs` is incomplete until Phase 2 patching.
- Do not add `clap` or `similar` yet — those are Phase 2 and 3 concerns.

---

## `agent-policy.schema.json`

This is the machine contract. Hand-authored, not generated. Lives at the repo root and is bundled into the binary at compile time via `include_str!`.

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://github.com/CameronBrooks11/agent-policy/agent-policy.schema.json",
  "title": "AgentPolicy",
  "description": "Schema for agent-policy.yaml configuration files.",
  "type": "object",
  "required": ["project"],
  "additionalProperties": false,
  "properties": {
    "project": {
      "description": "Project identity and purpose.",
      "type": "object",
      "required": ["name"],
      "additionalProperties": false,
      "properties": {
        "name": {
          "description": "Short identifier for this repository.",
          "type": "string",
          "minLength": 1
        },
        "summary": {
          "description": "One-sentence description of the repository purpose.",
          "type": "string"
        }
      }
    },
    "commands": {
      "description": "Shell commands agents should use in this repository.",
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "install": {
          "type": "string",
          "description": "Dependency installation command."
        },
        "dev": {
          "type": "string",
          "description": "Local development server command."
        },
        "lint": { "type": "string", "description": "Lint command." },
        "test": { "type": "string", "description": "Test suite command." },
        "build": { "type": "string", "description": "Build command." }
      }
    },
    "paths": {
      "description": "Repository path classifications using glob patterns.",
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "editable": {
          "description": "Paths agents may freely edit.",
          "type": "array",
          "items": { "type": "string", "minLength": 1 },
          "uniqueItems": true
        },
        "protected": {
          "description": "Paths that require human review before agent changes are accepted.",
          "type": "array",
          "items": { "type": "string", "minLength": 1 },
          "uniqueItems": true
        },
        "generated": {
          "description": "Paths that are generated artifacts agents should not edit directly.",
          "type": "array",
          "items": { "type": "string", "minLength": 1 },
          "uniqueItems": true
        }
      }
    },
    "roles": {
      "description": "Named agent roles with scoped path permissions.",
      "type": "object",
      "additionalProperties": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "editable": {
            "description": "Paths this role may edit (overrides global editable for this role).",
            "type": "array",
            "items": { "type": "string", "minLength": 1 },
            "uniqueItems": true
          },
          "forbidden": {
            "description": "Paths this role must never edit, even if globally editable.",
            "type": "array",
            "items": { "type": "string", "minLength": 1 },
            "uniqueItems": true
          }
        }
      }
    },
    "constraints": {
      "description": "Behavioral guardrails agents must observe.",
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "require_tests_for_code_changes": {
          "description": "Agent must include tests when making code changes.",
          "type": "boolean"
        },
        "forbid_secrets": {
          "description": "Agent must never commit secrets, credentials, or tokens.",
          "type": "boolean"
        },
        "require_human_review_for_protected_paths": {
          "description": "Changes to protected paths require human approval before merge.",
          "type": "boolean"
        }
      }
    },
    "outputs": {
      "description": "List of output target IDs to generate. Defaults to [\"agents-md\"] when omitted. Each ID maps to a specific compatibility file. Unknown IDs are rejected at normalization time.",
      "type": "array",
      "items": {
        "type": "string",
        "enum": ["agents-md", "claude-md", "cursor-rules"]
      },
      "uniqueItems": true
    }
  }
}
```

The schema is bundled in the binary:

```rust
// src/load/schema.rs
const SCHEMA_JSON: &str = include_str!("../../agent-policy.schema.json");
```

This means consuming repos need only the binary — no external schema file is required.

---

## Error Types (`src/error.rs`)

Define all error variants for the entire tool now so they are stable before renderers are built.

```rust
use std::path::PathBuf;

/// All errors produced by agent-policy.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An I/O operation failed.
    #[error("I/O error reading '{path}': {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// The YAML file could not be parsed.
    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// The policy failed JSON Schema validation.
    #[error("Policy validation failed:\n{0}")]
    Schema(String),

    /// A glob pattern in the policy is invalid.
    #[error("Invalid glob pattern '{pattern}': {source}")]
    Glob {
        pattern: String,
        #[source]
        source: globset::Error,
    },

    /// A role name contains invalid characters.
    #[error("Invalid role name '{name}': use only lowercase letters, digits, and underscores")]
    InvalidRoleName { name: String },

    /// Template rendering failed.
    #[error("Render error for target '{target}': {source}")]
    Render {
        target: String,
        #[source]
        source: minijinja::Error,
    },

    /// Generated file content differs from the committed file.
    #[error("Stale generated file: {path}")]
    CheckFailed { path: PathBuf },

    /// The outputs list is present but empty.
    #[error("No outputs are enabled. Add at least one target ID to `outputs` (e.g. `outputs: [agents-md]`).")]
    NoOutputs,

    /// An unrecognized target ID was specified in outputs.
    #[error("Unknown output target '{id}'. Supported targets for this version: agents-md, claude-md, cursor-rules.")]
    UnknownTarget { id: String },
}

/// Convenience alias.
pub type Result<T> = std::result::Result<T, Error>;
```

`minijinja` is in scope because it is added as a Phase 1 dependency (see Dependencies section above). Having the full `Error` enum in place before Phase 2 renderers are built keeps this module stable and avoids patching a foundational type mid-phase.

---

## Raw Policy Types (`src/model/policy.rs`)

These types are deserialized directly from YAML. They mirror the schema exactly. Fields are `Option<T>` wherever the schema allows omission.

```rust
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Raw deserialized policy as it appears in agent-policy.yaml.
/// Do not use this type in renderers — use the normalized [`Policy`] instead.
///
/// `Serialize` is required so `serde_json::to_value(&raw)` compiles in the load pipeline.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawPolicy {
    pub project: RawProject,
    pub commands: Option<RawCommands>,
    pub paths: Option<RawPaths>,
    pub roles: Option<IndexMap<String, RawRole>>,
    pub constraints: Option<RawConstraints>,
    /// List of output target IDs. Validated and mapped to [`OutputTargets`] during normalization.
    /// Valid values: `"agents-md"`, `"claude-md"`, `"cursor-rules"`. Defaults to `["agents-md"]` when omitted.
    pub outputs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawProject {
    pub name: String,
    pub summary: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawCommands {
    pub install: Option<String>,
    pub dev: Option<String>,
    pub lint: Option<String>,
    pub test: Option<String>,
    pub build: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawPaths {
    pub editable: Option<Vec<String>>,
    pub protected: Option<Vec<String>>,
    pub generated: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawRole {
    pub editable: Option<Vec<String>>,
    pub forbidden: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawConstraints {
    pub require_tests_for_code_changes: Option<bool>,
    pub forbid_secrets: Option<bool>,
    pub require_human_review_for_protected_paths: Option<bool>,
}
```

`#[serde(deny_unknown_fields)]` on every struct ensures unrecognized YAML keys produce clear errors rather than silently being ignored. This is belt-and-suspenders alongside JSON Schema validation. The `outputs` field is a plain `Vec<String>` rather than a named struct; unknown target IDs in that list are caught in `normalize()` with an explicit `UnknownTarget` error.

---

## Normalized Model (`src/model/normalized.rs`)

The stable internal representation consumed by all renderers and commands. All `Option`s from the raw model are resolved to concrete values with defaults applied.

```rust
use indexmap::IndexMap;

/// The fully normalized, validated policy model.
/// This is the type renderers and commands work with.
#[derive(Debug, Clone)]
pub struct Policy {
    pub project: Project,
    pub commands: Commands,
    pub paths: Paths,
    /// Roles in declaration order (IndexMap preserves insertion order).
    pub roles: IndexMap<String, Role>,
    pub constraints: Constraints,
    pub outputs: OutputTargets,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Commands {
    pub install: Option<String>,
    pub dev: Option<String>,
    pub lint: Option<String>,
    pub test: Option<String>,
    pub build: Option<String>,
}

impl Commands {
    /// Returns true if at least one command is defined.
    pub fn is_empty(&self) -> bool {
        self.install.is_none()
            && self.dev.is_none()
            && self.lint.is_none()
            && self.test.is_none()
            && self.build.is_none()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Paths {
    pub editable: Vec<String>,
    pub protected: Vec<String>,
    pub generated: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Role {
    /// The role identifier as declared in the YAML.
    pub name: String,
    pub editable: Vec<String>,
    pub forbidden: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Constraints {
    pub require_tests_for_code_changes: bool,
    pub forbid_secrets: bool,
    pub require_human_review_for_protected_paths: bool,
}
```

---

## Output Targets (`src/model/targets.rs`)

```rust
/// Which output files the policy is configured to generate.
#[derive(Debug, Clone)]
pub struct OutputTargets {
    /// Generate AGENTS.md (default: true).
    pub agents_md: bool,
    /// Generate CLAUDE.md (default: false).
    pub claude_md: bool,
    /// Generate .cursor/rules/default.mdc (default: false).
    pub cursor_rules: bool,
}

impl Default for OutputTargets {
    fn default() -> Self {
        Self {
            agents_md: true,
            claude_md: false,
            cursor_rules: false,
        }
    }
}

impl OutputTargets {
    /// Returns true if no outputs are enabled.
    pub fn is_empty(&self) -> bool {
        !self.agents_md && !self.claude_md && !self.cursor_rules
    }
}
```

---

## Load Pipeline (`src/load/`)

### `src/load/yaml.rs` — Parse raw YAML

```rust
use crate::{error::Result, model::policy::RawPolicy};

/// Parse raw YAML bytes into a RawPolicy.
pub fn parse(input: &str) -> Result<RawPolicy> {
    serde_yaml::from_str(input).map_err(crate::error::Error::Yaml)
}
```

### `src/load/schema.rs` — JSON Schema validation

The schema is compiled once (it is expensive) and reused. Expose a `validate` function that accepts the raw YAML value.

```rust
use crate::error::{Error, Result};
use std::sync::OnceLock;

const SCHEMA_JSON: &str = include_str!("../../agent-policy.schema.json");

/// Return a reference to the compiled JSON Schema validator.
///
/// The validator is compiled exactly once (on first call) and cached for the
/// lifetime of the process. Compiling a JSON Schema is expensive; calling
/// `validator_for` on every `validate()` invocation would be a measurable
/// overhead in test suites and CI pipelines that load many configs.
#[allow(clippy::expect_used)] // both panics are on invariants about bundled binary content
fn compiled_validator() -> &'static jsonschema::Validator {
    static VALIDATOR: OnceLock<jsonschema::Validator> = OnceLock::new();
    VALIDATOR.get_or_init(|| {
        let schema: serde_json::Value =
            serde_json::from_str(SCHEMA_JSON).expect("bundled schema is always valid JSON");
        jsonschema::validator_for(&schema).expect("bundled schema always compiles")
    })
}

/// Validate the parsed YAML document against the bundled JSON Schema.
///
/// The input must be a `serde_json::Value` representation of the raw YAML.
/// Convert via `serde_json::to_value(&raw_policy)`.
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
```

### `src/load/mod.rs` — Public load function

```rust
use crate::{error::Result, model::policy::RawPolicy};

pub mod schema;
pub mod yaml;

/// Load and validate an agent-policy.yaml from a string.
///
/// Returns the raw policy struct if validation passes.
/// The caller is responsible for normalization.
#[allow(clippy::expect_used)] // RawPolicy derives Serialize; to_value is infallible for these types
pub fn load_str(input: &str) -> Result<RawPolicy> {
    let raw = yaml::parse(input)?;
    let doc = serde_json::to_value(&raw)
        .expect("raw policy is always serializable to JSON");
    schema::validate(&doc)?;
    Ok(raw)
}

/// Load and validate an agent-policy.yaml from a file path.
pub fn load_file(path: &camino::Utf8Path) -> Result<RawPolicy> {
    let content = std::fs::read_to_string(path).map_err(|e| crate::error::Error::Io {
        path: path.as_std_path().to_owned(),
        source: e,
    })?;
    load_str(&content)
}
```

---

## Normalization Pass (`src/model/mod.rs`)

```rust
pub mod normalized;
pub mod policy;
pub mod targets;

use crate::error::{Error, Result};
use normalized::*;
use policy::RawPolicy;
use targets::OutputTargets;
use indexmap::IndexMap;

/// Normalize a validated RawPolicy into the stable Policy model.
///
/// This applies all defaults and validates semantic constraints
/// (valid glob patterns, valid role names).
pub fn normalize(raw: RawPolicy) -> Result<Policy> {
    // Validate and normalize roles
    let mut roles: IndexMap<String, Role> = IndexMap::new();
    if let Some(raw_roles) = raw.roles {
        for (name, raw_role) in raw_roles {
            validate_role_name(&name)?;
            let editable = raw_role.editable.unwrap_or_default();
            let forbidden = raw_role.forbidden.unwrap_or_default();
            validate_globs(&editable)?;
            validate_globs(&forbidden)?;
            roles.insert(name.clone(), Role { name, editable, forbidden });
        }
    }

    // Validate global path globs
    let raw_paths = raw.paths.unwrap_or_default();
    let editable = raw_paths.editable.unwrap_or_default();
    let protected = raw_paths.protected.unwrap_or_default();
    let generated = raw_paths.generated.unwrap_or_default();
    validate_globs(&editable)?;
    validate_globs(&protected)?;
    validate_globs(&generated)?;

    let raw_commands = raw.commands.unwrap_or_default();
    let raw_constraints = raw.constraints.unwrap_or_default();

    // When `outputs` is omitted entirely, default to generating agents-md only.
    let enabled_targets: Vec<String> = raw.outputs
        .unwrap_or_else(|| vec!["agents-md".to_owned()]);

    // Validate all target IDs. Unknown IDs surface a clear error rather than a
    // cryptic JSON Schema message, because the enum constraint in the schema
    // only fires during schema validation; any IDs that slip through (e.g. from
    // programmatic construction) are caught here.
    const VALID_TARGETS: &[&str] = &["agents-md", "claude-md", "cursor-rules"];
    for id in &enabled_targets {
        if !VALID_TARGETS.contains(&id.as_str()) {
            return Err(Error::UnknownTarget { id: id.clone() });
        }
    }

    let outputs = OutputTargets {
        agents_md:    enabled_targets.contains(&"agents-md".to_owned()),
        claude_md:    enabled_targets.contains(&"claude-md".to_owned()),
        cursor_rules: enabled_targets.contains(&"cursor-rules".to_owned()),
    };

    if outputs.is_empty() {
        return Err(Error::NoOutputs);
    }

    Ok(Policy {
        project: Project {
            name: raw.project.name,
            summary: raw.project.summary,
        },
        commands: Commands {
            install: raw_commands.install,
            dev:     raw_commands.dev,
            lint:    raw_commands.lint,
            test:    raw_commands.test,
            build:   raw_commands.build,
        },
        paths: Paths { editable, protected, generated },
        roles,
        constraints: Constraints {
            require_tests_for_code_changes:
                raw_constraints.require_tests_for_code_changes.unwrap_or(false),
            forbid_secrets:
                raw_constraints.forbid_secrets.unwrap_or(false),
            require_human_review_for_protected_paths:
                raw_constraints.require_human_review_for_protected_paths.unwrap_or(false),
        },
        outputs,
    })
}

fn validate_role_name(name: &str) -> Result<()> {
    let valid = name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
    if valid && !name.is_empty() {
        Ok(())
    } else {
        Err(Error::InvalidRoleName { name: name.to_owned() })
    }
}

fn validate_globs(patterns: &[String]) -> Result<()> {
    for pattern in patterns {
        globset::GlobBuilder::new(pattern)
            .build()
            .map_err(|e| Error::Glob { pattern: pattern.clone(), source: e })?;
    }
    Ok(())
}
```

---

## Path Policy Semantics

The policy model includes two distinct path-classification systems: **global paths** (`paths.*`) and **role-scoped paths** (`roles.<name>.editable/forbidden`). The following precedence rules govern how they interact during rendering and future enforcement:

### Precedence rules (highest to lowest)

1. **Role `forbidden` always wins.** If a path matches a role's `forbidden` list, that role's agent may never edit it — regardless of what `paths.editable` or the role's own `editable` list says.

2. **Global `paths.protected` overrides role `editable`.** If a path matches `paths.protected`, it requires human review regardless of which role is acting. A role's `editable` list cannot grant permission to bypass protected paths.

3. **Role `editable` is additive restriction, not elevation.** A role's `editable` list narrows what that role can edit within the overall editable space. It cannot grant access to paths not already considered editable globally.

4. **Global `paths.editable` defines the open set.** Paths matching `paths.editable` are available for agent modification (subject to the constraints above). Paths not matching any `editable` rule are implicitly restricted.

5. **`paths.generated` is informational.** Files listed under `generated` are documented as machine-produced and should not be manually edited. This is advisory — it affects what agents are told to avoid editing, not a security enforcement boundary.

### Conflict resolution

| Conflict scenario                                      | Resolution                                          |
| ------------------------------------------------------ | --------------------------------------------------- |
| Role `editable` overlaps with global `paths.protected` | `protected` wins — path requires human review       |
| Role `editable` overlaps with role `forbidden`         | `forbidden` wins — path is off-limits for that role |
| Two roles have overlapping `editable` patterns         | No conflict — roles are independent                 |
| `paths.editable` and `paths.protected` overlap         | `protected` wins                                    |

### Implementation note

The current v0.1 model stores these lists as raw glob strings. Enforcement of these rules during rendering is the responsibility of the renderer templates and the `check` command. A future enforcement engine (Phase 6) will apply them as hard access-control rules.

---

## Role `owner` Field — CODEOWNERS (Deferred to Phase 5)

The `roles` section in the v0.1 schema intentionally omits an `owner` identity field. Generating a valid `CODEOWNERS` file requires GitHub usernames or team handles (e.g., `@org/team-name`) mapped to path patterns. This is a Phase 5 schema extension.

**Why deferred:** Collecting and validating GitHub identity strings adds auth-model complexity that is out of scope for v0.1. The `check` and `generate` commands do not need owner identity to work correctly. CODEOWNERS generation is listed as a Phase 5 deliverable.

**What to add in Phase 5:**

```yaml
# agent-policy.yaml — Phase 5 schema extension (not in v0.1)
roles:
  docs_agent:
    owner: "@org/docs-team" # GitHub username or team handle
    editable:
      - docs/**
```

The schema will add an optional `owner: string` field to each role's `additionalProperties` object. The CODEOWNERS generator will use it to emit lines like:

```
docs/**   @org/docs-team
```

Until Phase 5, the `CODEOWNERS` file must be maintained manually or omitted.

---

## Public API (`src/lib.rs`)

Expose a clean top-level API:

```rust
pub mod commands;
pub mod error;
pub mod load;
pub mod model;
pub mod render;
pub(crate) mod util;

pub use error::{Error, Result};
pub use model::{normalize, normalized::Policy};

/// Load, validate, and normalize an agent-policy.yaml from a file path.
///
/// This is the main entry point for the entire load pipeline.
pub fn load(path: &camino::Utf8Path) -> Result<Policy> {
    let raw = load::load_file(path)?;
    model::normalize(raw)
}
```

---

## Test Strategy (`tests/schema.rs`)

```rust
// tests/schema.rs
use agent_policy::{load, load::load_str};
use camino::Utf8Path;

#[test]
fn minimal_valid_config_loads() {
    let yaml = r#"
project:
  name: test-project
"#;
    let result = load_str(yaml);
    assert!(result.is_ok(), "minimal valid config should load: {:?}", result);
}

#[test]
fn missing_project_name_fails() {
    let yaml = r#"
project:
  summary: no name here
"#;
    let result = load_str(yaml);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("validation failed") || err.contains("name"),
        "error should mention validation: {err}");
}

#[test]
fn unknown_top_level_key_fails() {
    let yaml = r#"
project:
  name: test
unknown_key: true
"#;
    let result = load_str(yaml);
    assert!(result.is_err());
}

#[test]
fn full_config_normalizes_correctly() {
    let yaml = r#"
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
    let policy = agent_policy::model::normalize(raw).expect("should normalize");
    assert_eq!(policy.project.name, "website");
    assert!(policy.outputs.agents_md);
    assert!(policy.outputs.claude_md);
    assert!(!policy.outputs.cursor_rules);
    assert_eq!(policy.roles.len(), 1);
    assert!(policy.constraints.forbid_secrets);
}

#[test]
fn invalid_role_name_fails_normalization() {
    let yaml = r#"
project:
  name: test
roles:
  "my bad role name":
    editable: []
"#;
    let raw = load_str(yaml).expect("yaml parses");
    let result = agent_policy::model::normalize(raw);
    assert!(result.is_err());
}

#[test]
fn no_outputs_enabled_fails() {
    let yaml = r#"
project:
  name: test
outputs: []
"#;
    let raw = load_str(yaml).expect("yaml parses");
    let result = agent_policy::model::normalize(raw);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("No outputs"), "error: {err}");
}

#[test]
fn examples_minimal_loads() {
    let path = Utf8Path::new("examples/minimal/agent-policy.yaml");
    let result = load(path);
    assert!(result.is_ok(), "examples/minimal should load: {:?}", result);
}
```

---

## Exit Condition

Phase 1 is complete when all of the following are true:

- [ ] `agent-policy.schema.json` is fully authored with all six top-level sections
- [ ] `RawPolicy` and all raw sub-types are defined with `#[serde(deny_unknown_fields)]`
- [ ] `Policy` normalized model and all sub-types are defined with doc comments
- [ ] Omitting `outputs` from YAML defaults to `agents_md: true`, others `false` (the implicit `["agents-md"]` default in `normalize()`)
- [ ] `load_str()` and `load_file()` are implemented and publicly exported
- [ ] `normalize()` is implemented and applies all defaults
- [ ] `normalize()` validates glob patterns and role names
- [ ] `Error` enum covers all failure cases with clean `Display` messages
- [ ] All six schema test cases in `tests/schema.rs` pass
- [ ] `examples/minimal/agent-policy.yaml` loads and normalizes without error
- [ ] `cargo test` passes with zero failures
- [ ] `cargo clippy --all-targets -- -D warnings` passes with zero warnings
- [ ] `cargo doc --no-deps` passes with `RUSTDOCFLAGS="-D warnings"` (all public items documented)
