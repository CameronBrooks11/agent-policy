# agent-policy — Junie Guidelines

Schema-first generator for coding-agent repo policies and compatibility files.

## Commands

- Test: `cargo test`
- Lint: `cargo clippy --all-targets -- -D warnings`
- Build: `cargo build --release`

## Directories & Permissions

You may freely edit:
- `src/**`
- `templates/**`
- `tests/**`
- `examples/**`

Do not modify without human review:
- `.github/workflows/**`
- `agent-policy.schema.json`
- `Cargo.toml`

## Roles

No roles defined.

## Rules & Anti-patterns
- Never commit secrets or credentials.- Include tests with code changes.