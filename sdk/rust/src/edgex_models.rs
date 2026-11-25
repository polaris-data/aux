// Generated from src/exchanges/edgex/schema/edgex.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{EdgeXClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "contract_id", skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<String>,
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsResponseData {
    #[serde(rename = "contractList", skip_serializing_if = "Option::is_none")]
    pub contract_list: Option<Vec<Market>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsResponse {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<MarketsResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookRequest {
    #[serde(rename = "contractId")]
    pub contract_id: String,
    #[serde(rename = "level")]
    pub level: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookResponse {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountResponseData {
    #[serde(rename = "collateralAssetList", skip_serializing_if = "Option::is_none")]
    pub collateral_asset_list: Option<Vec<serde_json::Value>>,
    #[serde(rename = "positionAssetList", skip_serializing_if = "Option::is_none")]
    pub position_asset_list: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountResponse {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<AccountResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "contractIdList")]
    pub contract_id_list: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsResponse {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "filterContractIdList", skip_serializing_if = "Option::is_none")]
    pub filter_contract_id_list: Option<Vec<String>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "offsetData", skip_serializing_if = "Option::is_none")]
    pub offset_data: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersResponseData {
    #[serde(rename = "dataList", skip_serializing_if = "Option::is_none")]
    pub data_list: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersResponse {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<OpenOrdersResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "orderIdList", skip_serializing_if = "Option::is_none")]
    pub order_id_list: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusResponse {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "filterContractIdList", skip_serializing_if = "Option::is_none")]
    pub filter_contract_id_list: Option<Vec<String>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsResponseData {
    #[serde(rename = "dataList", skip_serializing_if = "Option::is_none")]
    pub data_list: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsResponse {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<FillsResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderBody {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "contractId")]
    pub contract_id: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "price_bound", skip_serializing_if = "Option::is_none")]
    pub price_bound: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    #[serde(rename = "body")]
    pub body: PlaceOrderBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderResponseData {
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderResponse {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<PlaceOrderResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderBody {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "orderIdList")]
    pub order_id_list: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    #[serde(rename = "body")]
    pub body: CancelOrderBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderResponseData {
    #[serde(rename = "cancelResultMap", skip_serializing_if = "Option::is_none")]
    pub cancel_result_map: Option<std::collections::BTreeMap<String,String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderResponse {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<CancelOrderResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Market {
    #[serde(rename = "contractId")]
    pub contract_id: String,
    #[serde(rename = "contractName", skip_serializing_if = "Option::is_none")]
    pub contract_name: Option<String>,
    #[serde(rename = "quoteCoinId", skip_serializing_if = "Option::is_none")]
    pub quote_coin_id: Option<String>,
    #[serde(rename = "tickSize", skip_serializing_if = "Option::is_none")]
    pub tick_size: Option<String>,
    #[serde(rename = "stepSize", skip_serializing_if = "Option::is_none")]
    pub step_size: Option<String>,
    #[serde(rename = "resolution", skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderByClientIdRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "clientOrderIdList", skip_serializing_if = "Option::is_none")]
    pub client_order_id_list: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedEdgeXClient(EdgeXClient);
impl EdgeXClient {
    pub fn typed(self) -> TypedEdgeXClient { TypedEdgeXClient(self) }
}
impl TypedEdgeXClient {
    pub fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub fn order_book(&self, request: &OrderBookRequest) -> Result<OrderBookResponse> {
        serde_json::from_value(self.0.order_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub fn order_by_client_id(&self, request: &OrderByClientIdRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_by_client_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedEdgeXClient(crate::AsyncClient<EdgeXClient>);
#[cfg(feature = "async")]
impl TypedEdgeXClient {
    pub fn into_async(self) -> AsyncTypedEdgeXClient { AsyncTypedEdgeXClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedEdgeXClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub async fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub async fn order_book(&self, request: &OrderBookRequest) -> Result<OrderBookResponse> {
        serde_json::from_value(self.0.order_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub async fn order_by_client_id(&self, request: &OrderByClientIdRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_by_client_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Edgex response model"))
    }
}
