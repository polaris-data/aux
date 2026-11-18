# Pyth data connector

Enable `pyth` on `aux-rs` to use `PythProClient` and `PythHermesClient`.
Both clients are also available in `aux_py` and `aux-ts`. The [operation
inventory](operations.json) lists the methods and native request fields.

Pyth Pro exposes feed discovery, historical prices and candles, raw publisher
updates, signed router updates, and live subscriptions. Hermes v2 exposes
feed metadata, signed price updates and publisher stake caps, and SSE. The
legacy TWAP route is callable for compatibility. The current Hermes service
rejects it as deprecated.

Pro uses numeric `priceFeedIds` or supported symbol strings. Hermes uses
32-byte hexadecimal `ids`. Timestamp units follow each endpoint. Pro signed
history takes a microsecond integer, Hermes price updates take Unix seconds,
and Pro candle history takes Unix seconds. Price responses retain the native
mantissa, exponent, confidence, and update timestamps.

```rust,no_run
use aux_rs::pyth::{PythConfig, PythProClient, SymbolsRequest};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = PythProClient::new(PythConfig::default())?;
let symbols = client.symbols(SymbolsRequest { query: Some("BTC".into()), ..Default::default() }).await?;
println!("{}", symbols.data);
client.transport().shutdown().await?;
# Ok(())
# }
```

```python
async with PythProClient({'api_key': key}) as client:
    async with await client.subscribe({
        'priceFeedIds': [1], 'subscriptionId': 1,
        'channel': 'fixed_rate@200ms', 'formats': [],
        'properties': ['price', 'exponent', 'feedUpdateTimestamp'],
    }) as updates:
        async for update in updates:
            print(update)
            break
```

```ts
const client = new PythHermesClient({config: {api_key: key}});
try {
  const result = await client.latestPriceUpdates({request: {ids: [feedId], parsed: true}});
  console.log(result.data);
} finally {
  await client.close();
}
```

Pro public discovery works without credentials. Authenticated routes and
subscriptions use `api_key`. Supply `publisher_token` separately for raw
publisher updates. The official Hermes service requires a bearer API key.
Override `api_url`, `router_url`, `hermes_url`, or `ws_urls` in `PythConfig`
for custom endpoints. Rust supports independent built-in `http` and `ws`
features or caller-supplied transports.

HTTP responses expose `status`, `headers` (including `Pyth-Split-Version`
when present), and `data`. Pagination cursors remain opaque and callers
request each page. Python returns exact `Decimal` values. Node returns exact
fractional strings and uses `bigint` for large JSON integers.

Pro opens the three official WebSockets by default and deduplicates identical
price updates with a bounded cache. A recoverable gap means every source lost
continuity. Continue reading after a gap or reopen explicitly to start a new
subscription. Binary delivery exposes the complete frame in base64, decoded
JSON if present, and recognized binary payloads. Hermes SSE ends with a stream
interruption signal and can be opened again by the caller. Signed payloads
remain available for caller-side verification.

[Pyth Pro REST](https://docs.pyth.network/pro/api/rest), [WebSocket guide](https://docs.pyth.network/pro/subscribe-to-prices), [Hermes compatibility](https://docs.pyth.network/pro/pyth-core-upgrade), [source and fixture pins](API_SOURCES.json).
