//! Renderer for `.cursor/rules/default.mdc`.

use camino::Utf8PathBuf;
use minijinja::Environment;

use crate::{
    error::{Error, Result},
    model::normalized::Policy,
    render::RenderedOutput,
};

const TARGET: &str = ".cursor/rules/default.mdc";
const TEMPLATE: &str = include_str!("../../templates/cursor_rule.mdc.j2");

/// Render the `.cursor/rules/default.mdc` output for the given policy.
///
/// # Errors
///
/// Returns [`Error::Render`] if the template fails to compile or render.
pub fn render(policy: &Policy) -> Result<RenderedOutput> {
    let mut env = Environment::new();
    env.add_template(TARGET, TEMPLATE)
        .map_err(|e| Error::Render { target: TARGET.to_owned(), source: e })?;

    let tmpl = env
        .get_template(TARGET)
        .map_err(|e| Error::Render { target: TARGET.to_owned(), source: e })?;

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
        .map_err(|e| Error::Render { target: TARGET.to_owned(), source: e })?;

    Ok(RenderedOutput {
        path: Utf8PathBuf::from(TARGET),
        content,
    })
}
