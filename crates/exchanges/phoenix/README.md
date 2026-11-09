# Phoenix perpetuals

Enable `phoenix` on `aux-rs` for native Phoenix REST methods, WebSocket subscriptions, Ed25519 signing, Solana transaction support, and local instruction builders. This adapter targets the current perpetuals protocol.

```rust,no_run
use aux_rs::phoenix::{PhoenixClient, Subscription};
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = PhoenixClient::mainnet()?;
let markets = client.list_markets(Default::default()).await?;
let mut stream = client.subscribe(Subscription(serde_json::json!({
    "channel": "orderbook", "symbol": "SOL"
}))).await?;
let update = stream.next().await;
stream.close().await?;
client.transport().shutdown().await?;
# Ok(())
# }
```

Methods use native request types ending in `Params`; path/query fields are at the top level and JSON request bodies are under `body`. [Coverage](COVERAGE.md) lists every REST method. Native numbers retain arbitrary precision, and unknown response fields survive decoding.

`Phoenix::mainnet()` selects the documented HTTP and WebSocket deployment. `Phoenix::custom(http, ws, chain_id)` configures alternate endpoints explicitly. No public testnet endpoint is assumed. `with_transport` works without `http` or `ws`; the defaults on the facade enable the standard transports.

Most reads are public. Use `Credentials::new` and `with_auth` for authenticated endpoints. Wallet/service/Privy login, refresh and logout are explicit named operations; attach or replace the resulting access token explicitly. Login JWTs and Solana transaction signing are separate. Supply `expires_at` when attaching expiring credentials; the SDK does not persist tokens or renew sessions in the background.

`instructions` exposes local native builders and PDA helpers adapted from the pinned Rise reference. They accept explicit account addresses and native integer lots/ticks. Their default protocol addresses are the pinned mainnet addresses, independent of process environment variables. `build_instruction(name, request)` exposes the same builders to Python and Node; exact integer inputs and native enum names are required. No market metadata, slippage, funding, rent-paying account creation, or builder fee is chosen implicitly. See `NOTICE` and `RISE_LICENSE` for attribution.

`PhoenixChainClient` uses an explicit Solana RPC endpoint. Prepare a transaction from reviewed instructions and an explicit fee payer, collect every required signature, assemble, simulate, submit once, and query confirmation separately. `PreparedTransaction::validate_server_transaction` checks a server-built transaction against the reviewed message. Local compilation supports legacy and v0 messages with explicit, caller-reviewed lookup table contents and enforces the packet size limit. Submission retains a pending signature until explicit reconciliation; `clear_reconciled_submission` is caller attestation after checking the chain. RPC acceptance and exchange execution are distinct.

Streams preserve snapshots, deltas and native sequence metadata, including bounded decoding of `base64+zstd` exchange snapshots. Recovery reports an explicit gap and requires fresh snapshots for exchange/trader delta feeds. Applications own book and account reconstruction.

Python exports `PhoenixClient`, `PhoenixSigner`, and `PhoenixChainClient` from `aux_py`. Node exports the matching classes from `aux-ts`, with camelCase method names. Both use the Rust codecs, signers and transports. Python uses exact `Decimal`; Node uses the existing lossless values and `bigint` rules. Close clients and subscriptions explicitly.

See [source pins](API_SOURCES.json) and [verification](VERIFICATION.md). Public smoke examples do not trade. Funded checks are opt-in.
