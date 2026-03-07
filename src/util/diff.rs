// Unified diff utility — Phase 3

use similar::{ChangeTag, TextDiff};
use std::fmt::Write;

/// Format a human-readable unified diff between `old` and `new`.
///
/// `label` is used as the file path annotation in the diff header.
#[allow(clippy::unwrap_used)] // write! to a String is infallible
pub fn unified_diff(label: &str, old: &str, new: &str) -> String {
    let diff = TextDiff::from_lines(old, new);
    let mut out = String::new();

    writeln!(out, "--- {label} (committed)").unwrap();
    writeln!(out, "+++ {label} (generated)").unwrap();

    for group in diff.grouped_ops(3) {
        writeln!(out, "@@").unwrap();
        for op in group {
            for change in diff.iter_changes(&op) {
                let prefix = match change.tag() {
                    ChangeTag::Delete => '-',
                    ChangeTag::Insert => '+',
                    ChangeTag::Equal => ' ',
                };
                write!(out, "{prefix}{}", change.value()).unwrap();
                if change.missing_newline() {
                    writeln!(out).unwrap();
                }
            }
        }
    }
    out
}

/// Normalize line endings to `\n` before comparison.
///
/// Prevents false diff failures on Windows where files may be checked out
/// with `\r\n` line endings.
pub fn normalize_line_endings(s: &str) -> String {
    s.replace("\r\n", "\n")
}
