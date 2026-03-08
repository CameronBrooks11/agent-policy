# Roadmap: agent-policy

> Last updated: 2026-03-07 — post v0.5.0 release.
> For architectural vision and design principles, see [vision.md](vision.md).

---

## Current State

v0.5.0 ships a complete adoption-ready feature set:

- Full target coverage across all major agent tools
- `import`, `lint`, `install-hooks` commands for migration and enforcement
- Monorepo support via `--config` relative path resolution
- Self-dogfooding, golden tests, CI drift detection, crates.io releases

The install story still requires `cargo install agent-policy`, which is a blocker for the majority of potential users (JS/TS and Python shops). That is the next priority.

---

## Completed

| Milestone | Theme                 | Key deliverable                                                        | Status  |
| --------- | --------------------- | ---------------------------------------------------------------------- | ------- |
| v0.1      | Proof of concept      | Core pipeline, 3 targets, crates.io, self-dogfooding                   | ✅ Done |
| v0.2      | Target breadth        | GEMINI.md, Copilot instructions, `list-targets`, `schema_version`      | ✅ Done |
| v0.3      | Full target coverage  | Windsurf, Cline, Junie, `--targets` flag, stability tiers              | ✅ Done |
| v0.4      | Migration + linting   | `lint` command — semantic path conflict and scope validation            | ✅ Done |
| v0.5      | Migration (continued) | `import`, `install-hooks`, monorepo `--config` path resolution         | ✅ Done |

---

## Path Forward

The items below are listed roughly in priority order but are not version-locked. Ordering may shift based on demand, contributions, or ecosystem changes.

---

### npm wrapper

Removes the Rust toolchain requirement for the largest segment of potential users. JS/TS monorepos are this tool's primary natural habitat.

#### Tasks

- [ ] Publish `agent-policy` to npm as a thin JS wrapper that downloads and runs the correct prebuilt binary from GitHub releases
- [ ] Platform packages (`agent-policy-linux-x64`, `agent-policy-darwin-arm64`, `agent-policy-win32-x64`) using `optionalDependencies`
- [ ] `npx agent-policy` works zero-install
- [ ] npm publish step added to release workflow

#### Exit condition

- `npx agent-policy generate` works on macOS, Linux, and Windows with no Rust toolchain installed
- Binary resolution is deterministic and verified in CI

---

### pip wrapper

Covers Python-first teams and CI environments where pip is the standard install mechanism.

#### Tasks

- [ ] Publish `agent-policy` to PyPI as a package that downloads and runs the correct prebuilt binary
- [ ] `pip install agent-policy` + `agent-policy generate` works with no Rust toolchain
- [ ] pip publish step added to release workflow

#### Exit condition

- `pip install agent-policy && agent-policy generate` works on macOS, Linux, and Windows
- Works in a clean virtualenv with no Rust toolchain present

---

### Governance

Enterprise and team adoption driver. Teams using code review workflows need generated governance artifacts to trust the policy model.

#### Tasks

- [ ] Add `governance` section to schema (additive):
  ```yaml
  governance:
    codeowners: true   # derive CODEOWNERS from roles + protected paths
    branch_protection: false  # optional GitHub ruleset JSON export
  ```
- [ ] `CODEOWNERS` renderer — derive ownership from `roles` + `paths.protected`
- [ ] Add `codeowners` target to valid `outputs` values
- [ ] Optional GitHub branch ruleset JSON export
- [ ] Governance golden tests
- [ ] At least one real repo using the generated `CODEOWNERS`

#### Exit condition

- `agent-policy generate` can emit a `CODEOWNERS` consistent with declared role/path policy
- `agent-policy check` detects drift in the generated `CODEOWNERS`

---

### Enforcement integration

Closes the "policy as code, not documentation" loop. Without enforcement hooks, the tool generates instructions but has no way to verify agent behavior aligns with policy at runtime.

#### Tasks

- [ ] Claude Code hook config template generation (`claude_hooks` target)
- [ ] `agent-policy audit` command — reports which constraints have no downstream enforcement mechanism
- [ ] `agent-policy lint` mature — semantic conflict detection, actionable error messages
- [ ] CI recipe documentation: GitHub Actions and GitLab CI patterns for policy enforcement
- [ ] Research current Claude Code hooks API surface before implementation

#### Exit condition

- `agent-policy generate` emits a Claude Code hook config template
- `audit` command produces actionable output for at least the `forbid_secrets` and `require_tests_for_code_changes` constraints

---

### Stability declaration (v1.0)

Not a feature release — a commitment release. The goal is formal declaration that the schema is stable and the tool is safe to depend on without surprises.

#### Tasks

- [ ] Schema `v1` formally frozen: documented compatibility guarantee, deprecation policy
- [ ] All targets designated `stable` or `experimental` with documented update criteria
- [ ] `contributing.md`, code of conduct, issue and PR templates
- [ ] User guide, schema reference, cookbook, CI integration guide (complete documentation)
- [ ] CHANGELOG accurate from 0.1 forward
- [ ] Upgrade guide: 0.x → 1.0 migration notes
- [ ] `cargo audit` clean, MSRV pinned and tested in CI

#### Exit condition (the real 1.0 test)

1. A new user can install via `cargo install`, `npx`, or `pip install`, run `agent-policy import`, and have a working policy in under 5 minutes
2. Their CI runs `agent-policy check` and catches real drift
3. Upgrading from one minor version to the next never silently breaks their config
4. Their preferred agent tool has a supported, tested output target

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
