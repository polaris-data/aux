//! Native Lighter APIs and Rust signing. No runtime or networking implementation
//! is required. Local cryptographic primitives are unaudited: see SIGNING.md.
//!
//! Protected methods require an authenticated client:
//! ```compile_fail
//! use aux_rs_core::HttpTransport;
//! use aux_rs_lighter::{LighterClient, CancelOrder, TxOptions};
//! fn unsigned<T: HttpTransport>(client: LighterClient<T>) {
//!     let _ = client.cancel_order(CancelOrder { market_index: 4096, index: 1 }, TxOptions::new(1800000000000));
//! }
//! ```
mod auth;
mod client;
mod encoding;
mod endpoints;
mod error;
mod poseidon;
pub mod signing;
mod trading;
mod types;
mod ws;

pub use auth::*;
pub use client::LighterClient;
pub use endpoints::*;
pub use error::Error;
pub use signing::{LocalSigner, PublicKey, Signature, Signer, SigningPayload};
pub use trading::*;
pub use types::*;
pub use ws::*;

use aux_rs_core::Exchange;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Network {
    Mainnet,
    Testnet,
}
impl Network {
    pub fn chain_id(self) -> u32 {
        match self {
            Self::Mainnet => 304,
            Self::Testnet => 300,
        }
    }
}

/// Endpoint and chain configuration for `LighterClient`.
#[derive(Clone, PartialEq, Eq)]
pub struct Lighter {
    http_url: String,
    ws_url: String,
    chain_id: u32,
}
impl std::fmt::Debug for Lighter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lighter")
            .field("chain_id", &self.chain_id)
            .finish_non_exhaustive()
    }
}
impl Lighter {
    pub fn mainnet() -> Self {
        Self::custom(
            "https://mainnet.zklighter.elliot.ai",
            "wss://mainnet.zklighter.elliot.ai/stream",
            Network::Mainnet.chain_id(),
        )
    }
    pub fn testnet() -> Self {
        Self::custom(
            "https://testnet.zklighter.elliot.ai",
            "wss://testnet.zklighter.elliot.ai/stream",
            Network::Testnet.chain_id(),
        )
    }
    /// Custom endpoints always require explicit chain context. Never inferred from URLs.
    pub fn custom(http_url: impl Into<String>, ws_url: impl Into<String>, chain_id: u32) -> Self {
        Self {
            http_url: http_url.into().trim_end_matches('/').to_owned(),
            ws_url: ws_url.into(),
            chain_id,
        }
    }
    pub fn chain_id(&self) -> u32 {
        self.chain_id
    }
}
impl Exchange for Lighter {
    type Error = Error;
    fn http_url(&self) -> &str {
        &self.http_url
    }
    fn ws_url(&self) -> &str {
        &self.ws_url
    }
}
