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

Each client exposes its exchange's native operations and response types.
Prices and sizes preserve exact native values, and exchange-specific errors
remain distinguishable from transport failures.

| Feature | Default | Provides |
| --- | --- | --- |
| `http` | Yes | Built-in HTTP transport using Reqwest |
| `ws` | Yes | Built-in WebSocket transport using Tokio Tungstenite |
| `hyperliquid` | No | [Native Hyperliquid client and signing](crates/exchanges/hyperliquid/README.md) |
| `lighter` | No | [Native Lighter client and signing](crates/exchanges/lighter/README.md) |
| `qfex` | No | [Native QFEX client and HMAC authentication](crates/exchanges/qfex/README.md) |
| `arcus` | No | [Native Arcus client and Ed25519 signing](crates/exchanges/arcus/README.md) |

Exchange features do not activate networking. Disable default features to use
custom I/O; see the [custom transport guide](HARDENING.md#custom-transports).
The [transport guide](HARDENING.md) covers configuration, recovery gaps,
subscription lifecycle, and shutdown. Production dependencies are recorded in
the [dependency inventory](DEPENDENCIES.md).

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
- [Python bindings](crates/python/README.md): asyncio clients backed by the Rust SDK.

See the [architecture and roadmap](PLANNING.md) for design and remaining work.

## Contributing

See the [contributor guide](CONTRIBUTING.md) for workspace setup, Python checks,
and dependency maintenance. Run the Rust checks from the repository root:

```sh
bash scripts/check.sh
```

Normal checks use committed fixtures and local servers without exchange requests.

## License

Licensed under the [MIT license](LICENSE).
