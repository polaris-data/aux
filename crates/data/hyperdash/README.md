# Hyperdash Data API

`aux-rs-hyperdash` connects to Hyperdash's read-only Hyperliquid data service.
It covers all 254 operations in the pinned v1 OpenAPI spec, the documented
`me`, `quote`, `datasets`, `status`, and `changelog` routes, and 16 live stream
channels. The same Rust client backs `aux_py.HyperdashClient` and
`aux-ts.HyperdashClient`. [Hyperdash Data API](https://docs.hyperdash.com/data-api/overview)

## Connect

Enable the Rust facade's `hyperdash` feature and select `http` and `ws` for the
built-in transports. Use `with_config` to override the HTTP or WebSocket URL
and `with_transport` to supply a custom transport. The three public catalog
routes work without a key. All other operations use a bearer key from the
Hyperdash console. [API keys](https://docs.hyperdash.com/data-api/authentication)

```rust,no_run
use aux_rs::hyperdash::{GetCandlesRequest, HyperdashClient};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = HyperdashClient::new(Some(std::env::var("HYPERDASH_API_KEY")?))?;
let page = client.get_candles(GetCandlesRequest {
    coin: "BTC".into(),
    interval: None,
    start_time: None,
    end_time: None,
    limit: Some(100),
    cursor: None,
    max_credits: None,
    credit_price_version: None,
}).await?;
println!("{:?}", page.metadata.credits_charged);
client.transport().shutdown().await?;
# Ok(())
# }
```

```python
import os
from aux_py import HyperdashClient

async with HyperdashClient(os.environ["HYPERDASH_API_KEY"]) as client:
    page = await client.get_candles({"coin": "BTC", "limit": 100})
    print(page["metadata"].get("credits_charged"))
```

```ts
import { HyperdashClient } from "aux-ts";

const client = new HyperdashClient({ apiKey: process.env.HYPERDASH_API_KEY });
try {
  const page = await client.getCandles({ coin: "BTC", limit: 100n });
  console.log(page.metadata.credits_charged);
} finally {
  await client.close();
}
```

## REST responses and credits

Each request returns the HTTP status, response metadata, and native response
body. Models preserve decimal strings, Unix millisecond values, optional
fields, and vendor additions. The Python models are typed dictionary
declarations in `aux_py.hyperdash`, and Node exports TypeScript declarations
from `aux-ts`. Node uses `bigint` for typed integer fields and Python uses
`int`. Rust response models expose unknown fields in `extra`. For nullable,
optional Rust fields, `None` represents an absent key and `Some(None)` an
explicit null.

Use the `_csv` and `_ndjson` variants on paginated operations, or `Csv` and
`Ndjson` suffixes in Node. These return the response text and headers such as
`metadata.next_cursor`. For JSON, read `data.next_cursor` on the response.
Send the cursor with the same query to continue. Each method issues a single
request. Pagination and async job polling are caller-controlled.
[API conventions](https://docs.hyperdash.com/data-api/conventions)

Use `get_quote` (`getQuote` in Node) for a credit estimate. Where the operation
supports it, pass `max_credits` and `credit_price_version` to bound the cost or
lock the quoted price. On failure, inspect the HTTP status, response metadata
including `retry_after`, and native problem details. The Python and Node
bindings expose them on `DataVendorError`. [Credits](https://docs.hyperdash.com/data-api/credits) · [Errors](https://docs.hyperdash.com/data-api/errors)

## Live streams

Open a session with `open_stream` or `openStream` in Node. One authenticated
WebSocket supports up to 20 simultaneous subscriptions. The session responds to
pings while open and surfaces the vendor's control messages, gaps, and close
codes. A Rust custom WebSocket transport must support authenticated upgrades.
Built-in stream I/O uses Tokio.

Read and process each `data` batch, persist its cursor with the processed data,
then call `ack(sub, batch)` to authorize the charge. The next batch for that
subscription waits for the acknowledgement. Use `acked` to confirm the charge.
Keep the subscription's last confirmed cursor for reconnects and reconcile
any batch whose acknowledgement was interrupted. Reconnect explicitly and
supply your cursor to `subscribe`. Handle `gap` by backfilling the reported
range with REST. [Stream protocol](https://docs.hyperdash.com/data-api/streams)

```python
async with await client.open_stream() as stream:
    await stream.subscribe({"channel": "trades", "coin": "BTC", "cursor": saved_cursor})
    async for message in stream:
        if message["op"] == "data":
            await process_and_checkpoint(message["events"], message["cursor"])
            await stream.ack(message["sub"], message["batch"])
        elif message["op"] == "gap":
            await backfill(message["from"], message["to"])
```

## Sources and verification

`API_SOURCES.json` pins the vendor spec and the supplemental route contracts.
Run `python3 scripts/generate-hyperdash-bindings.py` from the workspace root to
regenerate the models, bindings, and synthetic dispatch fixtures. The generator
supports `--check`. Rust, Python, and Node tests use local peers with synthetic
credentials.
