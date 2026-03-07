# `windsurf-rules` — `.windsurf/rules/*.md`

## Summary

Repository-level rules directory for Windsurf IDE (Codeium). It guides its built-in AI agent (Cascade) with coding standards, architectural preferences, and path-scoped rules using YAML frontmatter.

## Tool

- **Primary:** Codeium Windsurf (`windsurf`)

## Support level

`planned` — v0.3

## Canonical path(s)

- `.windsurf/rules/*.md` (Supports individual Markdown files placed in this directory)

## Alternate / legacy path(s)

- `.windsurfrules` (legacy single-file format at the repository root, currently supported but acts as broadly "always on").

## File format

Markdown syntax for rules, with required YAML frontmatter at the top of the file to configure behaviors.

## Frontmatter

Windsurf expects YAML frontmatter dictating how and when the AI injects the file:

| Key | Type | Required | Default | Description |
|-----|------|----------|---------|-------------|
| `description` | string | Yes | None | A brief description of what the rules handle. Critical for `model_decision` trigger types. |
| `trigger` | string | Yes | None | One of `glob`, `always_on`, `model_decision`, `manual`. |
| `globs` | string[] | No | None | Required when `trigger: glob`. An array of file path patterns that determine when the rule is injected dynamically. |

## Discovery / scope behavior

- **Search:** Cascade discovers any `.md` file stored within `.windsurf/rules/` upon launch or interaction.
- **Scope:** Bounded exclusively by the `trigger` definitions. The renderer will automatically favor `glob` triggers, mapping roles' editable path scopes into the `globs` array.
- **Merge Strategy:** Windsurf evaluates the frontmatter constraints for matching context and merges appropriately gated rules automatically.

## Repo-safe

Yes. Meant to be committed in version control to ensure the Windsurf tools align across developer environments.

## Renderer notes

- **Target ID:** `windsurf-rules`
- **Output path:** `.windsurf/rules/`
- **Template:** `templates/.windsurf/rules.j2`
- **`OutputTargets` field:** `windsurf_rules: bool`
- **Notes:** Outputs similarly to Cursor: a global `default.md` using `trigger: always_on` and role-specific `.md` files using `trigger: glob` targeting the exact editable arrays configured in `agent-policy.yaml`.

## Known limitations / gotchas

- The `description` field is heavily mandated by Windsurf and must be adequately generated so that it does not confuse the AI when indexing rules.

## Official references

- Windsurf Cascade rules and guidelines documentation. (accessed 2026-03-07)

## Minimal example

```markdown
---
description: "React Frontend Patterns"
trigger: glob
globs:
  - "src/frontend/**/*.tsx"
  - "src/frontend/**/*.ts"
---

# React Rules
- Ensure all pages are strongly typed using TypeScript interfaces.
- Avoid Class components.
```

## Internal mapping notes

_Not yet implemented._
