# `copilot-instructions` — `.github/copilot-instructions.md`

## Summary

A repository-wide custom instructions file for GitHub Copilot. Placed at `.github/copilot-instructions.md`, it provides Copilot with context on how to understand the project and how to build, test, and validate changes. Instructions are automatically included in Copilot Chat, Copilot coding agent, and Copilot code review requests made in the context of the repository.

This is the **Copilot-specific** extended instructions target. It is distinct from agent-neutral files (`AGENTS.md`, `CLAUDE.md`, `GEMINI.md`) that Copilot also reads. The `copilot-instructions.md` file applies only to GitHub Copilot and has no effect in other tools.

## Tool

- **Primary:** GitHub Copilot — Copilot Chat (github.com/copilot), Copilot coding agent, Copilot code review
- **Requires:** GitHub Copilot subscription (Individual, Business, or Enterprise)
- Not read by Cursor, Claude Code, Gemini CLI, or any non-GitHub tool.

## Support level

`planned` — v0.2 target. Researched 2026-03-07.

## Canonical path(s)

- `.github/copilot-instructions.md` — the only valid path; no alternatives exist.
- The `.github/` directory must be present at the repository root (standard for GitHub configuration files; typically already exists in GitHub-hosted repos).

## Alternate / legacy path(s)

None. The path `.github/copilot-instructions.md` is fixed by the vendor spec with no configurable alternatives and no legacy variants documented.

## File format

Natural language instructions in Markdown format. No required structure.

- Whitespace between instructions is ignored — instructions may be written as a single paragraph, each on a new line, or separated by blank lines.
- No frontmatter.
- Best practice per vendor guidance: keep to approximately 2 pages or fewer; instructions must not be task-specific (they persist across all Copilot interactions in the repo).

**Copilot coding agent prompt guidance** (verbatim abridged from GitHub Docs, accessed 2026-03-07):

> Your task is to "onboard" this repository to Copilot coding agent... Instructions must be no longer than 2 pages. Instructions must not be task specific.

Recommended content categories (per vendor `WhatToAdd` guidance):

| Category | What to include |
|----------|----------------|
| High-level details | What the repo does, languages, frameworks, target runtimes, repo size/type |
| Build instructions | Bootstrap, build, test, run, lint commands with tool versions; document working command sequences and known failure modes |
| Project layout | Major architectural elements, relative paths to key files, location of config/lint/CI files; describe CI pipeline checks |

## Frontmatter

None for `.github/copilot-instructions.md`.

(Note: the related file type `.github/instructions/NAME.instructions.md` does use YAML frontmatter with `applyTo` for path-scoped instructions — but that is a separate, out-of-scope target for v0.2.)

## Discovery / scope behavior

Source: GitHub Docs — "Adding repository custom instructions for GitHub Copilot" (accessed 2026-03-07).

- Instructions are **automatically** added to all Copilot requests made in the context of the repository — no user action required beyond committing the file.
- Applied in: Copilot Chat (github.com), Copilot coding agent, Copilot code review.
- Whenever repository custom instructions are used, the file is listed as a **reference** in the chat response. Users can click it to inspect which instructions were applied.

**Priority order** (highest to lowest — all sets are provided to Copilot simultaneously):

1. Personal instructions (user-level)
2. Repository instructions — `.github/copilot-instructions.md` ← this target
3. Organization instructions (org-level)

All three levels are provided; Copilot uses all of them. Avoid providing conflicting sets across levels.

**Code review behavior:** When reviewing a pull request, Copilot uses the instructions from the **base branch**, not the head branch. Policy changes in a feature branch are not effective until merged.

**Enabling/disabling:** Repository custom instructions for code review can be toggled per-repo in: Settings → Copilot → Code review → "Use custom instructions when reviewing pull requests". Enabled by default.

## Repo-safe

**Yes.** `.github/copilot-instructions.md` is designed to be committed to version control.

## Renderer notes

- **Target ID:** `copilot-instructions`
- **Output path:** `.github/copilot-instructions.md`
- **Template:** `templates/copilot-instructions.md.j2`
- **`OutputTargets` field:** `copilot_instructions: bool`
- **Notes:**
  - Output path contains a subdirectory (`.github/`). The existing `util/fs.rs` atomic write creates parent directories automatically — no special handling needed.
  - Content is structurally identical to `AGENTS.md`. Template can mirror `AGENTS.md.j2` closely.
  - Single file output.
  - Snapshot filename: insta will encode `.github/copilot-instructions.md` as `.github__copilot-instructions.md` in the snap filename — verify after first test run.

## Known limitations / gotchas

- **Triple-redundant context with other Copilot-readable files:** GitHub Copilot also reads `AGENTS.md`, `CLAUDE.md`, and `GEMINI.md`. If a user enables `agents-md`, `claude-md`, and `copilot-instructions` simultaneously, Copilot will receive the same policy content three times. This is by-design GitHub behavior but consumes additional context tokens. Recommend noting this in the schema reference.

- **Code review uses base branch:** A policy change in a feature branch PR will not be applied to code review of that PR. Only takes effect after merge.

- **Not read by non-GitHub tools:** Unlike `AGENTS.md`, this file has no effect in Cursor, Claude Code, Gemini CLI, or other non-GitHub Copilot tools.

- **No path-scoped variant generated (v0.2):** GitHub supports path-specific instructions at `.github/instructions/NAME.instructions.md` with `applyTo` frontmatter globs. This is a distinct and more powerful feature not generated in v0.2; deferred to v0.3+.

- **Context, not enforcement:** Like all tools in this ecosystem, GitHub Copilot treats the instructions as prompt context injected into the model. Adhering to specific constraints depends on model behavior.

## Official references

- GitHub Docs — "Adding repository custom instructions for GitHub Copilot": <https://docs.github.com/en/copilot/how-tos/configure-custom-instructions/add-repository-instructions> (accessed 2026-03-07)
- GitHub Docs — "About customizing GitHub Copilot responses" (custom instruction support matrix): <https://docs.github.com/en/copilot/concepts/prompting/response-customization> (not directly fetched; linked from primary source)
- GitHub Docs — "Support for different types of custom instructions": <https://docs.github.com/en/copilot/reference/custom-instructions-support> (not directly fetched; linked from primary source)

## Minimal example

```markdown
<!-- .github/copilot-instructions.md — generated by agent-policy. Do not edit manually. -->

## About this repository

Schema-first generator for coding-agent repo policies and compatibility files.
Written in Rust. Published to crates.io as `agent-policy`.

## Build and test

- Build: `cargo build --release`
- Test: `cargo test --all-targets`
- Lint: `cargo clippy --all-targets -- -D warnings`
- Format: `cargo fmt --check`

MSRV: 1.75. Use only APIs stable in Rust 1.75.

## Project layout

- `src/` — library and CLI source
- `src/render/` — one module per output target
- `templates/` — minijinja templates for each target
- `tests/` — integration tests; `tests/snapshots/` — golden snapshots
- `agent-policy.yaml` — this repo's own policy (self-dogfooding)
- `agent-policy.schema.json` — JSON Schema for the YAML format

## Constraints

- Do not modify `agent-policy.schema.json` or `Cargo.toml` without human review.
- Do not modify `.github/workflows/` without human review.
- Always provide tests for changes to `src/`.
- Never commit secrets, credentials, or API keys.
```

## Internal mapping notes

- **Target ID:** `copilot-instructions` — to be added to `TargetId` enum in `src/model/targets.rs`
- **Output path constant:** `".github/copilot-instructions.md"` — to be returned by `TargetId::primary_path()`
- **Template file:** `templates/copilot-instructions.md.j2` — not yet created
- **Golden test snapshots:** not yet created
- **Renderer module:** `src/render/copilot_instructions.rs` — not yet created
- **Status:** planned for v0.2
