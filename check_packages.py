"""Build and install the local SDKs in empty environments and run their fixtures."""

import json
import os
import re
import shutil
from pathlib import Path
from tests.fixture_peer import fixture_peer
import subprocess
import sys
import tempfile
import venv

ROOT = Path(__file__).resolve().parent


def run(*args, cwd=ROOT, env=None):
    subprocess.run([str(x) for x in args], cwd=cwd, env=env, check=True)


def rust_consumer(work, crate, fixtures_env):
    run("tar", "-xf", crate, cwd=work)
    project = work / "rust-consumer"
    (project / "src").mkdir(parents=True)
    (project / "Cargo.toml").write_text(
        '[package]\nname = "sdk-consumer"\nversion = "0.0.0"\nedition = "2024"\n'
        f'[dependencies]\nserde_json = "1"\naux-rs = {{ path = "../{crate.stem}" }}\n'
    )
    entry = project / "src/main.rs"
    shutil.copy2(ROOT / "tests/consumer.rs", entry)
    for name in set(re.findall(r'include_(?:str|bytes)!\("\.\./([\w.\-]+)"\)', entry.read_text())):
        shutil.copy2(ROOT / "tests" / name, project / name)
    env = {**os.environ, "ZIG": str(work / "zig-unavailable"), **fixtures_env}
    run("cargo", "run", "--offline", cwd=project, env=env)
    manifest = project / "Cargo.toml"
    with manifest.open("a") as cargo:
        cargo.write('tokio = { version = "1", features = ["rt", "macros", "time"] }\n')
    shutil.copy2(ROOT / "tests/async_consumer.rs", entry)
    run("cargo", "run", "--offline", "--features", "aux-rs/async", cwd=project, env=env)


def main():
    run(sys.executable, ROOT / "tools/check_coverage.py")
    rust = ROOT / "sdk/rust/Cargo.toml"
    node = ROOT / "sdk/typescript"
    run("cargo", "test", "--locked", "--all-features", "--manifest-path", ROOT / "Cargo.toml")
    run(
        "cargo",
        "package",
        "-p",
        "aux-rs",
        "--allow-dirty",
        "--locked",
        "--manifest-path",
        rust,
    )
    run("npm", "ci", "--prefix", node, "--no-audit", "--no-fund")
    run("npm", "run", "build", cwd=node)
    run(
        node / "node_modules/.bin/tsc",
        "--strict",
        "--noEmit",
        "--target",
        "es2022",
        node / "js/index.d.ts",
    )
    with tempfile.TemporaryDirectory() as tmp, fixture_peer() as (origin, gateway):
        fixtures_env = {"AUX_FIXTURE_ORIGIN": origin, "AUX_DBN_GATEWAY": gateway}
        work = Path(tmp)
        run("npm", "pack", "--pack-destination", work, cwd=node)
        run(
            os.getenv("MATURIN", "maturin"),
            "build",
            "--release",
            "--locked",
            "--manifest-path",
            ROOT / "sdk/python/Cargo.toml",
            "--out",
            work,
        )
        metadata = json.loads(
            subprocess.check_output(
                ["cargo", "metadata", "--no-deps", "--format-version", "1"],
                cwd=ROOT,
                text=True,
            )
        )
        version = next(
            p["version"] for p in metadata["packages"] if p["name"] == "aux-rs"
        )
        crate = (
            Path(metadata["target_directory"]) / "package" / f"aux-rs-{version}.crate"
        )
        rust_consumer(work, crate, fixtures_env)
        clean = work / "consumer"
        clean.mkdir()
        venv.EnvBuilder(with_pip=True).create(clean / "venv")
        python = clean / "venv/bin/python"
        run(
            python,
            "-m",
            "pip",
            "install",
            "--no-index",
            "--only-binary=:all:",
            next(work.glob("*.whl")),
            cwd=clean,
        )
        run(
            "npm",
            "install",
            "--offline",
            "--ignore-scripts",
            "--no-audit",
            "--no-fund",
            next(work.glob("*.tgz")),
            cwd=clean,
        )
        shutil.copy2(ROOT / "tests/consumer.ts", clean / "consumer.ts")
        run(node / "node_modules/.bin/tsc", "--strict", "--noEmit", "--target", "es2022",
            "--module", "node16", "--moduleResolution", "node16", "consumer.ts", cwd=clean)
        run(python, "-I", ROOT / "tests/packages.py", cwd=clean, env={**os.environ, **fixtures_env})
        run("node", ROOT / "tests/packages.cjs", cwd=clean, env={**os.environ, **fixtures_env})
        run(
            "node",
            "--input-type=module",
            "--eval",
            'import { decodeResponse, PhoenixClient } from "aux-connectors";'
            'if (decodeResponse("extended",200,"123456789123456789") !== 123456789123456789n'
            ' || typeof new PhoenixClient().subscribe !== "function") process.exit(1);',
            cwd=clean,
        )
        dist = ROOT / "dist"
        dist.mkdir(exist_ok=True)
        for package in [crate, *work.glob("*.whl"), *work.glob("*.tgz")]:
            shutil.copy2(package, dist / package.name)


if __name__ == "__main__":
    main()
