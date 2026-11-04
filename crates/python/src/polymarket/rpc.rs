use super::*;
use tokio::sync::{Mutex, OwnedMutexGuard};
type NativeSession = v::RpcSession<aux_rs::standard::StandardConnection, Auth>;
#[pyclass(module = "aux_py._native")]
pub struct PolymarketRpcSession {
    inner: Arc<Mutex<NativeSession>>,
    parent: tokio::sync::watch::Receiver<bool>,
    closing: tokio::sync::watch::Sender<bool>,
}
impl PolymarketRpcSession {
    fn acquire(&self) -> PyResult<OwnedMutexGuard<NativeSession>> {
        if *self.closing.borrow() || *self.parent.borrow() {
            return Err(errors::ClosedError::new_err("command session is closed"));
        }
        self.inner
            .clone()
            .try_lock_owned()
            .map_err(|_| errors::BusyError::new_err("one command or reader is already active"))
    }
    fn run<'py, F, T>(&self, py: Python<'py>, future: F) -> PyResult<Bound<'py, PyAny>>
    where
        F: std::future::Future<Output = PyResult<T>> + Send + 'static,
        T: serde::Serialize + Send + 'static,
    {
        let mut closing = self.closing.subscribe();
        let mut parent = self.parent.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            if *closing.borrow() || *parent.borrow() {
                return Err(errors::ClosedError::new_err("command session is closed"));
            }
            let value = tokio::select! {biased;_=closing.changed()=>return Err(errors::ClosedError::new_err("command session is closed")),_=parent.changed()=>return Err(errors::ClosedError::new_err("client is closed")),r=future=>r?};
            convert::output(value)
        })
    }
}
#[pymethods]
impl PolymarketRpcSession {
    #[getter]
    fn pending(&self) -> PyResult<Py<PyAny>> {
        convert::output(self.acquire()?.pending())
    }
    fn reconcile<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let mut g = self.acquire()?;
        self.run(py, async move { g.reconcile().await.map_err(native) })
    }
    fn next_event<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let mut g = self.acquire()?;
        self.run(py, async move {
            g.next_event().await.transpose().map_err(native)
        })
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.closing.send_replace(true);
        let inner = self.inner.clone();
        let task = pyo3_async_runtimes::tokio::get_runtime()
            .spawn(async move { inner.lock_owned().await.close().await.map_err(native) });
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            task.await
                .map_err(|_| errors::ConnectorError::new_err("command cleanup failed"))?
        })
    }
    fn create_orders<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Value = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(
            py,
            async move { g.create_orders(request).await.map_err(native) },
        )
    }
    fn modify_orders<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Value = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(
            py,
            async move { g.modify_orders(request).await.map_err(native) },
        )
    }
    fn modify_orders_coid<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Value = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(py, async move {
            g.modify_orders_coid(request).await.map_err(native)
        })
    }
    fn cancel_orders<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Value = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(
            py,
            async move { g.cancel_orders(request).await.map_err(native) },
        )
    }
    fn cancel_orders_coid<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Value = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(py, async move {
            g.cancel_orders_coid(request).await.map_err(native)
        })
    }
    fn cancel_all<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Value = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(
            py,
            async move { g.cancel_all(request).await.map_err(native) },
        )
    }
    fn auto_cancel<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Value = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(
            py,
            async move { g.auto_cancel(request).await.map_err(native) },
        )
    }
    fn update_leverage<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Value = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(py, async move {
            g.update_leverage(request).await.map_err(native)
        })
    }
    fn update_leverages<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Value = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(py, async move {
            g.update_leverages(request).await.map_err(native)
        })
    }
    fn update_margin<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Value = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(
            py,
            async move { g.update_margin(request).await.map_err(native) },
        )
    }
    fn subscribe<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Vec<String> = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(
            py,
            async move { g.subscribe(request).await.map_err(native) },
        )
    }
    fn unsubscribe<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Vec<String> = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(
            py,
            async move { g.unsubscribe(request).await.map_err(native) },
        )
    }
    fn ping<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let mut g = self.acquire()?;
        self.run(py, async move { g.ping().await.map_err(native) })
    }
    fn quote<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Value = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(py, async move { g.quote(request).await.map_err(native) })
    }
    fn cancel_quote<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Value = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(
            py,
            async move { g.cancel_quote(request).await.map_err(native) },
        )
    }
    fn last_look<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: Value = convert::parse(&request)?;
        let mut g = self.acquire()?;
        self.run(
            py,
            async move { g.last_look(request).await.map_err(native) },
        )
    }
}
impl Drop for PolymarketRpcSession {
    fn drop(&mut self) {
        self.closing.send_replace(true);
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
            let _ = inner.lock_owned().await.close().await;
        });
    }
}
#[pymethods]
impl PolymarketClient {
    fn open_rpc<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?.scoped(py)?;
        let parent = self.state.closing_receiver();
        self.state.object(py, async move {
            let session = client.open_rpc().await.map_err(native)?;
            Python::attach(|py| {
                Py::new(
                    py,
                    PolymarketRpcSession {
                        inner: Arc::new(Mutex::new(session)),
                        parent,
                        closing: tokio::sync::watch::channel(false).0,
                    },
                )
                .map(|p| p.into_any())
            })
        })
    }
}
#[pymethods]
impl PolymarketPerpsClient {
    fn open_rpc<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?.scoped(py)?;
        let parent = self.state.closing_receiver();
        self.state.object(py, async move {
            let session = client.open_rpc().await.map_err(native)?;
            Python::attach(|py| {
                Py::new(
                    py,
                    PolymarketRpcSession {
                        inner: Arc::new(Mutex::new(session)),
                        parent,
                        closing: tokio::sync::watch::channel(false).0,
                    },
                )
                .map(|p| p.into_any())
            })
        })
    }
}
