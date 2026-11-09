#!/usr/bin/env python3
"""Check production dependency isolation for supported profiles."""

import argparse
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
TARGET = "aarch64-apple-darwin"
PROFILES = [
    ("Core contracts", "aux-rs-core", ""),
    ("Facade without adapters or standard transports", "aux-rs", ""),
    ("Shared data support", "aux-rs-data", ""),
    ("Tardis adapter", "aux-rs-tardis", ""),
    ("Databento adapter", "aux-rs-databento", ""),
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
    ("Monaco with custom transports", "aux-rs", "monaco"),
    ("Monaco with standard HTTP", "aux-rs", "monaco,http"),
    ("Monaco with standard WebSockets", "aux-rs", "monaco,ws"),
    ("Monaco with both standard transports", "aux-rs", "monaco,http,ws"),
    ("All five exchanges with custom transports", "aux-rs", "hyperliquid,lighter,qfex,arcus,monaco"),
    ("All five exchanges with both standard transports", "aux-rs", "hyperliquid,lighter,qfex,arcus,monaco,http,ws"),
    ("Python binding (separate opt-in crate)", "aux-rs-python", ""),
]


for venue in ("phoenix", "extended", "derive", "paradex", "metric", "polymarket", "kalshi", "pascal"):
    for stack in ("", ",http", ",ws", ",http,ws"):
        PROFILES.append((f"{venue.title()} with {stack or 'custom transports'}", "aux-rs", venue + stack))
for stack in ("", ",http,ws"):
    PROFILES.append(("Derive and Paradex" + stack, "aux-rs", "derive,paradex" + stack))
    PROFILES.append(("All seven exchanges" + stack, "aux-rs", "hyperliquid,lighter,qfex,arcus,monaco,derive,paradex" + stack))

for stack in ("", ",http,ws"):
    PROFILES.append(("All eight exchanges" + stack, "aux-rs", "hyperliquid,lighter,qfex,arcus,monaco,derive,paradex,metric" + stack))


for vendor in ("tardis", "databento"):
    for stack in ("", ",http", ",ws", ",http,ws"):
        PROFILES.append((f"{vendor.title()} with {stack or 'custom transports'}", "aux-rs", vendor + stack))
for features in ("tardis,databento", "tardis,databento,http,ws", "metric,tardis,databento", "metric,tardis,databento,http,ws", "hyperliquid,tardis,databento", "hyperliquid,tardis,databento,http,ws", "hyperliquid,lighter,qfex,arcus,monaco,derive,paradex,metric,tardis,databento,http,ws"):
    PROFILES.append(("Data/exchange integration: " + features, "aux-rs", features))

for stack in ("", ",http,ws"):
    PROFILES.append(("All eleven exchanges"+stack,"aux-rs","hyperliquid,lighter,qfex,arcus,monaco,derive,paradex,metric,polymarket,kalshi,pascal"+stack))


for stack in ("", ",http,ws"):
    PROFILES.append(("Phoenix and Extended"+stack,"aux-rs","phoenix,extended"+stack))
    PROFILES.append(("All thirteen exchanges"+stack,"aux-rs","hyperliquid,lighter,qfex,arcus,monaco,derive,paradex,metric,polymarket,kalshi,pascal,phoenix,extended"+stack))

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
        data_selected = package in {"aux-rs-data", "aux-rs-tardis", "aux-rs-databento"} or (
            package == "aux-rs" and bool(selected & {"tardis", "databento"})
        )
        if data_selected and "tokio" not in names:
            raise SystemExit(f"Data support is missing Tokio in profile: {name}")
        if package in {"aux-rs-core", "aux-rs-data", "aux-rs-tardis", "aux-rs-databento"}:
            if names & {"reqwest", "tokio-tungstenite", "rustls"}:
                raise SystemExit(f"Standard network stack leaked into base profile: {name}")
            if not data_selected and "tokio" in names:
                raise SystemExit(f"Tokio leaked into core profile: {name}")
        exchanges = {"hyperliquid", "lighter", "qfex", "arcus", "monaco", "derive", "paradex", "metric", "polymarket", "kalshi", "pascal", "tardis", "databento", "phoenix", "extended"}
        if package == "aux-rs":
            for exchange in exchanges - selected:
                if f"aux-rs-{exchange}" in names:
                    raise SystemExit(f"Unselected {exchange} adapter leaked into profile: {name}")
            # Primitives may be owned by several selected adapters.
            owners = {
                "k256": {"hyperliquid", "monaco", "derive", "paradex", "metric", "polymarket", "extended"},
                "sha3": {"hyperliquid", "monaco", "derive", "paradex", "metric", "polymarket", "extended"},
                "rmp-serde": {"hyperliquid"},
                "rmp": {"hyperliquid", "polymarket"},
                "starknet-core": {"paradex", "extended"},
                "starknet-crypto": {"paradex", "extended"},
                "starknet-types-core": {"paradex", "extended"},
                "hmac": {"hyperliquid", "monaco", "derive", "paradex", "metric", "polymarket", "extended"},
                "rsa": {"kalshi"},
                "solana-pubkey": {"pascal", "phoenix"},
                "aux-rs-evm": {"metric", "polymarket"},
                "goldilocks-crypto": {"lighter"},
                "poseidon-hash": {"lighter"},
                "ed25519-dalek": {"arcus", "monaco", "kalshi", "pascal", "phoenix"},
                "curve25519-dalek": {"arcus", "monaco", "kalshi", "pascal", "phoenix"},
            }
            for primitive, venues in owners.items():
                if not selected & venues and primitive in names:
                    raise SystemExit(f"Unselected signing primitive {primitive} leaked into profile: {name}")
            if "databento" not in selected and names & {"dbn", "dbn-macros"}:
                raise SystemExit(f"Unselected DBN codec leaked into profile: {name}")
            if not selected & {"tardis", "databento"} and names & {"aux-rs-data", "async-compression", "compression-codecs"}:
                raise SystemExit(f"Unselected data support leaked into profile: {name}")
            if not selected & {"phoenix", "tardis", "databento"} and "zstd" in names:
                raise SystemExit(f"Unselected snapshot decompressor leaked into profile: {name}")
            if "http" not in selected and "reqwest" in names:
                raise SystemExit(f"HTTP leaked into profile: {name}")
            if "ws" not in selected and "tokio-tungstenite" in names:
                raise SystemExit(f"WebSockets leaked into profile: {name}")
            if not selected & {"http", "ws"} and names & {"reqwest", "tokio-tungstenite", "rustls"}:
                raise SystemExit(f"Standard stack leaked into custom transport profile: {name}")
            if not selected & {"http", "ws"} and not data_selected and "tokio" in names:
                raise SystemExit(f"Tokio leaked into custom transport profile: {name}")
        if package == "aux-rs" and not selected & {"metric", "polymarket"} and any(n.startswith("alloy-") for n in names):
            raise SystemExit(f"Metric ABI/EVM dependencies leaked into profile: {name}")
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
