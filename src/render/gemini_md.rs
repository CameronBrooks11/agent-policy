//! Renderer for `GEMINI.md`.

use camino::Utf8PathBuf;
use minijinja::Environment;

use crate::{
    error::{Error, Result},
    model::normalized::Policy,
    render::RenderedOutput,
};

const TEMPLATE: &str = include_str!("../../templates/GEMINI.md.j2");

/// Render the `GEMINI.md` output for the given policy.
///
/// # Errors
///
/// Returns [`Error::Render`] if the template fails to compile or render.
pub fn render(policy: &Policy) -> Result<RenderedOutput> {
    let mut env = Environment::new();
    env.add_template("GEMINI.md", TEMPLATE)
        .map_err(|e| Error::Render {
            target: "GEMINI.md".to_owned(),
            source: e,
        })?;

    let tmpl = env.get_template("GEMINI.md").map_err(|e| Error::Render {
        target: "GEMINI.md".to_owned(),
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
            target: "GEMINI.md".to_owned(),
            source: e,
        })?;

    Ok(RenderedOutput {
        path: Utf8PathBuf::from("GEMINI.md"),
        content,
    })
}
