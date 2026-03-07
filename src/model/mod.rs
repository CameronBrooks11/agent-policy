// Data model — implemented in Phase 1

pub mod normalized;
pub mod policy;
pub mod targets;

use crate::error::{Error, Result};
use indexmap::IndexMap;
use normalized::{Commands, Constraints, Paths, Policy, Project, Role};
use policy::RawPolicy;
use targets::OutputTargets;

/// Valid output target IDs.
const VALID_TARGETS: &[&str] = &[
    "agents-md",
    "claude-md",
    "cursor-rules",
    "gemini-md",
    "copilot-instructions",
];

/// Normalize a validated [`RawPolicy`] into the stable [`Policy`] model.
///
/// Applies all defaults and validates semantic constraints:
/// valid glob patterns, valid role names, known output target IDs.
///
/// # Errors
///
/// Returns [`Error::InvalidRoleName`] for role names with disallowed characters,
/// [`Error::Glob`] for malformed glob patterns, [`Error::UnknownTarget`] for
/// unrecognized output target IDs, or [`Error::NoOutputs`] if the resolved
/// outputs list is empty.
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
            roles.insert(
                name.clone(),
                Role {
                    name,
                    editable,
                    forbidden,
                },
            );
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
    let enabled_targets: Vec<String> = raw.outputs.unwrap_or_else(|| vec!["agents-md".to_owned()]);

    // Validate all target IDs. Unknown IDs surface a clear error rather than
    // a cryptic JSON Schema message.
    for id in &enabled_targets {
        if !VALID_TARGETS.contains(&id.as_str()) {
            return Err(Error::UnknownTarget { id: id.clone() });
        }
    }

    let outputs = OutputTargets {
        agents_md: enabled_targets.contains(&"agents-md".to_owned()),
        claude_md: enabled_targets.contains(&"claude-md".to_owned()),
        cursor_rules: enabled_targets.contains(&"cursor-rules".to_owned()),
        gemini_md: enabled_targets.contains(&"gemini-md".to_owned()),
        copilot_instructions: enabled_targets.contains(&"copilot-instructions".to_owned()),
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
            dev: raw_commands.dev,
            lint: raw_commands.lint,
            test: raw_commands.test,
            build: raw_commands.build,
        },
        paths: Paths {
            editable,
            protected,
            generated,
        },
        roles,
        constraints: Constraints {
            require_tests_for_code_changes: raw_constraints
                .require_tests_for_code_changes
                .unwrap_or(false),
            forbid_secrets: raw_constraints.forbid_secrets.unwrap_or(false),
            require_human_review_for_protected_paths: raw_constraints
                .require_human_review_for_protected_paths
                .unwrap_or(false),
        },
        outputs,
    })
}

fn validate_role_name(name: &str) -> Result<()> {
    let valid = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
    if valid {
        Ok(())
    } else {
        Err(Error::InvalidRoleName {
            name: name.to_owned(),
        })
    }
}

fn validate_globs(patterns: &[String]) -> Result<()> {
    for pattern in patterns {
        globset::GlobBuilder::new(pattern)
            .build()
            .map_err(|e| Error::Glob {
                pattern: pattern.clone(),
                source: e,
            })?;
    }
    Ok(())
}
