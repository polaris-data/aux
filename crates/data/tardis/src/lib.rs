//! Tardis hosted data APIs and caller-managed Tardis Machine HTTP/WebSocket APIs.
//! Payloads retain Tardis/native JSON schemas and exact JSON numbers.
mod client;
mod stream;
mod types;
pub use aux_rs_data::download_to;
pub use aux_rs_data::{DecodedBody, Error, Params, Result, params};
pub use client::*;
pub use stream::*;
pub use types::*;
