use super::*;
impl ParadexRpcSession {
    pub(crate) async fn create_order(&self, request: Input) -> Result<Output> {
        let request: v::RequestsOrderRequest = convert::parse(&request)?;
        let mut session = self.acquire()?;
        self.run(async move { session.create_order(request).await.map_err(native) })
            .await
    }
    pub(crate) async fn create_batch(&self, requests: Input) -> Result<Output> {
        let requests: Vec<v::RequestsOrderRequest> = convert::parse(&requests)?;
        let mut session = self.acquire()?;
        self.run(async move { session.create_batch(requests).await.map_err(native) })
            .await
    }
    pub(crate) async fn modify_order(&self, order_id: String, request: Input) -> Result<Output> {
        let request: v::RequestsModifyOrderRequest = convert::parse(&request)?;
        let mut session = self.acquire()?;
        self.run(async move {
            session
                .modify_order(&order_id, request)
                .await
                .map_err(native)
        })
        .await
    }
    pub(crate) async fn cancel_order(&self, order_id: String) -> Result<Output> {
        let mut session = self.acquire()?;
        self.run(async move { session.cancel_order(&order_id).await.map_err(native) })
            .await
    }
    pub(crate) async fn cancel_by_client_id(
        &self,
        client_id: String,
        market: String,
    ) -> Result<Output> {
        let mut session = self.acquire()?;
        self.run(async move {
            session
                .cancel_by_client_id(&client_id, &market)
                .await
                .map_err(native)
        })
        .await
    }
    pub(crate) async fn cancel_all(&self, market: Option<String>) -> Result<Output> {
        let mut session = self.acquire()?;
        self.run(async move { session.cancel_all(market.as_deref()).await.map_err(native) })
            .await
    }
    pub(crate) async fn cancel_batch(&self, order_ids: Vec<String>) -> Result<Output> {
        let mut session = self.acquire()?;
        self.run(async move { session.cancel_batch(order_ids).await.map_err(native) })
            .await
    }
    pub(crate) async fn set_cancel_on_disconnect(&self, enabled: bool) -> Result<Output> {
        let mut session = self.acquire()?;
        self.run(async move {
            session
                .set_cancel_on_disconnect(enabled)
                .await
                .map_err(native)
        })
        .await
    }
    pub(crate) async fn modify_subscription(&self, request: Input) -> Result<Output> {
        let request: v::WsModifySubscriptionModifySubscriptionPublish = convert::parse(&request)?;
        let mut session = self.acquire()?;
        self.run(async move { session.modify_subscription(request).await.map_err(native) })
            .await
    }
    pub(crate) async fn subscribe_channel(&self, specification: Input) -> Result<Output> {
        let spec: v::Subscription = convert::parse(&specification)?;
        let mut session = self.acquire()?;
        self.run(async move { session.subscribe_channel(spec).await.map_err(native) })
            .await
    }
    pub(crate) async fn unsubscribe_channel(&self, specification: Input) -> Result<Output> {
        let spec: v::Subscription = convert::parse(&specification)?;
        let mut session = self.acquire()?;
        self.run(async move { session.unsubscribe_channel(spec).await.map_err(native) })
            .await
    }
}
