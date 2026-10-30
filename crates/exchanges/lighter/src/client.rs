use crate::Lighter;
use aux_rs_core::{DefaultTransport, Endpoint, Error, HttpTransport, Public, Unauthenticated};

/// A client for Lighter's native HTTP and WebSocket APIs.
///
/// Use `testnet` or `mainnet` with the built-in transports, or `with_transport`
/// for custom I/O. Authentication is attached with `with_auth`; trading methods
/// are available only when that authentication implements the native contract.
#[derive(Clone)]
pub struct LighterClient<T = DefaultTransport, A = Unauthenticated> {
    exchange: Lighter,
    transport: T,
    auth: A,
}

impl<T, A> std::fmt::Debug for LighterClient<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LighterClient").finish_non_exhaustive()
    }
}

impl<T> LighterClient<T> {
    /// Use a caller-owned transport and its configuration/lifecycle policy.
    pub fn with_transport(exchange: Lighter, transport: T) -> Self {
        Self {
            exchange,
            transport,
            auth: Unauthenticated,
        }
    }
}

impl<T, A> LighterClient<T, A> {
    pub fn exchange(&self) -> &Lighter {
        &self.exchange
    }
    pub fn transport(&self) -> &T {
        &self.transport
    }
    pub fn auth(&self) -> &A {
        &self.auth
    }

    /// Attach native authentication while retaining the existing transport.
    pub fn with_auth<B>(self, auth: B) -> LighterClient<T, B> {
        LighterClient {
            exchange: self.exchange,
            transport: self.transport,
            auth,
        }
    }
}

impl<T: HttpTransport, A: Sync> LighterClient<T, A> {
    /// Execute a typed native public endpoint. Protected operations have their
    /// own authentication-bound methods.
    pub async fn request<R: Endpoint<Lighter, Access = Public>>(
        &self,
        request: R,
    ) -> Result<R::Response, Error<crate::Error>> {
        aux_rs_core::execute_request(&self.exchange, &self.transport, request).await
    }
}

#[cfg(any(feature = "http", feature = "ws"))]
impl LighterClient<aux_rs_core::StandardTransport> {
    /// Construct a client with default transport settings. No exchange requests
    /// are made until an operation is called.
    pub fn new(exchange: Lighter) -> Result<Self, aux_rs_core::TransportError> {
        Self::with_config(exchange, aux_rs_core::TransportConfig::default())
    }

    pub fn testnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Lighter::testnet())
    }

    pub fn mainnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Lighter::mainnet())
    }

    pub fn with_config(
        exchange: Lighter,
        config: aux_rs_core::TransportConfig,
    ) -> Result<Self, aux_rs_core::TransportError> {
        Ok(Self::with_transport(
            exchange,
            aux_rs_core::StandardTransport::new(config)?,
        ))
    }
}
