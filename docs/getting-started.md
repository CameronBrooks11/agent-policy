# Getting Started

This guide walks through installing `agent-policy`, setting up a policy for your repository, and wiring up drift detection in CI.

---

## Install

### From npm (no Rust toolchain required)

```bash
npx agent-policy
# or install globally
npm install -g agent-policy
```

Requires Node.js 18+. npm automatically installs only the pre-built binary for your current platform — no compilation needed.

### From crates.io

Requires [Rust](https://rustup.rs/) (stable, MSRV 1.75):

```bash
cargo install agent-policy
```

### Pre-built binary

Download a binary for your platform from the [GitHub releases page](https://github.com/CameronBrooks11/agent-policy/releases). Extract and place it on your `PATH`.

Supported platforms: Linux x86_64, Linux arm64, macOS x86_64 (Intel), macOS arm64 (Apple Silicon), Windows x86_64.

### Verify

```bash
agent-policy --version
```

---

## Step 1 — Initialize

Run this in the root of your repository:

```bash
agent-policy init
```

This creates `agent-policy.yaml` with a minimal starting configuration. If the file already exists, `init` will not overwrite it unless you pass `--force`.

---

## Step 2 — Configure your policy

Open `agent-policy.yaml` and fill in the sections relevant to your project. Only `project.name` is required — everything else is optional and additive.

### Minimal example

```yaml
project:
  name: my-repo

outputs:
  - agents-md
```

### Full example

```yaml
project:
  name: my-repo
  summary: A web application backend and marketing site.

commands:
  install: npm install
  dev: npm run dev
  lint: npm run lint
  test: npm test
  build: npm run build

paths:
  editable:
    - src/**
    - tests/**
    - docs/**
  protected:
    - .github/workflows/**
    - package-lock.json
    - infrastructure/**
  generated:
    - AGENTS.md
    - CLAUDE.md
    - .cursor/rules/**

roles:
  docs_agent:
    editable:
      - docs/**
      - content/**
    forbidden:
      - src/**
      - infrastructure/**
  backend_agent:
    editable:
      - src/**
      - tests/**
    forbidden:
      - infrastructure/**

constraints:
  forbid_secrets: true
  require_tests_for_code_changes: true
  require_human_review_for_protected_paths: true

outputs:
  - agents-md
  - claude-md
  - cursor-rules
```

See the [Schema Reference](schema-reference.md) for a full description of every field.

---

## Step 3 — Generate files

```bash
agent-policy generate
```

This writes all output files to your repository root. The files that are written depend on which targets you listed in `outputs`:

| Target         | Files written                                                                                |
| -------------- | -------------------------------------------------------------------------------------------- |
| `agents-md`    | `AGENTS.md`                                                                                  |
| `claude-md`    | `CLAUDE.md`                                                                                  |
| `cursor-rules` | `.cursor/rules/default.mdc` and `.cursor/rules/{role}.mdc` for each role with editable paths |

!!! tip "Custom config path"
If your `agent-policy.yaml` is not at the repo root, pass the path explicitly:
`bash
    agent-policy generate --config path/to/agent-policy.yaml
    `

---

## Step 4 — Commit everything

Commit the policy file **and** all generated files. The generated files are committed intentionally — they are the compatibility artifacts your agent tools read.

```bash
git add agent-policy.yaml AGENTS.md CLAUDE.md .cursor/
git commit -m "chore: add agent-policy and generated instruction files"
```

---

## Step 5 — Add a CI check

Add `agent-policy check` to your CI pipeline. It runs the same generation pipeline in memory and exits non-zero if any committed file is missing or differs from what would be generated.

### GitHub Actions

```yaml
# In your existing CI workflow, add a new job or step:

- name: Check policy drift
  run: |
    curl -fsSL https://github.com/CameronBrooks11/agent-policy/releases/latest/download/agent-policy-x86_64-unknown-linux-gnu.tar.gz | tar xz
    ./agent-policy check
```

Or if you have npm available:

```yaml
- name: Install agent-policy
  run: npm install -g agent-policy

- name: Check agent policy
  run: agent-policy check
```

Or if you use Rust in CI already:

```yaml
- name: Install agent-policy
  run: cargo install agent-policy

- name: Check agent policy
  run: agent-policy check
```

### What check does

On success:

```
✓ All 3 generated file(s) are up to date.
```

On failure (exits 1):

```
Generated files are out of date:

  stale    AGENTS.md
--- AGENTS.md (committed)
+++ AGENTS.md (generated)
@@
 ## Commands
-
-- **Test:** `go test ./...`
+- **Test:** `go test -race ./...`

Run `agent-policy generate` to update.
```

!!! note "The full workflow"
`     edit agent-policy.yaml  →  agent-policy generate  →  commit  →  CI runs check
    `
Manual edits to generated files will be caught and rejected by the CI check. The policy YAML is always the canonical source.

---

## Commands reference

| Command                                 | Description                                                             |
| --------------------------------------- | ----------------------------------------------------------------------- |
| `agent-policy init`                     | Create a starter `agent-policy.yaml` in the current directory           |
| `agent-policy init --force`             | Overwrite an existing `agent-policy.yaml`                               |
| `agent-policy generate`                 | Render all enabled output targets to disk                               |
| `agent-policy generate --config <path>` | Use a config file at a custom path                                      |
| `agent-policy check`                    | Verify committed files match the current policy; exit 1 on any mismatch |
| `agent-policy check --config <path>`    | Check using a config file at a custom path                              |
| `agent-policy install-hooks`            | Install a pre-commit or pre-push git hook that runs `check` locally     |
| `agent-policy import`                   | Scaffold `agent-policy.yaml` by scraping an existing `AGENTS.md` or `CLAUDE.md` |
| `agent-policy --version`                | Print the installed version                                             |
| `agent-policy --help`                   | Print help                                                              |
