use crate::*;
use aux_rs_core::{
    Error as ClientError, RequestOutcome, TransportErrorKind, WebSocketConnection,
    WebSocketConnector,
};
use serde_json::{Value, json};
use std::{collections::VecDeque, time::Duration};
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct PendingCommand {
    pub id: u64,
    pub method: String,
    #[serde(default)]
    pub correlation: Option<String>,
    #[serde(default)]
    pub quote_id: Option<String>,
}
pub struct RpcSession<C, A> {
    connection: C,
    exchange: Polymarket,
    auth: A,
    _service: String,
    next_id: u64,
    pending: Option<PendingCommand>,
    events: VecDeque<Value>,
    capacity: usize,
    timeout: Duration,
}
impl<C, A> std::fmt::Debug for RpcSession<C, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RpcSession")
            .field("pending", &self.pending.as_ref().map(|p| p.id))
            .finish_non_exhaustive()
    }
}
impl<C, A> RpcSession<C, A> {
    pub fn pending(&self) -> Option<&PendingCommand> {
        self.pending.as_ref()
    }
    pub fn with_limits(mut self, capacity: usize, timeout: Duration) -> Result<Self, Error> {
        if capacity == 0 || timeout.is_zero() {
            return Err(Error::InvalidRequest(
                "positive RPC capacity/deadline required",
            ));
        }
        self.capacity = capacity;
        self.timeout = timeout;
        Ok(self)
    }
}
impl<C: WebSocketConnection, A> RpcSession<C, A> {
    pub async fn close(&mut self) -> Result<(), ClientError<Error>> {
        self.connection.close().await.map_err(Into::into)
    }
    pub async fn next_event(&mut self) -> Option<Result<Value, ClientError<Error>>> {
        if let Some(v) = self.events.pop_front() {
            return Some(Ok(v));
        }
        if self.pending.is_some() {
            return Some(Err(ClientError::Exchange(Error::PendingOutcome)));
        }
        let r = self.connection.next().await?;
        Some(
            r.map_err(Into::into).and_then(|s| {
                serde_json::from_str(&s).map_err(|e| ClientError::Exchange(e.into()))
            }),
        )
    }
    pub async fn reconcile(&mut self) -> Result<Value, ClientError<Error>> {
        let id = self
            .pending
            .as_ref()
            .ok_or(ClientError::Exchange(Error::InvalidRequest(
                "no pending command",
            )))?
            .id;
        let started = std::time::Instant::now();
        loop {
            let budget = self.timeout.checked_sub(started.elapsed()).ok_or_else(|| {
                ClientError::Transport(aux_rs_core::TransportError::new(
                    TransportErrorKind::Timeout,
                    RequestOutcome::Unknown,
                    "native command response timed out",
                ))
            })?;
            let text = self
                .connection
                .next_with_timeout(budget)
                .await
                .ok_or_else(|| {
                    ClientError::Transport(aux_rs_core::TransportError::new(
                        TransportErrorKind::Disconnected,
                        RequestOutcome::Unknown,
                        "native command connection closed",
                    ))
                })??;
            let v: Value =
                serde_json::from_str(&text).map_err(|e| ClientError::Exchange(e.into()))?;
            let matches = if self._service == "rfq" {
                let pending = self.pending.as_ref().expect("pending command");
                if pending.method == "auth" {
                    v["type"] == "auth"
                } else {
                    let native = v["type"].as_str().unwrap_or("");
                    let expected = format!("ACK_{}", pending.method);
                    let correlation = v["rfq_id"].as_str() == pending.correlation.as_deref();
                    let quote = pending
                        .quote_id
                        .as_ref()
                        .is_none_or(|id| v["quote_id"].as_str() == Some(id.as_str()));
                    correlation
                        && (native == expected && quote
                            || native == "RFQ_ERROR" && v["request_type"] == pending.method)
                }
            } else {
                v.get("id").and_then(|v| {
                    v.as_u64()
                        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
                }) == Some(id)
            };
            if matches {
                self.pending = None;
                return Ok(v);
            }
            if self.events.len() >= self.capacity {
                self.connection.close().await?;
                return Err(ClientError::Transport(aux_rs_core::TransportError::new(
                    TransportErrorKind::Overflow,
                    RequestOutcome::Unknown,
                    "native command event buffer overflowed",
                )));
            }
            self.events.push_back(v);
        }
    }
    async fn send_once(
        &mut self,
        method: &str,
        mut frame: Value,
    ) -> Result<Value, ClientError<Error>> {
        if self.pending.is_some() {
            return Err(ClientError::Exchange(Error::PendingOutcome));
        }
        let id = self.next_id;
        self.next_id =
            self.next_id
                .checked_add(1)
                .ok_or(ClientError::Exchange(Error::InvalidRequest(
                    "command IDs exhausted",
                )))?;
        if self._service != "rfq" {
            frame["id"] = json!(id);
        }
        let text = serde_json::to_string(&frame).map_err(|e| ClientError::Exchange(e.into()))?;
        self.pending = Some(PendingCommand {
            id,
            method: method.into(),
            quote_id: frame
                .get("quote_id")
                .and_then(Value::as_str)
                .map(str::to_owned),
            correlation: frame
                .get("rfq_id")
                .and_then(Value::as_str)
                .map(str::to_owned),
        });
        if let Err(e) = self.connection.send(text).await {
            if e.outcome == RequestOutcome::NotSent {
                self.pending = None;
            }
            return Err(e.into());
        }
        self.reconcile().await
    }
}
impl<C: WebSocketConnection, A: Authentication> RpcSession<C, A> {
    async fn signed_command(
        &mut self,
        kind: &str,
        args: Value,
    ) -> Result<Value, ClientError<Error>> {
        if self.pending.is_some() {
            return Err(ClientError::Exchange(Error::PendingOutcome));
        }
        if self._service != "perps" {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "perps command requires a perps session",
            )));
        }
        let op = Operation {
            service: "perps".into(),
            client: "PolymarketPerpsClient".into(),
            method: kind.into(),
            verb: "POST".into(),
            path: "/v1/trade/orders".into(),
            auth: "perps".into(),
            safe: false,
            parameters: vec![],
            has_body: true,
            fixture: String::new(),
        };
        let mut wire = prepare_request(
            &self.exchange,
            &op,
            json!({"body":{"op":{"type":kind,"args":args}}}),
        )
        .map_err(ClientError::Exchange)?;
        self.auth
            .prepare(&self.exchange, &op, &mut wire)
            .await
            .map_err(ClientError::Exchange)?;
        let mut frame: Value =
            serde_json::from_slice(&wire.body).map_err(|e| ClientError::Exchange(e.into()))?;
        frame["req"] = json!("post");
        self.send_once(kind, frame).await
    }
    pub async fn create_orders(&mut self, args: Value) -> Result<Value, ClientError<Error>> {
        self.signed_command("createOrders", args).await
    }
    pub async fn modify_orders(&mut self, args: Value) -> Result<Value, ClientError<Error>> {
        self.signed_command("modifyOrders", args).await
    }
    pub async fn modify_orders_coid(&mut self, args: Value) -> Result<Value, ClientError<Error>> {
        self.signed_command("modifyOrdersCOjson!(id)", args).await
    }
    pub async fn cancel_orders(&mut self, args: Value) -> Result<Value, ClientError<Error>> {
        self.signed_command("cancelOrders", args).await
    }
    pub async fn cancel_orders_coid(&mut self, args: Value) -> Result<Value, ClientError<Error>> {
        self.signed_command("cancelOrdersCOjson!(id)", args).await
    }
    pub async fn cancel_all(&mut self, args: Value) -> Result<Value, ClientError<Error>> {
        self.signed_command("cancelAll", args).await
    }
    pub async fn auto_cancel(&mut self, args: Value) -> Result<Value, ClientError<Error>> {
        self.signed_command("autoCancel", args).await
    }
    pub async fn update_leverage(&mut self, args: Value) -> Result<Value, ClientError<Error>> {
        self.signed_command("updateLeverage", args).await
    }
    pub async fn update_leverages(&mut self, args: Value) -> Result<Value, ClientError<Error>> {
        self.signed_command("updateLeverages", args).await
    }
    pub async fn update_margin(&mut self, args: Value) -> Result<Value, ClientError<Error>> {
        self.signed_command("updateMargin", args).await
    }
    pub async fn subscribe(&mut self, channels: Vec<String>) -> Result<Value, ClientError<Error>> {
        self.send_once("subscribe", json!({"req":"sub","chs":channels}))
            .await
    }
    pub async fn unsubscribe(
        &mut self,
        channels: Vec<String>,
    ) -> Result<Value, ClientError<Error>> {
        self.send_once("unsubscribe", json!({"req":"unsub","chs":channels}))
            .await
    }
    pub async fn ping(&mut self) -> Result<Value, ClientError<Error>> {
        self.send_once("ping", json!({"req":"post","op":{"type":"ping"}}))
            .await
    }
}
impl<T: WebSocketConnector, A: Authentication + StreamAuthentication> PolymarketPerpsClient<T, A> {
    pub async fn open_rpc(&self) -> Result<RpcSession<T::Connection, A>, ClientError<Error>> {
        let connection = self
            .transport
            .connect(&self.exchange.websocket["perps"])
            .await?;
        let mut session = RpcSession {
            connection,
            exchange: self.exchange.clone(),
            auth: self.auth.clone(),
            _service: "perps".into(),
            next_id: 1,
            pending: None,
            events: VecDeque::new(),
            capacity: 128,
            timeout: Duration::from_secs(10),
        };
        let credentials = self
            .auth
            .stream_credentials("perps")
            .map_err(ClientError::Exchange)?
            .ok_or(ClientError::Exchange(Error::Authentication(
                "perps credentials required",
            )))?;
        let ack = session
            .send_once(
                "auth",
                json!({"req":"post","op":{"type":"auth","args":credentials}}),
            )
            .await?;
        if ack["data"]["status"] != "ok" {
            session.close().await?;
            return Err(ClientError::Exchange(Error::Authentication(
                "perps WebSocket authentication rejected",
            )));
        }
        Ok(session)
    }
}

impl<C: WebSocketConnection, A: Authentication> RpcSession<C, A> {
    pub async fn quote(&mut self, mut payload: Value) -> Result<Value, ClientError<Error>> {
        if self._service != "rfq" {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "quote requires an RFQ session",
            )));
        }
        if self.pending.is_some() {
            return Err(ClientError::Exchange(Error::PendingOutcome));
        }
        let op = Operation {
            service: "clob".into(),
            client: "PolymarketClient".into(),
            method: "post_order".into(),
            verb: "POST".into(),
            path: "/order".into(),
            auth: "native".into(),
            safe: false,
            parameters: vec![],
            has_body: true,
            fixture: String::new(),
        };
        let mut wire = prepare_request(
            &self.exchange,
            &op,
            json!({"body":{"order":payload["signed_order"]}}),
        )
        .map_err(ClientError::Exchange)?;
        self.auth
            .prepare(&self.exchange, &op, &mut wire)
            .await
            .map_err(ClientError::Exchange)?;
        let body: Value =
            serde_json::from_slice(&wire.body).map_err(|e| ClientError::Exchange(e.into()))?;
        payload["signed_order"] = body["order"].clone();
        payload["type"] = json!("RFQ_QUOTE");
        self.send_once("RFQ_QUOTE", payload).await
    }
    pub async fn cancel_quote(&mut self, mut payload: Value) -> Result<Value, ClientError<Error>> {
        if self._service != "rfq" {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "RFQ session required",
            )));
        }
        payload["type"] = json!("RFQ_QUOTE_CANCEL");
        self.send_once("RFQ_QUOTE_CANCEL", payload).await
    }
    pub async fn last_look(&mut self, mut payload: Value) -> Result<Value, ClientError<Error>> {
        if self._service != "rfq" {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "RFQ session required",
            )));
        }
        payload["type"] = json!("RFQ_CONFIRMATION_RESPONSE");
        self.send_once("RFQ_CONFIRMATION_RESPONSE", payload).await
    }
}
impl<T: WebSocketConnector, A: Authentication + StreamAuthentication> PolymarketClient<T, A> {
    pub async fn open_rpc(&self) -> Result<RpcSession<T::Connection, A>, ClientError<Error>> {
        let credentials = self
            .auth
            .stream_credentials("rfq")
            .map_err(ClientError::Exchange)?
            .ok_or(ClientError::Exchange(Error::Authentication(
                "RFQ credentials required",
            )))?;
        let connection = self
            .transport
            .connect(&self.exchange.websocket["rfq"])
            .await?;
        let mut session = RpcSession {
            connection,
            exchange: self.exchange.clone(),
            auth: self.auth.clone(),
            _service: "rfq".into(),
            next_id: 1,
            pending: None,
            events: VecDeque::new(),
            capacity: 128,
            timeout: Duration::from_secs(10),
        };
        let ack = session.send_once("auth", credentials).await?;
        if ack["success"] != true {
            session.close().await?;
            return Err(ClientError::Exchange(Error::Authentication(
                "RFQ authentication rejected",
            )));
        }
        Ok(session)
    }
}
