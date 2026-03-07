//! `agent-policy generate` — load, normalize, render, and write output files.

use camino::Utf8Path;

use crate::{error::Result, load, model::normalize, render, util::fs::write_atomic};

/// Run the `generate` command.
///
/// Loads and validates `agent-policy.yaml` at `config`, normalizes the model,
/// renders all enabled outputs, and writes them to disk.
///
/// # Errors
///
/// Returns an [`crate::Error`] if any step of the pipeline fails.
pub fn run(config: &Utf8Path) -> Result<()> {
    let raw = load::load_file(config)?;
    let (policy, warnings) = normalize(raw)?;

    for w in &warnings {
        eprintln!("warning: {w}");
    }

    let outputs = render::render_all(&policy)?;

    for output in &outputs {
        write_atomic(output.path.as_std_path(), &output.content)?;
        println!("  wrote  {}", output.path);
    }

    println!("\nGenerated {} file(s).", outputs.len());
    Ok(())
}
