# Target Reference

Per-target specifications grounding the schema, renderer, and template implementations.
Each file follows the same section template from [target-spec-template.md](target-spec-template.md). Fill sections from vendor documentation — do not guess.

## v0.1 targets (supported)

| ID             | File                               | Tool                                                                                           |
| -------------- | ---------------------------------- | ---------------------------------------------------------------------------------------------- |
| `agents-md`    | [agents-md.md](agents-md.md)       | OpenAI Codex CLI — also read natively by Cursor, Windsurf/Cascade, GitHub Copilot coding agent |
| `claude-md`    | [claude-md.md](claude-md.md)       | Anthropic Claude Code — also accepted by GitHub Copilot coding agent                           |
| `cursor-rules` | [cursor-rules.md](cursor-rules.md) | Cursor                                                                                         |

## v0.2 targets (planned)

| ID                      | File                                                       | Tool                                          |
| ----------------------- | ---------------------------------------------------------- | --------------------------------------------- |
| `gemini-md`             | [gemini-md.md](gemini-md.md)                               | Google Gemini CLI                             |
| `copilot-instructions`  | [copilot-instructions.md](copilot-instructions.md)         | GitHub Copilot (Chat, coding agent, code review) |

## Future targets (post-v0.2, not yet researched)

| ID                       | Output path                                  | Tool                  | Milestone |
| ------------------------ | -------------------------------------------- | --------------------- | --------- |
| `windsurf-rules`         | `.windsurf/rules/*.md`                       | Windsurf (Codeium)    | v0.3      |
| `cline-rules`            | `.clinerules/`                               | Cline                 | v0.3      |
| `junie-guidelines`       | `.junie/guidelines.md`                       | JetBrains Junie       | v0.3      |
| `copilot-path-instructions` | `.github/instructions/*.instructions.md`  | GitHub Copilot (path-scoped) | v0.3 |

Add a spec file (from [target-spec-template.md](target-spec-template.md)) here when you begin researching each target before implementation.

---

## Adding a new target

1. Copy [target-spec-template.md](target-spec-template.md) to a new file named `<target-id>.md` in this directory.
2. Fill every section from vendor documentation. Record URL and access date for every source.
3. Add a row to the appropriate table above.
4. Add the file to `mkdocs.yml` nav under the Targets section.
5. Once implemented, update `Support level` to `supported — vX.Y` and move the row to the relevant supported table.

## Section template (summary)

Every target file uses exactly these sections in this order (see [target-spec-template.md](target-spec-template.md) for the full annotated version):

```
## Summary
## Tool
## Support level
## Canonical path(s)
## Alternate / legacy path(s)
## File format
## Frontmatter
## Discovery / scope behavior
## Repo-safe
## Renderer notes
## Known limitations / gotchas
## Official references
## Minimal example
## Internal mapping notes
```

Leave a section as `_Not yet researched._` rather than omitting it or guessing.
