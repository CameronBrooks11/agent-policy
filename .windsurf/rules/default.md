---
description: "agent-policy — Agent Policy Rules"
trigger: always_on
---

# agent-policy — Agent Policy

Schema-first generator for coding-agent repo policies and compatibility files.

## Commands

- Test: `cargo test`
- Lint: `cargo clippy --all-targets -- -D warnings`

## Edit permissions

You may freely edit:
- `src/**`
- `templates/**`
- `tests/**`
- `examples/**`

Do not modify without human review:
- `.github/workflows/**`
- `agent-policy.schema.json`
- `Cargo.toml`

## Constraints
- Never commit secrets or credentials.- Include tests with code changes.