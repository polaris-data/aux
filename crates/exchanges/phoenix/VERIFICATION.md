# Phoenix verification

Reference date: 2026-11-09. Scope is Phoenix perpetuals at the revisions in
[API_SOURCES.json](API_SOURCES.json), including native collateral and Flight
instructions. Legacy Phoenix spot, risk-authority liquidation, onboarder service
administration, strategies, state stores, and automatic funding are excluded.

## Offline coverage

- All 66 inventoried REST operations have typed request/response contract fixtures
  and Rust, Python and Node HTTP binding checks. Unknown fields, exact fractional
  JSON and large identifiers survive decoding.
- All nine native channels have subscription and frame fixtures. Local WebSocket
  tests cover reconnect, sequence gaps and fresh baselines. Shared core transport
  tests cover bounded queues, overflow, close and shutdown.
- All 67 client instruction builders have request, account-meta and byte fixtures
  exercised through Rust, Python and Node. The separate published Rise instruction
  vectors verify independent cross-language bytes. Synthetic dispatch fixtures are
  regression checks, not evidence of live execution.
- Transaction tests cover legacy/v0 compilation, caller-reviewed lookup tables,
  all required signatures, authority/fee-payer identity, server-message validation,
  tampering, packet limits, blockhash context, concurrent preparation, simulation,
  submission ambiguity, cancellation and explicit reconciliation.
- Private Rust methods require authentication. Credentials are redacted and
  replaceable. Native rejections inside HTTP 200 responses retain their details;
  state-changing submissions are not automatically retried.

Run the repository checks from its root:

```sh
bash scripts/check.sh
AUX_PYTHON=/absolute/path/to/venv/bin/python bash scripts/check-python.sh
bash scripts/check-node.sh
```

Coverage checks also verify source/fixture hashes, operation parity and generated
instruction dispatch. Feature profiles cover this venue alone, both new venues,
HTTP-only, WebSocket-only, injected transports and existing adapter combinations.
Production dependency checks exclude Python, Node and unselected venue stacks.

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

The Rust public mainnet example received market metadata, a subscription
acknowledgement and native order-book data on the reference date. The markets
endpoint returned HTTP 200.
Its first native market is retained as `tests/fixtures/public_markets.json` with
its source and digest pinned. This captured response is decoded in all languages;
string-encoded numeric fields are retained as strings.

Public smoke example (market metadata and one WebSocket frame; no credentials):

```sh
cargo run --locked -p aux-rs --no-default-features --features phoenix,http,ws --example phoenix_public
```

No testnet deployment is assumed. Custom deployments require explicit endpoints
and chain context.

Authenticated API, funded Solana execution and exchange fills have **not** been
verified live. The `phoenix_transaction` example requires caller-supplied RPC,
instructions, fee payer and signer. It prepares and simulates by default; only
`AUX_PHOENIX_SUBMIT=1` permits submission, and CI refuses that mode. Submission
requires explicit reconciliation and is separate from exchange execution.
