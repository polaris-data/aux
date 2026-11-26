//! Safe bindings for the bundled aux Zig core.
#[cfg(feature = "async")]
mod asynchronous;
#[cfg(feature = "async")]
pub use asynchronous::{AsyncClient, AsyncEventStream, AsyncSession};
pub mod encoding;
pub mod hermes;
pub mod json;
pub mod metric;
#[cfg(feature = "metric-models")]
pub mod metric_contracts {
    include!("metric_contracts_generated.rs");
}
pub mod phoenix;
pub mod polymarket;
pub use phoenix::PhoenixChainClient;

#[derive(Debug)]
pub enum Error {
    OutOfMemory,
    Network,
    ExchangeApi,
    ExchangeResponse(ExchangeResponse),
    SessionResponse(SessionResponse),
    AuthenticationRequired,
    PendingOutcome,
    PendingTransaction { hash: String },
    StreamGap,
    InvalidRequest(&'static str),
    Decode(&'static str),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ExchangeResponse {
    pub venue: String,
    pub status: u16,
    pub body: serde_json::Value,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SessionResponse {
    pub venue: String,
    pub body: serde_json::Value,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StreamGap => fmt.write_str("stream sequence or snapshot missing"),
            Self::PendingOutcome => fmt.write_str("submission outcome pending reconciliation"),
            Self::PendingTransaction { hash } => write!(fmt, "submission outcome pending reconciliation for {hash}"),
            Self::Network => fmt.write_str("HTTP transport failed"),
            Self::ExchangeResponse(response) => write!(
                fmt,
                "{} returned an exchange error (HTTP {})",
                response.venue, response.status
            ),
            Self::SessionResponse(response) => {
                write!(fmt, "{} returned a session error", response.venue)
            }
            Self::ExchangeApi => fmt.write_str("exchange returned an API error"),
            Self::AuthenticationRequired => fmt.write_str("operation requires authentication"),
            Self::OutOfMemory => fmt.write_str("allocation failed"),
            Self::InvalidRequest(message) | Self::Decode(message) => fmt.write_str(message),
        }
    }
}
impl std::error::Error for Error {}

pub mod contracts;
#[doc(hidden)]
pub mod native;
pub mod chainlink {
    use crate::{Error, Result, native::invoke};
    pub fn decode_report(full_report: &str) -> Result<serde_json::Value> {
        serde_json::from_str(&invoke(1, full_report.as_bytes())?)
            .map_err(|_| Error::Decode("invalid report result"))
    }
}

pub mod http;
#[cfg(feature = "rustls")]
mod tls;

pub mod auth {
    use crate::{Result, http::call};
    use serde_json::{Value, json};

    pub fn kalshi_headers(
        key: &str,
        seed: &str,
        method: &str,
        path: &str,
        timestamp: u64,
    ) -> Result<Value> {
        call(
            22,
            json!({"key":key,"seed":seed,"method":method,"path":path,"timestamp":timestamp}),
        )
    }
    pub fn qfex_signature(secret: &str, nonce: &str, timestamp: u64) -> Result<String> {
        call(
            13,
            json!({ "secret": secret, "nonce": nonce, "timestamp": timestamp }),
        )
    }
    pub fn databento_cram(key: &str, challenge: &str) -> Result<String> {
        call(14, json!({"key": key, "challenge": challenge}))
    }
    pub fn arcus_sign(seed: &str, payload: &str) -> Result<Value> {
        call(15, json!({"seed": seed,"payload": payload}))
    }
    pub fn monaco_sign(
        seed: &str,
        method: &str,
        target: &str,
        body: &str,
        created: u64,
    ) -> Result<Value> {
        call(
            16,
            json!({"seed":seed,"method":method,"target":target,"body":body,"created":created}),
        )
    }
}

pub mod evm {
    use crate::{Result, http::call};
    use serde_json::{Value, json};
    /// Sign a 32-byte transaction or typed-data digest using a recoverable EVM signature.
    pub fn sign_digest(key: &str, digest: &str) -> Result<String> {
        call(59, json!({"operation":"sign_digest","key":key,"digest":digest}))
    }
    pub fn sign_transaction(key: &str, prepared: &Value) -> Result<Value> {
        call(17, json!({"key":key,"prepared":prepared}))
    }
}

pub mod clients;
pub use clients::*;

pub use native::PriceStream;
pub use native::PriceStream as EventStream;

pub mod databento;

pub mod paradex {
    use crate::{Result, encoding, http::call};
    use serde_json::{Value, json};
    pub fn decode_sbe(frame: &[u8]) -> Result<Value> {
        call(19, json!({"data":encoding::hex_encode(frame)}))
    }
}

pub mod derive;
pub mod evm_rpc;
pub use evm_rpc::{
    EvmChainClient, EvmReader, MetricChainClient, PendingTransaction, PolymarketChainClient,
};

#[cfg(feature = "derive-models")]
#[rustfmt::skip]
pub mod derive_models;

#[cfg(feature = "paradex-models")]
#[rustfmt::skip]
pub mod paradex_models;

#[cfg(feature = "phoenix-models")]
#[rustfmt::skip]
pub mod phoenix_models;

#[cfg(feature = "extended-models")]
#[rustfmt::skip]
pub mod extended_models;

#[cfg(any(feature = "phoenix-models", feature = "extended-models"))]
mod model_number;

pub mod solana;

#[cfg(feature = "pascal-models")]
#[rustfmt::skip]
pub mod pascal_models;

#[cfg(feature = "kalshi-models")]
#[rustfmt::skip]
pub mod kalshi_models;

#[cfg(feature = "kalshi-perps-models")]
#[rustfmt::skip]
pub mod kalshi_perps_models;

#[cfg(feature = "polymarket-models")]
#[rustfmt::skip]
pub mod polymarket_models;

#[cfg(feature = "polymarket-perps-models")]
#[rustfmt::skip]
pub mod polymarket_perps_models;
pub use solana::PascalChainClient;

#[cfg(feature="hyperdash-models")]
#[rustfmt::skip]
pub mod hyperdash_models;

#[cfg(feature="birdeye-models")]
#[rustfmt::skip]
pub mod birdeye_models;

#[cfg(feature = "pyth-hermes-models")]
#[rustfmt::skip]
pub mod pyth_hermes_models;

#[cfg(feature = "pyth-pro-models")]
#[rustfmt::skip]
pub mod pyth_pro_models;

#[cfg(feature = "chainlink-streams-models")]
#[rustfmt::skip]
pub mod chainlink_streams_models;

#[cfg(feature = "chainlink-candles-models")]
#[rustfmt::skip]
pub mod chainlink_candles_models;

#[cfg(feature = "dune-models")]
#[rustfmt::skip]
pub mod dune_models;

pub mod extended {
    use crate::{Result, http::call};
    use serde_json::{Value, json};

    pub fn prepare_order(
        chain_id: &str,
        public_key: &str,
        body: &Value,
        context: &Value,
    ) -> Result<Value> {
        call(
            40,
            json!({"chain_id":chain_id,"public_key":public_key,"body":body,"context":context}),
        )
    }
    pub fn sign_order(
        chain_id: &str,
        stark_key: &str,
        body: &Value,
        context: &Value,
    ) -> Result<Value> {
        call(
            40,
            json!({"chain_id":chain_id,"stark_key":stark_key,"body":body,"context":context}),
        )
    }
    /// Hash a native settlement and optionally sign it with the matching Stark key.
    pub fn settlement(
        kind: &str,
        chain_id: &str,
        public_key: &str,
        message: &Value,
        stark_key: Option<&str>,
    ) -> Result<Value> {
        let mut args =
            json!({"kind":kind,"chain_id":chain_id,"public_key":public_key,"message":message});
        if let Some(key) = stark_key {
            args["stark_key"] = key.into();
        }
        call(39, args)
    }
}

#[cfg(feature = "arcus-models")]
#[rustfmt::skip]
pub mod arcus_models;
#[cfg(feature = "monaco-models")]
#[rustfmt::skip]
pub mod monaco_models;
#[cfg(feature = "hyperliquid-models")]
#[rustfmt::skip]
pub mod hyperliquid_models;
#[cfg(feature = "perpl-models")]
#[allow(clippy::all)]
pub mod perpl_models;
#[cfg(feature = "risex-models")]
#[allow(clippy::all)]
pub mod risex_models;
#[cfg(feature = "decibel-models")]
#[allow(clippy::all)]
pub mod decibel_models;
#[cfg(feature = "edgex-models")]
#[allow(clippy::all)]
pub mod edgex_models;
#[cfg(feature = "grvt-models")]
#[allow(clippy::all)]
pub mod grvt_models;
#[cfg(feature = "katana-models")]
#[allow(clippy::all)]
pub mod katana_models;
#[cfg(feature = "aevo-models")]
#[path = "aevo_models.rs"]
pub mod aevo_models;
#[cfg(feature = "apex-omni-models")]
#[path = "apex_omni_models.rs"]
pub mod apex_omni_models;
#[cfg(feature = "backpack-models")]
#[path = "backpack_models.rs"]
pub mod backpack_models;
#[cfg(feature = "binance-spot-models")]
#[path = "binance_spot_models.rs"]
pub mod binance_spot_models;
#[cfg(feature = "bybit-models")]
#[path = "bybit_models.rs"]
pub mod bybit_models;
#[cfg(feature = "deribit-models")]
#[path = "deribit_models.rs"]
pub mod deribit_models;
#[cfg(feature = "dydx-v4-models")]
#[path = "dydx_v4_models.rs"]
pub mod dydx_v4_models;
#[cfg(feature = "hibachi-models")]
#[path = "hibachi_models.rs"]
pub mod hibachi_models;
#[cfg(feature = "kraken-spot-models")]
#[path = "kraken_spot_models.rs"]
pub mod kraken_spot_models;
#[cfg(feature = "kuru-models")]
#[path = "kuru_models.rs"]
pub mod kuru_models;
#[cfg(feature = "okx-models")]
#[path = "okx_models.rs"]
pub mod okx_models;
#[cfg(feature = "orderly-models")]
#[path = "orderly_models.rs"]
pub mod orderly_models;
#[cfg(feature = "reya-models")]
#[path = "reya_models.rs"]
pub mod reya_models;
#[cfg(feature = "variational-models")]
#[allow(clippy::all)]
pub mod variational_models;
#[cfg(feature = "aster-models")]
#[rustfmt::skip]
pub mod aster_models;
#[cfg(feature = "injective-models")]
#[rustfmt::skip]
pub mod injective_models;
#[cfg(feature = "nado-models")]
#[rustfmt::skip]
pub mod nado_models;
#[cfg(feature = "ondo-perps-models")]
#[rustfmt::skip]
pub mod ondo_perps_models;
#[cfg(feature = "pacifica-models")]
#[rustfmt::skip]
pub mod pacifica_models;
#[cfg(feature = "paragon-models")]
#[rustfmt::skip]
pub mod paragon_models;
#[cfg(feature = "trade-xyz-models")]
#[rustfmt::skip]
pub mod trade_xyz_models;
#[cfg(feature = "databento-historical-models")]
#[rustfmt::skip]
pub mod databento_historical_models;
#[cfg(feature = "databento-reference-models")]
#[rustfmt::skip]
pub mod databento_reference_models;
#[cfg(feature = "metric-models")]
#[rustfmt::skip]
pub mod metric_models;
#[cfg(feature = "qfex-models")]
#[rustfmt::skip]
pub mod qfex_models;
#[cfg(feature = "tardis-models")]
#[rustfmt::skip]
pub mod tardis_models;
#[cfg(feature = "lighter-models")]
#[rustfmt::skip]
pub mod lighter_models;
