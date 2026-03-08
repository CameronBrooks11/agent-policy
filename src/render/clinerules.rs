//! Renderer for `.clinerules/` — global default plus one file per agent role.

use camino::Utf8PathBuf;
use minijinja::Environment;

use crate::{
    error::{Error, Result},
    model::normalized::Policy,
    render::RenderedOutput,
};

const DEFAULT_TARGET: &str = ".clinerules/default.md";
const DEFAULT_TEMPLATE: &str = include_str!("../../templates/clinerules_default.md.j2");
const ROLE_TEMPLATE: &str = include_str!("../../templates/clinerules_role.md.j2");

/// Render all `.clinerules/` outputs for the given policy.
///
/// Always produces `default.md` (global). Additionally
/// produces one `.md` per role that has at least one editable path, with
/// `paths` set to those patterns so Cline activates the rule automatically.
///
/// # Errors
///
/// Returns [`Error::Render`] if any template fails to compile or render.
pub fn render(policy: &Policy) -> Result<Vec<RenderedOutput>> {
    let mut outputs = Vec::new();

    // Global default rule
    let mut env = Environment::new();
    env.add_template("default.md", DEFAULT_TEMPLATE)
        .map_err(|e| Error::Render {
            target: DEFAULT_TARGET.to_owned(),
            source: e,
        })?;
    let tmpl = env.get_template("default.md").map_err(|e| Error::Render {
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
            .add_template("role.md", ROLE_TEMPLATE)
            .map_err(|e| Error::Render {
                target: "clinerules role".to_owned(),
                source: e,
            })?;
        let role_tmpl = role_env
            .get_template("role.md")
            .map_err(|e| Error::Render {
                target: "clinerules role".to_owned(),
                source: e,
            })?;

        for (name, role) in &policy.roles {
            if role.editable.is_empty() {
                continue;
            }
            let target = format!(".clinerules/{name}.md");
            let role_content = role_tmpl
                .render(minijinja::context! {
                    project => &policy.project,
                    role_name => name,
                    role => role,
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
