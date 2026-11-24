#!/usr/bin/env python3
"""Validate a release tag against the SDK it will publish."""

import argparse
import json
import os
from pathlib import Path
import re
import tomllib

ROOT = Path(__file__).resolve().parents[1]


def package_version(sdk):
    cargo = tomllib.loads((ROOT / "Cargo.toml").read_text())
    rust = cargo["workspace"]["package"]["version"]
    if sdk == "aux-rs":
        return rust
    if sdk == "aux-py":
        project = tomllib.loads((ROOT / "sdk/python/pyproject.toml").read_text())
        version = project["project"]["version"]
        if version != rust:
            raise SystemExit(f"Python version {version} differs from Rust version {rust}")
        return version
    if sdk == "aux-ts":
        package = ROOT / "sdk/typescript"
        manifest = json.loads((package / "package.json").read_text())
        lock = json.loads((package / "package-lock.json").read_text())
        version = manifest["version"]
        if version != lock["version"] or version != lock["packages"][""]["version"]:
            raise SystemExit("npm manifest and lockfile versions differ")
        return version
    manifest = (ROOT / "build.zig.zon").read_text()
    return re.search(r'^\s*\.version\s*=\s*"([^"]+)"', manifest, re.MULTILINE)[1]


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sdk", choices=("aux-rs", "aux-ts", "aux-py", "aux-zig"))
    sdk = parser.parse_args().sdk
    version = package_version(sdk)
    expected = f"{sdk}-v{version}"
    tag = os.environ.get("RELEASE_TAG")
    if tag and tag != expected:
        raise SystemExit(f"Expected tag {expected}, got {tag}")
    print(f"Validated {sdk} version {version}")
