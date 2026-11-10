# Birdeye market data

`aux-rs-birdeye` covers **46 REST endpoints** from Birdeye's Price & OHLCV,
Stats, Token/Market List, Transactions, and Security API groups. Rust,
`aux_py.BirdeyeClient`, and `aux-ts`'s `BirdeyeClient` expose the same operations.
See the [complete endpoint/method table](COVERAGE.md).

Other Data API groups, Perps, Blockchain Data, and WebSockets are outside this
connector's scope. Both the current and documented legacy market-data routes
are included; versions remain explicit in method names.

## Rust

Enable the facade's `birdeye` feature and `http` for built-in networking:

```rust,no_run
use aux_rs::birdeye::{BirdeyeClient, PriceRequest};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = BirdeyeClient::new(std::env::var("BIRDEYE_API_KEY")?)?
    .with_chain("solana")?;
let result = client.price(PriceRequest {
    address: "So11111111111111111111111111111111111111112".into(),
    include_liquidity: Some(true),
    ..Default::default()
}).await?;
println!("{}", result.data);
client.transport().shutdown().await?;
# Ok(())
# }
```

`with_config(api_url, api_key, config)` configures built-in HTTP and timeouts;
`with_transport(api_url, api_key, transport)` accepts a caller-supplied
`StreamingHttpTransport`. The adapter has no default transport feature.

## Python

```python
import os
from decimal import Decimal
from aux_py import BirdeyeClient

async def example():
    async with BirdeyeClient(os.environ["BIRDEYE_API_KEY"], chain="solana") as client:
        result = await client.price({
            "address": "So11111111111111111111111111111111111111112",
            "check_liquidity": Decimal("0.1000000000000000001"),
        })
        print(result["data"])
```

## TypeScript (Node)

```ts
import { BirdeyeClient, exactNumber } from "aux-ts";

const client = new BirdeyeClient({
  apiKey: process.env.BIRDEYE_API_KEY!, chain: "solana",
});
try {
  const result = await client.price({ request: {
    address: "So11111111111111111111111111111111111111112",
    check_liquidity: exactNumber("0.1000000000000000001"),
  }});
  console.log(result.data);
} finally {
  await client.close();
}
```

Constructor options are `api_key`, `chain`, `api_url`, and `timeout_ms` in Python
and camelCase in TypeScript. Request fields retain Birdeye's native spelling,
including `type`, `ui_amount_mode`, and `list_address`.

POST request objects keep native query fields at the top level and JSON payloads
under `body`. For example, `multi_price_post` / `multiPricePost` accepts:

```json
{
  "include_liquidity": true,
  "body": { "list_address": "TOKEN_A,TOKEN_B" }
}
```

The liquidity-latest-token body uses `addresses`, an array, rather than a
comma-separated `list_address`. The security endpoint accepts an `include` array,
encoded as a comma-separated query parameter. Typed request structures and
public declarations are generated from the pinned official operation schemas.
Enums remain strings so new server values are usable; server constraints and
package/chain entitlements remain native API rejections.

## Behavior

- Base URL: `https://public-api.birdeye.so`. Authentication uses `X-API-KEY`;
  `x-chain` defaults to `solana`. Choose the chain explicitly for other networks,
  including Base-only endpoints. Custom API URLs and timeouts are supported.
- Responses retain the `{ success, data, ... }` envelope. `data` is exact native
  JSON, including null, dynamic token-address maps, candles, transactions, security
  fields, pagination metadata, and chain/version-specific fields. The connector
  does not normalize these into a common market-data schema.
- Rust preserves JSON number tokens. Python returns integers and `Decimal`;
  Node returns safe dynamic integers as numbers, larger integers as `bigint`,
  and fractional numeric tokens as exact strings. Use `Decimal` / `exactNumber`
  for fractional numeric request tokens. Fractional floats are rejected.
- Optional parameters stay omitted. Queries with documented server defaults can
  omit those fields. Scroll continuation can send only `scroll_id`; pagination,
  polling, and retries are explicit caller work. POST query parameters stay in
  the URL, separate from their JSON body.
- HTTP failures and HTTP-200 `success: false` responses raise native data-vendor
  errors, preserving status, codes and redacted details. A 401 is authentication,
  403 is entitlement, and 429 is rate limiting. Error messages omit raw bodies.
- Credentials are redacted in error bodies and diagnostics and removed on
  redirects to a different origin. HTTPS cannot downgrade to HTTP. POST redirects
  are rejected. Requests are not implicitly retried.
- Responses are bounded to 16 MiB after decompression; use pagination for larger
  datasets. Python cancellation and explicit Python/Node close interrupt pending
  work. No file downloads or WebSocket subscriptions are added.

## Generation and verification

[Source pins](API_SOURCES.json) record the official OpenAPI/documentation source
hashes and the owned inventory and fixture hashes. All fixtures are synthetic;
no paid API calls or real credentials are required.

`operations.json` generates Rust requests/methods, Python native/public wrappers
and stubs, Node operation glue and request declarations, and coverage tables.
These checks require no native binary builds:

```sh
python3 scripts/generate-birdeye-bindings.py --check
python3 scripts/generate-node-bindings.py --check
python3 scripts/check-birdeye-source.py
node scripts/check-birdeye-source.mjs
python3 scripts/check-data-coverage.py
python3 scripts/check-node-coverage.py
npm --prefix crates/node run typecheck
```

The Python and Node source checks use fake native boundaries. They exercise all
46 public wrapper dispatches, constructor defaults, and exact Node input encoding;
they do not validate Rust networking or native FFI. Rust fixture contracts and
local-server Python/Node tests are included for execution after a native build.
`typescript.json` is generated alongside the Rust primitive request fields so
Node declarations can refresh without running the compiled ts-rs exporter.
