use crate::{Error, HyperdashClient, Result};
use aux_rs_core::{
    RequestOutcome, TransportError, TransportErrorKind, WebSocketConnection, WebSocketConnector,
};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::{mpsc, oneshot, watch};
use zeroize::Zeroizing;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Channel {
    Fills,
    Ledger,
    Twap,
    Orders,
    Funding,
    Account,
    Trades,
    Liquidations,
    Candles,
    Ctx,
    Book,
    Bbo,
    LiquidationLevels,
    StopLevels,
    Mids,
    Markets,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum CandleInterval {
    #[serde(rename = "1m")]
    Minute,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "1h")]
    Hour,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "1d")]
    Day,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SubscribeRequest {
    pub channel: Channel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<CandleInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl SubscribeRequest {
    fn validate(&self) -> Result<()> {
        let address = self.address.as_deref();
        let coin = self.coin.as_deref();
        let valid = match self.channel {
            Channel::Fills
            | Channel::Ledger
            | Channel::Twap
            | Channel::Orders
            | Channel::Funding
            | Channel::Account => address.is_some_and(valid_address) && coin.is_none(),
            Channel::Mids | Channel::Markets => {
                address.is_none() && matches!(coin, None | Some("*"))
            }
            Channel::Liquidations => address.is_none() && coin.is_some_and(|coin| !coin.is_empty()),
            _ => address.is_none() && coin.is_some_and(|coin| !coin.is_empty() && coin != "*"),
        };
        if !valid || (self.interval.is_some() && self.channel != Channel::Candles) {
            return Err(Error::InvalidRequest(
                "invalid Hyperdash subscription target or interval",
            ));
        }
        Ok(())
    }
}

fn valid_address(address: &str) -> bool {
    address
        .strip_prefix("0x")
        .is_some_and(|hex| hex.len() == 40 && hex.bytes().all(|byte| byte.is_ascii_hexdigit()))
}

/// Server-issued IDs are compared as sent and carried unchanged in commands.
#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum StreamId {
    Text(String),
    Integer(i128),
}

impl<'de> Deserialize<'de> for StreamId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        match Value::deserialize(deserializer)? {
            Value::String(id) => Ok(Self::Text(id)),
            Value::Number(id) => id
                .to_string()
                .parse()
                .map(Self::Integer)
                .map_err(serde::de::Error::custom),
            _ => Err(serde::de::Error::custom("expected a Hyperdash stream ID")),
        }
    }
}

// Keep each native frame typed and retain any fields added by the server.
#[derive(Debug, Deserialize, Serialize)]
pub struct Ready {
    pub key_id: String,
    pub environment: String,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Subscribed {
    pub sub: StreamId,
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::optional_nullable",
        skip_serializing_if = "Option::is_none"
    )]
    pub cursor: Option<Option<String>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Batch {
    pub sub: StreamId,
    pub batch: StreamId,
    pub cursor: String,
    pub events: Vec<Value>,
    pub credits: i128,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Acked {
    pub sub: StreamId,
    pub batch: StreamId,
    pub credits_charged: i128,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Unsubscribed {
    pub sub: StreamId,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Gap {
    pub sub: StreamId,
    pub from: Value,
    pub to: Value,
    pub detail: String,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreditsLow {
    pub remaining: i128,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StreamProblem {
    pub code: String,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<StreamId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_s: Option<i128>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum StreamMessage {
    Ready(Ready),
    Subscribed(Subscribed),
    Data(Batch),
    Acked(Acked),
    Unsubscribed(Unsubscribed),
    Gap(Gap),
    CreditsLow(CreditsLow),
    Error(StreamProblem),
    Unknown { native: Value },
}

fn frame(text: &str, authorization: &str) -> Result<Option<StreamMessage>> {
    fn typed<T: serde::de::DeserializeOwned>(text: &str) -> Result<T> {
        serde_json::from_str(text).map_err(|_| Error::Decode("invalid Hyperdash stream message"))
    }
    fn decode<T: serde::de::DeserializeOwned>(value: Value) -> Result<T> {
        serde_json::from_value(value).map_err(|_| Error::Decode("invalid Hyperdash stream message"))
    }
    let mut data: Value = typed(text)?;
    let operation = data.as_object_mut().and_then(|object| object.remove("op"));
    let message = match operation.as_ref().and_then(Value::as_str) {
        Some("ping") => return Ok(None),
        Some("ready") => StreamMessage::Ready(decode(data)?),
        Some("subscribed") => StreamMessage::Subscribed(decode(data)?),
        Some("data") => StreamMessage::Data(decode(data)?),
        Some("acked") => StreamMessage::Acked(decode(data)?),
        Some("unsubscribed") => StreamMessage::Unsubscribed(decode(data)?),
        Some("gap") => StreamMessage::Gap(decode(data)?),
        Some("credits_low") => StreamMessage::CreditsLow(decode(data)?),
        Some("error") => {
            let redacted = crate::client::redact(data, authorization);
            StreamMessage::Error(decode(redacted)?)
        }
        Some(_) => {
            data["op"] = operation.unwrap_or(Value::Null);
            StreamMessage::Unknown { native: data }
        }
        None => return Err(Error::Decode("missing Hyperdash stream operation")),
    };
    Ok(Some(message))
}

#[derive(Serialize)]
#[serde(tag = "op", rename_all = "snake_case")]
enum Action {
    Subscribe(SubscribeRequest),
    Ack { sub: StreamId, batch: StreamId },
    Unsubscribe { sub: StreamId },
}

struct Command {
    action: Action,
    answer: oneshot::Sender<Result<()>>,
}

#[derive(Default)]
struct Terminal {
    error: Option<Error>,
    discard: bool,
}

/// One authenticated multiplexed session. Clones share command and event queues.
/// At most one call to `next` may be in progress.
#[derive(Clone)]
pub struct HyperdashStream {
    inner: Arc<Session>,
}

struct Session {
    commands: mpsc::Sender<Command>,
    events: tokio::sync::Mutex<mpsc::Receiver<StreamMessage>>,
    terminal: Arc<Mutex<Terminal>>,
    stop: watch::Sender<bool>,
    done: watch::Receiver<bool>,
}

impl Drop for Session {
    fn drop(&mut self) {
        self.stop.send_replace(true);
    }
}

impl HyperdashStream {
    pub fn closed(&self) -> bool {
        *self.inner.stop.borrow() || *self.inner.done.borrow()
    }

    pub async fn subscribe(&self, request: SubscribeRequest) -> Result<()> {
        request.validate()?;
        self.send(Action::Subscribe(request)).await
    }

    pub async fn ack(&self, sub: StreamId, batch: StreamId) -> Result<()> {
        self.send(Action::Ack { sub, batch }).await
    }

    pub async fn unsubscribe(&self, sub: StreamId) -> Result<()> {
        self.send(Action::Unsubscribe { sub }).await
    }

    pub async fn next(&self) -> Result<Option<StreamMessage>> {
        let mut events =
            self.inner.events.try_lock().map_err(|_| {
                Error::InvalidRequest("a Hyperdash stream read is already in progress")
            })?;
        let mut stop = self.inner.stop.subscribe();
        if *stop.borrow() {
            return Ok(None);
        }
        let event = tokio::select! {
            biased;
            _ = stop.changed() => return Ok(None),
            event = events.recv() => event,
        };
        let mut terminal = self
            .inner
            .terminal
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if terminal.discard {
            events.close();
            while events.try_recv().is_ok() {}
        } else if event.is_some() {
            return Ok(event);
        }
        terminal.error.take().map_or(Ok(None), Err)
    }

    pub async fn close(&self) {
        self.inner.stop.send_replace(true);
        let mut done = self.inner.done.clone();
        let _ = done.wait_for(|done| *done).await;
    }

    async fn send(&self, action: Action) -> Result<()> {
        if self.closed() {
            return Err(closed());
        }
        let (answer, response) = oneshot::channel();
        self.inner
            .commands
            .try_send(Command { action, answer })
            .map_err(|error| match error {
                mpsc::error::TrySendError::Full(_) => {
                    transport(TransportErrorKind::Overflow, "Hyperdash command queue full")
                }
                mpsc::error::TrySendError::Closed(_) => closed(),
            })?;
        response.await.unwrap_or_else(|_| Err(closed()))
    }
}

impl<T: WebSocketConnector> HyperdashClient<T>
where
    T::Connection: 'static,
{
    pub async fn open_stream(&self) -> Result<HyperdashStream> {
        let runtime = tokio::runtime::Handle::try_current()
            .map_err(|_| Error::InvalidRequest("Hyperdash streams require a Tokio runtime"))?;
        let authorization = self.authorization.clone().ok_or(Error::InvalidRequest(
            "Hyperdash streams require an API key",
        ))?;
        let mut connection = self
            .transport()
            .connect_with_close_frames(
                self.ws_url.as_str(),
                vec![("authorization".into(), authorization.as_str().into())],
            )
            .await?;
        let initial =
            tokio::time::timeout(self.ready_timeout, ready(&mut connection, &authorization))
                .await
                .map_err(|_| {
                    transport(
                        TransportErrorKind::Timeout,
                        "Hyperdash WebSocket ready timed out",
                    )
                })??;

        let (commands, command_rx) = mpsc::channel(self.stream_capacity);
        let (event_tx, events) = mpsc::channel(self.stream_capacity);
        let (stop, stop_rx) = watch::channel(false);
        let (done_tx, done) = watch::channel(false);
        let terminal = Arc::new(Mutex::new(Terminal::default()));
        event_tx.try_send(initial).map_err(|_| closed())?;
        runtime.spawn(drive(
            connection,
            Driver {
                authorization,
                commands: command_rx,
                events: event_tx,
                terminal: terminal.clone(),
                stop: stop_rx,
                done: done_tx,
                timeout: self.ready_timeout,
            },
        ));
        Ok(HyperdashStream {
            inner: Arc::new(Session {
                commands,
                events: tokio::sync::Mutex::new(events),
                terminal,
                stop,
                done,
            }),
        })
    }
}

async fn ready<C: WebSocketConnection>(
    connection: &mut C,
    authorization: &str,
) -> Result<StreamMessage> {
    loop {
        let text = receive(connection, authorization).await?;
        match frame(&text, authorization)? {
            Some(message @ StreamMessage::Ready(_)) => return Ok(message),
            Some(StreamMessage::Error(problem)) => {
                return Err(Error::StreamError(
                    serde_json::to_value(StreamMessage::Error(problem)).unwrap_or(Value::Null),
                ));
            }
            None => connection.send(r#"{"op":"pong"}"#.into()).await?,
            Some(_) => return Err(Error::Decode("expected Hyperdash ready message")),
        }
    }
}

struct Driver {
    authorization: Arc<Zeroizing<String>>,
    commands: mpsc::Receiver<Command>,
    events: mpsc::Sender<StreamMessage>,
    terminal: Arc<Mutex<Terminal>>,
    stop: watch::Receiver<bool>,
    done: watch::Sender<bool>,
    timeout: Duration,
}

async fn drive<C: WebSocketConnection>(mut connection: C, mut driver: Driver) {
    let result = tokio::select! {
        biased;
        _ = driver.stop.wait_for(|stop| *stop) => Ok(()),
        result = pump(&mut connection, &driver.authorization, &mut driver.commands, &driver.events) => result,
    };
    if let Err(error) = result {
        let discard =
            matches!(&error, Error::Transport(error) if error.kind == TransportErrorKind::Overflow);
        *driver
            .terminal
            .lock()
            .unwrap_or_else(|error| error.into_inner()) = Terminal {
            error: Some(error),
            discard,
        };
    }
    drop(driver.events);
    drop(driver.commands);
    let _ = tokio::time::timeout(driver.timeout, connection.close()).await;
    driver.done.send_replace(true);
}

async fn pump<C: WebSocketConnection>(
    connection: &mut C,
    authorization: &str,
    commands: &mut mpsc::Receiver<Command>,
    events: &mpsc::Sender<StreamMessage>,
) -> Result<()> {
    let mut pending = HashMap::new();
    loop {
        tokio::select! {
            command = commands.recv() => {
                let Some(command) = command else { return Ok(()); };
                if command.answer.is_closed() {
                    continue;
                }
                let result = send_action(connection, &mut pending, command.action).await;
                let _ = command.answer.send(result);
            }
            text = receive(connection, authorization) => {
                let text = text?;
                let Some(message) = frame(&text, authorization)? else {
                    connection.send(r#"{"op":"pong"}"#.into()).await?;
                    continue;
                };
                match &message {
                    StreamMessage::Data(batch) => {
                        if pending.len() >= 20 && !pending.contains_key(&batch.sub) {
                            return Err(Error::Decode("Hyperdash sent too many pending batches"));
                        }
                        if pending.insert(batch.sub.clone(), batch.batch.clone()).is_some() {
                            return Err(Error::Decode("Hyperdash sent a second batch before acknowledgement"));
                        }
                    }
                    StreamMessage::Unsubscribed(message) => { pending.remove(&message.sub); }
                    StreamMessage::Error(message) if message.code == "ack_timeout" || message.code == "subscription_superseded" => {
                        if let Some(sub) = &message.sub { pending.remove(sub); }
                    }
                    _ => {}
                }
                if events.try_send(message).is_err() {
                    return Err(transport(TransportErrorKind::Overflow, "Hyperdash event queue overflowed"));
                }
            }
        }
    }
}

async fn send_action<C: WebSocketConnection>(
    connection: &mut C,
    pending: &mut HashMap<StreamId, StreamId>,
    action: Action,
) -> Result<()> {
    if let Action::Ack { sub, batch } = &action
        && pending.get(sub) != Some(batch)
    {
        return Err(Error::InvalidRequest("no matching pending Hyperdash batch"));
    }
    let text = serde_json::to_string(&action)
        .map_err(|_| Error::InvalidRequest("invalid Hyperdash message"))?;
    if text.len() > 64 * 1024 {
        return Err(Error::InvalidRequest("Hyperdash message exceeded 64 KiB"));
    }
    let result = connection.send(text).await;
    if let Action::Ack { sub, .. } = action
        && (result.is_ok()
            || result
                .as_ref()
                .is_err_and(|error| error.outcome == RequestOutcome::Unknown))
    {
        pending.remove(&sub);
    }
    result.map_err(Error::from)
}

async fn receive<C: WebSocketConnection>(
    connection: &mut C,
    authorization: &str,
) -> Result<String> {
    match connection.next().await {
        Some(text) => text.map_err(Error::from),
        None => {
            let frame = connection.take_close_frame();
            let code = frame.as_ref().map(|frame| frame.code);
            let reason =
                frame.map(|frame| crate::client::redact_text(&frame.reason, authorization));
            Err(Error::StreamClosed { code, reason })
        }
    }
}

fn transport(kind: TransportErrorKind, detail: &'static str) -> Error {
    Error::Transport(TransportError::new(kind, RequestOutcome::Unknown, detail))
}

fn closed() -> Error {
    Error::Transport(TransportError::new(
        TransportErrorKind::Closed,
        RequestOutcome::NotSent,
        "Hyperdash stream is closed",
    ))
}
