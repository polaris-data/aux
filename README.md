# aux

aux connects to exchanges and market data providers through Zig, Rust, TypeScript and Python SDKs. The Zig library owns HTTP and TLS, JSON tokenization, request encoding, exchange response handling and the binary codecs listed below. The Rust, TypeScript and Python packages call its versioned C ABI. The reference snapshot for the initial fixtures is aux commit `0b9b24f29c6ea5d23ddb0d470e196594acb9bccc`.

Zig applications import the public `aux` module from `sdk/zig/root.zig`. The static library for the other SDKs builds from `src/abi.zig`. Shared transport, wire formats and the C ABI live in `src/core/`. Exchange protocols live in `src/exchanges/<vendor>/` and data providers in `src/data/<vendor>/`. Each vendor keeps its SDK model inputs in `schema/` and its third-party attributions in `notices/` where applicable.

## Zig-native SDK

Use Zig 0.17.1 and add this directory as the `aux` dependency in your application's `build.zig.zon`. The external consumer in `tests/zig_consumer` shows a complete local path dependency. To use the verified source archive, run `zig fetch --save=aux /absolute/path/to/aux-zig-native-0.0.1.tar.gz` in the consuming project. Import the module in your application's build script.

```zig
const aux = b.dependency("aux", .{ .target = target, .optimize = optimize });
exe.root_module.addImport("aux", aux.module("aux"));
```

All 1,948 named HTTP methods use the shared SDK schemas. Model declarations live beside their client, for example `aux.clients.pyth_hermes.LatestPriceUpdatesRequest`. The compiler checks the request and infers the response type.

```zig
const aux = @import("aux");
const std = @import("std");

pub fn latest(allocator: std.mem.Allocator, ids: []const []const u8) !void {
    const client = aux.clients.PythHermesClient.init(allocator, .{});
    var reply = try client.latest_price_updates(.{ .ids = ids, .parsed = true });
    defer reply.deinit();
    std.debug.print("Hermes HTTP status {d}\n", .{reply.value.status});
}
```

Clients borrow their allocator, endpoint strings, credentials and callback contexts. Each successful request returns an `Owned(T)` whose arena owns the response. `aux.parseAs(T, allocator, bytes)` applies the same decoder to caller-supplied JSON. Open JSON fields preserve numeric tokens as `number_string`. Schema fields that accept arbitrary JSON numbers use `Number.text`. Supply a separate `Diagnostic` per concurrent operation to retain the venue's structured error. Streams own their handles. A single reader can call `next()` for JSON or `nextBytes()` for binary data. Call `cancel()` to wake a blocked reader, join it, then call `deinit()`. Each returned JSON event owns its storage. Free byte chunks with the client allocator.

| Zig API | Functionality |
| --- | --- |
| `aux.clients` | Named HTTP clients, schema types, streaming endpoints, trading and RPC session openers |
| `aux.sessions` | Commands for open exchange, Databento and Tardis sessions |
| `aux.chain`, `aux.contracts` | EVM reads, coordinated writes, Metric's named contract methods, Polymarket ABI calls, Phoenix and Pascal Solana transactions |
| `aux.workflows`, `aux.wallet`, `aux.identity` | Offline helpers, Phoenix instruction builders, local and callback signing, onboarding, login and restore |
| `aux.Client.invoke`, `invokeBytes`, `openDbn` | Direct access to the native operation inventory, raw report decoding and DBN files |
| `aux.Options.signer`, `transport`, `dial` | Application signing callbacks, unary HTTP exchanges and connected sockets for custom transport policies |

From this directory run `python3 check_zig_sdk.py` to fetch the source archive into a fresh Zig project, compile the consumer, and exercise the fixtures. It writes the verified archive to `dist/`. The generator check is `python3 tools/generate_clients.py --check`. The consumer itself builds using Zig and libc.

## Using the SDKs

The package names are `aux-rs` for Rust, `aux-connectors` for Node and TypeScript, and `aux-py` for Python (import `aux_py`). These packages expose the Zig-backed client interfaces documented below. Applications moving from the Rust-core releases should review constructor options, Cargo features and method signatures.

The Python and Node clients run network I/O off the application's event loop. Rust offers synchronous methods and optional Tokio adapters for unary requests, event subscriptions and trading sessions. These examples use the same request parameters.

```python
from aux_py import ExtendedClient

async def eth_price():
    return await ExtendedClient().get_asset_price({"asset": "ETH"})
```

```ts
import { ExtendedClient } from "aux-connectors";

const price = await new ExtendedClient().getAssetPrice({asset: "ETH"});
```

```rust
use aux_rs::ExtendedClient;
use serde_json::json;

let price = ExtendedClient::new().get_asset_price(&json!({"asset": "ETH"}))?;
```

Rust applications can enable the optional `rustls` Cargo feature and pass a `rustls::ClientConfig` to a named client's `with_tls`. The adapter executes the application’s certificate verifier and client identity selection for unary HTTP, streaming HTTP and WebSockets, including reconnects. Zig frames HTTP/1.1 and WebSocket traffic over the verified connection. For an application-owned HTTP client, `with_transport` accepts an `aux_rs::http::Transport` or a thread-safe request closure. Implement `connect_stream` on the transport to supply a connected socket for streaming or Lighter’s nonce-managed submissions. The Rust SDK links the bundled native archive with either option.

Python and TypeScript expose the core's authentication, transport, exchange, validation, decode and uncertain-outcome errors as subclasses of `ConnectorError`. Rust exposes these as `aux_rs::Error` variants.

The HTTP methods accept wire-shaped objects. Named Python unary calls accept the reference `request=` keyword and the standalone `params=` keyword. Dune also accepts separate query or execution identifiers and result options in Python, and the corresponding `queryId` or `executionId` options in TypeScript. Python and TypeScript expose request and response models for all 1,948 named unary methods in the standalone catalog. Rust provides the same contracts through venue Cargo features such as `metric-models` and `lighter-models` and the client's `typed()` interface. Some reference methods return arbitrary JSON or accept open parameter maps. Their SDK types preserve that flexibility. The default Rust interface accepts JSON with arbitrary precision. Python decodes decimal tokens to `Decimal`. Node returns unsafe integers as `bigint` and fractional or exponent tokens as strings. [Exchange sources](src/exchanges/) · [Data sources](src/data/)

The default native HTTP client reuses keep-alive connections. Up to eight pooled clients each retain one idle connection. Overlapping calls can still open additional connections. Common unary exchange errors carry the venue, HTTP status and lossless response body. Hyperdash masks secrets in its error payloads.

## Port coverage

The REST catalog has 1,448 fixture-backed unary routes. Hyperdash adds CSV and NDJSON variants for 250 routes, bringing the catalog to 1,948 named unary methods. Nine cataloged HTTP streams cover Hermes SSE, Chainlink candlesticks and Databento data. Tardis adds four HTTP streaming operations outside this catalog, including dataset downloads. Injective uses gRPC-Web for indexer subscriptions. Session commands, subscriptions, chain calls and offline signing helpers have separate coverage.

| Area | Implemented in Zig |
| --- | --- |
| Exchange and data HTTP | Derive, Extended, Paradex, Phoenix, Kalshi, Pascal, Polymarket, Birdeye, Hyperdash, Dune, Tardis, Hyperliquid, Lighter, Databento, Pyth, Chainlink, QFEX, Arcus, Metric, Monaco, Injective, Nado, Ondo Perps, Pacifica, Paragon, trade.xyz and Aster. Pyth and Chainlink each have two service catalogs. |
| HTTP credentials | QFEX positions and account balances use fresh HMAC headers. Derive uses wallet login proofs. Polymarket CLOB supports wallet login, order signing and L2 HMAC. Polymarket Perps supports proxy credentials and signed operations. Paradex accepts scoped bearer tokens and signs single, batch and modified orders, block trades and offers with Stark or EVM keys. Pascal signs all 12 owner permits. Polymarket supports relayer and builder service credentials. Other services use venue-specific key, bearer, Basic and Chainlink HMAC headers. Extended supports Stark settlement hashes and signatures for orders, limit orders, transfers and withdrawals. Order preparation uses exact decimal quantities, side-dependent price rounding and rounded-up fees. Attached and position-sized take-profit and stop-loss legs are signed independently. Extended and Phoenix support API keys. Kalshi predictions and perps use a key ID and a hex Ed25519 seed. Clearing uses a bearer token. |
| Streaming | Hermes SSE price updates and Chainlink concatenated JSON price records handle fragmented inputs, cancellation and repeatable close. Extended supports eight market and account channels through path-based and JSON-RPC subscriptions, with topic filtering and order-book snapshot recovery. Pascal read subscriptions and Kalshi predictions, perps and warp subscriptions support reconnects and subscription replay. QFEX and Phoenix market-data sockets support fragmented messages, ping/pong, disconnect recovery and subscription replay. Phoenix detects sequence gaps and decodes compressed snapshots. Paradex supports 19 channel types with JSON and SBE notifications, account and market checks on binary frames, and a fresh order-book snapshot on reconnect. Derive supports 13 public and private channels, with fresh wallet authentication on private reconnects. |
| Order workflows | Arcus place and cancel with local or external Ed25519 signing. QFEX authenticated trading sessions. Dedicated Kalshi predictions and perps sessions handle subscription changes. Pascal command sessions sign place, cancel and mass-cancel permits. Paradex command sessions authenticate, gate writes on token scope and sign order creation, batch and modification. A pending command can be reconciled after its deadline. Hyperliquid L1 order, cancel and cancel-by-client-ID with optional vault, expiry and builder. Lighter signed create and cancel, explicit nonce reconciliation and short-lived auth tokens. Monaco session login, refresh and revoke, signed reads and single and batch order operations. |
| Additional streams | Arcus account, trade and order-book subscriptions. Chainlink authenticated reports. Hyperdash stream sessions with acknowledgement and unsubscribe. Pyth Pro multi-endpoint prices and Merkle roots. Polymarket market, user, sports, PolyBolt, Perps, RTDS and RFQ. Hyperliquid book, trades, order updates, fills, clearinghouse state and open orders. Lighter book, trades, account state and authenticated account orders. Monaco trades, order book, depth, best bid and offer, balances, positions, orders and account events. Tardis hosted minute replay, ranged dataset downloads and Machine HTTP and WebSocket replay and real-time normalization. |
| Metric | Eight HTTP endpoints. Offline helpers cover compressed-oracle packets and signatures, stop-loss evaluation, liquidity planning, unsigned pool creation, swaps and native payment. |
| Contract encoding | Metric calls, outputs, events and reverts. Polymarket calldata and outputs. Legacy EIP-155 and EIP-1559 transaction signing. `EvmReader` implements 12 read-only JSON-RPC methods for balances, blocks, transactions, receipts, gas, logs, code and calls. `EvmChainClient` validates both RPC endpoints, supports code hash pins, submits signed transactions and reconciles uncertain nonces. |
| Phoenix Solana | Prepare legacy and v0 transaction messages with address lookup tables, assemble and verify multiple signatures, and validate server-provided transaction bytes. The chain client fetches blockhashes, simulates transactions, submits once and retains the signed bytes after an ambiguous response. All 67 public instruction-dispatch entries and three further Rust constructors have byte and account-permission vectors. These cover orders, conditionals, collateral swaps and liquidations, spline updates, TWAP, Hawkeye views, Flight and Ember. |
| Polymarket helpers | Token and outcome resolution, deployment-specific order-domain selection, wallet authentication typed data, Perps operation digests, and deposit-wallet batches and session-signature wrapping. |
| Solana collateral | Pascal associated token address derivation, SPL TransferChecked preparation and Ed25519 signing. `PascalChainClient` reads mint and token accounts, prepares deposits with a recent blockhash, checks expiry, submits a signed deposit and reconciles its signature and venue transfer history. |
| Binary market data | Databento DBN v1–v3 files, raw or Zstd-compressed, expose metadata then records one at a time. Record fixtures exercise 23 schemas, optional send timestamps and legacy layouts. Databento live TCP supports CRAM authentication, subscribe, unsubscribe, start, metadata, bounded batch reads and cancellation. Clients can configure connection, write and idle-read timeouts. All 12 templates in Paradex SBE schema 1:3. Chainlink Data Streams report decoding. |
| Text responses | Dune CSV results preserve UTF-8 content and pagination headers. Hyperdash CSV and NDJSON preserve the response text and metadata. Pyth Pro and Hermes expose liveness and readiness checks. |
| Signing primitives | Derive EIP-191 login and EIP-712 Matching actions for orders, RFQs, transfers, session keys, whitelists, liquidations and vaults. Kalshi Ed25519, QFEX HMAC, Databento CRAM, Arcus Ed25519 and Monaco HTTP message signatures. |

### Coverage against the reference SDKs

The Rust crate, Python wheel and Node tarball share fixtures for the dedicated Derive, Polymarket Perps and RFQ command sessions. Ordered commands support pending-outcome reconciliation. A rejected Derive or Paradex JSON-RPC command carries its venue and complete error frame. The Derive client also signs offline action envelopes and raw encoded modules. It checks the supplied owner and signer against the configured credentials. Polymarket typed-data hashing, Extended registration and key derivation, and Paradex onboarding and JWT acquisition have independent wallet vectors.

Metric supports block-pinned liquidity snapshots, oracle stop-loss status, and single- or multi-pool swap preparation. Its 26 named contract interfaces expose 504 reads and transaction builders. The Python and Node package tests exercise every named method against its ABI vector. Rust exposes the named interfaces with the `metric-models` feature. PolymarketChainClient exposes all 22 pinned contract functions through its ABI interface, including transaction preparation and block-tagged reads.

The optional Rust `async` feature adds `into_async()` to named clients. With the corresponding model feature, use `client.typed().into_async()` for typed requests and responses. These calls run native work on the Tokio blocking pool. Dropping a unary or command future aborts queued work. A write already in progress can reach the venue. Async Hermes, Chainlink, Pyth Pro and market-data subscribers retain an outstanding event when its read future is cancelled. Closing or dropping the stream interrupts the native read. Derive, Kalshi, Paradex, Pascal and Polymarket RPC sessions expose async commands, events and reconciliation. QFEX exposes an async trading session.

Python named clients support `async with` and `aclose()`. Node named clients expose `close()`. Closing releases owned subscriptions and rejects subsequent requests. Native operations already in flight may still finish. Hyperliquid public books and trades can share a socket across coins and channels. Lighter public trades, books and accounts can share one connection across filters. Lighter discards deltas until each subscription receives a fresh snapshot after reconnect. A late reader of an active Lighter channel gets a separate connection for its snapshot. QFEX shares public market-data subscriptions across symbols and channels, including wildcard readers. Hyperliquid account streams share only within a normalized user and compatible fill-aggregation setting. Both fields matter because some frames omit their account or aggregation mode. These venues replay active subscriptions when the connection recovers.

Kalshi prediction and perpetual HTTP clients support caller-owned Ed25519 and RSA-PSS signing. Rust accepts `with_request_signer(key_id, sign)`. Python and Node take a key ID and a signer with a `sign` callback. The callback receives the timestamp, method and path as UTF-8 bytes and returns raw signature bytes. The local peer verifies the signature and checks that the query string is excluded.

Metric and Polymarket chain submissions support an external EVM signer. The core checks the signature against the account, rechecks the nonce before dispatch and retains ambiguous outcomes for reconciliation. In Python, Phoenix and Pascal chain clients expose pending submissions through the read-only `pending` property. Their external Solana signers can review the transaction message before signing. Monaco login accepts a wallet callback, and session restore checks the public key, chain and expiry.

For a maintainer comparison, run `python3 tools/audit_reference.py --crates /path/to/reference/crates` from this directory. It inventories Phoenix constructors, Metric ABI functions and public Python client members. For Python it checks positional and keyword argument binding, including inherited methods and chain methods forwarded to native bindings. It reports constructors separately and lists reference constructors whose accepted options are hidden behind `**kwargs`. For TypeScript, run `node tools/audit_node.cjs /path/to/reference/crates` after installing the TypeScript package’s development dependencies. This compares member names on the exported client declarations. Add `--runtime-exports` to either audit to include top-level exports. The Python check uses an installed wheel and the Node check requires both built add-ons. The reference publishes 79 Python names and 61 Node runtime names. The Zig packages currently lack 23 and 29 of those names respectively, including signer classes, signing helpers and error classes. Fixtures establish observable behavior for the cases they exercise. Both audits are independent of package generation and installation.

| Area | Open parity work |
| --- | --- |
| Caller-owned signers | Several authenticated workflows still require local keys, including parts of Derive, Extended, Paradex, Lighter, Monaco and WebSocket authentication. |
| Subscription management | Derive and Monaco still need subscription-sharing coverage across compatible channels. Account isolation, replay and initial snapshots depend on the venue. The reference deliberately uses dedicated scopes for several venues, including Phoenix and Extended. |
| Errors | WebSocket subscription rejections with a JSON error frame retain the venue and full payload, including multiplexed feeds. Rejected HTTP stream opens and WebSocket upgrades can carry the status and up to 64 KiB of decoded venue detail. Authentication failures during a session, failed reconnects and some dedicated protocols can still expose only their error category. |
| Public SDK compatibility | Of 31 Python client names audited, 18 constructors reject at least one reference call form, including endpoint and timeout settings. Another 12 reference constructors take arbitrary keyword arguments and need runtime checks. TypeScript member inventory does not cover option types or overloads. Rust traits, builders, typed session return values and transport shutdown guarantees still require comparison with the reference. |

### Perpetuals connectors

All seven clients expose `markets`, `order_book`, `trades`, `account`, `positions`, `open_orders`, `place_order`, `cancel_order`, and `subscribe`. TypeScript uses camelCase. Unary methods use native request fields. Mutating request payloads use a `body` field except when the venue identifies the order in the route. Streams return native events as they arrive, with exact decimal strings and integer tokens.

Supply venue settings in Python or TypeScript as `auth`, in Rust through `with_auth`, or in Zig as `Options.auth`. The clients default to mainnet. Set `network` to `Testnet` for Injective, Nado, Ondo Perps sandbox, Pacifica, or trade.xyz. To use another streaming endpoint, pass it to `subscribe` in Rust, Python, or TypeScript, or set `Options.endpoint` in Zig.

| Client | Existing credentials and order workflow | Subscription fields |
| --- | --- | --- |
| Injective | `key` is a secp256k1 private key. Use the subaccount derived from that key. Supply a worst acceptable price for market orders, market `quote_decimals`, `gas_limit`, and `fee_amount` in base INJ units. `lcd_origin` selects a separate chain endpoint. After an uncertain broadcast, use `pending` to get its hash and `transaction` to check inclusion. | `channel` is `order_book`, `trades`, `orders`, `positions`, or `fills`. Use `market_id` for public feeds and `subaccount_id` for account feeds. |
| Nado | `key` is an existing wallet or linked signer key, with optional `subaccount` binding. Pass the native order expiration, nonce, and appendix. Use the IOC appendix for price-bounded market orders. `order_status` takes an archive query by digest. Use `rest_origin` for the contract lookup when opening an authenticated stream on a custom endpoint. | Use `type` with `trade`, `book_depth`, `best_bid_offer`, `order_update`, `fill`, or `position_change`. Pass `product_id` for market feeds and `subaccount` for account feeds. Bootstrap depth from `order_book` using its timestamp. After a stream gap, discard the local book, request a fresh snapshot, and apply only newer diffs. |
| Ondo Perps | Supply a JWT as `auth.token` or an HMAC key ID and secret through the existing SDK credential options. Use `order_status` with an exchange order ID or a `client:` ID to reconcile a write. | Use `channel` with `tradesPerps`, `depthBooksPerps`, `ordersPerps`, `fillsPerps`, `positionsPerps`, or `balancePerps`. Account streams require a JWT. |
| Pacifica | Supply `account` and a hex Ed25519 `seed`. Include `agent_wallet` when the seed belongs to a linked agent. Use `place_market_order` for an order with a slippage limit. Use `order_status` with an exchange order ID or check `open_orders` by client ID. | Use `source` with `trades`, `book`, `bbo`, `prices`, `account_info`, `account_positions`, `account_order_updates`, or `account_trades`. Include `symbol` or `account` where applicable. Book aggregation defaults to `1`. |
| Paragon and trade.xyz | Supply a Hyperliquid `key` and optional `vault_address`. Provide a fully qualified `coin` such as `para:TOTAL2` or `xyz:TSLA` with each order or cancel. The SDK resolves the asset ID before signing. Use an IOC limit with an explicit price bound for market execution. `order_status` takes an order ID or client order ID. | Hyperliquid subscription types `trades`, `l2Book`, `orderUpdates`, `userFills`, `clearinghouseState`, and `openOrders` use the client's pinned namespace. |
| Aster | Supply the master wallet address as `user` and the existing API wallet private key as `key`. To override the REST origin for WebSocket listen-key management, set `rest_origin`. Use `order_status` by order ID or original client order ID. | Use `channel` with `agg_trade` or `order_book` and a `symbol`. Use `account` for the authenticated listen-key stream, which includes order changes and fills. |

### Trading credentials and streaming accounts

Extended order preparation takes a chain ID, Stark public key, order body and settlement context. The context specifies the assets, their resolutions and the collateral position. Use `extended_prepare_order` in Python, `extendedPrepareOrder` in TypeScript or `aux_rs::extended::prepare_order` in Rust. The matching signing methods accept a private key and return the signed body. The settlement helper also hashes typed limit orders, transfers and withdrawals for an external signer.

For a Derive client, `api_key` is the owner wallet address and `api_secret` is the local secp256k1 signing key. Rust uses `with_credentials`. Order placement and replacement fetch public instrument metadata before signing. RFQs look up each distinct leg, and transfers and withdrawals resolve an unambiguous spot asset. Metadata reads use the same origin and transport as the order and contain no private headers. A failed lookup stops the write. Pass the nonce and expiry to reproduce a signature, or let the core generate them. Generated nonces are coordinated within one process. Use `prepare_signed` to sign a request with explicit module data and `sign_action` to sign an already encoded envelope. Paradex clients expose `block_signature` for offline block trades and offers with either a Stark or EVM signer. Pascal clients expose `wallet_outcome_pending` for checking an ambiguous wallet write before reconciliation.

The official production and testnet hosts select Ethereum chain IDs 1 and 11155111 respectively. For another origin, set `chain_id` in Python, `chainId` in TypeScript, or `with_signing_chain_id` in Rust. Offline signing is available through `derive_sign_action`, `deriveSignAction`, and `aux_rs::derive::sign_action`. The shared signing fixtures include 17 action examples and both networks.

Paradex accepts a caller-supplied JWT through `bearer={token, read_only}` in Python or the equivalent TypeScript client option. The core checks its expiry on dispatch. Read-only is the default. In Rust use `with_bearer_token(token, read_only)`. To subscribe to a private channel, include the Starknet `account` in the bearer configuration, or use `with_account_bearer(account, token, read_only)`. Each private reconnect authenticates before replaying the subscription. Clients can attach a new token after expiry.

For native Paradex order signing, include `account`, `network` and either `stark_key` or `evm_key` in the bearer configuration. Rust exposes `with_signer` and `ParadexSigner`. Set the Starknet chain name explicitly on custom origins. The fixture suite checks the message digest and signature on mainnet and testnet for both key types.

Pascal supports local and caller-owned Ed25519 keys for all 12 HTTP permits and the three signed WebSocket commands. Python accepts `owner` and a `PascalSigner.remote(public_key, callback)`. Node accepts a `permit` with `owner`, `public_key` and a `sign(bytes)` callback returning a base58 signature. Rust exposes `with_external_permit_signer`, whose callback receives the permit bytes as hex. Zig checks the proof against the outgoing request and the receive window before sending. For a local key, pass an owner, hex seed, deployment ID and receive window in `permit`, or use Rust’s `with_permit`. Use the venue's deployment ID for the destination host. The core coordinates concurrent submissions by deployment and wallet within the process. After an ambiguous write, check the venue's order or transfer state before calling `reconcile_wallet` (TypeScript `reconcileWallet`) to release the guard. Applications must retain enough context to reconcile across process restarts.

Polymarket CLOB accepts `clob` credentials with `address`, `key`, base64 `secret`, and `passphrase`. Supply a separate `wallet` with a hex signing `key` for API-key creation and derivation. Order placement supports individual and batch requests. For unsigned orders also supply `wallet.order_domain` with `chain_id`, the exchange contract address in `exchange`, and `version` (2 or 3). Set `wallet.address` for a distinct funding wallet. Rust exposes `with_clob_wallet` and `with_clob_credentials`. The core signs the order before computing the L2 HMAC over the outgoing bytes.

For service endpoints, `services.relayer` supplies the relayer key and address. `services.builder` supplies the builder key, base64 secret and passphrase. Gateway writes that require both CLOB and builder credentials sign both sets of headers over the same outgoing request.

Polymarket Perps takes a `perps` object or Rust's `with_perps_auth`. Account requests use `proxy` and `secret` with optional `expires_at_ms`. Trading uses `proxy_key`. Proxy creation, revocation and withdrawals use `owner_key`. Withdrawals also require the `withdrawal_contract` in the signing domain. The native signer uses Polygon chain ID 137. Supplying both `ts` and `salt` in the order envelope makes the signature reproducible.


For **QFEX**, open `trading_session` (`tradingSession` in TypeScript) with API key and secret credentials. Keep the session to consume account events and late acknowledgements. An acknowledgement timeout leaves the command outcome pending. Further writes on that session return an uncertain-outcome error until the late acknowledgement is read. The connector never resends the order automatically.

For **Arcus**, an external signer receives the canonical request bytes and timestamp from the core. For **Hyperliquid**, an external signer can provide the signature for orders and cancellations. Zig recovers and checks the configured signer address before dispatch. With a local signer, configure `key`, `network` (`Mainnet` or `Testnet`) and an optional `vault_address`. Signed orders use canonical decimal strings. `user_signing_payload` and `sign_user_action` also support offline USD-send and agent-approval signatures. For **Lighter**, configure `chain_id`, `account_index`, `api_key_index` and the 40-byte hex key. Use `prepare_transaction` to inspect the digest. After an uncertain submission, reconcile with the venue and call `resync_nonce` before submitting again. The sequence is shared across clients for that signing identity within the process.

For **Monaco**, `login` accepts the wallet address, client ID and Ed25519 session seed, plus a wallet-signing callback. Set `chain_id` for a custom origin. The callback receives a challenge only after Zig checks the expiry and session public key. The returned client can refresh and revoke its session. A registered session can also be supplied through the Python or TypeScript `session` option or Rust’s `with_session`. Choose `legacy` or `rfc9421` request signatures. Order placement and replacement take a `body` and the trading pair’s `market` metadata for tick, quantity and leverage checks. Batch creation and replacement accept a `markets` array. A lost acknowledgement on a dispatched order yields an uncertain-outcome error.

Monaco private subscriptions authenticate again after reconnecting. Pin a custom socket URL with `session.ws_origin` in Python and TypeScript or `with_session_ws_origin` in Rust. Session revocation and expiry interrupt blocked private reads. A missing baseline raises a stream gap and leaves the server notification available on the next read. A broken depth sequence ends the subscription.

For **Pyth Pro**, pass the subscription request and optional `endpoints` to `subscribe`. All configured sources must accept the initial handshake. Each source has its own reader and replays its subscription after a broken connection. The merged stream suppresses duplicate price updates within a 4,096-update window. **Polymarket** subscriptions take a `service` and service-specific `params`. **Tardis** hosted replay uses an exclusive end time and emits recorded disconnects. `TardisMachineClient` exposes the separate Machine endpoints.

### Chain submissions and Databento data

`EvmChainClient` accepts a chain ID, separate read and write RPC URLs, and optional contract code hash pins. Its local-signer flow coordinates nonces by chain and sender across client instances in the process. After an ambiguous send, use `pending` and `reconcile`, or explicitly resynchronize using the expected pending hash. Raw submission validates the signed envelope, chain and returned hash. The caller tracks raw submissions. The Pascal Solana client holds pending state per instance and checks the exact signature during reconciliation. Both chain clients use native HTTP.

Pass bytes to `aux_py.decode_dbn_file` in Python, `decodeDbnFile` in TypeScript or the Rust DBN file reader. Cursors own their input and decompress records on demand. HistoricalClient.get_range yields decoded metadata and records. HistoricalClient.timeseries returns the requested wire bytes. HistoricalClient.download reads a batch URL, and download_to publishes a complete file at a new path. API credentials apply only to downloads from the configured origin. ReferenceClient streams newline-delimited JSON for security master, corporate actions and adjustment factors.

For a Databento live feed, connect a LiveClient with a dataset, subscribe before starting the session, then consume the metadata and records. The first batch includes the metadata event.

```python
from aux_py import LiveClient

async def first_batch(api_key):
    session = await LiveClient(api_key).connect({"dataset": "GLBX.MDP3"})
    try:
        await session.subscribe({"schema": "trades", "symbols": ["ES.FUT"], "stype_in": "parent"})
        await session.start_session()
        return await session.read_batch(10)
    finally:
        await session.close()
```

## Build and verify

Run the following from this directory. Generating native archives requires Zig **0.17.1**. The packaged Rust crate links a bundled archive for Apple Silicon macOS, Intel macOS or x86-64 Linux with glibc 2.28+. Consumers on those targets need only their normal Rust toolchain. The Python wheel targets CPython 3.10+ through the Stable ABI. Node requires version 22 or newer.

```sh
zig build test
python3 rebuild.py --check
cargo test --locked
```

To rebuild the archives after changing Zig code, run `python3 rebuild.py`. Set `ZIG` to select a specific compiler executable. For a host-only build, add `--target aarch64-apple-darwin`, `--target x86_64-apple-darwin` or `--target x86_64-unknown-linux-gnu`.

With Rust 1.92+, Python 3.10+, Node 22+ and maturin 1.16 installed, run `python3 check_packages.py`. This verifies catalog coverage, runs the Rust fixtures, builds all three packages, installs the wheel and npm tarball into fresh consumer environments, and runs their fixtures. The verified files are copied to `dist/`. The npm tarball and wheel are specific to the packaging host. Example local installs on Apple Silicon follow.

```sh
python3 -m pip install dist/*arm64.whl
npm install ./dist/aux-connectors-0.0.1.tgz
cargo add --path /absolute/path/to/aux/sdk/rust
```


## Package build times

These historical medians cover an earlier revision of the standalone port on an Apple M2 Pro with 16 GB of memory running macOS 26.6.2 on November 24, 2026. Each total runs from Zig 0.17.1 ReleaseSafe compilation through creation of the installable package. Dependencies were prefetched. Each package had three trials with empty Zig and Cargo caches, each followed by a rebuild after changing interval validation in the Derive connector. The core and binding columns report their individual medians.

| Package | Clean total | Clean core | Clean bindings and packaging | Edited total | Edited core | Edited bindings and packaging | Packed size | Unpacked native binary |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Python abi3 wheel | 41.8 s | 32.1 s | 9.7 s | 38.0 s | 32.9 s | 5.1 s | 1.47 MiB | 3.09 MiB |
| Node npm tarball | 47.6 s | 34.1 s | 13.6 s | 40.3 s | 34.3 s | 5.7 s | 1.47 MiB | 3.14 MiB |

An edit to one connector recompiles the single native object that contains all connectors. The edited build reuses compiled binding dependencies. These local results measure absolute package build times. The optional CI benchmark uses the same harness on macOS arm64 and Linux x86-64. [Raw times and package sizes](benchmarks/macos-arm64-2026-11-24.json)

## Adding an operation

Add the native implementation to the vendor folder and expected wire data to `tests/`. Each vendor owns a `routes.zig` table. Register new vendor tables in `src/core/routes.zig`. Shared fixtures run in Rust and against the installed npm and Python packages. Add authenticated dispatch only after the signing and headers have fixture coverage. Run `python3 tools/generate_clients.py` to regenerate named SDK methods, then run the package checks. The native headers in `include/` specify buffer ownership and stream handle lifetimes.

To measure standalone release-package builds on a native runner, run `python3 tools/benchmark_packages.py --output timings.json` with `ZIG` and `MATURIN` set. The harness prefetches dependencies and measures each SDK in a temporary source copy. It records three clean builds and three rebuilds after a connector edit, including time in native compilation and wrapper packaging.
