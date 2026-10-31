# Hyperliquid

The `hyperliquid` feature exposes native market/account reads, subscriptions,
signing, and order/cancel submission through `aux_rs::hyperliquid`.
See the [SDK README](../../../README.md#installation) for installation and a
complete read-only example.

## Public reads and streams

From the repository root, run the public testnet example without credentials:

```sh
cargo run --locked -p aux-rs --features hyperliquid --example hyperliquid_public
```

The example reads metadata and an ETH book snapshot, receives ETH book and trade
events over a shared WebSocket, and explicitly closes both subscriptions.
See [the source](../../aux-rs/examples/hyperliquid_public.rs) for the complete flow.

## Endpoints and native values

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

This example checks state/open-order reads, state/order/fill snapshots, and the order-update acknowledgement.

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

This is an API snippet with caller-supplied key, account, and native asset ID. Use [the opt-in testnet example](../../aux-rs/examples/hyperliquid_testnet_trade.rs) for a complete executable flow. Asset IDs are native exchange IDs, not symbols; default perpetual IDs come from metadata universe indices. Account registration, funding, and agent approval are not performed automatically.

The account passed to `agent` is query context. Hyperliquid derives the master account for trading from the key's existing authorization; changing that context does not retarget a trade. Use `.with_vault(vault_or_subaccount)` to set the actual native `vaultAddress`; this also changes the handler's query account while keeping signer identity and nonce state intact.

- `order`, `cancel`, and `cancel_by_cloid` accept native batches. Limit and trigger orders, grouping, builder/sub-builder fees, reduce-only, and fixed-width client order IDs are represented explicitly.
- Read responses preserve exact numeric strings. Trading inputs require positive canonical decimals: no float conversion, exponent notation, leading zeros, or trailing fractional zeros. For metadata-aware precision checks, call `Order::validate_precision(sz_decimals, is_spot)`; it validates without rounding or fetching markets.
- `ExchangeResponse::Err` preserves a native whole-request rejection. `ExchangeResponse::Ok` still requires inspecting every `ItemStatus`: a batch may mix resting, filled, rejected, and native text/unknown outcomes. HTTP and decoding failures remain separate errors.
- `_with_options` variants accept `ExecutionOptions { nonce, expires_after }`. A supplied nonce bypasses automatic allocation and makes the caller responsible for collisions. Defaults use Unix milliseconds and one monotonically increasing coordinator per signer for the process lifetime, including across independent handlers and vault clones. Counters survive client reconstruction and are never rolled back after an error.
- Cross-process coordination is the caller's responsibility. A different process or a caller-managed nonce can collide with automatic allocation. Native nonce windows and in-flight ordering limits still apply.
- Signed envelopes are immutable, and signer recovery is checked before submission. Custom endpoints require `.with_network(Network::Testnet)` or `Mainnet`; signing context is never inferred from a URL.
- Implement `Signer` to use a remote signing service, or `AuthHandler` to replace request preparation/nonce management. `SignedRequest::from_signature` validates a custom handler's envelope. These contracts use `Send` futures without a runtime dependency or `async-trait`.

Native L1 action signing and offline user-signed primitives are covered by 44 vectors from the [pinned official SDK reference](https://github.com/hyperliquid-dex/hyperliquid-python-sdk/blob/2fdb18f9517675ea03695a0962bd19eece9c83f0/hyperliquid/utils/signing.py). The latter cover `UsdSend` and `ApproveAgent` hashing/signing only; there are no transfer or provisioning submission workflows. See [fixture provenance](tests/fixtures/README.md).

### Opt-in authenticated testnet smoke check

Set `HL_AGENT_PRIVATE_KEY`, `HL_ACCOUNT_ADDRESS`, `HL_TESTNET_ORDER_PRICE`, and `HL_TESTNET_ORDER_SIZE` in your environment, using an already-authorized/funded testnet account. Optional settings are `HL_TESTNET_COIN` (default `ETH`) and `HL_VAULT_ADDRESS`. Then run:

```sh
cargo run --locked -p aux-rs --features hyperliquid --example hyperliquid_testnet_trade -- --execute
```

The example validates market precision, places one post-only buy with a fresh client ID, attempts cancellation by that ID even when submission fails, and exposes both native results. It rejects missing `--execute`, never selects mainnet, and is not run by normal checks. Ambiguous submit/cancel outcomes require reconciliation; `expiresAfter` limits submission time and is not the lifetime of a resting order.

Funded authenticated testnet submit/cancel verification remains pending.

## Transports and recovery

Built-in transports support pooling, subscription acknowledgements, explicit
recovery gaps, and shutdown. Hyperliquid book events are full native snapshots;
the SDK does not build an application order book or replay cached snapshots to
late subscribers.

With built-in transports disabled, the adapter has no runtime or networking
dependencies. It uses K256 for secp256k1, SHA3 for Keccak, and rmp-serde for
MessagePack. K256 clears private-key storage on drop; temporary secret buffers
use zeroize, and private keys are excluded from debug/error output.
