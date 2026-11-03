use crate::*;
use aux_rs_core::{
    Error as ClientError, Exchange, RequestOutcome, TransportError, TransportErrorKind,
    Unauthenticated, WebSocketConnection, WebSocketConnector,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
#[derive(Clone, Debug)]
pub struct PendingCommand {
    pub id: u64,
    pub method: String,
}
/// An isolated ordered session. Writes are never reconnected or replayed.
pub struct RpcSession<C, A = Unauthenticated> {
    pub(crate) connection: C,
    pub(crate) auth: A,
    pub(crate) exchange: Derive,
    pending: Option<PendingCommand>,
    next_id: u64,
    events: VecDeque<Value>,
    capacity: usize,
    timeout: Duration,
    closed: bool,
}
impl<C, A> std::fmt::Debug for RpcSession<C, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RpcSession")
            .field("pending", &self.pending)
            .field("closed", &self.closed)
            .finish_non_exhaustive()
    }
}
impl<C: WebSocketConnection, A: Send + Sync> RpcSession<C, A> {
    pub fn auth(&self) -> &A {
        &self.auth
    }
    pub fn exchange(&self) -> &Derive {
        &self.exchange
    }
    fn new(connection: C, exchange: Derive, auth: A) -> Self {
        Self {
            connection,
            exchange,
            auth,
            pending: None,
            next_id: 1,
            events: VecDeque::new(),
            capacity: 128,
            timeout: Duration::from_secs(10),
            closed: false,
        }
    }
    pub fn pending(&self) -> Option<&PendingCommand> {
        self.pending.as_ref()
    }
    pub fn with_limits(mut self, capacity: usize, timeout: Duration) -> Result<Self, Error> {
        if capacity == 0 || timeout.is_zero() {
            return Err(Error::InvalidRequest(
                "positive RPC timeout and capacity required",
            ));
        }
        self.capacity = capacity;
        self.timeout = timeout;
        Ok(self)
    }
    pub(crate) fn ensure_ready(&self) -> Result<(), ClientError<Error>> {
        if self.pending.is_some() {
            return Err(ClientError::Exchange(Error::PendingOutcome));
        }
        if self.closed {
            return Err(ClientError::Transport(TransportError::new(
                TransportErrorKind::Closed,
                RequestOutcome::NotSent,
                "RPC session is closed",
            )));
        }
        Ok(())
    }
    pub(crate) async fn call<Q: Serialize, R: DeserializeOwned>(
        &mut self,
        method: &str,
        params: Q,
    ) -> Result<R, ClientError<Error>> {
        self.ensure_ready()?;
        let id = self.next_id;
        self.next_id =
            self.next_id
                .checked_add(1)
                .ok_or(ClientError::Exchange(Error::InvalidRequest(
                    "RPC request id exhausted",
                )))?;
        let text = serde_json::to_string(
            &json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}),
        )
        .map_err(Error::from)
        .map_err(ClientError::Exchange)?;
        self.pending = Some(PendingCommand {
            id,
            method: method.into(),
        });
        if let Err(error) = self.connection.send(text).await {
            if error.outcome == RequestOutcome::NotSent {
                self.pending = None;
            }
            return Err(ClientError::Transport(error));
        }
        let result = self.reconcile().await?;
        serde_json::from_value(result)
            .map_err(Error::from)
            .map_err(ClientError::Exchange)
    }
    /// Resume waiting after a timeout or cancelled command. Never resubmits it.
    pub async fn reconcile(&mut self) -> Result<Value, ClientError<Error>> {
        let id = self
            .pending
            .as_ref()
            .ok_or(ClientError::Exchange(Error::InvalidRequest(
                "no pending RPC command",
            )))?
            .id;
        let start = Instant::now();
        loop {
            let budget = self.timeout.saturating_sub(start.elapsed());
            if budget.is_zero() {
                return Err(ClientError::Transport(TransportError::new(
                    TransportErrorKind::Timeout,
                    RequestOutcome::Unknown,
                    "RPC response deadline elapsed",
                )));
            }
            let text = self
                .connection
                .next_with_timeout(budget)
                .await
                .ok_or_else(|| {
                    ClientError::Transport(TransportError::new(
                        TransportErrorKind::Disconnected,
                        RequestOutcome::Unknown,
                        "RPC connection ended with pending command",
                    ))
                })??;
            if text == "pong" {
                continue;
            }
            let value: Value = serde_json::from_str(&text)
                .map_err(Error::from)
                .map_err(ClientError::Exchange)?;
            if value.get("id").is_some_and(|v| !v.is_null()) {
                if value["id"] != id {
                    return Err(ClientError::Exchange(Error::InvalidRequest(
                        "unexpected RPC response id",
                    )));
                }
                if value.get("result").is_none() && value.get("error").is_none() {
                    return Err(ClientError::Exchange(Error::InvalidRequest(
                        "RPC response has no result or error",
                    )));
                }
                self.pending = None;
                if value.get("error").is_some_and(|e| !e.is_null()) {
                    return Err(ClientError::Exchange(Error::Api {
                        status: None,
                        body: value,
                    }));
                }
                return Ok(value["result"].clone());
            }
            if self.events.len() >= self.capacity {
                self.closed = true;
                let _ = self.connection.close().await;
                return Err(ClientError::Transport(TransportError::new(
                    TransportErrorKind::Overflow,
                    RequestOutcome::Unknown,
                    "RPC event buffer overflowed; reconcile account state",
                )));
            }
            self.events.push_back(value);
        }
    }
    pub async fn next_event(&mut self) -> Option<Result<Event, ClientError<Error>>> {
        if self.pending.is_some() {
            return Some(Err(ClientError::Exchange(Error::PendingOutcome)));
        }
        if let Some(value) = self.events.pop_front() {
            return Some(Event::decode(&value.to_string()).map_err(ClientError::Exchange));
        }
        if self.closed {
            return None;
        }
        loop {
            let text = match self.connection.next().await {
                Some(Ok(t)) => t,
                Some(Err(e)) => {
                    self.closed = true;
                    return Some(Err(ClientError::Transport(e)));
                }
                None => {
                    self.closed = true;
                    return None;
                }
            };
            if text == "pong" {
                continue;
            }
            return Some(Event::decode(&text).map_err(ClientError::Exchange));
        }
    }
    pub async fn close(&mut self) -> Result<(), ClientError<Error>> {
        self.closed = true;
        self.connection
            .close()
            .await
            .map_err(ClientError::Transport)
    }
}
impl<T: WebSocketConnector, A: Sync> DeriveClient<T, A> {
    pub async fn open_public_rpc(&self) -> Result<RpcSession<T::Connection>, ClientError<Error>> {
        let connection = self.transport.connect(self.exchange.ws_url()).await?;
        Ok(RpcSession::new(
            connection,
            self.exchange.clone(),
            Unauthenticated,
        ))
    }
}
impl<T: WebSocketConnector, A: Authentication + Clone> DeriveClient<T, A> {
    pub async fn open_rpc(&self) -> Result<RpcSession<T::Connection, A>, ClientError<Error>> {
        let login = self
            .auth
            .login(&self.exchange)
            .await
            .map_err(ClientError::Exchange)?;
        let connection = self.transport.connect(self.exchange.ws_url()).await?;
        let mut session = RpcSession::new(connection, self.exchange.clone(), self.auth.clone());
        let _: Vec<serde_json::Number> = session.call("public/login", login).await?;
        Ok(session)
    }
}
impl<C: WebSocketConnection, A: Authentication> RpcSession<C, A> {
    pub async fn set_cancel_on_disconnect(
        &mut self,
        request: SetCancelOnDisconnectRequest,
    ) -> Result<SetCancelOnDisconnectResponse, ClientError<Error>> {
        self.call("private/set_cancel_on_disconnect", request).await
    }
}
impl<C: WebSocketConnection, A: Authentication + Clone> RpcSession<C, A> {
    pub(crate) async fn prepare_command(
        &mut self,
        path: &str,
        body: &mut Value,
    ) -> Result<(), ClientError<Error>> {
        self.ensure_ready()?;
        let auth = self.auth.clone();
        let exchange = self.exchange.clone();
        crate::preparation::prepare_native(self, &exchange, &auth, path, body).await
    }
}
impl<C: WebSocketConnection, A: Send + Sync> RpcSession<C, A> {
    pub async fn subscribe_channels(
        &mut self,
        request: SubscribeParams,
    ) -> Result<SubscribeResult, ClientError<Error>> {
        self.call("subscribe", request).await
    }
    pub async fn unsubscribe_channels(
        &mut self,
        request: UnsubscribeParams,
    ) -> Result<UnsubscribeResult, ClientError<Error>> {
        self.call("unsubscribe", request).await
    }
}
