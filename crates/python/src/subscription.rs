use crate::{
    convert::to_python,
    errors::{self, ClientError},
};
use aux_rs::{hyperliquid as hl, lighter as lt, standard::StandardSubscription};
use pyo3::{exceptions::PyStopAsyncIteration, prelude::*};
use std::sync::{Arc, Mutex};
use tokio::sync::watch;

pub enum NativeHandle {
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
            Self::Qfex(handle) => handle.close().await.map_err(ClientError::Qfex),
            Self::Arcus(handle) => handle.close().await.map_err(ClientError::Arcus),
            Self::QfexTrade(handle) => handle.close().await.map_err(ClientError::Qfex),
            Self::Hyperliquid { handle, .. } => {
                handle.close().await.map_err(ClientError::Hyperliquid)
            }
            Self::Lighter(handle) => handle.unsubscribe().await.map_err(ClientError::Lighter),
        }
    }
    async fn next(&mut self) -> Option<Result<serde_json::Value, ClientError>> {
        use serde_json::json;
        match self {
            Self::Qfex(handle) => Some(
                handle
                    .next()
                    .await?
                    .map_err(ClientError::Qfex)
                    .and_then(|e| serde_json::to_value(e).map_err(|_| ClientError::Conversion)),
            ),
            Self::Arcus(handle) => Some(
                handle
                    .next()
                    .await?
                    .map_err(ClientError::Arcus)
                    .and_then(|e| serde_json::to_value(e).map_err(|_| ClientError::Conversion)),
            ),
            Self::QfexTrade(handle) => Some(
                handle
                    .next()
                    .await?
                    .map_err(ClientError::Qfex)
                    .and_then(|e| serde_json::to_value(e).map_err(|_| ClientError::Conversion)),
            ),
            Self::Hyperliquid { handle, book_only } => loop {
                let event = match handle.next().await? {
                    Ok(e) => e,
                    Err(e) => return Some(Err(ClientError::Hyperliquid(e))),
                };
                let value = if *book_only {
                    match event {
                        hl::Event::L2Book(book) => serde_json::to_value(book),
                        hl::Event::SubscriptionResponse(_) | hl::Event::Pong => continue,
                        _ => return Some(Err(ClientError::Conversion)),
                    }
                } else {
                    let (channel, data) = match event {
                        hl::Event::L2Book(v) => ("l2Book", serde_json::to_value(v)),
                        hl::Event::Trades(v) => ("trades", serde_json::to_value(v)),
                        hl::Event::OrderUpdates(v) => ("orderUpdates", serde_json::to_value(v)),
                        hl::Event::UserFills(v) => ("userFills", serde_json::to_value(v)),
                        hl::Event::ClearinghouseState(v) => {
                            ("clearinghouseState", serde_json::to_value(v))
                        }
                        hl::Event::OpenOrders(v) => ("openOrders", serde_json::to_value(v)),
                        hl::Event::SubscriptionResponse(v) => {
                            ("subscriptionResponse", serde_json::to_value(v))
                        }
                        hl::Event::Pong => continue,
                        hl::Event::Unknown { channel, data } => {
                            return Some(Ok(json!({"channel":channel,"data":data})));
                        }
                        _ => return Some(Err(ClientError::Conversion)),
                    };
                    data.map(|data| json!({"channel":channel,"data":data}))
                };
                return Some(value.map_err(|_| ClientError::Conversion));
            },
            Self::Lighter(handle) => {
                let event = match handle.next().await? {
                    Ok(e) => e,
                    Err(e) => return Some(Err(ClientError::Lighter(e))),
                };
                let value = match event {
                    lt::Event::OrderBook(v) => serde_json::to_value(v),
                    lt::Event::Trades(v) => serde_json::to_value(v),
                    lt::Event::Account(v) => serde_json::to_value(v),
                    lt::Event::AccountOrders(v) => serde_json::to_value(v),
                    lt::Event::Shutdown(v) | lt::Event::Unknown(v) => Ok(v),
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
        // Cleanup outlives a cancelled Python future. No interpreter access is
        // needed here, including when called during Python object destruction.
        pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
            let result = handle.close().await.map_err(Arc::new);
            state.phase.send_replace(Phase::Closed(result));
        });
    }

    pub async fn wait_closed(&self) -> PyResult<()> {
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

    fn lease(self: &Arc<Self>) -> PyResult<Option<HandleLease>> {
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

#[pyclass(name = "Subscription", module = "aux_py._native", frozen)]
pub struct Subscription {
    pub(crate) state: Arc<SubscriptionState>,
}

#[pymethods]
impl Subscription {
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }

    fn next<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let lease = self.state.lease()?;
        let state = self.state.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let Some(mut lease) = lease else {
                return Err(PyStopAsyncIteration::new_err(()));
            };
            let mut phase = state.phase.subscribe();
            {
                if state.is_closed() {
                    lease.close();
                    state.wait_closed().await?;
                    return Err(PyStopAsyncIteration::new_err(()));
                }
                let event = tokio::select! {
                    biased;
                    _ = phase.changed() => {
                        lease.close(); state.wait_closed().await?;
                        return Err(PyStopAsyncIteration::new_err(()));
                    }
                    event = lease.handle.as_mut().expect("lease owns handle").next() => event,
                };
                match event {
                    Some(Ok(value)) => {
                        let result = Python::attach(|py| to_python(py, value))?;
                        if !lease.restore() {
                            state.wait_closed().await?;
                            return Err(PyStopAsyncIteration::new_err(()));
                        }
                        Ok(result)
                    }
                    Some(Err(error)) => {
                        if error.is_gap() {
                            let mapped = errors::map_error(&error);
                            if !lease.restore() {
                                state.wait_closed().await?;
                                return Err(PyStopAsyncIteration::new_err(()));
                            }
                            return Err(mapped);
                        }
                        lease.close();
                        let _ = state.wait_closed().await;
                        Err(errors::map_error(&error))
                    }
                    None => {
                        lease.close();
                        state.wait_closed().await?;
                        Err(PyStopAsyncIteration::new_err(()))
                    }
                }
            }
        })
    }

    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.request_close();
        let state = self.state.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move { state.wait_closed().await })
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
