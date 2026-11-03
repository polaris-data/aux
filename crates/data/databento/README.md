# Databento data connector

Owned Databento historical HTTP, reference-data HTTP and live TCP clients.
The official `databento` SDK is a pinned compatibility reference, not a runtime
dependency. The focused `dbn` codec is pinned to 0.73.0 with its networking, Python
and default compression features disabled.

## Rust

Select facade feature `databento` and `http` for historical/reference clients, and
`databento,tcp` for the built-in live connector. HTTP and WebSockets remain the
facade defaults; TCP is explicit. This adapter does not use WebSockets.

```rust,no_run
use aux_rs::databento::{HistoricalClient, params};
use serde_json::json;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = HistoricalClient::new(std::env::var("DATABENTO_API_KEY")?)?;
let datasets = client.list_datasets(params(json!({}))?).await?;
let mut records = client.get_range(params(json!({
    "dataset": "GLBX.MDP3", "schema": "trades", "symbols": ["ES.c.0"],
    "stype_in": "continuous", "start": "2024-01-01", "end": "2024-01-02"
}))?).await?;
while let Some(event) = records.next().await {
    println!("{:?}", event?);
}
records.close().await?;
client.transport().shutdown().await?;
# Ok(()) }
```

Native HTTP operations use `Params`, a JSON parameter object. Symbols and other
array form/query fields use the vendor's comma-separated representation; exact
native strings and integers are retained. Responses are native JSON values,
rather than a shared cross-vendor model.

`HistoricalClient` exposes all pinned metadata operations, including prices,
record counts, billable sizes and cost estimates; `resolve`; and `submit_job`,
`list_jobs`, `get_job_details` and `list_files`. Batch submission executes once and
never retries an ambiguous outcome. Calling data or job APIs retains the vendor's
normal billing semantics; the SDK does not submit a job when estimating cost.

`get_range` requests uncompressed DBN and incrementally yields `DbnEvent::Metadata`
followed by owned `DbnEvent::Record` values. Rust records expose an official
`dbn::RecordBuf` for typed access and retain optional `ts_out`. All 20 published
market-data schemas and error/system/symbol-mapping records are covered. DBN v1–v3
are decoded using the official upgrade policy; unsupported versions fail explicitly.

`timeseries(parameters)` exposes original encoded bytes for native DBN, CSV or
JSON requests and compression choices. `download(url)` consumes a URL returned by
`list_files` as byte chunks. `download_to(url, path)` atomically publishes a complete
file without overwriting an existing path. Authentication is restricted to the
configured API origin and is stripped on cross-origin storage redirects.

`ReferenceClient` exposes `security_master_get_range`, `security_master_get_last`,
`corporate_actions_get_range`, `adjustment_factors_get_range`, `list_events` and
`list_enums`. Range/last responses are native JSON row streams, with optional
Zstandard decoding. Native `index`, allocation, country, security-type, event and
symbol filters remain caller-controlled.

## Live sessions

```rust,no_run
use aux_rs::databento::{LiveClient, SessionOptions, Subscription};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = LiveClient::new(std::env::var("DATABENTO_API_KEY")?)?;
let mut options = SessionOptions::new("GLBX.MDP3");
options.ts_out = true;
let mut session = client.connect(options).await?;
session.subscribe(Subscription {
    schema: "trades".into(), stype_in: "raw_symbol".into(),
    symbols: vec!["ESZ6".into()], start: None, snapshot: false, id: None,
}).await?;
session.start_session().await?;
if let Some(event) = session.next().await { println!("{:?}", event?); }
session.close().await?;
# Ok(()) }
```

Connections authenticate using the native CRAM challenge response and API-key
bucket suffix. Session options include gateway override, compression, output
timestamps, heartbeat interval and slow-reader behavior. Subscriptions support
exact Unix-nanosecond replay starts, snapshots, identifiers and symbol chunking;
`unsubscribe` supports removal while the session is running.

A cancelled/failed command quarantines the session against further writes. Live
connections never reconnect or silently resume; disconnects and idle timeouts
terminate delivery. Consumers receive native DBN control/error records and own
any application reconciliation or book reconstruction.

## Python

```python
import aux_py

async def example():
    async with aux_py.DatabentoHistoricalClient("YOUR_API_KEY") as client:
        datasets = await client.list_datasets()
        async with await client.get_range(
            dataset="GLBX.MDP3", schema="trades", symbols=["ES.c.0"],
            stype_in="continuous", start="2024-01-01", end="2024-01-02",
        ) as records:
            batch = await records.read_batch(256)
```

Python exposes `DatabentoHistoricalClient`, `DatabentoReferenceClient`,
`DatabentoLiveClient` and `DatabentoLiveSession`. Use `await client.connect(...)`,
then `await session.subscribe(...)` and `await session.start_session()` before
iterating a live session. Streams support async contexts and batch reads; closing
a parent client or cancelling a read releases its child resources.

DBN records use the official Databento JSON representation without pretty-price
or pretty-time conversion: 64-bit prices, IDs and timestamps are exact decimal
strings; smaller integers stay integers. Metadata is delivered as
`{"type":"metadata","metadata":...}` before records. Reference JSON follows the
existing integer/`Decimal` conversion rules. No protocol implementation or vendor
Python SDK runs in the binding layer.

## I/O and verification

HTTP clients accept `with_transport(api_url, api_key, transport)` using the core
`StreamingHttpTransport` contract. `LiveClient::with_connector` accepts independent
`ByteStream`/`ByteWriter` halves through the vendor's `ByteConnector` contract.
No built-in network stack is required for custom I/O. The optional `files` feature
adds download helpers independently; `http` enables it automatically.

Buffered metadata and individual JSON rows are limited to 16 MiB, byte chunks to
64 KiB, and batches to 1–4096 records. DBN metadata allocation is checked before
codec allocation. HTTP header/idle timeouts and TCP connect/idle timeouts are
bounded without imposing a whole-transfer deadline. No automatic request or
stream retries occur. Disk caching and ingestion/storage pipelines are outside
the connector.

Structured errors retain sanitized native details and distinguish transport,
decoding, authentication, entitlement, rate limiting, vendor rejection and I/O.
Python HTTP errors expose `DataVendorError.http_status`, `.kind` and `.native`.

[Coverage](coverage.json) records 29 operations and 24 schema/control entries at
[pinned official references](API_SOURCES.json). Offline tests cover HTTP forms,
all DBN schema families, compressed/fragmented input, `ts_out`, CRAM/control-message
vectors, real local TCP sockets, Python parity, cancellation and shutdown.
Live Databento services, paid data and account entitlements have not been verified;
credentialed smoke tests must be run separately with deliberately selected queries.
