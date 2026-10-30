use std::error::Error as StdError;

use crate::{Error, HttpRequest, HttpResponse, HttpTransport};

pub trait Exchange: Clone + Send + Sync + 'static {
    type Error: StdError + Send + Sync + 'static;
    fn http_url(&self) -> &str;
    fn ws_url(&self) -> &str;
}

/// Access marker for unsigned endpoints. Protected operations live in native
/// authentication-bound client methods; the generic executor only accepts Public.
#[derive(Clone, Copy, Debug)]
pub struct Public;

pub trait Endpoint<E: Exchange>: Send {
    /// Opt in only for semantically idempotent reads, including native POST reads.
    const SAFE_TO_RETRY: bool = false;
    type Response: Send;
    type Access;
    fn request(&self, exchange: &E) -> Result<HttpRequest, E::Error>;
    fn decode(response: HttpResponse) -> Result<Self::Response, E::Error>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Unauthenticated;

/// Shared executor for native public endpoints. Authenticated operations use
/// exchange-specific client methods instead.
#[doc(hidden)]
pub async fn execute_request<E, T, R>(
    exchange: &E,
    transport: &T,
    request: R,
) -> Result<R::Response, Error<E::Error>>
where
    E: Exchange,
    T: HttpTransport,
    R: Endpoint<E, Access = Public>,
{
    transport.ensure_open()?;
    let wire = request.request(exchange).map_err(Error::Exchange)?;
    let response = if R::SAFE_TO_RETRY {
        transport.execute_read(wire).await?
    } else {
        transport.execute(wire).await?
    };
    R::decode(response).map_err(Error::Exchange)
}

/// The built-in transport, when either networking feature is enabled.
#[cfg(any(feature = "http", feature = "ws"))]
pub use crate::StandardTransport as DefaultTransport;

/// No built-in transport is enabled. Construct an exchange client with
/// `with_transport` to supply your own I/O implementation.
#[cfg(not(any(feature = "http", feature = "ws")))]
#[derive(Clone, Copy, Debug)]
pub struct DefaultTransport;
