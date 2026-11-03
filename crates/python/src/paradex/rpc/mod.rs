use super::*;
use aux_rs::standard::StandardConnection;
use tokio::sync::{Mutex, OwnedMutexGuard};
mod operations;
type NativeSession = v::RpcSession<StandardConnection, v::ParadexAuth<Signer>>;
#[pyclass(module = "aux_py._native")]
pub struct ParadexRpcSession {
    pub(super) inner: Arc<Mutex<NativeSession>>,
    pub(super) state: Arc<ClientState<Resources>>,
    pub(super) closing: tokio::sync::watch::Sender<bool>,
}
impl ParadexRpcSession {
    fn run<'py, F, T>(&self, py: Python<'py>, future: F) -> PyResult<Bound<'py, PyAny>>
    where
        F: std::future::Future<Output = PyResult<T>> + Send + 'static,
        T: serde::Serialize + Send + 'static,
    {
        let mut closing = self.closing.subscribe();
        self.state.run(py, async move {
            if *closing.borrow() {return Err(errors::ClosedError::new_err("RPC session is closed"));}
            tokio::select! {biased; _=closing.changed()=>Err(errors::ClosedError::new_err("RPC session is closed")),result=future=>result}
        })
    }
    pub(super) fn acquire(&self) -> PyResult<OwnedMutexGuard<NativeSession>> {
        if *self.closing.borrow() {
            return Err(errors::ClosedError::new_err("RPC session is closed"));
        }
        self.inner.clone().try_lock_owned().map_err(|_| {
            errors::BusyError::new_err(
                "one reader or command is already active on this RPC session",
            )
        })
    }
}
#[pymethods]
impl ParadexRpcSession {
    #[getter]
    fn pending(&self) -> PyResult<Py<PyAny>> {
        let guard = self.acquire()?;
        convert::output(
            guard
                .pending()
                .map(|p| serde_json::json!({"id":p.id,"method":p.method})),
        )
    }
    fn reconcile<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let mut guard = self.acquire()?;
        self.run(py, async move { guard.reconcile().await.map_err(native) })
    }
    fn next_event<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let mut guard = self.acquire()?;
        self.run(py, async move {
            guard.next_event().await.transpose().map_err(native)
        })
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.closing.send_replace(true);
        let inner = self.inner.clone();
        let cleanup = pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
            let mut guard = inner.lock_owned().await;
            guard.close().await.map_err(native)
        });
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            cleanup
                .await
                .map_err(|_| errors::ConnectorError::new_err("RPC cleanup failed"))?
        })
    }
    fn __repr__(&self) -> &'static str {
        "ParadexRpcSession()"
    }
}
impl Drop for ParadexRpcSession {
    fn drop(&mut self) {
        self.closing.send_replace(true);
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
            let mut guard = inner.lock_owned().await;
            let _ = async { guard.close().await.map_err(native) }.await;
        });
    }
}
