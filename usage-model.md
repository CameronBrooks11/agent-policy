Design this so that **policy definition** and **generation targets** are cleanly separated. The policy should describe *how agents should behave in the repository*, while generation settings describe *which tool compatibility files should be emitted*. Mixing these concerns creates long-term friction.

Below is the design space and the recommended approach.

---

# 1. Conceptual separation (important)

Your system really has **three layers**.

### Layer 1 — Canonical policy

The actual rules about the repo.

Example:

* editable paths
* protected paths
* commands
* constraints
* agent roles

This should live in:

```
agent-policy.yaml
```

This file should be **tool-agnostic** and stable long-term.

---

### Layer 2 — generation profile

Which files should be produced for which tools.

Example:

* AGENTS.md
* CLAUDE.md
* Cursor rules
* Copilot instructions

This controls compatibility outputs.

---

### Layer 3 — CLI overrides

Temporary developer decisions.

Example:

```
agent-policy generate --targets agents-md,cursor
```

This should **override but not replace** configuration.

---

# 2. Options for controlling generation

There are four viable models.

---

# Option A — Single config file (policy + outputs)

Example:

```
agent-policy.yaml
```

```
project:
  name: website

policy:
  commands:
    build: npm run build
  paths:
    editable:
      - src/**
    protected:
      - .github/**
      - functions/**

outputs:
  agents_md: true
  claude_md: true
  cursor_rules: true
  copilot: false
```

### Pros

* simplest UX
* single source of truth
* minimal files

### Cons

* mixes policy and generator concerns
* harder to reuse policy across repos

---

# Option B — separate generator config

```
agent-policy.yaml
agent-policy.targets.yaml
```

Example:

```
targets:
  - agents-md
  - claude-md
  - cursor
```

### Pros

* clean separation
* reusable policy

### Cons

* more files
* unnecessary complexity for MVP

---

# Option C — profiles inside config (recommended)

Profiles allow selecting **target sets**.

Example:

```
agent-policy.yaml
```

```
project:
  name: website

policy:
  commands:
    build: npm run build

profiles:

  default:
    targets:
      - agents-md
      - claude-md

  openai:
    targets:
      - agents-md

  cursor:
    targets:
      - agents-md
      - cursor-rules

  full:
    targets:
      - agents-md
      - claude-md
      - gemini-md
      - copilot
      - cursor-rules
      - windsurf-rules
```

CLI:

```
agent-policy generate
```

uses `default`.

```
agent-policy generate --profile full
```

switches profile.

### Pros

* scalable
* easy for users
* policy stays single-file
* supports tool bundles
* future-proof

### Cons

slightly more schema complexity.

---

# Option D — CLI only

```
agent-policy generate --targets agents-md,cursor
```

### Pros

simple implementation

### Cons

* non reproducible
* bad for CI
* bad for teams

---

# 3. Recommended design

Use **profiles inside the main config**, with CLI overrides.

This balances:

* simplicity
* flexibility
* reproducibility

### Default behaviour

```
agent-policy generate
```

uses:

```
profiles.default
```

### Tool bundle

```
agent-policy generate --profile cursor
```

### Explicit override

```
agent-policy generate --targets agents-md,claude-md
```

CLI overrides config.

---

# 4. Target abstraction

Your generator should treat outputs as **targets**.

Example internal enum:

```
Target:
  AgentsMd
  ClaudeMd
  GeminiMd
  CursorRules
  CopilotInstructions
  CopilotPathInstructions
  JunieGuidelines
  WindsurfRules
  ContinueRules
  ClineRules
```

Targets belong to **categories**.

```
InstructionFile
RuleDirectory
AgentProfiles
ToolConfig
```

Example mapping:

| Target                                 | Class       |
| -------------------------------------- | ----------- |
| AGENTS.md                              | instruction |
| CLAUDE.md                              | instruction |
| GEMINI.md                              | instruction |
| .cursor/rules                          | rules       |
| .github/copilot-instructions.md        | instruction |
| .github/instructions/*.instructions.md | path rules  |
| .clinerules                            | rules       |
| .windsurf/rules                        | rules       |

---

# 5. Data driven target registry

Instead of hardcoding everything, keep metadata.

Example:

```
data/targets/agents-md.yaml
```

```
id: agents-md
tool: openai
category: instruction
path: AGENTS.md
format: markdown
description: OpenAI agent instructions
```

Another:

```
data/targets/cursor-rules.yaml
```

```
id: cursor-rules
tool: cursor
category: rules
path: .cursor/rules/
format: mdc
```

Your Rust code loads these definitions.

Advantages:

* new tools can be added without touching core logic
* community PRs easy
* generator remains generic

---

# 6. Template selection

Each target references a template.

Example:

```
templates/
  agents_md.j2
  claude_md.j2
  gemini_md.j2
  cursor_rule.mdc.j2
```

Target metadata:

```
template: agents_md.j2
```

The renderer injects the normalized policy model.

---

# 7. CLI interface design

Recommended commands:

```
agent-policy init
agent-policy generate
agent-policy check
agent-policy list-targets
agent-policy list-profiles
```

Examples:

```
agent-policy generate
```

```
agent-policy generate --profile full
```

```
agent-policy generate --targets agents-md,cursor-rules
```

```
agent-policy list-targets
```

Output:

```
agents-md
claude-md
gemini-md
cursor-rules
copilot-instructions
copilot-path-instructions
junie-guidelines
cline-rules
windsurf-rules
continue-rules
```

---

# 8. Recommended default profiles

Include these built-in profiles.

### minimal

```
targets:
  - agents-md
```

### mainstream

```
targets:
  - agents-md
  - claude-md
  - gemini-md
  - cursor-rules
```

### enterprise

```
targets:
  - agents-md
  - claude-md
  - gemini-md
  - cursor-rules
  - copilot-instructions
  - copilot-path-instructions
```

### full

all known targets.

---

# 9. Example final config

```
project:
  name: website

policy:
  commands:
    build: npm run build
    test: npm test

  paths:
    editable:
      - src/**
    protected:
      - .github/**
      - functions/**

profiles:

  default:
    targets:
      - agents-md
      - claude-md
      - cursor-rules

  minimal:
    targets:
      - agents-md

  full:
    targets:
      - agents-md
      - claude-md
      - gemini-md
      - cursor-rules
      - copilot-instructions
      - copilot-path-instructions
      - junie-guidelines
```

---

# 10. Why this architecture is correct

This model:

* separates **policy semantics** from **tool compatibility**
* allows **future tools** without schema redesign
* keeps **CLI simple**
* allows **CI reproducibility**
* allows **community target plugins later**

Most importantly:

The **policy remains stable even if the LLM ecosystem changes**.

Only targets/templates evolve.

---

# 11. MVP scope to actually implement

For MVP implement only:

targets:

```
agents-md
claude-md
gemini-md
cursor-rules
copilot-instructions
```

profiles:

```
default
minimal
full
```

Everything else can be added incrementally.

---

If you want, the next useful step would be designing the **exact JSON schema for `agent-policy.yaml`** so the Rust CLI can validate configs cleanly.
