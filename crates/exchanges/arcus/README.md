# Arcus

Enable `features = ["arcus"]` on `aux-rs` to use `ArcusClient`. This adapter covers the native perpetuals API. Arcus's separate spot router is outside this adapter.

```rust,no_run
use aux_rs::arcus::{ArcusClient, MarketsRequest, OrderBookRequest};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = ArcusClient::testnet()?;
let markets = client.markets(MarketsRequest::default()).await?;
let book = client.order_book(OrderBookRequest::new("BTC-USD")).await?;
# Ok(())
# }
```

`testnet()` selects `api.testnet.arcus.xyz`; `mainnet()` selects `api.arcus.xyz`. The default `http` and `ws` features supply networking. Disable defaults and use `with_transport` for caller-supplied I/O, or `with_config` to configure the built-in transport.

Implemented public HTTP operations are market metadata, order books, recent trades, account state, positions, and open orders. Account reads accept an explicit master address and account index (`0` through `9`) without requiring a signer. Decimal strings, sequence IDs, native responses, and unknown fields are preserved.

## Signing and orders

```rust,ignore
use aux_rs::arcus::{ArcusAuth, LocalSigner, OrderRequest, OrderType, Side, TimeInForce};
use zeroize::Zeroizing;

let signer = LocalSigner::from_hex(Zeroizing::new(ed25519_seed_hex))?;
let auth = ArcusAuth::new(signer, master_address, account_index)?;
let client = client.with_auth(auth);
let market = &markets.markets[0];

let result = client.place_order(market, OrderRequest {
    market_id: market.market_id,
    order_side: Side::Buy,
    order_type: OrderType::Limit,
    quantity: "0.001".into(),
    price: "50000.5".into(),
    time_in_force: TimeInForce::Alo,
    good_til_time: expiry_unix_microseconds,
    client_id: Some(unique_client_id),
    reduce_only: false,
}).await?;
```

The Ed25519 API key must already be registered to the supplied master address and account index. `LocalSigner` accepts a 32-byte seed; implement `Signer` for a remote signing service. Returned signatures are verified against the original payload before submission.

`place_order` requires caller-supplied market metadata. The signing payload converts price to integer ticks using the market's top-level `tickSize`, and size to integer quantums using `stepSize`, without floats or rounding. The top-level tick remains the signing denominator even when a market has price-dependent tick tiers. The venue validates tier alignment, notional/size bounds, and account permissions.

`goodTilTime` is a microsecond epoch string on the wire and becomes nanoseconds in the signed payload. Arcus requires this expiry on every order, including IOC/FOK, and currently requires it to be at least one month ahead. Supply the desired native expiry explicitly. Signed request timestamps use nanoseconds and a process-wide monotonic coordinator keyed by the Ed25519 public key.

`cancel_order` takes exactly one order ID or client ID. Both methods use native typed canonical payloads, preserve client-ID case, lowercase only the account address, and use a single HTTP attempt. An `ACK` / HTTP 202 acknowledges submission, not execution; observe native order/fill events to determine the outcome. Failed or cancelled submissions are never replayed automatically.

Batch/modify/TPSL workflows, legacy signed actions such as set leverage, API-key provisioning, withdrawals, transfers, and the spot router are outside the current surface. Authenticated trading is covered by local peers and independent Ed25519 fixtures; funded testnet verification remains pending.

## Streams

```rust,ignore
use aux_rs::arcus::{Channel, Subscription};
let mut books = client.subscribe(Subscription::market(Channel::OrderBookUpdates, "BTC-USD")).await?;
while let Some(event) = books.next().await {
    let event = event?;
    // Native type/channel/id/accountIndex/contents, including snapshot/degraded messages.
}
books.close().await?;
```

Supported channels are markets, full/delta books, trades, BBO, account, positions, orders, and user fills. Account channels are public; `Subscription::account(channel, address, account_index)` builds a correctly scoped request. Native account indices and market filters are part of view identity. `Event` preserves the full envelope; `decode_contents::<OrderBook>()` decodes a book snapshot when present.

The shared transport emits explicit gaps on reconnect. Delta-book subscriptions require a fresh `subscribed` snapshot before post-reconnect updates; degraded subscriptions remain pending until that snapshot is available. The adapter does not reconstruct an application order book. Each native view has an isolated connection scope so unkeyed errors cannot reach another account or view.

## Python

`aux_py.ArcusClient` covers the same reads, subscriptions, and place/cancel operations. `ArcusSigner(seed_hex)` supplies local Ed25519 signing. Configure `signer`, `address`, and `account_index` for writes. `place_order(market_metadata, request)` takes native dictionaries, with decimal prices and sizes supplied as strings. Async context managers and `aclose()` close owned work and subscriptions. Remote signer injection is currently available through the Rust `Signer` contract.

## Sources and checks

Reviewed on 2026-10-30 against Arcus's [API introduction](https://docs.arcus.xyz/api-reference/introduction), [authentication](https://docs.arcus.xyz/api-reference/authentication), [WebSocket protocol](https://docs.arcus.xyz/api-reference/websocket), and OpenAPI `openapi-2026.10.30` exposed by the [official docs MCP](https://docs.arcus.xyz/mcp).

`tests/fixtures/` contains read-only public testnet metadata, book, and stream captures from that date. The account fixture is synthetic public data. The signing fixture follows the documented canonical payload and was independently signed with Python cryptography's Ed25519 using the public test seed `07` repeated 32 times. No reference-generation scripts are included.

```sh
cargo test --locked -p aux-rs-arcus --no-default-features
cargo run --locked -p aux-rs --features arcus --example arcus_public
```

The public example never authenticates or trades.
