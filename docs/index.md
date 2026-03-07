# agent-policy

**One canonical YAML file. Every agent instruction file — generated, checked, and always in sync.**

---

The modern development ecosystem runs on coding agents — Codex, Claude Code, Cursor, Copilot, Gemini CLI, Windsurf, Cline. Each one expects its own configuration file in its own format at its own path. Every file expresses the same underlying policy: what agents can touch, what commands to run, what constraints to respect.

Maintaining those files by hand leads to drift, inconsistency, and fragile agent behavior.

`agent-policy` treats every agent instruction file as a **projection of a single policy model**:

```
agent-policy.yaml   ←  canonical source of truth
        │
        ├─ AGENTS.md               (OpenAI Codex, GitHub Copilot, Windsurf, ...)
        ├─ CLAUDE.md               (Anthropic Claude Code)
        └─ .cursor/rules/*.mdc     (Cursor — global + per-role)
```

Edit the YAML once. Generate all files. Catch drift in CI.

---

## Install

```bash
cargo install agent-policy
```

Binary releases for Linux, macOS, and Windows are available on the [GitHub releases page](https://github.com/CameronBrooks11/agent-policy/releases).

---

## Quick start

**1. Initialize a policy file in your repo:**

```bash
agent-policy init
```

**2. Edit `agent-policy.yaml`:**

```yaml
project:
  name: my-repo
  summary: A web application.

commands:
  test: npm test
  lint: npm run lint
  build: npm run build

paths:
  editable:
    - src/**
    - tests/**
  protected:
    - .github/workflows/**
    - package-lock.json
  generated:
    - AGENTS.md
    - CLAUDE.md

constraints:
  forbid_secrets: true
  require_tests_for_code_changes: true

outputs:
  - agents-md
  - claude-md
  - cursor-rules
```

**3. Generate and commit:**

```bash
agent-policy generate
git add AGENTS.md CLAUDE.md .cursor/
git commit -m "chore: add agent-policy generated files"
```

**4. Enforce in CI:**

```yaml
# .github/workflows/ci.yml
- name: Check agent policy
  run: agent-policy check
```

`check` exits non-zero and prints a diff if any generated file is out of sync with the current policy.

---

## What gets generated

| Output target  | Files                                                              | Tools                                                  |
| -------------- | ------------------------------------------------------------------ | ------------------------------------------------------ |
| `agents-md`    | `AGENTS.md`                                                        | Codex, Cursor, Copilot, Windsurf, Gemini CLI, and more |
| `claude-md`    | `CLAUDE.md`                                                        | Claude Code, GitHub Copilot                            |
| `cursor-rules` | `.cursor/rules/default.mdc`<br>`.cursor/rules/{role}.mdc` per role | Cursor                                                 |

---

## Next steps

- [Getting Started](getting-started.md) — full walkthrough from install to CI
- [Schema Reference](schema-reference.md) — every field in `agent-policy.yaml`
- [Targets](compatibility-matrix.md) — supported tools, stability tiers, and planned targets
