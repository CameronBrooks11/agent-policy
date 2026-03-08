use crate::{error::Result, load::load_file, model::normalize};
use camino::Utf8Path;

/// Run the lint command to validate policy constraints and semantics.
///
/// # Errors
/// Returns an error if the policy fails to load, or if any critical validation errors are found.
pub fn run(config_path: &Utf8Path) -> Result<()> {
    let raw = load_file(config_path)?;
    let (policy, default_warnings) = normalize(raw)?;

    let mut errors = Vec::new();
    let mut warnings = default_warnings;

    // Check for path conflicts in global paths
    for editable_path in &policy.paths.editable {
        if policy.paths.protected.contains(editable_path) {
            errors.push(format!(
                "Path conflict: '{editable_path}' is listed in both paths.editable and paths.protected."
            ));
        }
    }

    // Check for issues within roles
    for role in policy.roles.values() {
        if role.editable.is_empty() {
            warnings.push(format!(
                "Role '{}': has no editable paths. It will not be able to modify anything.",
                role.name
            ));
        }

        for editable_path in &role.editable {
            if role.forbidden.contains(editable_path) {
                errors.push(format!(
                    "Role '{}': Path conflict - '{editable_path}' is listed in both editable and forbidden.",
                    role.name
                ));
            }
        }
    }

    let has_errors = !errors.is_empty();

    // Print warnings
    for warn in warnings {
        println!("⚠️  Warning: {warn}");
    }

    // Print errors
    for err in errors {
        eprintln!("❌ Error: {err}");
    }

    if has_errors {
        eprintln!("\nLint failed with errors.");
        std::process::exit(1);
    } else {
        println!("\n✅ Lint passed.");
        Ok(())
    }
}
