# Archive: Considered and Decided-Against Directions

This document preserves the reasoning trail from the initial exploration phase — options considered, directions explored, and decisions made against certain approaches. It is not a to-do list or active design document. It exists so that future contributors can understand _why_ certain choices were made and avoid re-litigating settled questions without new information.

For current direction see [docs/vision.md](../docs/vision.md) and the [planning/roadmap.md](roadmap.md).

---

## Implementation Language — The Three-Way Debate

Before settling on Rust, three languages were seriously evaluated.

### Option A — Python first

**The case for it:**

- Fastest MVP iteration speed
- Rich YAML/JSON Schema tooling out of the box (`pyyaml`, `jsonschema`, `jinja2`)
- Easy CLI packaging (`click`, `typer`)
- Good ergonomics for an internal tool

**The case against it:**

- Weak "single static binary" distribution story — consuming repos need a Python environment
- Less portable as a long-term core
- Harder to embed or wrap from Rust/Node later if the core is Python

**The middle path explored:** Start in Python, prove the schema model, then reimplement the core in Rust once stable. This was a genuinely reasonable path and was the recommendation in early exploration. It was ultimately decided against because the schema is expected to be foundational infrastructure, and Rust from the start avoids a costly rewrite while the project is already in active use.

### Option B — TypeScript / Node first

**The case for it:**

- Easy npm distribution
- Natural fit for frontend and devtool-heavy teams
- Strong ecosystem for YAML, templating, CLIs
- Familiar to many developers

**The case against it:**

- Node becomes an implicit platform dependency for any consuming repo that runs `check` in CI
- Weaker as a universal low-level core than Rust
- Less clean if the goal is a language-neutral schema and adapters — Node is not neutral
- If Python/Rust users later want to treat this as neutral infra, a Node core is friction

**Verdict:** Rejected. Not recommended unless the team is deeply Node-centric.

### Option C — Rust from the start (chosen)

Rust was ultimately chosen for all the reasons documented in [docs/vision.md](../docs/vision.md). This decision was not unanimous in the exploration — early analysis leaned Python for speed — but solidified once the "foundational infrastructure" framing was locked in. The key insight: the schema is the durable asset, and Rust as a static binary does not add runtime dependencies to consuming repos.

---

## The "Python first, Rust later" Phased Approach

This specific strategy was explicitly considered and explicitly rejected:

1. Build Python MVP
2. Stabilize schema through real use
3. Reimplement in Rust once model is proven

**Why it was rejected:** The rewrite cost arrives at exactly the wrong time — when the tool is in active internal use and the schema is stabilizing. A Rust-first approach has more upfront cost but avoids disrupting the adoption curve. The schema + JSON Schema contract is language-agnostic regardless; the implementation language does not affect schema longevity.

---

## Markdown as the Source of Truth

An early framing considered whether the canonical policy should live in Markdown (i.e., a specially structured `AGENTS.md` acting as both source and output).

**Why it was rejected:**

- Markdown is not reliably machine-parseable
- Structured data in Markdown requires fragile conventions or embedded YAML/TOML blocks
- It conflates the authoring format with the output format
- Diffing policy intent in Markdown PRs is harder than diffing YAML
- It cannot serve as a validation contract the way JSON Schema can

**Verdict:** The canonical format is YAML validated by JSON Schema. Markdown files are output artifacts only.

---

## "One Universal Markdown File" Approach

A related idea: generate a single universal Markdown file that all agent tools could read, replacing `AGENTS.md`, `CLAUDE.md`, and `.cursor/rules` with one file.

**Why it was rejected:**

- Each tool already expects a specific file in a specific location with a specific structure
- Trying to serve all tools from one file means either lowest-common-denominator content or tool-specific sections that defeat the purpose
- `AGENTS.md` has genuine cross-tool momentum — OpenAI Codex explicitly reads it before work; it should not be competed with
- The correct model is target-specific renderers, not one universal output

**Verdict:** Emit the files each tool expects. Do not try to invent a new universal format.

---

## Replacing `AGENTS.md` as a Goal

Early framing touched on whether `agent-policy` could become an alternative standard that replaces `AGENTS.md` entirely.

**Why this was explicitly rejected:**

1. Tools already expect `AGENTS.md` — replacing it breaks existing compatibility
2. `AGENTS.md` is becoming a de facto discovery standard with real cross-tool momentum
3. The Linux Foundation's Agentic AI Foundation now stewards AGENTS.md as part of open agent infrastructure
4. Fighting existing conventions is expensive and unnecessary
5. Emitting `AGENTS.md` is strictly better than competing with it

**Verdict:** `agent-policy` generates `AGENTS.md`. It does not eliminate or replace it.

---

## Custom DSL / Policy Language

At various points, a custom policy language was considered — something more expressive than YAML that would read like code or have domain-specific syntax.

**Why it was rejected:**

- YAML + JSON Schema already provides portability, validation, editor support, and compatibility with nearly every language
- A custom DSL requires a parser, a language spec, editor extensions, and ongoing maintenance
- No sufficiently strong reason exists to justify that cost at this stage
- The history of custom policy DSLs in infra tooling (see: early CFEngine, early Puppet) is a cautionary tale about unnecessary language inventions

**Verdict:** YAML + JSON Schema. No custom DSL. This decision is locked.

---

## `schemars` for Schema Generation

The `schemars` Rust crate was considered for generating the JSON Schema automatically from Rust struct definitions.

**Why it was decided against (for now):**

- The JSON Schema is the stable, language-agnostic contract
- Auto-generating it from Rust types inverts the dependency — the schema should drive the types, not the other way around
- Coupling the schema to Rust struct representations makes the schema harder to evolve independently
- Hand-authored JSON Schema is more deliberate and reviewable

**Verdict:** Hand-authored `agent-policy.schema.json`. `schemars` may be revisited in future as a consistency-check tool, but not as the schema source of truth.

---

## `handlebars` vs `minijinja` for Templating

Both were explicitly evaluated for the renderer layer.

**Why `minijinja` was chosen over `handlebars`:**

- `minijinja` is a Rust-native Jinja2 implementation with a well-maintained API
- Jinja2 syntax is more expressive for template logic (conditionals, loops, filters) than Handlebars
- The Jinja2 syntax is widely known and readable for developers coming from Python tooling
- `handlebars` is a reasonable alternative but offers less template expressiveness

**Verdict:** `minijinja`. Handlebars remains a valid fallback if `minijinja` presents issues.

---

## Repo Name Alternatives

Before settling on `agent-policy`, two alternatives were considered:

| Name               | Assessment                                                                           |
| ------------------ | ------------------------------------------------------------------------------------ |
| `agent-policy-cli` | Redundant suffix; the "CLI" is obvious from context                                  |
| `policygen`        | Clean and memorable, but too generic — doesn't communicate the agent-specific domain |
| `agent-policy`     | Cleanest. Communicates domain (agent) and function (policy) without redundancy.      |

**Verdict:** `agent-policy`.

---

## License — MIT vs Apache-2.0

Both were considered for the initial license:

| License    | Notes                                                                                                    |
| ---------- | -------------------------------------------------------------------------------------------------------- |
| MIT        | Simplest open-source license; easiest for downstream adoption                                            |
| Apache-2.0 | Includes explicit patent grant; slightly more formal open-source posture; common in infrastructure tools |

**Verdict:** Apache-2.0 was chosen for the explicit patent grant and its alignment with the type of infrastructure tooling this project represents. Either would have been acceptable.

---

## Consuming Repo Layout Alternative

One early architecture exploration suggested that consuming repos structure their policy under a `policy/` subdirectory:

```text
repo/
  policy/
    policy.yaml
    schema.json
  tools/
    policygen/
  generated/
    AGENTS.md
    CLAUDE.md
    cursor_rules/
```

**Why this was simplified:**

- Most repos will not have a `tools/policygen/` directory — the tool is a separate CLI binary
- The `generated/` subdirectory adds a non-standard level of indirection; tools like Codex and Cursor expect their files at specific root-relative paths
- The simpler model — `agent-policy.yaml` at the repo root, generated files committed at their expected paths — is more compatible and less opinionated about repo structure

**Verdict:** `agent-policy.yaml` at the repo root. Generated files committed to their natural locations (`AGENTS.md` at root, `.cursor/rules/` at root level, etc.).

---

## Rust Edition

A minor but noted consideration: `edition = "2024"` vs `edition = "2021"` for `Cargo.toml`.

**Notes:** Use `edition = "2024"` if the toolchain supports it stably; otherwise `edition = "2021"`. This has no architectural significance and should be updated to the current stable edition at bootstrap time.

---

## Early-Stage Features Explicitly Deferred or Rejected

The following were discussed during exploration and explicitly placed out of scope for the foreseeable future. They are recorded here to prevent scope creep.

### SaaS platform / hosted service

At no point does `agent-policy` become a service. Policy files are local, version-controlled, and CI-enforced. A hosted service adds infrastructure cost and attack surface with no benefit over a committed binary + CI.

### Background daemon / runtime service

`agent-policy` runs on demand (`generate`, `check`) and in CI. There is no scenario in the current architecture where a persistent background process is needed. Rejected.

### GUI

A graphical interface was briefly mentioned and immediately dismissed. The tool's users are developers working in repos. CLI + CI is the correct interface.

### Plugin / extension system

A plugin system for community-contributed renderers was floated. **Deferred** to Phase 8 (Ecosystem) at the earliest, and only as a template system, not a general plugin architecture. Premature plugin systems are a common way to make tools unmaintainable.

### Deep GitHub API automation

Generating `CODEOWNERS` and branch ruleset configuration files is planned. Automating their application via GitHub API (i.e., calling GitHub's REST/GraphQL API to set rules programmatically) is a different concern and was explicitly deferred. The tool generates the artifacts; humans or separate automation apply them.

### MCP server at MVP

An MCP resource/server was discussed as a future direction. It was explicitly ruled out for v0.1 because MCP's consumption patterns in real agent tooling were not stable enough at the time of initial design to justify building against. Planned for Phase 7 only after the core model is stable and MCP adoption in tools has matured.

### Policy language beyond YAML + schema

Any form of a richer policy expression layer (e.g., rule inheritance, conditional policy, policy composition) was deferred. The MVP schema should "feel boring." Expressiveness comes later, if at all.

---

## The "Enforcement Conflation" Risk

One failure mode identified in early analysis: building a tool that only generates documentation-style files and calling it enforcement. This was identified as a weak-guarantee trap.

The distinction drawn and preserved in the architecture:

- `agent-policy` generates **context files** (AGENTS.md, CLAUDE.md, cursor rules) and **governance artifacts** (CODEOWNERS, hook configs)
- Real enforcement still lives in GitHub rulesets, branch protections, environment approvals, and Claude Code hooks
- Conflating these two things leads to false confidence

This is why [docs/vision.md](../docs/vision.md) explicitly separates the Enforcement Layer as external and why the README must state this boundary clearly.

---

## The Over-Engineering Trap

Identified as the largest single risk to the project: attempting to build a full policy engine immediately.

The concrete antipatterns flagged:

- Modeling every conceivable policy concept in v0.1
- Building policy inheritance or composition before the base model is proven
- Adding a hosted service before local tooling is stable
- Adding a plugin system before targets are stable
- Designing for the MCP integration before the file generation is mature

The corrective principle embedded in the roadmap: each phase must deliver standalone value and exit on a concrete condition before the next phase begins.
