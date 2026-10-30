#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

cargo fmt --all --check
cargo clippy --locked --workspace --exclude aux-rs-python --all-features --all-targets -- -D warnings
cargo check --locked -p aux-rs-core --no-default-features
cargo check --locked -p aux-rs --no-default-features
cargo check --locked -p aux-rs
cargo test --locked -p aux-rs-hyperliquid --no-default-features
cargo test --locked -p aux-rs --no-default-features --features hyperliquid
cargo test --locked -p aux-rs --no-default-features --features hyperliquid,http
cargo test --locked -p aux-rs --no-default-features --features hyperliquid,ws
cargo test --locked -p aux-rs-lighter --no-default-features
cargo test --locked -p aux-rs --no-default-features --features lighter
cargo test --locked -p aux-rs --no-default-features --features lighter,http
cargo test --locked -p aux-rs --no-default-features --features lighter,ws
cargo check --locked -p aux-rs --no-default-features --features hyperliquid,lighter
cargo test --locked --workspace --exclude aux-rs-python --all-features
python3 scripts/dependencies.py --check
