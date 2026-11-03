# Derive v3

Enable `features = ["derive"]` on `aux-rs`. This adapter targets the Derive **v3** API, with 128 typed REST methods, the same methods over an isolated WebSocket RPC session, all four WebSocket control methods, and all 13 documented subscription channels. It covers spot, perps, options, RFQ, account/session administration, transfers, withdrawal requests, liquidations, vaults, history, and rewards/referral reads.

[coverage.json](coverage.json) maps every operation/channel to its Rust and Python surface, fixture, and contract test. [Coverage tables](COVERAGE.md) provide the browsable inventory. The [pinned specifications](reference) define the coverage baseline; the generator does not introduce an exchange SDK dependency.

```rust
use aux_rs::derive::{DeriveClient, GetInstrumentRequest, Subscription};

let client = DeriveClient::testnet()?;
let instrument = client.public_get_instrument(GetInstrumentRequest {
    instrument_name: "ETH-PERP".into(),
    ..Default::default()
}).await?;
let mut book = client.subscribe(Subscription::Orderbook {
    instrument_name: instrument.instrument_name, group: 10, depth: 20,
}).await?;
// Events preserve the native envelope. event.payload::<OrderbookPayloadDoc>()
// decodes native channel data; the application owns book reconstruction.
book.close().await?;
client.transport().shutdown().await?;
```

`mainnet()` selects `https://api.derive.xyz/v3` / `wss://api.derive.xyz/v3/ws`, chain 1. `testnet()` selects `https://testnet.api.derive.xyz/v3` / `wss://testnet.api.derive.xyz/v3/ws`, chain 11155111. `Derive::custom(http, ws, chain_id, matching)` makes the signing deployment explicit. Clients also support `new`, `with_config`, `with_transport`, `with_auth`, and shared standard transport shutdown.

## Authentication and signing

Attach `DeriveAuth::new(client.exchange(), owner_wallet, signer)` with `with_auth`. `LocalSigner::from_hex(Zeroizing<String>)` and caller-defined async `Signer` implementations use the same contract. Wallet ownership is distinct from the signing key; registered session keys use the owner wallet's address.

HTTP private calls obtain a fresh EIP-191 login proof. Private subscriptions log in before subscribing and regenerate their proof before reconnecting. Server scopes, key registration/revocation, subaccount access, and native permissions remain authoritative. Private Rust methods require an `Authentication` bound; an unsigned client cannot submit orders.

Order and replace preparation resolves native instrument metadata. Transfers and withdrawals resolve deposit-enabled spot assets, including underlying decimals for withdrawals. Orders, RFQ/position transfers, session-key registration, whitelist updates, vault actions, subaccount deletion, and liquidation bids prepare and sign native EIP-712 actions when their signature field is empty. Supplying a complete signature preserves the caller's signed request. Account-administration endpoints that require only login do not receive an invented action signature.

`ModuleData` exposes typed, offline ABI encoders. `sign_request(exchange, auth, &mut native_json, &module_data)` prepares an explicit native envelope, and `ActionPayload` supports caller-prepared module bytes. Default signature validity is 300 seconds; explicit nonces and expiry remain caller-owned. Automatically allocated nonces are monotonic and shared by signing identity/network within the process. Coordinate independent processes and the submission order of workflows that require increasing nonces in the application.

Prices and amounts remain decimal strings; numeric IDs use arbitrary-precision `serde_json::Number`. Signing rejects excess precision instead of rounding. Most action amounts use e18 with at most 12 decimal places; withdrawal amounts use the underlying token's decimals. Liquidation fractions use `1` for 100%, in increments of `0.01`. [SIGNING.md](SIGNING.md) records references, compatibility vectors, and limits.

## Streams and RPC

`subscribe` accepts public channels. `subscribe_private` uses attached authentication and isolates wallet/signer scopes. Native acknowledgements are delivered; unsuccessful per-channel subscription statuses become native errors. Books contain periodic snapshots, not an internally maintained order book. Recoverable transport gaps precede fresh data after reconnect; overflow, protocol faults, and exhausted recovery terminate the subscription.

`open_public_rpc()` and authenticated `open_rpc()` create dedicated sessions with named native methods. `subscribe_channels`, `unsubscribe_channels`, and `set_cancel_on_disconnect` operate on that same connection. Cancel-on-disconnect follows Derive's account setting and native connection tracking; it covers applicable MMP orders and quotes, not every order type.

RPC sessions never reconnect or replay commands. A timeout or cancellation retains `pending()`; call `reconcile()` to await that request's response. Notifications received while waiting are buffered for `next_event()`. If the socket is lost, reconcile account state through another connection before deciding what to submit. `with_limits(capacity, timeout)` configures native RPC buffering and response deadlines.

## Python

```python
from aux_py import DeriveClient, DeriveSigner

async with DeriveClient() as client:
    instrument = await client.public_get_instrument({"instrument_name": "ETH-PERP"})
    async with await client.subscribe({
        "type": "orderbook", "instrument_name": "ETH-PERP", "group": 10, "depth": 20,
    }) as stream:
        event = await anext(stream)
```

All REST methods use the native names in the coverage manifest and accept a request mapping. Trading clients accept `signer=DeriveSigner(key), wallet=owner`. `DeriveSigner.remote(address, async_callback)` receives the finalized digest, chain ID, and signing purpose and returns a 65-byte hex signature. Callbacks retain the initiating asyncio context and are cancelled with their native operation. `prepare_signed` exposes `ModuleData`; `sign_action` exposes explicit action signing. Python RPC sessions expose the same named methods plus pending/reconciliation and async context management.

## Verification

Normal checks use synthetic contract fixtures, local peers, and pinned reference signing vectors. Run public testnet checks separately:

```sh
cargo run --locked -p aux-rs --features derive --example derive_public
cargo run --locked -p aux-rs --features derive --example derive_testnet_trade -- --execute
```

The trading example requires `DERIVE_PRIVATE_KEY`, `DERIVE_WALLET`, `DERIVE_SUBACCOUNT_ID`, `DERIVE_PRICE`, and `DERIVE_SIZE`. The Python equivalent is `crates/python/examples/derive_paradex_testnet_trade.py derive --execute`. [Verification results](VERIFICATION.md) separate offline checks, public network checks, and pending funded submit/cancel checks. Exchange API calls do not orchestrate chain RPC, bridging, approvals, gas, deposits, or settlement transactions.
