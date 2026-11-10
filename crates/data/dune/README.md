# Dune

`aux-rs-dune` implements Dune's nine **Executions and Results** REST endpoints.
The same Rust client powers `aux_py.DuneClient` and `aux-ts`'s `DuneClient`
(Node 22+). Enable the Rust facade's `dune` feature; add `http` for built-in HTTP.
No other Dune API family is included.

Base URL: `https://api.dune.com/api/v1`. Authentication uses `X-Dune-API-Key`.
Custom URLs and timeouts are supported. The API key is redacted from native
error bodies and diagnostics and removed on redirects to another origin.
HTTPS redirects cannot downgrade to HTTP. Execution/cancellation POSTs never
follow redirects. Every operation makes one request; there are no implicit
retries, polling loops or pagination loops.

| Endpoint (relative to base URL) | Rust / Python | TypeScript |
| --- | --- | --- |
| `POST /sql/execute` | `execute_sql` | `executeSql` |
| `POST /query/{query_id}/execute` | `execute_query` | `executeQuery` |
| `POST /query/{query_id}/pipeline/execute` | `execute_query_pipeline` | `executeQueryPipeline` |
| `GET /execution/{execution_id}/status` | `get_execution_status` | `getExecutionStatus` |
| `GET /execution/{execution_id}/results` | `get_execution_results` | `getExecutionResults` |
| `GET /execution/{execution_id}/results/csv` | `get_execution_results_csv` | `getExecutionResultsCsv` |
| `GET /query/{query_id}/results` | `get_latest_query_results` | `getLatestQueryResults` |
| `GET /query/{query_id}/results/csv` | `get_latest_query_results_csv` | `getLatestQueryResultsCsv` |
| `POST /execution/{execution_id}/cancel` | `cancel_execution` | `cancelExecution` |

SQL and saved-query execution return an `execution_id`. Query-pipeline execution
returns a **`pipeline_execution_id`**, which requires the separate pipeline
monitoring API (outside this connector's scope).

## Rust

```rust,no_run
use aux_rs::dune::{DuneClient, ExecuteSqlRequest, ResultOptions};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = DuneClient::new(std::env::var("DUNE_API_KEY")?)?;
let execution = client.execute_sql(ExecuteSqlRequest {
    sql: "select 1 as value".into(), performance: None,
}).await?;
let status = client.get_execution_status(&execution.execution_id).await?;
if status.state == "QUERY_STATE_COMPLETED" {
    let results = client.get_execution_results(
        &execution.execution_id, ResultOptions::default(),
    ).await?;
    println!("{:?}", results.result);
}
client.transport().shutdown().await?;
# Ok(())
# }
```

`DuneClient::with_config(api_url, api_key, config)` configures built-in HTTP;
`with_transport(api_url, api_key, transport)` accepts caller-supplied
`StreamingHttpTransport`. The crate has no default transport feature.

## Python

```python
import os
from aux_py import DuneClient

async def example():
    async with DuneClient(os.environ["DUNE_API_KEY"]) as client:
        execution = await client.execute_query(123)
        status = await client.get_execution_status(execution["execution_id"])
        if status["state"] == "QUERY_STATE_COMPLETED":
            result = await client.get_execution_results(
                execution["execution_id"], {"limit": 100}
            )
            print(result.get("result"))
```

## TypeScript (Node)

```ts
import { DuneClient } from "aux-ts";

const client = new DuneClient({ apiKey: process.env.DUNE_API_KEY! });
try {
  const execution = await client.executeQuery({ queryId: 123n });
  const status = await client.getExecutionStatus({ executionId: execution.execution_id });
  if (status.state === "QUERY_STATE_COMPLETED") {
    const result = await client.getExecutionResults({
      executionId: execution.execution_id, request: { limit: 100n },
    });
    console.log(result.result);
  }
} finally {
  await client.close();
}
```

Native request field names stay in snake_case, including in TypeScript's `request`.
Optional execution tiers and result options are omitted when unset so Dune keeps
its server defaults. Pipeline execution requires a performance tier. Saved-query
parameters are supplied as a native JSON object. Result options support `limit`,
`offset`, `columns`, `filters`, `sort_by`, `sample_count`, `allow_partial_results`
and `ignore_max_credits_per_request`. Sampling cannot combine with limit, offset
or filters. Credit/partial-result overrides require explicit caller selection.

JSON results preserve Dune's envelope, timestamps, native states, error details,
column metadata and dynamic row schemas, including unknown fields. Rust keeps
exact JSON numbers, Python returns integers/`Decimal`, and Node returns typed
64-bit integers as `bigint`, dynamic large integers as `bigint`, and dynamic
fractional numbers as exact strings. Python requests accept `Decimal`; Node
requests use `exactNumber(...)` for numeric decimal tokens. Fractional floats
are rejected at the binding boundaries.

CSV methods return `{ data, next_offset, next_uri }`: untouched UTF-8 text and
`X-Dune-Next-Offset` / `X-Dune-Next-Uri` continuation headers. Each JSON or CSV
response is bounded to 16 MiB after decompression. Use `limit` and continue with
`next_offset` to retrieve larger results. Continuation URIs are returned as
metadata and are never followed automatically. Python cancellation/client close
and Node client close interrupt pending requests; cancellation does not cancel
the server's execution. Use `cancel_execution` explicitly when needed.

## Sources and verification

[Source pins](API_SOURCES.json) record the official documentation and the
inventory/fixture hashes. Fixtures are synthetic offline protocol contracts;
no real API key or paid query is required for tests. These tests establish local
request/response compatibility, not funded live verification.

`operations.json` is the source for Rust native operations, Python native methods,
public Python wrappers/stubs, Node operation glue, and the coverage inventory.
Run these from the repository root:

```sh
python3 scripts/generate-dune-bindings.py
python3 scripts/generate-node-bindings.py --models
cargo test --locked -p aux-rs-dune --all-features
python3 scripts/check-data-coverage.py
```

Both generators support `--check`. Normal builds use checked-in generated output
and require no Dune schema downloads or external Dune SDK dependencies.
