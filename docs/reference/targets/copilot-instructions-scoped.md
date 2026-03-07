# `copilot-instructions-scoped` — `.github/instructions/*.md`

## Summary

Domain or task-specific instruction files for GitHub Copilot. Unlike the global `.github/copilot-instructions.md`, these scoped instructions allow developers to chunk logic for varying workspaces or languages, enabling Copilot Chat and generation to select context-relevant code guidelines.

## Tool

- **Primary:** GitHub Copilot (`github-copilot`)

## Support level

`planned` — v0.3

## Canonical path(s)

`.github/instructions/*.md`

## Alternate / legacy path(s)

None documented.

## File format

Standard Markdown (`.md`).
- Headings are encouraged for structure.
- Often kept concise so as not to overwhelm standard token contexts.

## Frontmatter

None.

## Discovery / scope behavior

- **Search:** Copilot natively discovers and parses any `.md` file stored within the `.github/instructions/` repository directory.
- **Scope:** Typically scoped dynamically based on user interaction (specifying `@file` instructions in chat prompt references) or through contextual inference.
- **Merge Strategy:** Pre-pended alongside system prompts during Copilot Chat interactions when triggered.

## Repo-safe

Yes. Meant to be committed in source control alongside `.github/workflows` to ensure Copilot rules are distributed repo-wide.

## Renderer notes

- **Target ID:** `copilot-instructions-scoped`
- **Output path:** `.github/instructions/`
- **Template:** `templates/.github/instructions.j2`
- **`OutputTargets` field:** `copilot_instructions_scoped: bool`
- **Notes:** Similar to `cursor-rules`, this will cleanly split out globally defined repo paths into a `default.md` alongside individual `[role_name].md` files reflecting the specialized constraints inside the YAML configs.

## Known limitations / gotchas

- Scoped instructions do not currently possess a documented internal "globbing" parameter like `cursor-rules` or `clinerules` to automatically gate themselves on exact paths. Efficacy heavily relies on the developer explicitly `@` mentioning them in chat, or Copilot's inference engine selecting them.

## Official references

- GitHub Copilot Documentation - Adding Custom Instructions to your repository: https://docs.github.com/en/copilot/customizing-copilot/adding-custom-instructions-for-github-copilot (accessed 2026-03-07)

## Minimal example

```markdown
# Frontend Style Rules

When creating components:
- Use Tailwind CSS for styling exclusively.
- Build components as pure functions in `src/components/`.
- Ensure all states use React Hooks.
```

## Internal mapping notes

_Not yet implemented._
