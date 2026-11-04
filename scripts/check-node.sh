#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
npm ci --prefix crates/node
cargo fmt --all --check
python3 scripts/generate-node-bindings.py --check --models
python3 scripts/generate-metric-bindings.py --check
python3 scripts/check-node-coverage.py
cargo clippy --locked -p aux-rs-node --all-targets --all-features -- -D warnings
cargo test --locked -p aux-rs-node --test values
npm --prefix crates/node run build
npm --prefix crates/node run typecheck
npm --prefix crates/node test
node scripts/check-node-package.mjs
