# Usage Model

## Overview

`agent-policy` is used as a repo-level policy compiler:

1. Maintain one canonical file: `agent-policy.yaml`.
2. Generate compatibility artifacts for agent tools.
3. Enforce drift checks in CI so generated files stay in sync.

Core contract:

`agent-policy.yaml` (source of truth) -> `agent-policy generate` (artifacts) -> `agent-policy check` (CI integrity gate)

---

## Default Operator Workflow

> **Implementation status:** This workflow describes the intended v0.1 end-state. CLI commands become operational as phases are completed. Phase 0 is done (project scaffolds and compiles). Phases 1–4 implement this workflow.

1. Initialize policy:

```bash
agent-policy init
```

2. Edit `agent-policy.yaml`.
3. Generate outputs:

```bash
agent-policy generate
```

4. Commit both policy and generated files.
5. Run drift check locally and in CI:

```bash
agent-policy check
```

---

## Usage Model by Phase

### Phase 0 — Bootstrap

Introduces:

- Project scaffold and CI foundation.

Usage change:

- No end-user workflow yet; project is not functionally usable.

### Phase 1 — Core Model

Introduces:

- Schema validation and normalization of `agent-policy.yaml`.

Usage change:

- Invalid policy definitions fail early with actionable errors.
- Policy authoring becomes a stable, reviewable interface.

### Phase 2 — Generation

Introduces:

- `init` and `generate` commands.
- First output targets: `AGENTS.md`, `CLAUDE.md`, `.cursor/rules/`.

Usage change:

- Teams adopt generate-and-commit workflow for compatibility artifacts.

### Phase 3 — Integrity

Introduces:

- `check` command and CI drift enforcement.

Usage change:

- Manual edits to generated files are rejected by CI.
- Standard workflow becomes: edit policy -> generate -> check -> commit.

### Phase 4 — Hardening & Release

Introduces:

- Path-scoped Cursor rules.
- Release-quality behavior and docs for general adoption.

Usage change:

- Cursor users get more context-aware rule application.
- CLI behavior and errors become production-grade for external users.

### Phase 5 — Governance

Introduces:

- Governance outputs (starting with `CODEOWNERS`, optional ruleset config).

Usage change:

- `agent-policy.yaml` starts driving both agent instruction files and repo governance artifacts.

### Phase 6 — Enforcement

Introduces:

- Hook-related outputs and policy linting (`agent-policy lint`).

Usage change:

- Teams add pre-generation semantic checks and enforcement-adjacent integrations.

### Phase 7 — Live Policy (MCP)

Introduces:

- `agent-policy serve` and machine-queryable live policy resource/tooling.

Usage change:

- Agents can query policy at runtime instead of relying only on static files.

### Phase 8 — Ecosystem

Introduces:

- Language wrappers and broader template ecosystem support.

Usage change:

- Adoption broadens beyond Rust-native workflows; integration paths diversify.

---

## Cross-Cutting Track: Target Expansion (Post-v0.1)

This track expands output coverage without destabilizing the core policy model.

Planned usage evolution:

1. Add target visibility (`list-targets`) while keeping current `outputs.*` behavior.
2. Add first-wave high-value targets (for example `GEMINI.md`, Copilot repo/path instructions, Junie guidelines).
3. Expand to additional rule ecosystems.
4. Consider `--targets` and profiles only after proven demand and safe migration path.

See:

- [target-support-policy.md](target-support-policy.md)
- [compatibility-matrix.md](compatibility-matrix.md)
- [planning/target-expansion.md](../planning/target-expansion.md)
