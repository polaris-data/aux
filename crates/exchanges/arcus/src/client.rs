use crate::{Arcus, Error};
use aux_rs_core::{
    DefaultTransport, Endpoint, Error as ClientError, HttpTransport, Public, Unauthenticated,
};
/// Native Arcus client; authenticated methods require attached credentials.
///
/// ```compile_fail
/// use aux_rs_core::HttpTransport;
/// use aux_rs_arcus::ArcusClient;
/// fn unsigned<T: HttpTransport>(client: ArcusClient<T>, market: &aux_rs_arcus::MarketInfo, order: aux_rs_arcus::OrderRequest) {
///     let _ = client.place_order(market, order);
/// }
/// ```
#[derive(Clone)]
pub struct ArcusClient<T = DefaultTransport, A = Unauthenticated> {
    pub(crate) exchange: Arcus,
    pub(crate) transport: T,
    pub(crate) auth: A,
}
impl<T, A> std::fmt::Debug for ArcusClient<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArcusClient").finish_non_exhaustive()
    }
}
impl<T> ArcusClient<T> {
    pub fn with_transport(exchange: Arcus, transport: T) -> Self {
        Self {
            exchange,
            transport,
            auth: Unauthenticated,
        }
    }
}
impl<T, A> ArcusClient<T, A> {
    pub fn exchange(&self) -> &Arcus {
        &self.exchange
    }
    pub fn transport(&self) -> &T {
        &self.transport
    }
    pub fn auth(&self) -> &A {
        &self.auth
    }
    pub fn with_auth<B>(self, auth: B) -> ArcusClient<T, B> {
        ArcusClient {
            exchange: self.exchange,
            transport: self.transport,
            auth,
        }
    }
}
impl<T: HttpTransport, A: Sync> ArcusClient<T, A> {
    pub async fn request<R: Endpoint<Arcus, Access = Public>>(
        &self,
        request: R,
    ) -> Result<R::Response, ClientError<Error>> {
        aux_rs_core::execute_request(&self.exchange, &self.transport, request).await
    }
}
#[cfg(any(feature = "http", feature = "ws"))]
impl ArcusClient<aux_rs_core::StandardTransport> {
    pub fn new(exchange: Arcus) -> Result<Self, aux_rs_core::TransportError> {
        Self::with_config(exchange, aux_rs_core::TransportConfig::default())
    }
    pub fn testnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Arcus::testnet())
    }
    pub fn mainnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Arcus::mainnet())
    }
    pub fn with_config(
        exchange: Arcus,
        config: aux_rs_core::TransportConfig,
    ) -> Result<Self, aux_rs_core::TransportError> {
        let client = Self::with_transport(exchange, aux_rs_core::StandardTransport::new(config)?);
        Ok(client)
    }
}
