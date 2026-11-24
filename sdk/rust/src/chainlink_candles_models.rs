// Generated from src/data/chainlink/schema/chainlink_candles.json.
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{ChainlinkCandlestickClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizeRequest {
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "password")]
    pub password: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeResponse {
    #[serde(rename = "status")]
    pub status: u16,
    #[serde(rename = "headers")]
    pub headers: Vec<[String;2]>,
    #[serde(rename = "data")]
    pub data: serde_json::Value,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolInfoRequest {
    #[serde(rename = "group", default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupsRequest {
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandleHistoryRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(rename = "from")]
    pub from: u64,
    #[serde(rename = "to")]
    pub to: u64,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandleHistoryRowsRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(rename = "from")]
    pub from: u64,
    #[serde(rename = "to")]
    pub to: u64,
}

pub struct TypedChainlinkCandlestickClient(ChainlinkCandlestickClient);
impl ChainlinkCandlestickClient {
    pub fn typed(self) -> TypedChainlinkCandlestickClient { TypedChainlinkCandlestickClient(self) }
}
impl TypedChainlinkCandlestickClient {
    pub fn authorize(&self, request: &AuthorizeRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.authorize(&json!(request))?)
            .map_err(|_| Error::Decode("invalid ChainlinkCandles response model"))
    }
    pub fn candle_history(&self, request: &CandleHistoryRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.candle_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid ChainlinkCandles response model"))
    }
    pub fn candle_history_rows(&self, request: &CandleHistoryRowsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.candle_history_rows(&json!(request))?)
            .map_err(|_| Error::Decode("invalid ChainlinkCandles response model"))
    }
    pub fn groups(&self, request: &GroupsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.groups(&json!(request))?)
            .map_err(|_| Error::Decode("invalid ChainlinkCandles response model"))
    }
    pub fn symbol_info(&self, request: &SymbolInfoRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.symbol_info(&json!(request))?)
            .map_err(|_| Error::Decode("invalid ChainlinkCandles response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedChainlinkCandlestickClient(crate::AsyncClient<ChainlinkCandlestickClient>);
#[cfg(feature = "async")]
impl TypedChainlinkCandlestickClient {
    pub fn into_async(self) -> AsyncTypedChainlinkCandlestickClient { AsyncTypedChainlinkCandlestickClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedChainlinkCandlestickClient {
    pub async fn authorize(&self, request: &AuthorizeRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.authorize(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid ChainlinkCandles response model"))
    }
    pub async fn candle_history(&self, request: &CandleHistoryRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.candle_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid ChainlinkCandles response model"))
    }
    pub async fn candle_history_rows(&self, request: &CandleHistoryRowsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.candle_history_rows(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid ChainlinkCandles response model"))
    }
    pub async fn groups(&self, request: &GroupsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.groups(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid ChainlinkCandles response model"))
    }
    pub async fn symbol_info(&self, request: &SymbolInfoRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.symbol_info(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid ChainlinkCandles response model"))
    }
}
