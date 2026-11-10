//! Native exchange and data-vendor adapters with optional HTTP and WebSocket transports.
//!
//! Enable `hyperliquid`, `lighter`, or both to select exchanges. The default `http` and `ws`
//! features supply standard transports; disable defaults to bring your own I/O.
//! Data vendors include Tokio-backed file helpers; Databento also includes its TCP connector.

pub use aux_rs_core::*;

#[cfg(feature = "hyperliquid")]
pub use aux_rs_hyperliquid as hyperliquid;

#[cfg(feature = "lighter")]
pub use aux_rs_lighter as lighter;

#[cfg(feature = "qfex")]
pub use aux_rs_qfex as qfex;

#[cfg(feature = "arcus")]
pub use aux_rs_arcus as arcus;

#[cfg(feature = "monaco")]
pub use aux_rs_monaco as monaco;

#[cfg(feature = "derive")]
pub use aux_rs_derive as derive;

#[cfg(feature = "paradex")]
pub use aux_rs_paradex as paradex;

#[cfg(feature = "tardis")]
pub use aux_rs_tardis as tardis;

#[cfg(feature = "databento")]
pub use aux_rs_databento as databento;
#[cfg(feature = "metric")]
pub use aux_rs_metric as metric;

#[cfg(feature = "kalshi")]
pub use aux_rs_kalshi as kalshi;
#[cfg(feature = "pascal")]
pub use aux_rs_pascal as pascal;
#[cfg(feature = "polymarket")]
pub use aux_rs_polymarket as polymarket;

#[cfg(feature = "phoenix")]
pub use aux_rs_phoenix as phoenix;

#[cfg(feature = "extended")]
pub use aux_rs_extended as extended;

#[cfg(feature = "dune")]
pub use aux_rs_dune as dune;

#[cfg(feature = "birdeye")]
pub use aux_rs_birdeye as birdeye;
