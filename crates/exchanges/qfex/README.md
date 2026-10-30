# QFEX

Enable `features = ["qfex"]` on `aux-rs`. `QfexClient::testnet()` selects QFEX's UAT exchange (`qfex.io`); `mainnet()` selects production (`qfex.com`). The default `http` and `ws` features provide networking. With defaults disabled, inject `HttpTransport`, `WebSocketTransport`, or `WebSocketConnector` through `QfexClient::with_transport`.

```rust,no_run
use aux_rs::qfex::{QfexClient, RefdataRequest, OrderBookRequest};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = QfexClient::testnet()?;
let symbols = client.refdata(RefdataRequest::default()).await?;
let book = client.order_book(OrderBookRequest::new("AAPL-USD")).await?;
# Ok(())
# }
```

Implemented public operations are `refdata`, `contracts`, `order_book`, and MDS subscriptions for order books, trades, BBO, mark price, funding rate, open interest, market stats, and reference data. REST requests pin `api_version=2026-10-29`.

Native JSON numbers use `serde_json::Number` with arbitrary precision. Decimal strings, trailing zeros, sequence IDs, and unknown fields remain available. `Event` retains the native envelope; `payloads::<OrderBookUpdate>()` and `payloads::<Trade>()` decode flat or contents-array payloads without building an application order book.

## Authentication and trading

```rust,ignore
use aux_rs::qfex::{QfexAuth, AddOrder, Side, OrderType, TimeInForce};
use zeroize::Zeroizing;

let auth = QfexAuth::new(public_key, Zeroizing::new(secret_key))?;
let client = client.with_auth(auth);
let positions = client.positions().await?;
let balances = client.account_balances().await?;

let mut trading = client.trading_session().await?;
let response = trading.add_order(AddOrder {
    symbol: "AAPL-USD".into(),
    side: Side::Buy,
    order_type: OrderType::Limit,
    order_time_in_force: TimeInForce::Gtc,
    quantity: "1.25".parse()?,
    price: Some("200.01".parse()?),
    client_order_id: unique_client_id,
    reduce_only: 0,
}).await?;
trading.close().await?;
```

QFEX uses HMAC-SHA256 over `nonce:unix_seconds`. Each authenticated REST read gets a fresh OS-random nonce and executes once, since reusing an authentication nonce during a retry may be rejected. `with_account(uuid)` selects a subaccount and `with_builder(uuid)` sets trade-session builder attribution.

A trading session authenticates successfully before subscribing to order responses or sending commands. `add_order` requires an explicit client order ID for response correlation; choose a fresh ID for each distinct order. `cancel_order` accepts an order ID or client order ID. `user_orders(UserOrdersRequest)` queries native user orders with symbol, limit, and offset filters. Native status strings, including rejections and partial outcomes, remain visible. `subscribe_account` enables native balances, positions, and fills on that session; poll `next()` to receive them.

Mutating commands are sent once. A dropped, timed-out, or failed command leaves the session pending until its matching native order response is consumed using `next()`. Reconcile an ambiguous outcome before choosing to submit another order. Trade sessions never reconnect or replay automatically. Close/drop and transport shutdown terminate their dedicated connection.

The client also has one-shot `add_order`, `cancel_order`, and `user_orders` convenience methods. Each opens an authenticated session, executes one command, and closes it. Retain `trading_session()` for sustained trading. Public MDS subscriptions use the shared transport's bounded buffers, retry/reconnect policy, explicit gap notifications, and unsubscribe/close behavior.

JWT/wallet login, key provisioning, TWAPs, stop-order workflows, transfers, and funding operations are outside this adapter's current surface. API keys must already exist with the required permissions. Authenticated order placement has been verified against local peers, not a funded UAT account.

## Python

`aux_py.QfexClient` exposes the same public reads, authenticated positions/balances and user orders, add/cancel commands, MDS subscriptions, and account subscriptions. Supply `public_key` and `secret_key` for authenticated operations. Order `quantity` and `price` accept decimal strings or integers; Python floats are rejected. Async context managers and `aclose()` shut down owned work and streams.

## Sources and checks

Contracts were reviewed on 2026-10-30 against QFEX's [REST introduction](https://docs.qfex.com/api-reference/introduction), [versioned OpenAPI](https://api.qfex.com/contracts/2026-10-29/openapi.yaml), [MDS AsyncAPI](https://docs.qfex.com/websocket/mds.yaml), [trade AsyncAPI](https://docs.qfex.com/websocket/trade.yaml), and [authentication reference](https://docs.qfex.com/websocket/channels/trade/authenticate).

Public UAT fixtures in `tests/fixtures/` were captured read-only on that date. `positions.json` is synthetic schema-shaped account data. `auth.json` uses public test credentials and an independently computed Python standard-library HMAC. Reference-generation scripts are not included.

```sh
cargo test --locked -p aux-rs-qfex --no-default-features
cargo run --locked -p aux-rs --features qfex --example qfex_public
```

The example contacts public UAT endpoints and never authenticates or trades.
