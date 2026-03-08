# agent-policy

Schema-first generator for coding-agent repo policies and compatibility files.

[![crates.io](https://img.shields.io/crates/v/agent-policy)](https://crates.io/crates/agent-policy)
[![npm](https://img.shields.io/npm/v/agent-policy)](https://www.npmjs.com/package/agent-policy)

## Install

```bash
# npm
npm install --save-dev agent-policy

# or run without installing
npx agent-policy --help
```

## Usage

```bash
# Scaffold a new agent-policy.yaml
npx agent-policy init

# Generate all output files (AGENTS.md, CLAUDE.md, .cursor/rules/, etc.)
npx agent-policy generate

# Check that committed files match the current policy
npx agent-policy check

# Import from existing AGENTS.md / CLAUDE.md
npx agent-policy import

# Install a git pre-commit hook
npx agent-policy install-hooks
```

## How it works

`agent-policy` is a Rust CLI distributed as prebuilt binaries. This npm package contains a thin JavaScript shim that resolves and executes the correct binary for your platform — no Rust toolchain required.

Supported platforms: Linux x64/arm64, macOS x64/arm64, Windows x64.

## Documentation

See the [GitHub repository](https://github.com/CameronBrooks11/agent-policy) for full documentation.

## License

Apache-2.0
