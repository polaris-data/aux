#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

binding_python="${AUX_PYTHON:-$PWD/.venv-python/bin/python}"
if [ ! -x "$binding_python" ]; then
    echo "Create .venv-python and install crates/python/dev-requirements.txt first." >&2
    exit 1
fi
binding_venv="$("$binding_python" -c 'import sys; assert sys.prefix != sys.base_prefix, "use a virtual environment"; print(sys.prefix)')"
export PYO3_PYTHON="$binding_python"

cargo fmt --all --check
cargo clippy --locked -p aux-rs-python --all-targets -- -D warnings
VIRTUAL_ENV="$binding_venv" "$binding_python" -m maturin develop --locked --manifest-path crates/python/Cargo.toml
"$binding_python" -m unittest discover -s crates/python/tests -v
"$binding_python" scripts/check-data-coverage.py
"$binding_python" scripts/check-native-coverage.py
"$binding_python" scripts/check-prediction-coverage.py
"$binding_python" scripts/dependencies.py --check
