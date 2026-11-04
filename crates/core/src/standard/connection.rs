use super::{OperationGuard, StandardTransport, closed, ws};
use crate::{
    RequestOutcome, TransportError, TransportErrorKind, WebSocketConnection, WebSocketConnector,
};
use futures_util::{SinkExt, StreamExt};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use tokio::{
    sync::{Notify, mpsc, oneshot, watch},
    time::{Instant, timeout},
};
use tokio_tungstenite::tungstenite::Message;

enum Command {
    Send(String, oneshot::Sender<Result<(), TransportError>>),
    Close(oneshot::Sender<Result<(), TransportError>>),
}
#[derive(Default)]
struct Queue {
    frames: VecDeque<String>,
    terminal: Option<TransportError>,
    closed: bool,
}
struct Inbox {
    queue: Mutex<Queue>,
    ready: Notify,
    space: Notify,
    capacity: usize,
    read_only: bool,
}
impl Inbox {
    fn can_read(&self) -> bool {
        !self.read_only
            || self
                .queue
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .frames
                .len()
                < self.capacity
    }

    fn finish(&self, error: Option<TransportError>) {
        let mut q = self.queue.lock().unwrap_or_else(|e| e.into_inner());
        q.closed = true;
        if error.is_some() {
            q.frames.clear();
            q.terminal = error;
        }
        drop(q);
        self.ready.notify_one();
    }
    fn push(&self, text: String) -> bool {
        let mut q = self.queue.lock().unwrap_or_else(|e| e.into_inner());
        if q.closed {
            return false;
        }
        if q.frames.len() >= self.capacity {
            drop(q);
            self.finish(Some(error(
                TransportErrorKind::Overflow,
                "WebSocket connection buffer overflowed",
            )));
            return false;
        }
        q.frames.push_back(text);
        drop(q);
        self.ready.notify_one();
        true
    }
    async fn next(&self) -> Option<Result<String, TransportError>> {
        loop {
            {
                let mut q = self.queue.lock().unwrap_or_else(|e| e.into_inner());
                if let Some(e) = q.terminal.take() {
                    return Some(Err(e));
                }
                if let Some(s) = q.frames.pop_front() {
                    self.space.notify_one();
                    return Some(Ok(s));
                }
                if q.closed {
                    return None;
                }
            }
            self.ready.notified().await;
        }
    }
}
fn error(kind: TransportErrorKind, message: &'static str) -> TransportError {
    TransportError::new(kind, RequestOutcome::Unknown, message)
}
/// Isolated connection with bounded buffering and no automatic replay/reconnect.
pub struct StandardConnection {
    commands: mpsc::UnboundedSender<Command>,
    inbox: Arc<Inbox>,
    timeout: std::time::Duration,
    ended: bool,
}
impl WebSocketConnection for StandardConnection {
    async fn next_with_timeout(
        &mut self,
        budget: std::time::Duration,
    ) -> Option<Result<String, TransportError>> {
        match timeout(budget, self.next()).await {
            Ok(r) => r,
            Err(_) => Some(Err(error(
                TransportErrorKind::Timeout,
                "WebSocket response timed out",
            ))),
        }
    }
    async fn send(&mut self, text: String) -> Result<(), TransportError> {
        if self.ended {
            return Err(closed(RequestOutcome::NotSent));
        }
        {
            let q = self.inbox.queue.lock().unwrap_or_else(|e| e.into_inner());
            if q.closed {
                return Err(closed(RequestOutcome::NotSent));
            }
        }
        let (tx, rx) = oneshot::channel();
        self.commands
            .send(Command::Send(text, tx))
            .map_err(|_| closed(RequestOutcome::NotSent))?;
        timeout(self.timeout, rx)
            .await
            .map_err(|_| error(TransportErrorKind::Timeout, "WebSocket send timed out"))?
            .unwrap_or_else(|_| {
                Err(error(
                    TransportErrorKind::Disconnected,
                    "WebSocket send interrupted",
                ))
            })
    }
    async fn next(&mut self) -> Option<Result<String, TransportError>> {
        if self.ended {
            return None;
        }
        let r = self.inbox.next().await;
        self.ended = r.as_ref().is_none_or(Result::is_err);
        r
    }
    async fn close(&mut self) -> Result<(), TransportError> {
        self.ended = true;
        self.inbox.finish(None);
        let (tx, rx) = oneshot::channel();
        if self.commands.send(Command::Close(tx)).is_err() {
            return Ok(());
        }
        timeout(self.timeout, rx)
            .await
            .map_err(|_| error(TransportErrorKind::Timeout, "WebSocket close timed out"))?
            .unwrap_or(Ok(()))
    }
}
impl Drop for StandardConnection {
    fn drop(&mut self) {
        self.inbox.finish(None);
        let (tx, _) = oneshot::channel();
        let _ = self.commands.send(Command::Close(tx));
    }
}
impl WebSocketConnector for StandardTransport {
    type Connection = StandardConnection;
    async fn connect(&self, url: &str) -> Result<Self::Connection, TransportError> {
        self.connect_dedicated(url, false, Vec::new()).await
    }
    async fn connect_with_headers(
        &self,
        url: &str,
        headers: Vec<(String, String)>,
    ) -> Result<Self::Connection, TransportError> {
        self.connect_dedicated(url, false, headers).await
    }
    async fn connect_read(&self, url: &str) -> Result<Self::Connection, TransportError> {
        self.connect_dedicated(url, true, Vec::new()).await
    }
}
impl StandardTransport {
    async fn connect_dedicated(
        &self,
        url: &str,
        read_only: bool,
        headers: Vec<(String, String)>,
    ) -> Result<StandardConnection, TransportError> {
        let guard = OperationGuard::start(&self.lifecycle)?;
        let mut shutdown = self.lifecycle.closed.subscribe();
        self.lifecycle.check()?;
        let socket = tokio::select! { biased; _=shutdown.changed()=>return Err(closed(RequestOutcome::NotSent)), r=ws::connect(url, self.tls.clone(), &self.config, headers)=>r? };
        self.lifecycle.check()?;
        let inbox = Arc::new(Inbox {
            queue: Mutex::default(),
            ready: Notify::new(),
            space: Notify::new(),
            capacity: self.config.subscription_capacity,
            read_only,
        });
        let (commands, receiver) = mpsc::unbounded_channel();
        tokio::spawn(run(
            socket,
            receiver,
            inbox.clone(),
            self.config.clone(),
            shutdown,
            guard,
        ));
        Ok(StandardConnection {
            commands,
            inbox,
            timeout: self.config.timeout,
            ended: false,
        })
    }
}
async fn run(
    mut socket: ws::Socket,
    mut commands: mpsc::UnboundedReceiver<Command>,
    inbox: Arc<Inbox>,
    config: crate::TransportConfig,
    mut shutdown: watch::Receiver<bool>,
    _guard: OperationGuard,
) {
    let mut heartbeat = tokio::time::interval(config.heartbeat_interval);
    heartbeat.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    heartbeat.tick().await;
    let mut last_read = Instant::now();
    loop {
        tokio::select! {
            biased;
            _=shutdown.changed()=> { inbox.finish(Some(closed(RequestOutcome::Unknown))); break; }
            command=commands.recv()=> match command {
                Some(Command::Send(text, done)) => {
                    // A cancelled caller must not leave a queued write to execute later.
                    if done.is_closed() { continue; }
                    let r = timeout(config.timeout, socket.send(Message::Text(text.into()))).await;
                    let result = match r { Ok(Ok(()))=>Ok(()), Ok(Err(_))=>Err(error(TransportErrorKind::Disconnected,"WebSocket send failed")), Err(_)=>Err(error(TransportErrorKind::Timeout,"WebSocket send timed out")) };
                    let failed = result.is_err();
                    if let Err(e)=&result { inbox.finish(Some(e.clone())); }
                    let _=done.send(result); if failed { break; }
                }
                Some(Command::Close(done))=> { let r=timeout(config.timeout, socket.close(None)).await; let _=done.send(match r { Ok(Ok(()))=>Ok(()), _=>Err(error(TransportErrorKind::Disconnected,"WebSocket close failed")) }); inbox.finish(None); return; }
                None=> { inbox.finish(None); break; }
            },
            _=inbox.space.notified(),if inbox.read_only=> {},
            frame=socket.next(),if inbox.can_read()=> {
                last_read=Instant::now();
                match frame {
                    Some(Ok(Message::Text(text)))=> if !inbox.push(text.to_string()) { break; },
                    Some(Ok(Message::Ping(data)))=> if !matches!(timeout(config.timeout,socket.send(Message::Pong(data))).await, Ok(Ok(()))) { inbox.finish(Some(error(TransportErrorKind::Timeout,"WebSocket pong timed out"))); break; },
                    Some(Ok(Message::Pong(_)))=> {},
                    Some(Ok(Message::Close(Some(frame)))) if inbox.read_only && frame.code==tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Normal=> {inbox.finish(None);break;},
                    Some(Ok(Message::Close(_)))|None=> { inbox.finish(Some(error(TransportErrorKind::Disconnected,"WebSocket disconnected"))); break; },
                    Some(Err(tokio_tungstenite::tungstenite::Error::Capacity(_)))=> { inbox.finish(Some(error(TransportErrorKind::ResponseTooLarge,"WebSocket message exceeds configured size limit"))); break; },
                    Some(Err(_))=> { inbox.finish(Some(error(TransportErrorKind::Disconnected,"WebSocket receive failed"))); break; },
                    _=> { inbox.finish(Some(error(TransportErrorKind::Protocol,"Invalid WebSocket frame"))); break; }
                }
            },
            _=heartbeat.tick()=> {
                if !inbox.can_read(){last_read=Instant::now();continue;}
                if last_read.elapsed()>=config.idle_timeout { inbox.finish(Some(error(TransportErrorKind::Timeout,"WebSocket idle timeout"))); break; }
                if !matches!(timeout(config.timeout,socket.send(Message::Ping(Vec::new().into()))).await,Ok(Ok(()))) { inbox.finish(Some(error(TransportErrorKind::Disconnected,"WebSocket heartbeat failed"))); break; }
            }
        }
    }
    let _ = timeout(config.timeout, socket.close(None)).await;
}
