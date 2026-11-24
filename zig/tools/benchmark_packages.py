"""Time native compilation through local package creation on an idle, native host."""

import argparse
import json
import os
from pathlib import Path
import platform
import shutil
import statistics
import subprocess
import sys
import tarfile
import zipfile
import tempfile
from time import perf_counter

ROOT = Path(__file__).resolve().parents[1]
HOSTS = {
    ("Darwin", "arm64"): "aarch64-apple-darwin",
    ("Darwin", "x86_64"): "x86_64-apple-darwin",
    ("Linux", "x86_64"): "x86_64-unknown-linux-gnu",
}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    host = HOSTS[(platform.system(), platform.machine())]
    zig = shutil.which(os.environ.get("ZIG", "zig"))
    maturin = shutil.which(os.environ.get("MATURIN", "maturin"))
    if not zig or not maturin:
        parser.error("Set ZIG and MATURIN to the installed compiler and packager")
    results = []
    report = {
        "target": host,
        "os": platform.platform(),
        "zig": subprocess.check_output([zig, "version"], text=True).strip(),
        "cargo": subprocess.check_output(["cargo", "--version"], text=True).strip(),
        "node": subprocess.check_output(["node", "--version"], text=True).strip(),
        "python": platform.python_version(),
        "runs": results,
    }
    with tempfile.TemporaryDirectory() as tmp:
        work = Path(tmp)
        source = work / "zig"
        shutil.copytree(
            ROOT,
            source,
            ignore=shutil.ignore_patterns(
                "target",
                "node_modules",
                ".zig-cache",
                "zig-out",
                "dist",
                "__pycache__",
                "*.node",
            ),
        )
        node = source / "sdk/typescript"
        subprocess.run(["cargo", "fetch", "--locked"], cwd=source, check=True)
        subprocess.run(["npm", "ci", "--no-audit", "--no-fund"], cwd=node, check=True)
        connector = source / "src/derive_stream.zig"
        original = connector.read_text()
        # A protocol edit that changes generated machine code in the temporary copy.
        probe = "interval != 1000"
        assert original.count(probe) == 1
        for trial in range(3):
            for sdk in ["python", "node"] if trial % 2 == 0 else ["node", "python"]:
                with tempfile.TemporaryDirectory(dir=work) as scratch:
                    cache = Path(scratch)
                    env = dict(
                        os.environ,
                        ZIG=zig,
                        CARGO_NET_OFFLINE="true",
                        CARGO_TARGET_DIR=str(cache / "cargo"),
                        ZIG_GLOBAL_CACHE_DIR=str(cache / "global"),
                        ZIG_LOCAL_CACHE_DIR=str(cache / "local"),
                    )

                    def clock(command, cwd):
                        start = perf_counter()
                        subprocess.run(command, cwd=cwd, env=env, check=True)
                        return perf_counter() - start

                    for clean in (True, False):
                        connector.write_text(
                            original
                            if clean
                            else original.replace(probe, "interval != 2000")
                        )
                        output = cache / ("cold" if clean else "edit")
                        output.mkdir()
                        start = perf_counter()
                        core = clock(
                            [sys.executable, source / "rebuild.py", "--target", host],
                            source,
                        )
                        if sdk == "python":
                            wrapper = clock(
                                [
                                    maturin,
                                    "build",
                                    "--release",
                                    "--locked",
                                    "--manifest-path",
                                    source / "sdk/python/Cargo.toml",
                                    "--out",
                                    output,
                                ],
                                source,
                            )
                            artifact = next(output.glob("*.whl"))
                        else:
                            wrapper = clock(["npm", "run", "build"], node) + clock(
                                ["npm", "pack", "--pack-destination", output], node
                            )
                            artifact = next(output.glob("*.tgz"))
                        total = perf_counter() - start
                        if sdk == "python":
                            with zipfile.ZipFile(artifact) as package:
                                (binary,) = [
                                    m.file_size
                                    for m in package.infolist()
                                    if m.filename.endswith((".so", ".pyd"))
                                ]
                        else:
                            with tarfile.open(artifact) as package:
                                (binary,) = [
                                    m.size for m in package if m.name.endswith(".node")
                                ]
                        record = dict(
                            trial=trial + 1,
                            sdk=sdk,
                            build="cold" if clean else "edit",
                            seconds=round(total, 3),
                            core_seconds=round(core, 3),
                            wrapper_and_pack_seconds=round(wrapper, 3),
                            package_bytes=artifact.stat().st_size,
                            binding_bytes=binary,
                        )
                        results.append(record)
                        print(record, flush=True)
                        args.output.write_text(json.dumps(report, indent=2) + "\n")
        for sdk in ("python", "node"):
            for build in ("cold", "edit"):
                rows = [
                    r["seconds"]
                    for r in results
                    if r["sdk"] == sdk and r["build"] == build
                ]
                print(f"{sdk} {build} median {statistics.median(rows):.3f}s")


if __name__ == "__main__":
    main()
