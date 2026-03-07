# Roadmap: agent-policy

This document is the top-level planning overview. Phases 0–4 (the path to `v0.1.0` and crates.io publication) each have a dedicated detail document in this directory. Phases 5–8 are planned future work and are described here only at summary level.

A parallel post-`v0.1.0` target-coverage expansion track is documented in [target-expansion.md](target-expansion.md).

For the full architectural vision and design principles, see [docs/vision.md](../docs/vision.md).

---

## Phases at a Glance

| Phase | Name                | Detail doc                                     | Status  |
| ----- | ------------------- | ---------------------------------------------- | ------- |
| 0     | Bootstrap           | [phase-0-bootstrap.md](phase-0-bootstrap.md)   | Pending |
| 1     | Core Model          | [phase-1-core-model.md](phase-1-core-model.md) | Pending |
| 2     | Generation          | [phase-2-generation.md](phase-2-generation.md) | Pending |
| 3     | Integrity           | [phase-3-integrity.md](phase-3-integrity.md)   | Pending |
| 4     | Hardening & Release | [phase-4-release.md](phase-4-release.md)       | Pending |
| 5     | Governance          | _(this doc)_                                   | Future  |
| 6     | Enforcement         | _(this doc)_                                   | Future  |
| 7     | Live Policy         | _(this doc)_                                   | Future  |
| 8     | Ecosystem           | _(this doc)_                                   | Future  |

---

## Cross-Cutting Planning Track

In addition to numbered phases, one cross-cutting track is maintained:

| Track | Name             | Detail doc                                 | Status  |
| ----- | ---------------- | ------------------------------------------ | ------- |
| T1    | Target Expansion | [target-expansion.md](target-expansion.md) | Planned |

This track captures how to expand compatibility outputs beyond the v0.1 core (`AGENTS.md`, `CLAUDE.md`, `.cursor/rules/`) without destabilizing the canonical policy model.

---

## Phases 0–4 — Path to v0.1.0 and crates.io

Each of these phases has a dedicated document covering goals, tasks, implementation detail, and a concrete exit condition. Work through them in order — each phase depends on the prior one being fully complete.

| Phase | Name                | Key deliverable                                                                                                      | Detail                                         |
| ----- | ------------------- | -------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------- |
| 0     | Bootstrap           | Compiling skeleton, CI green, all scaffolding committed                                                              | [phase-0-bootstrap.md](phase-0-bootstrap.md)   |
| 1     | Core Model          | `agent-policy.yaml` loads, validates, and normalizes into a stable `Policy` struct                                   | [phase-1-core-model.md](phase-1-core-model.md) |
| 2     | Generation          | `init` and `generate` commands work; `AGENTS.md`, `CLAUDE.md`, `.cursor/rules/` emitted; golden tests pass           | [phase-2-generation.md](phase-2-generation.md) |
| 3     | Integrity           | `check` command works; CI uses it; tool self-dogfoods its own `AGENTS.md`                                            | [phase-3-integrity.md](phase-3-integrity.md)   |
| 4     | Hardening & Release | Path-scoped Cursor rules; all pre-publish checks pass; `v0.1.0` tagged and published to crates.io | [phase-4-release.md](phase-4-release.md)       |

---

## Phase 5 — Repository Governance

**Goal:** Extend generation to cover repository governance files — `CODEOWNERS` and optionally GitHub branch ruleset configuration.

### Tasks

- [ ] Add `governance` section to `agent-policy.schema.json`:
  - `codeowners` — path-to-owner mappings derived from roles and paths
  - `branch_protection` — basic branch rule flags
- [ ] Implement `CODEOWNERS` renderer
  - derive ownership from `roles` + `paths.protected`
- [ ] Add `CODEOWNERS` to `outputs` flags
- [ ] Implement optional GitHub ruleset config generator (JSON export format)
- [ ] Add governance golden tests
- [ ] Document governance generation in README

### Exit Condition

- `agent-policy generate` can emit a `CODEOWNERS` file consistent with the declared role and path policy
- At least one real repo adopts the generated `CODEOWNERS`

---

## Phase 6 — Enforcement Integration

**Goal:** Generate enforcement-adjacent artifacts — Claude Code hook configuration templates and CI policy validation hooks.

### Tasks

- [ ] Research current Claude Code hooks API surface
- [ ] Add `hooks` section to schema:
  - pre-tool-use hooks
  - post-tool-use hooks
  - notification hooks
- [ ] Implement Claude hook config template renderer
- [ ] Add `claude_hooks` target to `outputs` flags
- [ ] Add policy linter (warn on conflicting path rules, overlapping roles, etc.)
- [ ] Implement `lint` command:
  - `agent-policy lint` — validates policy for semantic consistency
- [ ] Document enforcement boundary clearly in README and in generated files

### Exit Condition

- `agent-policy generate` can emit a `hook_config.json` or equivalent Claude hooks template
- `agent-policy lint` catches common policy definition errors before generation

---

## Phase 7 — Live Policy Exposure (MCP)

**Goal:** Expose the policy as a structured, machine-queryable MCP resource that agents can introspect at runtime without reading static files.

### Tasks

- [ ] Implement a minimal MCP server mode:
  - `agent-policy serve` — starts a local MCP server
  - Exposes a `repo_policy` resource (read-only, structured JSON)
  - Exposes a `policy_check` tool (validate a proposed action against policy)
- [ ] Define MCP resource schema aligned with the internal normalized model
- [ ] Add `--mcp` flag or config option to enable in CI/dev environments
- [ ] Document MCP integration pattern

### Notes

MCP is an evolving standard. This phase should only begin once the MCP specification has stabilized and real agent tooling has demonstrated reliable MCP resource consumption patterns. Do not implement this phase prematurely.

### Exit Condition

- An MCP client (e.g. Claude Code, Cursor with MCP support) can query live repo policy without reading any static file
- The MCP resource schema is stable enough to version

---

## Phase 8 — Ecosystem

**Goal:** Broaden access through language wrappers, community-contributed templates, and agent role libraries.

### Tasks

- [ ] Thin Python wrapper:
  - `pip install agent-policy`
  - delegates to bundled or installed binary
- [ ] Thin Node.js wrapper:
  - `npm install agent-policy`
  - delegates to bundled or installed binary
- [ ] Community target template system:
  - allow custom renderer templates from user-defined paths
  - document template authoring guide
- [ ] Agent role library:
  - pre-built role definitions for common roles (docs agent, frontend agent, infra agent, security agent)
  - shareable via URI reference in `agent-policy.yaml`
- [ ] Policy schema registry concept (versioned, published schema for adoption)

### Exit Condition

- Users in Python or Node environments can install and use `agent-policy` without knowing Rust
- Community can contribute renderer templates without forking the core tool

---

## Design Decisions (Locked)

These decisions should not be revisited without strong justification:

| Decision                                     | Rationale                                                                  |
| -------------------------------------------- | -------------------------------------------------------------------------- |
| YAML as canonical authoring format           | Human-reviewable, machine-readable, widely supported, easy to diff         |
| JSON Schema as machine contract              | Language-neutral, already aligned with MCP and agentic ecosystems          |
| Rust for core implementation                 | Single binary, portable, future-embeddable, strong CLI ecosystem           |
| Generated files committed to consuming repos | Tool-compatible, diff-visible in PRs, no runtime dependency                |
| Emit conventions, do not replace them        | AGENTS.md has cross-tool momentum; work with it                            |
| No custom DSL                                | YAML + JSON Schema is sufficient; a DSL adds maintenance cost without gain |
| Policy generation ≠ enforcement              | Clean separation keeps the scope honest and composable                     |
| Normalize before rendering                   | Decouples YAML surface from renderer; enables safe schema evolution        |

---

## Non-Goals (Permanent)

- Not a SaaS platform
- Not a hosted service or daemon
- Not a replacement for `AGENTS.md` or any other agent convention
- Not a full policy enforcement engine
- Not a GitHub API automation framework (governance generation, yes; automation, no)

---

## Versioning Intent

| Version | Milestone                                                                                                               |
| ------- | ----------------------------------------------------------------------------------------------------------------------- |
| v0.1.0  | Phase 0–4 complete: `init`, `generate`, `check`; three output targets; path-scoped cursor rules; published to crates.io |
| v0.2.0  | Phase 5 complete: CODEOWNERS generation, governance schema                                                              |
| v0.3.0  | Phase 6 complete: hook config templates, policy linter                                                                  |
| v1.0.0  | Schema declared stable; tool considered production-ready for broad internal use                                         |
| v1.x    | Phase 7 and 8 features added incrementally                                                                              |
