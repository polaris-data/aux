//! Dune's native execution and results APIs. Every operation performs one request.
mod client;
mod models;
mod operations;
pub use aux_rs_data::{Error, Result};
pub use client::*;
pub use models::*;
