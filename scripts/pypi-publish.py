#!/usr/bin/env python3
"""
scripts/pypi-publish.py

Downloads prebuilt binaries from a GitHub release, assembles them into
platform-tagged Python wheels, then uploads to PyPI via twine.

Usage:
    python scripts/pypi-publish.py --version 0.7.0
    python scripts/pypi-publish.py --version 0.7.0 --dry-run
    python scripts/pypi-publish.py --version 0.7.0 --repository testpypi

Requirements:
    - Python >= 3.8 (stdlib only; no third-party dependencies besides twine)
    - twine  (`pip install twine`)
    - Set TWINE_PASSWORD=pypi-<api-token> and TWINE_USERNAME=__token__
      (or configure ~/.pypirc, or pass --username / --password to twine separately)

Notes:
    - No Rust toolchain, Docker, or maturin required.
    - Binaries are sourced from the existing GitHub release produced by cargo-dist.
    - Each wheel is a platform-tagged zip containing the binary under
      <dist>.data/scripts/, which pip/pipx/uv install onto PATH automatically.
    - python/pyproject.toml version is bumped automatically on each run.
"""

from __future__ import annotations

import argparse
import base64
import hashlib
import io
import re
import shutil
import subprocess
import sys
import tarfile
import urllib.request
import zipfile
from pathlib import Path

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------

REPO        = "CameronBrooks11/agent-policy"
PKG_NAME    = "agent-policy"   # PyPI package name (hyphen)
DIST_NAME   = "agent_policy"   # normalized wheel name (underscore)
DESCRIPTION = "Schema-first generator for coding-agent repo policies and compatibility files."
AUTHOR      = "Cameron Brooks"
AUTHOR_EMAIL = "cambrooks3393@gmail.com"
LICENSE      = "Apache-2.0"
HOMEPAGE     = "https://github.com/CameronBrooks11/agent-policy"
CHANGELOG    = "https://github.com/CameronBrooks11/agent-policy/blob/main/CHANGELOG.md"
DOCS         = "https://cameronbrooks11.github.io/agent-policy/"

# cargo-dist target triple → Python wheel platform tag + binary details
PLATFORMS = [
    {
        "triple":       "x86_64-unknown-linux-gnu",
        "platform_tag": "manylinux_2_17_x86_64",
        "bin":          "agent-policy",
        "archive":      "tar.xz",
    },
    {
        "triple":       "aarch64-unknown-linux-gnu",
        "platform_tag": "manylinux_2_17_aarch64",
        "bin":          "agent-policy",
        "archive":      "tar.xz",
    },
    {
        "triple":       "x86_64-apple-darwin",
        "platform_tag": "macosx_10_12_x86_64",
        "bin":          "agent-policy",
        "archive":      "tar.xz",
    },
    {
        "triple":       "aarch64-apple-darwin",
        "platform_tag": "macosx_11_0_arm64",
        "bin":          "agent-policy",
        "archive":      "tar.xz",
    },
    {
        "triple":       "x86_64-pc-windows-msvc",
        "platform_tag": "win_amd64",
        "bin":          "agent-policy.exe",
        "archive":      "zip",
    },
]

# ---------------------------------------------------------------------------
# Logging helpers
# ---------------------------------------------------------------------------

def log(msg: str) -> None:  print(f"\n▶ {msg}")
def ok(msg: str) -> None:   print(f"  ✓ {msg}")
def info(msg: str) -> None: print(f"  · {msg}")

# ---------------------------------------------------------------------------
# Wheel assembly helpers
# ---------------------------------------------------------------------------

def sha256_b64(data: bytes) -> str:
    """urlsafe-base64-nopad SHA-256, as required by the wheel RECORD spec."""
    digest = hashlib.sha256(data).digest()
    return base64.urlsafe_b64encode(digest).rstrip(b"=").decode()


def extract_binary(archive: Path, triple: str, bin_name: str, fmt: str) -> bytes:
    """Read raw binary bytes from a cargo-dist archive (tar.xz or zip)."""
    if fmt == "tar.xz":
        # cargo-dist puts the binary at: agent-policy-<triple>/<bin>
        inner = f"agent-policy-{triple}/{bin_name}"
        with tarfile.open(archive, "r:xz") as tf:
            fh = tf.extractfile(tf.getmember(inner))
            return fh.read()
    else:
        # Windows zip: binary sits at the archive root (no subdirectory)
        with zipfile.ZipFile(archive) as zf:
            return zf.read(bin_name)


def build_wheel(
    version: str,
    platform: dict,
    binary_data: bytes,
    out_dir: Path,
    readme_text: str,
) -> Path:
    """
    Assemble a PEP 427 wheel for a single platform.

    Wheel structure:
        {dist_tag}.data/scripts/{binary}       ← installed to PATH by pip
        {dist_tag}.dist-info/WHEEL
        {dist_tag}.dist-info/METADATA
        {dist_tag}.dist-info/RECORD
    """
    dist_tag   = f"{DIST_NAME}-{version}"
    wheel_name = f"{dist_tag}-py3-none-{platform['platform_tag']}.whl"

    # WHEEL file
    wheel_meta_content = (
        "Wheel-Version: 1.0\n"
        "Generator: agent-policy-pypi-publish\n"
        "Root-Is-Purelib: false\n"
        f"Tag: py3-none-{platform['platform_tag']}\n"
    ).encode()

    # METADATA file (Core Metadata 2.1)
    metadata_content = "\n".join([
        "Metadata-Version: 2.1",
        f"Name: {PKG_NAME}",
        f"Version: {version}",
        f"Summary: {DESCRIPTION}",
        f"Home-page: {HOMEPAGE}",
        f"Author: {AUTHOR}",
        f"Author-email: {AUTHOR_EMAIL}",
        f"License: {LICENSE}",
        "Keywords: agent,policy,codegen,cli,llm",
        "Classifier: License :: OSI Approved :: Apache Software License",
        "Classifier: Programming Language :: Rust",
        "Classifier: Topic :: Software Development :: Code Generators",
        "Classifier: Topic :: Utilities",
        "Classifier: Environment :: Console",
        "Classifier: Development Status :: 4 - Beta",
        f"Project-URL: Repository, {HOMEPAGE}",
        f"Project-URL: Changelog, {CHANGELOG}",
        f"Project-URL: Documentation, {DOCS}",
        "Requires-Python: >=3.8",
        "Description-Content-Type: text/markdown",
        "",  # blank line separates headers from body
        readme_text,
    ]).encode()

    # Zip-internal paths
    script_path = f"{dist_tag}.data/scripts/{platform['bin']}"
    wheel_path  = f"{dist_tag}.dist-info/WHEEL"
    meta_path   = f"{dist_tag}.dist-info/METADATA"
    record_path = f"{dist_tag}.dist-info/RECORD"

    # RECORD rows — every file except RECORD itself
    def row(path: str, data: bytes) -> str:
        return f"{path},sha256={sha256_b64(data)},{len(data)}"

    record_content = "\n".join([
        row(script_path, binary_data),
        row(wheel_path,  wheel_meta_content),
        row(meta_path,   metadata_content),
        f"{record_path},,",   # RECORD does not hash itself
    ]).encode()

    # Build the zip
    buf = io.BytesIO()
    with zipfile.ZipFile(buf, "w", zipfile.ZIP_DEFLATED) as zf:
        zf.writestr(script_path, binary_data)
        zf.writestr(wheel_path,  wheel_meta_content)
        zf.writestr(meta_path,   metadata_content)
        zf.writestr(record_path, record_content)

    dest = out_dir / wheel_name
    dest.write_bytes(buf.getvalue())
    return dest


def update_pyproject_version(root: Path, version: str) -> None:
    """Bump the version field in python/pyproject.toml."""
    path = root / "python" / "pyproject.toml"
    text = path.read_text(encoding="utf-8")
    updated = re.sub(
        r'^(version\s*=\s*")[^"]*(")',
        rf"\g<1>{version}\g<2>",
        text,
        count=1,
        flags=re.MULTILINE,
    )
    if updated == text:
        raise RuntimeError("version field not found in python/pyproject.toml")
    path.write_text(updated, encoding="utf-8")
    ok(f"bumped python/pyproject.toml → {version}")


def download(url: str, dest: Path) -> None:
    info(f"GET {url}")
    dest.parent.mkdir(parents=True, exist_ok=True)
    with urllib.request.urlopen(url) as resp, dest.open("wb") as out:
        shutil.copyfileobj(resp, out)


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

def main() -> None:
    parser = argparse.ArgumentParser(
        description="Assemble and publish agent-policy wheels to PyPI.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__,
    )
    parser.add_argument("--version",    required=True,
                        help="Release version to publish, e.g. 0.7.0")
    parser.add_argument("--dry-run",    action="store_true",
                        help="Build wheels but skip twine upload")
    parser.add_argument("--repository", default="pypi",
                        help="twine --repository target (default: pypi; use 'testpypi' to test)")
    args = parser.parse_args()

    version    = args.version
    dry_run    = args.dry_run
    repository = args.repository

    root = Path(__file__).resolve().parent.parent
    tmp  = root / "tmp" / "pypi-publish"
    tmp.mkdir(parents=True, exist_ok=True)

    readme_text = (root / "README.md").read_text(encoding="utf-8") \
                  if (root / "README.md").exists() else ""

    print(
        f"\nagent-policy v{version} → PyPI"
        f"  (repository: {repository})"
        f"{' [DRY RUN]' if dry_run else ''}"
    )

    # ------------------------------------------------------------------
    log("Step 1 — bump python/pyproject.toml version")
    update_pyproject_version(root, version)

    # ------------------------------------------------------------------
    log("Step 2 — download release binaries and assemble wheels")
    wheels: list[Path] = []

    for platform in PLATFORMS:
        triple = platform["triple"]
        fmt    = platform["archive"]
        name   = f"agent-policy-{triple}.{fmt}"
        url    = f"https://github.com/{REPO}/releases/download/v{version}/{name}"
        arc    = tmp / name

        download(url, arc)

        info(f"extracting {platform['bin']} from {name}")
        binary_data = extract_binary(arc, triple, platform["bin"], fmt)

        info(f"assembling wheel  py3-none-{platform['platform_tag']}")
        wheel = build_wheel(version, platform, binary_data, tmp, readme_text)
        wheels.append(wheel)
        ok(f"built {wheel.name}  ({wheel.stat().st_size // 1024} KB)")

    # ------------------------------------------------------------------
    log(f"Step 3 — {'[dry-run] skipping upload' if dry_run else 'upload wheels via twine'}")

    if dry_run:
        for w in wheels:
            info(f"would upload: {w.name}")
    else:
        cmd = [
            sys.executable, "-m", "twine", "upload",
            "--repository", repository,
            "--non-interactive",
        ] + [str(w) for w in wheels]
        info("twine upload " + " ".join(w.name for w in wheels))
        subprocess.run(cmd, check=True)
        ok(f"uploaded {len(wheels)} wheel(s) to PyPI")

    # ------------------------------------------------------------------
    log("Step 4 — clean up temp files")
    shutil.rmtree(tmp, ignore_errors=True)
    ok("tmp/ cleaned")

    print(
        f"\n{'✅' if not dry_run else '☑  (dry run — nothing uploaded)'} "
        f"agent-policy {version} "
        f"{'published to PyPI' if not dry_run else 'wheels assembled OK'}"
    )


if __name__ == "__main__":
    main()
