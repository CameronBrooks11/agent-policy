#!/usr/bin/env node
"use strict";

const { execFileSync } = require("child_process");
const fs = require("fs");
const path = require("path");

const PLATFORMS = {
  "linux-x64":    { pkg: "@cameronbrooks11/agent-policy-linux-x64",    bin: "agent-policy"     },
  "linux-arm64":  { pkg: "@cameronbrooks11/agent-policy-linux-arm64",   bin: "agent-policy"     },
  "darwin-x64":   { pkg: "@cameronbrooks11/agent-policy-darwin-x64",    bin: "agent-policy"     },
  "darwin-arm64": { pkg: "@cameronbrooks11/agent-policy-darwin-arm64",  bin: "agent-policy"     },
  "win32-x64":    { pkg: "@cameronbrooks11/agent-policy-win32-x64",     bin: "agent-policy.exe" },
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

function run() {
  execFileSync(binPath, process.argv.slice(2), { stdio: "inherit" });
}

try {
  try {
    run();
  } catch (e) {
    // 0.6.0 shipped its Unix binaries without the execute bit (#20).
    if (e.code !== "EACCES") throw e;
    try {
      fs.chmodSync(binPath, 0o755);
    } catch {
      process.stderr.write(
        `agent-policy: ${binPath} is not executable and could not be fixed.\n` +
        `Run: chmod +x "${binPath}"\n`
      );
      process.exit(1);
    }
    run();
  }
} catch (e) {
  // A non-zero exit from the binary has a numeric status and has already
  // printed its own output; anything else is a spawn failure nobody has seen.
  if (typeof e.status === "number") process.exit(e.status);
  process.stderr.write(`agent-policy: failed to run ${binPath}: ${e.message}\n`);
  process.exit(1);
}
