# Python bindings

The `aux-py` distribution (import `aux_py`) exposes the implemented Hyperliquid, Lighter, QFEX, Arcus, Monaco, Derive, Paradex and Metric Rust SDK operations. Owned concrete PyO3 clients wrap the same Rust clients, signing, nonce coordination and transports. Python contains asyncio/context-manager glue, not exchange protocol implementations. Rust SDK crates have no PyO3 dependency.

## Coverage

| Capability | Hyperliquid Python | Lighter Python |
|---|---|---|
| Market metadata | `meta(dex=...)` | `order_books(market_id=...)` |
| Native book snapshots | `l2_book(...)` | `order_book_orders(...)` |
| Recent trades over HTTP | Not in the current Rust adapter | `recent_trades(...)` |
| Account state and positions | `clearinghouse_state(user, dex=...)` | `account(account_index=... / l1_address=...)` |
| Open orders | `open_orders(user, dex=...)` | Authenticated `active_orders(market_id=...)` |
| Submission/cancellation | `order`, `cancel`, `cancel_by_cloid`, with optional nonce/expiry | `create_order`, `cancel_order`, with optional expiry/attributes |
| Public streams | Book, trades | Book, trades, account |
| Account streams | Clearinghouse state, open orders, order updates, fills | Authenticated account orders; public account stream |
| Local and custom async signers | `HyperliquidSigner` | `LighterSigner` |
| Offline signing/verification | L1 and user-signed actions, action hashes, payloads, address recovery | Create/cancel transactions, auth tokens, payloads, signature verification |
| Exact value helpers | `validate_hyperliquid_order` | `scale_lighter_decimal` |
| Nonce recovery | Rust signer-scoped coordinator and explicit nonce options | Rust coordinator, public `next_nonce`, explicit `resync_nonce` |

This is parity with the operations currently implemented in Rust. Funding/withdrawal submissions, Lighter key registration/grouped batches and other unimplemented Rust operations remain outside scope. Python wraps concrete standard transports and default native auth handlers; generic Rust transport/auth traits are not Python subclass interfaces. Custom signing is available through explicit async callbacks.

## QFEX and Arcus

`QfexClient` and `ArcusClient` are included in the Python distribution. Each wraps
the same native Rust implementation and shares the existing exact conversion,
error mapping, cancellable subscription, and client shutdown infrastructure.

```python
from aux_py import QfexClient, ArcusClient, ArcusSigner

async with QfexClient(network="testnet") as qfex:
    metadata = await qfex.refdata(ticker="AAPL-USD")
    book = await qfex.order_book("AAPL-USD")

async with ArcusClient(network="testnet") as arcus:
    markets = await arcus.markets(market="BTC-USD")
    book = await arcus.order_book("BTC-USD")
```

QFEX authentication takes `public_key` and `secret_key`, optionally `account_id`
and `builder_code`. Authenticated methods are `positions`, `account_balances`,
`user_orders`, `add_order`, `cancel_order`, and `subscribe_account`; `subscribe(channel, symbol)`
selects public MDS channels. One-shot add/cancel methods authenticate a fresh
trade connection, send once, and close it. Quantity/price inputs accept exact
decimal strings or integers; floating-point inputs are rejected.

Arcus writes take `signer=ArcusSigner(seed_hex)`, `address`, and `account_index`.
`place_order(market_metadata, request)` and `cancel_order(request)` accept native
dictionaries. Markets, books, trades, account, positions, and open orders are
public reads. `subscribe(specification)` preserves native channel envelopes.

See the [QFEX guide](../exchanges/qfex/README.md) and
[Arcus guide](../exchanges/arcus/README.md) for native units, credentials,
supported methods, and pending authenticated testnet verification. The Python
suite has 49 tests, including local authenticated peers for both new exchanges.

## Build and verify

Python 3.10+, Rust 1.92+, and the checked-in lockfile are required. From `connectors/`:

```sh
python3.12 -m venv .venv-python
.venv-python/bin/pip install -r crates/python/dev-requirements.txt
bash scripts/check-python.sh
```

`AUX_PYTHON=/path/to/venv/bin/python` selects another venv. Maturin installs a local development extension. Runtime dependencies are the standard library and compiled extension; Maturin and websockets are development-only. The locally verified interpreter/platform is CPython 3.12.7 on macOS arm64. The release workflow below checks additional interpreters and platforms.

Offline tests exercise all bound operations and stream types, the 44 Hyperliquid and 66 Lighter reference vectors, exact values and unknown JSON numbers, partial batches, auth context, callback context/cancellation, concurrent nonces, ambiguous submissions, errors, overflow and cleanup. `bash scripts/check.sh` separately verifies the Rust-only feature/dependency matrix. Neither check makes exchange requests or places orders.

```sh
# Explicit read-only checks for both testnets; no credentials required.
.venv-python/bin/python crates/python/examples/public_testnet.py
```

Funded testnet submit/cancel and real authenticated Lighter order-stream verification still need configured credentials. Lighter's local cryptographic primitives remain experimental and unaudited; see [the signing notes](../exchanges/lighter/SIGNING.md). Python bindings do not remove those release gates.

## Releases

The [Python release workflow](../../.github/workflows/release-python.yml) builds
CPython 3.10–3.14 wheels for Linux x86_64 (glibc 2.28+) and macOS arm64 and x86_64,
plus a source distribution containing the local Rust workspace dependencies.
Each wheel and the source distribution are installed and run through the Python
test suite before publication. Windows, Linux arm64, and free-threaded Python
wheels are not included in this initial matrix.

Pull requests affecting the build inputs and manual workflow runs build and test
artifacts without publishing. Publishing a GitHub release with a tag matching
`aux-py-v<VERSION>` publishes those artifacts to PyPI using Trusted Publishing.
Other release tags are ignored. The first version is **0.0.0**, with tag
`aux-py-v0.0.0`.

Before the first release:

1. Create a GitHub Actions environment named `pypi` in `polaris-data/aux`.
2. In your PyPI account's [Publishing settings](https://pypi.org/manage/account/publishing/),
   add a pending GitHub publisher with project name `aux-py`, owner `polaris-data`,
   repository `aux`, workflow filename `release-python.yml`, and environment `pypi`.
3. Merge the workflow and run **Release Python SDK** manually on `main` to check
   all builds. This run does not publish or reserve the package name.
4. Publish a GitHub release tagged `aux-py-v0.0.0` from the verified commit.
   The workflow creates the PyPI project on its first successful upload.

For subsequent releases, update both `[project].version` in `pyproject.toml` and
`[workspace.package].version` in the root `Cargo.toml` before tagging. The workflow
rejects mismatches between those versions and the release tag. No PyPI API token
is required. See [PyPI's Trusted Publishing guide](https://docs.pypi.org/trusted-publishers/creating-a-project-through-oidc/)
for the account setup.

## Public reads and streams

```python
import asyncio
from aux_py import HyperliquidClient, LighterClient

async def main():
    async with HyperliquidClient(network="testnet") as client:
        meta = await client.meta()
        async with await client.subscribe_l2_book("ETH") as books:
            book = await anext(books)
            print(book["levels"][0][0]["px"])  # Native decimal string.

    async with LighterClient(network="testnet") as client:
        markets = await client.order_books()
        market = next(m for m in markets["order_books"] if m["status"] == "active")
        async with await client.subscribe_order_book(market["market_id"]) as books:
            event = await anext(books)
            print(event["type"], event["order_book"]["nonce"])

asyncio.run(main())
```

Constructors can run outside an asyncio loop and make no exchange requests. Testnet is the default; select mainnet explicitly. Override `http_url` and `ws_url` together for proxies/local servers; network still determines signing context. Lighter additionally accepts `chain_id` with custom endpoints.

`HyperliquidClient.subscribe(specification)` accepts the native Rust subscription dictionary. Convenience methods cover trades, clearinghouse state, open orders, order updates and fills. These generic streams yield native `{ "channel": ..., "data": ... }` envelopes, including subscription acknowledgements. The existing `subscribe_l2_book()` continues yielding bare native book snapshots and skips acknowledgements/pongs.

Lighter streams retain the complete native envelope, including `type`, `channel`, nonce/offset metadata and unknown fields. `subscribed/order_book` is a snapshot and `update/order_book` is a delta; zero-size levels remain in deltas. Fresh duplicate subscriptions receive their own snapshot. The SDK does not reconstruct a book. Standard transports reconnect with an explicit recoverable `GapError` before fresh data; applications own reconciliation after gaps.

All clients and iterators support `async with`, `closed`, and idempotent `await aclose()`. Close shuts down owned subscriptions and cancels pending client work. Only one pending `anext()` is allowed per iterator; competing reads raise `BusyError`. Cancelling a started read closes that iterator. Rust cleanup continues if the close await is cancelled or the Python object is dropped. Malformed/native-error/overflow streams and exhausted recovery close with structured exceptions. A recoverable `GapError` leaves the iterator open. Use explicit context management for deterministic cleanup.

## Native values and request dictionaries

Responses retain native field names. Decimal strings stay strings; integer IDs use Python integers, including values beyond 64 bits in unknown JSON fields. Other JSON numbers become `decimal.Decimal` from their source text. No monetary value is routed through a Python float.

Request dictionaries use the Rust adapters' native wire keys. Floating-point inputs and `Decimal` objects are rejected; explicitly supply native decimal strings or scaled integers. Input nesting is bounded to reject cycles safely. Examples below illustrate shape and require application-owned account/order values before execution.

```python
from aux_py import HyperliquidClient, HyperliquidSigner

signer = HyperliquidSigner(private_key_hex)  # Native Ethereum-style 32-byte key.
async with HyperliquidClient(signer=signer, account_address=account, network="testnet") as client:
    # Agent identity stays separate from the traded account. vault_address is optional.
    result = await client.order({
        "orders": [{"a": asset, "b": True, "p": "1670.1", "s": "0.0147",
                    "r": False, "t": {"limit": {"tif": "Alo"}}}],
        "grouping": "na",
    }, expires_after=expiry_unix_millis)
    # Preserve the native status/response and every per-item status, including errors.
```

```python
from aux_py import LighterClient, LighterSigner, scale_lighter_decimal

signer = LighterSigner(native_private_key_hex)  # Native 40-byte little-endian scalar.
async with LighterClient(signer=signer, account_index=account_index,
                         api_key_index=api_key_index, network="testnet") as client:
    result = await client.create_order({
        "MarketIndex": market_id, "ClientOrderIndex": unique_client_order_index,
        "BaseAmount": scale_lighter_decimal(size_text, size_decimals),
        "Price": scale_lighter_decimal(price_text, price_decimals),
        "IsAsk": 1, "Type": 0, "TimeInForce": 2, "ReduceOnly": 0,
        "TriggerPrice": 0, "OrderExpiry": expiry_unix_millis,
    })
    await client.cancel_order({"MarketIndex": market_id, "Index": unique_client_order_index})
```

Read/query identities remain explicit and need no signer where the native API is public. Trading without configured credentials raises `AuthenticationError` before network access. Signed fields stay inside Rust until submission; output dictionaries from offline signing are inspectable serialized copies.

Hyperliquid preserves its native batch result, including top-level rejection and mixed item outcomes. Lighter code `200` means submission acceptance, not execution. No trading request is replayed automatically. Lighter timeouts, rejection, malformed responses and cancellation during transport quarantine the shared chain/account/API-key sequence, including across independent client instances. Reconcile pending transactions before explicitly calling `resync_nonce()`; `next_nonce()` alone does not reset quarantine. Cross-process coordination remains the application's responsibility.

## Signers, callbacks and offline workflows

`HyperliquidSigner.sign_action(...)` returns a native signed envelope with an explicit nonce. `sign_user_action(...)` supports offline `usdSend` and `approveAgent` primitives; it does not submit those workflows. `LighterSigner.sign_transaction("create" / "cancel", request, ...)` returns `tx_type`, native `tx_info` and digest. Explicit offline nonces are caller-managed. `LighterSigner.auth_token(...)` and authenticated `client.auth_token(deadline)` return bearer credentials; deadlines use Unix seconds, while transaction/order expiry uses milliseconds. Do not log tokens or signatures used as auth credentials.

Pure helpers construct immutable `HyperliquidSigningPayload` or `LighterSigningPayload` objects: `hyperliquid_signing_payload`, `hyperliquid_user_signing_payload`, `lighter_transaction_signing_payload`, and `lighter_auth_signing_payload`. Payloads expose canonical digest bytes and native context, support address recovery/signature verification, and can be passed to `await signer.sign(payload)`.

```python
from aux_py import HyperliquidSigner

async def remote_sign(payload):
    # Check payload.network and payload.scheme, then send payload.digest to your
    # signing service. Return native {"r": "0x...", "s": "0x...", "v": 27/28}.
    return await signing_service.sign_hyperliquid(payload)

signer = HyperliquidSigner.remote(signer_address, remote_sign)
```

`LighterSigner.remote(public_key_hex, callback)` follows the same pattern, returning an 80-byte Schnorr signature encoded as hex. Its payload also exposes chain/account/API-key indexes and transaction/token purpose. Callbacks execute on the initiating asyncio loop with its `contextvars`; cancelling native work cancels the callback task. Rust verifies returned signatures against the original payload and declared identity before submission. Callback exceptions become sanitized `SigningError` values. Local keys and callback internals are excluded from signer `repr`.

The caller's original Python key string cannot be zeroized by the extension; its Rust input copy is cleared and native key storage follows the Rust signer's behavior. See Lighter's signing notes for limitations of its underlying primitives.

## Errors

All errors derive from `ConnectorError`: `ValidationError`, `AuthenticationError`, `SigningError`, `NonceUncertainError`, `DecodeError`, `ExchangeError`, `TransportError`, `GapError`, `ClosedError`, and `BusyError`.

`ExchangeError` exposes `venue`, `http_status`, `body`, `native_message`, native `code` where available, and exact `extra` fields. Server text remains explicitly inspectable but is omitted from exception `args`/`str`/`repr` because it can echo credentials. `TransportError` includes `kind` and `outcome` (`not_sent` or `unknown`). Native batch item errors remain in their response rather than being collapsed into a single exception.

Network, hashing and signing work runs in Rust without holding interpreter access. Interpreter access is acquired for Python argument/result conversion and user callbacks. PyO3's [owned-class boundary](https://pyo3.rs/main/class.html) and [async integration](https://pyo3.rs/main/ecosystem/async-await.html) remain confined to this crate.

## Recovery controls

Milestone 5 adds safe-read retry and reconnect policies. Defaults are two extra read attempts, three extra reconnect attempts, 100 ms initial exponential backoff capped at two seconds, and a 30-second connection stability threshold. Configure `read_retries`, `reconnect_retries`, `initial_backoff_ms`, `max_backoff_ms`, `reconnect_stable_ms`, `heartbeat_interval_ms`, `idle_timeout_ms` and `max_message_bytes` on either client. Set retry counts to zero for the previous one-attempt/terminal behavior. Existing timeout and buffer controls remain available.

`GapError` derives from `TransportError`. Catch it within the stream's context manager, invalidate/reconcile derived state and call `anext()` again; the iterator stays open. Old buffered frames are cleared before recovery data. Terminal overflow, protocol/size errors, expired replay credentials or exhausted recovery close the iterator. Private Lighter token refresh requires a new authenticated subscription. A plain `async for` does not suppress gap exceptions.

Client `aclose()` also shuts down the shared Rust transport and waits for its handshakes, requests and socket drivers. Safe reads honor bounded `Retry-After` handling; signed trading is never automatically retried, even after timeout/shutdown. The 49-test Python suite covers the existing 110 signing vectors, both new exchange adapters, recovery, budget, backoff cancellation, and no-replay cases.

## Derive v3 and Paradex

`DeriveClient` and `ParadexClient` expose every named REST operation in their
committed coverage manifests (128 and 142), native subscriptions, local or async
remote signers, and isolated RPC sessions. `DeriveSigner` accepts an EVM key;
`ParadexSigner` accepts Starknet or EVM keys. Both default to testnet. Decimal
strings remain strings, JSON numbers decode to exact `Decimal`/`int` values,
and remote callbacks run in the initiating asyncio context.

```python
from aux_py import DeriveClient, ParadexClient

async with DeriveClient() as derive:
    instrument = await derive.public_get_instrument({"instrument_name": "ETH-PERP"})
async with ParadexClient() as paradex:
    config = await paradex.get_system_config()
```

See the [Derive guide](../exchanges/derive/README.md) and
[Paradex guide](../exchanges/paradex/README.md) for authentication, signing,
all operation names, subscriptions, RPC commands, and native request bodies.
`open_rpc()` returns `DeriveRpcSession` or `ParadexRpcSession`; an uncertain
submission remains in `pending` until `reconcile()` resolves its response.
Paradex `authenticate()` explicitly replaces shared JWT state. Active sockets
retain their established permissions, while reconnects require valid credentials.
`examples/derive_paradex_public.py` runs public network checks separately from CI.
Funded submit/cancel verification remains pending without credentials.

## Monaco

`MonacoClient`, `MonacoWalletSigner` and `MonacoSessionSigner` expose the native
Rust spot/perps connector. Read methods accept native request dictionaries;
`login` authorizes a session, `restore_session` accepts caller-persisted credentials,
and refresh/revoke update shared session state. Public and private streams use
`subscribe` with native channel parameters. See the [Monaco guide](../exchanges/monaco/README.md)
for exact schemas, signing, trading and uncertain-outcome behavior. The explicit
read-only network example is `examples/monaco_public.py`. Offline tests cover
native parity, exact values, callback context/reconnect/cancellation and lifecycle.

## Data vendors

`aux_py` also exports `TardisClient`, `TardisMachineClient`,
`DatabentoHistoricalClient`, `DatabentoReferenceClient` and `DatabentoLiveClient`.
These use the same Rust data clients and native schemas. See the
[Tardis guide](../data/tardis/README.md) and
[Databento guide](../data/databento/README.md) for API coverage and examples.

Record streams support async iteration, `read_batch(limit=256)` and `aclose()`;
byte streams yield `bytes`. Async context managers provide deterministic cleanup.
Cancellation closes the stream and parent closure also closes live TCP sessions.
`DataVendorError` retains sanitized `.native` details, `.http_status` and `.kind`.
Normal tests use local HTTP/WebSocket/TCP servers and synthetic codec fixtures;
no paid data requests run in the check scripts.

## Metric

`MetricClient(chain_id=...)` exposes all eight documented HTTP endpoints. `MetricChainClient(chain_id=..., read_rpc=..., signer=...)` provides typed contract reads, named transaction builders, EVM signing/submission, logs, receipts and explicit nonce recovery. Both support async context managers. `MetricSigner.remote(address, async_callback)` uses the initiating asyncio context and verifies returned signatures in Rust.

```python
from aux_py import MetricClient, MetricChainClient

async with MetricClient(chain_id=1) as client:
    pools = await client.metadata({"count": 1})

async with MetricChainClient(chain_id=1, read_rpc=rpc_url) as chain:
    pool = chain.contract("MetricOmmPool", pool_address)
    immutables = await pool.get_immutables()
    prepared = pool.prepare_set_pause([1])
```

The [workflow guide](../exchanges/metric/WORKFLOWS.md) documents pool-creation preparation, native ETH liquidity, external-swap fallbacks and combined stop-loss evaluation, including `oracle_stop_loss_status` for reads anchored to one block.

The second builder returns inspectable calldata; explicit `send_transaction()` performs the write. The `aux_py.metric` module exposes the same Rust packing, oracle, liquidity-planning and native-payment helpers. Rust retains the ABI and protocol logic. See the [Metric guide](../exchanges/metric/README.md) for native units, API-key versus wallet authentication, the contract inventory and verification results.

## Prediction exchanges

`PolymarketClient`, `PolymarketPerpsClient`, `KalshiClient`, `KalshiPerpsClient` and `PascalClient` expose the pinned native REST operations and stream/command sessions. Requests use native mappings: path/query parameters at the top level, JSON request payloads under `body`. Prediction/perps credentials and models are separate. Local and async callback signers, exact `Decimal` conversion, structured exceptions and context-managed cleanup use the same Rust implementations. `PolymarketChainClient` handles explicit EVM operations; `PascalChainClient` handles explicit Solana USDC deposits and separate chain/API credit reconciliation. Floats are rejected recursively.

See [Polymarket](../exchanges/polymarket/README.md), [Kalshi](../exchanges/kalshi/README.md) and [Pascal](../exchanges/pascal/README.md) for native signing authority, endpoint overrides, deployment pins and publication limits. `examples/prediction_public.py` is an explicitly invoked public smoke check. `examples/prediction_write.py` requires a caller-reviewed JSON request and `--execute`; mainnet additionally requires `AUX_ALLOW_MAINNET=1`. No funded verification is run in CI.
