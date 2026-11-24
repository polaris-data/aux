// Generated from src/exchanges/extended/schema/extended.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{ExtendedClient, Error, Result};

fn zero_number() -> serde_json::Number { 0.into() }
pub use crate::model_number::NativeNumber;
pub type AssetOperationStatus = String;
pub type AssetOperationType = String;
pub type AssetType = String;
pub type ExitType = String;
pub type ExportType = String;
pub type LeaderboardInterval = String;
pub type MarketTradingHours = String;
pub type MarketType = String;
pub type OrderPriceType = String;
pub type OrderSide = String;
pub type OrderSortBy = String;
pub type OrderStatus = String;
pub type OrderStatusReason = String;
pub type OrderTimeInForce = String;
pub type OrderTpslType = String;
pub type OrderTriggerPriceType = String;
pub type OrderType = String;
pub type PositionSide = String;
pub type PositionStatus = String;
pub type ReferralDashboardGranularity = String;
pub type ReferralDashboardTimeRange = String;
pub type TimeInForce = String;
pub type TradeType = String;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountLeverage {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "leverage")]
    pub leverage: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountLeverageModel {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "leverage")]
    pub leverage: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountModel {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<NativeNumber>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "accountIndex")]
    pub account_index: NativeNumber,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "l2Key")]
    pub l2_key: String,
    #[serde(rename = "l2Vault")]
    pub l2_vault: NativeNumber,
    #[serde(rename = "bridgeStarknetAddress", default, skip_serializing_if = "Option::is_none")]
    pub bridge_starknet_address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountTradeModel {
    #[serde(rename = "id")]
    pub id: NativeNumber,
    #[serde(rename = "accountId")]
    pub account_id: NativeNumber,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "orderId")]
    pub order_id: NativeNumber,
    #[serde(rename = "side")]
    pub side: OrderSide,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "qty")]
    pub qty: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "isTaker")]
    pub is_taker: bool,
    #[serde(rename = "tradeType")]
    pub trade_type: TradeType,
    #[serde(rename = "createdTime")]
    pub created_time: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountUpdateRequest {
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApiKeyResponseModel {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "ok")]
    pub ok: bool,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AssetModel {
    #[serde(rename = "id")]
    pub id: NativeNumber,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "precision")]
    pub precision: NativeNumber,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isCollateral")]
    pub is_collateral: bool,
    #[serde(rename = "starkexId")]
    pub starkex_id: String,
    #[serde(rename = "starkexResolution")]
    pub starkex_resolution: NativeNumber,
    #[serde(rename = "l1Id")]
    pub l1_id: String,
    #[serde(rename = "l1Resolution")]
    pub l1_resolution: NativeNumber,
    #[serde(rename = "version")]
    pub version: NativeNumber,
    #[serde(rename = "type")]
    pub type_: AssetType,
    #[serde(rename = "canBeUsedAsCollateral")]
    pub can_be_used_as_collateral: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AssetOperationModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub type_: AssetOperationType,
    #[serde(rename = "status")]
    pub status: AssetOperationStatus,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "asset")]
    pub asset: NativeNumber,
    #[serde(rename = "time")]
    pub time: NativeNumber,
    #[serde(rename = "accountId")]
    pub account_id: NativeNumber,
    #[serde(rename = "counterpartyAccountId", default, skip_serializing_if = "Option::is_none")]
    pub counterparty_account_id: Option<NativeNumber>,
    #[serde(rename = "transactionHash", default, skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AssetOperationsParams {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<NativeNumber>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<NativeNumber>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<NativeNumber>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AssetOperationsResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<AssetOperationModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BalanceModel {
    #[serde(rename = "collateralName")]
    pub collateral_name: String,
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "equity")]
    pub equity: String,
    #[serde(rename = "availableForTrade")]
    pub available_for_trade: String,
    #[serde(rename = "availableForWithdrawal")]
    pub available_for_withdrawal: String,
    #[serde(rename = "unrealisedPnl")]
    pub unrealised_pnl: String,
    #[serde(rename = "initialMargin")]
    pub initial_margin: String,
    #[serde(rename = "marginRatio")]
    pub margin_ratio: String,
    #[serde(rename = "updatedTime")]
    pub updated_time: NativeNumber,
    #[serde(rename = "spotEquity", default, skip_serializing_if = "Option::is_none")]
    pub spot_equity: Option<String>,
    #[serde(rename = "spotEquityForAvailableForTrade", default, skip_serializing_if = "Option::is_none")]
    pub spot_equity_for_available_for_trade: Option<String>,
    #[serde(rename = "collateralReservedForSpotOrders", default, skip_serializing_if = "Option::is_none")]
    pub collateral_reserved_for_spot_orders: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgesConfigModel {
    #[serde(rename = "chains")]
    pub chains: Vec<ChainConfigModel>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuilderTradeModel {
    #[serde(rename = "id")]
    pub id: NativeNumber,
    #[serde(rename = "time")]
    pub time: NativeNumber,
    #[serde(rename = "volume")]
    pub volume: String,
    #[serde(rename = "makerId", default, skip_serializing_if = "Option::is_none")]
    pub maker_id: Option<NativeNumber>,
    #[serde(rename = "takerId", default, skip_serializing_if = "Option::is_none")]
    pub taker_id: Option<NativeNumber>,
    #[serde(rename = "makerBuilderId", default, skip_serializing_if = "Option::is_none")]
    pub maker_builder_id: Option<NativeNumber>,
    #[serde(rename = "takerBuilderId", default, skip_serializing_if = "Option::is_none")]
    pub taker_builder_id: Option<NativeNumber>,
    #[serde(rename = "makerFee", default, skip_serializing_if = "Option::is_none")]
    pub maker_fee: Option<String>,
    #[serde(rename = "takerFee", default, skip_serializing_if = "Option::is_none")]
    pub taker_fee: Option<String>,
    #[serde(rename = "makerBuilderFee", default, skip_serializing_if = "Option::is_none")]
    pub maker_builder_fee: Option<String>,
    #[serde(rename = "takerBuilderFee", default, skip_serializing_if = "Option::is_none")]
    pub taker_builder_fee: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderByExternalIdParams {
    #[serde(rename = "externalId", default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderByExternalIdResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<EmptyModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderParams {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CandleModel {
    #[serde(rename = "open", default, skip_serializing_if = "Option::is_none")]
    pub open: Option<String>,
    #[serde(rename = "low", default, skip_serializing_if = "Option::is_none")]
    pub low: Option<String>,
    #[serde(rename = "high", default, skip_serializing_if = "Option::is_none")]
    pub high: Option<String>,
    #[serde(rename = "close", default, skip_serializing_if = "Option::is_none")]
    pub close: Option<String>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ChainConfigModel {
    #[serde(rename = "chain")]
    pub chain: String,
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClaimResponseModel {
    #[serde(rename = "id")]
    pub id: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClaimTestingFundsParams {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClaimTestingFundsResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ClaimResponseModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClientModel {
    #[serde(rename = "id")]
    pub id: NativeNumber,
    #[serde(rename = "evmWalletAddress", default, skip_serializing_if = "Option::is_none")]
    pub evm_wallet_address: Option<serde_json::Value>,
    #[serde(rename = "starknetWalletAddress", default, skip_serializing_if = "Option::is_none")]
    pub starknet_wallet_address: Option<serde_json::Value>,
    #[serde(rename = "referralLinkCode", default, skip_serializing_if = "Option::is_none")]
    pub referral_link_code: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CollateralSlowWithdrawal {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "positionId", default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CommitBridgeQuoteParams {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CommitBridgeQuoteResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<EmptyModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateApiKeyParams {
    #[serde(rename = "L1_SIGNATURE")]
    pub l1_signature: String,
    #[serde(rename = "L1_MESSAGE_TIME")]
    pub l1_message_time: String,
    #[serde(rename = "X-X10-ACTIVE-ACCOUNT")]
    pub x_x10_active_account: String,
    #[serde(rename = "body")]
    pub body: CreateApiKeyRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateApiKeyResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ApiKeyResponseModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateDepositParams {
    #[serde(rename = "body")]
    pub body: CreateDepositRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateDepositRequest {
    #[serde(rename = "accountId")]
    pub account_id: serde_json::Number,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "sourceChain")]
    pub source_chain: String,
    #[serde(rename = "snTransactionHash", default, skip_serializing_if = "Option::is_none")]
    pub sn_transaction_hash: Option<String>,
    #[serde(rename = "fee", default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename = "bridgeId", default, skip_serializing_if = "Option::is_none")]
    pub bridge_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateDepositRequest {
    fn default() -> Self {
        Self {
            account_id: 0.into(),
            asset_id: Default::default(),
            amount: Default::default(),
            source_chain: Default::default(),
            sn_transaction_hash: Default::default(),
            fee: Default::default(),
            bridge_id: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Number>,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "type")]
    pub type_: OrderType,
    #[serde(rename = "side")]
    pub side: OrderSide,
    #[serde(rename = "qty")]
    pub qty: String,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "timeInForce")]
    pub time_in_force: OrderTimeInForce,
    #[serde(rename = "expiryEpochMillis", default, skip_serializing_if = "Option::is_none")]
    pub expiry_epoch_millis: Option<serde_json::Number>,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "nonce", default = "zero_number")]
    pub nonce: serde_json::Number,
    #[serde(rename = "reduceOnly", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "postOnly", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "trigger", default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<OrderConditionalTrigger>,
    #[serde(rename = "tpSlType", default, skip_serializing_if = "Option::is_none")]
    pub tp_sl_type: Option<OrderTpslType>,
    #[serde(rename = "twap", default, skip_serializing_if = "Option::is_none")]
    pub twap: Option<Twap>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateOrderRequest {
    fn default() -> Self {
        Self {
            id: Default::default(),
            market: Default::default(),
            type_: Default::default(),
            side: Default::default(),
            qty: Default::default(),
            price: Default::default(),
            time_in_force: Default::default(),
            expiry_epoch_millis: Default::default(),
            fee: Default::default(),
            nonce: 0.into(),
            reduce_only: Default::default(),
            post_only: Default::default(),
            trigger: Default::default(),
            tp_sl_type: Default::default(),
            twap: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateRFQOrderRequest {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Number>,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "type")]
    pub type_: OrderType,
    #[serde(rename = "side")]
    pub side: OrderSide,
    #[serde(rename = "qty")]
    pub qty: String,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "timeInForce")]
    pub time_in_force: OrderTimeInForce,
    #[serde(rename = "expiryEpochMillis", default, skip_serializing_if = "Option::is_none")]
    pub expiry_epoch_millis: Option<serde_json::Number>,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "nonce", default = "zero_number")]
    pub nonce: serde_json::Number,
    #[serde(rename = "reduceOnly", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "postOnly", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "trigger", default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<OrderConditionalTrigger>,
    #[serde(rename = "tpSlType", default, skip_serializing_if = "Option::is_none")]
    pub tp_sl_type: Option<OrderTpslType>,
    #[serde(rename = "twap", default, skip_serializing_if = "Option::is_none")]
    pub twap: Option<Twap>,
    #[serde(rename = "rfqStartPrice", default, skip_serializing_if = "Option::is_none")]
    pub rfq_start_price: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateRFQOrderRequest {
    fn default() -> Self {
        Self {
            id: Default::default(),
            market: Default::default(),
            type_: Default::default(),
            side: Default::default(),
            qty: Default::default(),
            price: Default::default(),
            time_in_force: Default::default(),
            expiry_epoch_millis: Default::default(),
            fee: Default::default(),
            nonce: 0.into(),
            reduce_only: Default::default(),
            post_only: Default::default(),
            trigger: Default::default(),
            tp_sl_type: Default::default(),
            twap: Default::default(),
            rfq_start_price: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateReferralLinkCodeRequest {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateReferralLinkParams {
    #[serde(rename = "body")]
    pub body: CreateReferralLinkCodeRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateScaledOrderParams {
    #[serde(rename = "body")]
    pub body: CreateScaledOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateScaledOrderRequest {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "side")]
    pub side: OrderSide,
    #[serde(rename = "totalQty")]
    pub total_qty: String,
    #[serde(rename = "numOrders")]
    pub num_orders: serde_json::Number,
    #[serde(rename = "priceMin")]
    pub price_min: String,
    #[serde(rename = "priceMax")]
    pub price_max: String,
    #[serde(rename = "timeInForce")]
    pub time_in_force: OrderTimeInForce,
    #[serde(rename = "expiryEpochMillis", default, skip_serializing_if = "Option::is_none")]
    pub expiry_epoch_millis: Option<serde_json::Number>,
    #[serde(rename = "reduceOnly", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateScaledOrderRequest {
    fn default() -> Self {
        Self {
            market: Default::default(),
            side: Default::default(),
            total_qty: Default::default(),
            num_orders: 0.into(),
            price_min: Default::default(),
            price_max: Default::default(),
            time_in_force: Default::default(),
            expiry_epoch_millis: Default::default(),
            reduce_only: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteApiKeyParams {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteUserOrderOrderIdParams {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteUserOrderOrderIdResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<EmptyModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DepositRequestModel {
    #[serde(rename = "fromAccountId")]
    pub from_account_id: NativeNumber,
    #[serde(rename = "toAccountId")]
    pub to_account_id: NativeNumber,
    #[serde(rename = "collateral")]
    pub collateral: String,
    #[serde(rename = "shares")]
    pub shares: String,
    #[serde(rename = "settlement")]
    pub settlement: LimitOrderSettlementModel,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DepositToVaultParams {
    #[serde(rename = "body")]
    pub body: DepositRequestModel,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DepositToVaultResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct EmptyModel {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExportCsvZipParams {
    #[serde(rename = "type[]")]
    pub type_: Vec<ExportType>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FundingRateConfigRequest {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FundingRateModel {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "fundingRate", default, skip_serializing_if = "Option::is_none")]
    pub funding_rate: Option<String>,
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetAccountEquityHistoryParams {
    #[serde(rename = "accountId")]
    pub account_id: serde_json::Number,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetAccountEquityHistoryParams {
    fn default() -> Self {
        Self {
            account_id: 0.into(),
            interval: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountEquityHistoryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetAccountEquityHistoryResultDataItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountEquityHistoryResultDataItem {
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountFundingChartParams {
    #[serde(rename = "accountId")]
    pub account_id: Vec<serde_json::Number>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountFundingChartResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetAccountFundingChartResultDataItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountFundingChartResultDataItem {
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountFundingHistoryParams {
    #[serde(rename = "accountId")]
    pub account_id: Vec<serde_json::Number>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Vec<String>>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountFundingHistoryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetAccountFundingHistoryResultDataItem>>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<GetAccountFundingHistoryResultPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountFundingHistoryResultDataItem {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Number>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<serde_json::Number>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "markPrice", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "fundingFee", default, skip_serializing_if = "Option::is_none")]
    pub funding_fee: Option<String>,
    #[serde(rename = "fundingRate", default, skip_serializing_if = "Option::is_none")]
    pub funding_rate: Option<String>,
    #[serde(rename = "paidTime", default, skip_serializing_if = "Option::is_none")]
    pub paid_time: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountFundingHistoryResultPagination {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<serde_json::Number>,
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountFundingStatsParams {
    #[serde(rename = "accountId")]
    pub account_id: Vec<serde_json::Number>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountFundingStatsResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GetAccountFundingStatsResultData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountFundingStatsResultData {
    #[serde(rename = "totalFunding", default, skip_serializing_if = "Option::is_none")]
    pub total_funding: Option<String>,
    #[serde(rename = "fundingPerMarket", default, skip_serializing_if = "Option::is_none")]
    pub funding_per_market: Option<Vec<GetAccountFundingStatsResultDataFundingPerMarketItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountFundingStatsResultDataFundingPerMarketItem {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "netFundingFees", default, skip_serializing_if = "Option::is_none")]
    pub net_funding_fees: Option<String>,
    #[serde(rename = "netExposure", default, skip_serializing_if = "Option::is_none")]
    pub net_exposure: Option<String>,
    #[serde(rename = "fundingRate", default, skip_serializing_if = "Option::is_none")]
    pub funding_rate: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountHealthParams {
    #[serde(rename = "accountId")]
    pub account_id: Vec<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountHealthResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetAccountHealthResultDataItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountHealthResultDataItem {
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<serde_json::Number>,
    #[serde(rename = "equity", default, skip_serializing_if = "Option::is_none")]
    pub equity: Option<String>,
    #[serde(rename = "unrealisedPnl", default, skip_serializing_if = "Option::is_none")]
    pub unrealised_pnl: Option<String>,
    #[serde(rename = "exposure", default, skip_serializing_if = "Option::is_none")]
    pub exposure: Option<String>,
    #[serde(rename = "netExposure", default, skip_serializing_if = "Option::is_none")]
    pub net_exposure: Option<String>,
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(rename = "initialMargin", default, skip_serializing_if = "Option::is_none")]
    pub initial_margin: Option<String>,
    #[serde(rename = "marginRatio", default, skip_serializing_if = "Option::is_none")]
    pub margin_ratio: Option<String>,
    #[serde(rename = "availableForTrading", default, skip_serializing_if = "Option::is_none")]
    pub available_for_trading: Option<String>,
    #[serde(rename = "availableForSpotTrading", default, skip_serializing_if = "Option::is_none")]
    pub available_for_spot_trading: Option<String>,
    #[serde(rename = "availableForWithdrawal", default, skip_serializing_if = "Option::is_none")]
    pub available_for_withdrawal: Option<String>,
    #[serde(rename = "health", default, skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountMaxDrawdownHistoryParams {
    #[serde(rename = "accountId")]
    pub account_id: Vec<serde_json::Number>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountMaxDrawdownHistoryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetAccountMaxDrawdownHistoryResultDataItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountMaxDrawdownHistoryResultDataItem {
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<serde_json::Number>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetAccountMaxDrawdownHistoryResultDataItemDataItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountMaxDrawdownHistoryResultDataItemDataItem {
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPerformanceParams {
    #[serde(rename = "accountId")]
    pub account_id: Vec<serde_json::Number>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "marketType", default, skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPerformanceResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetAccountPerformanceResultDataItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPerformanceResultDataItem {
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<serde_json::Number>,
    #[serde(rename = "realisedPnl", default, skip_serializing_if = "Option::is_none")]
    pub realised_pnl: Option<String>,
    #[serde(rename = "unrealisedPnl", default, skip_serializing_if = "Option::is_none")]
    pub unrealised_pnl: Option<String>,
    #[serde(rename = "totalPnl", default, skip_serializing_if = "Option::is_none")]
    pub total_pnl: Option<String>,
    #[serde(rename = "totalPnlPercentage", default, skip_serializing_if = "Option::is_none")]
    pub total_pnl_percentage: Option<String>,
    #[serde(rename = "profitFactor", default, skip_serializing_if = "Option::is_none")]
    pub profit_factor: Option<String>,
    #[serde(rename = "maxDrawdown", default, skip_serializing_if = "Option::is_none")]
    pub max_drawdown: Option<String>,
    #[serde(rename = "winRate", default, skip_serializing_if = "Option::is_none")]
    pub win_rate: Option<String>,
    #[serde(rename = "tradingVolume", default, skip_serializing_if = "Option::is_none")]
    pub trading_volume: Option<String>,
    #[serde(rename = "makerVolume", default, skip_serializing_if = "Option::is_none")]
    pub maker_volume: Option<String>,
    #[serde(rename = "sharpeRatio", default, skip_serializing_if = "Option::is_none")]
    pub sharpe_ratio: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetAccountPnLHistoryParams {
    #[serde(rename = "accountId")]
    pub account_id: serde_json::Number,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "pnlType")]
    pub pnl_type: String,
    #[serde(rename = "instrumentType", default, skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetAccountPnLHistoryParams {
    fn default() -> Self {
        Self {
            account_id: 0.into(),
            interval: Default::default(),
            pnl_type: Default::default(),
            instrument_type: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPnLHistoryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetAccountPnLHistoryResultDataItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPnLHistoryResultDataItem {
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPnLPercentageHistoryParams {
    #[serde(rename = "accountId")]
    pub account_id: Vec<serde_json::Number>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "pnlType")]
    pub pnl_type: String,
    #[serde(rename = "priceMarket", default, skip_serializing_if = "Option::is_none")]
    pub price_market: Option<Vec<String>>,
    #[serde(rename = "instrumentType", default, skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPnLPercentageHistoryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GetAccountPnLPercentageHistoryResultData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPnLPercentageHistoryResultData {
    #[serde(rename = "pnl", default, skip_serializing_if = "Option::is_none")]
    pub pnl: Option<Vec<GetAccountPnLPercentageHistoryResultDataPnlItem>>,
    #[serde(rename = "priceChanges", default, skip_serializing_if = "Option::is_none")]
    pub price_changes: Option<GetAccountPnLPercentageHistoryResultDataPriceChanges>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPnLPercentageHistoryResultDataPnlItem {
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPnLPercentageHistoryResultDataPriceChanges {
    #[serde(rename = "BTC-USD", default, skip_serializing_if = "Option::is_none")]
    pub btc_usd: Option<Vec<GetAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItem {
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPortfolioSummaryParams {
    #[serde(rename = "accountId")]
    pub account_id: Vec<serde_json::Number>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "instrumentType", default, skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPortfolioSummaryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GetAccountPortfolioSummaryResultData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPortfolioSummaryResultData {
    #[serde(rename = "equity", default, skip_serializing_if = "Option::is_none")]
    pub equity: Option<String>,
    #[serde(rename = "accountValue", default, skip_serializing_if = "Option::is_none")]
    pub account_value: Option<GetAccountPortfolioSummaryResultDataAccountValue>,
    #[serde(rename = "exposure", default, skip_serializing_if = "Option::is_none")]
    pub exposure: Option<String>,
    #[serde(rename = "totalPnl", default, skip_serializing_if = "Option::is_none")]
    pub total_pnl: Option<String>,
    #[serde(rename = "profitFactor", default, skip_serializing_if = "Option::is_none")]
    pub profit_factor: Option<String>,
    #[serde(rename = "winRate", default, skip_serializing_if = "Option::is_none")]
    pub win_rate: Option<String>,
    #[serde(rename = "tradingVolume", default, skip_serializing_if = "Option::is_none")]
    pub trading_volume: Option<String>,
    #[serde(rename = "makerVolume", default, skip_serializing_if = "Option::is_none")]
    pub maker_volume: Option<String>,
    #[serde(rename = "sharpeRatio", default, skip_serializing_if = "Option::is_none")]
    pub sharpe_ratio: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountPortfolioSummaryResultDataAccountValue {
    #[serde(rename = "accountNotionalValue", default, skip_serializing_if = "Option::is_none")]
    pub account_notional_value: Option<String>,
    #[serde(rename = "tradingEquity", default, skip_serializing_if = "Option::is_none")]
    pub trading_equity: Option<String>,
    #[serde(rename = "vaultNotionalValue", default, skip_serializing_if = "Option::is_none")]
    pub vault_notional_value: Option<String>,
    #[serde(rename = "spotNotionalValue", default, skip_serializing_if = "Option::is_none")]
    pub spot_notional_value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<AccountModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountVaultEquityHistoryParams {
    #[serde(rename = "accountId")]
    pub account_id: Vec<serde_json::Number>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountVaultEquityHistoryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetAccountVaultEquityHistoryResultDataItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountVaultEquityHistoryResultDataItem {
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountsResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<AccountModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAffiliateParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAffiliateRewardsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetApiKeysParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAssetOperationsParams {
    #[serde(rename = "accountId[]", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Vec<serde_json::Number>>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type[]", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<Vec<String>>,
    #[serde(rename = "status[]", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<String>>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<serde_json::Number>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAssetPriceParams {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAssetsParams {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<AssetType>,
    #[serde(rename = "asset", default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<Vec<String>>,
    #[serde(rename = "collateral", default, skip_serializing_if = "Option::is_none")]
    pub collateral: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAssetsResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<AssetModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBalanceParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBalanceResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BalanceModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBridgeConfigParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBridgeConfigResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BridgesConfigModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBridgeQuoteParams {
    #[serde(rename = "chainIn", default, skip_serializing_if = "Option::is_none")]
    pub chain_in: Option<String>,
    #[serde(rename = "chainOut", default, skip_serializing_if = "Option::is_none")]
    pub chain_out: Option<String>,
    #[serde(rename = "amount", default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBridgeQuoteResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<QuoteModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBuilderDashboardParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBuilderTradesParams {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<NativeNumber>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBuilderTradesResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BuilderTradeModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCandlesHistoryParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "candle_type")]
    pub candle_type: String,
    #[serde(rename = "interval", default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCandlesHistoryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<CandleModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetClaimParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetClaimParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetClaimsParams {
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetClientParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetClientResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ClientModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetCumulativeAccountPnLHistoryParams {
    #[serde(rename = "accountId")]
    pub account_id: serde_json::Number,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "pnlType")]
    pub pnl_type: String,
    #[serde(rename = "instrumentType", default, skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetCumulativeAccountPnLHistoryParams {
    fn default() -> Self {
        Self {
            account_id: 0.into(),
            interval: Default::default(),
            pnl_type: Default::default(),
            instrument_type: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCumulativeAccountPnLHistoryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetCumulativeAccountPnLHistoryResultDataItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCumulativeAccountPnLHistoryResultDataItem {
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCumulativeAccountPnLPercentageHistoryParams {
    #[serde(rename = "accountId")]
    pub account_id: Vec<serde_json::Number>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "pnlType")]
    pub pnl_type: String,
    #[serde(rename = "priceMarket", default, skip_serializing_if = "Option::is_none")]
    pub price_market: Option<Vec<String>>,
    #[serde(rename = "instrumentType", default, skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCumulativeAccountPnLPercentageHistoryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GetCumulativeAccountPnLPercentageHistoryResultData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCumulativeAccountPnLPercentageHistoryResultData {
    #[serde(rename = "pnl", default, skip_serializing_if = "Option::is_none")]
    pub pnl: Option<Vec<GetCumulativeAccountPnLPercentageHistoryResultDataPnlItem>>,
    #[serde(rename = "priceChanges", default, skip_serializing_if = "Option::is_none")]
    pub price_changes: Option<GetCumulativeAccountPnLPercentageHistoryResultDataPriceChanges>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCumulativeAccountPnLPercentageHistoryResultDataPnlItem {
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCumulativeAccountPnLPercentageHistoryResultDataPriceChanges {
    #[serde(rename = "BTC-USD", default, skip_serializing_if = "Option::is_none")]
    pub btc_usd: Option<Vec<GetCumulativeAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCumulativeAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItem {
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEarnedRewardsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetExchangeStatusParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFeesParams {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Vec<String>>,
    #[serde(rename = "builderId", default, skip_serializing_if = "Option::is_none")]
    pub builder_id: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFeesResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TradingFeeModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFundingRatesHistoryParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFundingRatesHistoryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<FundingRateModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetIndexPriceCandlesParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInfoAssetsAssetNamePriceParams {
    #[serde(rename = "asset_name")]
    pub asset_name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInfoAssetsAssetNamePriceResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetInfoCandlesMarketCandleTypeParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "candleType")]
    pub candle_type: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetInfoCandlesMarketCandleTypeParams {
    fn default() -> Self {
        Self {
            market: Default::default(),
            candle_type: Default::default(),
            interval: Default::default(),
            limit: 0.into(),
            end_time: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInfoCandlesMarketCandleTypeResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetInfoCandlesMarketCandleTypeResultDataItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInfoCandlesMarketCandleTypeResultDataItem {
    #[serde(rename = "o", default, skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,
    #[serde(rename = "l", default, skip_serializing_if = "Option::is_none")]
    pub l: Option<String>,
    #[serde(rename = "h", default, skip_serializing_if = "Option::is_none")]
    pub h: Option<String>,
    #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(rename = "v", default, skip_serializing_if = "Option::is_none")]
    pub v: Option<String>,
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestDailyMetricsParams {
    #[serde(rename = "accountId")]
    pub account_id: Vec<serde_json::Number>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestDailyMetricsResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GetInterestDailyMetricsResultData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestDailyMetricsResultData {
    #[serde(rename = "1000", default, skip_serializing_if = "Option::is_none")]
    pub n_1000: Option<Vec<GetInterestDailyMetricsResultData1000Item>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestDailyMetricsResultData1000Item {
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "assetId", default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<serde_json::Number>,
    #[serde(rename = "interestPaid", default, skip_serializing_if = "Option::is_none")]
    pub interest_paid: Option<String>,
    #[serde(rename = "averageBorrowed", default, skip_serializing_if = "Option::is_none")]
    pub average_borrowed: Option<String>,
    #[serde(rename = "annualizedInterestRate", default, skip_serializing_if = "Option::is_none")]
    pub annualized_interest_rate: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestKeyMetricsParams {
    #[serde(rename = "accountId")]
    pub account_id: Vec<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestKeyMetricsResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GetInterestKeyMetricsResultData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestKeyMetricsResultData {
    #[serde(rename = "borrowed", default, skip_serializing_if = "Option::is_none")]
    pub borrowed: Option<String>,
    #[serde(rename = "currentInterestRate", default, skip_serializing_if = "Option::is_none")]
    pub current_interest_rate: Option<String>,
    #[serde(rename = "estimatedNextPayment", default, skip_serializing_if = "Option::is_none")]
    pub estimated_next_payment: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestPaymentChartParams {
    #[serde(rename = "accountId")]
    pub account_id: Vec<serde_json::Number>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "bucket", default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestPaymentChartResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GetInterestPaymentChartResultData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestPaymentChartResultData {
    #[serde(rename = "resolution", default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(rename = "points", default, skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<GetInterestPaymentChartResultDataPointsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestPaymentChartResultDataPointsItem {
    #[serde(rename = "bucketStartTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub bucket_start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "totalInterestPaid", default, skip_serializing_if = "Option::is_none")]
    pub total_interest_paid: Option<String>,
    #[serde(rename = "averageBorrowed", default, skip_serializing_if = "Option::is_none")]
    pub average_borrowed: Option<String>,
    #[serde(rename = "averageApr", default, skip_serializing_if = "Option::is_none")]
    pub average_apr: Option<String>,
    #[serde(rename = "borrowingByCollateralAsset", default, skip_serializing_if = "Option::is_none")]
    pub borrowing_by_collateral_asset: Option<Vec<GetInterestPaymentChartResultDataPointsItemBorrowingByCollateralAssetItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestPaymentChartResultDataPointsItemBorrowingByCollateralAssetItem {
    #[serde(rename = "assetId", default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<serde_json::Number>,
    #[serde(rename = "averageBorrowed", default, skip_serializing_if = "Option::is_none")]
    pub average_borrowed: Option<String>,
    #[serde(rename = "averageApr", default, skip_serializing_if = "Option::is_none")]
    pub average_apr: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestPaymentsHistoryParams {
    #[serde(rename = "accountId")]
    pub account_id: Vec<serde_json::Number>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestPaymentsHistoryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GetInterestPaymentsHistoryResultData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestPaymentsHistoryResultData {
    #[serde(rename = "items", default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<GetInterestPaymentsHistoryResultDataItemsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestPaymentsHistoryResultDataItemsItem {
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<serde_json::Number>,
    #[serde(rename = "paymentAmount", default, skip_serializing_if = "Option::is_none")]
    pub payment_amount: Option<String>,
    #[serde(rename = "borrowedAmount", default, skip_serializing_if = "Option::is_none")]
    pub borrowed_amount: Option<String>,
    #[serde(rename = "effectiveOneHourRate", default, skip_serializing_if = "Option::is_none")]
    pub effective_one_hour_rate: Option<String>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestRateCurvesHistoryParams {
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestRateCurvesHistoryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GetInterestRateCurvesHistoryResultData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestRateCurvesHistoryResultData {
    #[serde(rename = "items", default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<GetInterestRateCurvesHistoryResultDataItemsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestRateCurvesHistoryResultDataItemsItem {
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<serde_json::Number>,
    #[serde(rename = "amountBorrowed", default, skip_serializing_if = "Option::is_none")]
    pub amount_borrowed: Option<String>,
    #[serde(rename = "globalCurve", default, skip_serializing_if = "Option::is_none")]
    pub global_curve: Option<GetInterestRateCurvesHistoryResultDataItemsItemGlobalCurve>,
    #[serde(rename = "assetCurves", default, skip_serializing_if = "Option::is_none")]
    pub asset_curves: Option<Vec<GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItem {
    #[serde(rename = "assetId", default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<serde_json::Number>,
    #[serde(rename = "curve", default, skip_serializing_if = "Option::is_none")]
    pub curve: Option<GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItemCurve>,
    #[serde(rename = "calculatedRate", default, skip_serializing_if = "Option::is_none")]
    pub calculated_rate: Option<String>,
    #[serde(rename = "amountBorrowed", default, skip_serializing_if = "Option::is_none")]
    pub amount_borrowed: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItemCurve {
    #[serde(rename = "currentUtilization", default, skip_serializing_if = "Option::is_none")]
    pub current_utilization: Option<String>,
    #[serde(rename = "currentRate", default, skip_serializing_if = "Option::is_none")]
    pub current_rate: Option<String>,
    #[serde(rename = "targetUtilization", default, skip_serializing_if = "Option::is_none")]
    pub target_utilization: Option<String>,
    #[serde(rename = "minRate", default, skip_serializing_if = "Option::is_none")]
    pub min_rate: Option<String>,
    #[serde(rename = "targetRate", default, skip_serializing_if = "Option::is_none")]
    pub target_rate: Option<String>,
    #[serde(rename = "maxRate", default, skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestRateCurvesHistoryResultDataItemsItemGlobalCurve {
    #[serde(rename = "currentUtilization", default, skip_serializing_if = "Option::is_none")]
    pub current_utilization: Option<String>,
    #[serde(rename = "currentRate", default, skip_serializing_if = "Option::is_none")]
    pub current_rate: Option<String>,
    #[serde(rename = "targetUtilization", default, skip_serializing_if = "Option::is_none")]
    pub target_utilization: Option<String>,
    #[serde(rename = "minRate", default, skip_serializing_if = "Option::is_none")]
    pub min_rate: Option<String>,
    #[serde(rename = "targetRate", default, skip_serializing_if = "Option::is_none")]
    pub target_rate: Option<String>,
    #[serde(rename = "maxRate", default, skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLatestInterestRateCurveParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLatestInterestRateCurveResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GetLatestInterestRateCurveResultData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLatestInterestRateCurveResultData {
    #[serde(rename = "item", default, skip_serializing_if = "Option::is_none")]
    pub item: Option<GetLatestInterestRateCurveResultDataItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLatestInterestRateCurveResultDataItem {
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<serde_json::Number>,
    #[serde(rename = "amountBorrowed", default, skip_serializing_if = "Option::is_none")]
    pub amount_borrowed: Option<String>,
    #[serde(rename = "globalCurve", default, skip_serializing_if = "Option::is_none")]
    pub global_curve: Option<GetLatestInterestRateCurveResultDataItemGlobalCurve>,
    #[serde(rename = "assetCurves", default, skip_serializing_if = "Option::is_none")]
    pub asset_curves: Option<Vec<GetLatestInterestRateCurveResultDataItemAssetCurvesItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLatestInterestRateCurveResultDataItemAssetCurvesItem {
    #[serde(rename = "assetId", default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<serde_json::Number>,
    #[serde(rename = "curve", default, skip_serializing_if = "Option::is_none")]
    pub curve: Option<GetLatestInterestRateCurveResultDataItemAssetCurvesItemCurve>,
    #[serde(rename = "calculatedRate", default, skip_serializing_if = "Option::is_none")]
    pub calculated_rate: Option<String>,
    #[serde(rename = "amountBorrowed", default, skip_serializing_if = "Option::is_none")]
    pub amount_borrowed: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLatestInterestRateCurveResultDataItemAssetCurvesItemCurve {
    #[serde(rename = "currentUtilization", default, skip_serializing_if = "Option::is_none")]
    pub current_utilization: Option<String>,
    #[serde(rename = "currentRate", default, skip_serializing_if = "Option::is_none")]
    pub current_rate: Option<String>,
    #[serde(rename = "targetUtilization", default, skip_serializing_if = "Option::is_none")]
    pub target_utilization: Option<String>,
    #[serde(rename = "minRate", default, skip_serializing_if = "Option::is_none")]
    pub min_rate: Option<String>,
    #[serde(rename = "targetRate", default, skip_serializing_if = "Option::is_none")]
    pub target_rate: Option<String>,
    #[serde(rename = "maxRate", default, skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLatestInterestRateCurveResultDataItemGlobalCurve {
    #[serde(rename = "currentUtilization", default, skip_serializing_if = "Option::is_none")]
    pub current_utilization: Option<String>,
    #[serde(rename = "currentRate", default, skip_serializing_if = "Option::is_none")]
    pub current_rate: Option<String>,
    #[serde(rename = "targetUtilization", default, skip_serializing_if = "Option::is_none")]
    pub target_utilization: Option<String>,
    #[serde(rename = "minRate", default, skip_serializing_if = "Option::is_none")]
    pub min_rate: Option<String>,
    #[serde(rename = "targetRate", default, skip_serializing_if = "Option::is_none")]
    pub target_rate: Option<String>,
    #[serde(rename = "maxRate", default, skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLeverageParams {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLeverageResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<AccountLeverageModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarkPriceCandlesParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketStatisticsParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketStatisticsResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<MarketStatsModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketTradesParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketsParams {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketsResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MarketModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketsStatsParams {
    #[serde(rename = "market[]", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetNextClaimTimeParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOpenInterestsParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "interval", default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<serde_json::Number>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOpenOrdersParams {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Vec<String>>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<OrderType>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOpenOrdersResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OpenOrderModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderByExternalIdParams {
    #[serde(rename = "externalId")]
    pub external_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderByIdParams {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderByIdResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<OpenOrderModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderParams {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderbookSnapshotParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderbookSnapshotResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<OrderbookUpdateModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrdersHistoryParams {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Vec<String>>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<OrderType>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<NativeNumber>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(rename = "sort", default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<OrderSortBy>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Number>,
    #[serde(rename = "externalId", default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrdersHistoryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OpenOrderModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPartnerRewardsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPositionsHistoryParams {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Vec<String>>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<PositionSide>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<NativeNumber>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPositionsHistoryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<PositionHistoryModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPositionsParams {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Vec<String>>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<PositionSide>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPositionsResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<PositionModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPublicLeaderboardParams {
    #[serde(rename = "period")]
    pub period: LeaderboardInterval,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPublicReferralLinkParams {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRebateStatsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRebatesCsvParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetReferralDashboardParams {
    #[serde(rename = "period")]
    pub period: ReferralDashboardTimeRange,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetReferralLinksParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetReferralRebatesByLinkCsvParams {
    #[serde(rename = "period")]
    pub period: ReferralDashboardTimeRange,
    #[serde(rename = "links[]", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetReferralRebatesByPeriodCsvParams {
    #[serde(rename = "period")]
    pub period: ReferralDashboardTimeRange,
    #[serde(rename = "granularity")]
    pub granularity: ReferralDashboardGranularity,
    #[serde(rename = "links[]", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "subaffiliates[]", default, skip_serializing_if = "Option::is_none")]
    pub subaffiliates: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetReferralRebatesBySubaffiliateCsvParams {
    #[serde(rename = "period")]
    pub period: ReferralDashboardTimeRange,
    #[serde(rename = "subaffiliates[]", default, skip_serializing_if = "Option::is_none")]
    pub subaffiliates: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetReferralRebatesByUserCsvParams {
    #[serde(rename = "period")]
    pub period: ReferralDashboardTimeRange,
    #[serde(rename = "links[]", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "subaffiliates[]", default, skip_serializing_if = "Option::is_none")]
    pub subaffiliates: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetReferralStatusParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRewardLeaderboardStatsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSettingsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSettingsResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<SettingsModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSpotBalancesParams {
    #[serde(rename = "accountId[]", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Vec<serde_json::Number>>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id_value: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSpotBalancesResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<SpotBalanceModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetStarknetDomainParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTradesParams {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Vec<String>>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<TradeType>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<NativeNumber>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTradesResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<AccountTradeModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTradingCandlesParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTradingFeeStatsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetUserFundingHistoryParams {
    #[serde(rename = "market[]", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Vec<String>>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<PositionSide>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market_value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetUserLeaderboardParams {
    #[serde(rename = "period")]
    pub period: LeaderboardInterval,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetUserOrdersExternalExternalIdParams {
    #[serde(rename = "external_id")]
    pub external_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetUserOrdersExternalExternalIdResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OpenOrderModel>>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultFundingHistoryParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultOrdersParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultPerformanceParams {
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultPerformanceResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GetVaultPerformanceResultData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultPerformanceResultData {
    #[serde(rename = "totalPnl", default, skip_serializing_if = "Option::is_none")]
    pub total_pnl: Option<String>,
    #[serde(rename = "aprPercentage", default, skip_serializing_if = "Option::is_none")]
    pub apr_percentage: Option<String>,
    #[serde(rename = "profitFactor", default, skip_serializing_if = "Option::is_none")]
    pub profit_factor: Option<String>,
    #[serde(rename = "winRate", default, skip_serializing_if = "Option::is_none")]
    pub win_rate: Option<String>,
    #[serde(rename = "maxDrawdown", default, skip_serializing_if = "Option::is_none")]
    pub max_drawdown: Option<String>,
    #[serde(rename = "tradingVolume", default, skip_serializing_if = "Option::is_none")]
    pub trading_volume: Option<String>,
    #[serde(rename = "sharpeRatio", default, skip_serializing_if = "Option::is_none")]
    pub sharpe_ratio: Option<String>,
    #[serde(rename = "aprBreakdown", default, skip_serializing_if = "Option::is_none")]
    pub apr_breakdown: Option<GetVaultPerformanceResultDataAprBreakdown>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultPerformanceResultDataAprBreakdown {
    #[serde(rename = "baseApr", default, skip_serializing_if = "Option::is_none")]
    pub base_apr: Option<String>,
    #[serde(rename = "maxExtraApr", default, skip_serializing_if = "Option::is_none")]
    pub max_extra_apr: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultPositionHistoryParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultPositionsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultSummaryParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultSummaryResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GetVaultSummaryResultData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultSummaryResultData {
    #[serde(rename = "equity", default, skip_serializing_if = "Option::is_none")]
    pub equity: Option<String>,
    #[serde(rename = "walletBalance", default, skip_serializing_if = "Option::is_none")]
    pub wallet_balance: Option<String>,
    #[serde(rename = "lastMonthApr", default, skip_serializing_if = "Option::is_none")]
    pub last_month_apr: Option<String>,
    #[serde(rename = "exposure", default, skip_serializing_if = "Option::is_none")]
    pub exposure: Option<String>,
    #[serde(rename = "numberOfDepositors", default, skip_serializing_if = "Option::is_none")]
    pub number_of_depositors: Option<serde_json::Number>,
    #[serde(rename = "ageDays", default, skip_serializing_if = "Option::is_none")]
    pub age_days: Option<serde_json::Number>,
    #[serde(rename = "profitShare", default, skip_serializing_if = "Option::is_none")]
    pub profit_share: Option<String>,
    #[serde(rename = "aprBreakdown", default, skip_serializing_if = "Option::is_none")]
    pub apr_breakdown: Option<GetVaultSummaryResultDataAprBreakdown>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultSummaryResultDataAprBreakdown {
    #[serde(rename = "baseApr", default, skip_serializing_if = "Option::is_none")]
    pub base_apr: Option<String>,
    #[serde(rename = "maxExtraApr", default, skip_serializing_if = "Option::is_none")]
    pub max_extra_apr: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultTradesParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWalletValidationParams {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWithdrawalsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWrapperParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct L2ConfigModel {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "collateralId")]
    pub collateral_id: String,
    #[serde(rename = "collateralResolution")]
    pub collateral_resolution: NativeNumber,
    #[serde(rename = "syntheticId")]
    pub synthetic_id: String,
    #[serde(rename = "syntheticResolution")]
    pub synthetic_resolution: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LimitOrderSettlementModel {
    #[serde(rename = "baseAmount")]
    pub base_amount: NativeNumber,
    #[serde(rename = "quoteAmount")]
    pub quote_amount: NativeNumber,
    #[serde(rename = "feeAmount")]
    pub fee_amount: NativeNumber,
    #[serde(rename = "baseAssetId")]
    pub base_asset_id: String,
    #[serde(rename = "quoteAssetId")]
    pub quote_asset_id: String,
    #[serde(rename = "feeAssetId")]
    pub fee_asset_id: String,
    #[serde(rename = "expirationTimestamp")]
    pub expiration_timestamp: NativeNumber,
    #[serde(rename = "nonce")]
    pub nonce: NativeNumber,
    #[serde(rename = "receiverPositionId")]
    pub receiver_position_id: NativeNumber,
    #[serde(rename = "senderPositionId")]
    pub sender_position_id: NativeNumber,
    #[serde(rename = "signature")]
    pub signature: SettlementSignatureModel,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarkWalletCheckedParams {
    #[serde(rename = "passed")]
    pub passed: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketModel {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub type_: MarketType,
    #[serde(rename = "assetName")]
    pub asset_name: String,
    #[serde(rename = "assetPrecision")]
    pub asset_precision: NativeNumber,
    #[serde(rename = "collateralAssetName")]
    pub collateral_asset_name: String,
    #[serde(rename = "collateralAssetPrecision")]
    pub collateral_asset_precision: NativeNumber,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "isRfq")]
    pub is_rfq: bool,
    #[serde(rename = "isOffHours")]
    pub is_off_hours: bool,
    #[serde(rename = "tradingHours")]
    pub trading_hours: MarketTradingHours,
    #[serde(rename = "marketStats")]
    pub market_stats: MarketStatsModel,
    #[serde(rename = "tradingConfig")]
    pub trading_config: TradingConfigModel,
    #[serde(rename = "l2Config")]
    pub l2_config: L2ConfigModel,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketStatsModel {
    #[serde(rename = "dailyVolume")]
    pub daily_volume: String,
    #[serde(rename = "dailyVolumeBase")]
    pub daily_volume_base: String,
    #[serde(rename = "dailyPriceChange")]
    pub daily_price_change: String,
    #[serde(rename = "dailyLow")]
    pub daily_low: String,
    #[serde(rename = "dailyHigh")]
    pub daily_high: String,
    #[serde(rename = "lastPrice")]
    pub last_price: String,
    #[serde(rename = "askPrice")]
    pub ask_price: String,
    #[serde(rename = "bidPrice")]
    pub bid_price: String,
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    #[serde(rename = "indexPrice")]
    pub index_price: String,
    #[serde(rename = "fundingRate")]
    pub funding_rate: String,
    #[serde(rename = "nextFundingRate")]
    pub next_funding_rate: NativeNumber,
    #[serde(rename = "openInterest")]
    pub open_interest: String,
    #[serde(rename = "openInterestBase")]
    pub open_interest_base: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MassCancelParams {
    #[serde(rename = "body")]
    pub body: OrderCancelRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MassCancelResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<EmptyModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OnChainTransfer {
    #[serde(rename = "fromVault")]
    pub from_vault: String,
    #[serde(rename = "toVault")]
    pub to_vault: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "transferredAsset")]
    pub transferred_asset: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OnboardClientParams {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OnboardClientResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<OnboardedClientModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OnboardSubaccountParams {
    #[serde(rename = "L1_SIGNATURE")]
    pub l1_signature: String,
    #[serde(rename = "L1_MESSAGE_TIME")]
    pub l1_message_time: String,
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OnboardSubaccountResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<AccountModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OnboardedClientModel {
    #[serde(rename = "l1Address")]
    pub l1_address: String,
    #[serde(rename = "defaultAccount", default)]
    pub default_account: AccountModel,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OnchainWithdrawalParams {
    #[serde(rename = "body")]
    pub body: CollateralSlowWithdrawal,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrderModel {
    #[serde(rename = "id")]
    pub id: NativeNumber,
    #[serde(rename = "accountId")]
    pub account_id: NativeNumber,
    #[serde(rename = "externalId")]
    pub external_id: String,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "type")]
    pub type_: OrderType,
    #[serde(rename = "side")]
    pub side: OrderSide,
    #[serde(rename = "status")]
    pub status: OrderStatus,
    #[serde(rename = "statusReason", default, skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<OrderStatusReason>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "averagePrice", default, skip_serializing_if = "Option::is_none")]
    pub average_price: Option<String>,
    #[serde(rename = "rfqStartPrice", default, skip_serializing_if = "Option::is_none")]
    pub rfq_start_price: Option<String>,
    #[serde(rename = "qty")]
    pub qty: String,
    #[serde(rename = "filledQty", default, skip_serializing_if = "Option::is_none")]
    pub filled_qty: Option<String>,
    #[serde(rename = "cancelledQty", default, skip_serializing_if = "Option::is_none")]
    pub cancelled_qty: Option<String>,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "postOnly")]
    pub post_only: bool,
    #[serde(rename = "chaseOrder", default, skip_serializing_if = "Option::is_none")]
    pub chase_order: Option<bool>,
    #[serde(rename = "payedFee", default, skip_serializing_if = "Option::is_none")]
    pub payed_fee: Option<String>,
    #[serde(rename = "createdTime")]
    pub created_time: NativeNumber,
    #[serde(rename = "updatedTime")]
    pub updated_time: NativeNumber,
    #[serde(rename = "expiryTime", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<NativeNumber>,
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "tpSlType", default, skip_serializing_if = "Option::is_none")]
    pub tp_sl_type: Option<OrderTpslType>,
    #[serde(rename = "takeProfit", default, skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<OpenOrderTpslTriggerModel>,
    #[serde(rename = "stopLoss", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<OpenOrderTpslTriggerModel>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrderTpslTriggerModel {
    #[serde(rename = "triggerPrice")]
    pub trigger_price: String,
    #[serde(rename = "triggerPriceType")]
    pub trigger_price_type: OrderTriggerPriceType,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "priceType")]
    pub price_type: OrderPriceType,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderCancelRequest {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
    #[serde(rename = "ids", default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<serde_json::Number>>,
    #[serde(rename = "externalIds", default, skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderConditionalTrigger {
    #[serde(rename = "triggerPrice", default, skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "triggerType", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderbookQuantityModel {
    #[serde(rename = "qty", default, skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderbookUpdateModel {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "bid", default, skip_serializing_if = "Option::is_none")]
    pub bid: Option<Vec<OrderbookQuantityModel>>,
    #[serde(rename = "ask", default, skip_serializing_if = "Option::is_none")]
    pub ask: Option<Vec<OrderbookQuantityModel>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderParams {
    #[serde(rename = "body")]
    pub body: CreateOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<PlacedOrderModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceRfqOrderParams {
    #[serde(rename = "body")]
    pub body: CreateRFQOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceRfqOrderResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<PlacedOrderModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlacedOrderModel {
    #[serde(rename = "id")]
    pub id: NativeNumber,
    #[serde(rename = "externalId")]
    pub external_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionHistoryModel {
    #[serde(rename = "id")]
    pub id: NativeNumber,
    #[serde(rename = "accountId")]
    pub account_id: NativeNumber,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "side")]
    pub side: PositionSide,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "maxPositionSize")]
    pub max_position_size: String,
    #[serde(rename = "leverage")]
    pub leverage: String,
    #[serde(rename = "openPrice")]
    pub open_price: String,
    #[serde(rename = "exitPrice", default, skip_serializing_if = "Option::is_none")]
    pub exit_price: Option<String>,
    #[serde(rename = "realisedPnl")]
    pub realised_pnl: String,
    #[serde(rename = "realisedPnlBreakdown")]
    pub realised_pnl_breakdown: RealisedPnlBreakdownModel,
    #[serde(rename = "createdTime")]
    pub created_time: NativeNumber,
    #[serde(rename = "exitType", default, skip_serializing_if = "Option::is_none")]
    pub exit_type: Option<ExitType>,
    #[serde(rename = "closedTime", default, skip_serializing_if = "Option::is_none")]
    pub closed_time: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionModel {
    #[serde(rename = "id")]
    pub id: NativeNumber,
    #[serde(rename = "accountId")]
    pub account_id: NativeNumber,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "status")]
    pub status: PositionStatus,
    #[serde(rename = "side")]
    pub side: PositionSide,
    #[serde(rename = "leverage")]
    pub leverage: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "openPrice")]
    pub open_price: String,
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    #[serde(rename = "liquidationPrice", default, skip_serializing_if = "Option::is_none")]
    pub liquidation_price: Option<String>,
    #[serde(rename = "unrealisedPnl")]
    pub unrealised_pnl: String,
    #[serde(rename = "realisedPnl")]
    pub realised_pnl: String,
    #[serde(rename = "tpPrice", default, skip_serializing_if = "Option::is_none")]
    pub tp_price: Option<String>,
    #[serde(rename = "slPrice", default, skip_serializing_if = "Option::is_none")]
    pub sl_price: Option<String>,
    #[serde(rename = "adl", default, skip_serializing_if = "Option::is_none")]
    pub adl: Option<NativeNumber>,
    #[serde(rename = "createdAt")]
    pub created_at: NativeNumber,
    #[serde(rename = "updatedAt")]
    pub updated_at: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PostUserTransferOnchainParams {
    #[serde(rename = "body")]
    pub body: OnChainTransfer,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PostUserTransferOnchainResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<TransferResponseModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct QuoteModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RealisedPnlBreakdownModel {
    #[serde(rename = "tradePnl")]
    pub trade_pnl: String,
    #[serde(rename = "fundingFees")]
    pub funding_fees: String,
    #[serde(rename = "openFees")]
    pub open_fees: String,
    #[serde(rename = "closeFees")]
    pub close_fees: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RiskFactorConfigModel {
    #[serde(rename = "upperBound")]
    pub upper_bound: String,
    #[serde(rename = "riskFactor")]
    pub risk_factor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SetDeadmanSwitchParams {
    #[serde(rename = "countdownTime")]
    pub countdown_time: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SetDeadmanSwitchParams {
    fn default() -> Self {
        Self {
            countdown_time: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SettingsModel {
    #[serde(rename = "starkExContractAddress")]
    pub stark_ex_contract_address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SettlementSignatureModel {
    #[serde(rename = "r")]
    pub r: String,
    #[serde(rename = "s")]
    pub s: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SpotBalanceModel {
    #[serde(rename = "accountId")]
    pub account_id: NativeNumber,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "indexPrice")]
    pub index_price: String,
    #[serde(rename = "notionalValue")]
    pub notional_value: String,
    #[serde(rename = "contributionFactor")]
    pub contribution_factor: String,
    #[serde(rename = "equityContribution")]
    pub equity_contribution: String,
    #[serde(rename = "availableToWithdraw", default, skip_serializing_if = "Option::is_none")]
    pub available_to_withdraw: Option<String>,
    #[serde(rename = "absolutePnl", default, skip_serializing_if = "Option::is_none")]
    pub absolute_pnl: Option<String>,
    #[serde(rename = "pnlPercentage", default, skip_serializing_if = "Option::is_none")]
    pub pnl_percentage: Option<String>,
    #[serde(rename = "averageEntryPrice", default, skip_serializing_if = "Option::is_none")]
    pub average_entry_price: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradingConfigModel {
    #[serde(rename = "minOrderSize")]
    pub min_order_size: String,
    #[serde(rename = "minOrderSizeChange")]
    pub min_order_size_change: String,
    #[serde(rename = "minPriceChange")]
    pub min_price_change: String,
    #[serde(rename = "maxMarketOrderValue")]
    pub max_market_order_value: String,
    #[serde(rename = "maxLimitOrderValue")]
    pub max_limit_order_value: String,
    #[serde(rename = "maxPositionValue")]
    pub max_position_value: String,
    #[serde(rename = "maxLeverage")]
    pub max_leverage: String,
    #[serde(rename = "maxNumOrders")]
    pub max_num_orders: NativeNumber,
    #[serde(rename = "limitPriceCap")]
    pub limit_price_cap: String,
    #[serde(rename = "limitPriceFloor")]
    pub limit_price_floor: String,
    #[serde(rename = "riskFactorConfig")]
    pub risk_factor_config: Vec<RiskFactorConfigModel>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradingFeeModel {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "makerFeeRate")]
    pub maker_fee_rate: String,
    #[serde(rename = "takerFeeRate")]
    pub taker_fee_rate: String,
    #[serde(rename = "builderFeeRate")]
    pub builder_fee_rate: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferParams {
    #[serde(rename = "body")]
    pub body: TransferRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferPatchParams {
    #[serde(rename = "body")]
    pub body: TransferRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    #[serde(rename = "fromAccount")]
    pub from_account: serde_json::Number,
    #[serde(rename = "toAccount")]
    pub to_account: serde_json::Number,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "transferredAsset")]
    pub transferred_asset: String,
    #[serde(rename = "settlement", default, skip_serializing_if = "Option::is_none")]
    pub settlement: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for TransferRequest {
    fn default() -> Self {
        Self {
            from_account: 0.into(),
            to_account: 0.into(),
            amount: Default::default(),
            transferred_asset: Default::default(),
            settlement: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferResponseModel {
    #[serde(rename = "validSignature")]
    pub valid_signature: bool,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    #[serde(rename = "hashCalculated", default, skip_serializing_if = "Option::is_none")]
    pub hash_calculated: Option<serde_json::Value>,
    #[serde(rename = "starkExRepresentation", default, skip_serializing_if = "Option::is_none")]
    pub stark_ex_representation: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Twap {
    #[serde(rename = "numParts", default, skip_serializing_if = "Option::is_none")]
    pub num_parts: Option<serde_json::Number>,
    #[serde(rename = "interval", default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateAccountInfoParams {
    #[serde(rename = "body")]
    pub body: AccountUpdateRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateFundingRateConfigParams {
    #[serde(rename = "body")]
    pub body: FundingRateConfigRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateLeverageParams {
    #[serde(rename = "body")]
    pub body: AccountLeverage,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateLeverageResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<EmptyModel>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateReferralLinkCodeRequest {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateReferralLinkParams {
    #[serde(rename = "body")]
    pub body: UpdateReferralLinkCodeRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UseReferralCodeParams {
    #[serde(rename = "body")]
    pub body: UseReferralCodeRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UseReferralCodeRequest {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ValidateWalletParams {
    #[serde(rename = "body")]
    pub body: WalletValidationRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WalletValidationRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WithdrawFromVaultParams {
    #[serde(rename = "body")]
    pub body: WithdrawRequestModel,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WithdrawFromVaultResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WithdrawParams {
    #[serde(rename = "body")]
    pub body: WithdrawalRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WithdrawRequestModel {
    #[serde(rename = "fromAccountId")]
    pub from_account_id: NativeNumber,
    #[serde(rename = "toAccountId")]
    pub to_account_id: NativeNumber,
    #[serde(rename = "collateral")]
    pub collateral: String,
    #[serde(rename = "shares")]
    pub shares: String,
    #[serde(rename = "settlement")]
    pub settlement: LimitOrderSettlementModel,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WithdrawResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<NativeNumber>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WithdrawalRequest {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "destination")]
    pub destination: String,
    #[serde(rename = "nonce", default = "zero_number")]
    pub nonce: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for WithdrawalRequest {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            asset: Default::default(),
            destination: Default::default(),
            nonce: 0.into(),
            extra: Default::default(),
        }
    }
}

pub struct TypedExtendedClient(ExtendedClient);
impl ExtendedClient {
    pub fn typed(self) -> TypedExtendedClient { TypedExtendedClient(self) }
}
impl TypedExtendedClient {
    pub fn asset_operations(&self, request: &AssetOperationsParams) -> Result<AssetOperationsResult> {
        serde_json::from_value(self.0.asset_operations(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn cancel_order_by_external_id(&self, request: &CancelOrderByExternalIdParams) -> Result<CancelOrderByExternalIdResult> {
        serde_json::from_value(self.0.cancel_order_by_external_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn claim_testing_funds(&self, request: &ClaimTestingFundsParams) -> Result<ClaimTestingFundsResult> {
        serde_json::from_value(self.0.claim_testing_funds(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn commit_bridge_quote(&self, request: &CommitBridgeQuoteParams) -> Result<CommitBridgeQuoteResult> {
        serde_json::from_value(self.0.commit_bridge_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn create_api_key(&self, request: &CreateApiKeyParams) -> Result<CreateApiKeyResult> {
        serde_json::from_value(self.0.create_api_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn create_deposit(&self, request: &CreateDepositParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.create_deposit(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn create_referral_link(&self, request: &CreateReferralLinkParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.create_referral_link(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn create_scaled_order(&self, request: &CreateScaledOrderParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.create_scaled_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn delete_api_key(&self, request: &DeleteApiKeyParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.delete_api_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn delete_user_order_order_id(&self, request: &DeleteUserOrderOrderIdParams) -> Result<DeleteUserOrderOrderIdResult> {
        serde_json::from_value(self.0.delete_user_order_order_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn deposit_to_vault(&self, request: &DepositToVaultParams) -> Result<DepositToVaultResult> {
        serde_json::from_value(self.0.deposit_to_vault(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn export_csv_zip(&self, request: &ExportCsvZipParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.export_csv_zip(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_account(&self, request: &GetAccountParams) -> Result<GetAccountResult> {
        serde_json::from_value(self.0.get_account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_account_equity_history(&self, request: &GetAccountEquityHistoryParams) -> Result<GetAccountEquityHistoryResult> {
        serde_json::from_value(self.0.get_account_equity_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_account_funding_chart(&self, request: &GetAccountFundingChartParams) -> Result<GetAccountFundingChartResult> {
        serde_json::from_value(self.0.get_account_funding_chart(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_account_funding_history(&self, request: &GetAccountFundingHistoryParams) -> Result<GetAccountFundingHistoryResult> {
        serde_json::from_value(self.0.get_account_funding_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_account_funding_stats(&self, request: &GetAccountFundingStatsParams) -> Result<GetAccountFundingStatsResult> {
        serde_json::from_value(self.0.get_account_funding_stats(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_account_health(&self, request: &GetAccountHealthParams) -> Result<GetAccountHealthResult> {
        serde_json::from_value(self.0.get_account_health(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_account_max_drawdown_history(&self, request: &GetAccountMaxDrawdownHistoryParams) -> Result<GetAccountMaxDrawdownHistoryResult> {
        serde_json::from_value(self.0.get_account_max_drawdown_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_account_performance(&self, request: &GetAccountPerformanceParams) -> Result<GetAccountPerformanceResult> {
        serde_json::from_value(self.0.get_account_performance(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_account_pn_l_history(&self, request: &GetAccountPnLHistoryParams) -> Result<GetAccountPnLHistoryResult> {
        serde_json::from_value(self.0.get_account_pn_l_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_account_pn_l_percentage_history(&self, request: &GetAccountPnLPercentageHistoryParams) -> Result<GetAccountPnLPercentageHistoryResult> {
        serde_json::from_value(self.0.get_account_pn_l_percentage_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_account_portfolio_summary(&self, request: &GetAccountPortfolioSummaryParams) -> Result<GetAccountPortfolioSummaryResult> {
        serde_json::from_value(self.0.get_account_portfolio_summary(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_account_vault_equity_history(&self, request: &GetAccountVaultEquityHistoryParams) -> Result<GetAccountVaultEquityHistoryResult> {
        serde_json::from_value(self.0.get_account_vault_equity_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_accounts(&self, request: &GetAccountsParams) -> Result<GetAccountsResult> {
        serde_json::from_value(self.0.get_accounts(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_affiliate(&self, request: &GetAffiliateParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_affiliate(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_affiliate_rewards(&self, request: &GetAffiliateRewardsParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_affiliate_rewards(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_api_keys(&self, request: &GetApiKeysParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_api_keys(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_asset_operations(&self, request: &GetAssetOperationsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_asset_operations(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_asset_price(&self, request: &GetAssetPriceParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_asset_price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_assets(&self, request: &GetAssetsParams) -> Result<GetAssetsResult> {
        serde_json::from_value(self.0.get_assets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_balance(&self, request: &GetBalanceParams) -> Result<GetBalanceResult> {
        serde_json::from_value(self.0.get_balance(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_bridge_config(&self, request: &GetBridgeConfigParams) -> Result<GetBridgeConfigResult> {
        serde_json::from_value(self.0.get_bridge_config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_bridge_quote(&self, request: &GetBridgeQuoteParams) -> Result<GetBridgeQuoteResult> {
        serde_json::from_value(self.0.get_bridge_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_builder_dashboard(&self, request: &GetBuilderDashboardParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_builder_dashboard(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_builder_trades(&self, request: &GetBuilderTradesParams) -> Result<GetBuilderTradesResult> {
        serde_json::from_value(self.0.get_builder_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_candles_history(&self, request: &GetCandlesHistoryParams) -> Result<GetCandlesHistoryResult> {
        serde_json::from_value(self.0.get_candles_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_claim(&self, request: &GetClaimParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_claim(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_claims(&self, request: &GetClaimsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_claims(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_client(&self, request: &GetClientParams) -> Result<GetClientResult> {
        serde_json::from_value(self.0.get_client(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_cumulative_account_pn_l_history(&self, request: &GetCumulativeAccountPnLHistoryParams) -> Result<GetCumulativeAccountPnLHistoryResult> {
        serde_json::from_value(self.0.get_cumulative_account_pn_l_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_cumulative_account_pn_l_percentage_history(&self, request: &GetCumulativeAccountPnLPercentageHistoryParams) -> Result<GetCumulativeAccountPnLPercentageHistoryResult> {
        serde_json::from_value(self.0.get_cumulative_account_pn_l_percentage_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_earned_rewards(&self, request: &GetEarnedRewardsParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_earned_rewards(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_exchange_status(&self, request: &GetExchangeStatusParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_exchange_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_fees(&self, request: &GetFeesParams) -> Result<GetFeesResult> {
        serde_json::from_value(self.0.get_fees(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_funding_rates_history(&self, request: &GetFundingRatesHistoryParams) -> Result<GetFundingRatesHistoryResult> {
        serde_json::from_value(self.0.get_funding_rates_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_index_price_candles(&self, request: &GetIndexPriceCandlesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_index_price_candles(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_info_assets_asset_name_price(&self, request: &GetInfoAssetsAssetNamePriceParams) -> Result<GetInfoAssetsAssetNamePriceResult> {
        serde_json::from_value(self.0.get_info_assets_asset_name_price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_info_candles_market_candle_type(&self, request: &GetInfoCandlesMarketCandleTypeParams) -> Result<GetInfoCandlesMarketCandleTypeResult> {
        serde_json::from_value(self.0.get_info_candles_market_candle_type(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_interest_daily_metrics(&self, request: &GetInterestDailyMetricsParams) -> Result<GetInterestDailyMetricsResult> {
        serde_json::from_value(self.0.get_interest_daily_metrics(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_interest_key_metrics(&self, request: &GetInterestKeyMetricsParams) -> Result<GetInterestKeyMetricsResult> {
        serde_json::from_value(self.0.get_interest_key_metrics(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_interest_payment_chart(&self, request: &GetInterestPaymentChartParams) -> Result<GetInterestPaymentChartResult> {
        serde_json::from_value(self.0.get_interest_payment_chart(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_interest_payments_history(&self, request: &GetInterestPaymentsHistoryParams) -> Result<GetInterestPaymentsHistoryResult> {
        serde_json::from_value(self.0.get_interest_payments_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_interest_rate_curves_history(&self, request: &GetInterestRateCurvesHistoryParams) -> Result<GetInterestRateCurvesHistoryResult> {
        serde_json::from_value(self.0.get_interest_rate_curves_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_latest_interest_rate_curve(&self, request: &GetLatestInterestRateCurveParams) -> Result<GetLatestInterestRateCurveResult> {
        serde_json::from_value(self.0.get_latest_interest_rate_curve(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_leverage(&self, request: &GetLeverageParams) -> Result<GetLeverageResult> {
        serde_json::from_value(self.0.get_leverage(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_mark_price_candles(&self, request: &GetMarkPriceCandlesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_mark_price_candles(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_market_statistics(&self, request: &GetMarketStatisticsParams) -> Result<GetMarketStatisticsResult> {
        serde_json::from_value(self.0.get_market_statistics(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_market_trades(&self, request: &GetMarketTradesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_market_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_markets(&self, request: &GetMarketsParams) -> Result<GetMarketsResult> {
        serde_json::from_value(self.0.get_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_markets_stats(&self, request: &GetMarketsStatsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_markets_stats(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_next_claim_time(&self, request: &GetNextClaimTimeParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_next_claim_time(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_open_interests(&self, request: &GetOpenInterestsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_open_interests(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_open_orders(&self, request: &GetOpenOrdersParams) -> Result<GetOpenOrdersResult> {
        serde_json::from_value(self.0.get_open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_order(&self, request: &GetOrderParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_order_by_external_id(&self, request: &GetOrderByExternalIdParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_order_by_external_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_order_by_id(&self, request: &GetOrderByIdParams) -> Result<GetOrderByIdResult> {
        serde_json::from_value(self.0.get_order_by_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_orderbook_snapshot(&self, request: &GetOrderbookSnapshotParams) -> Result<GetOrderbookSnapshotResult> {
        serde_json::from_value(self.0.get_orderbook_snapshot(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_orders_history(&self, request: &GetOrdersHistoryParams) -> Result<GetOrdersHistoryResult> {
        serde_json::from_value(self.0.get_orders_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_partner_rewards(&self, request: &GetPartnerRewardsParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_partner_rewards(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_positions(&self, request: &GetPositionsParams) -> Result<GetPositionsResult> {
        serde_json::from_value(self.0.get_positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_positions_history(&self, request: &GetPositionsHistoryParams) -> Result<GetPositionsHistoryResult> {
        serde_json::from_value(self.0.get_positions_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_public_leaderboard(&self, request: &GetPublicLeaderboardParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_public_leaderboard(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_public_referral_link(&self, request: &GetPublicReferralLinkParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_public_referral_link(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_rebate_stats(&self, request: &GetRebateStatsParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_rebate_stats(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_rebates_csv(&self, request: &GetRebatesCsvParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_rebates_csv(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_referral_dashboard(&self, request: &GetReferralDashboardParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_referral_dashboard(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_referral_links(&self, request: &GetReferralLinksParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_referral_links(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_referral_rebates_by_link_csv(&self, request: &GetReferralRebatesByLinkCsvParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_referral_rebates_by_link_csv(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_referral_rebates_by_period_csv(&self, request: &GetReferralRebatesByPeriodCsvParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_referral_rebates_by_period_csv(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_referral_rebates_by_subaffiliate_csv(&self, request: &GetReferralRebatesBySubaffiliateCsvParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_referral_rebates_by_subaffiliate_csv(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_referral_rebates_by_user_csv(&self, request: &GetReferralRebatesByUserCsvParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_referral_rebates_by_user_csv(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_referral_status(&self, request: &GetReferralStatusParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_referral_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_reward_leaderboard_stats(&self, request: &GetRewardLeaderboardStatsParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_reward_leaderboard_stats(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_settings(&self, request: &GetSettingsParams) -> Result<GetSettingsResult> {
        serde_json::from_value(self.0.get_settings(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_spot_balances(&self, request: &GetSpotBalancesParams) -> Result<GetSpotBalancesResult> {
        serde_json::from_value(self.0.get_spot_balances(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_starknet_domain(&self, request: &GetStarknetDomainParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_starknet_domain(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_trades(&self, request: &GetTradesParams) -> Result<GetTradesResult> {
        serde_json::from_value(self.0.get_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_trading_candles(&self, request: &GetTradingCandlesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_trading_candles(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_trading_fee_stats(&self, request: &GetTradingFeeStatsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_trading_fee_stats(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_user_funding_history(&self, request: &GetUserFundingHistoryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_user_funding_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_user_leaderboard(&self, request: &GetUserLeaderboardParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_user_leaderboard(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_user_orders_external_external_id(&self, request: &GetUserOrdersExternalExternalIdParams) -> Result<GetUserOrdersExternalExternalIdResult> {
        serde_json::from_value(self.0.get_user_orders_external_external_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_vault_funding_history(&self, request: &GetVaultFundingHistoryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_vault_funding_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_vault_orders(&self, request: &GetVaultOrdersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_vault_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_vault_performance(&self, request: &GetVaultPerformanceParams) -> Result<GetVaultPerformanceResult> {
        serde_json::from_value(self.0.get_vault_performance(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_vault_position_history(&self, request: &GetVaultPositionHistoryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_vault_position_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_vault_positions(&self, request: &GetVaultPositionsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_vault_positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_vault_summary(&self, request: &GetVaultSummaryParams) -> Result<GetVaultSummaryResult> {
        serde_json::from_value(self.0.get_vault_summary(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_vault_trades(&self, request: &GetVaultTradesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_vault_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_wallet_validation(&self, request: &GetWalletValidationParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_wallet_validation(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_withdrawals(&self, request: &GetWithdrawalsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_withdrawals(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn get_wrapper(&self, request: &GetWrapperParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_wrapper(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn mark_wallet_checked(&self, request: &MarkWalletCheckedParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.mark_wallet_checked(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn mass_cancel(&self, request: &MassCancelParams) -> Result<MassCancelResult> {
        serde_json::from_value(self.0.mass_cancel(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn onboard_client(&self, request: &OnboardClientParams) -> Result<OnboardClientResult> {
        serde_json::from_value(self.0.onboard_client(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn onboard_subaccount(&self, request: &OnboardSubaccountParams) -> Result<OnboardSubaccountResult> {
        serde_json::from_value(self.0.onboard_subaccount(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn onchain_withdrawal(&self, request: &OnchainWithdrawalParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.onchain_withdrawal(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderParams) -> Result<PlaceOrderResult> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn place_rfq_order(&self, request: &PlaceRfqOrderParams) -> Result<PlaceRfqOrderResult> {
        serde_json::from_value(self.0.place_rfq_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn post_user_transfer_onchain(&self, request: &PostUserTransferOnchainParams) -> Result<PostUserTransferOnchainResult> {
        serde_json::from_value(self.0.post_user_transfer_onchain(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn set_deadman_switch(&self, request: &SetDeadmanSwitchParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.set_deadman_switch(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn transfer(&self, request: &TransferParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.transfer(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn transfer_patch(&self, request: &TransferPatchParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.transfer_patch(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn update_account_info(&self, request: &UpdateAccountInfoParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.update_account_info(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn update_funding_rate_config(&self, request: &UpdateFundingRateConfigParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.update_funding_rate_config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn update_leverage(&self, request: &UpdateLeverageParams) -> Result<UpdateLeverageResult> {
        serde_json::from_value(self.0.update_leverage(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn update_referral_link(&self, request: &UpdateReferralLinkParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.update_referral_link(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn use_referral_code(&self, request: &UseReferralCodeParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.use_referral_code(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn validate_wallet(&self, request: &ValidateWalletParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.validate_wallet(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn withdraw(&self, request: &WithdrawParams) -> Result<WithdrawResult> {
        serde_json::from_value(self.0.withdraw(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub fn withdraw_from_vault(&self, request: &WithdrawFromVaultParams) -> Result<WithdrawFromVaultResult> {
        serde_json::from_value(self.0.withdraw_from_vault(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedExtendedClient(crate::AsyncClient<ExtendedClient>);
#[cfg(feature = "async")]
impl TypedExtendedClient {
    pub fn into_async(self) -> AsyncTypedExtendedClient { AsyncTypedExtendedClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedExtendedClient {
    pub async fn asset_operations(&self, request: &AssetOperationsParams) -> Result<AssetOperationsResult> {
        serde_json::from_value(self.0.asset_operations(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn cancel_order_by_external_id(&self, request: &CancelOrderByExternalIdParams) -> Result<CancelOrderByExternalIdResult> {
        serde_json::from_value(self.0.cancel_order_by_external_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn claim_testing_funds(&self, request: &ClaimTestingFundsParams) -> Result<ClaimTestingFundsResult> {
        serde_json::from_value(self.0.claim_testing_funds(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn commit_bridge_quote(&self, request: &CommitBridgeQuoteParams) -> Result<CommitBridgeQuoteResult> {
        serde_json::from_value(self.0.commit_bridge_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn create_api_key(&self, request: &CreateApiKeyParams) -> Result<CreateApiKeyResult> {
        serde_json::from_value(self.0.create_api_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn create_deposit(&self, request: &CreateDepositParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.create_deposit(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn create_referral_link(&self, request: &CreateReferralLinkParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.create_referral_link(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn create_scaled_order(&self, request: &CreateScaledOrderParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.create_scaled_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn delete_api_key(&self, request: &DeleteApiKeyParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.delete_api_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn delete_user_order_order_id(&self, request: &DeleteUserOrderOrderIdParams) -> Result<DeleteUserOrderOrderIdResult> {
        serde_json::from_value(self.0.delete_user_order_order_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn deposit_to_vault(&self, request: &DepositToVaultParams) -> Result<DepositToVaultResult> {
        serde_json::from_value(self.0.deposit_to_vault(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn export_csv_zip(&self, request: &ExportCsvZipParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.export_csv_zip(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_account(&self, request: &GetAccountParams) -> Result<GetAccountResult> {
        serde_json::from_value(self.0.get_account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_account_equity_history(&self, request: &GetAccountEquityHistoryParams) -> Result<GetAccountEquityHistoryResult> {
        serde_json::from_value(self.0.get_account_equity_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_account_funding_chart(&self, request: &GetAccountFundingChartParams) -> Result<GetAccountFundingChartResult> {
        serde_json::from_value(self.0.get_account_funding_chart(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_account_funding_history(&self, request: &GetAccountFundingHistoryParams) -> Result<GetAccountFundingHistoryResult> {
        serde_json::from_value(self.0.get_account_funding_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_account_funding_stats(&self, request: &GetAccountFundingStatsParams) -> Result<GetAccountFundingStatsResult> {
        serde_json::from_value(self.0.get_account_funding_stats(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_account_health(&self, request: &GetAccountHealthParams) -> Result<GetAccountHealthResult> {
        serde_json::from_value(self.0.get_account_health(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_account_max_drawdown_history(&self, request: &GetAccountMaxDrawdownHistoryParams) -> Result<GetAccountMaxDrawdownHistoryResult> {
        serde_json::from_value(self.0.get_account_max_drawdown_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_account_performance(&self, request: &GetAccountPerformanceParams) -> Result<GetAccountPerformanceResult> {
        serde_json::from_value(self.0.get_account_performance(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_account_pn_l_history(&self, request: &GetAccountPnLHistoryParams) -> Result<GetAccountPnLHistoryResult> {
        serde_json::from_value(self.0.get_account_pn_l_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_account_pn_l_percentage_history(&self, request: &GetAccountPnLPercentageHistoryParams) -> Result<GetAccountPnLPercentageHistoryResult> {
        serde_json::from_value(self.0.get_account_pn_l_percentage_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_account_portfolio_summary(&self, request: &GetAccountPortfolioSummaryParams) -> Result<GetAccountPortfolioSummaryResult> {
        serde_json::from_value(self.0.get_account_portfolio_summary(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_account_vault_equity_history(&self, request: &GetAccountVaultEquityHistoryParams) -> Result<GetAccountVaultEquityHistoryResult> {
        serde_json::from_value(self.0.get_account_vault_equity_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_accounts(&self, request: &GetAccountsParams) -> Result<GetAccountsResult> {
        serde_json::from_value(self.0.get_accounts(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_affiliate(&self, request: &GetAffiliateParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_affiliate(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_affiliate_rewards(&self, request: &GetAffiliateRewardsParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_affiliate_rewards(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_api_keys(&self, request: &GetApiKeysParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_api_keys(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_asset_operations(&self, request: &GetAssetOperationsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_asset_operations(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_asset_price(&self, request: &GetAssetPriceParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_asset_price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_assets(&self, request: &GetAssetsParams) -> Result<GetAssetsResult> {
        serde_json::from_value(self.0.get_assets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_balance(&self, request: &GetBalanceParams) -> Result<GetBalanceResult> {
        serde_json::from_value(self.0.get_balance(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_bridge_config(&self, request: &GetBridgeConfigParams) -> Result<GetBridgeConfigResult> {
        serde_json::from_value(self.0.get_bridge_config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_bridge_quote(&self, request: &GetBridgeQuoteParams) -> Result<GetBridgeQuoteResult> {
        serde_json::from_value(self.0.get_bridge_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_builder_dashboard(&self, request: &GetBuilderDashboardParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_builder_dashboard(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_builder_trades(&self, request: &GetBuilderTradesParams) -> Result<GetBuilderTradesResult> {
        serde_json::from_value(self.0.get_builder_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_candles_history(&self, request: &GetCandlesHistoryParams) -> Result<GetCandlesHistoryResult> {
        serde_json::from_value(self.0.get_candles_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_claim(&self, request: &GetClaimParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_claim(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_claims(&self, request: &GetClaimsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_claims(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_client(&self, request: &GetClientParams) -> Result<GetClientResult> {
        serde_json::from_value(self.0.get_client(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_cumulative_account_pn_l_history(&self, request: &GetCumulativeAccountPnLHistoryParams) -> Result<GetCumulativeAccountPnLHistoryResult> {
        serde_json::from_value(self.0.get_cumulative_account_pn_l_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_cumulative_account_pn_l_percentage_history(&self, request: &GetCumulativeAccountPnLPercentageHistoryParams) -> Result<GetCumulativeAccountPnLPercentageHistoryResult> {
        serde_json::from_value(self.0.get_cumulative_account_pn_l_percentage_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_earned_rewards(&self, request: &GetEarnedRewardsParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_earned_rewards(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_exchange_status(&self, request: &GetExchangeStatusParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_exchange_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_fees(&self, request: &GetFeesParams) -> Result<GetFeesResult> {
        serde_json::from_value(self.0.get_fees(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_funding_rates_history(&self, request: &GetFundingRatesHistoryParams) -> Result<GetFundingRatesHistoryResult> {
        serde_json::from_value(self.0.get_funding_rates_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_index_price_candles(&self, request: &GetIndexPriceCandlesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_index_price_candles(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_info_assets_asset_name_price(&self, request: &GetInfoAssetsAssetNamePriceParams) -> Result<GetInfoAssetsAssetNamePriceResult> {
        serde_json::from_value(self.0.get_info_assets_asset_name_price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_info_candles_market_candle_type(&self, request: &GetInfoCandlesMarketCandleTypeParams) -> Result<GetInfoCandlesMarketCandleTypeResult> {
        serde_json::from_value(self.0.get_info_candles_market_candle_type(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_interest_daily_metrics(&self, request: &GetInterestDailyMetricsParams) -> Result<GetInterestDailyMetricsResult> {
        serde_json::from_value(self.0.get_interest_daily_metrics(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_interest_key_metrics(&self, request: &GetInterestKeyMetricsParams) -> Result<GetInterestKeyMetricsResult> {
        serde_json::from_value(self.0.get_interest_key_metrics(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_interest_payment_chart(&self, request: &GetInterestPaymentChartParams) -> Result<GetInterestPaymentChartResult> {
        serde_json::from_value(self.0.get_interest_payment_chart(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_interest_payments_history(&self, request: &GetInterestPaymentsHistoryParams) -> Result<GetInterestPaymentsHistoryResult> {
        serde_json::from_value(self.0.get_interest_payments_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_interest_rate_curves_history(&self, request: &GetInterestRateCurvesHistoryParams) -> Result<GetInterestRateCurvesHistoryResult> {
        serde_json::from_value(self.0.get_interest_rate_curves_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_latest_interest_rate_curve(&self, request: &GetLatestInterestRateCurveParams) -> Result<GetLatestInterestRateCurveResult> {
        serde_json::from_value(self.0.get_latest_interest_rate_curve(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_leverage(&self, request: &GetLeverageParams) -> Result<GetLeverageResult> {
        serde_json::from_value(self.0.get_leverage(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_mark_price_candles(&self, request: &GetMarkPriceCandlesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_mark_price_candles(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_market_statistics(&self, request: &GetMarketStatisticsParams) -> Result<GetMarketStatisticsResult> {
        serde_json::from_value(self.0.get_market_statistics(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_market_trades(&self, request: &GetMarketTradesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_market_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_markets(&self, request: &GetMarketsParams) -> Result<GetMarketsResult> {
        serde_json::from_value(self.0.get_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_markets_stats(&self, request: &GetMarketsStatsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_markets_stats(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_next_claim_time(&self, request: &GetNextClaimTimeParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_next_claim_time(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_open_interests(&self, request: &GetOpenInterestsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_open_interests(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_open_orders(&self, request: &GetOpenOrdersParams) -> Result<GetOpenOrdersResult> {
        serde_json::from_value(self.0.get_open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_order(&self, request: &GetOrderParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_order_by_external_id(&self, request: &GetOrderByExternalIdParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_order_by_external_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_order_by_id(&self, request: &GetOrderByIdParams) -> Result<GetOrderByIdResult> {
        serde_json::from_value(self.0.get_order_by_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_orderbook_snapshot(&self, request: &GetOrderbookSnapshotParams) -> Result<GetOrderbookSnapshotResult> {
        serde_json::from_value(self.0.get_orderbook_snapshot(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_orders_history(&self, request: &GetOrdersHistoryParams) -> Result<GetOrdersHistoryResult> {
        serde_json::from_value(self.0.get_orders_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_partner_rewards(&self, request: &GetPartnerRewardsParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_partner_rewards(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_positions(&self, request: &GetPositionsParams) -> Result<GetPositionsResult> {
        serde_json::from_value(self.0.get_positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_positions_history(&self, request: &GetPositionsHistoryParams) -> Result<GetPositionsHistoryResult> {
        serde_json::from_value(self.0.get_positions_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_public_leaderboard(&self, request: &GetPublicLeaderboardParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_public_leaderboard(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_public_referral_link(&self, request: &GetPublicReferralLinkParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_public_referral_link(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_rebate_stats(&self, request: &GetRebateStatsParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_rebate_stats(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_rebates_csv(&self, request: &GetRebatesCsvParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_rebates_csv(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_referral_dashboard(&self, request: &GetReferralDashboardParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_referral_dashboard(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_referral_links(&self, request: &GetReferralLinksParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_referral_links(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_referral_rebates_by_link_csv(&self, request: &GetReferralRebatesByLinkCsvParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_referral_rebates_by_link_csv(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_referral_rebates_by_period_csv(&self, request: &GetReferralRebatesByPeriodCsvParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_referral_rebates_by_period_csv(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_referral_rebates_by_subaffiliate_csv(&self, request: &GetReferralRebatesBySubaffiliateCsvParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_referral_rebates_by_subaffiliate_csv(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_referral_rebates_by_user_csv(&self, request: &GetReferralRebatesByUserCsvParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_referral_rebates_by_user_csv(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_referral_status(&self, request: &GetReferralStatusParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_referral_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_reward_leaderboard_stats(&self, request: &GetRewardLeaderboardStatsParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_reward_leaderboard_stats(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_settings(&self, request: &GetSettingsParams) -> Result<GetSettingsResult> {
        serde_json::from_value(self.0.get_settings(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_spot_balances(&self, request: &GetSpotBalancesParams) -> Result<GetSpotBalancesResult> {
        serde_json::from_value(self.0.get_spot_balances(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_starknet_domain(&self, request: &GetStarknetDomainParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_starknet_domain(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_trades(&self, request: &GetTradesParams) -> Result<GetTradesResult> {
        serde_json::from_value(self.0.get_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_trading_candles(&self, request: &GetTradingCandlesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_trading_candles(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_trading_fee_stats(&self, request: &GetTradingFeeStatsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_trading_fee_stats(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_user_funding_history(&self, request: &GetUserFundingHistoryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_user_funding_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_user_leaderboard(&self, request: &GetUserLeaderboardParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_user_leaderboard(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_user_orders_external_external_id(&self, request: &GetUserOrdersExternalExternalIdParams) -> Result<GetUserOrdersExternalExternalIdResult> {
        serde_json::from_value(self.0.get_user_orders_external_external_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_vault_funding_history(&self, request: &GetVaultFundingHistoryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_vault_funding_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_vault_orders(&self, request: &GetVaultOrdersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_vault_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_vault_performance(&self, request: &GetVaultPerformanceParams) -> Result<GetVaultPerformanceResult> {
        serde_json::from_value(self.0.get_vault_performance(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_vault_position_history(&self, request: &GetVaultPositionHistoryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_vault_position_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_vault_positions(&self, request: &GetVaultPositionsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_vault_positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_vault_summary(&self, request: &GetVaultSummaryParams) -> Result<GetVaultSummaryResult> {
        serde_json::from_value(self.0.get_vault_summary(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_vault_trades(&self, request: &GetVaultTradesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_vault_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_wallet_validation(&self, request: &GetWalletValidationParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_wallet_validation(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_withdrawals(&self, request: &GetWithdrawalsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_withdrawals(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn get_wrapper(&self, request: &GetWrapperParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.get_wrapper(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn mark_wallet_checked(&self, request: &MarkWalletCheckedParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.mark_wallet_checked(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn mass_cancel(&self, request: &MassCancelParams) -> Result<MassCancelResult> {
        serde_json::from_value(self.0.mass_cancel(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn onboard_client(&self, request: &OnboardClientParams) -> Result<OnboardClientResult> {
        serde_json::from_value(self.0.onboard_client(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn onboard_subaccount(&self, request: &OnboardSubaccountParams) -> Result<OnboardSubaccountResult> {
        serde_json::from_value(self.0.onboard_subaccount(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn onchain_withdrawal(&self, request: &OnchainWithdrawalParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.onchain_withdrawal(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderParams) -> Result<PlaceOrderResult> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn place_rfq_order(&self, request: &PlaceRfqOrderParams) -> Result<PlaceRfqOrderResult> {
        serde_json::from_value(self.0.place_rfq_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn post_user_transfer_onchain(&self, request: &PostUserTransferOnchainParams) -> Result<PostUserTransferOnchainResult> {
        serde_json::from_value(self.0.post_user_transfer_onchain(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn set_deadman_switch(&self, request: &SetDeadmanSwitchParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.set_deadman_switch(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn transfer(&self, request: &TransferParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.transfer(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn transfer_patch(&self, request: &TransferPatchParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.transfer_patch(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn update_account_info(&self, request: &UpdateAccountInfoParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.update_account_info(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn update_funding_rate_config(&self, request: &UpdateFundingRateConfigParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.update_funding_rate_config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn update_leverage(&self, request: &UpdateLeverageParams) -> Result<UpdateLeverageResult> {
        serde_json::from_value(self.0.update_leverage(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn update_referral_link(&self, request: &UpdateReferralLinkParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.update_referral_link(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn use_referral_code(&self, request: &UseReferralCodeParams) -> Result<ApiResponse> {
        serde_json::from_value(self.0.use_referral_code(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn validate_wallet(&self, request: &ValidateWalletParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.validate_wallet(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn withdraw(&self, request: &WithdrawParams) -> Result<WithdrawResult> {
        serde_json::from_value(self.0.withdraw(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
    pub async fn withdraw_from_vault(&self, request: &WithdrawFromVaultParams) -> Result<WithdrawFromVaultResult> {
        serde_json::from_value(self.0.withdraw_from_vault(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Extended response model"))
    }
}
