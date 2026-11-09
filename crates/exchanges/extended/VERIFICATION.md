# Extended verification

Reference date: 2026-11-09. [API_SOURCES.json](API_SOURCES.json) pins the official
SDK, partial OpenAPI snapshot, documentation and focused crypto reference.
[COVERAGE.md](COVERAGE.md) inventories the combined client-facing surface.

## Offline coverage

- All 123 inventoried REST operations have typed contract fixtures and Rust,
  Python and Node HTTP binding checks. This includes perpetual/spot orders, RFQ,
  onboarding, administration, transfer/withdrawal, vault and analytics APIs.
- All eight URL streams and eight RPC subscription contracts have fixtures.
  Local servers exercise reconnect, native snapshots, sequence gaps and fresh
  baselines. RPC tests reject unrelated request/topic IDs and verify native ping
  replies. Core transport tests exercise bounded buffers and shutdown.
- Official reference vectors cover Stark settlement hashes, domain separation,
  EVM registration/key-derivation/API-request signatures, onboarding L2 signatures
  and wallet-to-Stark key derivation. Exact amount tests cover quantity precision,
  side-dependent collateral rounding, fee rounding and independent TP/SL legs.
- Authentication, mandatory User-Agent headers, native rejections in HTTP 200,
  malformed inputs and ambiguous write outcomes are tested. Accepted submissions
  preserve native IDs; they are not represented as confirmed working orders.
- Signing and native instruction/protocol work stays in Rust. The official SDK
  is a pinned reference, not a runtime dependency.

Run the repository checks from its root:

```sh
bash scripts/check.sh
AUX_PYTHON=/absolute/path/to/venv/bin/python bash scripts/check-python.sh
bash scripts/check-node.sh
```

The feature/dependency matrix covers this venue alone, both new venues, HTTP-only,
WebSocket-only, custom transports and existing adapters. Ordinary builds/checks
use committed generated artifacts and do not import the official Python SDK.

## Resolved source discrepancies

The SDK routes/models and documentation supplement the explicitly partial schema.
Operation records retain their source. Origin-relative onboarding routes and L1
signature headers follow the SDK. Live numeric strings are accepted alongside SDK
integer declarations without coercion. RFQ uses a dedicated submission model with
optional `rfqStartPrice`, which is excluded from settlement hashing. URL candle
names differ from RPC selectors; both native forms are retained. Network signing
domains remain explicit and do not follow endpoint hostname changes.

## Check results

On 2026-11-09, on `aarch64-apple-darwin`:

| Check | Result |
| --- | --- |
| Full Rust check suite, workspace tests and feature/dependency matrix | Passed |
| Python check suite | 111 tests passed; coverage and isolation checks passed |
| Node check suite | 33 tests passed; TypeScript and installed ESM/CommonJS package checks passed |
| Both new venues together, HTTP-only and WebSocket-only | Passed |
| Final adapter/example clippy checks with warnings denied | Passed |

## Live verification

The testnet markets endpoint returned HTTP 200 on the reference date. Its first
native market is retained as `tests/fixtures/public_markets.json` and decoded in
all languages. The testnet book WebSocket handshake returned HTTP 503; this does
not establish live testnet stream availability. The Rust public mainnet example received market metadata and a native book
snapshot. The Rust v2 RPC example also received `ORDERBOOKS.SNAPSHOT`; captured
acknowledgement/snapshot/delta envelopes are pinned in `rpc_public.json`. Both
URL and RPC mainnet stream surfaces passed public verification.

Public smoke examples require no credentials and never place orders:

```sh
cargo run --locked -p aux-rs --no-default-features --features extended,http,ws --example extended_public
AUX_EXTENDED_NETWORK=mainnet cargo run --locked -p aux-rs --no-default-features --features extended,http,ws --example extended_public
AUX_EXTENDED_NETWORK=mainnet AUX_EXTENDED_RPC=1 cargo run --locked -p aux-rs --no-default-features --features extended,http,ws --example extended_public
```

Both documented mainnet/testnet configurations are exposed. Authenticated routes,
funded trading, transfers and withdrawals have **not** been verified live.
`extended_testnet_trade` requires an explicit opt-in plus caller-supplied API key,
signer, settlement context and order body; it refuses CI/mainnet execution.
REST acceptance remains distinct from confirmation through account updates.
