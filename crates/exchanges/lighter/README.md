# Lighter adapter

Implemented Rust APIs: market metadata, native order-book snapshots, recent trades, public account/position reads, authenticated active-order reads, create/cancel submission, Rust auth tokens, and public book/trade/account plus private order streams. Select `lighter` on the facade; no exchange is selected by default.

The SDK is local and unpublished. [SIGNING.md](SIGNING.md) records the pinned official reference, cryptographic limitations, reproduction commands and outstanding funded testnet checks. The local signer is experimental and unaudited. [Python bindings](../../python/README.md) cover the implemented Lighter operations. Built-in transports support safe-read retries, explicit reconnect gaps and shutdown. Python wheel distribution remains later work.

## Public reads and streams

```toml
aux-rs = { path = "../connectors/crates/aux-rs", features = ["lighter"] }
# Both: features = ["hyperliquid", "lighter"]
```

```rust,ignore
use aux_rs::lighter::{
    AccountRequest, Event, LighterClient, OrderBookOrdersRequest,
    OrderBooksRequest, Subscription,
};

let client = LighterClient::testnet()?;
let markets = client.order_books(OrderBooksRequest::default()).await?;
let market = markets.order_books.iter().find(|m| m.status == "active")
    .ok_or("no active market")?;
let book = client.order_book_orders(OrderBookOrdersRequest {
    market_id: market.market_id, limit: 10,
}).await?;
let account = client.account(AccountRequest::Index(37)).await?;

let mut stream = client.subscribe(Subscription::OrderBook {
    market_id: market.market_id,
}).await?;
while let Some(event) = stream.next().await {
    if let Event::OrderBook(event) = event? {
        // "subscribed/order_book" is a snapshot; "update/order_book" is a delta.
        // Retain nonce, begin_nonce, offset, and zero-size levels as received.
        println!("{}: {}", event.event_type, event.order_book.nonce);
        break;
    }
}
stream.unsubscribe().await?;
```

`RecentTradesRequest` exposes recent HTTP trades. `Subscription::Trades` and `Subscription::Account` select `trade/{market}` and public `account_all/{account}` streams. Account lookup also accepts a validated L1 address. These public operations need no signer; query identity is explicit.

Prices/quantities in reads are native strings. Signed requests use native scaled integers. `market.scale_price("123.45")` and `market.scale_size("0.001")` check metadata precision and integer overflow without rounding. Market IDs do not encode spot/perpetual type; use `market_type`.

Stream events retain native snapshots and deltas; this crate does not build a local order book. Different channels share a compatible standard WebSocket session. A duplicate Lighter subscription gets a separate session for a fresh snapshot. Custom transports must honor snapshot, scope, credential-validity and JSON control contracts. Server JSON pings receive one pong per frame/connection. On reconnect, a recoverable `Gap` precedes fresh data and stale queued frames are cleared; discard/reconcile derived state and keep polling. Deltas before a new native snapshot are rejected. Overflow/protocol faults or exhausted recovery are terminal. Unsubscribe/drop cancel recovery when the final consumer leaves. Private token replay is refused after expiry; request a new authenticated subscription then. Background token refresh is not implemented.

## Authentication and submission

```rust,ignore
use std::sync::Arc;
use zeroize::Zeroizing;
use aux_rs::lighter::{
    CancelOrder, CreateOrder, LighterAuth, LighterClient, LocalSigner, Network, TxOptions,
};

let signer = Arc::new(LocalSigner::from_hex(Zeroizing::new(
    std::env::var("LIGHTER_API_PRIVATE_KEY")?,
))?);
let auth = LighterAuth::new(signer, Network::Testnet.chain_id(), account_index, api_key_index)?;
let client = LighterClient::testnet()?.with_auth(auth);
let open = client.active_orders(Some(market_id)).await?;
let mut private_stream = client.subscribe_account_orders().await?;
let accepted = client.create_order(CreateOrder {
    market_index: market_id,
    client_order_index: unique_client_order_index,
    base_amount: exact_scaled_size,
    price: exact_scaled_price,
    is_ask: 1,
    order_type: 0,       // Native limit order.
    time_in_force: 2,    // Native post-only.
    reduce_only: 0,
    trigger_price: 0,
    order_expiry: order_expiry_unix_millis,
}, TxOptions::short_lived()?).await?;
let cancelled = client.cancel_order(CancelOrder {
    market_index: market_id, index: unique_client_order_index,
}, TxOptions::short_lived()?).await?;
private_stream.unsubscribe().await?;
```

These are usage snippets with caller-supplied account/market values, not automated trading instructions. The key must already be registered for that account/API-key slot. `LocalSigner` takes a native 40-byte little-endian scalar, not an Ethereum key. API keys, account identities and network context are separate. Authenticated methods are unavailable on an unsigned client.

`Transaction::prepare` accepts an explicit nonce and creates an immutable signing payload for offline signing. `PreparedTransaction::sign` verifies the signer's result and returns immutable `SignedTransaction` fields. Applications using the exported native `tx_info` outside the client own their submission/nonce coordination. The automatic client path always uses its sequential coordinator.

Submission code `200` means accepted, not executed. Preserve `tx_hash` and reconcile native orders/transactions/streams. Transport errors retain their `RequestOutcome`; native errors retain their code, message and extra fields. Error formatting hides server text and bodies because they can echo credentials. Raw native error text remains explicitly inspectable in the error variant.

The process-wide coordinator serializes preparation and submission per chain/account/API-key identity, including independent auth instances. After uncertain execution, rejection, malformed responses or cancellation during transport, writes stop with `NonceUncertain`. **Reconcile pending transactions before explicitly calling `resync_nonce()`.** The SDK never automatically resynchronizes/replays or guesses that a timed-out order was not accepted. Separate processes/external clients require application coordination.

Create/cancel, fee/self-trade/order-version attributes and native order types `0..=6` are supported. Nonce-skip/cancel-all attributes, key registration, grouped/batch submission and other transaction families are not exposed. Native market limits and account permissions still apply server-side.

## Custom signer or transport

Implement the native `Signer` contract to replace local cryptography:

```rust,ignore
impl aux_rs::lighter::Signer for RemoteSigner {
    fn public_key(&self) -> aux_rs::lighter::PublicKey { self.public_key }
    async fn sign(
        &self,
        payload: &aux_rs::lighter::SigningPayload,
    ) -> Result<aux_rs::lighter::Signature, aux_rs::lighter::Error> {
        // Enforce chain/account/API-key/purpose policy before signing the
        // canonical 40-byte digest. Return native 80-byte Schnorr s || e.
        self.service.sign_lighter(payload).await
    }
}
```

The SDK verifies custom signer results against the declared public key and original digest before any submission. `AuthHandler` can also be replaced while reusing `NonceSequence::for_identity`. HTTP/WS-only and custom transport consumers keep the same native APIs. Disable facade defaults and use `LighterClient::with_transport(exchange, your_transport)` to avoid Tokio/Reqwest/Tungstenite dependencies. `Lighter::custom(http_base, ws_url, chain_id)` requires explicit signing context and supports proxy base paths. No network request occurs when building a client.

## Verification

From `connectors/`:

```sh
bash scripts/check.sh
cargo run --locked -p aux-rs --features lighter --example lighter_public
# Requires registered/funded TESTNET credentials; never run automatically:
cargo run --locked -p aux-rs --features lighter --example lighter_testnet_trade -- --execute
```

The read-only example selects an active testnet market and checks all public HTTP/stream types. The trading example only selects testnet, requires an explicit `--execute` flag and the environment variables listed in its startup message, submits a post-only ask, checks that it rests, cancels by its known client order index, and checks removal. An ambiguous submission stops for caller reconciliation. Funded trading/private-order-stream checks have not been run here because credentials are absent.
