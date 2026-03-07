// Normalized model — stable internal representation consumed by all renderers.

use indexmap::IndexMap;
use serde::Serialize;

/// The fully normalized, validated policy model.
///
/// All `Option`s from the raw model are resolved to concrete values with
/// defaults applied. This is the type renderers and commands work with.
#[derive(Debug, Clone, Serialize)]
pub struct Policy {
    pub project: Project,
    pub commands: Commands,
    pub paths: Paths,
    /// Roles in declaration order (`IndexMap` preserves insertion order).
    pub roles: IndexMap<String, Role>,
    pub constraints: Constraints,
    pub outputs: crate::model::targets::OutputTargets,
}

#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub name: String,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Commands {
    pub install: Option<String>,
    pub dev: Option<String>,
    pub lint: Option<String>,
    pub test: Option<String>,
    pub build: Option<String>,
}

impl Commands {
    /// Returns `true` if no commands are defined.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.install.is_none()
            && self.dev.is_none()
            && self.lint.is_none()
            && self.test.is_none()
            && self.build.is_none()
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Paths {
    pub editable: Vec<String>,
    pub protected: Vec<String>,
    pub generated: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Role {
    /// The role identifier as declared in the YAML.
    pub name: String,
    pub editable: Vec<String>,
    pub forbidden: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Constraints {
    pub require_tests_for_code_changes: bool,
    pub forbid_secrets: bool,
    pub require_human_review_for_protected_paths: bool,
}
