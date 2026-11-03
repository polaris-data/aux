# Paradex

Enable `features = ["paradex"]` on `aux-rs`. The adapter exposes 142 typed REST operations: 139 in the pinned production/testnet Swagger specifications plus the documented subkey JWT, referral-configuration, and legacy testnet max-slippage routes. It covers every published API family, including spot/perps/options, RFQ and block trades, algos, MMP, keys/tokens, account settings, vaults, transfers/history, staking, rewards, and system data.

[coverage.json](coverage.json) maps the REST operations, 30 documented/reference WebSocket entries, and all 12 SBE templates to their native interfaces and tests. [Coverage tables](COVERAGE.md) provide the browsable inventory. [Source pins](API_SOURCES.json) record schema hashes and upstream commits. Schemas are external generation inputs; they are not bundled in the repository.

```rust
use aux_rs::paradex::{ParadexClient, Subscription};

let client = ParadexClient::testnet()?;
let markets = client.get_markets(Default::default()).await?;
let mut book = client.subscribe(Subscription::OrderBook {
    market: "ETH-USD-PERP".into(), feed_type: "snapshot".into(),
    refresh_rate: "100ms".into(), price_tick: None,
}).await?;
book.close().await?;
client.transport().shutdown().await?;
```

REST methods use explicit parameter structs. Path/query/header parameters are fields of that struct; JSON request content is its `body` field. Prices and sizes remain strings, JSON numeric values remain arbitrary-precision numbers, and additional response fields survive decoding. A successful submission response can still represent a queued or subsequently rejected native order; reconcile through the native order reads/streams.

## Networks and credentials

`mainnet()` selects `https://api.prod.paradex.trade` and the production WebSocket endpoint, with `PRIVATE_SN_PARACLEAR_MAINNET`. `testnet()` selects `https://api.testnet.paradex.trade` and testnet WebSockets, with `PRIVATE_SN_PARACLEAR_TESTNET`. L1 chain IDs are 1 and 11155111. Custom configuration carries the API origin, WebSocket URL, appchain ID, L1 chain ID, SIWE domain and URI explicitly. HTTP configuration omits `/v1`; native v2 auth/onboarding routes use `/v2` at the origin, as the authentication guide specifies, rather than the Swagger UI's duplicated `/v1/v2` prefix.

Attach `ParadexAuth::new(exchange, account, signer, subkey_public_key)` using `with_auth`. `LocalStarkSigner`, `LocalEvmSigner`, and async caller-defined `Signer` implementations share one native interface. Account addresses are explicit, independent of the signer/subkey. `evm_account_address` derives an EVM account address from the class hash returned by system configuration; this does not deploy it.

`client.authenticate().await` explicitly obtains a JWT and updates shared token state. No REST method or WebSocket reconnect silently renews it. `auth.tokens().set(Zeroizing<String>, read_only)` attaches an externally acquired token, and `ParadexAuth::bearer` supports token-only clients. JWT payload decoding discovers expiry; it does not locally verify token authenticity. The exchange validates tokens and scopes. Read-only credentials are rejected locally for writes.

Starknet authentication supports main keys and `/v1/auth/{public_key}` subkeys. EVM authentication uses SIWE/EIP-191. Authenticated REST calls and reconnects require a valid token. An established WebSocket remains authenticated for its native lifetime, including after JWT expiry; refreshing REST credentials does not change that socket's established permissions.

Orders and modifications receive native signatures when their signature field is empty. REST batches sign each item while preserving native item outcomes. Block creation adds this account's signature when absent; offers bind their signature to the parent block ID. `block_signature` prepares signatures for caller-orchestrated block execution workflows. `sign_typed_data`, `onboarding_typed_data`, and `siwe_message` expose explicit native signing for account administration/onboarding. Keep external signatures, account permissions, and on-chain workflows explicit. See [SIGNING.md](SIGNING.md).

## Binary streams and command sessions

Public WebSocket payloads require **SBE**, and this adapter pins **schema 1:3**. Every configured public stream URL negotiates that version. Control messages and JSON channels remain JSON; the decoder also handles private SBE templates where served. Invalid schema/template combinations, malformed frames, invalid UTF-8, and inconsistent group dimensions fail explicitly.

Events preserve the native JSON envelope. Binary events include `sbe` metadata and typed `sbe::SbeMessage` variants accessible through `event.sbe_frame()`. Their `params.data` uses the **SBE schema's field names and units**: prices/quantities are exact decimal strings, timestamps retain microseconds, books carry bids/asks with `pkgType`, and `tradeIdStr` preserves the full trade ID. Do not substitute the deprecated truncated integer `tradeId`. These shapes differ from JSON channels; the application owns reconstruction and reconciliation.

The full channel identifies each pooled connection, preventing ambiguity between book feeds whose binary messages omit depth/refresh parameters. Late book consumers receive a fresh snapshot. Recovery reports gaps before fresh data; protocol faults, overflow, and exhausted recovery are terminal.

Authenticated `open_rpc()` uses a dedicated JSON command connection. It exposes create/modify/cancel and batch commands, cancel-on-disconnect, private subscriptions and subscription modification. Public streams use the separate SBE transport. Command sessions never reconnect or replay writes. Timeouts/cancellation retain `pending()` until `reconcile()` obtains the matching response; buffered notifications are available through `next_event()`. `with_limits` configures buffering and response deadlines. Batch command parameters follow the pinned official SDK's array form.

## Python

```python
from aux_py import ParadexClient, ParadexSigner

async with ParadexClient() as client:
    markets = await client.get_markets()
    book = await client.get_orderbook({"market": "ETH-USD-PERP", "depth": 15})
```

All methods accept native mappings, with JSON content under `body`. Trading clients accept `account=...` and `signer=ParadexSigner(key, kind="starknet")` or `kind="evm"`. Call `await client.authenticate()` explicitly. `set_token(token, read_only=True)` supports external renewal. `ParadexSigner.remote(public_key, async_callback, kind=...)` receives finalized native signing context; callback signatures are verified before submission. Streams and RPC sessions support async context management, single-reader enforcement, cancellation, and cleanup. Python contains no separate protocol or SBE implementation.

For native onboarding, `onboarding_typed_data()` returns the Starknet payload for `sign_typed_data`. `sign_siwe(statement, nonce, issued_at, expiration=None)` returns inspectable SIWE text, its base64 header value, and the verified EVM signature; supply these to the typed onboarding API. `evm_account_address(address, class_hash)` performs deterministic address calculation. Custom EVM endpoints require explicit `chain_id`, `evm_chain_id`, `siwe_domain`, and `siwe_uri` in Python as well.

## Verification

```sh
cargo run --locked -p aux-rs --features paradex --example paradex_public
cargo run --locked -p aux-rs --features paradex --example paradex_testnet_trade -- --execute
```

The opt-in trading example requires `PARADEX_PRIVATE_KEY`, `PARADEX_ACCOUNT`, `PARADEX_PRICE`, and `PARADEX_SIZE`, with optional `PARADEX_SUBKEY`. The Python equivalent is `crates/python/examples/derive_paradex_testnet_trade.py paradex --execute`. [Verification results](VERIFICATION.md) distinguish offline and public checks from pending funded actions; reference vectors establish compatibility, not an audit. Native API coverage does not include chain RPC, transaction submission, bridge orchestration, or gas management.
