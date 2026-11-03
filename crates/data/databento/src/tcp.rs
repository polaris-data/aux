use crate::{ByteConnector, ByteWriter};
use aux_rs_core::{ByteStream, RequestOutcome, TransportError, TransportErrorKind};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::mpsc,
};
fn error(kind: TransportErrorKind) -> TransportError {
    TransportError::new(
        kind,
        RequestOutcome::Unknown,
        "Databento TCP connection failed",
    )
}
#[derive(Clone, Debug)]
pub struct TcpConnector {
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
}
impl Default for TcpConnector {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(10),
            idle_timeout: Duration::from_secs(60),
        }
    }
}
pub struct TcpReader {
    receiver: mpsc::Receiver<Result<Vec<u8>, TransportError>>,
    driver: tokio::task::JoinHandle<()>,
}
pub struct TcpWriter {
    writer: OwnedWriteHalf,
    timeout: Duration,
}
impl Drop for TcpReader {
    fn drop(&mut self) {
        self.driver.abort();
    }
}
impl ByteStream for TcpReader {
    async fn next(&mut self) -> Option<Result<Vec<u8>, TransportError>> {
        self.receiver.recv().await
    }
    async fn close(&mut self) -> Result<(), TransportError> {
        self.driver.abort();
        let _ = (&mut self.driver).await;
        self.receiver.close();
        while self.receiver.try_recv().is_ok() {}
        Ok(())
    }
}
impl ByteWriter for TcpWriter {
    async fn write(&mut self, bytes: &[u8]) -> Result<(), TransportError> {
        tokio::time::timeout(self.timeout, self.writer.write_all(bytes))
            .await
            .map_err(|_| error(TransportErrorKind::Timeout))?
            .map_err(|_| error(TransportErrorKind::Connection))
    }
    async fn close(&mut self) -> Result<(), TransportError> {
        tokio::time::timeout(self.timeout, self.writer.shutdown())
            .await
            .map_err(|_| error(TransportErrorKind::Timeout))?
            .map_err(|_| error(TransportErrorKind::Connection))
    }
}
impl ByteConnector for TcpConnector {
    type Reader = TcpReader;
    type Writer = TcpWriter;
    async fn connect(&self, gateway: &str) -> Result<(Self::Reader, Self::Writer), TransportError> {
        if self.connect_timeout.is_zero() || self.idle_timeout.is_zero() {
            return Err(error(TransportErrorKind::Configuration));
        }
        let socket = tokio::time::timeout(self.connect_timeout, TcpStream::connect(gateway))
            .await
            .map_err(|_| error(TransportErrorKind::Timeout))?
            .map_err(|_| error(TransportErrorKind::Connection))?;
        socket
            .set_nodelay(true)
            .map_err(|_| error(TransportErrorKind::Connection))?;
        let (reader, writer) = socket.into_split();
        let (sender, receiver) = mpsc::channel(1);
        let timeout = self.idle_timeout;
        let driver = tokio::spawn(read(reader, sender, timeout));
        Ok((
            TcpReader { receiver, driver },
            TcpWriter {
                writer,
                timeout: self.connect_timeout,
            },
        ))
    }
}
async fn read(
    mut reader: OwnedReadHalf,
    sender: mpsc::Sender<Result<Vec<u8>, TransportError>>,
    timeout: Duration,
) {
    loop {
        let mut bytes = vec![0; 64 * 1024];
        let (result, terminal) = match tokio::time::timeout(timeout, reader.read(&mut bytes)).await
        {
            Ok(Ok(0)) => (Err(error(TransportErrorKind::Disconnected)), true),
            Ok(Ok(n)) => {
                bytes.truncate(n);
                (Ok(bytes), false)
            }
            Ok(Err(_)) => (Err(error(TransportErrorKind::Connection)), true),
            Err(_) => (Err(error(TransportErrorKind::Timeout)), true),
        };
        if sender.send(result).await.is_err() || terminal {
            break;
        }
    }
}
