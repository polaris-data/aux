#!/usr/bin/env python3
"""Check production dependency isolation for supported profiles."""

import argparse
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
TARGET = "aarch64-apple-darwin"
PROFILES = [
    ("Core contracts", "aux-rs-core", ""),
    ("Hyperliquid with custom transports", "aux-rs", "hyperliquid"),
    ("Hyperliquid with standard HTTP", "aux-rs", "hyperliquid,http"),
    ("Hyperliquid with standard WebSockets", "aux-rs", "hyperliquid,ws"),
    ("Hyperliquid with both standard transports", "aux-rs", "hyperliquid,http,ws"),
    ("Lighter with custom transports", "aux-rs", "lighter"),
    ("Lighter with standard HTTP", "aux-rs", "lighter,http"),
    ("Lighter with standard WebSockets", "aux-rs", "lighter,ws"),
    ("Lighter with both standard transports", "aux-rs", "lighter,http,ws"),
    ("Hyperliquid and Lighter with custom transports", "aux-rs", "hyperliquid,lighter"),
    ("Hyperliquid and Lighter with both standard transports", "aux-rs", "hyperliquid,lighter,http,ws"),
    ("QFEX with custom transports", "aux-rs", "qfex"),
    ("QFEX with standard HTTP", "aux-rs", "qfex,http"),
    ("QFEX with standard WebSockets", "aux-rs", "qfex,ws"),
    ("QFEX with both standard transports", "aux-rs", "qfex,http,ws"),
    ("Arcus with custom transports", "aux-rs", "arcus"),
    ("Arcus with standard HTTP", "aux-rs", "arcus,http"),
    ("Arcus with standard WebSockets", "aux-rs", "arcus,ws"),
    ("Arcus with both standard transports", "aux-rs", "arcus,http,ws"),
    ("QFEX and Arcus with custom transports", "aux-rs", "qfex,arcus"),
    ("All four exchanges with custom transports", "aux-rs", "hyperliquid,lighter,qfex,arcus"),
    ("All four exchanges with both standard transports", "aux-rs", "hyperliquid,lighter,qfex,arcus,http,ws"),
    ("Python binding (separate opt-in crate)", "aux-rs-python", ""),
]


def inventory(package, features, target):
    command = [
        "cargo", "tree", "--locked", "--package", package,
        "--no-default-features", "--target", target,
        "--edges", "normal,build", "--prefix", "none", "--format", "{p}",
    ]
    if features:
        command += ["--features", features]
    output = subprocess.check_output(command, cwd=ROOT, text=True)
    # Strip workspace paths and repeated-node annotations, but keep versions.
    packages = {line.split(" (")[0] for line in output.splitlines() if line.strip()}
    return sorted(p for p in packages if not p.startswith(package + " "))


def check(target):
    for name, package, features in PROFILES:
        packages = inventory(package, features, target)
        names = {dependency.split()[0] for dependency in packages}
        if "httpdate" in names:
            raise SystemExit(f"Replaced HTTP-date dependency leaked into profile: {name}")
        selected = set(features.split(","))
        exchanges = {"hyperliquid", "lighter", "qfex", "arcus"}
        if package == "aux-rs":
            for exchange in exchanges - selected:
                if f"aux-rs-{exchange}" in names:
                    raise SystemExit(f"Unselected {exchange} adapter leaked into profile: {name}")
            exclusive = {
                "hyperliquid": {"k256", "sha3", "rmp-serde", "hmac"},
                "lighter": {"goldilocks-crypto", "poseidon-hash"},
                "arcus": {"ed25519-dalek", "curve25519-dalek"},
            }
            for exchange, primitives in exclusive.items():
                if exchange not in selected and names & primitives:
                    raise SystemExit(f"Unselected {exchange} crypto leaked into profile: {name}")
            if not selected & {"http", "ws"} and names & {"tokio", "reqwest", "tokio-tungstenite", "rustls"}:
                raise SystemExit(f"Standard stack leaked into custom transport profile: {name}")
        if package != "aux-rs-python" and any(
            dependency.split()[0].startswith("pyo3")
            or dependency.split()[0] == "aux-rs-python"
            for dependency in packages
        ):
            raise SystemExit(f"Python dependencies leaked into Rust-only profile: {name}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", required=True)
    parser.add_argument("--target", default=TARGET)
    args = parser.parse_args()
    check(args.target)
    print(f"Production dependency isolation checks passed for {args.target}.")


if __name__ == "__main__":
    main()
