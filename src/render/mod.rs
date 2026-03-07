// Render pipeline — Phase 2

use camino::Utf8PathBuf;

use crate::{error::Result, model::normalized::Policy};

pub mod agents_md;
pub mod claude_md;
pub mod copilot_instructions;
pub mod cursor_rules;
pub mod gemini_md;

/// A single rendered output file.
pub struct RenderedOutput {
    /// Relative path from the repo root where this file should be written.
    pub path: Utf8PathBuf,
    /// The rendered string content.
    pub content: String,
}

/// Render all outputs enabled by the policy.
///
/// Returns a list of outputs in a deterministic order:
/// `AGENTS.md` → `CLAUDE.md` → cursor rules → `GEMINI.md` → copilot instructions.
///
/// # Errors
///
/// Returns [`crate::Error::Render`] if any template fails to render.
pub fn render_all(policy: &Policy) -> Result<Vec<RenderedOutput>> {
    let mut outputs = Vec::new();
    if policy.outputs.agents_md {
        outputs.push(agents_md::render(policy)?);
    }
    if policy.outputs.claude_md {
        outputs.push(claude_md::render(policy)?);
    }
    if policy.outputs.cursor_rules {
        // cursor_rules returns Vec — one default.mdc plus one per role
        outputs.extend(cursor_rules::render(policy)?);
    }
    if policy.outputs.gemini_md {
        outputs.push(gemini_md::render(policy)?);
    }
    if policy.outputs.copilot_instructions {
        outputs.push(copilot_instructions::render(policy)?);
    }
    Ok(outputs)
}
