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
for exchange in qfex arcus monaco derive paradex metric polymarket kalshi pascal; do
    cargo test --locked -p "aux-rs-$exchange" --no-default-features
    cargo test --locked -p aux-rs --no-default-features --features "$exchange"
    cargo test --locked -p aux-rs --no-default-features --features "$exchange,http"
    cargo test --locked -p aux-rs --no-default-features --features "$exchange,ws"
done
cargo check --locked -p aux-rs --no-default-features --features hyperliquid,lighter,qfex,arcus,monaco,derive,paradex,metric,polymarket,kalshi,pascal
cargo test --locked --workspace --exclude aux-rs-python --all-features
for vendor in tardis databento; do
    cargo test --locked -p "aux-rs-$vendor" --no-default-features
    cargo test --locked -p aux-rs --no-default-features --features "$vendor"
    cargo check --locked -p aux-rs --no-default-features --features "$vendor,http"
    cargo check --locked -p aux-rs --no-default-features --features "$vendor,ws"
done
cargo check --locked -p aux-rs --no-default-features --features databento,tcp
cargo check --locked -p aux-rs --no-default-features --features tardis,databento,http,ws,tcp
python3 scripts/check-data-coverage.py
python3 scripts/check-native-coverage.py
python3 scripts/check-prediction-coverage.py
python3 scripts/dependencies.py --check
