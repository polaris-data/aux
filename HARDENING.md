# Transport recovery and lifecycle

The standard Rust transports provide bounded safe-read retries, WebSocket
recovery with explicit discontinuities, fresh snapshot enforcement,
cancellation/shutdown, and credential-safe diagnostics. Injected transports own
their recovery/lifecycle policies. Native codecs, error codes, decimals, and
batch outcomes remain exchange-specific.

## Defaults and controls

Configure the built-in transport when constructing the client:

```rust,ignore
use aux_rs::{
    TransportConfig,
    hyperliquid::{Hyperliquid, HyperliquidClient},
};
use std::time::Duration;

let client = HyperliquidClient::with_config(
    Hyperliquid::testnet(),
    TransportConfig {
        timeout: Duration::from_secs(5),
        subscription_capacity: 64,
        ..TransportConfig::default()
    },
)?;
```

| Setting | Default |
|---|---|
| Network operation timeout | 10 seconds |
| Native heartbeat interval / idle timeout | 20 / 60 seconds |
| Logical subscription buffer | 128 data frames |
| Inbound WebSocket message / HTTP response cap | 8 MiB |
| Extra safe-read attempts | 2 |
| Extra reconnect attempts per consecutive-failure episode | 3 |
| Exponential backoff | 100 ms initially, capped at 2 seconds |
| Quiet connection stability period | 30 seconds |

Rust configures `TransportConfig.read_retry` and `.reconnect` separately with `BackoffConfig { max_retries, initial_delay, max_delay }`. `BackoffConfig::disabled()` disables either mechanism. Positive timeouts/capacities, `idle_timeout > heartbeat_interval`, and a positive stability period are required. Enabled backoff policies allow at most 16 retries and require `0 < initial_delay <= max_delay <= 60 seconds`. Heartbeat/idle deadlines that cannot be represented by the platform clock are rejected.

`StandardTransport::with_tls` supplies a common custom TLS policy for HTTP and WebSockets. The standard stack shares a Rustls trust policy and Ring provider across both protocols.

Python clients expose `read_retries`, `reconnect_retries`, `initial_backoff_ms`, `max_backoff_ms`, `reconnect_stable_ms`, `heartbeat_interval_ms`, `idle_timeout_ms`, `max_message_bytes`, `timeout_ms`, and `subscription_capacity`. Python's initial/max backoff values configure both policies. Set either retry count to zero for one-attempt/terminal behavior.

## Custom transports

No exchange is selected by default. The `http` and `ws` features select standard
transports independently of exchange features. Each adapter includes its native
signing code without enabling standard transports or an async runtime.
Without features, `aux-rs-core` has zero dependencies.

For custom I/O:

```toml
aux-rs = { path = "../connectors/crates/aux-rs", default-features = false, features = ["hyperliquid"] }
```

Implement `HttpTransport` and/or `WebSocketTransport`, then pass the implementation to `HyperliquidClient::with_transport(exchange, my_transport)`. The [protocol tests](crates/exchanges/hyperliquid/tests/protocol.rs) include a working custom HTTP transport. These contracts use `Send` futures without `async-trait`. Custom transports own their configuration and resource lifecycle.

## Connections and subscriptions

Cloned clients share their HTTP connection pool and WebSocket sessions per URL
and protocol scope. Hyperliquid public book/trade streams share one scope.
Account streams are isolated by account because order updates omit the user
address. Aggregated fills have a separate scope because their events omit the
aggregation parameter. Custom transports must honor `connection_scope` when
pooling.

Subscription creation waits for the send or local attachment, without waiting
for a remote acknowledgement. Native `SubscriptionResponse` events and API
errors expose the remote result. A late consumer of an existing subscription
may receive data without a new acknowledgement.

Late Hyperliquid consumers start with live data; earlier snapshots are not
replayed from a client cache. Lighter duplicate consumers get a fresh native
snapshot over a separate session. The SDK does not maintain an application order
book. A handle routes only matching native channel/coin data. Unknown channels
and malformed frames are exposed explicitly; application-level heartbeat pongs
are handled internally.

## Safe reads and submission outcomes

`HttpTransport.execute()` executes once, regardless of HTTP method. A typed `Endpoint` must explicitly opt in with `SAFE_TO_RETRY = true` to use `execute_read()`. Custom endpoints default to false; a public endpoint or GET method alone does not authorize replay. Custom transports' default `execute_read()` still executes once.

The built-in public metadata/book/account/trade/nonce reads opt in, including Hyperliquid's POST `/info` reads. Lighter's authenticated active-order read also opts in. Standard reads retry connection/timeouts and HTTP `408`, `429`, `500`, `502`, `503`, or `504`, within the configured extra-attempt budget. They preserve the same body and headers across attempts. Decode errors, oversized responses, native failures inside HTTP `200`, other statuses and redirects are not automatically retried. Final native responses/errors remain available to callers.

`Retry-After` supports both delta seconds and HTTP dates as defined in [RFC 9110 §10.2.3](https://www.rfc-editor.org/rfc/rfc9110.html#section-10.2.3), using the focused `httpdate` parser. Backoff never retries earlier than a valid server delay. If that delay exceeds the configured cap, the native response is returned immediately for application scheduling. There is no global rate-limit scheduler or automatic retry of arbitrary venue error codes.

Trading always uses the one-attempt executor. HTTP `503`, disconnect, timeout, cancellation or shutdown never triggers automatic replay, a new signature or a replacement nonce. A transport error's `RequestOutcome` remains `NotSent` or `Unknown`; absence of an acknowledgement does not prove rejection. Lighter retains its quarantine until explicit reconciliation followed by `resync_nonce()`. An accepted transaction still needs native execution reconciliation.

## WebSocket recovery and gaps

After an established session disconnects or exceeds its idle deadline, compatible logical subscriptions recover over a new connection using bounded exponential backoff. Shared remote keys are resubscribed once. New consumers arriving during recovery can establish an independent session. The first initial connection attempt remains bounded by the operation timeout and reports connection failure directly.

Every affected consumer receives a recoverable transport error with `TransportErrorKind::Gap` before any post-recovery data. Stale queued frames are cleared atomically when continuity is lost; the gap lives outside the bounded data queue. Several outages before polling may coalesce into one unread gap. A terminal error may supersede an unread gap. Neither behavior implies continuous delivery.

On a gap, discard or reconcile application-derived state and keep polling for native events. Hyperliquid resumes native subscription acknowledgements and snapshots/events. Lighter must deliver a fresh `subscribed/...` snapshot before a delta; an update arriving first is a terminal protocol error. The SDK preserves nonce/offset fields and zero-size deltas without reconstructing an application order book.

```rust,ignore
while let Some(result) = subscription.next().await {
    match result {
        Err(aux_rs::Error::Transport(error)) if error.is_gap() => {
            discard_derived_state(); // Application-owned reconciliation.
        }
        Ok(event) => handle_native_event(event),
        Err(error) => return Err(error.into()),
    }
}
```

Python raises `GapError`, a subclass of `TransportError`, while leaving the iterator open. Catch it and continue polling; a plain `async for` does not suppress errors:

```python
from aux_py import GapError

async with await client.subscribe_order_book(market_id) as stream:
    while True:
        try:
            event = await anext(stream)
        except GapError:
            discard_derived_state()  # Application-owned reconciliation.
            continue
        except StopAsyncIteration:
            break
        handle_native_event(event)
```

The consecutive reconnect budget resets after meaningful native data, or after the configured stability period on a quiet connection. Handshakes, greetings and acknowledgements alone cannot create an unlimited rapid reconnect loop. Exhaustion returns a terminal error. Reconnect dialing and restoring the subscription batch each have bounded deadlines; shutdown interrupts recovery waits and handshakes.

Overflow, invalid protocol/binary frames, oversized messages and expired credentials are terminal, with explicit errors. Overflow cannot be hidden by buffered data or repaired by silently reconnecting. Open a new stream after application reconciliation. Malformed native JSON is surfaced by the adapter; Python closes the iterator on decoding/native errors.

Lighter private order subscriptions can reconnect while their original auth token remains valid. Before replay the token deadline is checked; expired tokens are never sent. An `Authentication` transport error (`AuthenticationError` in Python) terminates that stream so the application can request a new subscription/token. Background token refresh is not implemented. Account/signer/key scopes remain isolated during recovery.

## Close, shutdown and diagnostics

`next()` is cancellation-safe. `unsubscribe()`/`close()` removes a logical consumer; the final consumer sends a remote unsubscribe. The last subscription closes its socket. During an outage, removing the last consumer cancels pending recovery rather than dialing again. Drop schedules local/remote cleanup without waiting; explicit close awaits bounded cleanup, not a server acknowledgement.

`StandardTransport.request_shutdown()` synchronously marks all clones closed and signals cancellation. `await transport.shutdown()` also waits for initial handshakes, HTTP operations and socket drivers, clears the session registry and releases the HTTP pool. New operations on any clone return `Closed`/`NotSent`. New trading calls fail before nonce allocation/signing. In-flight writes can return `Closed`/`Unknown`; they are not replayed. Shutdown attempts native unsubscribe for active keys within one aggregate deadline before bounded socket close. Offline or failed connections cannot guarantee remote acknowledgement.

Python `aclose()` closes owned subscriptions, cancels pending reads/trading/signing callbacks, signals the shared transport, and waits for its cleanup. Cleanup continues if a close await is cancelled. Cancellation of a stream read closes that iterator; a recoverable gap does not. Transport/native error formatting excludes endpoint URLs, headers, raw bodies, server credential echoes and callback secrets. Native response details remain explicitly inspectable in structured fields.

## Evidence and remaining release gates

Offline fault tests cover native POST/read retry classification, transient/permanent statuses, `Retry-After` bounds, connection loss, decode/size failures, unchanged authenticated headers, both venues' single-attempt trading, Lighter quarantine, buffered-data invalidation, duplicate-key resubscription, fresh snapshots, reconnect exhaustion/stability, expiry, heartbeat loss, malformed/oversized frames, unsubscribe/drop during backoff, shutdown during initial/reconnect handshakes and HTTP work, and Python recovery/cleanup.

Run `bash scripts/check.sh` and `bash scripts/check-python.sh`. Normal checks use fixtures/local servers and do not place orders. Dependency profiles cover core without dependencies, each exchange independently, both together, HTTP/WS/custom transports and the isolated Python crate.

This hardening does not complete funded authenticated testnet submit/cancel/private-stream verification, the review of Lighter's unaudited local cryptographic primitives, or Python's release-wheel/platform matrix. Those remain explicit release gates; public smoke examples are separate read-only checks.
