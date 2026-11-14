# Roadmap: agent-policy

> Last updated: 2026-11-13 — post v0.7.0 release.
> For architectural vision and design principles, see [vision.md](vision.md).

---

## Current State

v0.5.0 ships a complete adoption-ready feature set:

- Full target coverage across all major agent tools
- `import`, `lint`, `install-hooks` commands for migration and enforcement
- Monorepo support via `--config` relative path resolution
- Self-dogfooding, golden tests, CI drift detection, crates.io releases

Since v0.7.0 the CLI also installs without a Rust toolchain: `npx agent-policy` (npm, since v0.6.0, working on Linux since v0.7.0) and `pip install agent-policy-cli` (PyPI, since v0.7.0).

---

## Completed

| Milestone | Theme                 | Key deliverable                                                        | Status  |
| --------- | --------------------- | ---------------------------------------------------------------------- | ------- |
| v0.1      | Proof of concept      | Core pipeline, 3 targets, crates.io, self-dogfooding                   | ✅ Done |
| v0.2      | Target breadth        | GEMINI.md, Copilot instructions, `list-targets`, `schema_version`      | ✅ Done |
| v0.3      | Full target coverage  | Windsurf, Cline, Junie, `--targets` flag, stability tiers              | ✅ Done |
| v0.4      | Migration + linting   | `lint` command — semantic path conflict and scope validation            | ✅ Done |
| v0.5      | Migration (continued) | `import`, `install-hooks`, monorepo `--config` path resolution         | ✅ Done |
| v0.6      | npm wrapper           | `npx agent-policy` via per-platform binary packages                    | ✅ Done |
| v0.7      | pip wrapper           | `pip install agent-policy-cli`, npm execute-bit fix                    | ✅ Done |

---

## Path Forward

Listed roughly in priority order, but not tied to versions. The order may shift with demand, contributions or changes in the ecosystem. Each milestone is an issue that holds its tasks and exit condition.

| Milestone                   | Theme                                                         | Issue |
| --------------------------- | ------------------------------------------------------------- | ----- |
| Governance                  | `CODEOWNERS` and branch rulesets generated from policy        | [#40](https://github.com/CameronBrooks11/agent-policy/issues/40) |
| Enforcement integration     | Claude Code hooks target, `audit` command, CI recipes         | [#41](https://github.com/CameronBrooks11/agent-policy/issues/41) |
| Stability declaration (1.0) | Schema freeze, complete docs, upgrade guide, `cargo audit`    | [#42](https://github.com/CameronBrooks11/agent-policy/issues/42) |

The npm and pip wrappers that were listed here shipped in v0.6 and v0.7; see Completed.

---

## What to Defer

| Item                              | Reason                                                                                                      |
| --------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| MCP server (`agent-policy serve`) | MCP spec still churning; only justified once spec is stable and real agent tooling uses it reliably         |
| Community template registry       | Needs a stable, matured target model first                                                                  |
| Profile system / `--profile` flag | Only justified if target count + workflow complexity outgrows boolean output flags; reassess closer to 1.0  |

---

## Key Risks

**Schema churn kills trust.** Every breaking change without a migration path erodes the tool's core credibility. All new schema fields must be additive. The `schema_version` field is load-bearing for 1.0 credibility.

**Agent ecosystem is a moving target.** Cursor's `.mdc` format, Claude hooks, Copilot instructions — all changed significantly in 2025. Targets must be marked `stable` or `experimental` with documented update policies.

**Golden test maintenance scales linearly.** 10 targets × 3+ example configs = 30+ snapshots. Consider generating snapshot fixtures programmatically or a lighter structural test layer before the test surface becomes a bottleneck.
