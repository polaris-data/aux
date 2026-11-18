//! Chainlink offchain Data Streams and Candlestick APIs. Decoding does not verify signatures.
mod client;
mod models;
mod operations;
mod report;
mod ws;
pub use aux_rs_data::{Error, JsonStream, NativeResponse, Result};
pub use client::*;
pub use models::*;
pub use report::*;

/// Chainlink authorization envelope. The token is redacted from Debug output.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuthorizationResponse {
    pub s: String,
    pub d: AuthorizationToken,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuthorizationToken {
    pub access_token: String,
    /// Vendor-reported Unix seconds.
    pub expiration: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
