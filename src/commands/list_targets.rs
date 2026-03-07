//! `agent-policy list-targets` — print all supported output targets.

use crate::model::targets::TargetId;

/// Run the `list-targets` command.
///
/// Prints a table of all supported output targets with their YAML ID,
/// display label, output path, and support tier.
pub fn run() {
    println!("{:<25} {:<35} {:<10}", "ID", "OUTPUT PATH", "TIER");
    println!("{}", "-".repeat(72));
    for target in TargetId::ALL {
        println!(
            "{:<25} {:<35} {:<10}",
            target.id(),
            target.primary_path(),
            target.tier(),
        );
    }
}
