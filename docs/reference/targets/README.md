# Target Reference

Per-target specifications grounding the schema, renderer, and template implementations.
Each file follows the same section template. Fill sections from vendor documentation — do not guess.

## v0.1 targets (supported)

| ID | File | Tool |
|---|---|---|
| `agents-md` | [agents-md.md](agents-md.md) | OpenAI Codex CLI — also read natively by Cursor, Windsurf/Cascade, GitHub Copilot coding agent |
| `claude-md` | [claude-md.md](claude-md.md) | Anthropic Claude Code — also accepted by GitHub Copilot coding agent |
| `cursor-rules` | [cursor-rules.md](cursor-rules.md) | Cursor |

## Post-v0.1 targets (planned / experimental)

Add a file here when you begin researching each target before implementation.

---

## Section template

Every target file uses exactly these sections in this order:

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
```

Leave a section as `_Not yet researched._` rather than omitting it or guessing.
