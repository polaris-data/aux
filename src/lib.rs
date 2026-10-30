//! Native exchange adapters with composable clients and optional transports.
//!
//! Enable `hyperliquid`, `lighter`, or both to select exchanges. The default `http` and `ws`
//! features supply standard transports; disable defaults to bring your own I/O.

pub use aux_rs_core::*;

#[cfg(feature = "hyperliquid")]
pub use aux_rs_hyperliquid as hyperliquid;

#[cfg(feature = "lighter")]
pub use aux_rs_lighter as lighter;
