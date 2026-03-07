I would split the landscape into **four classes**, because not every mainstream tool uses the same concept:

1. **repo-scoped instruction files**
2. **repo-scoped rule/config directories**
3. **agent profile files**
4. **non-repo local/user config that you probably should not generate for teammates by default**

That framing matters because some tools are converging on repo files like `AGENTS.md`, while others still use tool-native config/rules folders or JSON/YAML settings. ([OpenAI Developers][1])

## The mainstream baseline to support first

These are the ones I would treat as your **core generator targets** for v1:

* `AGENTS.md` for OpenAI Codex, GitHub Copilot coding agent, and Windsurf/Cascade. Codex reads `AGENTS.md` files before work; Copilot supports one or more `AGENTS.md` files in the repo; Windsurf supports directory-scoped `AGENTS.md` discovery. ([OpenAI Developers][1])
* `CLAUDE.md` for Claude Code, and Copilot can also consume a root `CLAUDE.md` as an alternative agent-instructions file. Claude Code reads `CLAUDE.md` at session start. ([Claude][2])
* `GEMINI.md` for Gemini CLI, and Copilot can also consume a root `GEMINI.md`. Gemini CLI uses `GEMINI.md` by default and even allows imports and configurable alternate names. ([Gemini CLI][3])
* Cursor rules in `.cursor/rules/*.mdc` or `.md`; Cursor supports both, but `.mdc` with frontmatter is the richer format. ([Cursor][4])
* GitHub Copilot repository instructions and path-specific instructions, because Copilot has its own established non-AGENTS files: `.github/copilot-instructions.md` and `.github/instructions/*.instructions.md`. ([GitHub Docs][5])

If you only ship those five output families first, you already cover the biggest practical surface area. ([OpenAI Developers][1])

## Mainstream tool-by-tool reference

### OpenAI Codex

**Primary repo file:** `AGENTS.md`
**Also supports global file:** `~/.codex/AGENTS.md` or `~/.codex/AGENTS.override.md`
**Format:** plain Markdown
**Scoping:** layered; Codex reads `AGENTS.md` files before doing work, with global guidance plus project-specific overrides. ([OpenAI Developers][1])

### GitHub Copilot / VS Code Copilot / Copilot coding agent

Copilot now has **three distinct repo customization surfaces**:

**1. Repository-wide instructions**

* `.github/copilot-instructions.md`
* Markdown, repo-wide. ([GitHub Docs][5])

**2. Path-specific instructions**

* `.github/instructions/NAME.instructions.md`
* Markdown body with YAML frontmatter
* required frontmatter field: `applyTo` using glob syntax
* optional: `excludeAgent` such as `"code-review"` or `"coding-agent"`. ([GitHub Docs][5])

**3. Agent instructions for coding agents**

* `AGENTS.md` anywhere in the repo
* nearest file in the directory tree takes precedence
* Copilot also accepts a single root `CLAUDE.md` or `GEMINI.md` instead. ([GitHub Docs][5])

**4. Custom agent profiles**

* `.github/agents/*.agent.md`
* Markdown with YAML frontmatter such as `name`, `description`, `tools`, `agents`, and MCP server config. ([Visual Studio Code][6])

This means Copilot is not one target; it is really **four compatible targets**. ([GitHub Docs][5])

### Claude Code

Claude has **repo instructions + settings + subagents + memory**.

**Repo instructions**

* `CLAUDE.md` at project root, or `.claude/CLAUDE.md`
* local uncommitted override: `CLAUDE.local.md`
* format: plain Markdown. ([Claude][7])

**Project settings**

* `.claude/settings.json`
* local uncommitted override: `.claude/settings.local.json`
* format: JSON
* this is the official mechanism for permissions, sandboxing, plugins, and related behavior. ([Claude][7])

**Subagents**

* `.claude/agents/`
* VS Code’s Copilot docs confirm Claude-format agent files in `.claude/agents` are plain `.md` with Claude-specific frontmatter fields like `name`, `description`, `tools`, `disallowedTools`. ([Visual Studio Code][6])

**Per-project memory**

* machine-local, not meant for repo generation
* `MEMORY.md` is the index file inside Claude’s local project memory directory under `~/.claude/projects/<project>/memory/`
* `MEMORY.md` startup load is capped to the first 200 lines; `CLAUDE.md` is loaded in full. ([Claude][8])

For your generator, `CLAUDE.md` is the shared-repo target; `.claude/settings.json` is a possible later advanced target; `MEMORY.md` should not be repo-generated. ([Claude][7])

### Cursor

**Rules directory:** `.cursor/rules/`
**File extensions:** `.md` and `.mdc`
**Preferred rich format:** `.mdc` with frontmatter for metadata such as description and globs
**Format:** Markdown, with frontmatter for `.mdc`. ([Cursor][4])

Cursor’s docs are the least explicit in the snippets returned, but they do clearly state that `.md` and `.mdc` are supported and that `.mdc` is the richer format with frontmatter and path-based control. ([Cursor][4])

### Windsurf / Cascade

Windsurf has **multiple repo-level constructs**, and it is more formalized than many people realize.

**AGENTS.md**

* `AGENTS.md` or `agents.md` anywhere in the workspace
* plain Markdown
* automatic directory scoping based on file location. ([Windsurf Docs][9])

**Rules**

* `.windsurf/rules/*.md`
* Markdown, with optional YAML frontmatter for activation
* activation modes use `trigger`, with values like `always_on`, `model_decision`, `glob`, `manual`; glob rules also use `globs`. ([Windsurf Docs][10])

**Workflows**

* `.windsurf/workflows/*.md`
* Markdown files containing title, description, and step-by-step instructions. ([Windsurf Docs][11])

**Skills**

* `.windsurf/skills/<skill-name>/SKILL.md`
* Markdown with YAML frontmatter; example fields include `name` and `description`
* may include supporting files in the same directory. ([Windsurf Docs][12])

**Hooks**

* `.windsurf/hooks.json`
* JSON config for pre/post action shell hooks
* hooks receive JSON on stdin, and pre-hooks can block actions by exiting with code `2`. ([Windsurf Docs][13])

**Ignore file**

* `.codeiumignore` at workspace root
* gitignore-like syntax, used to block Cascade from viewing/editing/creating files in ignored paths. ([Windsurf Docs][14])

Windsurf is important because it spans both instruction files and enforceable hook/config surfaces. ([Windsurf Docs][10])

### Gemini CLI

**Primary repo file:** `GEMINI.md`
**Format:** plain Markdown
**Special features:** import syntax using `@file.md`; configurable alternate file names via `settings.json` `context.fileName`, including accepting `AGENTS.md` as a recognized context file name. ([Gemini CLI][3])

That makes Gemini especially friendly to your generator, because it already thinks in terms of repo context files rather than only app settings. ([Gemini CLI][3])

### JetBrains Junie

**Primary repo file:** `.junie/guidelines.md` at repo root
**Format:** plain Markdown or effectively free-form text; docs say it “can contain instructions in any format”
**Behavior:** automatically used if present. ([JetBrains][15])

This is a clean generator target and should be in your second wave. ([JetBrains][15])

### Continue

Continue is different: its primary customization layer is **YAML config plus rule files**.

**Main config**

* global/local config: `~/.continue/config.yaml`
* format: YAML
* top-level fields include `name`, `version`, `schema`, `models`, `context`, `rules`, `prompts`, `docs`, `mcpServers`, `data`. ([Continue Docs][16])

**Workspace rules**

* `.continue/rules/*.md`
* Markdown with frontmatter, for example `name:`
* local rules are auto-visible when using Hub configs. ([Continue Docs][17])

Continue is mainstream enough to track, but it is more of a full config platform than a simple instruction-file convention. ([Continue Docs][18])

### Cline

Cline now has a fairly mature rule system.

**Primary workspace rules**

* `.clinerules/` at project root
* all `.md` and `.txt` files inside are combined
* Markdown/text format. ([Cline Documentation][19])

**Conditional rules**

* YAML frontmatter supported
* current documented conditional is `paths` with glob arrays. ([Cline Documentation][19])

**Also imports other ecosystems**

* recognizes `.cursorrules`, `.windsurfrules`, and `AGENTS.md`
* Cline’s config UI explicitly lists Cursor rules and Windsurf rules as importable sources. ([Cline Documentation][19])

Cline is worth first-class support because it already acts like an aggregator. ([Cline Documentation][19])

### Aider

Aider is mainstream, but it does **not** have a single standardized repo instruction filename in the same way.

**Conventions file**

* docs suggest a small Markdown file such as `CONVENTIONS.md`
* usually manually loaded with `/read CONVENTIONS.md` or `aider --read CONVENTIONS.md`
* can be made persistent through `.aider.conf.yml` using `read:`. ([Aider][20])

**Config file**

* `.aider.conf.yml` in home directory or git repo root
* YAML config. ([Aider][21])

Aider should be tracked in your reference directory, but I would treat it as **manual-context / config-driven**, not as one clean repo-autodiscovery standard. ([Aider][20])

## What I would treat as the canonical baseline matrix

For your generator’s baseline data set, I would record each target like this:

* tool name
* category
* path pattern
* scope
* file format
* frontmatter format if any
* whether repo-committable
* whether officially auto-discovered
* whether shared/team-safe vs user-local only

A practical first-pass baseline would be:

* `AGENTS.md`
* `CLAUDE.md`
* `GEMINI.md`
* `.github/copilot-instructions.md`
* `.github/instructions/*.instructions.md`
* `.github/agents/*.agent.md`
* `.cursor/rules/*.mdc`
* `.windsurf/rules/*.md`
* `.windsurf/workflows/*.md`
* `.windsurf/skills/<name>/SKILL.md`
* `.windsurf/hooks.json`
* `.junie/guidelines.md`
* `.continue/rules/*.md`
* `.clinerules/*.{md,txt}`
* `.aider.conf.yml`
* `CONVENTIONS.md` as a conventional aider context file, but mark it as **not standardized auto-discovery**. ([OpenAI Developers][1])

## Suggested reference/data directory for your project

I would create something like:

```text
data/
  targets/
    agents-md.yaml
    claude-md.yaml
    gemini-md.yaml
    copilot-instructions.yaml
    copilot-path-instructions.yaml
    copilot-agent-profile.yaml
    cursor-rules.yaml
    windsurf-rules.yaml
    windsurf-workflows.yaml
    windsurf-skills.yaml
    windsurf-hooks.yaml
    junie-guidelines.yaml
    continue-rules.yaml
    continue-config.yaml
    cline-rules.yaml
    aider.yaml
```

And each target file would capture fields like:

```yaml
id: copilot-path-instructions
tool: github-copilot
status: mainstream
kind: path_scoped_instruction
repo_paths:
  - ".github/instructions/*.instructions.md"
format:
  type: markdown
  frontmatter: yaml
required_frontmatter:
  - applyTo
optional_frontmatter:
  - excludeAgent
autodiscovery: true
team_safe: true
notes:
  - "applyTo uses glob syntax"
  - "excludeAgent supports code-review or coding-agent"
```

That gives you a stable knowledge base for the generator without hard-coding everything in Rust.

## Best support order for your MVP generator

I would implement in this order:

1. `AGENTS.md`
2. `CLAUDE.md`
3. `GEMINI.md`
4. `.cursor/rules/*.mdc`
5. `.github/copilot-instructions.md`
6. `.github/instructions/*.instructions.md`
7. `.junie/guidelines.md`
8. `.clinerules/*.md`
9. `.windsurf/rules/*.md`
10. everything else

That order gives you maximum utility with minimum complexity, because the first six cover the most common repo instruction workflows while keeping formats simple. ([OpenAI Developers][1])

## Final recommendation

For your internal Rust MVP, I would define **“mainstream baseline”** as:

* OpenAI Codex
* GitHub Copilot
* Claude Code
* Cursor
* Windsurf
* Gemini CLI
* Continue
* Cline
* JetBrains Junie
* Aider

And I would treat **repo-committable instruction/rule artifacts** as first-class, while treating **user-local memory/settings** as tracked reference targets but not default generator outputs. That is the cleanest line between useful shared policy and tool-specific personal state. ([OpenAI Developers][1])

If you want, I can turn this into a concrete starter dataset schema plus 10 filled example target definition files for your Rust project.

[1]: https://developers.openai.com/codex/guides/agents-md/ "Custom instructions with AGENTS.md"
[2]: https://code.claude.com/docs/en/overview "Claude Code overview - Claude Code Docs"
[3]: https://geminicli.com/docs/cli/gemini-md/ "Provide context with GEMINI.md files | Gemini CLI"
[4]: https://cursor.com/docs/context/rules?utm_source=chatgpt.com "Rules | Cursor Docs"
[5]: https://docs.github.com/copilot/customizing-copilot/adding-custom-instructions-for-github-copilot "Adding repository custom instructions for GitHub Copilot - GitHub Docs"
[6]: https://code.visualstudio.com/docs/copilot/customization/custom-agents "Custom agents in VS Code"
[7]: https://code.claude.com/docs/en/settings "Claude Code settings - Claude Code Docs"
[8]: https://code.claude.com/docs/en/memory "How Claude remembers your project - Claude Code Docs"
[9]: https://docs.windsurf.com/windsurf/cascade/agents-md?utm_source=chatgpt.com "AGENTS.md"
[10]: https://docs.windsurf.com/windsurf/cascade/memories "Cascade Memories"
[11]: https://docs.windsurf.com/windsurf/cascade/workflows?utm_source=chatgpt.com "Workflows"
[12]: https://docs.windsurf.com/windsurf/cascade/skills?utm_source=chatgpt.com "Cascade Skills"
[13]: https://docs.windsurf.com/windsurf/cascade/hooks "Cascade Hooks"
[14]: https://docs.windsurf.com/windsurf/cascade/cascade?utm_source=chatgpt.com "Windsurf - Cascade"
[15]: https://www.jetbrains.com/guide/ai/article/junie/ "Junie Playbook - JetBrains Guide"
[16]: https://docs.continue.dev/customize/deep-dives/configuration?utm_source=chatgpt.com "How to Configure Continue"
[17]: https://docs.continue.dev/customize/deep-dives/rules?utm_source=chatgpt.com "How to Create and Manage Rules in Continue"
[18]: https://docs.continue.dev/reference "config.yaml Reference | Continue Docs"
[19]: https://docs.cline.bot/customization/cline-rules "Rules - Cline"
[20]: https://aider.chat/docs/usage/conventions.html "Specifying coding conventions | aider"
[21]: https://aider.chat/docs/config.html "Configuration | aider"
