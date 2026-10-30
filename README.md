# aux

`aux` is an SDK for connecting to DeFi-native exchange and data APIs, with
composable clients and opt-in exchange adapters. Written in Rust, it provides
market data, account reads, streams, and trading, with
[Python bindings](crates/python/README.md) available as `aux-py` (`aux_py`).

The packages are unpublished, and funded authenticated testnet verification is
pending for all exchanges. Lighter's local signing primitives are experimental
and unaudited; see the [signing notes](crates/exchanges/lighter/SIGNING.md).

## Installation

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

Each client exposes its exchange's native operations and response types.
Prices and sizes preserve exact native values, and exchange-specific errors
remain distinguishable from transport failures.

| Exchange | Feature | Guide |
| --- | --- | --- |
| Hyperliquid | `hyperliquid` | [Reads, streams, and signing](crates/exchanges/hyperliquid/README.md) |
| Lighter | `lighter` | [Reads, streams, and signing](crates/exchanges/lighter/README.md) |
| QFEX | `qfex` | [Market data and WebSocket trading](crates/exchanges/qfex/README.md) |
| Arcus | `arcus` | [Perpetuals data and REST trading](crates/exchanges/arcus/README.md) |

The workspace contains:

- [`aux-rs`](crates/aux-rs): public Rust SDK facade, examples, and integration tests.
- [`aux-rs-core`](crates/core): shared client and transport contracts.
- [Exchange adapters](crates/exchanges): native clients, types, and signing.
- [Python bindings](crates/python/README.md): asyncio clients backed by the Rust SDK.

See the [architecture and roadmap](PLANNING.md) for design and remaining work.

## Features

| Feature | Default | Provides |
| --- | --- | --- |
| `http` | Yes | Pooled Reqwest HTTP transport |
| `ws` | Yes | Tokio Tungstenite WebSocket sessions |
| `hyperliquid` | No | Native Hyperliquid client and signing |
| `lighter` | No | Native Lighter client and signing |
| `qfex` | No | Native QFEX client and HMAC authentication |
| `arcus` | No | Native Arcus client and Ed25519 signing |

Exchange features do not activate networking. Disable default features to use
custom I/O; see the [custom transport guide](HARDENING.md#custom-transports).
The [transport guide](HARDENING.md) covers configuration, recovery gaps,
subscription lifecycle, and shutdown. Production dependencies are recorded in
the [dependency inventory](DEPENDENCIES.md).

## Contributing

See the [contributor guide](CONTRIBUTING.md) for workspace setup, Python checks,
and dependency maintenance. Run the Rust checks from the repository root:

```sh
bash scripts/check.sh
```

Normal checks use committed fixtures and local servers without exchange requests.

## License

Licensed under the [MIT license](LICENSE).
