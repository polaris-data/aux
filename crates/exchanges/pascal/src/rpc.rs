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
}
pub struct RpcSession<C, A> {
    connection: C,
    exchange: Pascal,
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
            let matches = v.get("id").and_then(|v| {
                v.as_u64()
                    .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
            }) == Some(id);
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
        frame["id"] = json!(id.to_string());
        let text = serde_json::to_string(&frame).map_err(|e| ClientError::Exchange(e.into()))?;
        self.pending = Some(PendingCommand {
            id,
            method: method.into(),
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
        descriptor: &str,
        payload: Value,
    ) -> Result<Value, ClientError<Error>> {
        if self.pending.is_some() {
            return Err(ClientError::Exchange(Error::PendingOutcome));
        }
        let op: Operation =
            serde_json::from_str(descriptor).map_err(|e| ClientError::Exchange(e.into()))?;
        let mut wire = prepare_request(&self.exchange, &op, json!({"body":payload}))
            .map_err(ClientError::Exchange)?;
        self.auth
            .prepare(&self.exchange, &op, &mut wire)
            .await
            .map_err(ClientError::Exchange)?;
        let payload: Value =
            serde_json::from_slice(&wire.body).map_err(|e| ClientError::Exchange(e.into()))?;
        self.send_once(kind, json!({"type":kind,"payload":payload}))
            .await
    }
    pub async fn place_orders(&mut self, payload: Value) -> Result<Value, ClientError<Error>> {
        self.signed_command("place", write::OP_PLACE_AND_REPLACE_ORDERS, payload)
            .await
    }
    pub async fn cancel_orders(&mut self, payload: Value) -> Result<Value, ClientError<Error>> {
        self.signed_command("cancel", write::OP_CANCEL_ORDERS, payload)
            .await
    }
    pub async fn mass_cancel(&mut self, payload: Value) -> Result<Value, ClientError<Error>> {
        self.signed_command("mass_cancel", write::OP_MASS_CANCEL_ORDERS, payload)
            .await
    }
    pub async fn ping(&mut self) -> Result<Value, ClientError<Error>> {
        self.send_once("ping", json!({"type":"ping"})).await
    }
}
impl<T: WebSocketConnector, A: Authentication + StreamAuthentication> PascalClient<T, A> {
    pub async fn open_rpc(&self) -> Result<RpcSession<T::Connection, A>, ClientError<Error>> {
        let connection = self
            .transport
            .connect(&self.exchange.websocket["write"])
            .await?;
        Ok(RpcSession {
            connection,
            exchange: self.exchange.clone(),
            auth: self.auth.clone(),
            _service: "write".into(),
            next_id: 1,
            pending: None,
            events: VecDeque::new(),
            capacity: 128,
            timeout: Duration::from_secs(10),
        })
    }
}
