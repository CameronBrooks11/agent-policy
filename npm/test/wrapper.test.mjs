// Tests for npm/agent-policy/bin/agent-policy.js. Run: node --test npm/test/
//
// Each test installs the wrapper and a stand-in platform package into a temp
// node_modules tree, so require.resolve finds the stand-in binary.

import { test } from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const WRAPPER = join(dirname(fileURLToPath(import.meta.url)), "..", "agent-policy", "bin", "agent-policy.js");
const unix = process.platform !== "win32";

function install(t, writeBin) {
  const root = mkdtempSync(join(tmpdir(), "agent-policy-wrapper-"));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  const nm = join(root, "node_modules");
  const wrapper = join(nm, "agent-policy", "bin", "agent-policy.js");
  mkdirSync(dirname(wrapper), { recursive: true });
  copyFileSync(WRAPPER, wrapper);
  const pkgDir = join(nm, "@cameronbrooks11", `agent-policy-${process.platform}-${process.arch}`);
  mkdirSync(pkgDir, { recursive: true });
  const bin = join(pkgDir, "agent-policy");
  writeBin(bin);
  return { wrapper, bin };
}

function run(wrapper, ...args) {
  return spawnSync(process.execPath, [wrapper, ...args], { encoding: "utf8" });
}

test("repairs a binary shipped without the execute bit and runs it", { skip: !unix }, (t) => {
  const { wrapper, bin } = install(t, (p) =>
    writeFileSync(p, '#!/bin/sh\necho "ran $1"\n', { mode: 0o644 }));

  const r = run(wrapper, "--version");

  assert.equal(r.status, 0, r.stderr);
  assert.equal(r.stdout, "ran --version\n");
  assert.notEqual(statSync(bin).mode & 0o111, 0);
});

test("passes the binary's exit status through", { skip: !unix }, (t) => {
  const { wrapper } = install(t, (p) =>
    writeFileSync(p, "#!/bin/sh\nexit 3\n", { mode: 0o755 }));

  assert.equal(run(wrapper).status, 3);
});

test("reports a binary it cannot run instead of exiting silently", { skip: !unix }, (t) => {
  // The spawn fails before the binary runs, so the error carries no exit
  // status: the case 0.6.0 turned into a bare exit 1 (#20).
  const { wrapper, bin } = install(t, (p) =>
    writeFileSync(p, "#!/nonexistent/interpreter\n", { mode: 0o755 }));

  const r = run(wrapper);

  assert.equal(r.status, 1);
  assert.match(r.stderr, /^agent-policy: failed to run .+: .+/);
  assert.ok(r.stderr.includes(bin));
});
