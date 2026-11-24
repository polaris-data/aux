//! Application-owned TLS for the Zig HTTP/1.1 and WebSocket clients.
use crate::{
    Error, Result,
    http::{OutboundRequest, Transport, TransportResponse},
};
use std::{
    os::{fd::OwnedFd, unix::net::UnixStream},
    sync::{Arc, OnceLock, mpsc},
    time::Duration,
};
use tokio::{
    io,
    net::TcpStream,
    runtime::{Builder, Runtime},
};
use tokio_rustls::{
    TlsConnector,
    rustls::{ClientConfig, pki_types::ServerName},
};

pub(crate) struct RustlsPolicy(TlsConnector);

impl RustlsPolicy {
    pub(crate) fn new(mut config: ClientConfig) -> Self {
        config.alpn_protocols = vec![b"http/1.1".to_vec()];
        Self(Arc::new(config).into())
    }
}

fn runtime() -> Result<&'static Runtime> {
    static RUNTIME: OnceLock<std::io::Result<Runtime>> = OnceLock::new();
    RUNTIME
        .get_or_init(|| {
            Builder::new_multi_thread()
                .worker_threads(2)
                .thread_name("aux-tls")
                .enable_io()
                .enable_time()
                .build()
        })
        .as_ref()
        .map_err(|_| Error::Network)
}

impl Transport for RustlsPolicy {
    fn send(&self, _: OutboundRequest) -> Result<TransportResponse> {
        Err(Error::InvalidRequest(
            "the TLS adapter requires native HTTP framing",
        ))
    }

    fn connect_stream(&self, host: &str, port: u16, tls: bool) -> Result<OwnedFd> {
        let host = host.to_owned();
        let connector = self.0.clone();
        let (ready, connection) = mpsc::sync_channel(1);
        runtime()?.spawn(async move {
            // The caller gets a socket only after certificate verification and the handshake.
            let establish = async {
                let tcp = TcpStream::connect((host.as_str(), port))
                    .await
                    .map_err(|_| Error::Network)?;
                if !tls {
                    let socket = tcp.into_std().map_err(|_| Error::Network)?;
                    socket.set_nonblocking(false).map_err(|_| Error::Network)?;
                    return Ok((socket.into(), None));
                }
                let name = ServerName::try_from(host)
                    .map_err(|_| Error::InvalidRequest("invalid TLS hostname"))?;
                let session = connector
                    .connect(name, tcp)
                    .await
                    .map_err(|_| Error::Network)?;
                let (native, bridge) = UnixStream::pair().map_err(|_| Error::Network)?;
                bridge.set_nonblocking(true).map_err(|_| Error::Network)?;
                let bridge =
                    tokio::net::UnixStream::from_std(bridge).map_err(|_| Error::Network)?;
                Ok((native.into(), Some((bridge, session))))
            };
            match tokio::time::timeout(Duration::from_secs(30), establish).await {
                Ok(Ok((fd, bridge))) => {
                    if ready.send(Ok(fd)).is_err() {
                        return;
                    }
                    if let Some((mut bridge, mut session)) = bridge {
                        // A close from either side drops both sockets, including a peer waiting for more data.
                        let (mut app_read, mut app_write) = io::split(&mut bridge);
                        let (mut tls_read, mut tls_write) = io::split(&mut session);
                        tokio::select! {
                            _=io::copy(&mut app_read,&mut tls_write)=>{},
                            _=io::copy(&mut tls_read,&mut app_write)=>{},
                        }
                    }
                }
                Ok(Err(e)) => {
                    let _ = ready.send(Err(e));
                }
                Err(_) => {
                    let _ = ready.send(Err(Error::Network));
                }
            }
        });
        connection.recv().map_err(|_| Error::Network)?
    }
}
