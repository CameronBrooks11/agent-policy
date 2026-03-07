# Target Spec Template

Copy this file when documenting a new target. Replace every `<placeholder>` and every guidance comment (`<!-- ... -->` blocks) with real, source-cited content. Never guess — leave a section as `_Not yet researched._` if the information is unavailable, and note what source is needed.

See [index.md](index.md) for the registry of all target files.

---

<!-- File name convention: use the target ID as the filename, e.g. gemini-md.md for target ID `gemini-md`. -->

# `<target-id>` — `<canonical output filename>`

<!-- Example: # `gemini-md` — `GEMINI.md` -->

## Summary

<!-- 1–3 sentences. What is this file? Who reads it? What is it for? Name the vendor/tool and the agent ecosystem it serves. -->

_Not yet researched._

## Tool

<!-- Primary tool (vendor name, binary name). List any other tools confirmed to read this file natively. Format:
- **Primary:** <Vendor Name> (`<binary>`)
- **Also read natively by:** <Tool A> (cite source), <Tool B> (cite source)
Note: do not list tools unless you have a cited vendor source confirming support. -->

_Not yet researched._

## Support level

<!-- One of:
- `supported` — implemented, tested, released
- `experimental` — implemented but format is unstable / vendor may change it
- `planned` — researched, not yet implemented
- `stub` — placeholder, research incomplete
Include which release added support, e.g. "supported — v0.1 Core target." -->

_Not yet researched._

## Canonical path(s)

<!-- The exact file path(s) relative to the repository root that the tool reads by default. If multiple paths are valid, list primary first. Include any subdirectory creation requirements. Source each path to vendor docs. -->

_Not yet researched._

## Alternate / legacy path(s)

<!-- Any deprecated, legacy, or optional alternate paths. Include:
- Legacy format files still supported by the tool.
- Configurable filename alternatives (e.g. via a settings file).
- Explicit note if there are none.
Format as a table if multiple with distinct attributes, or a bullet list. -->

None documented. _(confirm from vendor docs)_

## File format

<!-- Plain Markdown, JSON, YAML, MDC, TOML, etc. Describe:
- Required structure (headings, sections, keys).
- Optional structure (conventional sections not enforced by tooling).
- Size limits if documented.
- Any vendor-provided example (quote verbatim, cite source).
Do NOT describe renderer output format here — that belongs in Renderer notes. -->

_Not yet researched._

## Frontmatter

<!-- Does the file use YAML frontmatter? List every supported frontmatter key:
| Key | Type | Required | Default | Description |
|-----|------|----------|---------|-------------|
Source each field to vendor docs. If no frontmatter: state "None." -->

_Not yet researched._

## Discovery / scope behavior

<!-- How does the tool find and load this file? Describe:
- Search algorithm (root walk, CWD walk, fixed path, etc.)
- Scope semantics (global, project, directory, file-specific)
- Load order when multiple files apply
- Merge strategy (concatenate, prepend, override, etc.)
- Any size/count limits
Cite vendor docs for each behavior. -->

_Not yet researched._

## Repo-safe

<!-- Is this file designed to be committed to version control and shared with the team?
Answer Yes / No / Partial (with explanation). If partial, clarify which paths are repo-safe and which are machine-local. -->

_Not yet researched._

## Renderer notes

<!-- agent-policy-specific implementation notes. Include:
- Target ID (used in agent-policy.yaml `outputs` list)
- Output path string (exactly as written to disk)
- Template file name under `templates/`
- OutputTargets struct field name
- Any special rendering logic (e.g. one file per role, dynamic paths)
- Content sections the template should include
This section is written by the implementer, not sourced from vendor docs. -->

- **Target ID:** `<target-id>`
- **Output path:** `<path/to/output/file>`
- **Template:** `templates/<template-file>.j2`
- **`OutputTargets` field:** `<field_name>: bool`
- **Notes:** _Add implementation-specific notes._

## Known limitations / gotchas

<!-- Practical issues implementers and users should know about:
- Silent conflicts with other files, tools, or settings
- Behaviors that differ from what the file format suggests
- Context/token implications
- Things the renderer does NOT generate (by design)
Cite vendor docs if the gotcha comes from documented behavior; otherwise label as "observed" or "inferred." -->

_Not yet researched._

## Official references

<!-- List every vendor doc URL used to write this file. Format:
- <Description>: <URL> (accessed YYYY-MM-DD)
All entries must have an access date. Do not list URLs you have not actually read. -->

_No sources recorded yet._

## Minimal example

<!-- A realistic, self-contained example of this file as it would appear in a real repo. Keep it short (10–30 lines). Use a fenced code block with the appropriate language tag (markdown, yaml, json, etc.). -->

```markdown
<!-- replace with a realistic example -->
```

## Internal mapping notes

<!-- Cross-reference to the implementation. Update when the target is implemented:
- Target ID confirmed in `TargetId` enum
- Output path constant confirmed in `TargetId::primary_path()`
- Template file exists at `templates/`
- Golden test snapshot(s): list snapshot file names
- Renderer module: `src/render/<module>.rs` -->

_Not yet implemented._
