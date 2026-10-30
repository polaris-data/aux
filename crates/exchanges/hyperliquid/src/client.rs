use crate::Hyperliquid;
use aux_rs_core::{DefaultTransport, Endpoint, Error, HttpTransport, Public, Unauthenticated};

/// A client for Hyperliquid's native HTTP and WebSocket APIs.
///
/// Use `testnet` or `mainnet` with the built-in transports, or `with_transport`
/// for custom I/O. Authentication is attached with `with_auth`; trading methods
/// are available only when that authentication implements the native contract.
#[derive(Clone)]
pub struct HyperliquidClient<T = DefaultTransport, A = Unauthenticated> {
    exchange: Hyperliquid,
    transport: T,
    auth: A,
}

impl<T, A> std::fmt::Debug for HyperliquidClient<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyperliquidClient").finish_non_exhaustive()
    }
}

impl<T> HyperliquidClient<T> {
    /// Use a caller-owned transport and its configuration/lifecycle policy.
    pub fn with_transport(exchange: Hyperliquid, transport: T) -> Self {
        Self {
            exchange,
            transport,
            auth: Unauthenticated,
        }
    }
}

impl<T, A> HyperliquidClient<T, A> {
    pub fn exchange(&self) -> &Hyperliquid {
        &self.exchange
    }
    pub fn transport(&self) -> &T {
        &self.transport
    }
    pub fn auth(&self) -> &A {
        &self.auth
    }

    /// Attach native authentication while retaining the existing transport.
    pub fn with_auth<B>(self, auth: B) -> HyperliquidClient<T, B> {
        HyperliquidClient {
            exchange: self.exchange,
            transport: self.transport,
            auth,
        }
    }
}

impl<T: HttpTransport, A: Sync> HyperliquidClient<T, A> {
    /// Execute a typed native public endpoint. Protected operations have their
    /// own authentication-bound methods.
    pub async fn request<R: Endpoint<Hyperliquid, Access = Public>>(
        &self,
        request: R,
    ) -> Result<R::Response, Error<crate::Error>> {
        aux_rs_core::execute_request(&self.exchange, &self.transport, request).await
    }
}

#[cfg(any(feature = "http", feature = "ws"))]
impl HyperliquidClient<aux_rs_core::StandardTransport> {
    /// Construct a client with default transport settings. No exchange requests
    /// are made until an operation is called.
    pub fn new(exchange: Hyperliquid) -> Result<Self, aux_rs_core::TransportError> {
        Self::with_config(exchange, aux_rs_core::TransportConfig::default())
    }

    pub fn testnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Hyperliquid::testnet())
    }

    pub fn mainnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Hyperliquid::mainnet())
    }

    pub fn with_config(
        exchange: Hyperliquid,
        config: aux_rs_core::TransportConfig,
    ) -> Result<Self, aux_rs_core::TransportError> {
        Ok(Self::with_transport(
            exchange,
            aux_rs_core::StandardTransport::new(config)?,
        ))
    }
}
