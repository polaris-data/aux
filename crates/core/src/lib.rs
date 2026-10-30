//! Native endpoint and I/O contracts. With no features this crate has no dependencies.

mod encoding;
mod endpoint;
mod error;
mod transport;

#[cfg(any(feature = "http", feature = "ws"))]
pub mod standard;

pub use encoding::*;
pub use endpoint::*;
pub use error::*;
#[cfg(any(feature = "http", feature = "ws"))]
pub use standard::StandardTransport;
pub use transport::*;
