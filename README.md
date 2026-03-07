# agent-policy

Schema-first generator for coding-agent repo policies and compatibility files.

[![crates.io](https://img.shields.io/crates/v/agent-policy)](https://crates.io/crates/agent-policy)
[![docs.rs](https://img.shields.io/docsrs/agent-policy)](https://docs.rs/agent-policy)
[![CI](https://github.com/CameronBrooks11/agent-policy/actions/workflows/ci.yml/badge.svg)](https://github.com/CameronBrooks11/agent-policy/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

## What it does

Maintaining `AGENTS.md`, `CLAUDE.md`, and Cursor rules separately means the same policy information gets duplicated across files that diverge over time. `agent-policy` solves this by letting you define your repo's coding-agent policy once in a canonical `agent-policy.yaml`, then generating all the tool-specific compatibility files from that single source of truth.

## Quick start

> _Coming in Phase 2 — `agent-policy init` and `agent-policy generate`_

## Commands

| Command | Description |
|---|---|
| `agent-policy init` | Scaffold a starter `agent-policy.yaml` in the current directory |
| `agent-policy generate` | Generate all output files from `agent-policy.yaml` |
| `agent-policy check` | Verify committed files are in sync with the current policy |

> _Fully implemented in Phases 2–3_

## Schema

`agent-policy.yaml` is validated against a bundled JSON Schema (`agent-policy.schema.json`). Full schema documentation coming in Phase 1.

## Non-goals

- Not a replacement for `AGENTS.md` — it _generates_ `AGENTS.md`
- Not a full enforcement engine or policy-as-code runtime
- Not a SaaS platform or hosted service
- Not a daemon or background process

## License

Apache-2.0
