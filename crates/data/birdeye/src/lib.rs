//! Birdeye's native market data REST APIs. Pagination and retries are caller-owned.
mod client;
mod models;
mod operations;
pub use aux_rs_data::{Error, Result};
pub use client::*;
pub use models::*;
