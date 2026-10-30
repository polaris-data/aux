use std::{fmt, future::Future, time::Duration};

use crate::{RequestOutcome, TransportError, TransportErrorKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl fmt::Debug for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpRequest")
            .field("method", &self.method)
            .field("body_bytes", &self.body.len())
            .finish_non_exhaustive()
    }
}

pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl fmt::Debug for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpResponse")
            .field("status", &self.status)
            .field("body_bytes", &self.body.len())
            .finish_non_exhaustive()
    }
}

pub trait HttpTransport: Send + Sync {
    /// Preflight before allocating/signing protected requests. Custom transports
    /// may override this to expose their lifecycle; it never sends a request.
    fn ensure_open(&self) -> Result<(), TransportError> {
        Ok(())
    }
    /// Execute once. Never retry a potentially submitted write automatically.
    fn execute(
        &self,
        request: HttpRequest,
    ) -> impl Future<Output = Result<HttpResponse, TransportError>> + Send;
    /// Explicitly safe read. Default custom transports still execute once.
    /// Never use for writes, irrespective of their HTTP method.
    fn execute_read(
        &self,
        request: HttpRequest,
    ) -> impl Future<Output = Result<HttpResponse, TransportError>> + Send {
        self.execute(request)
    }
}

/// A dedicated, ordered WebSocket connection. Trading connections must never
/// reconnect or replay writes automatically. Cancelling `next` must not consume
/// a message; a failed send may have reached the exchange.
pub trait WebSocketConnection: Send {
    /// Wait for one message within the supplied response budget. Timeout must
    /// not consume a later message; callers reconcile ambiguous writes explicitly.
    fn next_with_timeout(
        &mut self,
        timeout: Duration,
    ) -> impl Future<Output = Option<Result<String, TransportError>>> + Send;
    fn send(&mut self, text: String) -> impl Future<Output = Result<(), TransportError>> + Send;
    fn next(&mut self) -> impl Future<Output = Option<Result<String, TransportError>>> + Send;
    fn close(&mut self) -> impl Future<Output = Result<(), TransportError>> + Send;
}

/// Opens isolated WebSocket connections for protocols with session authentication
/// and order commands. Subscription pooling remains a separate transport contract.
pub trait WebSocketConnector: Send + Sync {
    type Connection: WebSocketConnection;
    fn connect(
        &self,
        url: &str,
    ) -> impl Future<Output = Result<Self::Connection, TransportError>> + Send;
}

/// Wire-level subscription behavior supplied by the exchange adapter.
///
/// Keys must include all subscription parameters. Equal keys normally share one
/// remote subscription, except when a fresh snapshot is required. `matches`
/// must route malformed or error messages for explicit
/// handling rather than silently discarding them.
pub trait WebSocketSubscription: Send + Sync + 'static {
    /// Fixed native credentials can forbid replay after expiry. Applications
    /// then open a new subscription with fresh credentials.
    fn resubscribe_allowed(&self) -> bool {
        true
    }
    /// For delta feeds, reject updates that precede a new native snapshot.
    fn is_initial_snapshot(&self, _text: &str) -> bool {
        true
    }
    /// Only meaningful native data resets a consecutive reconnect budget.
    fn is_data_message(&self, _text: &str) -> bool {
        true
    }
    /// Protocols whose messages omit subscription/account identity can request
    /// an isolated session. Equal URL/scope pairs share a connection.
    fn connection_scope(&self) -> String {
        String::new()
    }
    fn key(&self) -> String;
    /// Delta protocols need a fresh native snapshot for every new consumer.
    /// Such subscriptions may share a session with different keys, but an
    /// already-active key must use a separate session instead of deduplication.
    fn requires_fresh_snapshot(&self) -> bool {
        false
    }
    fn subscribe_message(&self) -> String;
    fn unsubscribe_message(&self) -> String;
    fn matches(&self, text: &str) -> bool;
    fn heartbeat_message(&self) -> Option<String> {
        None
    }
    /// Optional reply to exchange-level control frames (e.g. a JSON ping).
    /// Transports must send identical replies only once per connection/frame.
    fn response_message(&self, _text: &str) -> Option<String> {
        None
    }
}

/// A bounded, cancellable logical subscription; not an application order book.
pub trait WebSocketStream: Send {
    /// Cancellation-safe: cancelling this future must not consume an event.
    /// Gap errors are recoverable and precede fresh post-reconnect events.
    /// Other transport errors terminate the logical subscription.
    fn next(&mut self) -> impl Future<Output = Option<Result<String, TransportError>>> + Send;
    fn close(&mut self) -> impl Future<Output = Result<(), TransportError>> + Send;
}

pub trait WebSocketTransport: Send + Sync {
    type Stream: WebSocketStream;

    /// Implementations that pool connections must honor `connection_scope` to
    /// keep messages without account identity isolated from other accounts,
    /// and `requires_fresh_snapshot` so late consumers do not start with deltas.
    fn subscribe<S: WebSocketSubscription>(
        &self,
        url: &str,
        subscription: S,
    ) -> impl Future<Output = Result<Self::Stream, TransportError>> + Send;
}

#[derive(Clone, Debug)]
pub struct BackoffConfig {
    /// Extra attempts after the original attempt; zero disables recovery.
    pub max_retries: usize,
    pub initial_delay: Duration,
    pub max_delay: Duration,
}
impl BackoffConfig {
    pub fn disabled() -> Self {
        Self {
            max_retries: 0,
            initial_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
        }
    }
    pub fn delay(&self, retry: usize) -> Duration {
        self.initial_delay
            .saturating_mul(1u32 << retry.min(16))
            .min(self.max_delay)
    }
    fn valid(&self) -> bool {
        self.max_retries <= 16
            && (self.max_retries == 0
                || (!self.initial_delay.is_zero()
                    && self.max_delay >= self.initial_delay
                    && self.max_delay <= Duration::from_secs(60)))
    }
}

#[derive(Clone, Debug)]
pub struct TransportConfig {
    pub timeout: Duration,
    pub heartbeat_interval: Duration,
    pub idle_timeout: Duration,
    pub subscription_capacity: usize,
    pub max_message_bytes: usize,
    pub read_retry: BackoffConfig,
    pub reconnect: BackoffConfig,
    /// A quiet but stable connection also resets the consecutive retry budget.
    pub reconnect_stable_after: Duration,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            heartbeat_interval: Duration::from_secs(20),
            idle_timeout: Duration::from_secs(60),
            subscription_capacity: 128,
            max_message_bytes: 8 * 1024 * 1024,
            read_retry: BackoffConfig {
                max_retries: 2,
                initial_delay: Duration::from_millis(100),
                max_delay: Duration::from_secs(2),
            },
            reconnect: BackoffConfig {
                max_retries: 3,
                initial_delay: Duration::from_millis(100),
                max_delay: Duration::from_secs(2),
            },
            reconnect_stable_after: Duration::from_secs(30),
        }
    }
}

impl TransportConfig {
    pub fn validate(&self) -> Result<(), TransportError> {
        if self.timeout.is_zero()
            || self.heartbeat_interval.is_zero()
            || self.idle_timeout <= self.heartbeat_interval
            || self.subscription_capacity == 0
            || self.max_message_bytes == 0
            || self.reconnect_stable_after.is_zero()
            || std::time::Instant::now()
                .checked_add(self.heartbeat_interval)
                .is_none()
            || std::time::Instant::now()
                .checked_add(self.idle_timeout)
                .is_none()
            || !self.read_retry.valid()
            || !self.reconnect.valid()
        {
            return Err(TransportError::new(
                TransportErrorKind::Configuration,
                RequestOutcome::NotSent,
                "positive timeouts/capacities and idle > heartbeat required; backoff requires <=16 retries and 0 < initial <= max <=60s",
            ));
        }
        Ok(())
    }
}
