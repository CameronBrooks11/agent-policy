# Vision: agent-policy

## One-Line Statement

`agent-policy` is a schema-first policy engine for agentic development workflows — a single canonical source of truth that generates, validates, and enforces coding-agent instructions and guardrails across a fragmented ecosystem of agent tools.

---

## The Problem

The modern agentic development ecosystem is fragmented by convention. Every major coding agent tool expects its own configuration format in its own location:

| Tool                          | File                   |
| ----------------------------- | ---------------------- |
| OpenAI Codex / GitHub Copilot | `AGENTS.md`            |
| Claude Code                   | `CLAUDE.md`            |
| Cursor                        | `.cursor/rules/*.mdc`  |
| GitHub repo governance        | `CODEOWNERS`, rulesets |
| CI                            | policy check workflows |

Despite living in different places with different syntax, these files all express the **same underlying policy**:

- What agents are allowed to modify
- What paths are protected
- What commands to use
- What constraints must be satisfied
- Who or what must review sensitive changes

Maintaining these files by hand leads to configuration drift, inconsistency, and fragile agent behavior. As agent tooling grows in capability and scope, handcrafted per-tool instruction files become a liability, not an asset.

---

## The Core Idea

**Treat every agent instruction file as a projection of a single underlying policy model.**

This mirrors the pattern used in mature infrastructure systems:

- Kubernetes: YAML manifests → controllers → resources
- Terraform: HCL → providers → infrastructure
- Bazel: BUILD files → toolchain actions
- OPA/Rego: policy files → enforcement decisions

`agent-policy` applies this same pattern to coding-agent governance:

```
agent-policy.yaml   ← canonical source of truth
       ↓ validate
agent-policy.schema.json   ← machine contract
       ↓ compile / render
AGENTS.md            ← OpenAI Codex, GitHub Copilot
CLAUDE.md            ← Anthropic Claude Code
.cursor/rules/*.mdc  ← Cursor
CODEOWNERS           ← GitHub repository governance
hook configs         ← Claude Code hooks
CI checks            ← policy drift detection
```

The generated files are **compatibility artifacts**. The canonical truth is the YAML policy and its JSON Schema contract. The schema is the real durable asset — the implementation can change; the schema must remain stable.

---

## Guiding Principles

### 1. Policy as code, not documentation

The source of truth is a structured, machine-readable, version-controlled YAML file — not prose. Policies should be diffable, reviewable in PRs, and auditable over time.

### 2. Generated files are output, not source

Every agent instruction file in a consuming repo is a generated artifact produced by `agent-policy generate`. Editing those files by hand defeats the purpose. The canonical edit point is always `agent-policy.yaml`.

### 3. Emit conventions, do not replace them

`agent-policy` does not invent a new universal agent instruction format. It emits `AGENTS.md` because Codex reads it. It emits `.cursor/rules` because Cursor reads it. The tool adapts to emerging conventions rather than competing with them.

### 4. Policy generation is not enforcement

Instruction files provide context to agents — they are not hard enforcement boundaries. Real enforcement lives in:

- GitHub branch rulesets
- `CODEOWNERS`
- CI policy checks
- Environment approvals
- Claude Code hooks

`agent-policy` generates inputs for those enforcement layers. It does not replace them.

### 5. Schema stability is paramount

The `agent-policy.schema.json` is the long-term contract. Everything else — the CLI, the renderer implementations, the output formats — is replaceable. A stable schema means the canonical truth survives tooling churn.

### 6. No custom DSL

YAML + JSON Schema provides a portable, language-neutral, editor-supported authoring experience. A custom policy language would increase maintenance burden with no meaningful gain at this stage.

### 7. Layered scope: roles and path-scoped policy

Modern repos involve multiple specialized agents — a docs agent, a frontend agent, an infra agent. Policy must express per-role access boundaries with path-scoped granularity as a first-class concern, not an afterthought.

---

## The Policy Model

The canonical `agent-policy.yaml` expresses policy across six top-level dimensions:

```yaml
project: # name, purpose, summary
commands: # install, dev, test, lint, build
paths: # editable, protected, generated
roles: # per-agent role scopes and restrictions
constraints: # behavioral guardrails
outputs: # which target files to generate
```

This schema is intentionally minimal. It should feel boring. Its value comes from consistency and stability, not expressiveness.

---

## The Architecture

The system is organized into clean, separable layers:

```
┌─────────────────────────────────────────────────┐
│ Authoring Layer                                 │
│   agent-policy.yaml  (human-edited)             │
└────────────────────────┬────────────────────────┘
                         │
┌────────────────────────▼────────────────────────┐
│ Validation Layer                                │
│   agent-policy.schema.json  (JSON Schema)       │
└────────────────────────┬────────────────────────┘
                         │
┌────────────────────────▼────────────────────────┐
│ Semantic Layer                                  │
│   Internal normalized policy model             │
│   (stable internal representation)             │
└────────────────────────┬────────────────────────┘
                         │
┌────────────────────────▼────────────────────────┐
│ Render Layer                                    │
│   Target-specific renderers                     │
│   AGENTS.md / CLAUDE.md / .cursor/rules / etc   │
└────────────────────────┬────────────────────────┘
                         │
┌────────────────────────▼────────────────────────┐
│ Enforcement Layer  (external systems)           │
│   CI drift checks / CODEOWNERS / rulesets /     │
│   GitHub enforcement / Claude hooks             │
└─────────────────────────────────────────────────┘
```

The normalization step — parsing YAML into a stable internal model before rendering — is critical. It decouples the YAML surface from renderer logic and makes future schema evolution safer.

---

## Implementation Direction

**Language: Rust**

The core implementation is a Rust CLI binary. This choice is driven by:

- Single static binary, zero runtime dependency for consuming repos
- Easy cross-platform distribution
- Strong CLI ecosystem (`clap`, `serde`, `minijinja`, etc.)
- Ideal architecture for a "core compiler + thin language wrappers" model
- Long-term suitability as a portable, embeddable core if Python/Node adapters are ever needed

The schema and semantic model are **language-agnostic**. The Rust binary is the first implementation, not the permanent implementation. If the project grows to need a Python SDK or an npm package, those are thin wrappers around the same schema and the same canonical model — they do not redefine it.

---

## Long-Term Destination

The full long-term value of `agent-policy` is **policy-as-code for the entire agentic development lifecycle**, not just instruction file generation.

This means the tool eventually covers:

1. **Agent instruction files** (MVP) — AGENTS.md, CLAUDE.md, Cursor rules
2. **Repository governance** — CODEOWNERS, GitHub branch rulesets, environment approvals
3. **CI enforcement** — policy drift detection, schema validation gates
4. **Hook configuration** — Claude Code hook templates, pre/post execution policies
5. **MCP policy resource** — exposing live policy as a structured, machine-queryable MCP resource that agents can introspect at runtime
6. **Multi-language distribution** — npm package, PyPI package, Cargo crate as thin wrappers around the core

The progression across roadmap phases is:

```
Generation    →  file generation (compatibility artifacts)
Governance    →  repo governance generation (CODEOWNERS, ruleset automation)
Enforcement   →  enforcement integration (CI, hooks, approval gates)
Live Policy   →  live policy exposure (MCP resource, policy queries)
Ecosystem     →  language wrappers, community templates, agent role libraries
```

Each phase delivers standalone value. Each phase leaves the door open for the next.

---

## What agent-policy Is Not

- Not a replacement for `AGENTS.md` — it generates `AGENTS.md`
- Not a full enforcement engine — it generates inputs for enforcement systems
- Not a SaaS platform or hosted service — it is a local CLI tool committed to the repo
- Not a custom policy DSL — it uses YAML + JSON Schema
- Not a daemon or background service — it runs in CI and on demand
- Not a plugin system (yet) — targets are built into the renderer layer until the model matures

---

## Why Now

The agentic development ecosystem is at an inflection point. `AGENTS.md` has real cross-tool momentum. The Linux Foundation's Agentic AI Foundation now stewards open standards that include both MCP and AGENTS.md. MCP itself uses JSON Schema as a core validation mechanism — a direct signal that structured, schema-validated policy is the direction the ecosystem is moving.

The risk of waiting is fragmentation lock-in: more tools, more hand-maintained instruction files, more drift. The opportunity of acting now is establishing a single durable schema and generation model before the ecosystem ossifies around today's ad hoc conventions.

The architecture is aligned with where agent tooling is heading. The schema matters more than the first implementation. The time to define that schema is now.
