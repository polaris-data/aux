# Kalshi

Enable `aux-rs` with `kalshi` for native models, codecs and injected I/O. Add `http` and/or `ws` only when standard networking is needed; exchange selection alone does not enable a runtime or socket stack. Constructors accept explicit deployment environments and custom endpoint maps. `with_transport` accepts caller-owned I/O. Standard clients expose their transport for deterministic `shutdown`.

`KalshiClient` exposes predictions in `kalshi::predictions`; `KalshiPerpsClient` exposes distinct `perps` and clearing-member `clearing` models. Predictions retain yes/no bid books and fixed-point strings. No complement asks or synthetic perps models are introduced. Production and demo are explicit configurations (`Kalshi::mainnet()` / `demo()`). Override endpoint maps, including `predictions_warp` and `perps_warp`, for dedicated fill/order hosts.

Use `KalshiAuth::new(key_id, signer)` with RSA PKCS8/PKCS1 or Ed25519 PKCS8 keys (`LocalSigner::from_pem`), or implement `RequestSigner`. RSA uses PSS/SHA-256 with 32-byte salt; Ed25519 signs the same native message. The preimage is timestamp milliseconds + uppercase verb + full URL path, excluding the query. WebSocket headers sign the actual upgrade path on every dial. Clearing-member bearer endpoints use `KalshiBearerAuth`, separately from request-signing credentials. API roles and institutional access are server permissions; fixture success does not establish entitlement.

Streams use `Subscription { service: "predictions".into(), params: json!({"channel":"orderbook_delta","market_tickers":["TICKER"]}) }`. `open_rpc()` exposes subscribe/unsubscribe/list/update controls with response correlation. Native sid/sequence gaps invalidate the session and surface a gap before fresh snapshots. FIX is outside this connector.

## Requests, values and Python

Every REST operation has a named async method; the inventory maps `215` REST operations and `75` WebSocket message contracts. Rust request types are under the service namespace and end in `Params`. Path/query/header fields are at the top level and JSON request bodies are under `body`. Unknown response fields and explicit nulls are retained. Native numeric strings remain strings; JSON numbers use arbitrary precision rather than binary floating point.

Python exports matching clients and local/async callback signers from `aux_py`. Use `async with Client(...) as client` or `await client.aclose()`. Constructors accept `network`, `endpoints`, `websocket`, timeout and bounded subscription capacity. Requests are native mappings with the same `body` envelope as Rust. Python floats are rejected recursively; use `Decimal`, integers or native strings. Responses preserve JSON integers, decode fractions to `Decimal`, and keep string-encoded decimals as strings. Native errors carry venue/status/code/body fields. `Subscription` supports async iteration and explicit close; parent shutdown cancels children. Command helpers live in `aux_py._prediction_rpc` and are returned by `open_rpc()`.

Writes execute once. Transport errors with unknown outcomes require explicit caller reconciliation against order IDs, transaction hashes or native history. Public idempotent reads may use the bounded standard retry policy; signed reads are prepared afresh per call and are not automatically retried. Client close/cancellation never silently repeats a write.

See [coverage](COVERAGE.md), [signing](SIGNING.md), [verification](VERIFICATION.md), [source pins](API_SOURCES.json) and [notice](NOTICE). Public smoke and explicitly gated write examples are in `crates/python/examples/prediction_*.py`.
