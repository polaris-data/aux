# Monaco

Enable `features = ["monaco"]` on `aux-rs` for native Monaco spot and perpetuals APIs. Perpetual orders use Monaco's native `MARGIN` trading mode.

```rust,no_run
use aux_rs::monaco::{MonacoClient, TradingPairsRequest, OrderBookRequest};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = MonacoClient::testnet()?;
let pairs = client.trading_pairs(TradingPairsRequest::default()).await?;
if let Some(pair) = pairs.trading_pairs.and_then(|pairs| pairs.into_iter().next()) {
    let book = client.order_book(OrderBookRequest {
        trading_pair_id: pair.id.ok_or("missing pair ID")?,
        trading_mode: pair.market_type,
        levels: Some(20),
        ..Default::default()
    }).await?;
    println!("{:?}", book.data);
}
client.transport().shutdown().await?;
# Ok(())
# }
```

`mainnet()` uses `https://apimonaco.xyz` and `wss://ws.apimonaco.xyz/ws` (Sei chain ID 1329). `testnet()` uses `https://staging.apimonaco.xyz` and `wss://ws-staging.apimonaco.xyz/ws` (1328). `Monaco::custom(http, ws, chain_id)?` requires explicit chain context; its HTTP base may end at the origin, `/api`, or `/api/v1`. Requests append `/api/v1` exactly once. URL credentials, query strings and fragments are rejected in endpoint configuration.

`new(exchange)` and `with_config(exchange, transport_config)` supply built-in networking. `with_transport(exchange, transport)` accepts caller I/O. The adapter has no default features; facade `http`/`ws` features activate its standard networking independently. Public requests always work without authentication. User-Agent is set on native HTTP requests, including those sent through custom transports. Custom transports must preserve prepared URLs, headers and body bytes, disable redirects, and never replay writes.

## Supported operations

| Area | Client methods |
| --- | --- |
| Markets | `trading_pairs`, `trading_pair`, `market_metadata` |
| Public market data | `order_book`, `trades`, `mark_price`, `index_price`, `funding`, `funding_history` |
| Account reads | `balances`, `parent_margin_account`, `margin_accounts`, `positions`, `orders`, `order` |
| Orders | `place_order`, `replace_order`, `cancel_order`, `batch_create`, `batch_replace`, `batch_cancel`, `cancel_all` |
| Sessions | `create_challenge`, `verify_signature`, `login`, `refresh_session`, `revoke_session` |
| Public streams | `Subscription::Trades`, `Orderbook`, `Depth`, `Bbo` via `subscribe` |
| Private streams | `AccountSubscription::Orders`, `Balances`, `Positions`, `Account` via `subscribe_account` |

Each read accepts its native request struct. Optional filters and page/page-token fields are preserved; pagination is caller-managed. Response fields follow the official OpenAPI schemas, including optional fields and unknown response properties. Decimal strings and scaled integers remain exact. No common exchange trading model is introduced.

Order submission and replacement require the relevant `TradingPairData` so tick size, quantity increments, quantity bounds, trading mode and leverage can be checked without rounding. `CreateOrderRequest::limit` and `::market` are convenience constructors. Supply current metadata; refresh it when the exchange changes market constraints. Orders preserve native margin account/risk bucket IDs, strategy keys, cross/isolated margin, leverage, reduce-only, post-only, TIF, STP, client correlation and idempotency fields, plus supported attached TP/SL/trailing-stop fields. Native risk and balance checks remain server-owned.

`batch_create` takes metadata for each pair represented in the batch; `batch_replace` takes one market per input item in the same order. Results remain in native input order, including mixed successes and errors. Batch replacement can cancel an original before a later placement failure; inspect each result and reconcile the order stream. A successful HTTP response or `status: SUCCESS` can contain `matchResult.status: REJECTED`.

## Authentication

Monaco requires an application client ID, authorized separately for each network. Login uses a wallet challenge to register a locally generated Ed25519 session public key. Session requests then use that key rather than the wallet key. Sessions have expiry, refresh and revocation; they are not bearer tokens.

```rust,ignore
use aux_rs::monaco::*;
use zeroize::Zeroizing;

let public = MonacoClient::testnet()?;
let wallet = LocalWalletSigner::from_hex(Zeroizing::new(wallet_key_hex))?;
let session = LocalSessionSigner::generate()?;
let auth = public.login(&wallet, client_id, session, LoginOptions::default()).await?;
let client = public.with_auth(auth);
let balances = client.balances(BalancesRequest::default()).await?;
client.refresh_session().await?;
client.revoke_session().await?;
```

Local wallet signing uses EIP-191 and K256. External wallets implement async `WalletSigner`; pass their returned authorization unchanged, including supported contract-wallet signatures. External session signers implement async `SessionSigner`. Ed25519 signatures are verified against their public key before transmission. Secrets are redacted from Debug/Display and local private-key storage is zeroized.

For caller-owned secure persistence, explicitly export `LocalSessionSigner::seed()` and store `auth.session_info()` separately. Restore with `MonacoAuth::restore(exchange, signer, info, protocol)?`; the registered public key, wallet, client ID, expiry and network must describe that session. Metadata does not independently prove registration: the server authenticates the registered public key. Authenticated clients reject endpoint/chain mismatches. Never reuse a session key across independently trusted deployments.

Legacy signing is the default; select `RequestSigning::Rfc9421` in login options or restoration to opt in. There is no fallback or automatic downgrade. Both sign the final escaped path/query and exact serialized body. Legacy timestamps are milliseconds; RFC timestamps are seconds. Timestamps provide freshness, not replay prevention. See [signing provenance and vectors](SIGNING.md).

Refresh explicitly. Expiry updates and local revocation are shared by cloned auth handlers and private streams. `revoke_session` invalidates local credentials even on an error or cancellation; a failed network request does not prove remote revocation. Ordinary client/transport close releases local resources without revoking remote sessions. No credentials are persisted automatically.

## Outcomes and recovery

All writes and signed reads execute once. Public reads alone opt into the standard safe-read retry policy. The transport never retries a signed request with an aged signature.

Set and persist `idempotencyKey` before creating an order or each batch item. The connector neither generates nor replaces keys. Monaco protects an accepted create for 15 minutes; explicit recovery must reuse the same key and payload within that original window. `clientOrderId` is a reusable correlation label and does not prevent duplicate execution. Never interpret silence, an empty order-history read, or a lost response as evidence of rejection.

Transport failures retain `RequestOutcome::NotSent` or `Unknown`. HTTP 5xx or an undecodable successful write response returns `Error::OutcomeUncertain`; structured native bodies remain available. Other native HTTP errors preserve status/body/code. Per-item batch errors remain in the response rather than flattening the whole batch into success or failure. Formatting errors excludes server text, which may echo sensitive input.

## Streams

Public and private subscriptions are separate Rust methods so unsigned clients cannot access protected channels. Native `Subscribed`, `Snapshot`, `Event` and error details are preserved. Stream payloads keep their native snake_case fields and exact values; account/order versions are not interchangeable with contiguous replay cursors.

Private sockets authenticate before subscribing and wait for `Authenticated`. Reconnect signs a fresh handshake, emits a recoverable transport gap, and requires fresh channel snapshots. Scope includes endpoint, chain, wallet, client and session key. Late consumers of snapshot-based channels receive their own fresh baseline. Revocation and expiry wake quiet private streams and terminate them. Custom WebSocket transports must honor the optional authentication, invalidation, connection-scope and fresh-snapshot hooks in the core contract.

`SNAPSHOT_UNAVAILABLE` produces a recoverable gap followed by the native error envelope. Depth/BBO wait for the server to supply a later baseline. Other channels may continue publishing live frames; refetch their REST state/history and reconcile it with incoming events. Depth tracks native `book_seq` links while preserving envelopes. A broken link closes that subscription and emits a gap: open a new subscription for a fresh snapshot. Overlapping valid links are accepted. Every snapshot may replace a prior baseline, including one sent mid-stream. Applications own book reconstruction, stale-frame handling and account reconciliation; REST snapshots are not substitutes for a depth stream's baseline.

Heartbeats, bounded reconnect/backoff, overflow and explicit shutdown use the shared transport. Malformed frames, rejected subscriptions and exhausted recovery terminate the handle. Close or drop subscription handles for cleanup. The connector does not opt into Monaco's optional scoped-resync capability.

## Python

`aux_py.MonacoClient` calls the same Rust implementation. Read methods accept optional dictionaries using native camelCase keys; methods requiring an ID validate it before I/O.

```python
from aux_py import MonacoClient

async with MonacoClient() as client:
    pairs = await client.trading_pairs({"pageSize": 20})
    pair = pairs["tradingPairs"][0]
    book = await client.order_book({
        "tradingPairId": pair["id"], "tradingMode": pair["marketType"],
    })
```

Use `MonacoWalletSigner(private_key)` or `.remote(address, async_callback)` for wallet authorization. `MonacoSessionSigner(seed=None)` generates or restores a session; `.remote(public_key, async_callback)` receives message bytes and returns a hex signature. `login(wallet, client_id, session_signer=..., request_signing="legacy")` returns nonsecret session metadata. `session_signer.export_seed()` is an explicit secret export; `restore_session(signer, info, request_signing=...)` restores it. Callback work uses the initiating asyncio loop/context, including WebSocket reconnect signing, and is cancelled with native work.

Account/trading method names match Rust. Pass native market dictionaries to `place_order`/`replace_order` and metadata lists to batch methods. `subscribe({"type": "orders"})` opens a private stream; `subscribe({"type": "orderbook", "tradingPairId": pair_id, "mode": "SPOT"})` opens a public stream. Subscriptions support async iteration and context managers. Request floats are rejected; unknown response fractional numbers become `Decimal`, while native decimal strings stay strings. Errors retain structured codes and uncertain outcome details.

## Verification and boundaries

Normal tests use committed fixtures and local HTTP/WebSocket servers, with no exchange requests or orders. Run the explicit public testnet checks separately:

```sh
cargo run --locked -p aux-rs --features monaco --example monaco_public
.venv-python/bin/python crates/python/examples/monaco_public.py
```

On 2026-11-03, staging REST metadata/books/trades and public `orderbook` snapshots were reachable. Staging rejected the documented `depth` channel with `INVALID_SUBSCRIPTION`; depth/BBO live compatibility remains a release gate despite offline protocol coverage. No automatic channel substitution occurs.

Authenticated submit/cancel verification requires an authorized testnet client ID and a funded account. The testnet example requires `--execute` and the `MONACO_TESTNET_WALLET_KEY`, `CLIENT_ID`, `PAIR_ID`, `PRICE`, `QUANTITY`, and `IDEMPOTENCY_KEY` variables (each with the `MONACO_TESTNET_` prefix); margin orders additionally require `MONACO_TESTNET_LEVERAGE`:

```sh
cargo run --locked -p aux-rs --features monaco --example monaco_testnet_trade -- --execute
```

The example requests a post-only limit order and then cancellation; neither post-only nor cancellation guarantees an order cannot fill later. Retain the original payload/key and reconcile unknown outcomes. Authenticated live verification remains pending; normal CI does not need credentials. No live mainnet orders are part of these checks.

Deposits, withdrawals, collateral transfers, delegation provisioning, separate conditional/TWAP management, copy trading, analytics and gRPC are outside this connector's initial surface.
