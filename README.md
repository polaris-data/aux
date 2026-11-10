# aux

`aux` connects you to DeFi-native exchange APIs, external trading infrastructure and market data vendors, through SDKs in Rust and Python.

## Installation

### Rust

Requires **Rust 1.92 or later**. The crates are local workspace packages; add a
path dependency and adjust the path to your checkout:

```toml
[dependencies]
aux-rs = { path = "../connectors/crates/aux-rs", features = ["hyperliquid"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Import the Rust package as `aux_rs`. HTTP and WebSocket transports are enabled
by default; select each exchange explicitly with its feature flag. Unselected
adapters and Python bindings are excluded from Rust consumers.

#### Features

Each client exposes its exchange or data vendor’s native operations and response types.
Prices and sizes preserve exact native values, and exchange-specific errors
remain distinguishable from transport failures.

| Feature | Default | Provides |
| --- | --- | --- |
| `http` | Yes | Built-in HTTP transport using Reqwest |
| `ws` | Yes | Built-in WebSocket transport using Tokio Tungstenite |
| `phoenix` | No | [Native Phoenix perpetuals APIs, Solana instructions and signing](crates/exchanges/phoenix/README.md) |
| `extended` | No | [Native Extended APIs, streams and Stark signing](crates/exchanges/extended/README.md) |
| `hyperliquid` | No | [Native Hyperliquid client and signing](crates/exchanges/hyperliquid/README.md) |
| `lighter` | No | [Native Lighter client and signing](crates/exchanges/lighter/README.md) |
| `qfex` | No | [Native QFEX client and HMAC authentication](crates/exchanges/qfex/README.md) |
| `arcus` | No | [Native Arcus client and Ed25519 signing](crates/exchanges/arcus/README.md) |
| `monaco` | No | [Native Monaco spot/perps client and wallet/session signing](crates/exchanges/monaco/README.md) |
| `derive` | No | [Native Derive v3 client, wallet/session signing, and RPC sessions](crates/exchanges/derive/README.md) |
| `tardis` | No | [Tardis hosted data and external Machine replay/live APIs, including file downloads](crates/data/tardis/README.md) |
| `birdeye` | No | [Birdeye prices/OHLCV, token/pair stats, discovery, trades and security](crates/data/birdeye/README.md) |
| `dune` | No | [Dune SQL/saved-query execution, status, cancellation, and JSON/CSV results](crates/data/dune/README.md) |
| `databento` | No | [Databento historical, reference and live clients, including TCP connections and file downloads](crates/data/databento/README.md) |
| `paradex` | No | [Native Paradex v1/v2 client, Starknet/EVM signing, and SBE streams](crates/exchanges/paradex/README.md) |
| `metric` | No | [Metric liquidity/routing HTTP APIs, native EVM contracts and transaction signing](crates/exchanges/metric/README.md) |
| `polymarket` | No | [International predictions, perps, streams, signing and EVM calls](crates/exchanges/polymarket/README.md) |
| `kalshi` | No | [Predictions, perps and clearing APIs with RSA/Ed25519 authentication](crates/exchanges/kalshi/README.md) |
| `pascal` | No | [Prediction APIs, binary permits and Solana collateral](crates/exchanges/pascal/README.md) |

Disable default features to omit built-in HTTP and WebSocket transports and use
custom I/O. Data-vendor features include Tokio for file downloads, and Databento
also includes its built-in TCP connector. Selecting an adapter does not initiate
connections. Built-in transports support configurable timeouts, safe-read retries,
explicit recovery gaps, and shutdown.

### Python

Requires **Python 3.10 or later** and **Rust 1.92 or later**. From the repository
root, create a virtual environment and build the SDK locally:

```sh
python3 -m venv .venv-python
source .venv-python/bin/activate
python -m pip install "maturin==1.16.0"
python -m maturin develop --locked --manifest-path crates/python/Cargo.toml
```

Import the Python package as `aux_py`. See the [Python guide](crates/python/README.md)
for usage examples and verification instructions.

## Examples

### Market data and streams

Connect to Hyperliquid testnet, read market metadata and an ETH book snapshot,
then receive trades and close the subscription. No credentials are required.

```rust
use aux_rs::hyperliquid::{
    Event, HyperliquidClient, L2BookRequest, Subscription,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HyperliquidClient::testnet()?;
    let metadata = client.meta().await?;
    let book = client.l2_book(L2BookRequest::new("ETH")).await?;
    println!(
        "{} markets; best bid: {:?}",
        metadata.universe.len(),
        book.levels[0].first()
    );

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

Run the public testnet example from the repository root:

```sh
cargo run --locked -p aux-rs --features hyperliquid --example hyperliquid_public
```

See the [executable examples](crates/aux-rs/examples) for public data, account
reads, and opt-in testnet trading. The exchange guides cover authentication,
native units, signing, and supported operations.

## Overview

The workspace contains:

- [`aux-rs`](crates/aux-rs): public Rust SDK facade, examples, and integration tests.
- [`aux-rs-core`](crates/core): shared client and transport contracts.
- [Exchange adapters](crates/exchanges): native clients, types, and signing.
- [Data adapters](crates/data): native Tardis, Databento, Dune, and Birdeye clients, streaming data and downloads.
- [Python bindings](crates/python/README.md): asyncio clients backed by the Rust SDK.

See the [architecture](CONTRIBUTING.md#architecture) and
[roadmap](CONTRIBUTING.md#roadmap-and-release-gates) for design and remaining work.

## Contributing

See the [contributor guide](CONTRIBUTING.md) for workspace setup, Python checks,
and dependency maintenance. Run the Rust checks from the repository root:

```sh
bash scripts/check.sh
```

Normal checks use committed fixtures and local servers without exchange requests.

## License

Licensed under the [MIT license](LICENSE).
