#!/usr/bin/env python3
"""Verify that the SDK versions agree with the optional release tag."""

import os
from pathlib import Path
import tomllib


def main():
    root = Path(__file__).resolve().parents[1]
    with (root / "sdk/python/pyproject.toml").open("rb") as manifest:
        python_version = tomllib.load(manifest)["project"]["version"]
    with (root / "Cargo.toml").open("rb") as manifest:
        rust_version = tomllib.load(manifest)["workspace"]["package"]["version"]
    if python_version != rust_version:
        raise SystemExit(
            f"Python version {python_version} differs from Rust version {rust_version}"
        )
    tag = os.environ.get("RELEASE_TAG")
    if tag and tag != f"aux-py-v{python_version}":
        raise SystemExit(f"Expected tag aux-py-v{python_version}, got {tag}")
    print(f"Validated aux-py version {python_version}")


if __name__ == "__main__":
    main()
