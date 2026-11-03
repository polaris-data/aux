#[cfg(feature = "tcp")]
use aux_rs_core::TransportErrorKind;
use aux_rs_core::{ByteStream, TransportError};
use aux_rs_databento::{
    ByteConnector, ByteWriter, DbnEvent, LiveClient, SessionOptions, Subscription,
};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
#[derive(Clone)]
struct Connector {
    bytes: Vec<u8>,
    writes: Arc<Mutex<Vec<Vec<u8>>>>,
}
struct Reader(VecDeque<Vec<u8>>);
struct Writer(Arc<Mutex<Vec<Vec<u8>>>>);
impl ByteStream for Reader {
    async fn next(&mut self) -> Option<Result<Vec<u8>, TransportError>> {
        self.0.pop_front().map(Ok)
    }
    async fn close(&mut self) -> Result<(), TransportError> {
        self.0.clear();
        Ok(())
    }
}
impl ByteWriter for Writer {
    async fn write(&mut self, bytes: &[u8]) -> Result<(), TransportError> {
        self.0.lock().unwrap().push(bytes.to_vec());
        Ok(())
    }
    async fn close(&mut self) -> Result<(), TransportError> {
        Ok(())
    }
}
impl ByteConnector for Connector {
    type Reader = Reader;
    type Writer = Writer;
    async fn connect(&self, _: &str) -> Result<(Reader, Writer), TransportError> {
        Ok((
            Reader(self.bytes.chunks(3).map(|b| b.to_vec()).collect()),
            Writer(self.writes.clone()),
        ))
    }
}
const KEY: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaa12345";
fn subscription() -> Subscription {
    Subscription {
        schema: "trades".into(),
        stype_in: "raw_symbol".into(),
        symbols: vec!["ESZ4".into()],
        start: None,
        snapshot: false,
        id: Some(7),
    }
}
#[tokio::test]
async fn live_protocol_inventory() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/live.json")).unwrap();
    let mut bytes =
        b"lsg_version=1\ncram=fixture-challenge\nsuccess=1|session_id=fixture-session\n".to_vec();
    bytes.extend(include_bytes!("fixtures/ts_out.dbn"));
    let connector = Connector {
        bytes,
        writes: Arc::default(),
    };
    let writes = connector.writes.clone();
    let client = LiveClient::with_connector(KEY, connector).unwrap();
    let mut options = SessionOptions::new("GLBX.MDP3");
    options.ts_out = true;
    options.heartbeat_interval_s = Some(5);
    options.slow_reader_behavior = Some("warn".into());
    let mut session = client.connect(options).await.unwrap();
    assert_eq!(session.session_id, "fixture-session");
    session.subscribe(subscription()).await.unwrap();
    session.start_session().await.unwrap();
    assert!(matches!(
        session.next().await.unwrap().unwrap(),
        DbnEvent::Metadata(_)
    ));
    match session.next().await.unwrap().unwrap() {
        DbnEvent::Record(r) => assert_eq!(r.ts_out, Some(1704067200000000999)),
        _ => panic!("expected record"),
    }
    session.unsubscribe(subscription()).await.unwrap();
    session.close().await.unwrap();
    assert!(session.next().await.is_none());
    let actual = writes
        .lock()
        .unwrap()
        .iter()
        .map(|b| String::from_utf8(b.clone()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        actual,
        fixture["commands"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_owned())
            .collect::<Vec<_>>()
    );
}
#[tokio::test]
async fn live_authentication_and_validation_failures() {
    let connector = Connector {
        bytes: format!("hello\ncram=x\nsuccess=0|error=invalid {KEY}\n").into_bytes(),
        writes: Arc::default(),
    };
    let client = LiveClient::with_connector(KEY, connector).unwrap();
    let error = match client.connect(SessionOptions::new("GLBX.MDP3")).await {
        Err(e) => e,
        Ok(_) => panic!("authentication must fail"),
    };
    assert_eq!(error.kind(), "authentication");
    assert!(!format!("{error:?}").contains(KEY));
    assert!(
        LiveClient::with_connector(
            "bad",
            Connector {
                bytes: Vec::new(),
                writes: Arc::default()
            }
        )
        .is_err()
    );
    let mut options = SessionOptions::new("GLBX.MDP3\nmalicious");
    assert!(client.connect(options.clone()).await.is_err());
    options.dataset = "GLBX.MDP3".into();
    options.compression = "gzip".into();
    assert!(client.connect(options).await.is_err());
}
#[cfg(feature = "tcp")]
#[tokio::test]
async fn actual_tcp_fragments_disconnect_and_shutdown() {
    use tokio::{
        io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
        net::TcpListener,
    };
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        socket
            .write_all(b"hello\ncram=fixture-challenge\n")
            .await
            .unwrap();
        let mut socket = BufReader::new(socket);
        let mut line = String::new();
        socket.read_line(&mut line).await.unwrap();
        assert!(line.starts_with("auth="));
        socket
            .get_mut()
            .write_all(b"success=1|session_id=local\n")
            .await
            .unwrap();
        line.clear();
        socket.read_line(&mut line).await.unwrap();
        assert!(line.starts_with("schema=trades|"));
        line.clear();
        socket.read_line(&mut line).await.unwrap();
        assert_eq!(line, "start_session\n");
        for bytes in include_bytes!("fixtures/ts_out.dbn").chunks(7) {
            socket.get_mut().write_all(bytes).await.unwrap();
        }
        socket.get_mut().shutdown().await.unwrap();
    });
    let client = LiveClient::new(KEY).unwrap();
    let mut options = SessionOptions::new("GLBX.MDP3");
    options.gateway = Some(address.to_string());
    options.ts_out = true;
    let mut session = client.connect(options).await.unwrap();
    session.subscribe(subscription()).await.unwrap();
    session.start_session().await.unwrap();
    assert!(session.next().await.unwrap().is_ok());
    assert!(session.next().await.unwrap().is_ok());
    let error = session.next().await.unwrap().unwrap_err();
    assert!(
        matches!(error,aux_rs_databento::Error::Transport(e) if e.kind==TransportErrorKind::Disconnected)
    );
    assert!(session.next().await.is_none());
    session.close().await.unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn cancelled_command_quarantines_session_without_replay() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct Blocking(Arc<AtomicUsize>);
    struct BlockingWriter(Arc<AtomicUsize>);
    impl ByteWriter for BlockingWriter {
        async fn write(&mut self, _: &[u8]) -> Result<(), TransportError> {
            if self.0.fetch_add(1, Ordering::SeqCst) > 0 {
                std::future::pending::<()>().await;
            }
            Ok(())
        }
        async fn close(&mut self) -> Result<(), TransportError> {
            Ok(())
        }
    }
    impl ByteConnector for Blocking {
        type Reader = Reader;
        type Writer = BlockingWriter;
        async fn connect(&self, _: &str) -> Result<(Reader, BlockingWriter), TransportError> {
            Ok((
                Reader(vec![b"hello\ncram=challenge\nsuccess=1\n".to_vec()].into()),
                BlockingWriter(self.0.clone()),
            ))
        }
    }
    let writes = Arc::new(AtomicUsize::new(0));
    let client = LiveClient::with_connector(KEY, Blocking(writes.clone())).unwrap();
    let mut session = client
        .connect(SessionOptions::new("GLBX.MDP3"))
        .await
        .unwrap();
    assert!(
        tokio::time::timeout(
            std::time::Duration::from_millis(10),
            session.subscribe(subscription())
        )
        .await
        .is_err()
    );
    assert!(session.closed());
    assert!(session.subscribe(subscription()).await.is_err());
    assert_eq!(writes.load(Ordering::SeqCst), 2);
    session.close().await.unwrap();
}
