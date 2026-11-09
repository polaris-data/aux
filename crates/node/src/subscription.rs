use crate::prelude::*;
use crate::{
    convert,
    errors::{self, ClientError},
};
use aux_rs::{hyperliquid as hl, lighter as lt, standard::StandardSubscription};
use std::sync::{Arc, Mutex};
use tokio::sync::watch;

pub enum NativeHandle {
    Extended(aux_rs::extended::SubscriptionHandle<StandardSubscription>),
    Phoenix(aux_rs::phoenix::SubscriptionHandle<StandardSubscription>),
    Derive(aux_rs::derive::SubscriptionHandle<StandardSubscription>),
    Paradex(aux_rs::paradex::SubscriptionHandle<StandardSubscription>),
    Monaco(aux_rs::monaco::SubscriptionHandle<StandardSubscription>),
    Qfex(aux_rs::qfex::SubscriptionHandle<StandardSubscription>),
    Arcus(aux_rs::arcus::SubscriptionHandle<StandardSubscription>),
    QfexTrade(aux_rs::qfex::TradingSession<aux_rs::standard::StandardConnection>),
    Hyperliquid {
        handle: hl::SubscriptionHandle<StandardSubscription>,
        book_only: bool,
    },
    Lighter(lt::SubscriptionHandle<StandardSubscription>),
}
impl NativeHandle {
    async fn close(&mut self) -> Result<(), ClientError> {
        match self {
            Self::Extended(h) => h.close().await.map_err(ClientError::Extended),
            Self::Phoenix(h) => h.close().await.map_err(ClientError::Phoenix),
            Self::Derive(handle) => handle.close().await.map_err(ClientError::Derive),
            Self::Paradex(handle) => handle.close().await.map_err(ClientError::Paradex),
            Self::Monaco(handle) => handle.close().await.map_err(ClientError::Monaco),
            Self::Qfex(handle) => handle.close().await.map_err(ClientError::Qfex),
            Self::Arcus(handle) => handle.close().await.map_err(ClientError::Arcus),
            Self::QfexTrade(handle) => handle.close().await.map_err(ClientError::Qfex),
            Self::Hyperliquid { handle, .. } => {
                handle.close().await.map_err(ClientError::Hyperliquid)
            }
            Self::Lighter(handle) => handle.unsubscribe().await.map_err(ClientError::Lighter),
        }
    }
    async fn next(&mut self) -> Option<Result<Output, ClientError>> {
        match self {
            Self::Extended(h) => h.next().await.map(|r| {
                r.map_err(ClientError::Extended)
                    .and_then(|e| convert::output(e.0).map_err(|_| ClientError::Conversion))
            }),
            Self::Phoenix(h) => h.next().await.map(|r| {
                r.map_err(ClientError::Phoenix)
                    .and_then(|e| convert::output(e.0).map_err(|_| ClientError::Conversion))
            }),
            Self::Derive(handle) => Some(
                handle
                    .next()
                    .await?
                    .map_err(ClientError::Derive)
                    .and_then(|e| convert::output(e).map_err(|_| ClientError::Conversion)),
            ),
            Self::Paradex(handle) => Some(
                handle
                    .next()
                    .await?
                    .map_err(ClientError::Paradex)
                    .and_then(|e| convert::output(e).map_err(|_| ClientError::Conversion)),
            ),
            Self::Monaco(handle) => Some(
                handle
                    .next()
                    .await?
                    .map_err(ClientError::Monaco)
                    .and_then(|e| convert::output(e).map_err(|_| ClientError::Conversion)),
            ),
            Self::Qfex(handle) => Some(
                handle
                    .next()
                    .await?
                    .map_err(ClientError::Qfex)
                    .and_then(|e| convert::output(e).map_err(|_| ClientError::Conversion)),
            ),
            Self::Arcus(handle) => Some(
                handle
                    .next()
                    .await?
                    .map_err(ClientError::Arcus)
                    .and_then(|e| convert::output(e).map_err(|_| ClientError::Conversion)),
            ),
            Self::QfexTrade(handle) => Some(
                handle
                    .next()
                    .await?
                    .map_err(ClientError::Qfex)
                    .and_then(|e| convert::output(e).map_err(|_| ClientError::Conversion)),
            ),
            Self::Hyperliquid { handle, book_only } => loop {
                let event = match handle.next().await? {
                    Ok(e) => e,
                    Err(e) => return Some(Err(ClientError::Hyperliquid(e))),
                };
                let value = if *book_only {
                    match event {
                        hl::Event::L2Book(book) => convert::output(book),
                        hl::Event::SubscriptionResponse(_) | hl::Event::Pong => continue,
                        _ => return Some(Err(ClientError::Conversion)),
                    }
                } else {
                    let (channel, data) = match event {
                        hl::Event::L2Book(v) => ("l2Book", convert::output(v)),
                        hl::Event::Trades(v) => ("trades", convert::output(v)),
                        hl::Event::OrderUpdates(v) => ("orderUpdates", convert::output(v)),
                        hl::Event::UserFills(v) => ("userFills", convert::output(v)),
                        hl::Event::ClearinghouseState(v) => {
                            ("clearinghouseState", convert::output(v))
                        }
                        hl::Event::OpenOrders(v) => ("openOrders", convert::output(v)),
                        hl::Event::SubscriptionResponse(v) => {
                            ("subscriptionResponse", convert::output(v))
                        }
                        hl::Event::Pong => continue,
                        hl::Event::Unknown { channel, data } => {
                            return Some(
                                convert::output(Envelope { channel, data })
                                    .map_err(|_| ClientError::Conversion),
                            );
                        }
                        _ => return Some(Err(ClientError::Conversion)),
                    };
                    data.and_then(|data| convert::output(Envelope { channel, data }))
                };
                return Some(value.map_err(|_| ClientError::Conversion));
            },
            Self::Lighter(handle) => {
                let event = match handle.next().await? {
                    Ok(e) => e,
                    Err(e) => return Some(Err(ClientError::Lighter(e))),
                };
                let value = match event {
                    lt::Event::OrderBook(v) => convert::output(v),
                    lt::Event::Trades(v) => convert::output(v),
                    lt::Event::Account(v) => convert::output(v),
                    lt::Event::AccountOrders(v) => convert::output(v),
                    lt::Event::Shutdown(v) | lt::Event::Unknown(v) => convert::output(v),
                    _ => return Some(Err(ClientError::Conversion)),
                };
                Some(value.map_err(|_| ClientError::Conversion))
            }
        }
    }
}
type CloseResult = Result<(), Arc<ClientError>>;

#[derive(Clone)]
enum Phase {
    Open,
    Closing,
    Closed(CloseResult),
}

pub(crate) struct SubscriptionState {
    handle: Mutex<Option<NativeHandle>>,
    phase: watch::Sender<Phase>,
}

impl SubscriptionState {
    pub fn new(handle: NativeHandle) -> Arc<Self> {
        Arc::new(Self {
            handle: Mutex::new(Some(handle)),
            phase: watch::channel(Phase::Open).0,
        })
    }
    pub fn is_closed(&self) -> bool {
        !matches!(*self.phase.borrow(), Phase::Open)
    }

    pub fn request_close(self: &Arc<Self>) {
        let handle = {
            let mut slot = self.handle.lock().unwrap_or_else(|e| e.into_inner());
            if matches!(*self.phase.borrow(), Phase::Closed(_)) {
                return;
            }
            self.phase.send_replace(Phase::Closing);
            slot.take()
        };
        if let Some(handle) = handle {
            self.finish_close(handle);
        }
    }

    fn finish_close(self: &Arc<Self>, mut handle: NativeHandle) {
        self.phase.send_replace(Phase::Closing);
        let state = self.clone();
        // Cleanup outlives the consumer and runs independently on the native runtime.
        napi::bindgen_prelude::spawn(async move {
            let result = handle.close().await.map_err(Arc::new);
            state.phase.send_replace(Phase::Closed(result));
        });
    }

    pub async fn wait_closed(&self) -> Result<()> {
        let mut phase = self.phase.subscribe();
        loop {
            if let Phase::Closed(result) = phase.borrow().clone() {
                return result.map_err(|error| errors::map_error(&error));
            }
            if phase.changed().await.is_err() {
                return Ok(());
            }
        }
    }

    fn lease(self: &Arc<Self>) -> Result<Option<HandleLease>> {
        let mut slot = self.handle.lock().unwrap_or_else(|e| e.into_inner());
        if self.is_closed() {
            return Ok(None);
        }
        let handle = slot.take().ok_or_else(|| {
            errors::BusyError::new_err("only one pending read is allowed per subscription")
        })?;
        Ok(Some(HandleLease {
            handle: Some(handle),
            state: self.clone(),
        }))
    }
}

/// Own the native receiver across await without holding a lock. Dropping a
/// cancelled read schedules cleanup, including cancellation before first poll.
struct HandleLease {
    handle: Option<NativeHandle>,
    state: Arc<SubscriptionState>,
}
impl HandleLease {
    fn restore(mut self) -> bool {
        let mut slot = self.state.handle.lock().unwrap_or_else(|e| e.into_inner());
        if self.state.is_closed() {
            return false;
        }
        *slot = self.handle.take();
        true
    }
    fn close(mut self) {
        if let Some(handle) = self.handle.take() {
            self.state.finish_close(handle);
        }
    }
}
impl Drop for HandleLease {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            self.state.finish_close(handle);
        }
    }
}

pub struct Subscription {
    pub(crate) state: Arc<SubscriptionState>,
}
impl Subscription {
    pub fn closed(&self) -> bool {
        self.state.is_closed()
    }
    pub async fn next(&self) -> Result<Option<crate::convert::Output>> {
        let Some(mut lease) = self.state.lease()? else {
            return Ok(None);
        };
        let mut phase = self.state.phase.subscribe();
        if self.state.is_closed() {
            lease.close();
            self.state.wait_closed().await?;
            return Ok(None);
        }
        let event = tokio::select! {biased;_=phase.changed()=>{lease.close();self.state.wait_closed().await?;return Ok(None)},event=lease.handle.as_mut().expect("lease owns handle").next()=>event};
        match event {
            Some(Ok(value)) => {
                let value = convert::output(value)?;
                if !lease.restore() {
                    self.state.wait_closed().await?;
                    return Ok(None);
                }
                Ok(Some(value))
            }
            Some(Err(error)) if error.is_gap() => {
                let mapped = errors::map_error(&error);
                if !lease.restore() {
                    self.state.wait_closed().await?;
                    return Ok(None);
                }
                Err(mapped)
            }
            Some(Err(error)) => {
                lease.close();
                let _ = self.state.wait_closed().await;
                Err(errors::map_error(&error))
            }
            None => {
                lease.close();
                self.state.wait_closed().await?;
                Ok(None)
            }
        }
    }
    pub async fn close(&self) -> Result<()> {
        self.state.request_close();
        self.state.wait_closed().await
    }
}
impl Drop for Subscription {
    fn drop(&mut self) {
        self.state.request_close();
    }
}

#[derive(serde::Serialize)]
struct Envelope<C, D> {
    channel: C,
    data: D,
}
