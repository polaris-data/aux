//! Hyperliquid-native public/account APIs, Rust signing, and trading.
//! No network implementation or async runtime is required by this adapter.

mod account;
mod auth;
mod client;
mod endpoints;
mod error;
mod identifiers;
pub mod signing;
mod trading;
mod types;
mod ws;

pub use account::*;
pub use auth::*;
pub use client::HyperliquidClient;
pub use endpoints::*;
pub use error::Error;
pub use identifiers::{Address, Cloid};
pub use signing::{LocalSigner, Signature, Signer, SignerError, SigningPayload};
pub use trading::*;
pub use types::*;
pub use ws::*;

use aux_rs_core::Exchange;

/// Endpoint and signing-network configuration for `HyperliquidClient`.
/// HTTP endpoints are base URLs (without `/info`).
#[derive(Clone, PartialEq, Eq)]
pub struct Hyperliquid {
    http_url: String,
    ws_url: String,
    network: Option<Network>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Network {
    Mainnet,
    Testnet,
}

impl Network {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mainnet => "Mainnet",
            Self::Testnet => "Testnet",
        }
    }
}

impl std::fmt::Debug for Hyperliquid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Custom endpoint URLs may contain credentials supplied by callers.
        f.debug_struct("Hyperliquid").finish_non_exhaustive()
    }
}

impl Hyperliquid {
    pub fn mainnet() -> Self {
        Self::custom(
            "https://api.hyperliquid.xyz",
            "wss://api.hyperliquid.xyz/ws",
        )
        .with_network(Network::Mainnet)
    }

    pub fn testnet() -> Self {
        Self::custom(
            "https://api.hyperliquid-testnet.xyz",
            "wss://api.hyperliquid-testnet.xyz/ws",
        )
        .with_network(Network::Testnet)
    }

    /// Custom URLs are validated by the chosen transport when used. Base paths
    /// are supported, for example `http://localhost:8080/hyperliquid`.
    pub fn custom(http_base_url: impl Into<String>, ws_url: impl Into<String>) -> Self {
        Self {
            http_url: http_base_url.into().trim_end_matches('/').to_owned(),
            ws_url: ws_url.into(),
            network: None,
        }
    }

    /// Custom URLs need explicit signing context; it is never inferred from a URL.
    pub fn with_network(mut self, network: Network) -> Self {
        self.network = Some(network);
        self
    }
    pub fn network(&self) -> Option<Network> {
        self.network
    }
}

impl Exchange for Hyperliquid {
    type Error = Error;
    fn http_url(&self) -> &str {
        &self.http_url
    }
    fn ws_url(&self) -> &str {
        &self.ws_url
    }
}
