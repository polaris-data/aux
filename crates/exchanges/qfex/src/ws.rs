use crate::*;
use aux_rs_core::{
    Error as ClientError, Exchange, WebSocketConnection, WebSocketConnector, WebSocketStream,
    WebSocketSubscription, WebSocketTransport,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    #[serde(rename = "level2")]
    OrderBook,
    #[serde(rename = "trade")]
    Trades,
    Bbo,
    MarkPrice,
    FundingRate,
    OpenInterest,
    MarketStats,
    Refdata,
}
impl Channel {
    fn name(&self) -> &'static str {
        match self {
            Self::OrderBook => "level2",
            Self::Trades => "trade",
            Self::Bbo => "bbo",
            Self::MarkPrice => "mark_price",
            Self::FundingRate => "funding_rate",
            Self::OpenInterest => "open_interest",
            Self::MarketStats => "market_stats",
            Self::Refdata => "refdata",
        }
    }
}
#[derive(Clone, Debug)]
pub struct Subscription {
    pub channel: Channel,
    pub symbol: String,
}
impl Subscription {
    pub fn new(channel: Channel, symbol: impl Into<String>) -> Self {
        Self {
            channel,
            symbol: symbol.into(),
        }
    }
    fn wire(&self, kind: &str) -> String {
        json!({"type":kind,"channels":[self.channel.name()],"symbols":[self.symbol]}).to_string()
    }
}
impl WebSocketSubscription for Subscription {
    fn key(&self) -> String {
        self.wire("subscribe")
    }
    fn subscribe_message(&self) -> String {
        self.wire("subscribe")
    }
    fn unsubscribe_message(&self) -> String {
        self.wire("unsubscribe")
    }
    fn matches(&self, text: &str) -> bool {
        let Ok(v) = serde_json::from_str::<Value>(text) else {
            return true;
        };
        if v.get("error_code").is_some() || v.get("error").is_some() {
            return true;
        }
        if let Some(symbol) = v.get("symbol").and_then(Value::as_str) {
            return (self.symbol == "*" || symbol == self.symbol)
                && v["type"] == self.channel.name();
        }
        if let Some(channels) = v.get("channels").and_then(Value::as_array) {
            return channels.iter().any(|c| c == self.channel.name())
                && v.get("symbols")
                    .and_then(Value::as_array)
                    .is_none_or(|xs| xs.iter().any(|s| s == &self.symbol || s == "*"));
        }
        v["type"] == self.channel.name() || v["channel"] == self.channel.name()
    }
    fn is_data_message(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| v["type"] == self.channel.name())
    }
}
/// Native envelopes are preserved, including sequence IDs and unknown fields.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "type", default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    #[serde(flatten)]
    pub data: ExtraFields,
}
impl Event {
    pub fn decode(text: &str) -> Result<Self, Error> {
        let v: Value = serde_json::from_str(text)?;
        if v.get("error_code").is_some()
            || v.get("error").is_some()
            || v.get("err_response").is_some()
        {
            return Err(Error::Api {
                status: None,
                body: v,
            });
        }
        serde_json::from_value(v).map_err(Error::Decode)
    }
    /// Decode either the current top-level payload or legacy contents-array payloads.
    pub fn payloads<T: DeserializeOwned>(&self) -> Result<Vec<T>, Error> {
        if let Some(v) = self.data.get("contents") {
            if v.is_array() {
                serde_json::from_value(v.clone()).map_err(Error::Decode)
            } else {
                Ok(vec![serde_json::from_value(v.clone())?])
            }
        } else {
            Ok(vec![serde_json::from_value(serde_json::to_value(self)?)?])
        }
    }
}
pub struct SubscriptionHandle<S> {
    stream: S,
}
impl<S: WebSocketStream> SubscriptionHandle<S> {
    pub async fn next(&mut self) -> Option<Result<Event, ClientError<Error>>> {
        self.stream.next().await.map(|r| {
            r.map_err(ClientError::Transport)
                .and_then(|t| Event::decode(&t).map_err(ClientError::Exchange))
        })
    }
    pub async fn close(&mut self) -> Result<(), ClientError<Error>> {
        self.stream.close().await.map_err(ClientError::Transport)
    }
    pub async fn unsubscribe(&mut self) -> Result<(), ClientError<Error>> {
        self.close().await
    }
}
impl<T: WebSocketTransport, A: Sync> QfexClient<T, A> {
    pub async fn subscribe(
        &self,
        s: Subscription,
    ) -> Result<SubscriptionHandle<T::Stream>, ClientError<Error>> {
        nonempty(&s.symbol).map_err(ClientError::Exchange)?;
        Ok(SubscriptionHandle {
            stream: self.transport.subscribe(self.exchange.ws_url(), s).await?,
        })
    }
}
#[derive(Clone)]
enum Pending {
    Order(String),
    UserOrders,
}
impl Pending {
    fn response<'a>(&self, event: &'a Event) -> Option<&'a Value> {
        match self {
            Self::Order(id) => event
                .data
                .get("order_response")
                .filter(|r| r["client_order_id"] == *id || r["order_id"] == *id),
            Self::UserOrders => event.data.get("all_orders_response"),
        }
    }
}
/// Authenticated, isolated trade connection. Mutating commands are sent once;
/// cancellation or an ambiguous response prevents subsequent commands until the
/// pending native order response has been consumed with `next`.
pub struct TradingSession<C> {
    connection: C,
    events: VecDeque<Event>,
    pending: Option<Pending>,
    timeout: Duration,
}
impl<C: WebSocketConnection> TradingSession<C> {
    pub async fn next(&mut self) -> Option<Result<Event, ClientError<Error>>> {
        if let Some(e) = self.events.pop_front() {
            return Some(Ok(e));
        }
        let r = self.connection.next().await?;
        Some(
            r.map_err(ClientError::Transport)
                .and_then(|text| Event::decode(&text).map_err(ClientError::Exchange))
                .inspect(|e| {
                    if self
                        .pending
                        .as_ref()
                        .is_some_and(|id| id.response(e).is_some())
                    {
                        self.pending = None;
                    }
                }),
        )
    }
    pub async fn close(&mut self) -> Result<(), ClientError<Error>> {
        self.connection
            .close()
            .await
            .map_err(ClientError::Transport)
    }
    pub async fn subscribe_account(&mut self) -> Result<(), ClientError<Error>> {
        self.connection
            .send(
                json!({"type":"subscribe","params":{"channels":["balances","positions","fills"]}})
                    .to_string(),
            )
            .await?;
        Ok(())
    }
    pub async fn add_order(
        &mut self,
        order: AddOrder,
    ) -> Result<OrderResponse, ClientError<Error>> {
        order.validate().map_err(ClientError::Exchange)?;
        let id = order.client_order_id.clone();
        self.command(
            json!({"type":"add_order","params":order}),
            Pending::Order(id),
        )
        .await
    }
    pub async fn cancel_order(
        &mut self,
        order: CancelOrder,
    ) -> Result<OrderResponse, ClientError<Error>> {
        nonempty(&order.symbol).map_err(ClientError::Exchange)?;
        nonempty(&order.order_id).map_err(ClientError::Exchange)?;
        let id = order.order_id.clone();
        self.command(
            json!({"type":"cancel_order","params":order}),
            Pending::Order(id),
        )
        .await
    }
    pub async fn user_orders(
        &mut self,
        request: UserOrdersRequest,
    ) -> Result<UserOrders, ClientError<Error>> {
        request.validate().map_err(ClientError::Exchange)?;
        self.command(
            json!({"type":"get_user_orders","params":request}),
            Pending::UserOrders,
        )
        .await
    }
    async fn command<R: DeserializeOwned>(
        &mut self,
        wire: Value,
        id: Pending,
    ) -> Result<R, ClientError<Error>> {
        if self.pending.is_some() {
            return Err(ClientError::Exchange(Error::PendingOutcome));
        }
        self.pending = Some(id.clone());
        self.connection.send(wire.to_string()).await?;
        let start = Instant::now();
        loop {
            let remaining = self.timeout.saturating_sub(start.elapsed());
            let text = self
                .connection
                .next_with_timeout(remaining)
                .await
                .ok_or(ClientError::Exchange(Error::PendingOutcome))??;
            let event = Event::decode(&text).map_err(ClientError::Exchange)?;
            if let Some(response) = id.response(&event) {
                self.pending = None;
                return serde_json::from_value(response.clone())
                    .map_err(|e| ClientError::Exchange(Error::Decode(e)));
            }
            if self.events.len() >= 128 {
                self.events.clear();
                let _ = self.connection.close().await;
                return Err(ClientError::Transport(aux_rs_core::TransportError::new(
                    aux_rs_core::TransportErrorKind::Overflow,
                    aux_rs_core::RequestOutcome::Unknown,
                    "trade-session event buffer overflowed; reconcile account state and command outcome",
                )));
            }
            self.events.push_back(event);
        }
    }
}
impl<T: WebSocketConnector, A: AuthHandler> QfexClient<T, A> {
    /// One isolated authenticated session per command. For sustained trading,
    /// explicitly retain a `trading_session` and reconcile ambiguous outcomes.
    pub async fn add_order(&self, order: AddOrder) -> Result<OrderResponse, ClientError<Error>> {
        order.validate().map_err(ClientError::Exchange)?;
        let mut session = self.trading_session().await?;
        let result = session.add_order(order).await;
        let _ = session.close().await;
        result
    }
    pub async fn cancel_order(
        &self,
        order: CancelOrder,
    ) -> Result<OrderResponse, ClientError<Error>> {
        nonempty(&order.symbol).map_err(ClientError::Exchange)?;
        nonempty(&order.order_id).map_err(ClientError::Exchange)?;
        let mut session = self.trading_session().await?;
        let result = session.cancel_order(order).await;
        let _ = session.close().await;
        result
    }
    pub async fn user_orders(
        &self,
        request: UserOrdersRequest,
    ) -> Result<UserOrders, ClientError<Error>> {
        request.validate().map_err(ClientError::Exchange)?;
        let mut session = self.trading_session().await?;
        let result = session.user_orders(request).await;
        let _ = session.close().await;
        result
    }
    pub async fn trading_session(
        &self,
    ) -> Result<TradingSession<T::Connection>, ClientError<Error>> {
        let url = self
            .auth
            .trade_url(&self.exchange)
            .map_err(ClientError::Exchange)?;
        let mut connection = self.transport.connect(&url).await?;
        let auth = self
            .auth
            .authenticate()
            .await
            .map_err(ClientError::Exchange)?;
        connection.send(auth.to_string()).await?;
        let started = Instant::now();
        loop {
            let text = connection
                .next_with_timeout(self.timeout.saturating_sub(started.elapsed()))
                .await
                .ok_or(ClientError::Exchange(Error::Signing(
                    "authentication connection ended",
                )))??;
            let v: Value =
                serde_json::from_str(&text).map_err(|e| ClientError::Exchange(Error::Decode(e)))?;
            if v["authenticated"] == true {
                break;
            }
            if v.get("error_code").is_some()
                || v.get("err_response").is_some()
                || v["authenticated"] == false
            {
                return Err(ClientError::Exchange(Error::Api {
                    status: None,
                    body: v,
                }));
            }
        }
        connection
            .send(json!({"type":"subscribe","params":{"channels":["order_responses"]}}).to_string())
            .await?;
        Ok(TradingSession {
            connection,
            events: VecDeque::new(),
            pending: None,
            timeout: self.timeout,
        })
    }
}
