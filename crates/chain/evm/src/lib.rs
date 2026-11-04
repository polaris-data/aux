//! Venue-independent EVM transactions, RPC and signer-scoped nonce reconciliation.
mod chain;
mod error;
pub use chain::*;
pub use error::Error;
use serde::de::DeserializeOwned;
pub fn validate_url(s: &str) -> Result<(), Error> {
    let url = url::Url::parse(s).map_err(|_| Error::InvalidRequest("invalid endpoint URL"))?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(Error::InvalidRequest(
            "endpoint must be an HTTP origin without credentials, query or fragment",
        ));
    }
    Ok(())
}
pub fn decode_response<R: DeserializeOwned>(
    response: aux_rs_core::HttpResponse,
) -> Result<R, Error> {
    if !(200..300).contains(&response.status) {
        let body = serde_json::from_slice(&response.body).unwrap_or_else(|_| {
            serde_json::Value::String(String::from_utf8_lossy(&response.body).into_owned())
        });
        return Err(Error::Api {
            status: response.status,
            body,
        });
    }
    Ok(serde_json::from_slice(&response.body)?)
}
