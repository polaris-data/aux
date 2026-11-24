// Generated from schema/hyperliquid.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{HyperliquidClient, Error, Result};

#[derive(Clone, Serialize, Deserialize)]
pub enum Tif {
    #[serde(rename = "Alo")]
    Alo,
    #[serde(rename = "Ioc")]
    Ioc,
    #[serde(rename = "Gtc")]
    Gtc,
}
#[derive(Clone, Serialize, Deserialize)]
pub enum Tpsl {
    #[serde(rename = "tp")]
    Tp,
    #[serde(rename = "sl")]
    Sl,
}
#[derive(Clone, Serialize, Deserialize)]
pub enum NamedGrouping {
    #[serde(rename = "na")]
    Na,
    #[serde(rename = "normalTpsl")]
    NormalTpsl,
    #[serde(rename = "positionTpsl")]
    PositionTpsl,
}
#[derive(Clone, Serialize, Deserialize)]
pub enum CrossKind {
    #[serde(rename = "cross")]
    Cross,
}
#[derive(Clone, Serialize, Deserialize)]
pub enum IsolatedKind {
    #[serde(rename = "isolated")]
    Isolated,
}
#[derive(Clone, Serialize, Deserialize)]
pub enum OkStatus {
    #[serde(rename = "ok")]
    Ok,
}
#[derive(Clone, Serialize, Deserialize)]
pub enum ErrStatus {
    #[serde(rename = "err")]
    Err,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderType {
    Limit(LimitOrder),
    Trigger(TriggerOrder),
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Grouping {
    Named(NamedGrouping),
    Priority(PriorityGrouping),
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemStatus {
    Resting(RestingStatus),
    Filled(FilledStatus),
    Error(ErrorStatus),
    Text(String),
    Unknown(serde_json::Value),
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TradingResponse {
    Ok(OkResponse),
    Err(ErrResponse),
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Leverage {
    Cross(CrossLeverage),
    Isolated(IsolatedLeverage),
}
pub type Cloid = String;
pub type Address = String;

#[derive(Clone, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "universe")]
    pub universe: Vec<Asset>,
    #[serde(rename = "marginTables", default)]
    pub margin_tables: Vec<(u32, MarginTable)>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "szDecimals")]
    pub sz_decimals: u32,
    #[serde(rename = "maxLeverage")]
    pub max_leverage: u32,
    #[serde(rename = "onlyIsolated", skip_serializing_if = "Option::is_none")]
    pub only_isolated: Option<bool>,
    #[serde(rename = "isDelisted", skip_serializing_if = "Option::is_none")]
    pub is_delisted: Option<bool>,
    #[serde(rename = "marginTableId", skip_serializing_if = "Option::is_none")]
    pub margin_table_id: Option<u32>,
    #[serde(rename = "marginMode", skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MarginTable {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "marginTiers")]
    pub margin_tiers: Vec<MarginTier>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MarginTier {
    #[serde(rename = "lowerBound")]
    pub lower_bound: String,
    #[serde(rename = "maxLeverage")]
    pub max_leverage: u32,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct L2Book {
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "time")]
    pub time: u64,
    #[serde(rename = "levels")]
    pub levels: [Vec<Level>; 2],
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Level {
    #[serde(rename = "px")]
    pub px: String,
    #[serde(rename = "sz")]
    pub sz: String,
    #[serde(rename = "n")]
    pub n: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MetaRequest {
    #[serde(rename = "dex", skip_serializing_if = "Option::is_none")]
    pub dex: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct L2BookRequest {
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "nSigFigs", skip_serializing_if = "Option::is_none")]
    pub n_sig_figs: Option<u8>,
    #[serde(rename = "mantissa", skip_serializing_if = "Option::is_none")]
    pub mantissa: Option<u8>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "a")]
    pub asset: u32,
    #[serde(rename = "b")]
    pub is_buy: bool,
    #[serde(rename = "p")]
    pub limit_px: String,
    #[serde(rename = "s")]
    pub size: String,
    #[serde(rename = "r")]
    pub reduce_only: bool,
    #[serde(rename = "t")]
    pub order_type: OrderType,
    #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
    pub cloid: Option<Cloid>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SubBuilder {
    #[serde(rename = "b")]
    pub b: Address,
    #[serde(rename = "s")]
    pub s: u32,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BuilderFee {
    #[serde(rename = "b")]
    pub b: Address,
    #[serde(rename = "f")]
    pub f: u32,
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    pub s: Option<Vec<SubBuilder>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    #[serde(rename = "orders")]
    pub orders: Vec<Order>,
    #[serde(rename = "grouping", skip_serializing_if = "Option::is_none")]
    pub grouping: Option<Grouping>,
    #[serde(rename = "builder", skip_serializing_if = "Option::is_none")]
    pub builder: Option<BuilderFee>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<ExecutionOptions>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Cancel {
    #[serde(rename = "a")]
    pub asset: u32,
    #[serde(rename = "o")]
    pub oid: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelRequest {
    #[serde(rename = "cancels")]
    pub cancels: Vec<Cancel>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<ExecutionOptions>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelByCloid {
    #[serde(rename = "asset")]
    pub asset: u32,
    #[serde(rename = "cloid")]
    pub cloid: Cloid,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelByCloidRequest {
    #[serde(rename = "cancels")]
    pub cancels: Vec<CancelByCloid>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<ExecutionOptions>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExecutionOptions {
    #[serde(rename = "nonce", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u64>,
    #[serde(rename = "expires_after", skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchResponse {
    #[serde(rename = "type")]
    pub response_type: String,
    #[serde(rename = "data")]
    pub data: BatchData,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchData {
    #[serde(rename = "statuses")]
    pub statuses: Vec<ItemStatus>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RestingOrder {
    #[serde(rename = "oid")]
    pub oid: u64,
    #[serde(rename = "cloid", skip_serializing_if = "Option::is_none")]
    pub cloid: Option<Cloid>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FilledOrder {
    #[serde(rename = "totalSz")]
    pub total_sz: String,
    #[serde(rename = "avgPx")]
    pub avg_px: String,
    #[serde(rename = "oid")]
    pub oid: u64,
    #[serde(rename = "cloid", skip_serializing_if = "Option::is_none")]
    pub cloid: Option<Cloid>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserStateRequest {
    #[serde(rename = "user")]
    pub user: Address,
    #[serde(rename = "dex", skip_serializing_if = "Option::is_none")]
    pub dex: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "user")]
    pub user: Address,
    #[serde(rename = "dex", skip_serializing_if = "Option::is_none")]
    pub dex: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserState {
    #[serde(rename = "assetPositions")]
    pub asset_positions: Vec<AssetPosition>,
    #[serde(rename = "crossMaintenanceMarginUsed")]
    pub cross_maintenance_margin_used: String,
    #[serde(rename = "crossMarginSummary")]
    pub cross_margin_summary: MarginSummary,
    #[serde(rename = "marginSummary")]
    pub margin_summary: MarginSummary,
    #[serde(rename = "withdrawable")]
    pub withdrawable: String,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MarginSummary {
    #[serde(rename = "accountValue")]
    pub account_value: String,
    #[serde(rename = "totalMarginUsed")]
    pub total_margin_used: String,
    #[serde(rename = "totalNtlPos")]
    pub total_ntl_pos: String,
    #[serde(rename = "totalRawUsd")]
    pub total_raw_usd: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AssetPosition {
    #[serde(rename = "type")]
    pub position_type: String,
    #[serde(rename = "position")]
    pub position: Position,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "cumFunding")]
    pub cum_funding: CumulativeFunding,
    #[serde(rename = "entryPx", skip_serializing_if = "Option::is_none")]
    pub entry_px: Option<String>,
    #[serde(rename = "leverage")]
    pub leverage: Leverage,
    #[serde(rename = "liquidationPx", skip_serializing_if = "Option::is_none")]
    pub liquidation_px: Option<String>,
    #[serde(rename = "marginUsed")]
    pub margin_used: String,
    #[serde(rename = "maxLeverage")]
    pub max_leverage: u32,
    #[serde(rename = "positionValue")]
    pub position_value: String,
    #[serde(rename = "returnOnEquity")]
    pub return_on_equity: String,
    #[serde(rename = "szi")]
    pub szi: String,
    #[serde(rename = "unrealizedPnl")]
    pub unrealized_pnl: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CumulativeFunding {
    #[serde(rename = "allTime")]
    pub all_time: String,
    #[serde(rename = "sinceChange")]
    pub since_change: String,
    #[serde(rename = "sinceOpen")]
    pub since_open: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OpenOrder {
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "limitPx")]
    pub limit_px: String,
    #[serde(rename = "oid")]
    pub oid: u64,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "sz")]
    pub sz: String,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "cloid", skip_serializing_if = "Option::is_none")]
    pub cloid: Option<Cloid>,
    #[serde(rename = "origSz", skip_serializing_if = "Option::is_none")]
    pub orig_sz: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LimitDetails {
    #[serde(rename = "tif")]
    pub tif: Tif,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TriggerDetails {
    #[serde(rename = "isMarket")]
    pub is_market: bool,
    #[serde(rename = "triggerPx")]
    pub trigger_px: String,
    #[serde(rename = "tpsl")]
    pub tpsl: Tpsl,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LimitOrder {
    #[serde(rename = "limit")]
    pub limit: LimitDetails,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TriggerOrder {
    #[serde(rename = "trigger")]
    pub trigger: TriggerDetails,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriorityGrouping {
    #[serde(rename = "p")]
    pub p: u32,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RestingStatus {
    #[serde(rename = "resting")]
    pub resting: RestingOrder,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilledStatus {
    #[serde(rename = "filled")]
    pub filled: FilledOrder,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorStatus {
    #[serde(rename = "error")]
    pub error: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OkResponse {
    #[serde(rename = "status")]
    pub status: OkStatus,
    #[serde(rename = "response")]
    pub response: BatchResponse,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ErrResponse {
    #[serde(rename = "status")]
    pub status: ErrStatus,
    #[serde(rename = "response")]
    pub response: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrossLeverage {
    #[serde(rename = "type")]
    pub kind: CrossKind,
    #[serde(rename = "value")]
    pub value: u32,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IsolatedLeverage {
    #[serde(rename = "type")]
    pub kind: IsolatedKind,
    #[serde(rename = "value")]
    pub value: u32,
    #[serde(rename = "rawUsd")]
    pub raw_usd: String,
}

pub struct TypedHyperliquidClient(HyperliquidClient);
impl HyperliquidClient {
    pub fn typed(self) -> TypedHyperliquidClient { TypedHyperliquidClient(self) }
}
impl TypedHyperliquidClient {
    pub fn cancel(&self, request: &CancelRequest) -> Result<TradingResponse> {
        serde_json::from_value(self.0.cancel(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Hyperliquid response model"))
    }
    pub fn cancel_by_cloid(&self, request: &CancelByCloidRequest) -> Result<TradingResponse> {
        serde_json::from_value(self.0.cancel_by_cloid(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Hyperliquid response model"))
    }
    pub fn clearinghouse_state(&self, request: &UserStateRequest) -> Result<UserState> {
        serde_json::from_value(self.0.clearinghouse_state(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Hyperliquid response model"))
    }
    pub fn l2_book(&self, request: &L2BookRequest) -> Result<L2Book> {
        serde_json::from_value(self.0.l2_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Hyperliquid response model"))
    }
    pub fn meta(&self, request: &MetaRequest) -> Result<Meta> {
        serde_json::from_value(self.0.meta(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Hyperliquid response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<Vec<OpenOrder>> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Hyperliquid response model"))
    }
    pub fn order(&self, request: &OrderRequest) -> Result<TradingResponse> {
        serde_json::from_value(self.0.order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Hyperliquid response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedHyperliquidClient(crate::AsyncClient<HyperliquidClient>);
#[cfg(feature = "async")]
impl TypedHyperliquidClient {
    pub fn into_async(self) -> AsyncTypedHyperliquidClient { AsyncTypedHyperliquidClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedHyperliquidClient {
    pub async fn cancel(&self, request: &CancelRequest) -> Result<TradingResponse> {
        serde_json::from_value(self.0.cancel(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Hyperliquid response model"))
    }
    pub async fn cancel_by_cloid(&self, request: &CancelByCloidRequest) -> Result<TradingResponse> {
        serde_json::from_value(self.0.cancel_by_cloid(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Hyperliquid response model"))
    }
    pub async fn clearinghouse_state(&self, request: &UserStateRequest) -> Result<UserState> {
        serde_json::from_value(self.0.clearinghouse_state(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Hyperliquid response model"))
    }
    pub async fn l2_book(&self, request: &L2BookRequest) -> Result<L2Book> {
        serde_json::from_value(self.0.l2_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Hyperliquid response model"))
    }
    pub async fn meta(&self, request: &MetaRequest) -> Result<Meta> {
        serde_json::from_value(self.0.meta(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Hyperliquid response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<Vec<OpenOrder>> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Hyperliquid response model"))
    }
    pub async fn order(&self, request: &OrderRequest) -> Result<TradingResponse> {
        serde_json::from_value(self.0.order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Hyperliquid response model"))
    }
}
