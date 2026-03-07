//! `agent-policy check` — verify committed generated files match the current policy.

use std::path::PathBuf;

use camino::Utf8Path;

use crate::{
    error::{Error, Result},
    load,
    model::normalize,
    render,
    util::{diff, fs::read_if_exists},
};

/// Result of checking one output file.
enum FileCheck {
    Ok,
    Missing { path: String },
    Stale { path: String, diff: String },
}

/// Run the `check` command.
///
/// Loads the policy, renders all outputs in memory, then compares each to
/// the committed file on disk. Exits with an error if any file is missing or
/// out of date.
///
/// # Errors
///
/// Returns [`Error::CheckFailed`] if any generated file is stale or missing.
/// Returns other [`crate::Error`] variants if the load/normalize/render
/// pipeline fails.
pub fn run(config: &Utf8Path) -> Result<()> {
    let raw = load::load_file(config)?;
    let policy = normalize(raw)?;
    let outputs = render::render_all(&policy)?;

    let mut checks: Vec<FileCheck> = Vec::new();

    for output in &outputs {
        let generated = diff::normalize_line_endings(&output.content);
        let committed = read_if_exists(output.path.as_std_path())?;

        match committed {
            None => {
                checks.push(FileCheck::Missing {
                    path: output.path.to_string(),
                });
            }
            Some(committed_raw) => {
                let committed_norm = diff::normalize_line_endings(&committed_raw);
                if committed_norm == generated {
                    checks.push(FileCheck::Ok);
                } else {
                    let d = diff::unified_diff(output.path.as_str(), &committed_norm, &generated);
                    checks.push(FileCheck::Stale {
                        path: output.path.to_string(),
                        diff: d,
                    });
                }
            }
        }
    }

    let failures: Vec<&FileCheck> = checks
        .iter()
        .filter(|c| !matches!(c, FileCheck::Ok))
        .collect();

    if failures.is_empty() {
        let count = checks.len();
        println!("\u{2713} All {count} generated file(s) are up to date.");
        return Ok(());
    }

    eprintln!("Generated files are out of date:\n");
    for check in &failures {
        match check {
            FileCheck::Missing { path } => {
                eprintln!("  missing  {path}");
                eprintln!("  \u{2192} Run: agent-policy generate\n");
            }
            FileCheck::Stale { path, diff: d } => {
                eprintln!("  stale    {path}");
                eprintln!("{d}");
            }
            FileCheck::Ok => unreachable!(),
        }
    }

    eprintln!("Run `agent-policy generate` to update.");
    Err(Error::CheckFailed {
        path: failures
            .first()
            .map(|c| match c {
                FileCheck::Missing { path } | FileCheck::Stale { path, .. } => PathBuf::from(path),
                FileCheck::Ok => unreachable!(),
            })
            .unwrap_or_default(),
    })
}
