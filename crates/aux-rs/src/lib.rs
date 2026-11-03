//! Native exchange adapters with composable clients and optional transports.
//!
//! Enable `hyperliquid`, `lighter`, or both to select exchanges. The default `http` and `ws`
//! features supply standard transports; disable defaults to bring your own I/O.

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
