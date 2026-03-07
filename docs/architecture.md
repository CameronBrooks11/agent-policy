# Architecture

## Overview

`agent-policy` is a pipeline tool: it loads a structured policy file, validates and normalizes it into a stable internal model, then renders that model into one or more compatibility output files. A separate `check` mode runs the exact same pipeline in memory and compares results against committed files to detect drift.

```
agent-policy.yaml  (canonical source of truth)
        ↓ load
raw YAML string
        ↓ serde_yaml::from_str
RawPolicy  (raw serde types, mirrors schema)
        ↓ serde_json::to_value → jsonschema validate
validated raw document
        ↓ normalize()
Policy  (stable normalized model)
        ↓ render_all()
Vec<RenderedOutput>  { path, content }
        ↓
  generate: write to disk
  check:    compare against committed files, diff to stderr
```

---

## Layer Separation and Why It Matters

### 1. Load layer (`src/load/`)

Responsible for reading bytes from disk, deserializing into `RawPolicy` via `serde_yaml`, then validating the result against the bundled JSON Schema via `jsonschema`. Fails fast with actionable errors on bad input.

The JSON Schema (`agent-policy.schema.json`) is bundled into the binary at compile time via `include_str!`. This means consuming repos need only the binary — no external schema file is required at runtime.

### 2. Raw vs. normalized model

`RawPolicy` (in `src/model/policy.rs`) mirrors the YAML schema exactly, with all optional fields as `Option<T>`. It is the deserialization target only — renderers never consume it directly.

`Policy` (in `src/model/normalized.rs`) is what everything downstream consumes. `normalize()` applies defaults, validates semantic constraints (e.g., role name format, non-empty outputs list), and produces a fully resolved model with no `Option` fields where they are not meaningful.

**Why this separation?** The YAML schema can evolve without touching renderer logic. Renderers write against the stable normalized contract, not against the shape of the YAML file. This was the most important architectural decision made in the project.

### 3. Render layer (`src/render/`)

`render_all(policy: &Policy) -> Result<Vec<RenderedOutput>>` is the single entry point. It dispatches to per-target renderer modules based on `policy.outputs`. Each renderer takes the normalized `Policy` and returns a `RenderedOutput` containing a relative path and a string content.

Templates live in `templates/` and are embedded at compile time via `include_str!`. Templates use [minijinja](https://docs.rs/minijinja) (a Rust-native Jinja2 implementation). Template content is versioned with the binary — no runtime template files are needed.

The cursor rules renderer returns `Vec<RenderedOutput>` rather than a single output because Cursor supports multiple `.mdc` files (one global `default.mdc` plus one per role). `render_all()` uses `.extend()` for this renderer to flatten the results into the top-level vec. This design generalizes cleanly if future targets also require multiple output files.

### 4. Check mode (not a separate pipeline)

`agent-policy check` does **not** have its own pipeline. It calls `render_all()` from the same render layer, reads each committed file from disk, normalizes line endings (to handle CRLF/LF differences on Windows), and compares strings. If any file is missing or differs, it formats a unified diff using the `similar` crate and exits non-zero.

This matters: check and generate are guaranteed to agree on what the output should look like, because they share the exact same render function. There is no risk of "check uses a separate code path that says X is fine while generate produces Y."

---

## Error Handling

All errors flow through a single `Error` enum in `src/error.rs`, using `thiserror`. The complete set of variants is defined before Phase 2 renderers were built — this kept the error type stable and prevented mid-build patching of a foundational type.

Key variants:

- `Io` — file read/write failure with path context
- `Yaml` — serde_yaml parse failure
- `Schema` — JSON Schema validation failure (includes formatted error message)
- `Glob` — invalid glob pattern in policy
- `InvalidRoleName` — role names must be lowercase letters, digits, and underscores
- `Render` — minijinja template rendering failure with target name context
- `CheckFailed` — generated file differs from committed (used to trigger non-zero exit)
- `NoOutputs` — outputs list is present but empty
- `UnknownTarget` — unrecognized target ID in outputs list

`unwrap()` and `expect()` are banned in library code via `#![deny(clippy::unwrap_used)]` and `#![deny(clippy::expect_used)]` in `src/lib.rs`. The `#[allow(clippy::unwrap_used)]` attribute in `util/diff.rs` is the only deliberate exception — `write!` to a `String` is provably infallible.

---

## Atomic File Writes

Generated files are written atomically via `write_file_atomic()` in `src/util/fs.rs`. On Unix this means write-to-temp-then-rename; on Windows it writes directly due to rename-over-existing-file limitations. This prevents partially written output files if the process is interrupted mid-generate.

---

## Crate Structure

The crate is structured as both a binary and a library from day one:

- `src/lib.rs` — public API, re-exports, crate-level docs
- `src/main.rs` — thin entry point; parses arguments and calls into the library

All real logic lives in the library. This means integration tests can call library functions directly without spawning a subprocess, while `tests/cli.rs` uses `assert_cmd` to test binary behavior end-to-end.

---

## Testing Layers

| Layer           | Location          | Purpose                                                             |
| --------------- | ----------------- | ------------------------------------------------------------------- |
| Schema tests    | `tests/schema.rs` | YAML parsing, schema validation, normalization edge cases           |
| CLI tests       | `tests/cli.rs`    | Binary command behavior, exit codes, error output                   |
| Integrity tests | `tests/check.rs`  | `check` command — drift detection, diff output, round-trip          |
| Golden tests    | `tests/golden.rs` | Snapshot render output for all examples; uses `insta` for snapshots |

Golden tests use [`insta`](https://docs.rs/insta). New snapshots are accepted by running `INSTA_UPDATE=unseen cargo test`. Existing snapshots must never be silently updated — snapshot changes should be reviewed in PRs like any other output change.
