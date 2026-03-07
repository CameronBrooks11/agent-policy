# Phase 0 — Bootstrap

**Goal:** A working Rust project with a clean, publication-ready foundation: CI green, project structure in final form, all scaffolding committed, README scope statement written. No functional code yet — just a skeleton you can build on without ever needing to restructure.

**Depends on:** Nothing (first phase)  
**Unlocks:** Phase 1

---

## Why this matters

The skeleton you lay in Phase 0 is hard to change later without churn. Getting the crate structure, `Cargo.toml` metadata, module layout, and CI workflow right now means you never have to restructure the repo mid-build, and the crates.io metadata is correct from the first publish attempt.

---

## Crate Structure

The crate is structured as **both a binary and a library from day one**. This is non-negotiable for testability and for clean crates.io publication.

```
[lib]   src/lib.rs      — public API, re-exports, crate-level docs
[[bin]] src/main.rs     — thin entry point, hands off to lib
```

The binary does one thing: parse args and call into the library. All real logic lives in the library. This means:

- Integration tests can call library functions directly without spawning a subprocess
- CLI integration tests can use `assert_cmd` to test the binary end-to-end
- The crate is usable as a library by future tooling

---

## `Cargo.toml`

Set all crates.io metadata correctly now so there are no surprises at publish time.

```toml
[package]
name = "agent-policy"
version = "0.0.0"
edition = "2021"
rust-version = "1.75"
authors = ["Your Name <you@example.com>"]
description = "Schema-first generator for coding-agent repo policies and compatibility files."
license = "Apache-2.0"
repository = "https://github.com/CameronBrooks11/agent-policy"
homepage = "https://github.com/CameronBrooks11/agent-policy"
documentation = "https://docs.rs/agent-policy"
readme = "README.md"
keywords = ["agent", "policy", "codegen", "cli", "llm"]
categories = ["command-line-utilities", "development-tools"]
exclude = [
    "docs/",
    "planning/",
    ".github/",
    "scratch/",
    "tmp/",
]

[[bin]]
name = "agent-policy"
path = "src/main.rs"

[lib]
name = "agent_policy"
path = "src/lib.rs"

[dependencies]
# Populated in phases 1–3

[dev-dependencies]
# Populated in phases 1–3
```

Notes:

- `rust-version` pins the MSRV. Set it to whatever stable version you build against and never let it drift silently.
- `exclude` keeps the published crate lean — docs and planning files do not belong in what users download.
- `keywords` must be max 5 items, max 20 chars each. These are what crates.io search indexes.
- `categories` must be exact strings from [crates.io/category_slugs](https://crates.io/category_slugs).

---

## Project Tree

This is the final target layout. Create all directories and stub files now.

```
agent-policy/
├── Cargo.toml
├── Cargo.lock
├── LICENSE                         ← Apache-2.0 full text
├── README.md                       ← scope statement (see below)
├── CHANGELOG.md                    ← stub: "# Changelog\n\n## Unreleased\n"
├── .gitignore
├── .editorconfig
├── rustfmt.toml
├── clippy.toml
├── agent-policy.schema.json        ← stub: {} for now
├── docs/
│   └── vision.md
├── planning/
│   ├── roadmap.md
│   ├── archive.md
│   └── phase-*.md
├── examples/
│   ├── minimal/
│   │   ├── agent-policy.yaml       ← stub config
│   │   └── expected/               ← empty dir, populated in phase 2
│   └── website/
│       └── agent-policy.yaml       ← stub config, expanded in phase 4
├── templates/
│   ├── AGENTS.md.j2                ← stub
│   ├── CLAUDE.md.j2                ← stub
│   └── cursor_rule.mdc.j2          ← stub
├── src/
│   ├── lib.rs                      ← stub
│   ├── main.rs                     ← stub
│   ├── cli.rs                      ← stub
│   ├── error.rs                    ← stub
│   ├── model/
│   │   ├── mod.rs
│   │   ├── policy.rs
│   │   ├── normalized.rs
│   │   └── targets.rs
│   ├── load/
│   │   ├── mod.rs
│   │   ├── yaml.rs
│   │   └── schema.rs
│   ├── render/
│   │   ├── mod.rs
│   │   ├── agents_md.rs
│   │   ├── claude_md.rs
│   │   └── cursor_rules.rs
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   ├── generate.rs
│   │   └── check.rs
│   └── util/
│       ├── mod.rs
│       ├── fs.rs
│       └── diff.rs
└── tests/
    ├── cli.rs                      ← stub
    ├── schema.rs                   ← stub
    └── golden.rs                   ← stub
```

All `mod.rs` and stub source files should be minimal but valid Rust — e.g., an empty `pub mod` declaration or a `// TODO` comment. The project must compile from day one.

---

## Config Files

### `.gitignore`

Start from the official Rust template and add:

```gitignore
/target
Cargo.lock     # remove this line — Cargo.lock SHOULD be committed for binary crates
*.log
.env
.env.*
tmp/
scratch/
.DS_Store
.idea/
.vscode/
```

Note: For binary crates (as opposed to library crates), `Cargo.lock` **must be committed**. This ensures reproducible builds. Do not add `Cargo.lock` to `.gitignore`.

### `.editorconfig`

```ini
root = true

[*]
charset = utf-8
end_of_line = lf
indent_style = space
indent_size = 4
trim_trailing_whitespace = true
insert_final_newline = true

[*.{yml,yaml,json,toml,md}]
indent_size = 2

[Makefile]
indent_style = tab
```

### `rustfmt.toml`

```toml
edition = "2021"
max_width = 100
use_field_init_shorthand = true
use_try_shorthand = true
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
```

### `clippy.toml`

```toml
msrv = "1.75"
```

Also add a `.clippy.toml`-compatible deny list at the top of `src/lib.rs`:

```rust
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
```

This is intentionally strict. `unwrap()` and `expect()` in library code should never make it to production — force proper error handling from the start.

---

## `README.md` — Phase 0 Content

The README must communicate clearly from day one. Write all sections now even if some are stubs.

Required sections:

```markdown
# agent-policy

Schema-first generator for coding-agent repo policies and compatibility files.

[![crates.io](badge url)](https://crates.io/crates/agent-policy)
[![docs.rs](badge url)](https://docs.rs/agent-policy)
[![CI](badge url)](GitHub Actions workflow url)
[![License: Apache-2.0](badge)](LICENSE)

## What it does

One-paragraph description of the problem and solution.

## Quick start

(stub — will be filled in Phase 2)

## Commands

(stub — will be filled in Phase 2)

## Schema

(stub — will be filled in Phase 1)

## Non-goals

- Not a replacement for AGENTS.md — it generates AGENTS.md
- Not a full enforcement engine
- Not a SaaS platform or hosted service
- Not a daemon or background process

## License

Apache-2.0
```

The non-goals section must be in the README. It sets expectations for users and contributors and prevents scope creep via pull requests.

---

## CI Workflow

File: `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  check:
    name: Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo check --all-targets

  fmt:
    name: Format
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --check

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --all-targets -- -D warnings

  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --all-targets

  doc:
    name: Docs
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo doc --no-deps --document-private-items
        env:
          RUSTDOCFLAGS: "-D warnings"
```

Notes:

- `Swatinem/rust-cache@v2` is the standard Rust caching action — include it on all steps that run cargo commands.
- The `doc` job uses `RUSTDOCFLAGS: "-D warnings"` — this causes missing doc comments to fail CI. This enforces documentation discipline from the start.
- `dtolnay/rust-toolchain` is the recommended way to install Rust in GitHub Actions (maintained by the `serde` author, widely trusted).

---

## Stub Source Files

### `src/main.rs`

```rust
fn main() {
    // Thin entry point — hands off to library in Phase 2
}
```

### `src/lib.rs`

```rust
//! # agent-policy
//!
//! Schema-first generator for coding-agent repo policies and compatibility files.
//!
//! See the [README](https://github.com/CameronBrooks11/agent-policy) for usage.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod commands;
pub mod error;
pub mod load;
pub mod model;
pub mod render;
pub(crate) mod util;
```

---

## Example Stub Configs

### `examples/minimal/agent-policy.yaml`

```yaml
# Minimal agent-policy configuration
# See agent-policy.schema.json for full schema
project:
  name: minimal-example
  summary: A minimal example configuration.
commands:
  test: "echo 'no tests yet'"
outputs:
  - agents-md
```

### `examples/website/agent-policy.yaml`

```yaml
# Website repo example — expanded in phase 4
project:
  name: website
  summary: Marketing website repository.
```

---

## Exit Condition

Phase 0 is complete when all of the following are true:

- [ ] `cargo build` succeeds with zero errors and zero warnings
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test` passes (empty test suite is acceptable)
- [ ] `cargo doc --no-deps` passes with no warnings (`RUSTDOCFLAGS="-D warnings"`)
- [ ] CI workflow runs and all jobs pass on the main branch
- [ ] All directories and stub files from the project tree exist
- [ ] `Cargo.toml` has complete crates.io metadata (description, keywords, categories, repository, license, readme)
- [ ] `CHANGELOG.md` exists with an "Unreleased" section
- [ ] `README.md` has the scope statement and non-goals section
- [ ] `Cargo.lock` is committed
