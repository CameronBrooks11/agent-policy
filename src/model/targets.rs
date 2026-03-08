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
    Clinerules,
    WindsurfRules,
    CopilotInstructionsScoped,
    JunieGuidelines,
}

impl TargetId {
    /// All targets in a defined stable order.
    pub const ALL: &'static [TargetId] = &[
        TargetId::AgentsMd,
        TargetId::ClaudeMd,
        TargetId::CursorRules,
        TargetId::GeminiMd,
        TargetId::CopilotInstructions,
        TargetId::Clinerules,
        TargetId::WindsurfRules,
        TargetId::CopilotInstructionsScoped,
        TargetId::JunieGuidelines,
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
            TargetId::Clinerules => "clinerules",
            TargetId::WindsurfRules => "windsurf-rules",
            TargetId::CopilotInstructionsScoped => "copilot-instructions-scoped",
            TargetId::JunieGuidelines => "junie-guidelines",
        }
    }

    /// Parse a target ID from its YAML string representation.
    #[must_use]
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "agents-md" => Some(TargetId::AgentsMd),
            "claude-md" => Some(TargetId::ClaudeMd),
            "cursor-rules" => Some(TargetId::CursorRules),
            "gemini-md" => Some(TargetId::GeminiMd),
            "copilot-instructions" => Some(TargetId::CopilotInstructions),
            "clinerules" => Some(TargetId::Clinerules),
            "windsurf-rules" => Some(TargetId::WindsurfRules),
            "copilot-instructions-scoped" => Some(TargetId::CopilotInstructionsScoped),
            "junie-guidelines" => Some(TargetId::JunieGuidelines),
            _ => None,
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
            TargetId::Clinerules => ".clinerules/",
            TargetId::WindsurfRules => ".windsurf/rules/",
            TargetId::CopilotInstructionsScoped => ".github/instructions/",
            TargetId::JunieGuidelines => ".junie/guidelines.md",
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
            TargetId::Clinerules => ".clinerules/default.md",
            TargetId::WindsurfRules => ".windsurf/rules/default.md",
            TargetId::CopilotInstructionsScoped => ".github/instructions/default.md",
            TargetId::JunieGuidelines => ".junie/guidelines.md",
        }
    }

    /// Glob pattern(s) that cover all output files produced by this target.
    ///
    /// Used to auto-populate `paths.generated` in the normalized model so that
    /// users do not need to list output files in both `outputs:` and
    /// `paths.generated:`.
    #[must_use]
    pub fn generated_glob(self) -> &'static str {
        match self {
            TargetId::AgentsMd => "AGENTS.md",
            TargetId::ClaudeMd => "CLAUDE.md",
            TargetId::CursorRules => ".cursor/rules/**",
            TargetId::GeminiMd => "GEMINI.md",
            TargetId::CopilotInstructions => ".github/copilot-instructions.md",
            TargetId::Clinerules => ".clinerules/**",
            TargetId::WindsurfRules => ".windsurf/rules/**",
            TargetId::CopilotInstructionsScoped => ".github/instructions/**",
            TargetId::JunieGuidelines => ".junie/guidelines.md",
        }
    }

    /// Support tier: `"stable"` or `"experimental"`.
    #[must_use]
    pub fn tier(self) -> Tier {
        match self {
            TargetId::AgentsMd
            | TargetId::ClaudeMd
            | TargetId::CursorRules
            | TargetId::GeminiMd
            | TargetId::CopilotInstructions => Tier::Stable,
            TargetId::Clinerules
            | TargetId::WindsurfRules
            | TargetId::CopilotInstructionsScoped
            | TargetId::JunieGuidelines => Tier::Experimental,
        }
    }
}

/// The stability tier of a target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Tier {
    Stable,
    Experimental,
}

impl Tier {
    /// String representation of the tier.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Tier::Stable => "stable",
            Tier::Experimental => "experimental",
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
    /// Generate `.clinerules/` directory.
    pub clinerules: bool,
    /// Generate `.windsurf/rules/` directory.
    pub windsurf_rules: bool,
    /// Generate `.github/instructions/` directory.
    pub copilot_instructions_scoped: bool,
    /// Generate `.junie/guidelines.md`.
    pub junie_guidelines: bool,
}

impl Default for OutputTargets {
    fn default() -> Self {
        Self {
            agents_md: true,
            claude_md: false,
            cursor_rules: false,
            gemini_md: false,
            copilot_instructions: false,
            clinerules: false,
            windsurf_rules: false,
            copilot_instructions_scoped: false,
            junie_guidelines: false,
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
            && !self.clinerules
            && !self.windsurf_rules
            && !self.copilot_instructions_scoped
            && !self.junie_guidelines
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
        if self.clinerules {
            out.push(TargetId::Clinerules);
        }
        if self.windsurf_rules {
            out.push(TargetId::WindsurfRules);
        }
        if self.copilot_instructions_scoped {
            out.push(TargetId::CopilotInstructionsScoped);
        }
        if self.junie_guidelines {
            out.push(TargetId::JunieGuidelines);
        }
        out
    }

    /// Construct `OutputTargets` directly from a list of `TargetId`s.
    #[must_use]
    pub fn from_targets(targets: &[TargetId]) -> Self {
        let mut out = Self {
            agents_md: false,
            claude_md: false,
            cursor_rules: false,
            gemini_md: false,
            copilot_instructions: false,
            clinerules: false,
            windsurf_rules: false,
            copilot_instructions_scoped: false,
            junie_guidelines: false,
        };
        for t in targets {
            match t {
                TargetId::AgentsMd => out.agents_md = true,
                TargetId::ClaudeMd => out.claude_md = true,
                TargetId::CursorRules => out.cursor_rules = true,
                TargetId::GeminiMd => out.gemini_md = true,
                TargetId::CopilotInstructions => out.copilot_instructions = true,
                TargetId::Clinerules => out.clinerules = true,
                TargetId::WindsurfRules => out.windsurf_rules = true,
                TargetId::CopilotInstructionsScoped => out.copilot_instructions_scoped = true,
                TargetId::JunieGuidelines => out.junie_guidelines = true,
            }
        }
        out
    }
}
