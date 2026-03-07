# `clinerules` — `.clinerules`

## Summary

Custom repository instruction rules for the Cline AI assistant (formerly Claude Dev). It supports defining multi-file path-scoped rule combinations or single-file repository globals to standardize how the bot creates files, runs commands, and structures code.

## Tool

- **Primary:** Cline (`cline`)
- **Also read natively by:** Roo Code (a popular Cline fork)

## Support level

`planned` — v0.3

## Canonical path(s)

- `.clinerules/` (directory containing multiple rule files)
- `.clinerules` (single file at the workspace root)

## Alternate / legacy path(s)

None documented.

## File format

Markdown. When using the directory structure, the content within each markdown file is governed by YAML frontmatter to determine contextual scope.

## Frontmatter

When splitting rules across multiple files inside a `.clinerules/` directory, Cline uses frontmatter to selectively inject them:

| Key | Type | Required | Default | Description |
|-----|------|----------|---------|-------------|
| `paths` | string[] / string | No | None | Glob patterns used to determine which workspace files will trigger these rules via the `picomatch` library. |

## Discovery / scope behavior

- **Search:** Walks the root directory looking for either a `.clinerules` file or a `.clinerules/` directory.
- **Scope:** Glob patterns in frontmatter map against candidate files active in the user's prompt or context. If they match, the rule is loaded.
- **Merge strategy:** If multiple files in `.clinerules/` are matched in context, they are concatenated. Global rules in a user's setting profile are applied alongside workspace rules.

## Repo-safe

Yes. This file or directory is meant to be committed to version history to enforce consistent AI guidelines across the repository.

## Renderer notes

- **Target ID:** `clinerules`
- **Output path:** `.clinerules/` (the renderer will default to the directory structure to support role-based scoped rules).
- **Template:** `templates/clinerules.j2`
- **`OutputTargets` field:** `clinerules: bool`
- **Notes:** Outputting exactly like `cursor-rules`, where a `default.md` handles the global paths, and subsequent `role.md` files contain the YAML `paths:` frontmatter block matching the `Globs` arrays for context boundaries.

## Known limitations / gotchas

- The `.clinerules/` directory inherently reserves sub-folders like `workflows`, `hooks`, and `skills` for advanced Cline operations. Rendered rules must not clash with these reserved names.
- It uses `picomatch` for glob evaluations, which allows standard advanced `**/*.ts` inclusion scopes.

## Official references

- Cline GitHub Repository `.clinerules` implementation tracking: https://github.com/cline/cline (accessed 2026-03-07)

## Minimal example

```markdown
---
paths:
  - "src/frontend/**/*.tsx"
  - "docs/**"
---

# Frontend Docs Rules
Always use functional components and document them with accurate JSDoc strings.
Never modify backend components from this scope.
```

## Internal mapping notes

_Not yet implemented._
