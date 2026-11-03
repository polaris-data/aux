use super::{OperationGuard, StandardTransport, closed};
use crate::{
    RequestOutcome, TransportConfig, TransportError, TransportErrorKind, WebSocketStream,
    WebSocketSubscription, WebSocketTransport,
};
use futures_util::{SinkExt, StreamExt};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};
use tokio::{
    net::TcpStream,
    sync::{Notify, mpsc, oneshot, watch},
    time::{Instant, timeout},
};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream as SocketStream,
    tungstenite::{Error as WsError, Message, protocol::WebSocketConfig},
};

pub(super) type Socket = SocketStream<MaybeTlsStream<TcpStream>>;
type Completion = oneshot::Sender<Result<(), TransportError>>;
enum Attachment {
    Attached,
    FreshConnection,
}
static NEXT_ID: AtomicU64 = AtomicU64::new(1);
#[derive(Clone)]
pub(super) struct Session {
    commands: mpsc::UnboundedSender<Command>,
}
enum Command {
    Add {
        id: u64,
        spec: Arc<dyn WebSocketSubscription>,
        peer: Arc<Mailbox>,
        done: oneshot::Sender<Result<Attachment, TransportError>>,
    },
    Remove {
        id: u64,
        done: Option<Completion>,
    },
}
#[derive(Default)]
struct Queue {
    frames: VecDeque<String>,
    gap: bool,
    terminal: Option<TransportError>,
    closed: bool,
}
/// One consumer per mailbox. Control/error state cannot be hidden behind a full
/// data queue. A discontinuity atomically discards stale buffered frames.
struct Mailbox {
    queue: Mutex<Queue>,
    ready: Notify,
    capacity: usize,
}
impl Mailbox {
    fn new(capacity: usize) -> Arc<Self> {
        Arc::new(Self {
            queue: Mutex::default(),
            ready: Notify::new(),
            capacity,
        })
    }
    fn push(&self, text: &str) -> bool {
        let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
        if queue.closed || queue.terminal.is_some() {
            return false;
        }
        if queue.frames.len() >= self.capacity {
            queue.terminal = Some(TransportError::new(
                TransportErrorKind::Overflow,
                RequestOutcome::Unknown,
                "subscription buffer overflowed; stream continuity was lost",
            ));
            queue.frames.clear();
            drop(queue);
            self.ready.notify_one();
            return false;
        }
        queue.frames.push_back(text.to_owned());
        drop(queue);
        self.ready.notify_one();
        true
    }
    fn gap(&self) {
        let mut q = self.queue.lock().unwrap_or_else(|e| e.into_inner());
        q.frames.clear();
        q.gap = true;
        drop(q);
        self.ready.notify_one();
    }
    fn finish(&self, error: Option<TransportError>) {
        let mut q = self.queue.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(error) = error {
            q.terminal = Some(error);
            q.frames.clear();
        }
        q.closed = true;
        drop(q);
        self.ready.notify_one();
    }
    async fn next(&self) -> Option<Result<String, TransportError>> {
        loop {
            {
                let mut q = self.queue.lock().unwrap_or_else(|e| e.into_inner());
                if let Some(error) = q.terminal.take() {
                    return Some(Err(error));
                }
                if q.gap {
                    q.gap = false;
                    return Some(Err(TransportError::new(
                        TransportErrorKind::Gap,
                        RequestOutcome::Unknown,
                        "WebSocket continuity lost; discard derived state and reconcile fresh subscriptions",
                    )));
                }
                if let Some(text) = q.frames.pop_front() {
                    return Some(Ok(text));
                }
                if q.closed {
                    return None;
                }
            }
            // notify_one stores a permit even before this single waiter polls.
            self.ready.notified().await;
        }
    }
}
struct Entry {
    spec: Arc<dyn WebSocketSubscription>,
    peers: HashMap<u64, Arc<Mailbox>>,
    awaiting_snapshot: bool,
}
/// Gap errors are recoverable. Overflow, invalid frames, exhausted recovery and
/// expired credentials are terminal. close() and drop detach this consumer.
pub struct StandardSubscription {
    id: u64,
    commands: mpsc::UnboundedSender<Command>,
    mailbox: Arc<Mailbox>,
    timeout: std::time::Duration,
    ended: bool,
    detached: bool,
}
fn disconnected() -> TransportError {
    TransportError::new(
        TransportErrorKind::Disconnected,
        RequestOutcome::Unknown,
        "WebSocket disconnected; stream continuity was lost",
    )
}
fn timed_out() -> TransportError {
    TransportError::new(
        TransportErrorKind::Timeout,
        RequestOutcome::Unknown,
        "WebSocket operation timed out",
    )
}
fn protocol(message: &'static str) -> TransportError {
    TransportError::new(
        TransportErrorKind::Protocol,
        RequestOutcome::Unknown,
        message,
    )
}
fn auth_expired() -> TransportError {
    TransportError::new(
        TransportErrorKind::Authentication,
        RequestOutcome::NotSent,
        "subscription credentials expired; open a new authenticated subscription",
    )
}
impl WebSocketStream for StandardSubscription {
    async fn next(&mut self) -> Option<Result<String, TransportError>> {
        if self.ended {
            return None;
        }
        let result = self.mailbox.next().await;
        self.ended = result
            .as_ref()
            .is_none_or(|r| r.as_ref().is_err_and(|e| !e.is_gap()));
        result
    }
    async fn close(&mut self) -> Result<(), TransportError> {
        if self.detached {
            return Ok(());
        }
        self.detached = true;
        self.ended = true;
        self.mailbox.finish(None);
        let (done, completed) = oneshot::channel();
        if self
            .commands
            .send(Command::Remove {
                id: self.id,
                done: Some(done),
            })
            .is_err()
        {
            return Ok(());
        }
        timeout(self.timeout, completed)
            .await
            .map_err(|_| timed_out())?
            .unwrap_or(Ok(()))
    }
}
impl Drop for StandardSubscription {
    fn drop(&mut self) {
        if !self.detached {
            self.mailbox.finish(None);
            let _ = self.commands.send(Command::Remove {
                id: self.id,
                done: None,
            });
        }
    }
}

pub(super) async fn connect(
    url: &str,
    tls: Arc<rustls::ClientConfig>,
    config: &TransportConfig,
) -> Result<Socket, TransportError> {
    let socket_config = WebSocketConfig::default()
        .max_message_size(Some(config.max_message_bytes))
        .max_frame_size(Some(config.max_message_bytes));
    let connection = tokio_tungstenite::connect_async_tls_with_config(
        url,
        Some(socket_config),
        false,
        Some(tokio_tungstenite::Connector::Rustls(tls)),
    );
    let (socket, _) = timeout(config.timeout, connection)
        .await
        .map_err(|_| {
            TransportError::new(
                TransportErrorKind::Timeout,
                RequestOutcome::NotSent,
                "WebSocket connection timed out",
            )
        })?
        .map_err(|_| {
            TransportError::new(
                TransportErrorKind::Connection,
                RequestOutcome::NotSent,
                "WebSocket connection failed",
            )
        })?;
    Ok(socket)
}
impl StandardTransport {
    async fn session(&self, url: &str, scope: String) -> Result<Session, TransportError> {
        let _connecting = OperationGuard::start(&self.lifecycle)?;
        let mut shutdown = self.lifecycle.closed.subscribe();
        self.lifecycle.check()?;
        let key = (url.to_owned(), scope);
        let mut sessions = self.sessions.lock().await;
        self.lifecycle.check()?;
        if let Some(session) = sessions.get(&key).filter(|s| !s.commands.is_closed()) {
            return Ok(session.clone());
        }
        sessions.retain(|_, s| !s.commands.is_closed());
        let socket = tokio::select! {biased;_=shutdown.changed()=>return Err(closed(RequestOutcome::NotSent)),result=connect(url,self.tls.clone(),&self.config)=>result?};
        self.lifecycle.check()?;
        let (sender, receiver) = mpsc::unbounded_channel();
        let session = Session { commands: sender };
        let guard = OperationGuard::start(&self.lifecycle)?;
        tokio::spawn(run(
            socket,
            receiver,
            url.to_owned(),
            self.tls.clone(),
            self.config.clone(),
            shutdown,
            guard,
        ));
        sessions.insert(key, session.clone());
        Ok(session)
    }
}
impl WebSocketTransport for StandardTransport {
    type Stream = StandardSubscription;
    async fn subscribe<S: WebSocketSubscription>(
        &self,
        url: &str,
        spec: S,
    ) -> Result<Self::Stream, TransportError> {
        let spec: Arc<dyn WebSocketSubscription> = Arc::new(spec);
        let mut scope = spec.connection_scope();
        for _ in 0..3 {
            let session = self.session(url, scope.clone()).await?;
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            let mailbox = Mailbox::new(self.config.subscription_capacity);
            let mut subscription = StandardSubscription {
                id,
                commands: session.commands.clone(),
                mailbox: mailbox.clone(),
                timeout: self.config.timeout,
                ended: false,
                detached: false,
            };
            let (done, completed) = oneshot::channel();
            if session
                .commands
                .send(Command::Add {
                    id,
                    spec: spec.clone(),
                    peer: mailbox,
                    done,
                })
                .is_err()
            {
                continue;
            }
            match timeout(self.config.timeout, completed).await {
                Ok(Ok(result)) => match result? {
                    Attachment::Attached => return Ok(subscription),
                    Attachment::FreshConnection => {
                        subscription.detached = true;
                        scope = format!("{}\0fresh:{id}", spec.connection_scope());
                    }
                },
                Ok(Err(_)) => continue,
                Err(_) => return Err(timed_out()),
            }
        }
        Err(disconnected())
    }
}
async fn send(
    socket: &mut Socket,
    text: String,
    config: &TransportConfig,
) -> Result<(), TransportError> {
    timeout(config.timeout, socket.send(Message::Text(text.into())))
        .await
        .map_err(|_| timed_out())?
        .map_err(|_| disconnected())
}
// Called once before the first subscription, and again before reconnect replay.
// One deadline covers signer callbacks, sending, and acknowledgement; shutdown
// cancels the entire operation, including caller-supplied signing futures.
async fn authenticate(
    socket: &mut Socket,
    spec: &dyn WebSocketSubscription,
    config: &TransportConfig,
    shutdown: &mut watch::Receiver<bool>,
) -> Result<Option<String>, TransportError> {
    let setup = async {
        let Some(message) = spec.authentication_message().await? else {
            return Ok(None);
        };
        send(socket, message, config).await?;
        loop {
            match socket.next().await {
                Some(Ok(Message::Text(text))) => {
                    if let Some(reply) = spec.response_message(&text) {
                        send(socket, reply, config).await?;
                    }
                    if let Some(result) = spec.authentication_result(&text) {
                        result?;
                        if !spec.resubscribe_allowed() {
                            return Err(auth_expired());
                        }
                        return Ok(Some(text.to_string()));
                    }
                }
                Some(Ok(Message::Binary(bytes))) => {
                    let text = spec.decode_binary_message(&bytes)?;
                    if text.len() > config.max_message_bytes {
                        return Err(TransportError::new(
                            TransportErrorKind::ResponseTooLarge,
                            RequestOutcome::Unknown,
                            "decoded authentication frame exceeded limit",
                        ));
                    }
                    if spec.is_data_message(&text) {
                        return Err(protocol(
                            "channel data arrived before authentication completed",
                        ));
                    }
                }
                Some(Ok(Message::Ping(_))) => {
                    socket.flush().await.map_err(|_| disconnected())?;
                }
                Some(Ok(Message::Pong(_))) => {}
                Some(Ok(Message::Close(_))) | None => return Err(disconnected()),
                Some(Err(WsError::Capacity(_))) => {
                    return Err(TransportError::new(
                        TransportErrorKind::ResponseTooLarge,
                        RequestOutcome::Unknown,
                        "authentication frame exceeded limit",
                    ));
                }
                Some(Err(_)) => return Err(disconnected()),
                _ => return Err(protocol("invalid authentication frame")),
            }
        }
    };
    tokio::select! { biased;
        _ = shutdown.changed() => Err(closed(RequestOutcome::NotSent)),
        result = timeout(config.timeout, setup) => result.map_err(|_| timed_out())?,
    }
}
fn detach(entries: &mut HashMap<String, Entry>, id: u64) -> Option<Arc<dyn WebSocketSubscription>> {
    let key = entries
        .iter()
        .find_map(|(key, e)| e.peers.contains_key(&id).then(|| key.clone()))?;
    let entry = entries.get_mut(&key).expect("located entry");
    if let Some(peer) = entry.peers.remove(&id) {
        peer.finish(None);
    }
    if entry.peers.is_empty() {
        Some(entries.remove(&key).expect("located entry").spec)
    } else {
        None
    }
}
fn prune_expired(entries: &mut HashMap<String, Entry>) {
    entries.retain(|_, entry| {
        if entry.spec.resubscribe_allowed() {
            true
        } else {
            for peer in entry.peers.values() {
                peer.finish(Some(auth_expired()));
            }
            false
        }
    });
}
fn offline_command(
    entries: &mut HashMap<String, Entry>,
    command: Option<Command>,
) -> Result<(), Option<Completion>> {
    match command {
        Some(Command::Remove { id, done }) => {
            detach(entries, id);
            if entries.is_empty() {
                return Err(done);
            }
            if let Some(done) = done {
                let _ = done.send(Ok(()));
            }
        }
        // New consumers need not wait for an old session's recovery budget.
        Some(Command::Add { done, .. }) => {
            let _ = done.send(Ok(Attachment::FreshConnection));
        }
        None => return Err(None),
    }
    Ok(())
}
/// Run one connected period. A healthy period requires actual native data,
/// rather than just greetings/acks, to reset the consecutive reconnect budget.
async fn connected(
    socket: &mut Socket,
    entries: &mut HashMap<String, Entry>,
    commands: &mut mpsc::UnboundedReceiver<Command>,
    config: &TransportConfig,
    shutdown: &mut watch::Receiver<bool>,
) -> (Option<TransportError>, bool, Option<Completion>) {
    let mut healthy = false;
    let mut initialized = !entries.is_empty();
    let mut last_received = Instant::now();
    let mut heartbeat = tokio::time::interval_at(
        Instant::now() + config.heartbeat_interval,
        config.heartbeat_interval,
    );
    heartbeat.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    loop {
        let owner = entries.values().next().map(|e| e.spec.clone());
        let changed = async {
            if let Some(spec) = &owner {
                let changed = spec.authentication_changed();
                // Register before checking validity to avoid missing a revoke
                // between construction of this wait and its first poll.
                if !spec.resubscribe_allowed() {
                    return;
                }
                if let Some(future) = changed {
                    future.await;
                    return;
                }
            }
            std::future::pending::<()>().await;
        };
        let expiry = entries
            .values()
            .filter_map(|e| e.spec.authentication_expires_at())
            .min();
        let expired = async {
            if let Some(deadline) = expiry {
                tokio::time::sleep(
                    deadline
                        .duration_since(std::time::SystemTime::now())
                        .unwrap_or_default(),
                )
                .await;
            } else {
                std::future::pending::<()>().await;
            }
        };
        tokio::select! {biased;
            _=shutdown.changed()=>return (Some(closed(RequestOutcome::Unknown)),healthy,None),
            _=changed=>{ prune_expired(entries); if entries.is_empty(){return(None,healthy,None);} },
            _=expired=>{ prune_expired(entries); if entries.is_empty(){return(None,healthy,None);} },
            command=commands.recv()=>match command {
                Some(Command::Add{id,spec,peer,done})=>{
                    if done.is_closed() { continue; }
                    let key=spec.key();
                    if entries.contains_key(&key)&&spec.requires_fresh_snapshot(){let _=done.send(Ok(Attachment::FreshConnection));continue;}
                    if !spec.resubscribe_allowed(){let _=done.send(Err(auth_expired()));continue;}
                    if !initialized {
                        match authenticate(socket, &*spec, config, shutdown).await {
                            Ok(Some(ack)) => { peer.push(&ack); }
                            Ok(None) => {}
                            Err(error) => { let _=done.send(Err(error.clone())); return(Some(error),healthy,None); }
                        }
                        initialized=true;
                    }
                    if !entries.contains_key(&key){
                        if let Err(error)=send(socket,spec.subscribe_message(),config).await{let _=done.send(Err(error.clone()));return(Some(error),healthy,None);}
                        entries.insert(key.clone(),Entry{awaiting_snapshot:spec.requires_fresh_snapshot(),spec,peers:HashMap::new()});
                    }
                    entries.get_mut(&key).expect("inserted entry").peers.insert(id,peer);let _=done.send(Ok(Attachment::Attached));
                },
                Some(Command::Remove{id,done})=>{
                    if let Some(spec)=detach(entries,id) && let Err(error)=send(socket,spec.unsubscribe_message(),config).await{if let Some(done)=done{let _=done.send(Err(error.clone()));}return(Some(error),healthy,None);}
                    if entries.is_empty(){return(None,healthy,done);}
                    if let Some(done)=done{let _=done.send(Ok(()));}
                },
                None=>return(None,healthy,None),
            },
            message=socket.next()=>{
                let previously_active=!entries.is_empty();
                prune_expired(entries);
                if previously_active && entries.is_empty() { return(None,healthy,None); }
                last_received=Instant::now();
                let text=match message {
                    Some(Ok(Message::Text(text)))=>text.to_string(),
                    Some(Ok(Message::Ping(_)))=>{if timeout(config.timeout,socket.flush()).await.map_or(true,|r|r.is_err()){return(Some(disconnected()),healthy,None);}continue;},
                    Some(Ok(Message::Pong(_)))=>continue,
                    Some(Ok(Message::Binary(bytes)))=>match owner.as_ref().map(|s|s.decode_binary_message(&bytes)) {
                        Some(Ok(text))=>{if text.len()>config.max_message_bytes{return(Some(TransportError::new(TransportErrorKind::ResponseTooLarge,RequestOutcome::Unknown,"decoded WebSocket frame exceeded configured limit")),healthy,None);}text},
                        Some(Err(error))=>return(Some(error),healthy,None),
                        None=>return(Some(protocol("binary frame without a subscription codec")),healthy,None),
                    },
                    Some(Ok(Message::Frame(_)))=>return(Some(protocol("unexpected raw WebSocket frame")),healthy,None),
                    Some(Err(WsError::Capacity(_)))=>return(Some(TransportError::new(TransportErrorKind::ResponseTooLarge,RequestOutcome::Unknown,"WebSocket frame exceeded configured limit")),healthy,None),
                    Some(Err(WsError::Protocol(_)))|Some(Err(WsError::Utf8(_)))=>return(Some(protocol("invalid WebSocket protocol frame")),healthy,None),
                    Some(Ok(Message::Close(_)))|Some(Err(_))|None=>return(Some(disconnected()),healthy,None),
                };
                let had_entries=!entries.is_empty();
                let replies:HashSet<_>=entries.values().filter_map(|e|e.spec.response_message(&text)).collect();
                for reply in replies{if let Err(error)=send(socket,reply,config).await{return(Some(error),healthy,None);}}
                let mut removed=Vec::new();
                for entry in entries.values_mut(){
                    if !entry.spec.matches(&text){continue;}
                    if entry.awaiting_snapshot&&!entry.spec.is_snapshot_control(&text)&&!entry.spec.is_initial_snapshot(&text){
                        for (id,peer) in &entry.peers{peer.finish(Some(protocol("native delta arrived before subscription snapshot")));removed.push(*id);}continue;
                    }
                    if !entry.spec.is_snapshot_control(&text) { entry.awaiting_snapshot=false; }
                    for (id,peer) in &entry.peers{if !peer.push(&text){removed.push(*id);}else if entry.spec.is_data_message(&text){healthy=true;}}
                }
                for id in removed{if let Some(spec)=detach(entries,id) && let Err(error)=send(socket,spec.unsubscribe_message(),config).await{return(Some(error),healthy,None);}}
                if had_entries&&entries.is_empty(){return(None,healthy,None);}
            },
            _=heartbeat.tick()=>{
                if last_received.elapsed()>=config.idle_timeout{return(Some(timed_out()),healthy,None);}
                let messages:HashSet<_>=entries.values().filter_map(|e|e.spec.heartbeat_message()).collect();
                for message in messages{if let Err(error)=send(socket,message,config).await{return(Some(error),healthy,None);}}
            },
        }
    }
}
async fn run(
    mut socket: Socket,
    mut commands: mpsc::UnboundedReceiver<Command>,
    url: String,
    tls: Arc<rustls::ClientConfig>,
    config: TransportConfig,
    mut shutdown: watch::Receiver<bool>,
    _guard: OperationGuard,
) {
    let mut entries: HashMap<String, Entry> = HashMap::new();
    let mut attempts = 0;
    let mut closing_done = None;
    let failure = 'session: loop {
        if *shutdown.borrow() {
            break Some(closed(RequestOutcome::Unknown));
        }
        let connected_since = Instant::now();
        let (failure, healthy, done) = connected(
            &mut socket,
            &mut entries,
            &mut commands,
            &config,
            &mut shutdown,
        )
        .await;
        if done.is_some() {
            closing_done = done;
        }
        let Some(error) = failure else {
            break None;
        };
        if healthy || connected_since.elapsed() >= config.reconnect_stable_after {
            attempts = 0;
        }
        if entries.is_empty()
            || !matches!(
                error.kind,
                TransportErrorKind::Disconnected | TransportErrorKind::Timeout
            )
            || attempts >= config.reconnect.max_retries
        {
            break Some(error);
        }
        for entry in entries.values_mut() {
            entry.awaiting_snapshot = entry.spec.requires_fresh_snapshot();
            for peer in entry.peers.values() {
                peer.gap();
            }
        }
        // Do not wait for an unresponsive peer's close handshake before recovery.
        // The disconnected socket is replaced/dropped on the next successful dial.
        loop {
            prune_expired(&mut entries);
            if entries.is_empty() {
                break 'session None;
            }
            if attempts >= config.reconnect.max_retries {
                break 'session Some(disconnected());
            }
            let delay = config.reconnect.delay(attempts);
            attempts += 1;
            let pause = tokio::time::sleep(delay);
            tokio::pin!(pause);
            loop {
                tokio::select! {biased;
                    _=shutdown.changed()=>break 'session Some(closed(RequestOutcome::Unknown)),
                    command=commands.recv()=>if let Err(done)=offline_command(&mut entries,command){closing_done=done;break 'session None;},
                    _=&mut pause=>break,
                }
            }
            let dial = connect(&url, tls.clone(), &config);
            tokio::pin!(dial);
            let candidate = loop {
                tokio::select! {biased;
                    _=shutdown.changed()=>break 'session Some(closed(RequestOutcome::Unknown)),
                    command=commands.recv()=>if let Err(done)=offline_command(&mut entries,command){closing_done=done;break 'session None;},
                    result=&mut dial=>break result,
                }
            };
            let Ok(mut candidate) = candidate else {
                continue;
            };
            prune_expired(&mut entries);
            if entries.is_empty() {
                socket = candidate;
                break 'session None;
            }
            let spec = entries
                .values()
                .next()
                .expect("nonempty entries")
                .spec
                .clone();
            let authenticated_ack =
                match authenticate(&mut candidate, &*spec, &config, &mut shutdown).await {
                    Ok(ack) => ack,
                    Err(error)
                        if matches!(
                            error.kind,
                            TransportErrorKind::Authentication
                                | TransportErrorKind::Protocol
                                | TransportErrorKind::ResponseTooLarge
                                | TransportErrorKind::Closed
                        ) =>
                    {
                        break 'session Some(error);
                    }
                    Err(_) => continue,
                };
            let restore = timeout(config.timeout, async {
                for entry in entries.values() {
                    candidate
                        .send(Message::Text(entry.spec.subscribe_message().into()))
                        .await?;
                }
                Ok::<(), WsError>(())
            });
            let restored = tokio::select! {biased;_=shutdown.changed()=>break 'session Some(closed(RequestOutcome::Unknown)),result=restore=>matches!(result,Ok(Ok(())))};
            if !restored {
                continue;
            }
            if let Some(ack) = authenticated_ack {
                for entry in entries.values() {
                    for peer in entry.peers.values() {
                        peer.push(&ack);
                    }
                }
            }
            socket = candidate;
            break;
        }
    };
    commands.close();
    if *shutdown.borrow() {
        // Explicit transport shutdown still detaches active native subscriptions.
        // One total deadline prevents many entries multiplying the close budget.
        let _ = timeout(config.timeout, async {
            for entry in entries.values() {
                if socket
                    .send(Message::Text(entry.spec.unsubscribe_message().into()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        })
        .await;
    }
    for entry in entries.values() {
        for peer in entry.peers.values() {
            peer.finish(failure.clone());
        }
    }
    let _ = timeout(config.timeout, socket.close(None)).await;
    if let Some(done) = closing_done {
        let _ = done.send(Ok(()));
    }
}
