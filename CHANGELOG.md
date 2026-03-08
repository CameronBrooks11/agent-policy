# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] — 2026-03-07

### Added

- `windsurf-rules` output target — generates `.windsurf/rules/` rule files for Codeium's Cascade AI based on `trigger: glob`
- `clinerules` output target — generates scoped `.clinerules/` logic via `paths:` matching schemas
- `copilot-instructions-scoped` output target — generates domain specific path-centric rules into `.github/instructions/`
- `junie-guidelines` output target — generates single-file framework rules for JetBrains Junie inside `.junie/guidelines.md`
- Added `--targets` flag to the CLI (`generate` and `check`) to quickly override configured behaviors dynamically
- Established formal Support Tiers `enum Tier { Stable, Experimental }` to classify targets and output requirements going forward

### Changed

- Expanded the validation schema (`agent-policy.schema.json`) to allow four new outputs mapping
- `docs/compatibility-matrix.md` updated reflecting `windsurf-rules`, `clinerules`, `copilot-instructions-scoped`, and `junie-guidelines` graduation to Core execution wave targets
- `docs/reference/targets/index.md` appropriately updated resolving all four experimental specs

### Fixed

- Resolved a Jinja template unpack error during `roles` iteration when rendering JetBrains Junie templates
- Added missing `# Errors` documentation block in newly introduced renderers fixing strict Clippy warnings

## [0.2.1] — 2026-03-07

### Added

- Auto-inject generation targets into `paths.generated`. Users no longer need to manually list output targets under `paths.generated` in their config.
- Non-fatal warnings when a user explicitly lists a generated path that is already handled automatically.

## [0.2.0] — 2026-03-07

### Added

- `agent-policy list-targets` — print a table of all supported output targets with their ID, output path, and stability tier
- `gemini-md` output target — generates `GEMINI.md` for Google Gemini CLI
- `copilot-instructions` output target — generates `.github/copilot-instructions.md` for GitHub Copilot (Chat, coding agent, code review)
- `schema_version` field — required top-level field in `agent-policy.yaml`; use `"1"` for all current files. Enforced via JSON Schema so missing or mismatched versions produce a clear error
- `TargetId` enum (`AgentsMd`, `ClaudeMd`, `CursorRules`, `GeminiMd`, `CopilotInstructions`) with `id()`, `label()`, `primary_path()`, `tier()` methods
- `OutputTargets::enabled()` — returns the list of active `TargetId`s in stable order
- Test: `missing_schema_version_fails_validation` — verifies a config without `schema_version` is rejected
- Test: `list_targets_runs_and_includes_all_ids` — verifies all 5 targets appear in CLI output

### Changed

- `agent-policy init` now emits `schema_version: "1"` at the top and lists all five output targets (four commented out) with inline comments showing each target's output path and tool
- All test fixtures updated to include `schema_version: "1"`
- JSON Schema `outputs` enum expanded to include `"gemini-md"` and `"copilot-instructions"`
- `docs/schema-reference.md` updated with `schema_version` field documentation and new `outputs` values
- `docs/compatibility-matrix.md` — `GEMINI.md` and `.github/copilot-instructions.md` rows updated from Planned to Core
- `docs/reference/targets/index.md` — `gemini-md` and `copilot-instructions` moved from "v0.2 planned" to "v0.2 supported"

## [0.1.0] — 2026-03-07

### Added

- `agent-policy init` — write a starter `agent-policy.yaml` with commented-out optional sections
- `agent-policy generate` — generate `AGENTS.md`, `CLAUDE.md`, and `.cursor/rules/` from `agent-policy.yaml`
- `agent-policy check` — verify committed generated files match the current policy (exits non-zero on drift, prints unified diff)
- `--config` / `-c` flag on `generate` and `check` for non-default config locations
- JSON Schema (`agent-policy.schema.json`, Draft 2020-12) validating all policy configuration
- Path-scoped `.cursor/rules/` generation: `default.mdc` (global) plus one `.mdc` per agent role with `globs` set to the role's editable patterns
- Human-readable unified diff output from `check` on stale files (diff to stderr, success to stdout)
- Golden snapshot tests for all three render targets across minimal, full, and website examples
- CLI integration tests and `check` command integration tests
- Self-dogfooding: `agent-policy` uses itself to manage its own `AGENTS.md` and `CLAUDE.md`
- CI `policy-check` job enforcing that generated files stay in sync

[0.2.0]: https://github.com/CameronBrooks11/agent-policy/releases/tag/v0.2.0
[0.1.0]: https://github.com/CameronBrooks11/agent-policy/releases/tag/v0.1.0
