# Extended

Enable `extended` on `aux-rs` for native Extended APIs, URL-based WebSocket streams, v2 RPC subscriptions, Stark settlement hashing/signing, and wallet registration signing.

```rust,no_run
use aux_rs::extended::{ExtendedClient, Subscription};
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = ExtendedClient::testnet()?;
let markets = client.get_markets(Default::default()).await?;
let mut stream = client.subscribe(Subscription {
    channel: "orderbooks".into(), market: Some("BTC-USD".into()),
    params: serde_json::json!({}), rpc: false,
}).await?;
let update = stream.next().await;
stream.close().await?;
client.transport().shutdown().await?;
# Ok(())
# }
```

[Coverage](COVERAGE.md) lists the typed REST methods. The inventory combines the official SDK models/routes, its partial OpenAPI snapshot, and the published API documentation, including spot, perpetuals, RFQ, vaults, onboarding, account administration and portfolio/interest endpoints. Requests preserve native parameter names; JSON bodies are under `body`. Unknown response fields and native numeric representations remain available. SDK integer fields that arrive as either JSON numbers or strings use `NativeNumber` in Rust; Python and Node retain the received representation.

`Extended::mainnet()` and `testnet()` select the published deployments. `custom(http, ws, chain_id)` requires explicit endpoints and signing context. Production retains the `SN_MAIN` Stark domain for compatibility, even though the API documentation describes Arc settlement. URL text does not determine the signing domain. Custom wallet registration also requires its explicit `signingDomain`.

Use `Credentials::new(api_key, access_token, expires_at)` and `with_auth`. The adapter sends the API key and/or bearer token explicitly supplied by the caller. Mandatory `User-Agent` headers are included on HTTP and WebSocket requests. Sensitive endpoints retain their native wallet-signature or settlement fields. Session persistence and renewal are application-owned.

`OrderContext` supplies the native asset identifiers, resolutions and collateral position. `prepare_order` produces the native settlement hash without signing. `sign_order` attaches a verified local or remote Stark signature. Supply exact decimal strings, nonce, expiry and fee limits. Quantity conversion rejects excess precision; derived quote and fee rounding follows the pinned official reference. Presigned native envelopes are accepted by the typed REST methods, including conditional and TPSL structures. The low-level `settlement` types and `order_hash`, `limit_order_hash`, `transfer_hash`, and `withdrawal_hash` expose exact native signing inputs. Wallet onboarding uses a separate EIP-712 payload and EVM signer.

Order submission is asynchronous: REST acceptance does not mean the order is working. Track native account stream updates and reconcile order IDs. RFQ orders have their own submission route; the standard RFQ book is indicative, while `rfq_orderbooks` exposes the resting-order feed. No state-changing request is retried automatically.

Set `Subscription.rpc = true` to use the v2 JSON-RPC subscription endpoint. The adapter supports public books, RFQ books, trades, funding, candles, mark/index prices and private account updates on both native stream surfaces. URL candle types are `trades`, `mark-prices`, and `index-prices`; RPC selectors use `last`, `mark`, and `index`. RPC replies are correlated by request ID and stream data by native topic ID; RPC heartbeats use the native ping method. Native sequence gaps trigger bounded reconnect and a fresh baseline. Applications own state reconstruction.

Python exports `ExtendedClient` and `ExtendedSigner` from `aux_py`; Node exports matching classes from `aux-ts` with camelCase methods. Signing callbacks run through the existing async runtime boundaries. Close subscriptions and clients explicitly.

See [source pins](API_SOURCES.json), [verification](VERIFICATION.md), `NOTICE` and `CRYPTO_LICENSE`. Normal tests use fixtures and local servers.
