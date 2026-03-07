// Output target flags — which compatibility files to generate.

use serde::Serialize;

/// A stable identifier for each supported output target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TargetId {
    AgentsMd,
    ClaudeMd,
    CursorRules,
    GeminiMd,
    CopilotInstructions,
}

impl TargetId {
    /// All targets in a defined stable order.
    pub const ALL: &'static [TargetId] = &[
        TargetId::AgentsMd,
        TargetId::ClaudeMd,
        TargetId::CursorRules,
        TargetId::GeminiMd,
        TargetId::CopilotInstructions,
    ];

    /// The YAML ID string used in `outputs:` lists.
    #[must_use]
    pub fn id(self) -> &'static str {
        match self {
            TargetId::AgentsMd => "agents-md",
            TargetId::ClaudeMd => "claude-md",
            TargetId::CursorRules => "cursor-rules",
            TargetId::GeminiMd => "gemini-md",
            TargetId::CopilotInstructions => "copilot-instructions",
        }
    }

    /// A human-readable display label.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            TargetId::AgentsMd => "AGENTS.md",
            TargetId::ClaudeMd => "CLAUDE.md",
            TargetId::CursorRules => ".cursor/rules/",
            TargetId::GeminiMd => "GEMINI.md",
            TargetId::CopilotInstructions => ".github/copilot-instructions.md",
        }
    }

    /// Primary output path produced by this target.
    #[must_use]
    pub fn primary_path(self) -> &'static str {
        match self {
            TargetId::AgentsMd => "AGENTS.md",
            TargetId::ClaudeMd => "CLAUDE.md",
            TargetId::CursorRules => ".cursor/rules/default.mdc",
            TargetId::GeminiMd => "GEMINI.md",
            TargetId::CopilotInstructions => ".github/copilot-instructions.md",
        }
    }

    /// Support tier: `"stable"` or `"experimental"`.
    #[must_use]
    pub fn tier(self) -> &'static str {
        match self {
            TargetId::AgentsMd
            | TargetId::ClaudeMd
            | TargetId::CursorRules
            | TargetId::GeminiMd
            | TargetId::CopilotInstructions => "stable",
        }
    }
}

/// Which output files the policy is configured to generate.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize)]
pub struct OutputTargets {
    /// Generate `AGENTS.md` (default: true when `outputs` is omitted).
    pub agents_md: bool,
    /// Generate `CLAUDE.md`.
    pub claude_md: bool,
    /// Generate `.cursor/rules/default.mdc`.
    pub cursor_rules: bool,
    /// Generate `GEMINI.md`.
    pub gemini_md: bool,
    /// Generate `.github/copilot-instructions.md`.
    pub copilot_instructions: bool,
}

impl Default for OutputTargets {
    fn default() -> Self {
        Self {
            agents_md: true,
            claude_md: false,
            cursor_rules: false,
            gemini_md: false,
            copilot_instructions: false,
        }
    }
}

impl OutputTargets {
    /// Returns `true` if no outputs are enabled.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.agents_md
            && !self.claude_md
            && !self.cursor_rules
            && !self.gemini_md
            && !self.copilot_instructions
    }

    /// Returns the list of enabled [`TargetId`]s in stable order.
    #[must_use]
    pub fn enabled(&self) -> Vec<TargetId> {
        let mut out = Vec::new();
        if self.agents_md {
            out.push(TargetId::AgentsMd);
        }
        if self.claude_md {
            out.push(TargetId::ClaudeMd);
        }
        if self.cursor_rules {
            out.push(TargetId::CursorRules);
        }
        if self.gemini_md {
            out.push(TargetId::GeminiMd);
        }
        if self.copilot_instructions {
            out.push(TargetId::CopilotInstructions);
        }
        out
    }
}
