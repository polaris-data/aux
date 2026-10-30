# aux-rs

The Rust package is `aux-rs` (import `aux_rs`). Python bindings are distributed as
`aux-py` (import `aux_py`); see [the Python guide](crates/python/README.md).

The repository root contains the Cargo workspace configuration. The public Rust
SDK facade, examples, and integration tests live in `crates/aux-rs/`. Shared client
and transport code lives in `crates/core/`, exchange adapters in
`crates/exchanges/`, and Python bindings in `crates/python/`.

A standalone Rust SDK for native exchange APIs, with composable clients and opt-in exchanges. See [PLANNING.md](PLANNING.md) for the architecture and remaining milestones.

**Implemented: Hyperliquid, Lighter, QFEX, and Arcus Rust clients with Python bindings for their supported exchange operations.** Both languages expose native market/account reads and streams, local/custom signing, and order/cancel submission. Hyperliquid supports batches and signer-scoped nonces; Lighter serializes single transactions per chain/account/API-key identity. Python also covers offline signing/verification, exact numeric conversion and cancellable async iterators. Public Rust and Python smoke checks have passed all four testnets. Funded authenticated submit/cancel verification remains pending for all venues. Lighter's local signing primitives are experimental and unaudited; see [compatibility evidence and review limits](crates/exchanges/lighter/SIGNING.md).

## QFEX and Arcus

Enable `qfex` or `arcus` to select their concrete clients. Each exchange is optional;
Rust consumers do not compile unselected adapters or Python bindings.

```toml
aux-rs = { path = "../connectors/crates/aux-rs", features = ["qfex", "arcus"] }
```

```rust,ignore
use aux_rs::{qfex::QfexClient, arcus::ArcusClient};
let qfex = QfexClient::testnet()?; // QFEX UAT
let arcus = ArcusClient::testnet()?;
```

[QFEX](crates/exchanges/qfex/README.md) exposes public REST/MDS data,
HMAC-authenticated account reads, and authenticated WebSocket add/cancel commands.
[Arcus](crates/exchanges/arcus/README.md) exposes perpetuals market/account reads,
public streams, and Ed25519-signed REST place/cancel operations with exact native
price/size conversion. Both have Python clients and fixture/local-peer tests;
funded authenticated testnet verification remains pending.

```sh
cargo run --locked -p aux-rs --features qfex --example qfex_public
cargo run --locked -p aux-rs --features arcus --example arcus_public
```

These examples only read public testnet data. See the exchange guides for
credentials, native units, supported operations, and current coverage limits.

## Lighter

Select `features = ["lighter"]`, or `["hyperliquid", "lighter"]` for both adapters. The standard networking defaults remain exchange-neutral. Lighter-only consumers do not compile Hyperliquid's signing code, and Hyperliquid-only consumers do not compile Lighter's.

```sh
cargo run --locked -p aux-rs --features lighter --example lighter_public
```

The read-only example selects an active testnet market, checks metadata/book/recent-trade/account HTTP reads, and receives native book/trade/account stream snapshots. See [the Lighter guide](crates/exchanges/lighter/README.md) for native units, authenticated examples, custom signers, nonce recovery, and the opt-in `lighter_testnet_trade -- --execute` flow. It uses Rust hashing/signing throughout; compatibility tests use committed reference fixtures.

## Run the public testnet example

Requires Rust 1.92 or later. From this directory:

```sh
cargo run --locked -p aux-rs --features hyperliquid --example hyperliquid_public
```

The example reads metadata and an ETH book snapshot, receives ETH book and trade events over a shared WebSocket, and explicitly closes both subscriptions. It uses public testnet endpoints and requires no credentials. This flow was verified against Hyperliquid testnet on 2026-10-29.

## Use the SDK

The crates are local workspace packages and are not published. Adjust the path for the consuming crate:

```toml
[dependencies]
aux-rs = { path = "../connectors/crates/aux-rs", features = ["hyperliquid"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

```rust
use aux_rs::hyperliquid::{
    Event, HyperliquidClient, L2BookRequest, Subscription,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HyperliquidClient::testnet()?;
    let metadata = client.meta().await?;
    let book = client.l2_book(L2BookRequest::new("ETH")).await?;
    println!("{} markets; best bid: {:?}", metadata.universe.len(), book.levels[0].first());

    let mut trades = client
        .subscribe(Subscription::Trades { coin: "ETH".into() })
        .await?;
    while let Some(event) = trades.next().await {
        if let Event::Trades(batch) = event? {
            println!("Trades: {batch:?}");
            break;
        }
    }
    trades.close().await?;
    Ok(())
}
```

`HyperliquidClient::mainnet()` selects the public mainnet endpoints explicitly. For local servers and proxies, pass `Hyperliquid::custom(http_base_url, ws_url)` to `HyperliquidClient::new(...)`; the HTTP URL is a base URL without `/info`, query parameters, or a fragment. The chosen transport validates URLs when used. Constructing a client performs no exchange requests.

`HyperliquidClient` exposes HTTP operations and subscriptions as inherent methods, without extension-trait imports. Methods require a transport with the corresponding HTTP or WebSocket capability. For a builder-deployed perp DEX, call `meta_with(MetaRequest { dex: Some("xyz".into()) })`. The generic `client.request(native_request)` method executes typed public endpoints.

Prices and sizes stay as native strings, including trailing zeros. IDs and timestamps use integers. Book levels are `[bids, asks]`, and book events are full native snapshots. Unknown response fields remain available through each native type's `extra` map. Native HTTP status/body and API errors remain distinguishable from transport or decoding failures.

## Account reads and subscriptions

`HyperliquidClient` exposes `clearinghouse_state(UserStateRequest)` and `open_orders(OpenOrdersRequest)`. Positions are part of the native clearinghouse response. These endpoints accept the actual account address and do not require a signer. Margin summaries describe the native perpetuals account; this is not a normalized spot/perp portfolio balance.

Account subscriptions are `ClearinghouseState`, `OpenOrders`, `OrderUpdates`, and `UserFills`. They also use public account addresses, with optional DEX selection where supported. Run the read-only testnet example with an optional account address:

```sh
cargo run --locked -p aux-rs --features hyperliquid --example hyperliquid_account
# Append an account address to inspect that testnet account instead.
```

This example checks state/open-order reads, state/order/fill snapshots, and the order-update acknowledgement. The default public address and all four subscriptions were verified on 2026-10-29 without using a private key.

## Signing and trading

`HyperliquidClient` exposes trading methods only when its auth state implements the native `AuthHandler` contract. An unsigned client cannot call trading methods. `HyperliquidAuth` supplies the default handler; use `direct(signer)` for a master key or `agent(signer, account)` for an already-authorized agent.

```rust,ignore
use aux_rs::hyperliquid::{
    HyperliquidAuth, HyperliquidClient, LocalSigner, Order, OrderRequest, Tif,
};

let signer: LocalSigner = private_key.parse()?;
let auth = HyperliquidAuth::agent(signer, account_address);
let client = HyperliquidClient::testnet()?.with_auth(auth);
let request = OrderRequest::new(vec![Order::limit(asset_id, true, "1670.1", "0.0147", Tif::Alo)]);
let result = client.order(request).await?;
```

This is an API snippet with caller-supplied key, account, and native asset ID. Use [the opt-in testnet example](crates/aux-rs/examples/hyperliquid_testnet_trade.rs) for a complete executable flow. Asset IDs are native exchange IDs, not symbols; default perpetual IDs come from metadata universe indices. Account registration, funding, and agent approval are not performed automatically.

The account passed to `agent` is query context. Hyperliquid derives the master account for trading from the key's existing authorization; changing that context does not retarget a trade. Use `.with_vault(vault_or_subaccount)` to set the actual native `vaultAddress`; this also changes the handler's query account while keeping signer identity and nonce state intact.

- `order`, `cancel`, and `cancel_by_cloid` accept native batches. Limit and trigger orders, grouping, builder/sub-builder fees, reduce-only, and fixed-width client order IDs are represented explicitly.
- Read responses preserve exact numeric strings. Trading inputs require positive canonical decimals: no float conversion, exponent notation, leading zeros, or trailing fractional zeros. For metadata-aware precision checks, call `Order::validate_precision(sz_decimals, is_spot)`; it validates without rounding or fetching markets.
- `ExchangeResponse::Err` preserves a native whole-request rejection. `ExchangeResponse::Ok` still requires inspecting every `ItemStatus`: a batch may mix resting, filled, rejected, and native text/unknown outcomes. HTTP and decoding failures remain separate errors.
- `_with_options` variants accept `ExecutionOptions { nonce, expires_after }`. A supplied nonce bypasses automatic allocation and makes the caller responsible for collisions. Defaults use Unix milliseconds and one monotonically increasing coordinator per signer for the process lifetime, including across independent handlers and vault clones. Counters survive client reconstruction and are never rolled back after an error.
- Cross-process coordination is the caller's responsibility. A different process or a caller-managed nonce can collide with automatic allocation. Native nonce windows and in-flight ordering limits still apply.
- Signed envelopes are immutable, and signer recovery is checked before submission. Custom endpoints require `.with_network(Network::Testnet)` or `Mainnet`; signing context is never inferred from a URL.
- Implement `Signer` to use a remote signing service, or `AuthHandler` to replace request preparation/nonce management. `SignedRequest::from_signature` validates a custom handler's envelope. These contracts use `Send` futures without a runtime dependency or `async-trait`.

Native L1 action signing and offline user-signed primitives are covered by 44 vectors from the [pinned official SDK reference](https://github.com/hyperliquid-dex/hyperliquid-python-sdk/blob/2fdb18f9517675ea03695a0962bd19eece9c83f0/hyperliquid/utils/signing.py). The latter cover `UsdSend` and `ApproveAgent` hashing/signing only; there are no transfer or provisioning submission workflows. See [fixture provenance](crates/exchanges/hyperliquid/tests/fixtures/README.md).

### Opt-in authenticated testnet smoke check

Set `HL_AGENT_PRIVATE_KEY`, `HL_ACCOUNT_ADDRESS`, `HL_TESTNET_ORDER_PRICE`, and `HL_TESTNET_ORDER_SIZE` in your environment, using an already-authorized/funded testnet account. Optional settings are `HL_TESTNET_COIN` (default `ETH`) and `HL_VAULT_ADDRESS`. Then run:

```sh
cargo run --locked -p aux-rs --features hyperliquid --example hyperliquid_testnet_trade -- --execute
```

The example validates market precision, places one post-only buy with a fresh client ID, attempts cancellation by that ID even when submission fails, and exposes both native results. It rejects missing `--execute`, never selects mainnet, and is not run by normal checks. Ambiguous submit/cancel outcomes require reconciliation; `expiresAfter` limits submission time and is not the lifetime of a resting order. No authenticated smoke trade has been run in this workspace because the required credentials are absent.

## Features and custom transports

| Feature | Default | Effect |
|---|---|---|
| `http` | Yes | Pooled Reqwest HTTP client |
| `ws` | Yes | Tokio Tungstenite WebSocket sessions |
| `hyperliquid` | No | Native Hyperliquid types, Rust signing, auth, trading, and public/account methods |
| `lighter` | No | Native Lighter types, Rust signing, auth, trading, and public/account methods |
| `qfex` | No | Native QFEX data, HMAC authentication, and WebSocket trading |
| `arcus` | No | Native Arcus perpetuals data and Ed25519-signed REST trading |

No exchange is selected by default. Exchange features do not activate networking. Each adapter includes its native signing code without enabling standard transports or an async runtime.

For custom I/O:

```toml
aux-rs = { path = "../connectors/crates/aux-rs", default-features = false, features = ["hyperliquid"] }
```

Implement `HttpTransport` and/or `WebSocketTransport`, then pass the implementation to `HyperliquidClient::with_transport(exchange, my_transport)`. The [protocol tests](crates/exchanges/hyperliquid/tests/protocol.rs) include a working custom HTTP transport. These contracts use `Send` futures without `async-trait`. Custom transports own their configuration and resource lifecycle.

Without features, `aux-rs-core` has zero dependencies. With built-in transport features disabled, the Hyperliquid adapter has no runtime or networking dependencies. Its direct dependencies are core, Serde/Serde JSON, K256 for secp256k1, SHA3 for Keccak, rmp-serde for MessagePack, and zeroize for temporary secret buffers. Native identifiers and signatures use the shared core hex helpers; Lighter's transaction signatures use a private Base64 encoder. Both implementations record pinned upstream sources, protocol references, and license notices. K256 clears its private-key storage on drop; private keys are excluded from debug/error output. The standard stack shares a Rustls trust policy and Ring provider across HTTP and WebSockets. See [DEPENDENCIES.md](DEPENDENCIES.md) for production inventories, including Reqwest's transitive Tower/tracing crates.

## Python bindings

The separate [Python crate](crates/python/README.md) uses PyO3 and `pyo3-async-runtimes` with concrete owned `HyperliquidClient` and `LighterClient` wrappers. It exposes the implemented Rust exchange reads, positions/account/open-order APIs, submission/cancellation and stream types through asyncio, plus offline signing/verification and local or custom async signers. Native response keys and decimal strings remain intact; integers keep their full precision and other JSON numbers become Python `Decimal`. Structured exceptions retain native codes and ambiguous outcomes without formatting server credential echoes.

From this directory, build and validate in a dedicated Python 3.10+ environment:

```sh
python3.12 -m venv .venv-python
.venv-python/bin/pip install -r crates/python/dev-requirements.txt
bash scripts/check-python.sh
.venv-python/bin/python crates/python/examples/public_testnet.py
```

The last command explicitly contacts both public testnets; the preceding checks use local mock servers. Cancelling a started stream read closes its native subscription. Client close shuts down owned subscriptions and cancels pending work, including custom Python signing callbacks; cleanup continues if a close await is cancelled. Lighter's nonce quarantine and explicit recovery carry through unchanged. The existing bare Hyperliquid book iterator remains compatible; new generic iterators expose native envelopes. Release wheel distribution remains separate work.

Rust-only workspace defaults and `scripts/check.sh` exclude the Python crate. PyO3 is absent from every Rust-only dependency profile, and the baseline checker enforces that separation. The binding was validated with CPython 3.12.7 on macOS arm64; the broader platform/interpreter matrix is later work.

## Connection behavior

Milestone 5 recovery/shutdown behavior is implemented. See [HARDENING.md](HARDENING.md) for contracts, defaults, gap-handling examples and fault-test coverage.

- Cloned clients share their HTTP connection pool and WebSocket sessions per URL and protocol scope. Public book/trade streams share one scope. Account streams are isolated by account because order updates omit the user address. Aggregated fills have a separate scope because their events omit the aggregation parameter. Custom transports must honor `connection_scope` when pooling.
- Subscription creation waits for the send or local attachment. Native `SubscriptionResponse` events and API errors expose the remote result; it does not wait for a remote acknowledgement. A late consumer of an existing subscription may receive data without a new acknowledgement.
- Late Hyperliquid consumers start with live data; earlier snapshots are not replayed from a client cache. Lighter duplicate consumers get a fresh native snapshot over a separate session. The SDK does not maintain an application order book.
- A handle routes only matching native channel/coin data. Unknown channels and malformed frames are exposed explicitly. Application-level heartbeat pongs are handled internally.
- Each handle has a bounded buffer. Overflow, protocol/size faults, expired replay credentials and exhausted recovery are terminal; buffered data cannot hide them. Disconnects and idle timeouts use bounded reconnect/resubscribe by default.
- Recovery emits a nonterminal `TransportErrorKind::Gap` before fresh data and clears stale queued frames. Catch `error.is_gap()`, discard/reconcile derived state, and keep polling. Python uses a recoverable `GapError`. Lighter rejects deltas before a fresh native snapshot. Fixed private tokens are never replayed after expiry; create a new authenticated subscription then.
- `next()` is cancellation-safe. `close()` / `unsubscribe()` detaches one consumer, sends remote unsubscribe for the last consumer of a subscription, and closes the socket when no subscriptions remain. Drop schedules the same cleanup without waiting for it. Explicit close waits for local cleanup and sending unsubscribe, not a server acknowledgement.
- Typed safe reads opt into bounded retries, including native POST reads and Lighter private order reads. `Retry-After` is honored within the policy cap. Trading, unmarked endpoints, decoding/size failures and redirects are never automatically retried. Transport errors retain `NotSent`/`Unknown`; native error formatting hides server credential echoes while raw structured fields remain inspectable.
- `StandardTransport::request_shutdown()` signals all clones closed. `shutdown().await` also waits for handshakes, HTTP work and socket drivers, attempts native unsubscribe, clears the session registry and releases the HTTP pool. Python `aclose()` uses this shared lifecycle. New trading calls on a closed transport fail before nonce allocation/signing.

Configure the built-in transport when constructing the client:

```rust,ignore
use aux_rs::{
    TransportConfig,
    hyperliquid::{Hyperliquid, HyperliquidClient},
};
use std::time::Duration;

let client = HyperliquidClient::with_config(
    Hyperliquid::testnet(),
    TransportConfig {
        timeout: Duration::from_secs(5),
        subscription_capacity: 64,
        ..TransportConfig::default()
    },
)?;
```

Defaults are a 10-second operation timeout, 20-second heartbeat interval, 60-second idle threshold, 128 frames per subscription, and an 8 MiB inbound message/HTTP-response limit. Safe reads get two extra attempts and reconnect gets three, with 100 ms initial exponential backoff capped at two seconds. The reconnect budget resets on native data or 30 seconds of stability. Configure `read_retry`, `reconnect` and `reconnect_stable_after`; use `BackoffConfig::disabled()` to disable recovery. Timeouts/capacities must be positive and idle must exceed heartbeat. `StandardTransport::with_tls` supplies a common custom TLS policy for both protocols.

## Verify changes

```sh
bash scripts/check.sh
```

This runs formatting, Clippy with warnings denied, feature-isolated compilation/tests, offline fixture and local-server tests, and dependency-baseline checks for Rust. Tests include 44 Hyperliquid and 66 Lighter signing vectors, altered-context negatives, public/private account reads, native Hyperliquid batch errors, custom signers/handlers, concurrent nonces, ambiguous/cancelled submissions, stream isolation, fresh snapshots, retries/rate limits, gaps, reconnect exhaustion/stability, expiry, malformed/oversized frames and shutdown. Compile-fail checks verify that unsigned clients cannot trade. `scripts/check-python.sh` separately runs 49 Python tests, including the same 110 reference vectors and recovery/cleanup scenarios. Normal checks use committed fixtures and local servers without exchange requests.

To review a dependency change:

```sh
cargo tree -p aux-rs --features hyperliquid -e features
python3 scripts/dependencies.py --write
```

Review the baseline diff before committing it. The baseline target is explicitly recorded; use `--target` to inspect another platform. Core, each exchange, combined exchanges, custom/standard transport profiles, and the separate Python binding are recorded. Automated checks reject cross-exchange signing and Rust-only Python dependency leaks.

Milestones 1–5 and Python operation parity are implemented. Funded authenticated testnet verification and Lighter cryptographic review remain outstanding release gates. Python wheel distribution and a broader interpreter/platform matrix follow separately.
