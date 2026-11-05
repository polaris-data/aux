# Tardis data connector

Native hosted Tardis APIs and caller-managed Tardis Machine, in Rust and Python.
The connector owns its protocol implementation. It does not launch Machine or
implement exchange collection, normalization, book reconstruction or caching.

## Rust

Select `aux-rs` features `tardis,http` for hosted data and HTTP replay, and
`tardis,ws` for Machine WebSockets. Default facade features already include HTTP
and WebSockets. The adapter crate itself has no default networking features.

```rust,no_run
use aux_rs::tardis::{TardisClient, FeedRequest, ReplayRequest};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = TardisClient::new()?; // optional .with_api_key(key)
let exchanges = client.exchanges().await?;
let mut minute = client.data_feeds(FeedRequest::new("bitmex", "2024-01-01")).await?;
while let Some(event) = minute.next().await {
    println!("{:?}", event?);
}
minute.close().await?;
let mut replay = client.replay(ReplayRequest {
    exchange: "bitmex".into(),
    from: "2024-01-01".into(),
    to: "2024-01-01T00:02:00Z".into(),
    filters: vec![],
})?;
let batch = replay.read_batch(256).await?;
replay.close().await?;
client.transport().shutdown().await?;
# Ok(()) }
```

`exchanges`, `exchange_details`, `exchange_summary`, `symbols`, `instruments`,
`instrument` and `api_key_info` expose the native discovery and entitlement APIs.
JSON responses are `serde_json::Value` with arbitrary-precision numbers. Use
`params(json!({...}))` to construct native parameter objects; instrument filters
remain JSON, while symbol-search arrays use native comma-separated encoding.
Pagination cursors and availability fields remain in their native responses.

`data_feeds` accepts native `from`, minute `offset`, filters and `sliceSize`.
`replay` traverses a UTC minute-aligned, inclusive-start/exclusive-end range in
capture order, including empty minutes. Its raw messages use
`{"localTimestamp": ..., "message": ...}` envelopes, retaining native payloads
and timestamp precision. Recorded blank lines become `{"type":"disconnect"}`.

`dataset(DatasetRequest)` returns original `.csv.gz` bytes, including optional
single native HTTP `range`, rather than decoding or rounding CSV fields.
`download_dataset(request, path)` writes a sibling temporary file and publishes
the complete result without overwriting an existing destination. Interrupted or truncated transfers never publish a partial destination file.

## Tardis Machine

Construct `TardisMachineClient::new("http://localhost:8000")` with the address of
a running service. The HTTP origin is converted to `ws`/`wss` for socket endpoints by default. Use
`.with_ws_url("ws://localhost:8001")` in Rust, or `ws_url="ws://localhost:8001"`
in Python, when Machine serves WebSockets on a different port.

- `replay(options)` and `replay_normalized(options)` expose streaming HTTP.
- `ws_replay(exchange, from, to, subscribe_messages, session)` sends caller-supplied native
  exchange subscription messages to Machine's raw replay endpoint. An optional
  `session` key synchronizes multiple raw replay connections.
- `ws_replay_normalized(options)` and `stream_normalized(options)` expose native
  normalized replay and live streams. Options accept objects or arrays, including
  multi-exchange replay and the vendor's computed snapshot/bar parameters.

Machine JSON events remain unchanged, including disconnect/error records and
vendor-defined normalized types. Clean WebSocket closure drains received records
and ends iteration; abnormal closure fails explicitly. No reconnect or replay
occurs automatically.

## Python

```python
import aux_py

async def example():
    async with aux_py.TardisClient(api_key="YOUR_DATA_API_KEY") as client:
        symbols = await client.symbols(exchanges=["bitmex"], baseCurrency="BTC")
        async with await client.replay({
            "exchange": "bitmex", "from": "2024-01-01", "to": "2024-01-02",
        }) as records:
            batch = await records.read_batch(256)
```

The public classes are `TardisClient`, `TardisMachineClient`, `DataStream` and
`ByteStream`. All network work and codecs run in Rust. Native JSON numbers follow
the existing Python integer/`Decimal` conversion rules. Streams allow one pending
read, support async iteration and batches, and close on asyncio cancellation.
Closing a client closes its child streams.

## I/O, errors and verification

`with_transport(api_url, datasets_url, transport)` and Machine's
`with_transport(url, transport)` accept caller-supplied I/O. Hosted methods require
`StreamingHttpTransport`; sockets require `WebSocketConnector`. The core `Exchange`
trait is not involved. Custom bodies must release their resources on drop.
File helpers and their Tokio dependency are always included, even with default
features disabled. Built-in HTTP and WebSocket transports remain optional.

Lines and buffered metadata responses are limited to 16 MiB; decoded byte chunks
are at most 64 KiB and batches contain 1–4096 records. HTTP timeout configuration
bounds header and idle-read waits, not the entire transfer. Rust cancelled reads
retain partial parsing state; Python cancellation closes the stream. No SDK read
or stream retry is implicit, and persistent caches are application-owned.

Errors distinguish transport, native HTTP rejection, validation, decoding and
file I/O. Rejections retain status and sanitized native bodies; authentication,
entitlement and rate limiting have distinct `kind()` values. Credentials are
excluded from diagnostics and removed from echoed rejection details. Redirects
never forward authorization across origins or downgrade HTTPS.

[Coverage](coverage.json) records all 16 operations, 11 normalized event
families, nine opaque CSV dataset families and exchange-native feed records at the [pinned references](API_SOURCES.json). `python3
scripts/check-data-coverage.py` verifies the inventory and fixture hashes offline.
Rust fixtures and Python local HTTP/WebSocket servers cover requests, exact
values, compression, range replay, cancellation, downloads and shutdown. No live
vendor request was used to validate this connector; paid-plan entitlements and
compatibility with a deployed Machine version require separate verification.
