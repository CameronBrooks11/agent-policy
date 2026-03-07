# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[0.1.0]: https://github.com/CameronBrooks11/agent-policy/releases/tag/v0.1.0
