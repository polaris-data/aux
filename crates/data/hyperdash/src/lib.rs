//! Hyperdash Data API clients and native request, response, and stream types.

mod client;
mod generated;
mod stream;

pub use client::*;
pub use generated::*;
pub use stream::*;

/// Deserialize a field whose schema allows null but whose key must be present.
fn present<'de, D, T>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    serde::Deserialize::deserialize(deserializer)
}

/// Distinguish a missing key (handled by serde's default) from explicit null.
fn optional_nullable<'de, D, T>(deserializer: D) -> std::result::Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    present(deserializer).map(Some)
}
