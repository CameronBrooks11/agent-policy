# `junie-guidelines` — `.junie/guidelines.md`

## Summary

Code style and framework convention guardrails tailored specifically for JetBrains Junie (their suite-wide AI assistant). It establishes project expectations regarding testing, styling, folder structures, and tech stacks out of the box.

## Tool

- **Primary:** JetBrains Junie (`junie`)

## Support level

`planned` — v0.3

## Canonical path(s)

- `.junie/guidelines.md`

## Alternate / legacy path(s)

- `AGENTS.md` (Native fallback supported natively by JetBrains in newer plugin updates)
- `.junie/AGENTS.md`
- Custom IDE location via `Settings | Tools | Junie | Project Settings`.

## File format

Markdown. Highly structured with explicit sections for Core Stack, Coding Conventions, Directories, and Anti-Patterns.
- Snippets of code are encouraged to teach Junie by example.

## Frontmatter

None.

## Discovery / scope behavior

- **Search:** Discovered dynamically by JetBrains IDE upon project loading when the file sits at the root `.junie/guidelines.md` or as `AGENTS.md`.
- **Scope:** Workspace-global. Absorbed entirely during generation or chat tasks to steer the codebase design naturally.
- **Merge Strategy:** Appends rules universally across file types, allowing cross-project logic combinations.

## Repo-safe

Yes. Committed globally to unify developer and AI pipelines.

## Renderer notes

- **Target ID:** `junie-guidelines`
- **Output path:** `.junie/guidelines.md`
- **Template:** `templates/.junie/guidelines.j2`
- **`OutputTargets` field:** `junie_guidelines: bool`
- **Notes:** Like Windsurf, Junie rules lack isolated glob-controlled multi-file scopes natively. The renderer must dump the full configuration (including role partitions as headers) into the single markdown file using textual conditionals.

## Known limitations / gotchas

- The official Junie Github states `.junie/guidelines.md` is becoming mostly a legacy structure, with their primary supported focus moving towards generic project `AGENTS.md` files. This means our existing `agents-md` target technically accomplishes the heavy lifting for JetBrains natively already.
- Lack of path-scoping means all rules share the prompt window.

## Official references

- JetBrains Junie Open Source Guidelines templates: https://github.com/JetBrains/junie-guidelines (accessed 2026-03-07)

## Minimal example

```markdown
# Junie Repository Conventions

## Testing
- Use `pytest`.
- Async tests must reside entirely in `pytest-asyncio`.
- All endpoints must include respective integration tests.

## Anti-patterns
- Never hardcode credentials.
- Do not utilize `kwargs` dynamically; enforce strict Pydantic typing.
```

## Internal mapping notes

_Not yet implemented._
