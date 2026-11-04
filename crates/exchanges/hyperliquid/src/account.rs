use crate::HyperliquidClient;
use crate::{Address, Cloid, Error, ExtraFields, Hyperliquid};
use aux_rs_core::{
    Endpoint, Error as ClientError, Exchange, HttpMethod, HttpRequest, HttpResponse, HttpTransport,
    Public,
};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserStateRequest {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub user: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex: Option<String>,
}
impl UserStateRequest {
    pub fn new(user: Address) -> Self {
        Self { user, dex: None }
    }
}

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub user: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex: Option<String>,
}
impl OpenOrdersRequest {
    pub fn new(user: Address) -> Self {
        Self { user, dex: None }
    }
}

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Serialize)]
#[serde(tag = "type")]
enum AccountRequest<'a> {
    #[serde(rename = "clearinghouseState")]
    State(&'a UserStateRequest),
    #[serde(rename = "openOrders")]
    Orders(&'a OpenOrdersRequest),
}
fn encode(exchange: &Hyperliquid, request: AccountRequest<'_>) -> Result<HttpRequest, Error> {
    Ok(HttpRequest {
        method: HttpMethod::Post,
        url: format!("{}/info", exchange.http_url()),
        headers: vec![("content-type".into(), "application/json".into())],
        body: serde_json::to_vec(&request).map_err(Error::Encode)?,
    })
}
impl Endpoint<Hyperliquid> for UserStateRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = UserState;
    type Access = Public;
    fn request(&self, exchange: &Hyperliquid) -> Result<HttpRequest, Error> {
        encode(exchange, AccountRequest::State(self))
    }
    fn decode(response: HttpResponse) -> Result<UserState, Error> {
        crate::endpoints::decode(response)
    }
}
impl Endpoint<Hyperliquid> for OpenOrdersRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = Vec<OpenOrder>;
    type Access = Public;
    fn request(&self, exchange: &Hyperliquid) -> Result<HttpRequest, Error> {
        encode(exchange, AccountRequest::Orders(self))
    }
    fn decode(response: HttpResponse) -> Result<Vec<OpenOrder>, Error> {
        crate::endpoints::decode(response)
    }
}

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserState {
    pub asset_positions: Vec<AssetPosition>,
    pub cross_maintenance_margin_used: String,
    pub cross_margin_summary: MarginSummary,
    pub margin_summary: MarginSummary,
    pub withdrawable: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub time: Option<u64>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginSummary {
    pub account_value: String,
    pub total_margin_used: String,
    pub total_ntl_pos: String,
    pub total_raw_usd: String,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetPosition {
    #[serde(rename = "type")]
    pub position_type: String,
    pub position: Position,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub coin: String,
    pub cum_funding: CumulativeFunding,
    pub entry_px: Option<String>,
    pub leverage: Leverage,
    pub liquidation_px: Option<String>,
    pub margin_used: String,
    pub max_leverage: u32,
    pub position_value: String,
    pub return_on_equity: String,
    pub szi: String,
    pub unrealized_pnl: String,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CumulativeFunding {
    pub all_time: String,
    pub since_change: String,
    pub since_open: String,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Leverage {
    Cross {
        value: u32,
    },
    Isolated {
        value: u32,
        #[serde(rename = "rawUsd")]
        raw_usd: String,
    },
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrder {
    pub coin: String,
    pub limit_px: String,
    pub oid: u64,
    pub side: String,
    pub sz: String,
    pub timestamp: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(type = "(string) | null", optional = nullable))]
    pub cloid: Option<Cloid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub orig_sz: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}

/// Account reads use the account address, without requiring its signing key.
impl<T: HttpTransport, A: Sync> HyperliquidClient<T, A> {
    pub async fn clearinghouse_state(
        &self,
        request: UserStateRequest,
    ) -> Result<UserState, ClientError<Error>> {
        self.request(request).await
    }
    pub async fn open_orders(
        &self,
        request: OpenOrdersRequest,
    ) -> Result<Vec<OpenOrder>, ClientError<Error>> {
        self.request(request).await
    }
}

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountStateEvent {
    pub dex: String,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub user: Address,
    pub clearinghouse_state: UserState,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenOrdersEvent {
    pub dex: String,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub user: Address,
    pub orders: Vec<OpenOrder>,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderUpdate {
    pub order: OpenOrder,
    pub status: String,
    pub status_timestamp: u64,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFills {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub is_snapshot: Option<bool>,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub user: Address,
    pub fills: Vec<Fill>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    pub coin: String,
    pub px: String,
    pub sz: String,
    pub side: String,
    pub time: u64,
    pub start_position: String,
    pub dir: String,
    pub closed_pnl: String,
    pub hash: String,
    pub oid: u64,
    pub crossed: bool,
    pub fee: String,
    pub tid: u64,
    pub fee_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub builder_fee: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
