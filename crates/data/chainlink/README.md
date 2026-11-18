# Chainlink data connector

Enable `chainlink` on `aux-rs` to use `ChainlinkDataStreamsClient` and
`ChainlinkCandlestickClient`. Both clients are also available in `aux_py`
and `aux-ts`. The [operation inventory](operations.json) lists the methods
and native request fields. Choose `mainnet` or `testnet` when constructing
either client.

Data Streams supports public and authenticated discovery, latest and historical
signed reports, bulk and paged reports, and live WebSockets. Pass `api_key` and
`api_secret` together for reports or hidden feeds. The connector signs the
encoded request target and uses a fresh timestamp on WebSocket reconnect.
For bulk queries, `status: 206` identifies partial results.

```rust,no_run
use aux_rs::chainlink::{ChainlinkConfig, ChainlinkDataStreamsClient, DiscoveryRequest, Network};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = ChainlinkDataStreamsClient::new(ChainlinkConfig::new(Network::Mainnet))?;
let response = client.discovery(DiscoveryRequest::default()).await?;
println!("{}", response.data);
client.transport().shutdown().await?;
# Ok(())
# }
```

Call `decode_report` (Node `decodeReport({fullReport})`) on a report's
`fullReport` hex. It returns the unchanged envelope and the extracted report
body. Versions 2, 3, 7, 8, 9, 10, 11 and 14 expose their native decoded fields.
Wide signed integers and timestamps within reports are decimal strings.
Unknown versions retain the envelope and body for inspection. Decoding
performs structural validation. Signature verification is caller-owned.

Use `authorize` with the Candlestick login and password to obtain
`data.d.access_token` and `data.d.expiration`. Supply that token to a client
for symbol/group discovery, historical candles, and live price updates. In
Rust, `with_token` returns a client with the replacement token. Python and
Node callers can instantiate a new client with the replacement. For example:

```ts
const auth = new ChainlinkCandlestickClient({config: {network: "testnet"}});
let token;
try {
  const response = await auth.authorize({request: {login, password}});
  token = response.data.d.access_token;
} finally {
  await auth.close();
}
const candles = new ChainlinkCandlestickClient({config: {network: "testnet", token}});
try {
  const result = await candles.candleHistoryRows({request: {symbol: "ETHUSD", resolution: "1m", from, to}});
  console.log(result.data);
} finally {
  await candles.close();
}
```

Candlestick values and heartbeat objects retain their native shape. Candles
preserve the vendor's scale and exact JSON numbers. Python returns fractions
as `Decimal` and Node returns exact strings. HTTP streams report interruptions
and can be opened again by the caller. Data Streams WebSockets expose a gap
before resumed reports. Clients and subscriptions support explicit close.

Use `api_url`, `ws_url`, and `candles_url` overrides for custom endpoints.
Rust supports independent built-in `http` and `ws` features or caller-supplied
transports. All test fixtures run offline.

[Data Streams](https://docs.chain.link/data-streams/reference/data-streams-api/interface-api), [Candlestick API](https://docs.chain.link/data-streams/reference/candlestick-api), [source and fixture pins](API_SOURCES.json).
