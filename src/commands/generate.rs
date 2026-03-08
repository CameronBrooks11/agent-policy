//! `agent-policy generate` — load, normalize, render, and write output files.

use camino::Utf8Path;

use crate::{
    error::{Error, Result},
    load,
    model::{normalize, targets::TargetId},
    render,
    util::fs::write_atomic,
};

/// Run the `generate` command.
///
/// Loads and validates `agent-policy.yaml` at `config`, normalizes the model,
/// renders all enabled outputs, and writes them to disk.
///
/// # Errors
///
/// Returns an [`crate::Error`] if any step of the pipeline fails.
pub fn run(config: &Utf8Path, targets: Option<&[String]>) -> Result<()> {
    let raw = load::load_file(config)?;
    let (mut policy, warnings) = normalize(raw)?;

    for w in &warnings {
        eprintln!("warning: {w}");
    }

    if let Some(target_strs) = targets {
        let mut explicit_targets = Vec::new();
        for t in target_strs {
            let id = TargetId::from_id(t).ok_or_else(|| Error::UnknownTarget { id: t.clone() })?;
            explicit_targets.push(id);
        }
        policy.outputs = crate::model::targets::OutputTargets::from_targets(&explicit_targets);
    }

    let outputs = render::render_all(&policy)?;

    let base_dir = config.parent().unwrap_or(Utf8Path::new(""));

    for output in &outputs {
        let path = base_dir.join(&output.path);
        write_atomic(path.as_std_path(), &output.content)?;
        println!("  wrote  {path}");
    }

    println!("\nGenerated {} file(s).", outputs.len());
    Ok(())
}
