//! Renderer for `.cursor/rules/` — global default plus one file per agent role.

use camino::Utf8PathBuf;
use minijinja::Environment;

use crate::{
    error::{Error, Result},
    model::normalized::Policy,
    render::RenderedOutput,
};

const DEFAULT_TARGET: &str = ".cursor/rules/default.mdc";
const DEFAULT_TEMPLATE: &str = include_str!("../../templates/cursor_rule.mdc.j2");
const ROLE_TEMPLATE: &str = include_str!("../../templates/cursor_role.mdc.j2");

/// Render all `.cursor/rules/` outputs for the given policy.
///
/// Always produces `default.mdc` (global, `alwaysApply: true`). Additionally
/// produces one `.mdc` per role that has at least one editable path, with
/// `globs` set to those patterns so Cursor activates the rule automatically.
///
/// # Errors
///
/// Returns [`Error::Render`] if any template fails to compile or render.
pub fn render(policy: &Policy) -> Result<Vec<RenderedOutput>> {
    let mut outputs = Vec::new();

    // Global default rule
    let mut env = Environment::new();
    env.add_template("default.mdc", DEFAULT_TEMPLATE)
        .map_err(|e| Error::Render {
            target: DEFAULT_TARGET.to_owned(),
            source: e,
        })?;
    let tmpl = env.get_template("default.mdc").map_err(|e| Error::Render {
        target: DEFAULT_TARGET.to_owned(),
        source: e,
    })?;
    let commands_defined = !policy.commands.is_empty();
    let content = tmpl
        .render(minijinja::context! {
            project => &policy.project,
            commands => &policy.commands,
            commands_defined => commands_defined,
            paths => &policy.paths,
            roles => &policy.roles,
            constraints => &policy.constraints,
        })
        .map_err(|e| Error::Render {
            target: DEFAULT_TARGET.to_owned(),
            source: e,
        })?;
    outputs.push(RenderedOutput {
        path: Utf8PathBuf::from(DEFAULT_TARGET),
        content,
    });

    // Per-role rules
    if !policy.roles.is_empty() {
        let mut role_env = Environment::new();
        role_env
            .add_template("role.mdc", ROLE_TEMPLATE)
            .map_err(|e| Error::Render {
                target: "cursor role".to_owned(),
                source: e,
            })?;
        let role_tmpl = role_env
            .get_template("role.mdc")
            .map_err(|e| Error::Render {
                target: "cursor role".to_owned(),
                source: e,
            })?;

        for (name, role) in &policy.roles {
            if role.editable.is_empty() {
                continue;
            }
            // Cursor globs field: comma-separated list of patterns
            let globs_pattern = role.editable.join(",");
            let target = format!(".cursor/rules/{name}.mdc");
            let role_content = role_tmpl
                .render(minijinja::context! {
                    project => &policy.project,
                    role_name => name,
                    role => role,
                    globs_pattern => &globs_pattern,
                })
                .map_err(|e| Error::Render {
                    target: target.clone(),
                    source: e,
                })?;
            outputs.push(RenderedOutput {
                path: Utf8PathBuf::from(&target),
                content: role_content,
            });
        }
    }

    Ok(outputs)
}
