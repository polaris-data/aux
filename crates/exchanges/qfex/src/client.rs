use crate::{Error, Qfex};
use aux_rs_core::{
    DefaultTransport, Endpoint, Error as ClientError, HttpTransport, Public, Unauthenticated,
};
/// Native QFEX client; authenticated methods require attached credentials.
///
/// ```compile_fail
/// use aux_rs_core::WebSocketConnector;
/// use aux_rs_qfex::QfexClient;
/// fn unsigned<T: WebSocketConnector>(client: QfexClient<T>, order: aux_rs_qfex::AddOrder) {
///     let _ = client.add_order(order);
/// }
/// ```
#[derive(Clone)]
pub struct QfexClient<T = DefaultTransport, A = Unauthenticated> {
    pub(crate) exchange: Qfex,
    pub(crate) transport: T,
    pub(crate) auth: A,
    pub(crate) timeout: std::time::Duration,
}
impl<T, A> std::fmt::Debug for QfexClient<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QfexClient").finish_non_exhaustive()
    }
}
impl<T> QfexClient<T> {
    pub fn with_transport(exchange: Qfex, transport: T) -> Self {
        Self {
            exchange,
            transport,
            auth: Unauthenticated,
            timeout: std::time::Duration::from_secs(10),
        }
    }
}
impl<T, A> QfexClient<T, A> {
    pub fn exchange(&self) -> &Qfex {
        &self.exchange
    }
    pub fn transport(&self) -> &T {
        &self.transport
    }
    pub fn auth(&self) -> &A {
        &self.auth
    }
    pub fn with_auth<B>(self, auth: B) -> QfexClient<T, B> {
        QfexClient {
            exchange: self.exchange,
            transport: self.transport,
            auth,
            timeout: self.timeout,
        }
    }
}
impl<T: HttpTransport, A: Sync> QfexClient<T, A> {
    pub async fn request<R: Endpoint<Qfex, Access = Public>>(
        &self,
        request: R,
    ) -> Result<R::Response, ClientError<Error>> {
        aux_rs_core::execute_request(&self.exchange, &self.transport, request).await
    }
}
#[cfg(any(feature = "http", feature = "ws"))]
impl QfexClient<aux_rs_core::StandardTransport> {
    pub fn new(exchange: Qfex) -> Result<Self, aux_rs_core::TransportError> {
        Self::with_config(exchange, aux_rs_core::TransportConfig::default())
    }
    pub fn testnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Qfex::testnet())
    }
    pub fn mainnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Qfex::mainnet())
    }
    pub fn with_config(
        exchange: Qfex,
        config: aux_rs_core::TransportConfig,
    ) -> Result<Self, aux_rs_core::TransportError> {
        let timeout = config.timeout;
        let mut client =
            Self::with_transport(exchange, aux_rs_core::StandardTransport::new(config)?);
        client.timeout = timeout;
        Ok(client)
    }
}
