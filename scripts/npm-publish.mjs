#!/usr/bin/env node
/**
 * scripts/npm-publish.mjs
 *
 * Downloads prebuilt binaries from a GitHub release, places them into the
 * npm platform packages, bumps all package.json versions, then publishes to npm.
 *
 * Usage:
 *   node scripts/npm-publish.mjs --version 0.6.0
 *   node scripts/npm-publish.mjs --version 0.6.0 --dry-run
 *   node scripts/npm-publish.mjs --version 0.6.0 --tag next
 *
 * Requirements:
 *   - Node.js >= 18 (uses built-in fetch + ReadableStream)
 *   - `tar` in PATH  (available on macOS, Linux, Windows 10+)
 *   - npm logged in, or NPM_TOKEN set in environment
 */

import { execFileSync, execSync } from "child_process";
import { createWriteStream, mkdirSync, chmodSync, existsSync, renameSync, rmSync } from "fs";
import { readFile, writeFile } from "fs/promises";
import { join, dirname } from "path";
import { fileURLToPath } from "url";
import { pipeline } from "stream/promises";
import { Readable } from "stream";

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

const REPO = "CameronBrooks11/agent-policy";

/** cargo-dist target triple → { npm package dir (relative to repo root), binary filename } */
const PLATFORMS = [
  {
    triple:  "x86_64-unknown-linux-gnu",
    dir:     "npm/platforms/linux-x64",
    pkg:     "@agent-policy/linux-x64",
    bin:     "agent-policy",
    archive: "tar.xz",
  },
  {
    triple:  "aarch64-unknown-linux-gnu",
    dir:     "npm/platforms/linux-arm64",
    pkg:     "@agent-policy/linux-arm64",
    bin:     "agent-policy",
    archive: "tar.xz",
  },
  {
    triple:  "x86_64-apple-darwin",
    dir:     "npm/platforms/darwin-x64",
    pkg:     "@agent-policy/darwin-x64",
    bin:     "agent-policy",
    archive: "tar.xz",
  },
  {
    triple:  "aarch64-apple-darwin",
    dir:     "npm/platforms/darwin-arm64",
    pkg:     "@agent-policy/darwin-arm64",
    bin:     "agent-policy",
    archive: "tar.xz",
  },
  {
    triple:  "x86_64-pc-windows-msvc",
    dir:     "npm/platforms/win32-x64",
    pkg:     "@agent-policy/win32-x64",
    bin:     "agent-policy.exe",
    archive: "zip",
  },
];

const MAIN_PKG_DIR = "npm/agent-policy";

// ---------------------------------------------------------------------------
// Args
// ---------------------------------------------------------------------------

const args = process.argv.slice(2);
const get = (flag) => {
  const i = args.indexOf(flag);
  return i !== -1 ? args[i + 1] : null;
};
const has = (flag) => args.includes(flag);

const version = get("--version");
const tag     = get("--tag") ?? "latest";
const dryRun  = has("--dry-run");

if (!version) {
  console.error("Usage: node scripts/npm-publish.mjs --version <version> [--tag <tag>] [--dry-run]");
  process.exit(1);
}

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const TMP  = join(ROOT, "tmp", "npm-publish");

log(`Publishing agent-policy v${version} to npm (tag: ${tag})${dryRun ? " [DRY RUN]" : ""}`);

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function log(msg)  { console.log(`\n▶ ${msg}`); }
function ok(msg)   { console.log(`  ✓ ${msg}`); }
function info(msg) { console.log(`  · ${msg}`); }

async function download(url, dest) {
  info(`GET ${url}`);
  const res = await fetch(url, { redirect: "follow" });
  if (!res.ok) throw new Error(`HTTP ${res.status} fetching ${url}`);
  mkdirSync(dirname(dest), { recursive: true });
  await pipeline(Readable.fromWeb(res.body), createWriteStream(dest));
}

function extract(archivePath, destDir, triple, binName, format) {
  mkdirSync(destDir, { recursive: true });
  if (format === "tar.xz") {
    // Filter must be the full in-archive path; --strip-components=1 removes the top dir
    // cargo-dist names the inner dir `agent-policy-<triple>/`
    execFileSync("tar", ["-xJf", archivePath, "--strip-components=1", "-C", destDir, `agent-policy-${triple}/${binName}`], {
      stdio: "inherit",
    });
  } else {
    // zip — Windows binary; cargo-dist zips without a top-level subdirectory
    execFileSync("tar", ["-xf", archivePath, "-C", destDir, binName], { stdio: "inherit" });
  }
}

async function bumpVersion(pkgJsonPath, ver) {
  const raw = JSON.parse(await readFile(pkgJsonPath, "utf8"));
  raw.version = ver;
  // Also update optionalDependencies versions on the main package
  if (raw.optionalDependencies) {
    for (const k of Object.keys(raw.optionalDependencies)) {
      raw.optionalDependencies[k] = ver;
    }
  }
  await writeFile(pkgJsonPath, JSON.stringify(raw, null, 2) + "\n");
  ok(`bumped ${pkgJsonPath} → ${ver}`);
}

function npmPublish(dir, npmTag) {
  const cmd = ["npm", "publish", "--access", "public", "--tag", npmTag];
  if (dryRun) cmd.push("--dry-run");
  info(`Running: ${cmd.join(" ")} (in ${dir})`);
  execSync(cmd.join(" "), { cwd: join(ROOT, dir), stdio: "inherit" });
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

log("Step 1 — bump all package.json versions");
await bumpVersion(join(ROOT, MAIN_PKG_DIR, "package.json"), version);
for (const p of PLATFORMS) {
  await bumpVersion(join(ROOT, p.dir, "package.json"), version);
}

log("Step 2 — download release binaries");
mkdirSync(TMP, { recursive: true });

for (const p of PLATFORMS) {
  const archiveFile = `agent-policy-${p.triple}.${p.archive}`;
  const downloadUrl = `https://github.com/${REPO}/releases/download/v${version}/${archiveFile}`;
  const archiveDest = join(TMP, archiveFile);

  await download(downloadUrl, archiveDest);

  log(`  extracting ${p.bin} from ${archiveFile} → ${p.dir}/`);
  const platformDir = join(ROOT, p.dir);
  extract(archiveDest, platformDir, p.triple, p.bin, p.archive);

  // Ensure executable on Unix
  if (p.archive === "tar.xz") {
    try { chmodSync(join(platformDir, p.bin), 0o755); } catch {}
  }

  ok(`${p.pkg} binary ready`);
}

log("Step 3 — publish platform packages to npm");
for (const p of PLATFORMS) {
  npmPublish(p.dir, tag);
  ok(`published ${p.pkg}@${version}`);
}

log("Step 4 — publish main package to npm");
npmPublish(MAIN_PKG_DIR, tag);
ok(`published agent-policy@${version}`);

log("Step 5 — clean up temp files");
try { rmSync(TMP, { recursive: true, force: true }); } catch {}
ok("tmp/ cleaned");

console.log(`\n✅ agent-policy@${version} published to npm${dryRun ? " (dry run — nothing actually published)" : ""}`);
