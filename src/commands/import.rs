use crate::error::Result;
use camino::Utf8Path;
use std::fmt::Write;
use std::fs;

/// Try to import existing constraints and generate a starter agent-policy.yaml.
///
/// # Errors
/// Returns an error if writing the resulting config fails.
pub fn run() -> Result<()> {
    let out_path = Utf8Path::new("agent-policy.yaml");
    if out_path.exists() {
        println!("⚠️  agent-policy.yaml already exists. Skipping import.");
        return Ok(());
    }

    let agents_md = Utf8Path::new("AGENTS.md");
    let claude_md = Utf8Path::new("CLAUDE.md");

    let mut editable: Vec<String> = Vec::new();
    let mut imported_files = Vec::new();

    for file_path in [agents_md, claude_md] {
        if file_path.exists() {
            imported_files.push(file_path.as_str());
            if let Ok(content) = fs::read_to_string(file_path) {
                for line in content.lines() {
                    let line = line.trim();
                    // Extremely naive heuristic for path extraction:
                    // Look for bullet lists with backticks like: - `src/**`
                    if line.starts_with('-') && line.contains('`') {
                        let parts: Vec<&str> = line.split('`').collect();
                        if parts.len() >= 3 {
                            let extracted_path = parts[1];
                            if !extracted_path.is_empty() {
                                // Add to editable as a guess, avoiding duplicates
                                if !editable.contains(&extracted_path.to_string()) {
                                    editable.push(extracted_path.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut yaml_out = String::from("schema_version: \"1\"\n");
    yaml_out.push_str("project:\n  name: imported-project\n\n");

    if !imported_files.is_empty() {
        let _ = writeln!(
            yaml_out,
            "# Imported paths from: {}",
            imported_files.join(", ")
        );
    }

    yaml_out.push_str("paths:\n  editable:\n");
    if editable.is_empty() {
        yaml_out.push_str("    - src/**\n");
    } else {
        for path in editable {
            let _ = writeln!(yaml_out, "    - {path}");
        }
    }

    yaml_out.push_str("  protected:\n    - .github/**\n");

    yaml_out.push_str("\noutputs:\n  - agents-md\n  - claude-md\n");

    fs::write(out_path, yaml_out).map_err(|e| crate::error::Error::Io {
        path: out_path.as_std_path().to_path_buf(),
        source: e,
    })?;

    println!("✅ Wrote imported config to agent-policy.yaml");
    if !imported_files.is_empty() {
        println!("Note: Check agent-policy.yaml to ensure extracted paths are correct.");
    }

    Ok(())
}
