// Output target flags — which compatibility files to generate.

use serde::Serialize;

/// Which output files the policy is configured to generate.
#[derive(Debug, Clone, Serialize)]
pub struct OutputTargets {
    /// Generate `AGENTS.md` (default: true when `outputs` is omitted).
    pub agents_md: bool,
    /// Generate `CLAUDE.md`.
    pub claude_md: bool,
    /// Generate `.cursor/rules/default.mdc`.
    pub cursor_rules: bool,
}

impl Default for OutputTargets {
    fn default() -> Self {
        Self {
            agents_md: true,
            claude_md: false,
            cursor_rules: false,
        }
    }
}

impl OutputTargets {
    /// Returns `true` if no outputs are enabled.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.agents_md && !self.claude_md && !self.cursor_rules
    }
}
