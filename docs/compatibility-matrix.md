# Compatibility Matrix

Date: 2026-03-07
Status: Planning reference (updated as target support changes)

This matrix captures retained ideas from ecosystem research while mapping them to project scope and support tier.

| Target family                 | Typical repo path(s)                     | Class                 | Repo-safe         | Tier           | Notes                                                                                                           |
| ----------------------------- | ---------------------------------------- | --------------------- | ----------------- | -------------- | --------------------------------------------------------------------------------------------------------------- |
| AGENTS instructions           | `AGENTS.md`                              | Repo instruction      | Yes               | Core           | In v0.1; also read natively by Windsurf/Cascade and GitHub Copilot coding agent — one file, three tool families |
| Claude instructions           | `CLAUDE.md`                              | Repo instruction      | Yes               | Core           | In v0.1                                                                                                         |
| Cursor rules                  | `.cursor/rules/*.mdc`                    | Rule/config directory | Yes               | Core           | In v0.1                                                                                                         |
| Gemini instructions           | `GEMINI.md`                              | Repo instruction      | Yes               | Core           | In v0.2; `gemini-md` output target                                                                              |
| Copilot repo instructions     | `.github/copilot-instructions.md`        | Repo instruction      | Yes               | Core           | In v0.2; `copilot-instructions` output target                                                                   |
| Copilot path instructions     | `.github/instructions/*.instructions.md` | Rule/config directory | Yes               | Planned        | Requires path-frontmatter rendering model                                                                       |
| Copilot agent profiles        | `.github/agents/*.agent.md`              | Agent profile         | Yes               | Experimental   | Add only after profile model is stable                                                                          |
| Junie guidelines              | `.junie/guidelines.md`                   | Repo instruction      | Yes               | Planned        | Low schema impact target                                                                                        |
| Windsurf rules                | `.windsurf/rules/*.md`                   | Rule/config directory | Yes               | Experimental   | Candidate after core expansion waves                                                                            |
| Cline rules                   | `.clinerules/*.{md,txt}`                 | Rule/config directory | Yes               | Experimental   | Candidate after core expansion waves                                                                            |
| Continue rules                | `.continue/rules/*.md`                   | Rule/config directory | Yes               | Experimental   | Candidate after core expansion waves                                                                            |
| CODEOWNERS                    | `CODEOWNERS`                             | Governance artifact   | Yes               | Planned        | Phase 5 governance scope                                                                                        |
| Local/user memory or settings | Home/local tool config paths             | Local/user config     | No (team default) | Reference-only | Track only; do not default-generate                                                                             |

## Interpretation Rules

1. `Core` and `Planned` tiers drive roadmap and implementation priorities.
2. `Experimental` targets require explicit gating and clear instability messaging.
3. `Reference-only` targets are documented but intentionally excluded from default generation.

## Source of Truth

Target selection decisions are governed by [target-support-policy.md](target-support-policy.md).
Execution order and milestones are tracked in [roadmap.md](roadmap.md).
