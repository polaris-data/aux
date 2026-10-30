# Contributing

## Workspace

Requires Rust 1.92 or later. Run commands below from the repository root, which
contains the Cargo workspace configuration and committed lockfile.

| Directory | Purpose |
| --- | --- |
| `crates/aux-rs/` | Public SDK facade, examples, and integration tests |
| `crates/core/` | Shared client and transport contracts |
| `crates/exchanges/` | Native exchange clients, types, and signing |
| `crates/python/` | Separate Python bindings |

Read the [architecture](PLANNING.md) and [domain terminology](CONTEXT.md) before
changing shared interfaces. Exchange guides are linked from the
[SDK features](README.md#features); recovery and lifecycle contracts live in
the [transport guide](HARDENING.md).

## Rust checks

```sh
bash scripts/check.sh
```

This runs formatting, Clippy with warnings denied, feature-isolated compilation
and tests, offline fixture and local-server tests, and dependency-baseline checks.
It also checks that unsigned clients cannot trade. Normal checks make no exchange
requests and place no orders.

Rust-only workspace defaults and this script exclude the Python crate. PyO3 must
remain absent from every Rust-only dependency profile, and exchange features must
not pull in unselected adapters or their signing dependencies.

## Python checks

Use a dedicated Python 3.10+ virtual environment. For example, with Python 3.12:

```sh
python3.12 -m venv .venv-python
.venv-python/bin/pip install -r crates/python/dev-requirements.txt
bash scripts/check-python.sh
```

The script runs formatting, Python-crate Clippy, a local Maturin development
build, Python tests, and dependency checks. It uses fixtures and local mock
servers. Set `AUX_PYTHON=/path/to/venv/bin/python` to select another environment.

See the [Python guide](crates/python/README.md) for API coverage, examples, and
platform limits. CPython 3.12.7 on macOS arm64 has been validated; release wheels
and a broader interpreter/platform matrix remain separate work.

## Dependency changes

Inspect feature activation and regenerate the production baseline after changing
dependencies:

```sh
cargo tree -p aux-rs --features hyperliquid -e features
python3 scripts/dependencies.py --write
```

Review the [baseline diff](DEPENDENCIES.md) before committing it. The inventory
records its target explicitly; use `--target` to inspect another platform.
Profiles cover core, each exchange, exchange combinations, custom and standard
transports, and the separate Python binding. Automated checks reject
cross-exchange signing and Rust-only Python dependency leaks.

Core has zero dependencies without features. Native identifiers and signatures
use shared hex helpers; Lighter transaction signatures use a private Base64
encoder. QFEX's private HMAC-SHA256 helper uses sha2, and standard HTTP uses a
private HTTP-date parser for `Retry-After`. These implementations record pinned
upstream sources, protocol references, and license notices. The inventory also
records transitive dependencies, including Reqwest's Tower and tracing crates.

## Live verification

Public testnet examples are explicit read-only network checks, separate from
the normal scripts. See the [examples](crates/aux-rs/examples) and exchange guides
for commands and supported endpoints.

Authenticated trading examples require registered, funded testnet accounts and
an explicit `--execute` flag. Funded authenticated submit/cancel verification
remains pending for all exchanges. Lighter's local signing primitives remain
experimental and unaudited; see its [signing notes](crates/exchanges/lighter/SIGNING.md).
