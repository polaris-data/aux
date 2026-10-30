#!/usr/bin/env python3
"""Explicit development oracle; Go and network access are never required by CI.

Generate: python3 scripts/lighter_vectors.py [--reference path/to/lighter-go]
Verify Rust output: add --verify-rust path/to/signatures.json
All private keys and Schnorr nonces in this oracle are public test data.
"""
import argparse
import hashlib
import json
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REVISION = "ebae8edecfee9986b0a5fcde3cb9a06bd4af3a3a"
REPO = "https://github.com/elliottech/lighter-go"


def run(reference, verify):
    actual = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=reference, text=True).strip()
    if actual != REVISION:
        raise SystemExit(f"Expected reference {REVISION}, got {actual}")
    if subprocess.check_output(["git", "diff", "HEAD", "--"], cwd=reference):
        raise SystemExit("Reference has tracked modifications")
    command = ["go", "run", str(ROOT / "scripts/lighter_reference.go")]
    if verify:
        subprocess.run(command + [str(verify.resolve())], cwd=reference, check=True)
        return
    vectors = json.loads(subprocess.check_output(command, cwd=reference))
    sources = ["go.mod", "go.sum", "signer/key_manager.go", "types/tx_request.go",
               "types/txtypes/create_order.go", "types/txtypes/cancel_order.go",
               "types/txtypes/tx_attributes.go", "types/txtypes/constants.go"]
    hashes = {p: hashlib.sha256((reference / p).read_bytes()).hexdigest() for p in sources}
    result = {"repository": REPO, "revision": REVISION, "poseidon_crypto": "v0.0.15",
              "source_sha256": hashes, "vectors": vectors}
    target = ROOT / "crates/exchanges/lighter/tests/fixtures/signing_vectors.json"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(result, indent=2) + "\n")
    print(f"Recorded {len(vectors)} vectors from pinned Go reference")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--reference", type=Path)
    parser.add_argument("--verify-rust", type=Path)
    args = parser.parse_args()
    if args.reference:
        run(args.reference.resolve(), args.verify_rust)
    else:
        with tempfile.TemporaryDirectory() as directory:
            reference = Path(directory) / "reference"
            subprocess.run(["git", "clone", "--no-checkout", REPO, str(reference)], check=True)
            subprocess.run(["git", "checkout", "--detach", REVISION], cwd=reference, check=True)
            run(reference, args.verify_rust)


if __name__ == "__main__":
    main()
