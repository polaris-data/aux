"""Rebuild bundled archives with the release Zig compiler. Run from any directory."""

import argparse
import os
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory

ROOT = Path(__file__).resolve().parent
TARGETS = {
    "aarch64-apple-darwin": "aarch64-macos.11.0",
    "x86_64-apple-darwin": "x86_64-macos.10.12",
    "x86_64-unknown-linux-gnu": "x86_64-linux-gnu.2.28",
}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target", choices=TARGETS)
    parser.add_argument(
        "--check",
        action="store_true",
        help="check that the packaged archives match the sources",
    )
    args = parser.parse_args()
    zig = os.getenv("ZIG", "zig")
    if subprocess.check_output([zig, "version"], text=True).strip() != "0.17.1":
        parser.error("Zig 0.17.1 is required")
    for target in [args.target] if args.target else TARGETS:
        dest = ROOT / "sdk" / "rust" / "prebuilt" / target
        dest.mkdir(parents=True, exist_ok=True)
        with TemporaryDirectory(dir=dest) as work:
            artifact = Path(work) / "libaux_zig_core.a"
            obj = Path(work) / "aux_zig_core.o"
            subprocess.run(
                [
                    zig,
                    "build-obj",
                    "src/root.zig",
                    "-lc",
                    "-fPIC",
                    "-fcompiler-rt",
                    "-O",
                    "ReleaseSafe",
                    "-fstrip",
                    "-target",
                    TARGETS[target],
                    "-mcpu",
                    "baseline",
                    "-femit-bin=" + str(obj),
                ],
                cwd=ROOT,
                check=True,
            )
            subprocess.run(
                [zig, "ar", "rcsD", artifact.name, obj.name], cwd=work, check=True
            )
            packaged = dest / artifact.name
            if args.check:
                if (
                    not packaged.is_file()
                    or artifact.read_bytes() != packaged.read_bytes()
                ):
                    parser.exit(1, f"stale native archive: {packaged}\n")
            else:
                artifact.replace(packaged)


if __name__ == "__main__":
    main()
