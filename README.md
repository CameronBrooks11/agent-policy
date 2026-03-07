# agent-policy

Schema-first generator for coding-agent repo policies and compatibility files.

**Generates `AGENTS.md`, `CLAUDE.md`, and `.cursor/rules/` from a single `agent-policy.yaml`.**

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
| `agent-policy generate` | Generate all enabled output files from `agent-policy.yaml` |
| `agent-policy check` | Verify committed files match the current policy (CI use) |

Both `generate` and `check` accept `--config` / `-c` to specify a non-default config path:

```bash
agent-policy generate --config infra/agent-policy.yaml
agent-policy check -c path/to/agent-policy.yaml
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

Full example — see [`examples/website/agent-policy.yaml`](examples/website/agent-policy.yaml).

The file is validated against [`agent-policy.schema.json`](agent-policy.schema.json) (JSON Schema Draft 2020-12). The schema is bundled with the binary and is versioned alongside it. It is considered unstable until v1.0.0.

## Generated outputs

| Target ID | File | Enabled by default |
|---|---|---|
| `agents-md` | `AGENTS.md` | Yes |
| `claude-md` | `CLAUDE.md` | No |
| `cursor-rules` | `.cursor/rules/default.mdc` + one per role | No |

When `cursor-rules` is enabled and roles are defined, `agent-policy` generates one `.mdc` file per role with `globs` set to the role's editable paths, so Cursor activates the rule automatically based on which file is open.

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
