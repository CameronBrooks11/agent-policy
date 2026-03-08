//! Renderer for `.junie/guidelines.md`.

use camino::Utf8PathBuf;
use minijinja::Environment;

use crate::{
    error::{Error, Result},
    model::normalized::Policy,
    render::RenderedOutput,
};

const TARGET: &str = ".junie/guidelines.md";
const TEMPLATE_STR: &str = include_str!("../../templates/junie_guidelines.md.j2");

/// Render the `.junie/guidelines.md` output for the given policy.
///
/// # Errors
///
/// Returns [`Error::Render`] if any template fails to compile or render.
pub fn render(policy: &Policy) -> Result<RenderedOutput> {
    let mut env = Environment::new();
    env.add_template("junie_guidelines.md", TEMPLATE_STR)
        .map_err(|e| Error::Render {
            target: TARGET.to_owned(),
            source: e,
        })?;

    let tmpl = env
        .get_template("junie_guidelines.md")
        .map_err(|e| Error::Render {
            target: TARGET.to_owned(),
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
            target: TARGET.to_owned(),
            source: e,
        })?;

    Ok(RenderedOutput {
        path: Utf8PathBuf::from(TARGET),
        content,
    })
}
