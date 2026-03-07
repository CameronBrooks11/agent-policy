# Cursor Rules

## Summary

Markdown (`.md`) or MDC (`.mdc`) files stored in `.cursor/rules/` that provide persistent, reusable instructions to Cursor's AI agent. Support four activation modes controlled via YAML frontmatter: always-on, agent-decided, file-glob-scoped, and manual. Version-controlled and intended to be committed with the project.

## Tool

Cursor (Anysphere). Used by Cursor Agent (Chat). Also applied in Inline Edit (Cmd/Ctrl+K) where applicable.

## Support level

`supported` — v0.1 Core target.

## Canonical path(s)

- `.cursor/rules/*.md` — plain Markdown rule files
- `.cursor/rules/*.mdc` — MDC rule files with YAML frontmatter for activation control
- Subdirectories supported: `.cursor/rules/frontend/components.md` etc.

The `.cursor/rules/` directory is version-controlled and checked into git.

## Alternate / legacy path(s)

- `.cursorrules` — legacy single-file format from earlier Cursor versions. Still recognized by Cursor but superseded by `.cursor/rules/`. New projects should use the directory-based approach.
- `AGENTS.md` at project root — a simpler, frontmatter-free alternative to `.cursor/rules/` accepted by Cursor for straightforward use cases. Cursor supports AGENTS.md in the project root and subdirectories.

## File format

Two supported extensions:

- **`.md`** — plain Markdown, no frontmatter. Rule is activated manually (via `@rule-name` mention).
- **`.mdc`** — Markdown with optional YAML frontmatter controlling activation mode. Preferred for any rule that should activate automatically.

Frontmatter fields (all optional):

| Field         | Type   | Purpose                                                                                         |
| ------------- | ------ | ----------------------------------------------------------------------------------------------- |
| `description` | string | Shown to Cursor Agent to decide relevance when `alwaysApply` is false                           |
| `globs`       | string | File pattern(s), comma-separated for multiple — rule applies when matching files are in context |
| `alwaysApply` | bool   | If `true`, included in every chat session regardless of other fields                            |

All three fields are optional. The combination of set/unset fields determines activation mode:

| Mode                    | `alwaysApply`   | `globs` | `description` | When applied                                           |
| ----------------------- | --------------- | ------- | ------------- | ------------------------------------------------------ |
| Always Apply            | `true`          | any     | any           | Every chat session                                     |
| Apply to Specific Files | —               | set     | —             | When file matches glob pattern                         |
| Apply Intelligently     | `false` / unset | unset   | set           | When agent decides it is relevant based on description |
| Apply Manually          | `false` / unset | unset   | unset         | Only when @-mentioned in chat                          |

## Frontmatter

YAML frontmatter block at the top of `.mdc` files. Example for a glob-scoped rule:

```yaml
---
description: "Standards for frontend components and API validation"
alwaysApply: false
---
```

Example for an always-on rule:

```yaml
---
alwaysApply: true
---
```

Example for a glob-scoped rule with no description:

```yaml
---
globs: src/api/**/*.ts
alwaysApply: false
---
```

`globs` is a string. Use comma-separated patterns for multiple globs (e.g. `src/**/*.ts,src/**/*.tsx`). Vendor examples only show string format; array format is not documented as supported.

## Discovery / scope behavior

- Cursor scans `.cursor/rules/` recursively at project load.
- Files can be organized into subdirectories: all `.md` and `.mdc` files at any depth are discovered.
- **Precedence order:** Team Rules (dashboard) → Project Rules (`.cursor/rules/`) → User Rules (Cursor Settings → Rules). All applicable rules are merged; earlier sources take precedence on conflicts.
- **User rules** are global preferences set in `Cursor Settings → Rules`, applying to all projects. Not file-based; not applicable to `agent-policy` generation.
- **Team rules** (Team/Enterprise plans) are administered via the Cursor dashboard. Not file-based; not applicable to `agent-policy` generation.

## Repo-safe

Yes. `.cursor/rules/` is explicitly designed to be committed to version control and shared with teammates.

## Renderer notes

- Output paths: `.cursor/rules/default.mdc` (global rule) plus `.cursor/rules/<role-name>.mdc` for each defined role (Phase 4 path-scoped rules).
- v0.1 generates only `default.mdc` with `alwaysApply: true`.
- Phase 4 adds per-role rules with `globs` frontmatter set from `role.editable` patterns.
- Each file is a `RenderedOutput` entry; `cursor_rules::render()` returns `Vec<RenderedOutput>`.
- `globs` for per-role rules: join `role.editable` with commas for a multi-pattern string (Cursor accepts comma-separated patterns in the `globs` field).
- Keep rule files under 500 lines (vendor best-practice recommendation).

## Known limitations / gotchas

- **`alwaysApply: true` consumes context every session:** Rules that are always-on are injected into every chat regardless of relevance. Use glob-scoped or agent-decided rules for large or specialized content.
- **`.cursorrules` is legacy:** Single-file `.cursorrules` in the project root is an older format. It still works but Cursor now recommends the `.cursor/rules/` directory structure. The `agent-policy` generator uses the directory structure only.
- **`globs` is a string, not a YAML array:** Vendor examples consistently show `globs` as a plain string. Use comma-separated values for multiple patterns. Confirm this is still the case before the Phase 4 per-role renderer is implemented.
- **Does not apply to Cursor Tab (autocomplete):** Rules apply to Agent (Chat) mode. Cursor Tab uses a separate mechanism.
- **500 lines is a soft limit:** No hard enforcement, but vendor recommends splitting large rules rather than making one large file.

## Official references

- Cursor Rules documentation: <https://cursor.com/docs/rules> (accessed 2026-03-07)
- Cursor Rules — rule anatomy section: <https://cursor.com/docs/rules#rule-anatomy> (accessed 2026-03-07)
- Cursor Rules — rule file format: <https://cursor.com/docs/rules#rule-file-format> (accessed 2026-03-07)
- Cursor Rules — AGENTS.md section: <https://cursor.com/docs/rules#agentsmd> (accessed 2026-03-07)

## Minimal example

`default.mdc` — a global always-on rule:

```markdown
---
alwaysApply: true
---

# Project conventions

## Build and test

- Build: `cargo build`
- Test: `cargo test --all-targets`
- Lint: `cargo clippy --all-targets -- -D warnings`

## Code style

- Do not use `unwrap()` or `expect()` in library code.
- All errors propagate via the `Error` and `Result` types in `src/error.rs`.
- MSRV is 1.75 — do not use APIs stabilized after 1.75.

## Architecture

- Business logic lives in the library crate (`src/lib.rs`).
- The binary (`src/main.rs`) is a thin arg-parsing entry point only.
```

Per-role example (Phase 4), `docs-agent.mdc`:

```markdown
---
description: "Rules for the docs agent working on documentation files"
globs: docs/**
alwaysApply: false
---

# Docs agent rules

- All documentation is written in Markdown.
- Cross-reference other docs with relative links.
- Do not modify source code files.
```

## Internal mapping notes

- Target ID: `cursor-rules`
- Output paths: `['.cursor/rules/default.mdc', ...]` (vec, Phase 4 adds per-role files)
- Template (global): `templates/cursor_rule.mdc.j2`
- Template (per-role, Phase 4): `templates/cursor_role.mdc.j2`
- `OutputTargets.cursor_rules: bool`
