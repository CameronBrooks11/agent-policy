# agent-policy

Schema-first generator for coding-agent repo policies and compatibility files.

**Generates `AGENTS.md`, `CLAUDE.md`, `.cursor/rules/`, and `.windsurf/rules/` (among others) from a single `agent-policy.yaml`.**

[![crates.io](https://img.shields.io/crates/v/agent-policy)](https://crates.io/crates/agent-policy)
[![docs.rs](https://img.shields.io/docsrs/agent-policy)](https://docs.rs/agent-policy)
[![Docs site](https://img.shields.io/badge/docs-site-blue)](https://cameronbrooks11.github.io/agent-policy/)
[![CI](https://github.com/CameronBrooks11/agent-policy/actions/workflows/ci.yml/badge.svg)](https://github.com/CameronBrooks11/agent-policy/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

## What it does

Maintaining `AGENTS.md`, `CLAUDE.md`, and Cursor rules separately means the same policy information gets duplicated across files that diverge over time. `agent-policy` solves this by letting you define your repo's coding-agent policy once in a canonical `agent-policy.yaml`, then generating all the tool-specific compatibility files from that single source of truth.

Keep one policy file. Generate all agent instruction files from it. Detect drift in CI.

## Install

### From crates.io

```bash
cargo install agent-policy
```

### Binary releases

Download pre-built binaries from [GitHub Releases](https://github.com/CameronBrooks11/agent-policy/releases).

## Quick start

```bash
# Create a starter policy
agent-policy init

# Edit agent-policy.yaml, then generate
agent-policy generate

# Add to CI to prevent drift
agent-policy check
```

## Commands

| Command | Description |
|---|---|
| `agent-policy init` | Write a starter `agent-policy.yaml` in the current directory |
| `agent-policy list-targets` | View a table of all available export formats and their stability |
| `agent-policy generate` | Generate all enabled output files from `agent-policy.yaml` |
| `agent-policy lint` | Analyze `agent-policy.yaml` for semantic errors and warnings |
| `agent-policy check` | Verify committed files match the current policy (CI use) |

Both `generate` and `check` accept `--config` / `-c` to specify a non-default config path, and `--targets` to natively override outputs:

```bash
agent-policy generate --config infra/agent-policy.yaml
agent-policy check --targets agents-md,clinerules
```

## agent-policy.yaml

Minimal example:

```yaml
project:
  name: my-project
  summary: A short description.
outputs:
  - agents-md
```

*Note: You do not need to manually add `.md` outputs or target paths to your `paths.generated` config list. Any active `outputs` are injected into your generated list automatically!*

Full example — see [`examples/website/agent-policy.yaml`](examples/website/agent-policy.yaml).

The file is validated against [`agent-policy.schema.json`](agent-policy.schema.json) (JSON Schema Draft 2020-12). The schema is bundled with the binary and is versioned alongside it. It is considered unstable until v1.0.0.

## Generated outputs

| Target ID | File | Enabled by default | Tool(s) |
|---|---|---|---|
| `agents-md` | `AGENTS.md` | Yes | Codex, Windsurf, Copilot |
| `claude-md` | `CLAUDE.md` | No | Claude Code, Copilot |
| `cursor-rules` | `.cursor/rules/*.mdc` | No | Cursor |
| `gemini-md` | `GEMINI.md` | No | Google Gemini CLI |
| `windsurf-rules` | `.windsurf/rules/*.md` | No | Codeium Windsurf |
| `clinerules` | `.clinerules/*.md` | No | Cline |
| `copilot-instructions` | `.github/copilot-instructions.md` | No | Copilot (Global) |
| `copilot-instructions-scoped` | `.github/instructions/*.md` | No | Copilot (Scoped Context) |
| `junie-guidelines` | `.junie/guidelines.md` | No | JetBrains Junie |

When outputting directory rules like `cursor-rules` or `clinerules`, `agent-policy` generates one default layout and optionally one file per customized role, intelligently assigning frontmatter globs and path references directly into the generated configurations to cleanly silo AI behaviors scope-to-scope!

## CI integration

Add to your CI workflow to enforce that generated files are never edited by hand:

```yaml
- name: Check agent policy
  run: agent-policy check
```

This step exits non-zero if any generated file is stale or missing, and prints a unified diff to stderr.

## Non-goals

- Not a replacement for `AGENTS.md` — it _generates_ `AGENTS.md`
- Not a full enforcement engine or policy-as-code runtime
- Not a SaaS platform or hosted service
- Not a daemon or background process

## License

Apache-2.0
