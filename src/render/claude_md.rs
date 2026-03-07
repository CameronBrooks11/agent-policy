//! Renderer for `CLAUDE.md`.

use camino::Utf8PathBuf;
use minijinja::Environment;

use crate::{
    error::{Error, Result},
    model::normalized::Policy,
    render::RenderedOutput,
};

const TEMPLATE: &str = include_str!("../../templates/CLAUDE.md.j2");

/// Render the `CLAUDE.md` output for the given policy.
///
/// # Errors
///
/// Returns [`Error::Render`] if the template fails to compile or render.
pub fn render(policy: &Policy) -> Result<RenderedOutput> {
    let mut env = Environment::new();
    env.add_template("CLAUDE.md", TEMPLATE)
        .map_err(|e| Error::Render {
            target: "CLAUDE.md".to_owned(),
            source: e,
        })?;

    let tmpl = env.get_template("CLAUDE.md").map_err(|e| Error::Render {
        target: "CLAUDE.md".to_owned(),
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
            target: "CLAUDE.md".to_owned(),
            source: e,
        })?;

    Ok(RenderedOutput {
        path: Utf8PathBuf::from("CLAUDE.md"),
        content,
    })
}
