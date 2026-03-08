#!/usr/bin/env node
"use strict";

const { execFileSync } = require("child_process");
const path = require("path");

const PLATFORMS = {
  "linux-x64":    { pkg: "@agent-policy/linux-x64",    bin: "agent-policy"     },
  "linux-arm64":  { pkg: "@agent-policy/linux-arm64",   bin: "agent-policy"     },
  "darwin-x64":   { pkg: "@agent-policy/darwin-x64",    bin: "agent-policy"     },
  "darwin-arm64": { pkg: "@agent-policy/darwin-arm64",  bin: "agent-policy"     },
  "win32-x64":    { pkg: "@agent-policy/win32-x64",     bin: "agent-policy.exe" },
};

const key = `${process.platform}-${process.arch}`;
const entry = PLATFORMS[key];

if (!entry) {
  process.stderr.write(
    `agent-policy: unsupported platform: ${key}\n` +
    `Supported platforms: ${Object.keys(PLATFORMS).join(", ")}\n` +
    `As a fallback, install via: cargo install agent-policy\n`
  );
  process.exit(1);
}

let binPath;
try {
  binPath = require.resolve(path.join(entry.pkg, entry.bin));
} catch {
  process.stderr.write(
    `agent-policy: platform package ${entry.pkg} is not installed.\n` +
    `This is likely an npm install issue. Try reinstalling agent-policy.\n` +
    `As a fallback, install via: cargo install agent-policy\n`
  );
  process.exit(1);
}

try {
  execFileSync(binPath, process.argv.slice(2), { stdio: "inherit" });
} catch (e) {
  process.exit(e.status ?? 1);
}
