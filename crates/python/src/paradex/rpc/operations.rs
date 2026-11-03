use super::*;
#[pymethods]
impl ParadexRpcSession {
    fn create_order<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: v::RequestsOrderRequest = convert::parse(&request)?;
        let mut session = self.acquire()?;
        self.run(py, async move {
            session.create_order(request).await.map_err(native)
        })
    }
    fn create_batch<'py>(
        &self,
        py: Python<'py>,
        requests: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let requests: Vec<v::RequestsOrderRequest> = convert::parse(&requests)?;
        let mut session = self.acquire()?;
        self.run(py, async move {
            session.create_batch(requests).await.map_err(native)
        })
    }
    fn modify_order<'py>(
        &self,
        py: Python<'py>,
        order_id: String,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: v::RequestsModifyOrderRequest = convert::parse(&request)?;
        let mut session = self.acquire()?;
        self.run(py, async move {
            session
                .modify_order(&order_id, request)
                .await
                .map_err(native)
        })
    }
    fn cancel_order<'py>(&self, py: Python<'py>, order_id: String) -> PyResult<Bound<'py, PyAny>> {
        let mut session = self.acquire()?;
        self.run(py, async move {
            session.cancel_order(&order_id).await.map_err(native)
        })
    }
    fn cancel_by_client_id<'py>(
        &self,
        py: Python<'py>,
        client_id: String,
        market: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let mut session = self.acquire()?;
        self.run(py, async move {
            session
                .cancel_by_client_id(&client_id, &market)
                .await
                .map_err(native)
        })
    }
    #[pyo3(signature=(market=None))]
    fn cancel_all<'py>(
        &self,
        py: Python<'py>,
        market: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let mut session = self.acquire()?;
        self.run(py, async move {
            session.cancel_all(market.as_deref()).await.map_err(native)
        })
    }
    fn cancel_batch<'py>(
        &self,
        py: Python<'py>,
        order_ids: Vec<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let mut session = self.acquire()?;
        self.run(py, async move {
            session.cancel_batch(order_ids).await.map_err(native)
        })
    }
    fn set_cancel_on_disconnect<'py>(
        &self,
        py: Python<'py>,
        enabled: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let mut session = self.acquire()?;
        self.run(py, async move {
            session
                .set_cancel_on_disconnect(enabled)
                .await
                .map_err(native)
        })
    }
    fn modify_subscription<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: v::WsModifySubscriptionModifySubscriptionPublish = convert::parse(&request)?;
        let mut session = self.acquire()?;
        self.run(py, async move {
            session.modify_subscription(request).await.map_err(native)
        })
    }
    fn subscribe_channel<'py>(
        &self,
        py: Python<'py>,
        specification: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let spec: v::Subscription = convert::parse(&specification)?;
        let mut session = self.acquire()?;
        self.run(py, async move {
            session.subscribe_channel(spec).await.map_err(native)
        })
    }
    fn unsubscribe_channel<'py>(
        &self,
        py: Python<'py>,
        specification: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let spec: v::Subscription = convert::parse(&specification)?;
        let mut session = self.acquire()?;
        self.run(py, async move {
            session.unsubscribe_channel(spec).await.map_err(native)
        })
    }
}
