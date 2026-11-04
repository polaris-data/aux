use crate::prelude::*;
use crate::{
    convert, errors,
    subscription::{NativeHandle, Subscription, SubscriptionState},
};
use std::{
    future::Future,
    sync::{Arc, Mutex, Weak},
    time::Duration,
};
use tokio::sync::watch;

pub struct ClientState<P> {
    client: Mutex<Option<P>>,
    closing: watch::Sender<bool>,
    subscriptions: Mutex<Vec<Weak<SubscriptionState>>>,
    transport: aux_rs::StandardTransport,
}
pub trait ClientResources: Clone + Send + Sync + 'static {
    fn transport(&self) -> &aux_rs::StandardTransport;
}
impl<P: ClientResources> ClientState<P> {
    pub async fn object<F, T>(self: &Arc<Self>, future: F) -> Result<T>
    where
        F: Future<Output = Result<T>> + Send,
        T: Send,
    {
        let mut closing = self.closing.subscribe();
        if self.is_closed() {
            return Err(errors::ClosedError::new_err("client is closed"));
        }
        tokio::select! {biased;_=closing.changed()=>Err(errors::ClosedError::new_err("client is closed")),result=future=>result}
    }
    pub fn new(client: P) -> Arc<Self> {
        Arc::new(Self {
            transport: client.transport().clone(),
            client: Mutex::new(Some(client)),
            closing: watch::channel(false).0,
            subscriptions: Mutex::default(),
        })
    }
    pub fn client(&self) -> Result<P> {
        self.client
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .cloned()
            .ok_or_else(|| errors::ClosedError::new_err("client is closed"))
    }
    pub fn is_closed(&self) -> bool {
        *self.closing.borrow()
    }
    fn register(&self, state: &Arc<SubscriptionState>) -> bool {
        let mut subscriptions = self.subscriptions.lock().unwrap_or_else(|e| e.into_inner());
        if self.is_closed() {
            return false;
        }
        subscriptions.retain(|w| w.strong_count() > 0);
        subscriptions.push(Arc::downgrade(state));
        true
    }
    pub fn request_close(&self) -> Vec<Arc<SubscriptionState>> {
        let (states, client): (Vec<_>, _) = {
            let subscriptions = self.subscriptions.lock().unwrap_or_else(|e| e.into_inner());
            self.closing.send_replace(true);
            let client = self.client.lock().unwrap_or_else(|e| e.into_inner()).take();
            (
                subscriptions.iter().filter_map(Weak::upgrade).collect(),
                client,
            )
        };
        // Release both Rust mutexes before dropping credential and callback owners.
        for state in &states {
            state.request_close();
        }
        self.transport.request_shutdown();
        drop(client);
        states
    }
    pub async fn run<F, T>(self: &Arc<Self>, future: F) -> Result<crate::convert::Output>
    where
        F: Future<Output = Result<T>> + Send,
        T: serde::Serialize + Send,
    {
        convert::output(self.object(future).await?)
    }
    pub async fn stream<F>(self: &Arc<Self>, future: F) -> Result<Subscription>
    where
        F: Future<Output = Result<NativeHandle>> + Send,
    {
        let handle = self.object(future).await?;
        let subscription = SubscriptionState::new(handle);
        if !self.register(&subscription) {
            subscription.request_close();
            let _ = subscription.wait_closed().await;
            return Err(errors::ClosedError::new_err("client is closed"));
        }
        Ok(Subscription {
            state: subscription,
        })
    }
    pub async fn close(&self) -> Result<()> {
        let states = self.request_close();
        let mut first = None;
        for state in states {
            if let Err(e) = state.wait_closed().await {
                first.get_or_insert(e);
            }
        }
        if let Err(e) = self.transport.shutdown().await {
            first.get_or_insert_with(|| errors::transport(&e));
        }
        first.map_or(Ok(()), Err)
    }
}
#[allow(clippy::too_many_arguments)]
pub fn config(
    timeout_ms: u64,
    capacity: usize,
    read_retries: usize,
    reconnect_retries: usize,
    initial_backoff_ms: u64,
    max_backoff_ms: u64,
    heartbeat_interval_ms: u64,
    idle_timeout_ms: u64,
    max_message_bytes: usize,
    reconnect_stable_ms: u64,
) -> Result<aux_rs::TransportConfig> {
    let backoff = |max_retries| aux_rs::BackoffConfig {
        max_retries,
        initial_delay: Duration::from_millis(initial_backoff_ms),
        max_delay: Duration::from_millis(max_backoff_ms),
    };
    let config = aux_rs::TransportConfig {
        timeout: Duration::from_millis(timeout_ms),
        subscription_capacity: capacity,
        heartbeat_interval: Duration::from_millis(heartbeat_interval_ms),
        idle_timeout: Duration::from_millis(idle_timeout_ms),
        max_message_bytes,
        read_retry: backoff(read_retries),
        reconnect: backoff(reconnect_retries),
        reconnect_stable_after: Duration::from_millis(reconnect_stable_ms),
    };
    config
        .validate()
        .map_err(|e| errors::ValidationError::new_err(e.to_string()))?;
    Ok(config)
}
pub fn build<T: Send>(
    f: impl FnOnce() -> std::result::Result<T, aux_rs::TransportError> + Send,
) -> Result<T> {
    napi::bindgen_prelude::within_runtime_if_available(f).map_err(|e| errors::transport(&e))
}
