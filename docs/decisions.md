# Architectural Decisions

This document records the reasoning behind key technical decisions made during the design and implementation of `agent-policy`. It exists so that future contributors can understand _why_ certain choices were made and avoid re-litigating settled questions without new information.

---

## Implementation Language: Rust

Three languages were seriously evaluated before Rust was chosen.

### Python (rejected)

Strong case for it: fastest MVP iteration, rich YAML/JSON Schema tooling (`pyyaml`, `jsonschema`, `jinja2`), easy CLI packaging. Also the initial recommendation in early exploration.

Rejected because: weak single-binary distribution story — consuming repos need a Python environment; harder to embed or wrap from other languages later. The specific variant explored — "start in Python, prove the schema, then reimplement in Rust once stable" — was also rejected because the rewrite cost would arrive at exactly the wrong time: when the tool is in active internal use and the schema is stabilizing.

### TypeScript / Node (rejected)

Strong case for it: easy npm distribution, natural for frontend teams, strong YAML/CLI ecosystem.

Rejected because: Node becomes an implicit platform dependency for consuming repos that run `check` in CI; less clean as a language-neutral core if Python/Rust users want to treat the tool as neutral infrastructure.

### Rust (chosen)

Single static binary with zero runtime dependencies for consuming repos. Strong CLI ecosystem (`clap`, `serde`, `minijinja`). Ideal architecture for a "core compiler + thin language wrappers" model if Python or Node adapters are ever needed — those would be thin wrappers around the same Rust binary, not reimplementations. The schema and policy model are language-agnostic regardless of implementation language; the Rust binary is the first implementation, not the permanent constraint.

---

## Canonical Format: YAML + JSON Schema (not Markdown, not a custom DSL)

### Markdown as source of truth (rejected)

An early framing considered whether the canonical policy could live in a specially structured `AGENTS.md` that served as both source and output. Rejected because: Markdown is not reliably machine-parseable; diffing policy intent in Markdown PRs is harder than diffing YAML; it cannot serve as a validation contract the way JSON Schema can; it conflates the authoring format with the output format.

### One universal Markdown file (rejected)

A related idea: generate a single universal Markdown file that all agent tools would read, replacing `AGENTS.md`, `CLAUDE.md`, and `.cursor/rules` with one file. Rejected because each tool already expects a specific file at a specific location with a specific structure. Trying to serve all tools from one file means either lowest-common-denominator content or tool-specific sections that defeat the purpose. The correct model is target-specific renderers, not a universal output format.

### Custom DSL / policy language (rejected, locked)

A custom policy language was considered at various points. Rejected because: YAML + JSON Schema provides portability, validation, editor support, and compatibility with nearly every language; a custom DSL requires a parser, a language spec, editor extensions, and ongoing maintenance; no sufficiently strong reason exists to justify that cost. **This decision is locked.** The history of custom policy DSLs in infrastructure tooling is a cautionary tale about unnecessary language inventions.

---

## Schema Generation: Hand-authored (not schemars)

The `schemars` Rust crate was considered for auto-generating the JSON Schema from Rust struct definitions. Rejected because: the JSON Schema is the stable, language-agnostic contract and should drive the types, not be derived from them. Auto-generating it from Rust types inverts the dependency and makes the schema harder to evolve independently. The decision: hand-authored `agent-policy.schema.json`. `schemars` may be revisited as a consistency-check tool, but never as the schema source of truth.

---

## Templating: minijinja (not handlebars)

Both were evaluated for the renderer layer. `minijinja` was chosen because:

- Rust-native Jinja2 implementation with a well-maintained API
- More expressive template logic (conditionals, loops, filters) than Handlebars
- Jinja2 syntax is widely known and readable for developers coming from Python tooling

`handlebars` remains a valid fallback if `minijinja` presents issues in the future.

---

## Replacing AGENTS.md: No

An early framing touched on whether `agent-policy` could become an alternative standard that replaces `AGENTS.md` entirely. Explicitly rejected:

1. Tools already expect `AGENTS.md` — a replacement breaks existing compatibility
2. `AGENTS.md` is becoming a de facto discovery standard with real cross-tool momentum
3. The Linux Foundation's Agentic AI Foundation now stewards AGENTS.md as part of open agent infrastructure
4. Emitting `AGENTS.md` is strictly better than competing with it

`agent-policy` generates `AGENTS.md`. It does not eliminate or replace it.

---

## `CODEOWNERS` and Governance: Generate only, no GitHub API automation

Generating `CODEOWNERS` and branch ruleset configuration files is planned (Phase 5). Automating their application via the GitHub API (calling REST/GraphQL to set rules programmatically) is a different concern and is explicitly deferred. The tool generates artifacts; humans or separate automation apply them.

---

## `outputs` Field: Array of IDs (not a boolean map per target)

A boolean map per target (`outputs: { agents_md: true, claude_md: false }`) was considered. The chosen array design (`outputs: [agents-md, claude-md]`) was preferred because: adding a new target requires only a new `enum` value in the JSON Schema; no structural migration is needed; the array approach avoids one-key-per-target schema sprawl. This keeps the migration path clean as the target surface grows.

---

## Consuming Repo Layout: Root-level (not a policy/ subdirectory)

One early architecture exploration suggested consuming repos structure their policy under a `policy/` subdirectory with generated files committed under `generated/`. Rejected because: most repos will not have this structure; tools like Codex and Cursor expect their files at specific root-relative paths; the additional directory level adds indirection with no practical benefit. The convention: `agent-policy.yaml` at the repo root, generated files committed to their natural locations (`AGENTS.md` at root, `.cursor/rules/` at root, etc.).

---

## License: Apache-2.0 (not MIT)

Both MIT and Apache-2.0 were considered. Apache-2.0 was chosen for its explicit patent grant and its alignment with the type of infrastructure tooling this project represents. Either would have been acceptable.

---

## Features Explicitly Deferred or Ruled Out

The following were discussed and placed out of scope for the foreseeable future.

| Feature                              | Status            | Reasoning                                                                                                                       |
| ------------------------------------ | ----------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| SaaS platform / hosted service       | Out of scope      | Policy files are local and version-controlled; a hosted service adds infrastructure cost and attack surface with no benefit     |
| Background daemon                    | Out of scope      | The tool runs on demand and in CI; no persistent process is needed                                                              |
| GUI                                  | Out of scope      | The tool's users are developers; CLI + CI is the correct interface                                                              |
| Plugin / extension system            | Deferred to v0.8+ | Premature plugin systems are a common way to make tools unmaintainable; deferred until target surface is stable                 |
| Deep GitHub API automation           | Deferred          | Generating governance artifacts is planned; applying them via GitHub API is a separate concern                                  |
| MCP server at MVP                    | Deferred          | MCP consumption patterns in real agent tooling were not stable enough at the time of initial design to justify building against |
| Policy language beyond YAML + schema | Out of scope      | The MVP schema should "feel boring"; expressiveness comes later, if at all                                                      |
| `--profile` / profile system         | Deferred          | Only justified if target count and user workflow complexity outgrow boolean output flags; reassess when needed                  |

---

## The Enforcement Conflation Risk

One failure mode identified in early analysis: building a tool that only generates documentation-style files and calling it enforcement (a "weak-guarantee trap"). The distinction drawn and preserved in the architecture:

- `agent-policy` generates **context files** (AGENTS.md, CLAUDE.md, cursor rules) and **governance artifacts** (CODEOWNERS, hook configs, planned)
- Real enforcement still lives in GitHub rulesets, branch protections, environment approvals, and Claude Code hooks
- Conflating these two things leads to false confidence

This is why `docs/vision.md` explicitly separates the Enforcement Layer as external and why the README states this boundary clearly.
