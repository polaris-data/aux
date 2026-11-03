use crate::{Error, Monaco};
use aux_rs_core::{
    DefaultTransport, Endpoint, Error as ClientError, HttpTransport, Public, Unauthenticated,
};
/// Native Monaco client; authenticated methods require attached credentials.
///
/// ```compile_fail
/// use aux_rs_core::HttpTransport;
/// use aux_rs_monaco::MonacoClient;
/// fn unsigned<T: HttpTransport>(client: MonacoClient<T>, market: &aux_rs_monaco::TradingPairData, order: aux_rs_monaco::CreateOrderRequest) {
///     let _ = client.place_order(market, order);
/// }
/// ```
#[derive(Clone)]
pub struct MonacoClient<T = DefaultTransport, A = Unauthenticated> {
    pub(crate) exchange: Monaco,
    pub(crate) transport: T,
    pub(crate) auth: A,
}
impl<T, A> std::fmt::Debug for MonacoClient<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MonacoClient").finish_non_exhaustive()
    }
}
impl<T> MonacoClient<T> {
    pub fn with_transport(exchange: Monaco, transport: T) -> Self {
        Self {
            exchange,
            transport,
            auth: Unauthenticated,
        }
    }
}
impl<T, A> MonacoClient<T, A> {
    pub fn exchange(&self) -> &Monaco {
        &self.exchange
    }
    pub fn transport(&self) -> &T {
        &self.transport
    }
    pub fn auth(&self) -> &A {
        &self.auth
    }
    pub fn with_auth<B>(self, auth: B) -> MonacoClient<T, B> {
        MonacoClient {
            exchange: self.exchange,
            transport: self.transport,
            auth,
        }
    }
}
impl<T: HttpTransport, A: Sync> MonacoClient<T, A> {
    pub async fn request<R: Endpoint<Monaco, Access = Public>>(
        &self,
        request: R,
    ) -> Result<R::Response, ClientError<Error>> {
        aux_rs_core::execute_request(&self.exchange, &self.transport, request).await
    }
}
#[cfg(any(feature = "http", feature = "ws"))]
impl MonacoClient<aux_rs_core::StandardTransport> {
    pub fn new(exchange: Monaco) -> Result<Self, aux_rs_core::TransportError> {
        Self::with_config(exchange, aux_rs_core::TransportConfig::default())
    }
    pub fn testnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Monaco::testnet())
    }
    pub fn mainnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Monaco::mainnet())
    }
    pub fn with_config(
        exchange: Monaco,
        config: aux_rs_core::TransportConfig,
    ) -> Result<Self, aux_rs_core::TransportError> {
        let client = Self::with_transport(exchange, aux_rs_core::StandardTransport::new(config)?);
        Ok(client)
    }
}
