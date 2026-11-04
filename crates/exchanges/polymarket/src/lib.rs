//! Native Polymarket REST/WebSocket interfaces and signing.
mod auth;
mod client;
mod error;
pub use auth::*;
pub use client::*;
pub use error::Error;
pub mod bridge;
pub mod clob;
pub mod combos_rfq;
pub mod data;
pub mod gamma;
pub mod perps;
pub mod relayer;

mod perps_auth;
pub use perps_auth::*;

mod ws;
pub use ws::*;

mod rpc;
pub use rpc::*;

mod chain;
pub mod contracts;
pub use alloy_primitives::{Address, B256, Bytes, Signature, U256};
pub use alloy_sol_types::{SolCall, SolError, SolEvent};
pub use chain::*;

mod abi;
pub use abi::*;
pub type PreparedCall = ContractCall;

pub mod ws_types;

pub mod builder_gateway;
pub mod collateral_return;
