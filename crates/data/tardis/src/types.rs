use aux_rs_data::{Error, Result};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    pub channel: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub symbols: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedRequest {
    pub exchange: String,
    pub from: String,
    #[serde(default)]
    pub offset: u64,
    #[serde(default)]
    pub filters: Vec<Filter>,
    #[serde(default = "one", rename = "sliceSize")]
    pub slice_size: u8,
}
fn one() -> u8 {
    1
}
impl FeedRequest {
    pub fn new(exchange: impl Into<String>, from: impl Into<String>) -> Self {
        Self {
            exchange: exchange.into(),
            from: from.into(),
            offset: 0,
            filters: Vec::new(),
            slice_size: 1,
        }
    }
    pub fn validate(&self) -> Result<()> {
        if self.exchange.is_empty() || self.from.is_empty() {
            return Err(Error::InvalidRequest("exchange and from are required"));
        }
        if !(1..=10).contains(&self.slice_size) || (self.slice_size > 1 && self.filters.is_empty())
        {
            return Err(Error::InvalidRequest(
                "sliceSize requires 1..10 and filters for multi-minute slices",
            ));
        }
        if self
            .filters
            .iter()
            .any(|f| f.channel.is_empty() || f.symbols.len() > 50)
        {
            return Err(Error::InvalidRequest(
                "filters require a channel and at most 50 symbols",
            ));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplayRequest {
    pub exchange: String,
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub filters: Vec<Filter>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatasetRequest {
    pub exchange: String,
    pub data_type: String,
    pub date: String,
    pub symbol: String,
    /// Optional native single HTTP byte range, for example `bytes=0-1023`.
    #[serde(default)]
    pub range: Option<String>,
}
