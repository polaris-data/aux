//! Opt-in standard HTTP/WebSocket transports, with one TLS policy.

use std::sync::Arc;
use tokio::sync::watch;

use crate::{RequestOutcome, TransportConfig, TransportError, TransportErrorKind};

#[cfg(feature = "http")]
mod http;
#[cfg(feature = "ws")]
mod ws;
#[cfg(feature = "ws")]
pub use ws::StandardSubscription;

/// Clones reuse the HTTP pool and the WebSocket session registry.
#[derive(Clone)]
pub struct StandardTransport {
    config: TransportConfig,
    #[cfg(feature = "http")]
    client: Arc<std::sync::Mutex<Option<reqwest::Client>>>,
    lifecycle: Arc<Lifecycle>,
    #[cfg(feature = "ws")]
    tls: Arc<rustls::ClientConfig>,
    #[cfg(feature = "ws")]
    sessions: Arc<tokio::sync::Mutex<std::collections::HashMap<(String, String), ws::Session>>>,
}

struct Lifecycle {
    gate: std::sync::Mutex<()>,
    closed: watch::Sender<bool>,
    drivers: watch::Sender<usize>,
}
impl Lifecycle {
    fn new() -> Self {
        Self {
            gate: std::sync::Mutex::new(()),
            closed: watch::channel(false).0,
            drivers: watch::channel(0).0,
        }
    }
    fn check(&self) -> Result<(), TransportError> {
        if *self.closed.borrow() {
            Err(closed(RequestOutcome::NotSent))
        } else {
            Ok(())
        }
    }
}
struct OperationGuard(Arc<Lifecycle>);
impl OperationGuard {
    fn start(lifecycle: &Arc<Lifecycle>) -> Result<Self, TransportError> {
        let _gate = lifecycle.gate.lock().unwrap_or_else(|e| e.into_inner());
        lifecycle.check()?;
        lifecycle.drivers.send_modify(|n| *n += 1);
        Ok(Self(lifecycle.clone()))
    }
}
impl Drop for OperationGuard {
    fn drop(&mut self) {
        self.0.drivers.send_modify(|n| *n = n.saturating_sub(1));
    }
}
fn closed(outcome: RequestOutcome) -> TransportError {
    TransportError::new(
        TransportErrorKind::Closed,
        outcome,
        "transport is shut down",
    )
}

impl std::fmt::Debug for StandardTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StandardTransport")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

impl StandardTransport {
    /// Synchronous cancellation signal; shutdown continues if its await is cancelled.
    /// All clones share this lifecycle, so none can open new operations afterward.
    pub fn request_shutdown(&self) {
        {
            let _gate = self
                .lifecycle
                .gate
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            self.lifecycle.closed.send_replace(true);
        }
        #[cfg(feature = "http")]
        {
            let client = self.client.lock().unwrap_or_else(|e| e.into_inner()).take();
            drop(client);
        }
    }
    /// Cancel in-flight operations, close pooled sockets and wait for drivers.
    /// Waits for in-flight HTTP futures and socket drivers to finish.
    pub async fn shutdown(&self) -> Result<(), TransportError> {
        self.request_shutdown();
        let mut drivers = self.lifecycle.drivers.subscribe();
        loop {
            let idle = *drivers.borrow() == 0;
            if idle {
                #[cfg(feature = "ws")]
                self.sessions.lock().await.clear();
                return Ok(());
            }
            if drivers.changed().await.is_err() {
                return Ok(());
            }
        }
    }
    pub fn new(config: TransportConfig) -> Result<Self, TransportError> {
        let roots =
            rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
        let tls = rustls::ClientConfig::builder_with_provider(Arc::new(
            rustls::crypto::ring::default_provider(),
        ))
        .with_safe_default_protocol_versions()
        .map_err(|_| configuration_error("unable to configure TLS protocol versions"))?
        .with_root_certificates(roots)
        .with_no_client_auth();
        Self::with_tls(config, tls)
    }

    /// Use a custom trust store or client identity for both HTTP and WebSockets.
    /// HTTP advertises HTTP/1.1 and WebSocket handshakes use HTTP/1.1.
    pub fn with_tls(
        config: TransportConfig,
        mut tls: rustls::ClientConfig,
    ) -> Result<Self, TransportError> {
        config.validate()?;
        tls.alpn_protocols = vec![b"http/1.1".to_vec()];
        #[cfg(feature = "http")]
        let client = reqwest::Client::builder()
            .use_preconfigured_tls(tls.clone())
            .timeout(config.timeout)
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .user_agent(concat!("aux-rs/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|_| configuration_error("unable to build HTTP client"))?;
        Ok(Self {
            config,
            #[cfg(feature = "http")]
            client: Arc::new(std::sync::Mutex::new(Some(client))),
            lifecycle: Arc::new(Lifecycle::new()),
            #[cfg(feature = "ws")]
            tls: Arc::new(tls),
            #[cfg(feature = "ws")]
            sessions: Arc::default(),
        })
    }
}

fn configuration_error(message: &'static str) -> TransportError {
    TransportError::new(
        TransportErrorKind::Configuration,
        RequestOutcome::NotSent,
        message,
    )
}
