// Generated from schema/derive.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{DeriveClient, Error, Result};

fn zero_number() -> serde_json::Number { 0.into() }
pub type Address = String;
pub type AddressKind = String;
pub type AlgoType = String;
pub type AssetType = String;
pub type AuctionType = String;
pub type BatchStatus = String;
pub type CancelAllAlgoOrdersResponse = String;
pub type CancelAllResponse = String;
pub type CancelAllTriggerOrdersResponse = String;
pub type CancelReason = String;
pub type CancelRfqResponse = String;
pub type DecodableActionType = String;
pub type Direction = String;
pub type LiquidityRole = String;
pub type MarginType = String;
pub type MarketType = String;
pub type MonitorType = String;
pub type OffchainKeyScope = String;
pub type OrderStatus = String;
pub type OrderType = String;
pub type PerformanceResolution = String;
pub type RFQCancelReason = String;
pub type RFQStatus = String;
pub type ResetMmpResponse = String;
pub type Severity = String;
pub type TimeInForce = String;
pub type TriggerPriceType = String;
pub type TriggerType = String;
pub type WithdrawalForwardStatus = String;
pub type JsonRpcId = serde_json::Value;
pub type SetCancelOnDisconnectResponse = String;
pub type AuctionStateType = String;
pub type BalanceUpdateType = String;
pub type LiquidityRole2 = String;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountFeeInfo {
    #[serde(rename = "base_fee_discount")]
    pub base_fee_discount: String,
    #[serde(rename = "option_maker_fee", default, skip_serializing_if = "Option::is_none")]
    pub option_maker_fee: Option<String>,
    #[serde(rename = "option_taker_fee", default, skip_serializing_if = "Option::is_none")]
    pub option_taker_fee: Option<String>,
    #[serde(rename = "perp_maker_fee", default, skip_serializing_if = "Option::is_none")]
    pub perp_maker_fee: Option<String>,
    #[serde(rename = "perp_taker_fee", default, skip_serializing_if = "Option::is_none")]
    pub perp_taker_fee: Option<String>,
    #[serde(rename = "rfq_maker_discount")]
    pub rfq_maker_discount: String,
    #[serde(rename = "rfq_taker_discount")]
    pub rfq_taker_discount: String,
    #[serde(rename = "spot_maker_fee", default, skip_serializing_if = "Option::is_none")]
    pub spot_maker_fee: Option<String>,
    #[serde(rename = "spot_taker_fee", default, skip_serializing_if = "Option::is_none")]
    pub spot_taker_fee: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "data")]
    pub data: Vec<serde_json::Number>,
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(rename = "module")]
    pub module: Address,
    #[serde(rename = "nonce")]
    pub nonce: serde_json::Number,
    #[serde(rename = "owner")]
    pub owner: Address,
    #[serde(rename = "signer")]
    pub signer: Address,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for Action {
    fn default() -> Self {
        Self {
            data: Default::default(),
            expiry: 0.into(),
            module: Default::default(),
            nonce: 0.into(),
            owner: Default::default(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AggregatedOrdersResult {
    #[serde(rename = "orders")]
    pub orders: Vec<Order>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for AggregatedOrdersResult {
    fn default() -> Self {
        Self {
            orders: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AggregatedTriggerOrdersResult {
    #[serde(rename = "orders")]
    pub orders: Vec<Order>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for AggregatedTriggerOrdersResult {
    fn default() -> Self {
        Self {
            orders: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AssetEntry {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "universes")]
    pub universes: Vec<AssetUniverse>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AssetUniverse {
    #[serde(rename = "oi")]
    pub oi: OpenInterestStats,
    #[serde(rename = "risk_universe_id")]
    pub risk_universe_id: serde_json::Number,
    #[serde(rename = "risk_universe_name", default, skip_serializing_if = "Option::is_none")]
    pub risk_universe_name: Option<String>,
    #[serde(rename = "srm_perp_margin_requirements", default, skip_serializing_if = "Option::is_none")]
    pub srm_perp_margin_requirements: Option<SrmPerpMarginRequirements>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for AssetUniverse {
    fn default() -> Self {
        Self {
            oi: Default::default(),
            risk_universe_id: 0.into(),
            risk_universe_name: Default::default(),
            srm_perp_margin_requirements: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuctionBidEvent {
    #[serde(rename = "amounts_liquidated")]
    pub amounts_liquidated: std::collections::BTreeMap<String,String>,
    #[serde(rename = "bidder_id")]
    pub bidder_id: serde_json::Number,
    #[serde(rename = "cash_received")]
    pub cash_received: String,
    #[serde(rename = "discount_pnl")]
    pub discount_pnl: String,
    #[serde(rename = "operation_id")]
    pub operation_id: String,
    #[serde(rename = "percent_liquidated")]
    pub percent_liquidated: String,
    #[serde(rename = "positions_realized_pnl")]
    pub positions_realized_pnl: std::collections::BTreeMap<String,String>,
    #[serde(rename = "positions_realized_pnl_excl_fees")]
    pub positions_realized_pnl_excl_fees: std::collections::BTreeMap<String,String>,
    #[serde(rename = "realized_pnl")]
    pub realized_pnl: String,
    #[serde(rename = "realized_pnl_excl_fees")]
    pub realized_pnl_excl_fees: String,
    #[serde(rename = "risk_universe_id")]
    pub risk_universe_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    #[serde(rename = "unit_price")]
    pub unit_price: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for AuctionBidEvent {
    fn default() -> Self {
        Self {
            amounts_liquidated: Default::default(),
            bidder_id: 0.into(),
            cash_received: Default::default(),
            discount_pnl: Default::default(),
            operation_id: Default::default(),
            percent_liquidated: Default::default(),
            positions_realized_pnl: Default::default(),
            positions_realized_pnl_excl_fees: Default::default(),
            realized_pnl: Default::default(),
            realized_pnl_excl_fees: Default::default(),
            risk_universe_id: 0.into(),
            timestamp: 0.into(),
            tx_hash: Default::default(),
            unit_price: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuctionHistory {
    #[serde(rename = "auction_id")]
    pub auction_id: String,
    #[serde(rename = "auction_type")]
    pub auction_type: AuctionType,
    #[serde(rename = "bids")]
    pub bids: Vec<AuctionBidEvent>,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "risk_universe_id")]
    pub risk_universe_id: serde_json::Number,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for AuctionHistory {
    fn default() -> Self {
        Self {
            auction_id: Default::default(),
            auction_type: Default::default(),
            bids: Default::default(),
            end_timestamp: Default::default(),
            fee: Default::default(),
            risk_universe_id: 0.into(),
            start_timestamp: 0.into(),
            subaccount_id: 0.into(),
            tx_hash: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BurnSharesRequest {
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "request_id")]
    pub request_id: VaultRequestId,
    #[serde(rename = "share_price")]
    pub share_price: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: Address,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "withdraw_hash")]
    pub withdraw_hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for BurnSharesRequest {
    fn default() -> Self {
        Self {
            nonce: Default::default(),
            request_id: Default::default(),
            share_price: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            withdraw_hash: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelAlgoOrderRequest {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelAlgoOrderRequest {
    fn default() -> Self {
        Self {
            order_id: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelAllAlgoOrdersRequest {
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelAllAlgoOrdersRequest {
    fn default() -> Self {
        Self {
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelAllRequest {
    #[serde(rename = "cancel_algo_orders", default, skip_serializing_if = "Option::is_none")]
    pub cancel_algo_orders: Option<bool>,
    #[serde(rename = "cancel_trigger_orders", default, skip_serializing_if = "Option::is_none")]
    pub cancel_trigger_orders: Option<bool>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelAllRequest {
    fn default() -> Self {
        Self {
            cancel_algo_orders: Default::default(),
            cancel_trigger_orders: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelAllTriggerOrdersRequest {
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelAllTriggerOrdersRequest {
    fn default() -> Self {
        Self {
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelBatchQuotesRequest {
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "nonce", default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(rename = "quote_id", default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(rename = "rfq_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelBatchQuotesRequest {
    fn default() -> Self {
        Self {
            label: Default::default(),
            nonce: Default::default(),
            quote_id: Default::default(),
            rfq_id: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelBatchResult {
    #[serde(rename = "cancelled_ids")]
    pub cancelled_ids: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelBatchRfqsRequest {
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "nonce", default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(rename = "rfq_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelBatchRfqsRequest {
    fn default() -> Self {
        Self {
            label: Default::default(),
            nonce: Default::default(),
            rfq_id: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelBatchRfqsResponse {
    #[serde(rename = "cancelled_ids")]
    pub cancelled_ids: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelByInstrumentRequest {
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelByInstrumentRequest {
    fn default() -> Self {
        Self {
            instrument_name: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelByInstrumentResponse {
    #[serde(rename = "cancelled_orders")]
    pub cancelled_orders: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelByInstrumentResponse {
    fn default() -> Self {
        Self {
            cancelled_orders: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelByLabelRequest {
    #[serde(rename = "instrument_name", default, skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelByLabelRequest {
    fn default() -> Self {
        Self {
            instrument_name: Default::default(),
            label: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelByLabelResponse {
    #[serde(rename = "cancelled_orders")]
    pub cancelled_orders: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelByLabelResponse {
    fn default() -> Self {
        Self {
            cancelled_orders: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelByNonceRequest {
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelByNonceRequest {
    fn default() -> Self {
        Self {
            instrument_name: Default::default(),
            nonce: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelByNonceResponse {
    #[serde(rename = "cancelled_orders")]
    pub cancelled_orders: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelByNonceResponse {
    fn default() -> Self {
        Self {
            cancelled_orders: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelOrderRequest {
    fn default() -> Self {
        Self {
            instrument_name: Default::default(),
            order_id: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelQuoteRequest {
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "nonce", default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "rfq_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelQuoteRequest {
    fn default() -> Self {
        Self {
            label: Default::default(),
            nonce: Default::default(),
            quote_id: Default::default(),
            rfq_id: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelRfqRequest {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelRfqRequest {
    fn default() -> Self {
        Self {
            rfq_id: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelTriggerOrderRequest {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelTriggerOrderRequest {
    fn default() -> Self {
        Self {
            order_id: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelVaultRequestRequest {
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: Address,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "vault_subaccount_id")]
    pub vault_subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelVaultRequestRequest {
    fn default() -> Self {
        Self {
            nonce: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            vault_subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ChangeSubaccountLabelRequest {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ChangeSubaccountLabelRequest {
    fn default() -> Self {
        Self {
            label: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Collateral {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "amount_step")]
    pub amount_step: String,
    #[serde(rename = "asset_name")]
    pub asset_name: String,
    #[serde(rename = "asset_type")]
    pub asset_type: String,
    #[serde(rename = "average_price")]
    pub average_price: String,
    #[serde(rename = "average_price_excl_fees")]
    pub average_price_excl_fees: String,
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: serde_json::Number,
    #[serde(rename = "cumulative_interest")]
    pub cumulative_interest: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "delta")]
    pub delta: String,
    #[serde(rename = "delta_currency")]
    pub delta_currency: String,
    #[serde(rename = "initial_margin")]
    pub initial_margin: String,
    #[serde(rename = "maintenance_margin")]
    pub maintenance_margin: String,
    #[serde(rename = "mark_price")]
    pub mark_price: String,
    #[serde(rename = "mark_value")]
    pub mark_value: String,
    #[serde(rename = "open_orders_margin")]
    pub open_orders_margin: String,
    #[serde(rename = "pending_interest")]
    pub pending_interest: String,
    #[serde(rename = "realized_pnl")]
    pub realized_pnl: String,
    #[serde(rename = "realized_pnl_excl_fees")]
    pub realized_pnl_excl_fees: String,
    #[serde(rename = "total_fees")]
    pub total_fees: String,
    #[serde(rename = "unrealized_pnl")]
    pub unrealized_pnl: String,
    #[serde(rename = "unrealized_pnl_excl_fees")]
    pub unrealized_pnl_excl_fees: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for Collateral {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            amount_step: Default::default(),
            asset_name: Default::default(),
            asset_type: Default::default(),
            average_price: Default::default(),
            average_price_excl_fees: Default::default(),
            creation_timestamp: 0.into(),
            cumulative_interest: Default::default(),
            currency: Default::default(),
            delta: Default::default(),
            delta_currency: Default::default(),
            initial_margin: Default::default(),
            maintenance_margin: Default::default(),
            mark_price: Default::default(),
            mark_value: Default::default(),
            open_orders_margin: Default::default(),
            pending_interest: Default::default(),
            realized_pnl: Default::default(),
            realized_pnl_excl_fees: Default::default(),
            total_fees: Default::default(),
            unrealized_pnl: Default::default(),
            unrealized_pnl_excl_fees: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    #[serde(rename = "algo_duration_sec", default, skip_serializing_if = "Option::is_none")]
    pub algo_duration_sec: Option<serde_json::Number>,
    #[serde(rename = "algo_num_slices", default, skip_serializing_if = "Option::is_none")]
    pub algo_num_slices: Option<serde_json::Number>,
    #[serde(rename = "algo_type", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<AlgoType>,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "client", default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "extra_fee", default, skip_serializing_if = "Option::is_none")]
    pub extra_fee: Option<String>,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "is_atomic_signing", default, skip_serializing_if = "Option::is_none")]
    pub is_atomic_signing: Option<bool>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "limit_price")]
    pub limit_price: String,
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "mmp", default, skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "order_type", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "referral_code", default, skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(rename = "reject_post_only", default, skip_serializing_if = "Option::is_none")]
    pub reject_post_only: Option<bool>,
    #[serde(rename = "reject_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub reject_timestamp: Option<serde_json::Number>,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "time_in_force", default, skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(rename = "trigger_price", default, skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "trigger_price_type", default, skip_serializing_if = "Option::is_none")]
    pub trigger_price_type: Option<TriggerPriceType>,
    #[serde(rename = "trigger_type", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<TriggerType>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateOrderRequest {
    fn default() -> Self {
        Self {
            algo_duration_sec: Default::default(),
            algo_num_slices: Default::default(),
            algo_type: Default::default(),
            amount: Default::default(),
            client: Default::default(),
            direction: Default::default(),
            extra_fee: Default::default(),
            instrument_name: Default::default(),
            is_atomic_signing: Default::default(),
            label: Default::default(),
            limit_price: Default::default(),
            max_fee: Default::default(),
            mmp: Default::default(),
            nonce: Default::default(),
            order_type: Default::default(),
            reduce_only: Default::default(),
            referral_code: Default::default(),
            reject_post_only: Default::default(),
            reject_timestamp: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            time_in_force: Default::default(),
            trigger_price: Default::default(),
            trigger_price_type: Default::default(),
            trigger_type: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateVaultRequest {
    #[serde(rename = "benchmark_asset", default, skip_serializing_if = "Option::is_none")]
    pub benchmark_asset: Option<Address>,
    #[serde(rename = "cooldown_sec")]
    pub cooldown_sec: serde_json::Number,
    #[serde(rename = "deposit_spot_asset")]
    pub deposit_spot_asset: Address,
    #[serde(rename = "initial_deposit")]
    pub initial_deposit: String,
    #[serde(rename = "initial_share_price_usd")]
    pub initial_share_price_usd: String,
    #[serde(rename = "management_fee_bps")]
    pub management_fee_bps: serde_json::Number,
    #[serde(rename = "manager_id")]
    pub manager_id: serde_json::Number,
    #[serde(rename = "max_fee_usd")]
    pub max_fee_usd: String,
    #[serde(rename = "max_slippage_bps")]
    pub max_slippage_bps: serde_json::Number,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "performance_fee_bps")]
    pub performance_fee_bps: serde_json::Number,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: Address,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateVaultRequest {
    fn default() -> Self {
        Self {
            benchmark_asset: Default::default(),
            cooldown_sec: 0.into(),
            deposit_spot_asset: Default::default(),
            initial_deposit: Default::default(),
            initial_share_price_usd: Default::default(),
            management_fee_bps: 0.into(),
            manager_id: 0.into(),
            max_fee_usd: Default::default(),
            max_slippage_bps: 0.into(),
            nonce: Default::default(),
            performance_fee_bps: 0.into(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Currency {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "managers")]
    pub managers: Vec<UniverseManagers>,
    #[serde(rename = "market_type")]
    pub market_type: MarketType,
    #[serde(rename = "option", default, skip_serializing_if = "Option::is_none")]
    pub option: Option<AssetEntry>,
    #[serde(rename = "perp", default, skip_serializing_if = "Option::is_none")]
    pub perp: Option<AssetEntry>,
    #[serde(rename = "spot")]
    pub spot: Vec<SpotAssetEntry>,
    #[serde(rename = "spot_price")]
    pub spot_price: String,
    #[serde(rename = "spot_price_24h", default, skip_serializing_if = "Option::is_none")]
    pub spot_price_24h: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DailyTradingStatistics {
    #[serde(rename = "c")]
    pub c: String,
    #[serde(rename = "h")]
    pub h: String,
    #[serde(rename = "l")]
    pub l: String,
    #[serde(rename = "n")]
    pub n: serde_json::Number,
    #[serde(rename = "oi")]
    pub oi: String,
    #[serde(rename = "p")]
    pub p: String,
    #[serde(rename = "pr")]
    pub pr: String,
    #[serde(rename = "v")]
    pub v: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DailyTradingStatistics {
    fn default() -> Self {
        Self {
            c: Default::default(),
            h: Default::default(),
            l: Default::default(),
            n: 0.into(),
            oi: Default::default(),
            p: Default::default(),
            pr: Default::default(),
            v: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DecodeActionRequest {
    #[serde(rename = "action_type")]
    pub action_type: DecodableActionType,
    #[serde(rename = "encoded_data")]
    pub encoded_data: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DecodeActionResponse {
    #[serde(rename = "action_type")]
    pub action_type: DecodableActionType,
    #[serde(rename = "decoded")]
    pub decoded: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeleteSubaccountRequest {
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DeleteSubaccountRequest {
    fn default() -> Self {
        Self {
            nonce: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeleteSubaccountResponse {
    #[serde(rename = "op_uuid")]
    pub op_uuid: String,
    #[serde(rename = "operation_id")]
    pub operation_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DeleteSubaccountResponse {
    fn default() -> Self {
        Self {
            op_uuid: Default::default(),
            operation_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DepositBridgeOrigin {
    #[serde(rename = "rail")]
    pub rail: String,
    #[serde(rename = "source_address", default, skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,
    #[serde(rename = "source_chain_id")]
    pub source_chain_id: serde_json::Number,
    #[serde(rename = "source_tx_hash")]
    pub source_tx_hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DepositBridgeOrigin {
    fn default() -> Self {
        Self {
            rail: Default::default(),
            source_address: Default::default(),
            source_chain_id: 0.into(),
            source_tx_hash: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DepositEntry {
    #[serde(rename = "action_id", default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<serde_json::Number>,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "batch_status")]
    pub batch_status: BatchStatus,
    #[serde(rename = "batch_uuid")]
    pub batch_uuid: String,
    #[serde(rename = "bridge_origin", default, skip_serializing_if = "Option::is_none")]
    pub bridge_origin: Option<DepositBridgeOrigin>,
    #[serde(rename = "fallback_error_code", default, skip_serializing_if = "Option::is_none")]
    pub fallback_error_code: Option<serde_json::Number>,
    #[serde(rename = "fallback_error_data", default, skip_serializing_if = "Option::is_none")]
    pub fallback_error_data: Option<String>,
    #[serde(rename = "fallback_error_message", default, skip_serializing_if = "Option::is_none")]
    pub fallback_error_message: Option<String>,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "is_fallback")]
    pub is_fallback: bool,
    #[serde(rename = "l1_sender", default, skip_serializing_if = "Option::is_none")]
    pub l1_sender: Option<String>,
    #[serde(rename = "l1_tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub l1_tx_hash: Option<String>,
    #[serde(rename = "new_subaccount")]
    pub new_subaccount: bool,
    #[serde(rename = "operation_id")]
    pub operation_id: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DepositEntry {
    fn default() -> Self {
        Self {
            action_id: Default::default(),
            amount: Default::default(),
            asset: Default::default(),
            batch_status: Default::default(),
            batch_uuid: Default::default(),
            bridge_origin: Default::default(),
            fallback_error_code: Default::default(),
            fallback_error_data: Default::default(),
            fallback_error_message: Default::default(),
            fee: Default::default(),
            is_fallback: Default::default(),
            l1_sender: Default::default(),
            l1_tx_hash: Default::default(),
            new_subaccount: Default::default(),
            operation_id: Default::default(),
            subaccount_id: 0.into(),
            timestamp: 0.into(),
            tx_hash: Default::default(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DepositHistoryResult {
    #[serde(rename = "deposits")]
    pub deposits: Vec<DepositEntry>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct EditSessionKeyRequest {
    #[serde(rename = "ip_whitelist", default, skip_serializing_if = "Option::is_none")]
    pub ip_whitelist: Option<Vec<String>>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "offchain_scopes", default, skip_serializing_if = "Option::is_none")]
    pub offchain_scopes: Option<Vec<String>>,
    #[serde(rename = "public_session_key")]
    pub public_session_key: String,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct EmptyRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Erc20Details {
    #[serde(rename = "decimals")]
    pub decimals: serde_json::Number,
    #[serde(rename = "underlying_erc20", default, skip_serializing_if = "Option::is_none")]
    pub underlying_erc20: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for Erc20Details {
    fn default() -> Self {
        Self {
            decimals: 0.into(),
            underlying_erc20: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExecuteQuoteRequest {
    #[serde(rename = "client", default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "enable_taker_protection", default, skip_serializing_if = "Option::is_none")]
    pub enable_taker_protection: Option<bool>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "legs")]
    pub legs: Vec<PricedLegParamsAndResponse>,
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "referral_code", default, skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: Address,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ExecuteQuoteRequest {
    fn default() -> Self {
        Self {
            client: Default::default(),
            direction: Default::default(),
            enable_taker_protection: Default::default(),
            label: Default::default(),
            legs: Default::default(),
            max_fee: Default::default(),
            nonce: Default::default(),
            quote_id: Default::default(),
            referral_code: Default::default(),
            rfq_id: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExpirySettlementPrice {
    #[serde(rename = "expiry_date")]
    pub expiry_date: String,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "utc_expiry_sec")]
    pub utc_expiry_sec: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ExpirySettlementPrice {
    fn default() -> Self {
        Self {
            expiry_date: Default::default(),
            price: Default::default(),
            utc_expiry_sec: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExternalTransferActionDataResponse {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "max_fee_usd")]
    pub max_fee_usd: String,
    #[serde(rename = "new_subaccount_manager")]
    pub new_subaccount_manager: serde_json::Number,
    #[serde(rename = "recipient_address")]
    pub recipient_address: String,
    #[serde(rename = "sub_id")]
    pub sub_id: String,
    #[serde(rename = "to_subaccount_id")]
    pub to_subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ExternalTransferActionDataResponse {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            asset: Default::default(),
            max_fee_usd: Default::default(),
            new_subaccount_manager: 0.into(),
            recipient_address: Default::default(),
            sub_id: Default::default(),
            to_subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ForceBurnRequest {
    #[serde(rename = "holder")]
    pub holder: Address,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: Address,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ForceBurnRequest {
    fn default() -> Self {
        Self {
            holder: Default::default(),
            nonce: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ForwardFeedDataResponse {
    #[serde(rename = "confidence")]
    pub confidence: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "deadline")]
    pub deadline: serde_json::Number,
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(rename = "fwd_diff")]
    pub fwd_diff: String,
    #[serde(rename = "signatures")]
    pub signatures: OracleSignatureDataResponse,
    #[serde(rename = "spot_aggregate_latest")]
    pub spot_aggregate_latest: String,
    #[serde(rename = "spot_aggregate_start")]
    pub spot_aggregate_start: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ForwardFeedDataResponse {
    fn default() -> Self {
        Self {
            confidence: Default::default(),
            currency: Default::default(),
            deadline: 0.into(),
            expiry: 0.into(),
            fwd_diff: Default::default(),
            signatures: Default::default(),
            spot_aggregate_latest: Default::default(),
            spot_aggregate_start: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ForwardedToken {
    #[serde(rename = "bridge_type")]
    pub bridge_type: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FundingFeedDataResponse {
    #[serde(rename = "confidence")]
    pub confidence: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "deadline")]
    pub deadline: serde_json::Number,
    #[serde(rename = "funding_rate")]
    pub funding_rate: String,
    #[serde(rename = "signatures")]
    pub signatures: OracleSignatureDataResponse,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for FundingFeedDataResponse {
    fn default() -> Self {
        Self {
            confidence: Default::default(),
            currency: Default::default(),
            deadline: 0.into(),
            funding_rate: Default::default(),
            signatures: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FundingRateCandle {
    #[serde(rename = "close")]
    pub close: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "funding_rate")]
    pub funding_rate: String,
    #[serde(rename = "high")]
    pub high: String,
    #[serde(rename = "low")]
    pub low: String,
    #[serde(rename = "open")]
    pub open: String,
    #[serde(rename = "risk_universe_id")]
    pub risk_universe_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for FundingRateCandle {
    fn default() -> Self {
        Self {
            close: Default::default(),
            currency: Default::default(),
            funding_rate: Default::default(),
            high: Default::default(),
            low: Default::default(),
            open: Default::default(),
            risk_universe_id: 0.into(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FundingRateHistoryResult {
    #[serde(rename = "funding_rate_history")]
    pub funding_rate_history: Vec<FundingRateCandle>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountRequest {
    #[serde(rename = "wallet")]
    pub wallet: Address,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetAlgoOrdersRequest {
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetAlgoOrdersRequest {
    fn default() -> Self {
        Self {
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAllInstrumentsRequest {
    #[serde(rename = "currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "expired")]
    pub expired: bool,
    #[serde(rename = "instrument_type")]
    pub instrument_type: AssetType,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "risk_universe_id", default, skip_serializing_if = "Option::is_none")]
    pub risk_universe_id: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAllInstrumentsResponse {
    #[serde(rename = "instruments")]
    pub instruments: Vec<Instrument>,
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAllPortfoliosRequest {
    #[serde(rename = "wallet")]
    pub wallet: Address,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAllReferralCodesParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetCollateralsRequest {
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetCollateralsRequest {
    fn default() -> Self {
        Self {
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCuratedVaultsRequest {
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCurrencyRequest {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetDepositHistoryRequest {
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<serde_json::Number>,
    #[serde(rename = "wallet", default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetErc20TransferHistoryRequest {
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<serde_json::Number>,
    #[serde(rename = "wallet", default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFundingHistoryRequest {
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "instrument_name", default, skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<serde_json::Number>,
    #[serde(rename = "wallet", default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFundingRateHistoryRequest {
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "period", default, skip_serializing_if = "Option::is_none")]
    pub period: Option<serde_json::Number>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetIndexChartDataRequest {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: serde_json::Number,
    #[serde(rename = "period")]
    pub period: serde_json::Number,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetIndexChartDataRequest {
    fn default() -> Self {
        Self {
            currency: Default::default(),
            end_timestamp: 0.into(),
            period: 0.into(),
            start_timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInstrumentRequest {
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestHistoryRequest {
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<serde_json::Number>,
    #[serde(rename = "wallet", default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInterestRateHistoryRequest {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "period", default, skip_serializing_if = "Option::is_none")]
    pub period: Option<serde_json::Number>,
    #[serde(rename = "risk_universe_id", default, skip_serializing_if = "Option::is_none")]
    pub risk_universe_id: Option<serde_json::Number>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLatestSignedFeedsRequest {
    #[serde(rename = "currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "expiry", default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLatestSignedFeedsResponse {
    #[serde(rename = "funding_data")]
    pub funding_data: std::collections::BTreeMap<String,FundingFeedDataResponse>,
    #[serde(rename = "fwd_data")]
    pub fwd_data: std::collections::BTreeMap<String,std::collections::BTreeMap<String,ForwardFeedDataResponse>>,
    #[serde(rename = "perp_data")]
    pub perp_data: std::collections::BTreeMap<String,std::collections::BTreeMap<String,PerpFeedDataResponse>>,
    #[serde(rename = "rate_data")]
    pub rate_data: std::collections::BTreeMap<String,std::collections::BTreeMap<String,RateFeedDataResponse>>,
    #[serde(rename = "spot_data")]
    pub spot_data: std::collections::BTreeMap<String,SpotFeedDataResponse>,
    #[serde(rename = "vol_data")]
    pub vol_data: std::collections::BTreeMap<String,std::collections::BTreeMap<String,VolFeedDataResponse>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLiquidationHistoryRequest {
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetLiveBurnRequestsRequest {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetLiveBurnRequestsRequest {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLiveIncidentsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLiveIncidentsResponse {
    #[serde(rename = "incidents")]
    pub incidents: Vec<IncidentResponse>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetLiveMintRequestsRequest {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetLiveMintRequestsRequest {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLiveVaultRequestsRequest {
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMakerProgramScoresParams {
    #[serde(rename = "epoch_start_timestamp")]
    pub epoch_start_timestamp: serde_json::Number,
    #[serde(rename = "program_name")]
    pub program_name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetMakerProgramScoresParams {
    fn default() -> Self {
        Self {
            epoch_start_timestamp: 0.into(),
            program_name: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMakerProgramScoresResponse {
    #[serde(rename = "program")]
    pub program: ProgramResponse,
    #[serde(rename = "scores")]
    pub scores: Vec<ScoreBreakdown>,
    #[serde(rename = "total_score")]
    pub total_score: String,
    #[serde(rename = "total_volume")]
    pub total_volume: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMakerProgramsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMakerRfqHistoryRequest {
    #[serde(rename = "from_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<serde_json::Number>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "rfq_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(rename = "rfq_subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_subaccount_id: Option<serde_json::Number>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RFQStatus>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "to_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub to_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetMakerRfqHistoryRequest {
    fn default() -> Self {
        Self {
            from_timestamp: Default::default(),
            page: Default::default(),
            page_size: Default::default(),
            rfq_id: Default::default(),
            rfq_subaccount_id: Default::default(),
            status: Default::default(),
            subaccount_id: 0.into(),
            to_timestamp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMarginResponse {
    #[serde(rename = "is_valid_trade")]
    pub is_valid_trade: bool,
    #[serde(rename = "post_initial_margin")]
    pub post_initial_margin: String,
    #[serde(rename = "post_maintenance_margin")]
    pub post_maintenance_margin: String,
    #[serde(rename = "pre_initial_margin")]
    pub pre_initial_margin: String,
    #[serde(rename = "pre_maintenance_margin")]
    pub pre_maintenance_margin: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetMarginResponse {
    fn default() -> Self {
        Self {
            is_valid_trade: Default::default(),
            post_initial_margin: Default::default(),
            post_maintenance_margin: Default::default(),
            pre_initial_margin: Default::default(),
            pre_maintenance_margin: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOnchainActionHistoryParams {
    #[serde(rename = "action_type", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<serde_json::Number>,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "wallet", default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOnchainActionHistoryResponse {
    #[serde(rename = "actions")]
    pub actions: Vec<OnchainActionHistoryEntry>,
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetOpenOrdersRequest {
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetOpenOrdersRequest {
    fn default() -> Self {
        Self {
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOptionSettlementHistoryParams {
    #[serde(rename = "subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<serde_json::Number>,
    #[serde(rename = "wallet", default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOptionSettlementPricesRequest {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderHistoryRequest {
    #[serde(rename = "from_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<serde_json::Number>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<serde_json::Number>,
    #[serde(rename = "to_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub to_timestamp: Option<serde_json::Number>,
    #[serde(rename = "wallet", default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetOrderRequest {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetOrderRequest {
    fn default() -> Self {
        Self {
            order_id: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPendingDepositsParams {
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPendingDepositsResult {
    #[serde(rename = "pending_deposits")]
    pub pending_deposits: Vec<PendingDepositEntry>,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPerpImpactTwapRequest {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "end_time")]
    pub end_time: serde_json::Number,
    #[serde(rename = "start_time")]
    pub start_time: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetPerpImpactTwapRequest {
    fn default() -> Self {
        Self {
            currency: Default::default(),
            end_time: 0.into(),
            start_time: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPositionsRequest {
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetPositionsRequest {
    fn default() -> Self {
        Self {
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPublicTradeHistoryRequest {
    #[serde(rename = "batch_status", default, skip_serializing_if = "Option::is_none")]
    pub batch_status: Option<BatchStatus>,
    #[serde(rename = "currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "from_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<serde_json::Number>,
    #[serde(rename = "instrument_name", default, skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    #[serde(rename = "instrument_type", default, skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<AssetType>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<serde_json::Number>,
    #[serde(rename = "to_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub to_timestamp: Option<serde_json::Number>,
    #[serde(rename = "trade_id", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetQuotesRequest {
    #[serde(rename = "from_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<serde_json::Number>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "quote_id", default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(rename = "rfq_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "to_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub to_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetQuotesRequest {
    fn default() -> Self {
        Self {
            from_timestamp: Default::default(),
            page: Default::default(),
            page_size: Default::default(),
            quote_id: Default::default(),
            rfq_id: Default::default(),
            status: Default::default(),
            subaccount_id: 0.into(),
            to_timestamp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetReferralPerformanceParams {
    #[serde(rename = "end_ms")]
    pub end_ms: serde_json::Number,
    #[serde(rename = "referral_code", default, skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(rename = "start_ms")]
    pub start_ms: serde_json::Number,
    #[serde(rename = "wallet", default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetReferralPerformanceParams {
    fn default() -> Self {
        Self {
            end_ms: 0.into(),
            referral_code: Default::default(),
            start_ms: 0.into(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetReferralPerformanceResult {
    #[serde(rename = "fee_share_percentage")]
    pub fee_share_percentage: String,
    #[serde(rename = "referral_code")]
    pub referral_code: String,
    #[serde(rename = "rewards")]
    pub rewards: std::collections::BTreeMap<String,std::collections::BTreeMap<String,std::collections::BTreeMap<String,ReferralPerformanceByInstrumentType>>>,
    #[serde(rename = "stdrv_balance")]
    pub stdrv_balance: String,
    #[serde(rename = "total_builder_fee_collected")]
    pub total_builder_fee_collected: String,
    #[serde(rename = "total_fee_rewards")]
    pub total_fee_rewards: String,
    #[serde(rename = "total_notional_volume")]
    pub total_notional_volume: String,
    #[serde(rename = "total_referred_fees")]
    pub total_referred_fees: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetRfqsRequest {
    #[serde(rename = "from_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<serde_json::Number>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "rfq_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "to_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub to_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetRfqsRequest {
    fn default() -> Self {
        Self {
            from_timestamp: Default::default(),
            page: Default::default(),
            page_size: Default::default(),
            rfq_id: Default::default(),
            status: Default::default(),
            subaccount_id: 0.into(),
            to_timestamp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetShareholderVaultsRequest {
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetSubaccountRequest {
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetSubaccountRequest {
    fn default() -> Self {
        Self {
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSubaccountValueHistoryRequest {
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "period", default, skip_serializing_if = "Option::is_none")]
    pub period: Option<serde_json::Number>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<serde_json::Number>,
    #[serde(rename = "wallet", default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSubaccountsRequest {
    #[serde(rename = "wallet")]
    pub wallet: Address,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTickerRequest {
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTickersRequest {
    #[serde(rename = "currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "expiry_date", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<serde_json::Number>,
    #[serde(rename = "instrument_type")]
    pub instrument_type: AssetType,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTickersResponse {
    #[serde(rename = "tickers")]
    pub tickers: std::collections::BTreeMap<String,TickerSlimSnapshot>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTradeHistoryRequest {
    #[serde(rename = "from_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<serde_json::Number>,
    #[serde(rename = "instrument_name", default, skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    #[serde(rename = "order_id", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "quote_id", default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(rename = "subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<serde_json::Number>,
    #[serde(rename = "to_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub to_timestamp: Option<serde_json::Number>,
    #[serde(rename = "wallet", default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetTradingviewChartDataRequest {
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: serde_json::Number,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "period")]
    pub period: serde_json::Number,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetTradingviewChartDataRequest {
    fn default() -> Self {
        Self {
            end_timestamp: 0.into(),
            instrument_name: Default::default(),
            period: 0.into(),
            start_timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTransactionParams {
    #[serde(rename = "op_uuid")]
    pub op_uuid: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTransactionResult {
    #[serde(rename = "data")]
    pub data: String,
    #[serde(rename = "error_log", default, skip_serializing_if = "Option::is_none")]
    pub error_log: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<BatchStatus>,
    #[serde(rename = "transaction_hash", default, skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetTriggerOrdersRequest {
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetTriggerOrdersRequest {
    fn default() -> Self {
        Self {
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetVaultActionHistoryRequest {
    #[serde(rename = "event_type", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetVaultActionHistoryRequest {
    fn default() -> Self {
        Self {
            event_type: Default::default(),
            page: Default::default(),
            page_size: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetVaultPerformanceHistoryRequest {
    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "resolution")]
    pub resolution: PerformanceResolution,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetVaultPerformanceHistoryRequest {
    fn default() -> Self {
        Self {
            from: Default::default(),
            limit: Default::default(),
            resolution: Default::default(),
            subaccount_id: 0.into(),
            to: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetVaultRequest {
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetVaultRequest {
    fn default() -> Self {
        Self {
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultRequestHistoryRequest {
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultSharesRequest {
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVaultsRequest {
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWalletsFromSessionKeyRequest {
    #[serde(rename = "public_session_key")]
    pub public_session_key: String,
    #[serde(rename = "scope", default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<OffchainKeyScope>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWithdrawAddressesParams {
    #[serde(rename = "wallet")]
    pub wallet: Address,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWithdrawAddressesResult {
    #[serde(rename = "withdraw_addresses")]
    pub withdraw_addresses: Vec<WithdrawAddressEntry>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWithdrawalHistoryRequest {
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<serde_json::Number>,
    #[serde(rename = "wallet", default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct IncidentResponse {
    #[serde(rename = "creation_timestamp_sec")]
    pub creation_timestamp_sec: serde_json::Number,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "monitor_type")]
    pub monitor_type: MonitorType,
    #[serde(rename = "severity")]
    pub severity: Severity,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for IncidentResponse {
    fn default() -> Self {
        Self {
            creation_timestamp_sec: 0.into(),
            label: Default::default(),
            message: Default::default(),
            monitor_type: Default::default(),
            severity: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct IndexCandle {
    #[serde(rename = "close_price")]
    pub close_price: String,
    #[serde(rename = "high_price")]
    pub high_price: String,
    #[serde(rename = "low_price")]
    pub low_price: String,
    #[serde(rename = "open_price")]
    pub open_price: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "timestamp_bucket")]
    pub timestamp_bucket: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for IndexCandle {
    fn default() -> Self {
        Self {
            close_price: Default::default(),
            high_price: Default::default(),
            low_price: Default::default(),
            open_price: Default::default(),
            price: Default::default(),
            timestamp: 0.into(),
            timestamp_bucket: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Instrument {
    #[serde(rename = "amount_step")]
    pub amount_step: String,
    #[serde(rename = "base_asset_address")]
    pub base_asset_address: String,
    #[serde(rename = "base_asset_sub_id")]
    pub base_asset_sub_id: String,
    #[serde(rename = "base_currency")]
    pub base_currency: String,
    #[serde(rename = "base_fee")]
    pub base_fee: String,
    #[serde(rename = "erc20_details", default, skip_serializing_if = "Option::is_none")]
    pub erc20_details: Option<SpotPublicDetails>,
    #[serde(rename = "fifo_min_allocation")]
    pub fifo_min_allocation: String,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "instrument_type")]
    pub instrument_type: AssetType,
    #[serde(rename = "is_active")]
    pub is_active: bool,
    #[serde(rename = "maker_fee_rate")]
    pub maker_fee_rate: String,
    #[serde(rename = "mark_price_fee_rate_cap", default, skip_serializing_if = "Option::is_none")]
    pub mark_price_fee_rate_cap: Option<String>,
    #[serde(rename = "maximum_amount")]
    pub maximum_amount: String,
    #[serde(rename = "minimum_amount")]
    pub minimum_amount: String,
    #[serde(rename = "option_details", default, skip_serializing_if = "Option::is_none")]
    pub option_details: Option<OptionDetails>,
    #[serde(rename = "perp_details", default, skip_serializing_if = "Option::is_none")]
    pub perp_details: Option<PerpDetails>,
    #[serde(rename = "pro_rata_amount_step")]
    pub pro_rata_amount_step: String,
    #[serde(rename = "pro_rata_fraction")]
    pub pro_rata_fraction: String,
    #[serde(rename = "quote_currency")]
    pub quote_currency: String,
    #[serde(rename = "risk_universe_id")]
    pub risk_universe_id: serde_json::Number,
    #[serde(rename = "scheduled_activation")]
    pub scheduled_activation: serde_json::Number,
    #[serde(rename = "scheduled_deactivation")]
    pub scheduled_deactivation: serde_json::Number,
    #[serde(rename = "taker_fee_rate")]
    pub taker_fee_rate: String,
    #[serde(rename = "tick_size")]
    pub tick_size: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for Instrument {
    fn default() -> Self {
        Self {
            amount_step: Default::default(),
            base_asset_address: Default::default(),
            base_asset_sub_id: Default::default(),
            base_currency: Default::default(),
            base_fee: Default::default(),
            erc20_details: Default::default(),
            fifo_min_allocation: Default::default(),
            instrument_name: Default::default(),
            instrument_type: Default::default(),
            is_active: Default::default(),
            maker_fee_rate: Default::default(),
            mark_price_fee_rate_cap: Default::default(),
            maximum_amount: Default::default(),
            minimum_amount: Default::default(),
            option_details: Default::default(),
            perp_details: Default::default(),
            pro_rata_amount_step: Default::default(),
            pro_rata_fraction: Default::default(),
            quote_currency: Default::default(),
            risk_universe_id: 0.into(),
            scheduled_activation: 0.into(),
            scheduled_deactivation: 0.into(),
            taker_fee_rate: Default::default(),
            tick_size: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct InterestHistoryResult {
    #[serde(rename = "events")]
    pub events: Vec<InterestPayment>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InterestPayment {
    #[serde(rename = "interest")]
    pub interest: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for InterestPayment {
    fn default() -> Self {
        Self {
            interest: Default::default(),
            subaccount_id: 0.into(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InterestRateCandle {
    #[serde(rename = "borrow_apy")]
    pub borrow_apy: Ohlc,
    #[serde(rename = "risk_universe_id")]
    pub risk_universe_id: serde_json::Number,
    #[serde(rename = "supply_apy")]
    pub supply_apy: Ohlc,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "total_borrow")]
    pub total_borrow: String,
    #[serde(rename = "total_supply")]
    pub total_supply: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for InterestRateCandle {
    fn default() -> Self {
        Self {
            borrow_apy: Default::default(),
            risk_universe_id: 0.into(),
            supply_apy: Default::default(),
            timestamp: 0.into(),
            total_borrow: Default::default(),
            total_supply: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct InterestRateHistoryResult {
    #[serde(rename = "interest_rate_history")]
    pub interest_rate_history: Vec<InterestRateCandle>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LegUnpricedParams {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LendingDetails {
    #[serde(rename = "borrow_apy")]
    pub borrow_apy: String,
    #[serde(rename = "supply_apy")]
    pub supply_apy: String,
    #[serde(rename = "total_borrow")]
    pub total_borrow: String,
    #[serde(rename = "total_borrow_cap")]
    pub total_borrow_cap: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LiquidationHistoryResult {
    #[serde(rename = "auctions")]
    pub auctions: Vec<AuctionHistory>,
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LiveAuction {
    #[serde(rename = "currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "estimated_bid_price")]
    pub estimated_bid_price: String,
    #[serde(rename = "estimated_discount_pnl")]
    pub estimated_discount_pnl: String,
    #[serde(rename = "estimated_mtm")]
    pub estimated_mtm: String,
    #[serde(rename = "estimated_percent_bid")]
    pub estimated_percent_bid: String,
    #[serde(rename = "margin_type")]
    pub margin_type: String,
    #[serde(rename = "min_price_limit")]
    pub min_price_limit: String,
    #[serde(rename = "risk_universe_id")]
    pub risk_universe_id: serde_json::Number,
    #[serde(rename = "subaccount_balances")]
    pub subaccount_balances: std::collections::BTreeMap<String,String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for LiveAuction {
    fn default() -> Self {
        Self {
            currency: Default::default(),
            estimated_bid_price: Default::default(),
            estimated_discount_pnl: Default::default(),
            estimated_mtm: Default::default(),
            estimated_percent_bid: Default::default(),
            margin_type: Default::default(),
            min_price_limit: Default::default(),
            risk_universe_id: 0.into(),
            subaccount_balances: Default::default(),
            subaccount_id: 0.into(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ManagerCollateral {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "erc20")]
    pub erc20: Erc20Details,
    #[serde(rename = "im_discount")]
    pub im_discount: String,
    #[serde(rename = "min_deposit_usd")]
    pub min_deposit_usd: String,
    #[serde(rename = "mm_discount")]
    pub mm_discount: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MarginWatchRequest {
    #[serde(rename = "is_delayed_liquidation", default, skip_serializing_if = "Option::is_none")]
    pub is_delayed_liquidation: Option<bool>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for MarginWatchRequest {
    fn default() -> Self {
        Self {
            is_delayed_liquidation: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MintSharesRequest {
    #[serde(rename = "deposit_hash")]
    pub deposit_hash: String,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "request_id")]
    pub request_id: VaultRequestId,
    #[serde(rename = "share_price")]
    pub share_price: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: Address,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for MintSharesRequest {
    fn default() -> Self {
        Self {
            deposit_hash: Default::default(),
            nonce: Default::default(),
            request_id: Default::default(),
            share_price: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MmpConfigResult {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "is_frozen")]
    pub is_frozen: bool,
    #[serde(rename = "mmp_amount_limit")]
    pub mmp_amount_limit: String,
    #[serde(rename = "mmp_delta_limit")]
    pub mmp_delta_limit: String,
    #[serde(rename = "mmp_frozen_time")]
    pub mmp_frozen_time: serde_json::Number,
    #[serde(rename = "mmp_interval")]
    pub mmp_interval: serde_json::Number,
    #[serde(rename = "mmp_unfreeze_time")]
    pub mmp_unfreeze_time: serde_json::Number,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for MmpConfigResult {
    fn default() -> Self {
        Self {
            currency: Default::default(),
            is_frozen: Default::default(),
            mmp_amount_limit: Default::default(),
            mmp_delta_limit: Default::default(),
            mmp_frozen_time: 0.into(),
            mmp_interval: 0.into(),
            mmp_unfreeze_time: 0.into(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MmpScopeRequest {
    #[serde(rename = "currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for MmpScopeRequest {
    fn default() -> Self {
        Self {
            currency: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MultipleVaultRequestsResponse {
    #[serde(rename = "requests")]
    pub requests: Vec<VaultRequestResponse>,
    #[serde(rename = "total")]
    pub total: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for MultipleVaultRequestsResponse {
    fn default() -> Self {
        Self {
            requests: Default::default(),
            total: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OffchainAckResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Ohlc {
    #[serde(rename = "close")]
    pub close: String,
    #[serde(rename = "high")]
    pub high: String,
    #[serde(rename = "low")]
    pub low: String,
    #[serde(rename = "open")]
    pub open: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OnchainActionHistoryEntry {
    #[serde(rename = "acc")]
    pub acc: String,
    #[serde(rename = "action_id")]
    pub action_id: serde_json::Number,
    #[serde(rename = "action_type")]
    pub action_type: serde_json::Number,
    #[serde(rename = "action_type_label")]
    pub action_type_label: String,
    #[serde(rename = "block_number")]
    pub block_number: serde_json::Number,
    #[serde(rename = "data")]
    pub data: String,
    #[serde(rename = "error_code", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<serde_json::Number>,
    #[serde(rename = "error_data", default, skip_serializing_if = "Option::is_none")]
    pub error_data: Option<String>,
    #[serde(rename = "error_message", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "fallback_at", default, skip_serializing_if = "Option::is_none")]
    pub fallback_at: Option<serde_json::Number>,
    #[serde(rename = "first_failed_at", default, skip_serializing_if = "Option::is_none")]
    pub first_failed_at: Option<serde_json::Number>,
    #[serde(rename = "l1_sender")]
    pub l1_sender: String,
    #[serde(rename = "last_failed_at", default, skip_serializing_if = "Option::is_none")]
    pub last_failed_at: Option<serde_json::Number>,
    #[serde(rename = "op_uuid", default, skip_serializing_if = "Option::is_none")]
    pub op_uuid: Option<String>,
    #[serde(rename = "queue")]
    pub queue: String,
    #[serde(rename = "skip_reason", default, skip_serializing_if = "Option::is_none")]
    pub skip_reason: Option<String>,
    #[serde(rename = "skip_reason_code", default, skip_serializing_if = "Option::is_none")]
    pub skip_reason_code: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for OnchainActionHistoryEntry {
    fn default() -> Self {
        Self {
            acc: Default::default(),
            action_id: 0.into(),
            action_type: 0.into(),
            action_type_label: Default::default(),
            block_number: 0.into(),
            data: Default::default(),
            error_code: Default::default(),
            error_data: Default::default(),
            error_message: Default::default(),
            fallback_at: Default::default(),
            first_failed_at: Default::default(),
            l1_sender: Default::default(),
            last_failed_at: Default::default(),
            op_uuid: Default::default(),
            queue: Default::default(),
            skip_reason: Default::default(),
            skip_reason_code: Default::default(),
            status: Default::default(),
            tx_hash: Default::default(),
            updated_at: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenInterestStats {
    #[serde(rename = "current_open_interest")]
    pub current_open_interest: String,
    #[serde(rename = "interest_cap")]
    pub interest_cap: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OptionDetails {
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(rename = "index")]
    pub index: String,
    #[serde(rename = "option_type")]
    pub option_type: String,
    #[serde(rename = "settlement_price", default, skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<String>,
    #[serde(rename = "strike")]
    pub strike: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for OptionDetails {
    fn default() -> Self {
        Self {
            expiry: 0.into(),
            index: Default::default(),
            option_type: Default::default(),
            settlement_price: Default::default(),
            strike: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OptionPricing {
    #[serde(rename = "ai")]
    pub ai: String,
    #[serde(rename = "bi")]
    pub bi: String,
    #[serde(rename = "d")]
    pub d: String,
    #[serde(rename = "df")]
    pub df: String,
    #[serde(rename = "f")]
    pub f: String,
    #[serde(rename = "g")]
    pub g: String,
    #[serde(rename = "i")]
    pub i: String,
    #[serde(rename = "m")]
    pub m: String,
    #[serde(rename = "r")]
    pub r: String,
    #[serde(rename = "t")]
    pub t: String,
    #[serde(rename = "v")]
    pub v: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OptionSettlementHistoryResponse {
    #[serde(rename = "settlements")]
    pub settlements: Vec<OptionSettlementResponse>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OptionSettlementPricesResult {
    #[serde(rename = "expiries")]
    pub expiries: Vec<ExpirySettlementPrice>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OptionSettlementResponse {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "option_settlement_pnl", skip_serializing_if = "Option::is_none")]
    pub option_settlement_pnl: Option<String>,
    #[serde(rename = "option_settlement_pnl_excl_fees", skip_serializing_if = "Option::is_none")]
    pub option_settlement_pnl_excl_fees: Option<String>,
    #[serde(rename = "settlement_price")]
    pub settlement_price: String,
    #[serde(rename = "settlement_value")]
    pub settlement_value: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for OptionSettlementResponse {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            expiry: 0.into(),
            instrument_name: Default::default(),
            option_settlement_pnl: Default::default(),
            option_settlement_pnl_excl_fees: Default::default(),
            settlement_price: Default::default(),
            settlement_value: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OracleSignatureDataResponse {
    #[serde(rename = "signatures")]
    pub signatures: Vec<String>,
    #[serde(rename = "signers")]
    pub signers: Vec<Address>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "algo_duration_sec", default, skip_serializing_if = "Option::is_none")]
    pub algo_duration_sec: Option<serde_json::Number>,
    #[serde(rename = "algo_num_slices", default, skip_serializing_if = "Option::is_none")]
    pub algo_num_slices: Option<serde_json::Number>,
    #[serde(rename = "algo_slices_completed", default, skip_serializing_if = "Option::is_none")]
    pub algo_slices_completed: Option<serde_json::Number>,
    #[serde(rename = "algo_type", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<AlgoType>,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "average_price")]
    pub average_price: String,
    #[serde(rename = "cancel_reason", default, skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<CancelReason>,
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: serde_json::Number,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "extra_fee")]
    pub extra_fee: String,
    #[serde(rename = "filled_amount")]
    pub filled_amount: String,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "is_transfer")]
    pub is_transfer: bool,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "last_update_timestamp")]
    pub last_update_timestamp: serde_json::Number,
    #[serde(rename = "limit_price")]
    pub limit_price: String,
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "mmp")]
    pub mmp: bool,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "order_fee")]
    pub order_fee: String,
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "order_status")]
    pub order_status: OrderStatus,
    #[serde(rename = "order_type")]
    pub order_type: OrderType,
    #[serde(rename = "quote_id", skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(rename = "replaced_order_id", skip_serializing_if = "Option::is_none")]
    pub replaced_order_id: Option<String>,
    #[serde(rename = "signature")]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signed_limit_price", skip_serializing_if = "Option::is_none")]
    pub signed_limit_price: Option<String>,
    #[serde(rename = "signer")]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "time_in_force")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "trigger_price", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "trigger_price_type", default, skip_serializing_if = "Option::is_none")]
    pub trigger_price_type: Option<TriggerPriceType>,
    #[serde(rename = "trigger_reject_message", default, skip_serializing_if = "Option::is_none")]
    pub trigger_reject_message: Option<String>,
    #[serde(rename = "trigger_type", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<TriggerType>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for Order {
    fn default() -> Self {
        Self {
            algo_duration_sec: Default::default(),
            algo_num_slices: Default::default(),
            algo_slices_completed: Default::default(),
            algo_type: Default::default(),
            amount: Default::default(),
            average_price: Default::default(),
            cancel_reason: Default::default(),
            creation_timestamp: 0.into(),
            direction: Default::default(),
            extra_fee: Default::default(),
            filled_amount: Default::default(),
            instrument_name: Default::default(),
            is_transfer: Default::default(),
            label: Default::default(),
            last_update_timestamp: 0.into(),
            limit_price: Default::default(),
            max_fee: Default::default(),
            mmp: Default::default(),
            nonce: Default::default(),
            order_fee: Default::default(),
            order_id: Default::default(),
            order_status: Default::default(),
            order_type: Default::default(),
            quote_id: Default::default(),
            replaced_order_id: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signed_limit_price: Default::default(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            time_in_force: Default::default(),
            trigger_price: Default::default(),
            trigger_price_type: Default::default(),
            trigger_reject_message: Default::default(),
            trigger_type: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderActionDataResponse {
    #[serde(rename = "asset_address")]
    pub asset_address: String,
    #[serde(rename = "asset_sub_id")]
    pub asset_sub_id: String,
    #[serde(rename = "desired_amount")]
    pub desired_amount: String,
    #[serde(rename = "is_bid")]
    pub is_bid: bool,
    #[serde(rename = "limit_price")]
    pub limit_price: String,
    #[serde(rename = "recipient_id")]
    pub recipient_id: serde_json::Number,
    #[serde(rename = "worst_fee")]
    pub worst_fee: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for OrderActionDataResponse {
    fn default() -> Self {
        Self {
            asset_address: Default::default(),
            asset_sub_id: Default::default(),
            desired_amount: Default::default(),
            is_bid: Default::default(),
            limit_price: Default::default(),
            recipient_id: 0.into(),
            worst_fee: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderActionInputData {
    #[serde(rename = "data")]
    pub data: OrderActionDataResponse,
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "signer")]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for OrderActionInputData {
    fn default() -> Self {
        Self {
            data: Default::default(),
            expiry: 0.into(),
            module: Default::default(),
            nonce: Default::default(),
            owner: Default::default(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderCreatedResponse {
    #[serde(rename = "order")]
    pub order: Order,
    #[serde(rename = "trades")]
    pub trades: Vec<Trade>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderDebugResponse {
    #[serde(rename = "action_hash")]
    pub action_hash: String,
    #[serde(rename = "action_typehash")]
    pub action_typehash: String,
    #[serde(rename = "domain_separator")]
    pub domain_separator: String,
    #[serde(rename = "encoded_data")]
    pub encoded_data: String,
    #[serde(rename = "encoded_data_hashed")]
    pub encoded_data_hashed: String,
    #[serde(rename = "expected_signer")]
    pub expected_signer: String,
    #[serde(rename = "input_data")]
    pub input_data: OrderActionInputData,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "recovered_signer", default, skip_serializing_if = "Option::is_none")]
    pub recovered_signer: Option<String>,
    #[serde(rename = "typed_data_hash")]
    pub typed_data_hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderQuoteRequest {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "client", default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "extra_fee", default, skip_serializing_if = "Option::is_none")]
    pub extra_fee: Option<String>,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "is_atomic_signing", default, skip_serializing_if = "Option::is_none")]
    pub is_atomic_signing: Option<bool>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "limit_price")]
    pub limit_price: String,
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "mmp", default, skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "order_type", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "referral_code", default, skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(rename = "reject_post_only", default, skip_serializing_if = "Option::is_none")]
    pub reject_post_only: Option<bool>,
    #[serde(rename = "reject_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub reject_timestamp: Option<serde_json::Number>,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "time_in_force", default, skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(rename = "trigger_price", default, skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "trigger_price_type", default, skip_serializing_if = "Option::is_none")]
    pub trigger_price_type: Option<TriggerPriceType>,
    #[serde(rename = "trigger_type", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<TriggerType>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for OrderQuoteRequest {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            client: Default::default(),
            direction: Default::default(),
            extra_fee: Default::default(),
            instrument_name: Default::default(),
            is_atomic_signing: Default::default(),
            label: Default::default(),
            limit_price: Default::default(),
            max_fee: Default::default(),
            mmp: Default::default(),
            nonce: Default::default(),
            order_type: Default::default(),
            reduce_only: Default::default(),
            referral_code: Default::default(),
            reject_post_only: Default::default(),
            reject_timestamp: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            time_in_force: Default::default(),
            trigger_price: Default::default(),
            trigger_price_type: Default::default(),
            trigger_type: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderQuoteResponse {
    #[serde(rename = "estimated_fee")]
    pub estimated_fee: String,
    #[serde(rename = "estimated_fill_amount")]
    pub estimated_fill_amount: String,
    #[serde(rename = "estimated_fill_price")]
    pub estimated_fill_price: String,
    #[serde(rename = "estimated_order_status")]
    pub estimated_order_status: OrderStatus,
    #[serde(rename = "estimated_realized_pnl")]
    pub estimated_realized_pnl: String,
    #[serde(rename = "estimated_realized_pnl_excl_fees")]
    pub estimated_realized_pnl_excl_fees: String,
    #[serde(rename = "invalid_reason", default, skip_serializing_if = "Option::is_none")]
    pub invalid_reason: Option<String>,
    #[serde(rename = "is_valid")]
    pub is_valid: bool,
    #[serde(rename = "max_amount", default, skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<String>,
    #[serde(rename = "post_initial_margin")]
    pub post_initial_margin: String,
    #[serde(rename = "post_liquidation_price", default, skip_serializing_if = "Option::is_none")]
    pub post_liquidation_price: Option<String>,
    #[serde(rename = "pre_initial_margin")]
    pub pre_initial_margin: String,
    #[serde(rename = "suggested_max_fee")]
    pub suggested_max_fee: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PaginatedOrdersResult {
    #[serde(rename = "orders")]
    pub orders: Vec<Order>,
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PaginatedOrdersResult {
    fn default() -> Self {
        Self {
            orders: Default::default(),
            pagination: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PaginatedTradesResult {
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "trades")]
    pub trades: Vec<TradeHistoryResponse>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PaginatedTradesResult {
    fn default() -> Self {
        Self {
            pagination: Default::default(),
            subaccount_id: 0.into(),
            trades: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PaginatedVaultActionHistory {
    #[serde(rename = "events")]
    pub events: Vec<PublicVaultActionResponse>,
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PaginatedVaultActionHistory {
    fn default() -> Self {
        Self {
            events: Default::default(),
            pagination: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PaginatedVaultRequestHistory {
    #[serde(rename = "actions")]
    pub actions: Vec<VaultActionResponse>,
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Pagination {
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "num_pages")]
    pub num_pages: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for Pagination {
    fn default() -> Self {
        Self {
            count: 0.into(),
            num_pages: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PendingDepositBridgeOrigin {
    #[serde(rename = "bridge_id")]
    pub bridge_id: String,
    #[serde(rename = "bridge_status")]
    pub bridge_status: String,
    #[serde(rename = "delivered_at_ms", default, skip_serializing_if = "Option::is_none")]
    pub delivered_at_ms: Option<serde_json::Number>,
    #[serde(rename = "deposit_address")]
    pub deposit_address: String,
    #[serde(rename = "expected_amount")]
    pub expected_amount: String,
    #[serde(rename = "expires_at_ms")]
    pub expires_at_ms: serde_json::Number,
    #[serde(rename = "provider_transfer_id", default, skip_serializing_if = "Option::is_none")]
    pub provider_transfer_id: Option<String>,
    #[serde(rename = "rail")]
    pub rail: String,
    #[serde(rename = "receiving_amount", default, skip_serializing_if = "Option::is_none")]
    pub receiving_amount: Option<String>,
    #[serde(rename = "receiving_token", default, skip_serializing_if = "Option::is_none")]
    pub receiving_token: Option<String>,
    #[serde(rename = "receiving_tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub receiving_tx_hash: Option<String>,
    #[serde(rename = "source_address", default, skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,
    #[serde(rename = "source_chain_id")]
    pub source_chain_id: serde_json::Number,
    #[serde(rename = "source_tx_hash")]
    pub source_tx_hash: String,
    #[serde(rename = "substatus")]
    pub substatus: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PendingDepositBridgeOrigin {
    fn default() -> Self {
        Self {
            bridge_id: Default::default(),
            bridge_status: Default::default(),
            delivered_at_ms: Default::default(),
            deposit_address: Default::default(),
            expected_amount: Default::default(),
            expires_at_ms: 0.into(),
            provider_transfer_id: Default::default(),
            rail: Default::default(),
            receiving_amount: Default::default(),
            receiving_token: Default::default(),
            receiving_tx_hash: Default::default(),
            source_address: Default::default(),
            source_chain_id: 0.into(),
            source_tx_hash: Default::default(),
            substatus: Default::default(),
            token: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PendingDepositEntry {
    #[serde(rename = "action_id", default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<serde_json::Number>,
    #[serde(rename = "action_type")]
    pub action_type: String,
    #[serde(rename = "amount", default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "block_number", default, skip_serializing_if = "Option::is_none")]
    pub block_number: Option<serde_json::Number>,
    #[serde(rename = "bridge_to_l1", default, skip_serializing_if = "Option::is_none")]
    pub bridge_to_l1: Option<PendingDepositBridgeOrigin>,
    #[serde(rename = "deposit_type")]
    pub deposit_type: String,
    #[serde(rename = "log_index", default, skip_serializing_if = "Option::is_none")]
    pub log_index: Option<serde_json::Number>,
    #[serde(rename = "manager_id")]
    pub manager_id: serde_json::Number,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "updated_at_ms")]
    pub updated_at_ms: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PendingDepositEntry {
    fn default() -> Self {
        Self {
            action_id: Default::default(),
            action_type: Default::default(),
            amount: Default::default(),
            asset: Default::default(),
            block_number: Default::default(),
            bridge_to_l1: Default::default(),
            deposit_type: Default::default(),
            log_index: Default::default(),
            manager_id: 0.into(),
            status: Default::default(),
            subaccount_id: 0.into(),
            timestamp: 0.into(),
            tx_hash: Default::default(),
            updated_at_ms: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpDetails {
    #[serde(rename = "aggregate_funding")]
    pub aggregate_funding: String,
    #[serde(rename = "funding_rate")]
    pub funding_rate: String,
    #[serde(rename = "index")]
    pub index: String,
    #[serde(rename = "max_rate_per_hour")]
    pub max_rate_per_hour: String,
    #[serde(rename = "min_rate_per_hour")]
    pub min_rate_per_hour: String,
    #[serde(rename = "srm_perp_margin_requirements", default, skip_serializing_if = "Option::is_none")]
    pub srm_perp_margin_requirements: Option<SrmPerpMarginRequirements>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpFeedDataResponse {
    #[serde(rename = "confidence")]
    pub confidence: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "deadline")]
    pub deadline: serde_json::Number,
    #[serde(rename = "signatures")]
    pub signatures: OracleSignatureDataResponse,
    #[serde(rename = "spot_diff_value")]
    pub spot_diff_value: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpFeedDataResponse {
    fn default() -> Self {
        Self {
            confidence: Default::default(),
            currency: Default::default(),
            deadline: 0.into(),
            signatures: Default::default(),
            spot_diff_value: Default::default(),
            timestamp: 0.into(),
            type_: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpImpactTwapResult {
    #[serde(rename = "ask_impact_diff_twap")]
    pub ask_impact_diff_twap: String,
    #[serde(rename = "bid_impact_diff_twap")]
    pub bid_impact_diff_twap: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "mid_price_diff_twap")]
    pub mid_price_diff_twap: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpSettlementEventResponse {
    #[serde(rename = "batch_status")]
    pub batch_status: BatchStatus,
    #[serde(rename = "batch_uuid")]
    pub batch_uuid: String,
    #[serde(rename = "funding")]
    pub funding: String,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "pnl")]
    pub pnl: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpSettlementEventResponse {
    fn default() -> Self {
        Self {
            batch_status: Default::default(),
            batch_uuid: Default::default(),
            funding: Default::default(),
            instrument_name: Default::default(),
            pnl: Default::default(),
            subaccount_id: 0.into(),
            timestamp: 0.into(),
            tx_hash: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpSettlementHistoryResponse {
    #[serde(rename = "events")]
    pub events: Vec<PerpSettlementEventResponse>,
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PollQuotesRequest {
    #[serde(rename = "from_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<serde_json::Number>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "quote_id", default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(rename = "rfq_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "to_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub to_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PollQuotesRequest {
    fn default() -> Self {
        Self {
            from_timestamp: Default::default(),
            page: Default::default(),
            page_size: Default::default(),
            quote_id: Default::default(),
            rfq_id: Default::default(),
            status: Default::default(),
            subaccount_id: 0.into(),
            to_timestamp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PollRfqsRequest {
    #[serde(rename = "from_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<serde_json::Number>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "rfq_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(rename = "rfq_subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_subaccount_id: Option<serde_json::Number>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "to_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub to_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PollRfqsRequest {
    fn default() -> Self {
        Self {
            from_timestamp: Default::default(),
            page: Default::default(),
            page_size: Default::default(),
            rfq_id: Default::default(),
            rfq_subaccount_id: Default::default(),
            status: Default::default(),
            subaccount_id: 0.into(),
            to_timestamp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "amount_step")]
    pub amount_step: String,
    #[serde(rename = "average_price")]
    pub average_price: String,
    #[serde(rename = "average_price_excl_fees")]
    pub average_price_excl_fees: String,
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: serde_json::Number,
    #[serde(rename = "cumulative_funding")]
    pub cumulative_funding: String,
    #[serde(rename = "delta")]
    pub delta: String,
    #[serde(rename = "gamma")]
    pub gamma: String,
    #[serde(rename = "index_price")]
    pub index_price: String,
    #[serde(rename = "initial_margin")]
    pub initial_margin: String,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "instrument_type")]
    pub instrument_type: AssetType,
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(rename = "liquidation_price", default, skip_serializing_if = "Option::is_none")]
    pub liquidation_price: Option<String>,
    #[serde(rename = "maintenance_margin")]
    pub maintenance_margin: String,
    #[serde(rename = "mark_price")]
    pub mark_price: String,
    #[serde(rename = "mark_value")]
    pub mark_value: String,
    #[serde(rename = "net_settlements")]
    pub net_settlements: String,
    #[serde(rename = "open_orders_margin")]
    pub open_orders_margin: String,
    #[serde(rename = "pending_funding")]
    pub pending_funding: String,
    #[serde(rename = "realized_pnl")]
    pub realized_pnl: String,
    #[serde(rename = "realized_pnl_excl_fees")]
    pub realized_pnl_excl_fees: String,
    #[serde(rename = "theta")]
    pub theta: String,
    #[serde(rename = "total_fees")]
    pub total_fees: String,
    #[serde(rename = "unrealized_pnl")]
    pub unrealized_pnl: String,
    #[serde(rename = "unrealized_pnl_excl_fees")]
    pub unrealized_pnl_excl_fees: String,
    #[serde(rename = "vega")]
    pub vega: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for Position {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            amount_step: Default::default(),
            average_price: Default::default(),
            average_price_excl_fees: Default::default(),
            creation_timestamp: 0.into(),
            cumulative_funding: Default::default(),
            delta: Default::default(),
            gamma: Default::default(),
            index_price: Default::default(),
            initial_margin: Default::default(),
            instrument_name: Default::default(),
            instrument_type: Default::default(),
            leverage: Default::default(),
            liquidation_price: Default::default(),
            maintenance_margin: Default::default(),
            mark_price: Default::default(),
            mark_value: Default::default(),
            net_settlements: Default::default(),
            open_orders_margin: Default::default(),
            pending_funding: Default::default(),
            realized_pnl: Default::default(),
            realized_pnl_excl_fees: Default::default(),
            theta: Default::default(),
            total_fees: Default::default(),
            unrealized_pnl: Default::default(),
            unrealized_pnl_excl_fees: Default::default(),
            vega: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PricedLegParamsAndResponse {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateChangeSubaccountLabelResponse {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PrivateChangeSubaccountLabelResponse {
    fn default() -> Self {
        Self {
            label: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateGetAccountResponse {
    #[serde(rename = "cancel_on_disconnect")]
    pub cancel_on_disconnect: bool,
    #[serde(rename = "creation_timestamp_sec", default, skip_serializing_if = "Option::is_none")]
    pub creation_timestamp_sec: Option<serde_json::Number>,
    #[serde(rename = "fallback_subaccount_id")]
    pub fallback_subaccount_id: serde_json::Number,
    #[serde(rename = "fee_info")]
    pub fee_info: AccountFeeInfo,
    #[serde(rename = "is_rfq_maker")]
    pub is_rfq_maker: bool,
    #[serde(rename = "per_endpoint_tps")]
    pub per_endpoint_tps: std::collections::BTreeMap<String,serde_json::Number>,
    #[serde(rename = "referral_code", default, skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(rename = "subaccount_ids")]
    pub subaccount_ids: Vec<serde_json::Number>,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(rename = "websocket_matching_tps")]
    pub websocket_matching_tps: serde_json::Number,
    #[serde(rename = "websocket_non_matching_tps")]
    pub websocket_non_matching_tps: serde_json::Number,
    #[serde(rename = "websocket_option_tps")]
    pub websocket_option_tps: serde_json::Number,
    #[serde(rename = "websocket_perp_tps")]
    pub websocket_perp_tps: serde_json::Number,
    #[serde(rename = "whitelisted_recipients")]
    pub whitelisted_recipients: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PrivateGetAccountResponse {
    fn default() -> Self {
        Self {
            cancel_on_disconnect: Default::default(),
            creation_timestamp_sec: Default::default(),
            fallback_subaccount_id: 0.into(),
            fee_info: Default::default(),
            is_rfq_maker: Default::default(),
            per_endpoint_tps: Default::default(),
            referral_code: Default::default(),
            subaccount_ids: Default::default(),
            wallet: Default::default(),
            websocket_matching_tps: 0.into(),
            websocket_non_matching_tps: 0.into(),
            websocket_option_tps: 0.into(),
            websocket_perp_tps: 0.into(),
            whitelisted_recipients: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateGetCollateralsResponse {
    #[serde(rename = "collaterals")]
    pub collaterals: Vec<Collateral>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PrivateGetCollateralsResponse {
    fn default() -> Self {
        Self {
            collaterals: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateGetMarginRequest {
    #[serde(rename = "is_delayed_liquidation", default, skip_serializing_if = "Option::is_none")]
    pub is_delayed_liquidation: Option<bool>,
    #[serde(rename = "simulated_collateral_changes", default, skip_serializing_if = "Option::is_none")]
    pub simulated_collateral_changes: Option<Vec<SimulatedCollateralRequest>>,
    #[serde(rename = "simulated_position_changes", default, skip_serializing_if = "Option::is_none")]
    pub simulated_position_changes: Option<Vec<SimulatedPositionRequest>>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PrivateGetMarginRequest {
    fn default() -> Self {
        Self {
            is_delayed_liquidation: Default::default(),
            simulated_collateral_changes: Default::default(),
            simulated_position_changes: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateGetPositionsResponse {
    #[serde(rename = "positions")]
    pub positions: Vec<Position>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PrivateGetPositionsResponse {
    fn default() -> Self {
        Self {
            positions: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PrivateGetSubaccountsResponse {
    #[serde(rename = "subaccount_ids")]
    pub subaccount_ids: Vec<serde_json::Number>,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateLiquidateRequest {
    #[serde(rename = "liquidate_subaccount_id")]
    pub liquidate_subaccount_id: serde_json::Number,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "percent_of_acc")]
    pub percent_of_acc: String,
    #[serde(rename = "price_limit")]
    pub price_limit: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PrivateLiquidateRequest {
    fn default() -> Self {
        Self {
            liquidate_subaccount_id: 0.into(),
            nonce: Default::default(),
            percent_of_acc: Default::default(),
            price_limit: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateLiquidateResponse {
    #[serde(rename = "op_uuid")]
    pub op_uuid: String,
    #[serde(rename = "operation_id")]
    pub operation_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PrivateLiquidateResponse {
    fn default() -> Self {
        Self {
            op_uuid: Default::default(),
            operation_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PrivateSessionKeysResponse {
    #[serde(rename = "public_session_keys")]
    pub public_session_keys: Vec<SessionKey>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateSetSessionKeyResponse {
    #[serde(rename = "expiry_sec")]
    pub expiry_sec: serde_json::Number,
    #[serde(rename = "ip_whitelist")]
    pub ip_whitelist: Vec<String>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "offchain_scopes")]
    pub offchain_scopes: Vec<String>,
    #[serde(rename = "protocol_scopes")]
    pub protocol_scopes: Vec<String>,
    #[serde(rename = "public_session_key")]
    pub public_session_key: String,
    #[serde(rename = "subaccount_ids")]
    pub subaccount_ids: Vec<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PrivateSetSessionKeyResponse {
    fn default() -> Self {
        Self {
            expiry_sec: 0.into(),
            ip_whitelist: Default::default(),
            label: Default::default(),
            offchain_scopes: Default::default(),
            protocol_scopes: Default::default(),
            public_session_key: Default::default(),
            subaccount_ids: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateTransferSpotExternalRequest {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset_name")]
    pub asset_name: String,
    #[serde(rename = "max_fee_usd")]
    pub max_fee_usd: String,
    #[serde(rename = "new_subaccount_manager")]
    pub new_subaccount_manager: serde_json::Number,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "recipient_address")]
    pub recipient_address: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: String,
    #[serde(rename = "sub_id")]
    pub sub_id: serde_json::Number,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "to_subaccount_id")]
    pub to_subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PrivateTransferSpotExternalRequest {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            asset_name: Default::default(),
            max_fee_usd: Default::default(),
            new_subaccount_manager: 0.into(),
            nonce: Default::default(),
            recipient_address: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            sub_id: 0.into(),
            subaccount_id: 0.into(),
            to_subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateTransferSpotExternalResponse {
    #[serde(rename = "op_uuid")]
    pub op_uuid: String,
    #[serde(rename = "operation_id")]
    pub operation_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PrivateTransferSpotExternalResponse {
    fn default() -> Self {
        Self {
            op_uuid: Default::default(),
            operation_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateTransferSpotRequest {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset_name")]
    pub asset_name: String,
    #[serde(rename = "max_fee_usd")]
    pub max_fee_usd: String,
    #[serde(rename = "new_subaccount_manager")]
    pub new_subaccount_manager: serde_json::Number,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: String,
    #[serde(rename = "sub_id")]
    pub sub_id: serde_json::Number,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "to_subaccount_id")]
    pub to_subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PrivateTransferSpotRequest {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            asset_name: Default::default(),
            max_fee_usd: Default::default(),
            new_subaccount_manager: 0.into(),
            nonce: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            sub_id: 0.into(),
            subaccount_id: 0.into(),
            to_subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateTransferSpotResponse {
    #[serde(rename = "op_uuid")]
    pub op_uuid: String,
    #[serde(rename = "operation_id")]
    pub operation_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PrivateTransferSpotResponse {
    fn default() -> Self {
        Self {
            op_uuid: Default::default(),
            operation_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateWithdrawRequest {
    #[serde(rename = "amount_in_underlying")]
    pub amount_in_underlying: String,
    #[serde(rename = "asset_name")]
    pub asset_name: String,
    #[serde(rename = "force_batch")]
    pub force_batch: bool,
    #[serde(rename = "max_fee_usd")]
    pub max_fee_usd: String,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "recipient", default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PrivateWithdrawRequest {
    fn default() -> Self {
        Self {
            amount_in_underlying: Default::default(),
            asset_name: Default::default(),
            force_batch: Default::default(),
            max_fee_usd: Default::default(),
            nonce: Default::default(),
            recipient: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateWithdrawResponse {
    #[serde(rename = "op_uuid")]
    pub op_uuid: String,
    #[serde(rename = "operation_id")]
    pub operation_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PrivateWithdrawResponse {
    fn default() -> Self {
        Self {
            op_uuid: Default::default(),
            operation_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProgramResponse {
    #[serde(rename = "asset_types")]
    pub asset_types: Vec<String>,
    #[serde(rename = "currencies")]
    pub currencies: Vec<String>,
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: serde_json::Number,
    #[serde(rename = "min_notional")]
    pub min_notional: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "rewards")]
    pub rewards: std::collections::BTreeMap<String,String>,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ProgramResponse {
    fn default() -> Self {
        Self {
            asset_types: Default::default(),
            currencies: Default::default(),
            end_timestamp: 0.into(),
            min_notional: Default::default(),
            name: Default::default(),
            rewards: Default::default(),
            start_timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProtocolVault {
    #[serde(rename = "closed")]
    pub closed: bool,
    #[serde(rename = "config")]
    pub config: VaultConfig,
    #[serde(rename = "global_hwm")]
    pub global_hwm: String,
    #[serde(rename = "last_fee_settled_at_sec")]
    pub last_fee_settled_at_sec: serde_json::Number,
    #[serde(rename = "protocol_fee_share_bps")]
    pub protocol_fee_share_bps: serde_json::Number,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "total_shares")]
    pub total_shares: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ProtocolVault {
    fn default() -> Self {
        Self {
            closed: Default::default(),
            config: Default::default(),
            global_hwm: Default::default(),
            last_fee_settled_at_sec: 0.into(),
            protocol_fee_share_bps: 0.into(),
            subaccount_id: 0.into(),
            total_shares: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicExecuteQuoteDebugRequest {
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "legs")]
    pub legs: Vec<PricedLegParamsAndResponse>,
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: Address,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PublicExecuteQuoteDebugRequest {
    fn default() -> Self {
        Self {
            direction: Default::default(),
            legs: Default::default(),
            max_fee: Default::default(),
            nonce: Default::default(),
            quote_id: Default::default(),
            rfq_id: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PublicGetLiveAuctionsResponse {
    #[serde(rename = "auctions")]
    pub auctions: Vec<LiveAuction>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PublicGetMarginRequest {
    #[serde(rename = "margin_type")]
    pub margin_type: String,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "simulated_collateral_changes", default, skip_serializing_if = "Option::is_none")]
    pub simulated_collateral_changes: Option<Vec<SimulatedCollateralRequest>>,
    #[serde(rename = "simulated_collaterals")]
    pub simulated_collaterals: Vec<SimulatedCollateralRequest>,
    #[serde(rename = "simulated_position_changes", default, skip_serializing_if = "Option::is_none")]
    pub simulated_position_changes: Option<Vec<SimulatedPositionRequest>>,
    #[serde(rename = "simulated_positions")]
    pub simulated_positions: Vec<SimulatedPositionRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PublicGetSubaccountsRequest {
    #[serde(rename = "wallet")]
    pub wallet: Address,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicGetWalletRequest {
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PublicGetWalletRequest {
    fn default() -> Self {
        Self {
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicGetWalletResponse {
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PublicGetWalletResponse {
    fn default() -> Self {
        Self {
            subaccount_id: 0.into(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PublicGetWalletsFromSessionKeyResponse {
    #[serde(rename = "wallets")]
    pub wallets: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicMarginWatchResponse {
    #[serde(rename = "collaterals")]
    pub collaterals: Vec<Collateral>,
    #[serde(rename = "currency")]
    pub currency: Vec<String>,
    #[serde(rename = "has_delayed_liquidation")]
    pub has_delayed_liquidation: bool,
    #[serde(rename = "initial_margin")]
    pub initial_margin: String,
    #[serde(rename = "is_delayed_liquidation")]
    pub is_delayed_liquidation: bool,
    #[serde(rename = "maintenance_margin")]
    pub maintenance_margin: String,
    #[serde(rename = "margin_type")]
    pub margin_type: String,
    #[serde(rename = "positions")]
    pub positions: Vec<Position>,
    #[serde(rename = "regular_margin_flagged_sec", default, skip_serializing_if = "Option::is_none")]
    pub regular_margin_flagged_sec: Option<serde_json::Number>,
    #[serde(rename = "risk_universe_id")]
    pub risk_universe_id: serde_json::Number,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "subaccount_value")]
    pub subaccount_value: String,
    #[serde(rename = "valuation_timestamp")]
    pub valuation_timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PublicMarginWatchResponse {
    fn default() -> Self {
        Self {
            collaterals: Default::default(),
            currency: Default::default(),
            has_delayed_liquidation: Default::default(),
            initial_margin: Default::default(),
            is_delayed_liquidation: Default::default(),
            maintenance_margin: Default::default(),
            margin_type: Default::default(),
            positions: Default::default(),
            regular_margin_flagged_sec: Default::default(),
            risk_universe_id: 0.into(),
            subaccount_id: 0.into(),
            subaccount_value: Default::default(),
            valuation_timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicQuote {
    #[serde(rename = "cancel_reason")]
    pub cancel_reason: RFQCancelReason,
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: serde_json::Number,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "fill_pct")]
    pub fill_pct: String,
    #[serde(rename = "last_update_timestamp")]
    pub last_update_timestamp: serde_json::Number,
    #[serde(rename = "legs")]
    pub legs: Vec<PricedLegParamsAndResponse>,
    #[serde(rename = "legs_hash")]
    pub legs_hash: String,
    #[serde(rename = "liquidity_role")]
    pub liquidity_role: LiquidityRole,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "status")]
    pub status: RFQStatus,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "wallet")]
    pub wallet: Address,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PublicQuote {
    fn default() -> Self {
        Self {
            cancel_reason: Default::default(),
            creation_timestamp: 0.into(),
            direction: Default::default(),
            fill_pct: Default::default(),
            last_update_timestamp: 0.into(),
            legs: Default::default(),
            legs_hash: Default::default(),
            liquidity_role: Default::default(),
            quote_id: Default::default(),
            rfq_id: Default::default(),
            status: Default::default(),
            subaccount_id: 0.into(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicRfq {
    #[serde(rename = "cancel_reason")]
    pub cancel_reason: RFQCancelReason,
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: serde_json::Number,
    #[serde(rename = "fill_rate", skip_serializing_if = "Option::is_none")]
    pub fill_rate: Option<String>,
    #[serde(rename = "filled_direction", default, skip_serializing_if = "Option::is_none")]
    pub filled_direction: Option<Direction>,
    #[serde(rename = "filled_pct")]
    pub filled_pct: String,
    #[serde(rename = "last_update_timestamp")]
    pub last_update_timestamp: serde_json::Number,
    #[serde(rename = "legs")]
    pub legs: Vec<LegUnpricedParams>,
    #[serde(rename = "partial_fill_step")]
    pub partial_fill_step: String,
    #[serde(rename = "recent_fill_rate", skip_serializing_if = "Option::is_none")]
    pub recent_fill_rate: Option<String>,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "status")]
    pub status: RFQStatus,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "total_cost", skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<String>,
    #[serde(rename = "valid_until")]
    pub valid_until: serde_json::Number,
    #[serde(rename = "wallet")]
    pub wallet: Address,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PublicRfq {
    fn default() -> Self {
        Self {
            cancel_reason: Default::default(),
            creation_timestamp: 0.into(),
            fill_rate: Default::default(),
            filled_direction: Default::default(),
            filled_pct: Default::default(),
            last_update_timestamp: 0.into(),
            legs: Default::default(),
            partial_fill_step: Default::default(),
            recent_fill_rate: Default::default(),
            rfq_id: Default::default(),
            status: Default::default(),
            subaccount_id: 0.into(),
            total_cost: Default::default(),
            valid_until: 0.into(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicSendQuoteDebugRequest {
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "legs")]
    pub legs: Vec<PricedLegParamsAndResponse>,
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: Address,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PublicSendQuoteDebugRequest {
    fn default() -> Self {
        Self {
            direction: Default::default(),
            legs: Default::default(),
            max_fee: Default::default(),
            nonce: Default::default(),
            rfq_id: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PublicTradesResult {
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "trades")]
    pub trades: Vec<SettledTrade>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicVaultActionResponse {
    #[serde(rename = "curator_shares_minted")]
    pub curator_shares_minted: String,
    #[serde(rename = "event_ts")]
    pub event_ts: serde_json::Number,
    #[serde(rename = "event_type")]
    pub event_type: String,
    #[serde(rename = "holder")]
    pub holder: String,
    #[serde(rename = "management_shares_minted")]
    pub management_shares_minted: String,
    #[serde(rename = "nav")]
    pub nav: String,
    #[serde(rename = "new_high_water_mark")]
    pub new_high_water_mark: String,
    #[serde(rename = "old_high_water_mark")]
    pub old_high_water_mark: String,
    #[serde(rename = "operation_uuid")]
    pub operation_uuid: String,
    #[serde(rename = "performance_shares_minted")]
    pub performance_shares_minted: String,
    #[serde(rename = "protocol_shares_minted")]
    pub protocol_shares_minted: String,
    #[serde(rename = "share_price")]
    pub share_price: String,
    #[serde(rename = "shares_delta")]
    pub shares_delta: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "total_shares")]
    pub total_shares: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PublicVaultActionResponse {
    fn default() -> Self {
        Self {
            curator_shares_minted: Default::default(),
            event_ts: 0.into(),
            event_type: Default::default(),
            holder: Default::default(),
            management_shares_minted: Default::default(),
            nav: Default::default(),
            new_high_water_mark: Default::default(),
            old_high_water_mark: Default::default(),
            operation_uuid: Default::default(),
            performance_shares_minted: Default::default(),
            protocol_shares_minted: Default::default(),
            share_price: Default::default(),
            shares_delta: Default::default(),
            status: Default::default(),
            subaccount_id: 0.into(),
            total_shares: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicWithdrawDebugRequest {
    #[serde(rename = "amount_in_underlying")]
    pub amount_in_underlying: String,
    #[serde(rename = "asset_name")]
    pub asset_name: String,
    #[serde(rename = "force_batch")]
    pub force_batch: bool,
    #[serde(rename = "max_fee_usd")]
    pub max_fee_usd: String,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "recipient", default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PublicWithdrawDebugRequest {
    fn default() -> Self {
        Self {
            amount_in_underlying: Default::default(),
            asset_name: Default::default(),
            force_batch: Default::default(),
            max_fee_usd: Default::default(),
            nonce: Default::default(),
            recipient: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Quote {
    #[serde(rename = "batch_status", default, skip_serializing_if = "Option::is_none")]
    pub batch_status: Option<BatchStatus>,
    #[serde(rename = "cancel_reason")]
    pub cancel_reason: RFQCancelReason,
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: serde_json::Number,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "extra_fee")]
    pub extra_fee: String,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "fill_pct")]
    pub fill_pct: String,
    #[serde(rename = "is_transfer")]
    pub is_transfer: bool,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "last_update_timestamp")]
    pub last_update_timestamp: serde_json::Number,
    #[serde(rename = "legs")]
    pub legs: Vec<PricedLegParamsAndResponse>,
    #[serde(rename = "legs_hash")]
    pub legs_hash: String,
    #[serde(rename = "liquidity_role")]
    pub liquidity_role: LiquidityRole,
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "mmp")]
    pub mmp: bool,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "signature_expiry_sec")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "status")]
    pub status: RFQStatus,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for Quote {
    fn default() -> Self {
        Self {
            batch_status: Default::default(),
            cancel_reason: Default::default(),
            creation_timestamp: 0.into(),
            direction: Default::default(),
            extra_fee: Default::default(),
            fee: Default::default(),
            fill_pct: Default::default(),
            is_transfer: Default::default(),
            label: Default::default(),
            last_update_timestamp: 0.into(),
            legs: Default::default(),
            legs_hash: Default::default(),
            liquidity_role: Default::default(),
            max_fee: Default::default(),
            mmp: Default::default(),
            nonce: Default::default(),
            quote_id: Default::default(),
            rfq_id: Default::default(),
            signature_expiry_sec: 0.into(),
            status: Default::default(),
            subaccount_id: 0.into(),
            tx_hash: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct QuoteExecuteDebugResult {
    #[serde(rename = "action_hash")]
    pub action_hash: String,
    #[serde(rename = "encoded_data")]
    pub encoded_data: String,
    #[serde(rename = "encoded_data_hashed")]
    pub encoded_data_hashed: String,
    #[serde(rename = "encoded_legs")]
    pub encoded_legs: String,
    #[serde(rename = "legs_hash")]
    pub legs_hash: String,
    #[serde(rename = "typed_data_hash")]
    pub typed_data_hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QuoteExecuteResponse {
    #[serde(rename = "cancel_reason")]
    pub cancel_reason: RFQCancelReason,
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: serde_json::Number,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "extra_fee")]
    pub extra_fee: String,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "fill_pct")]
    pub fill_pct: String,
    #[serde(rename = "is_transfer")]
    pub is_transfer: bool,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "last_update_timestamp")]
    pub last_update_timestamp: serde_json::Number,
    #[serde(rename = "legs")]
    pub legs: Vec<PricedLegParamsAndResponse>,
    #[serde(rename = "legs_hash")]
    pub legs_hash: String,
    #[serde(rename = "liquidity_role")]
    pub liquidity_role: LiquidityRole,
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "mmp")]
    pub mmp: bool,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "rfq_filled_pct")]
    pub rfq_filled_pct: String,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "signature_expiry_sec")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "status")]
    pub status: RFQStatus,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for QuoteExecuteResponse {
    fn default() -> Self {
        Self {
            cancel_reason: Default::default(),
            creation_timestamp: 0.into(),
            direction: Default::default(),
            extra_fee: Default::default(),
            fee: Default::default(),
            fill_pct: Default::default(),
            is_transfer: Default::default(),
            label: Default::default(),
            last_update_timestamp: 0.into(),
            legs: Default::default(),
            legs_hash: Default::default(),
            liquidity_role: Default::default(),
            max_fee: Default::default(),
            mmp: Default::default(),
            nonce: Default::default(),
            quote_id: Default::default(),
            rfq_filled_pct: Default::default(),
            rfq_id: Default::default(),
            signature_expiry_sec: 0.into(),
            status: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct QuoteGetResponse {
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "quotes")]
    pub quotes: Vec<Quote>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct QuotePollResponse {
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "quotes")]
    pub quotes: Vec<PublicQuote>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct QuoteReplaceResponse {
    #[serde(rename = "cancelled_quote")]
    pub cancelled_quote: Quote,
    #[serde(rename = "create_quote_error", default, skip_serializing_if = "Option::is_none")]
    pub create_quote_error: Option<RPCError>,
    #[serde(rename = "quote", default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<Quote>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct QuoteSendDebugResult {
    #[serde(rename = "action_hash")]
    pub action_hash: String,
    #[serde(rename = "encoded_data")]
    pub encoded_data: String,
    #[serde(rename = "encoded_data_hashed")]
    pub encoded_data_hashed: String,
    #[serde(rename = "typed_data_hash")]
    pub typed_data_hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RFQGetResponse {
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "rfqs")]
    pub rfqs: Vec<Rfq>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RFQPollResponse {
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "rfqs")]
    pub rfqs: Vec<PublicRfq>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RPCError {
    #[serde(rename = "code")]
    pub code: serde_json::Number,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RPCError {
    fn default() -> Self {
        Self {
            code: 0.into(),
            data: Default::default(),
            message: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RateFeedDataResponse {
    #[serde(rename = "confidence")]
    pub confidence: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "deadline")]
    pub deadline: serde_json::Number,
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(rename = "rate")]
    pub rate: String,
    #[serde(rename = "signatures")]
    pub signatures: OracleSignatureDataResponse,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RateFeedDataResponse {
    fn default() -> Self {
        Self {
            confidence: Default::default(),
            currency: Default::default(),
            deadline: 0.into(),
            expiry: 0.into(),
            rate: Default::default(),
            signatures: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    #[serde(rename = "consumedPoints")]
    pub consumed_points: serde_json::Number,
    #[serde(rename = "isFirstInDuration")]
    pub is_first_in_duration: bool,
    #[serde(rename = "msBeforeNext")]
    pub ms_before_next: serde_json::Number,
    #[serde(rename = "remainingPoints")]
    pub remaining_points: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RateLimitInfo {
    fn default() -> Self {
        Self {
            consumed_points: 0.into(),
            is_first_in_duration: Default::default(),
            ms_before_next: 0.into(),
            remaining_points: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RateLimitResult {
    #[serde(rename = "remaining_connections", default, skip_serializing_if = "Option::is_none")]
    pub remaining_connections: Option<RateLimitInfo>,
    #[serde(rename = "remaining_matching")]
    pub remaining_matching: RateLimitInfo,
    #[serde(rename = "remaining_non_matching")]
    pub remaining_non_matching: RateLimitInfo,
    #[serde(rename = "remaining_per_endpoint")]
    pub remaining_per_endpoint: std::collections::BTreeMap<String,RateLimitInfo>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ReferralPerformanceByInstrumentType {
    #[serde(rename = "builder_fee")]
    pub builder_fee: String,
    #[serde(rename = "fee_reward")]
    pub fee_reward: String,
    #[serde(rename = "notional_volume")]
    pub notional_volume: String,
    #[serde(rename = "referred_fee")]
    pub referred_fee: String,
    #[serde(rename = "unique_traders_referred")]
    pub unique_traders_referred: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ReferralPerformanceByInstrumentType {
    fn default() -> Self {
        Self {
            builder_fee: Default::default(),
            fee_reward: Default::default(),
            notional_volume: Default::default(),
            referred_fee: Default::default(),
            unique_traders_referred: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Referrer {
    #[serde(rename = "receiving_wallet", default, skip_serializing_if = "Option::is_none")]
    pub receiving_wallet: Option<String>,
    #[serde(rename = "referral_code")]
    pub referral_code: String,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RegisterBridgeDepositParams {
    #[serde(rename = "bridge_token")]
    pub bridge_token: String,
    #[serde(rename = "bridge_token_decimals")]
    pub bridge_token_decimals: serde_json::Number,
    #[serde(rename = "deposit_address")]
    pub deposit_address: String,
    #[serde(rename = "expected_amount")]
    pub expected_amount: String,
    #[serde(rename = "provider_transfer_id", default, skip_serializing_if = "Option::is_none")]
    pub provider_transfer_id: Option<String>,
    #[serde(rename = "rail")]
    pub rail: String,
    #[serde(rename = "source_chain_id")]
    pub source_chain_id: serde_json::Number,
    #[serde(rename = "source_token")]
    pub source_token: String,
    #[serde(rename = "source_tx_hash")]
    pub source_tx_hash: String,
    #[serde(rename = "tool", default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RegisterBridgeDepositParams {
    fn default() -> Self {
        Self {
            bridge_token: Default::default(),
            bridge_token_decimals: 0.into(),
            deposit_address: Default::default(),
            expected_amount: Default::default(),
            provider_transfer_id: Default::default(),
            rail: Default::default(),
            source_chain_id: 0.into(),
            source_token: Default::default(),
            source_tx_hash: Default::default(),
            tool: Default::default(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RegisterDepositAddressParams {
    #[serde(rename = "manager_id", default, skip_serializing_if = "Option::is_none")]
    pub manager_id: Option<serde_json::Number>,
    #[serde(rename = "subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<serde_json::Number>,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RegisterDepositAddressResult {
    #[serde(rename = "deposit_address")]
    pub deposit_address: String,
    #[serde(rename = "manager_id", default, skip_serializing_if = "Option::is_none")]
    pub manager_id: Option<serde_json::Number>,
    #[serde(rename = "subaccount_id", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<serde_json::Number>,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RegisterWithdrawAddressParams {
    #[serde(rename = "chain_id")]
    pub chain_id: serde_json::Number,
    #[serde(rename = "recipient")]
    pub recipient: String,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RegisterWithdrawAddressParams {
    fn default() -> Self {
        Self {
            chain_id: 0.into(),
            recipient: Default::default(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RegisterWithdrawAddressResult {
    #[serde(rename = "chain_id")]
    pub chain_id: serde_json::Number,
    #[serde(rename = "recipient")]
    pub recipient: String,
    #[serde(rename = "recipient_display")]
    pub recipient_display: String,
    #[serde(rename = "routes")]
    pub routes: Vec<ForwardedToken>,
    #[serde(rename = "withdraw_address")]
    pub withdraw_address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RegisterWithdrawAddressResult {
    fn default() -> Self {
        Self {
            chain_id: 0.into(),
            recipient: Default::default(),
            recipient_display: Default::default(),
            routes: Default::default(),
            withdraw_address: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RejectDepositRequestRequest {
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "request_id")]
    pub request_id: VaultRequestId,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RejectDepositRequestRequest {
    fn default() -> Self {
        Self {
            reason: Default::default(),
            request_id: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ReplaceOrderRequest {
    #[serde(rename = "algo_duration_sec", default, skip_serializing_if = "Option::is_none")]
    pub algo_duration_sec: Option<serde_json::Number>,
    #[serde(rename = "algo_num_slices", default, skip_serializing_if = "Option::is_none")]
    pub algo_num_slices: Option<serde_json::Number>,
    #[serde(rename = "algo_type", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<AlgoType>,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "client", default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "expected_filled_amount", default, skip_serializing_if = "Option::is_none")]
    pub expected_filled_amount: Option<String>,
    #[serde(rename = "extra_fee", default, skip_serializing_if = "Option::is_none")]
    pub extra_fee: Option<String>,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "is_atomic_signing", default, skip_serializing_if = "Option::is_none")]
    pub is_atomic_signing: Option<bool>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "limit_price")]
    pub limit_price: String,
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "mmp", default, skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "nonce_to_cancel", default, skip_serializing_if = "Option::is_none")]
    pub nonce_to_cancel: Option<String>,
    #[serde(rename = "order_id_to_cancel", default, skip_serializing_if = "Option::is_none")]
    pub order_id_to_cancel: Option<String>,
    #[serde(rename = "order_type", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "referral_code", default, skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(rename = "reject_post_only", default, skip_serializing_if = "Option::is_none")]
    pub reject_post_only: Option<bool>,
    #[serde(rename = "reject_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub reject_timestamp: Option<serde_json::Number>,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "time_in_force", default, skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(rename = "trigger_price", default, skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "trigger_price_type", default, skip_serializing_if = "Option::is_none")]
    pub trigger_price_type: Option<TriggerPriceType>,
    #[serde(rename = "trigger_type", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<TriggerType>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ReplaceOrderRequest {
    fn default() -> Self {
        Self {
            algo_duration_sec: Default::default(),
            algo_num_slices: Default::default(),
            algo_type: Default::default(),
            amount: Default::default(),
            client: Default::default(),
            direction: Default::default(),
            expected_filled_amount: Default::default(),
            extra_fee: Default::default(),
            instrument_name: Default::default(),
            is_atomic_signing: Default::default(),
            label: Default::default(),
            limit_price: Default::default(),
            max_fee: Default::default(),
            mmp: Default::default(),
            nonce: Default::default(),
            nonce_to_cancel: Default::default(),
            order_id_to_cancel: Default::default(),
            order_type: Default::default(),
            reduce_only: Default::default(),
            referral_code: Default::default(),
            reject_post_only: Default::default(),
            reject_timestamp: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            time_in_force: Default::default(),
            trigger_price: Default::default(),
            trigger_price_type: Default::default(),
            trigger_type: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReplaceOrderResponse {
    #[serde(rename = "cancelled_order")]
    pub cancelled_order: Order,
    #[serde(rename = "create_order_error", default, skip_serializing_if = "Option::is_none")]
    pub create_order_error: Option<RPCError>,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(rename = "trades", default, skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<Trade>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ReplaceQuoteRequest {
    #[serde(rename = "client", default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "extra_fee", default, skip_serializing_if = "Option::is_none")]
    pub extra_fee: Option<String>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "legs")]
    pub legs: Vec<PricedLegParamsAndResponse>,
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "mmp", default, skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "nonce_to_cancel", default, skip_serializing_if = "Option::is_none")]
    pub nonce_to_cancel: Option<String>,
    #[serde(rename = "quote_id_to_cancel", default, skip_serializing_if = "Option::is_none")]
    pub quote_id_to_cancel: Option<String>,
    #[serde(rename = "referral_code", default, skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: Address,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ReplaceQuoteRequest {
    fn default() -> Self {
        Self {
            client: Default::default(),
            direction: Default::default(),
            extra_fee: Default::default(),
            label: Default::default(),
            legs: Default::default(),
            max_fee: Default::default(),
            mmp: Default::default(),
            nonce: Default::default(),
            nonce_to_cancel: Default::default(),
            quote_id_to_cancel: Default::default(),
            referral_code: Default::default(),
            rfq_id: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestVaultDepositRequest {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "deposit_spot_asset")]
    pub deposit_spot_asset: Address,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: Address,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "vault_subaccount_id")]
    pub vault_subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RequestVaultDepositRequest {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            deposit_spot_asset: Default::default(),
            nonce: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            vault_subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestVaultWithdrawRequest {
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "shares_to_burn")]
    pub shares_to_burn: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: Address,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "vault_subaccount_id")]
    pub vault_subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RequestVaultWithdrawRequest {
    fn default() -> Self {
        Self {
            nonce: Default::default(),
            shares_to_burn: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            vault_subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Rfq {
    #[serde(rename = "ask_total_cost", skip_serializing_if = "Option::is_none")]
    pub ask_total_cost: Option<String>,
    #[serde(rename = "bid_total_cost", skip_serializing_if = "Option::is_none")]
    pub bid_total_cost: Option<String>,
    #[serde(rename = "cancel_reason")]
    pub cancel_reason: RFQCancelReason,
    #[serde(rename = "counterparties", default, skip_serializing_if = "Option::is_none")]
    pub counterparties: Option<Vec<Address>>,
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: serde_json::Number,
    #[serde(rename = "filled_direction", default, skip_serializing_if = "Option::is_none")]
    pub filled_direction: Option<Direction>,
    #[serde(rename = "filled_pct")]
    pub filled_pct: String,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "last_update_timestamp")]
    pub last_update_timestamp: serde_json::Number,
    #[serde(rename = "legs")]
    pub legs: Vec<LegUnpricedParams>,
    #[serde(rename = "mark_total_cost", skip_serializing_if = "Option::is_none")]
    pub mark_total_cost: Option<String>,
    #[serde(rename = "max_total_cost", skip_serializing_if = "Option::is_none")]
    pub max_total_cost: Option<String>,
    #[serde(rename = "min_total_cost", skip_serializing_if = "Option::is_none")]
    pub min_total_cost: Option<String>,
    #[serde(rename = "partial_fill_step")]
    pub partial_fill_step: String,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "status")]
    pub status: RFQStatus,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "total_cost", skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<String>,
    #[serde(rename = "valid_until")]
    pub valid_until: serde_json::Number,
    #[serde(rename = "wallet")]
    pub wallet: Address,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for Rfq {
    fn default() -> Self {
        Self {
            ask_total_cost: Default::default(),
            bid_total_cost: Default::default(),
            cancel_reason: Default::default(),
            counterparties: Default::default(),
            creation_timestamp: 0.into(),
            filled_direction: Default::default(),
            filled_pct: Default::default(),
            label: Default::default(),
            last_update_timestamp: 0.into(),
            legs: Default::default(),
            mark_total_cost: Default::default(),
            max_total_cost: Default::default(),
            min_total_cost: Default::default(),
            partial_fill_step: Default::default(),
            rfq_id: Default::default(),
            status: Default::default(),
            subaccount_id: 0.into(),
            total_cost: Default::default(),
            valid_until: 0.into(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RfqGetBestQuoteRequest {
    #[serde(rename = "client", default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(rename = "direction", default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(rename = "extra_fee", default, skip_serializing_if = "Option::is_none")]
    pub extra_fee: Option<String>,
    #[serde(rename = "legs", default, skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<LegUnpricedParams>>,
    #[serde(rename = "rfq_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RfqGetBestQuoteRequest {
    fn default() -> Self {
        Self {
            client: Default::default(),
            direction: Default::default(),
            extra_fee: Default::default(),
            legs: Default::default(),
            rfq_id: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqGetBestQuoteResponse {
    #[serde(rename = "best_quote", default, skip_serializing_if = "Option::is_none")]
    pub best_quote: Option<PublicQuote>,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "down_liquidation_price", skip_serializing_if = "Option::is_none")]
    pub down_liquidation_price: Option<String>,
    #[serde(rename = "estimated_fee")]
    pub estimated_fee: String,
    #[serde(rename = "estimated_realized_pnl")]
    pub estimated_realized_pnl: String,
    #[serde(rename = "estimated_realized_pnl_excl_fees")]
    pub estimated_realized_pnl_excl_fees: String,
    #[serde(rename = "estimated_total_cost")]
    pub estimated_total_cost: String,
    #[serde(rename = "filled_pct")]
    pub filled_pct: String,
    #[serde(rename = "invalid_reason", default, skip_serializing_if = "Option::is_none")]
    pub invalid_reason: Option<String>,
    #[serde(rename = "is_valid")]
    pub is_valid: bool,
    #[serde(rename = "orderbook_total_cost", skip_serializing_if = "Option::is_none")]
    pub orderbook_total_cost: Option<String>,
    #[serde(rename = "post_initial_margin")]
    pub post_initial_margin: String,
    #[serde(rename = "post_liquidation_price", skip_serializing_if = "Option::is_none")]
    pub post_liquidation_price: Option<String>,
    #[serde(rename = "pre_initial_margin")]
    pub pre_initial_margin: String,
    #[serde(rename = "suggested_max_fee")]
    pub suggested_max_fee: String,
    #[serde(rename = "up_liquidation_price", skip_serializing_if = "Option::is_none")]
    pub up_liquidation_price: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqLegResponse {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset_address")]
    pub asset_address: String,
    #[serde(rename = "asset_sub_id")]
    pub asset_sub_id: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqMakerActionDataResponse {
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "trades")]
    pub trades: Vec<RfqLegResponse>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqTakerActionDataResponse {
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "order_hash")]
    pub order_hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RiskUniverse {
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "managers")]
    pub managers: Vec<RiskUniverseManager>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "risk_universe_id")]
    pub risk_universe_id: serde_json::Number,
    #[serde(rename = "security_module")]
    pub security_module: SecurityModuleDetails,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RiskUniverse {
    fn default() -> Self {
        Self {
            description: Default::default(),
            managers: Default::default(),
            name: Default::default(),
            risk_universe_id: 0.into(),
            security_module: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RiskUniverseManager {
    #[serde(rename = "collaterals")]
    pub collaterals: Vec<ManagerCollateral>,
    #[serde(rename = "instruments")]
    pub instruments: Vec<String>,
    #[serde(rename = "manager_id")]
    pub manager_id: serde_json::Number,
    #[serde(rename = "margin_type")]
    pub margin_type: MarginType,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RiskUniverseManager {
    fn default() -> Self {
        Self {
            collaterals: Default::default(),
            instruments: Default::default(),
            manager_id: 0.into(),
            margin_type: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    #[serde(rename = "coverage_score")]
    pub coverage_score: String,
    #[serde(rename = "holder_boost")]
    pub holder_boost: String,
    #[serde(rename = "quality_score")]
    pub quality_score: String,
    #[serde(rename = "total_score")]
    pub total_score: String,
    #[serde(rename = "volume")]
    pub volume: String,
    #[serde(rename = "volume_multiplier")]
    pub volume_multiplier: String,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SecurityModuleDetails {
    #[serde(rename = "cash_asset")]
    pub cash_asset: String,
    #[serde(rename = "cash_currency")]
    pub cash_currency: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SecurityModuleDetails {
    fn default() -> Self {
        Self {
            cash_asset: Default::default(),
            cash_currency: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SendQuoteRequest {
    #[serde(rename = "client", default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "extra_fee", default, skip_serializing_if = "Option::is_none")]
    pub extra_fee: Option<String>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "legs")]
    pub legs: Vec<PricedLegParamsAndResponse>,
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "mmp", default, skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "referral_code", default, skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: Address,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SendQuoteRequest {
    fn default() -> Self {
        Self {
            client: Default::default(),
            direction: Default::default(),
            extra_fee: Default::default(),
            label: Default::default(),
            legs: Default::default(),
            max_fee: Default::default(),
            mmp: Default::default(),
            nonce: Default::default(),
            referral_code: Default::default(),
            rfq_id: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SendRfqRequest {
    #[serde(rename = "client", default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(rename = "counterparties", default, skip_serializing_if = "Option::is_none")]
    pub counterparties: Option<Vec<String>>,
    #[serde(rename = "extra_fee", default, skip_serializing_if = "Option::is_none")]
    pub extra_fee: Option<String>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "legs")]
    pub legs: Vec<LegUnpricedParams>,
    #[serde(rename = "max_total_cost", default, skip_serializing_if = "Option::is_none")]
    pub max_total_cost: Option<String>,
    #[serde(rename = "min_total_cost", default, skip_serializing_if = "Option::is_none")]
    pub min_total_cost: Option<String>,
    #[serde(rename = "partial_fill_step", default, skip_serializing_if = "Option::is_none")]
    pub partial_fill_step: Option<String>,
    #[serde(rename = "referral_code", default, skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SendRfqRequest {
    fn default() -> Self {
        Self {
            client: Default::default(),
            counterparties: Default::default(),
            extra_fee: Default::default(),
            label: Default::default(),
            legs: Default::default(),
            max_total_cost: Default::default(),
            min_total_cost: Default::default(),
            partial_fill_step: Default::default(),
            referral_code: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SessionKey {
    #[serde(rename = "expiry_sec")]
    pub expiry_sec: serde_json::Number,
    #[serde(rename = "ip_whitelist")]
    pub ip_whitelist: Vec<String>,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "offchain_scopes")]
    pub offchain_scopes: Vec<String>,
    #[serde(rename = "protocol_scopes")]
    pub protocol_scopes: Vec<String>,
    #[serde(rename = "public_session_key")]
    pub public_session_key: String,
    #[serde(rename = "registered_sec")]
    pub registered_sec: serde_json::Number,
    #[serde(rename = "subaccount_ids")]
    pub subaccount_ids: Vec<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SessionKey {
    fn default() -> Self {
        Self {
            expiry_sec: 0.into(),
            ip_whitelist: Default::default(),
            label: Default::default(),
            offchain_scopes: Default::default(),
            protocol_scopes: Default::default(),
            public_session_key: Default::default(),
            registered_sec: 0.into(),
            subaccount_ids: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SessionKeysRequest {
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SetMmpConfigRequest {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "mmp_amount_limit", default, skip_serializing_if = "Option::is_none")]
    pub mmp_amount_limit: Option<String>,
    #[serde(rename = "mmp_delta_limit", default, skip_serializing_if = "Option::is_none")]
    pub mmp_delta_limit: Option<String>,
    #[serde(rename = "mmp_frozen_time")]
    pub mmp_frozen_time: serde_json::Number,
    #[serde(rename = "mmp_interval")]
    pub mmp_interval: serde_json::Number,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SetMmpConfigRequest {
    fn default() -> Self {
        Self {
            currency: Default::default(),
            mmp_amount_limit: Default::default(),
            mmp_delta_limit: Default::default(),
            mmp_frozen_time: 0.into(),
            mmp_interval: 0.into(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SetMmpConfigResponse {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "mmp_amount_limit")]
    pub mmp_amount_limit: String,
    #[serde(rename = "mmp_delta_limit")]
    pub mmp_delta_limit: String,
    #[serde(rename = "mmp_frozen_time")]
    pub mmp_frozen_time: serde_json::Number,
    #[serde(rename = "mmp_interval")]
    pub mmp_interval: serde_json::Number,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SetMmpConfigResponse {
    fn default() -> Self {
        Self {
            currency: Default::default(),
            mmp_amount_limit: Default::default(),
            mmp_delta_limit: Default::default(),
            mmp_frozen_time: 0.into(),
            mmp_interval: 0.into(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SetSessionKeyActionDataResponse {
    #[serde(rename = "expiry_sec")]
    pub expiry_sec: serde_json::Number,
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    #[serde(rename = "session_key")]
    pub session_key: String,
    #[serde(rename = "subaccounts")]
    pub subaccounts: Vec<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SetSessionKeyActionDataResponse {
    fn default() -> Self {
        Self {
            expiry_sec: 0.into(),
            scopes: Default::default(),
            session_key: Default::default(),
            subaccounts: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SetSessionKeyActionInputData {
    #[serde(rename = "data")]
    pub data: SetSessionKeyActionDataResponse,
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "signer")]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SetSessionKeyActionInputData {
    fn default() -> Self {
        Self {
            data: Default::default(),
            expiry: 0.into(),
            module: Default::default(),
            nonce: Default::default(),
            owner: Default::default(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetSessionKeyDebugResponse {
    #[serde(rename = "action_hash")]
    pub action_hash: String,
    #[serde(rename = "action_typehash")]
    pub action_typehash: String,
    #[serde(rename = "domain_separator")]
    pub domain_separator: String,
    #[serde(rename = "encoded_data")]
    pub encoded_data: String,
    #[serde(rename = "encoded_data_hashed")]
    pub encoded_data_hashed: String,
    #[serde(rename = "expected_signer")]
    pub expected_signer: String,
    #[serde(rename = "input_data")]
    pub input_data: SetSessionKeyActionInputData,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "recovered_signer", default, skip_serializing_if = "Option::is_none")]
    pub recovered_signer: Option<String>,
    #[serde(rename = "typed_data_hash")]
    pub typed_data_hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SetSessionKeyRequest {
    #[serde(rename = "expiry_sec")]
    pub expiry_sec: serde_json::Number,
    #[serde(rename = "ip_whitelist", default, skip_serializing_if = "Option::is_none")]
    pub ip_whitelist: Option<Vec<String>>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "offchain_scopes")]
    pub offchain_scopes: Vec<String>,
    #[serde(rename = "protocol_scopes")]
    pub protocol_scopes: Vec<String>,
    #[serde(rename = "public_session_key")]
    pub public_session_key: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: String,
    #[serde(rename = "subaccount_ids", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_ids: Option<Vec<serde_json::Number>>,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SetSessionKeyRequest {
    fn default() -> Self {
        Self {
            expiry_sec: 0.into(),
            ip_whitelist: Default::default(),
            label: Default::default(),
            nonce: Default::default(),
            offchain_scopes: Default::default(),
            protocol_scopes: Default::default(),
            public_session_key: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_ids: Default::default(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SettledTrade {
    #[serde(rename = "batch_status", default, skip_serializing_if = "Option::is_none")]
    pub batch_status: Option<BatchStatus>,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "expected_rebate")]
    pub expected_rebate: String,
    #[serde(rename = "extra_fee")]
    pub extra_fee: String,
    #[serde(rename = "index_price")]
    pub index_price: String,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "liquidity_role")]
    pub liquidity_role: LiquidityRole,
    #[serde(rename = "mark_price")]
    pub mark_price: String,
    #[serde(rename = "quote_id", skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(rename = "realized_pnl")]
    pub realized_pnl: String,
    #[serde(rename = "realized_pnl_excl_fees")]
    pub realized_pnl_excl_fees: String,
    #[serde(rename = "rfq_id", skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "trade_amount")]
    pub trade_amount: String,
    #[serde(rename = "trade_fee")]
    pub trade_fee: String,
    #[serde(rename = "trade_id")]
    pub trade_id: String,
    #[serde(rename = "trade_price")]
    pub trade_price: String,
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SettledTrade {
    fn default() -> Self {
        Self {
            batch_status: Default::default(),
            direction: Default::default(),
            expected_rebate: Default::default(),
            extra_fee: Default::default(),
            index_price: Default::default(),
            instrument_name: Default::default(),
            liquidity_role: Default::default(),
            mark_price: Default::default(),
            quote_id: Default::default(),
            realized_pnl: Default::default(),
            realized_pnl_excl_fees: Default::default(),
            rfq_id: Default::default(),
            subaccount_id: 0.into(),
            timestamp: 0.into(),
            trade_amount: Default::default(),
            trade_fee: Default::default(),
            trade_id: Default::default(),
            trade_price: Default::default(),
            tx_hash: Default::default(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SignedAction {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "signature")]
    pub signature: Vec<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SignedTransferQuoteRequest {
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "legs")]
    pub legs: Vec<PricedLegParamsAndResponse>,
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: Address,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SignedTransferQuoteRequest {
    fn default() -> Self {
        Self {
            direction: Default::default(),
            legs: Default::default(),
            max_fee: Default::default(),
            nonce: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SimulatedCollateralRequest {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset_name")]
    pub asset_name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SimulatedPositionRequest {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "entry_price", default, skip_serializing_if = "Option::is_none")]
    pub entry_price: Option<String>,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SpotAssetEntry {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "erc20")]
    pub erc20: Erc20Details,
    #[serde(rename = "min_deposit_usd")]
    pub min_deposit_usd: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "universes")]
    pub universes: Vec<SpotUniverse>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpotFeedDataResponse {
    #[serde(rename = "confidence")]
    pub confidence: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "deadline")]
    pub deadline: serde_json::Number,
    #[serde(rename = "feed_source_type", default, skip_serializing_if = "Option::is_none")]
    pub feed_source_type: Option<String>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "signatures")]
    pub signatures: OracleSignatureDataResponse,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SpotFeedDataResponse {
    fn default() -> Self {
        Self {
            confidence: Default::default(),
            currency: Default::default(),
            deadline: 0.into(),
            feed_source_type: Default::default(),
            price: Default::default(),
            signatures: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpotPublicDetails {
    #[serde(rename = "borrow_index")]
    pub borrow_index: String,
    #[serde(rename = "decimals")]
    pub decimals: serde_json::Number,
    #[serde(rename = "supply_index")]
    pub supply_index: String,
    #[serde(rename = "underlying_erc20_address")]
    pub underlying_erc20_address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SpotPublicDetails {
    fn default() -> Self {
        Self {
            borrow_index: Default::default(),
            decimals: 0.into(),
            supply_index: Default::default(),
            underlying_erc20_address: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpotUniverse {
    #[serde(rename = "lending", default, skip_serializing_if = "Option::is_none")]
    pub lending: Option<LendingDetails>,
    #[serde(rename = "oi")]
    pub oi: OpenInterestStats,
    #[serde(rename = "pm2_im_discount")]
    pub pm2_im_discount: String,
    #[serde(rename = "pm2_mm_discount")]
    pub pm2_mm_discount: String,
    #[serde(rename = "risk_universe_id")]
    pub risk_universe_id: serde_json::Number,
    #[serde(rename = "srm_im_discount")]
    pub srm_im_discount: String,
    #[serde(rename = "srm_mm_discount")]
    pub srm_mm_discount: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SpotUniverse {
    fn default() -> Self {
        Self {
            lending: Default::default(),
            oi: Default::default(),
            pm2_im_discount: Default::default(),
            pm2_mm_discount: Default::default(),
            risk_universe_id: 0.into(),
            srm_im_discount: Default::default(),
            srm_mm_discount: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SrmPerpMarginRequirements {
    #[serde(rename = "im_perp_req")]
    pub im_perp_req: String,
    #[serde(rename = "max_leverage")]
    pub max_leverage: String,
    #[serde(rename = "mm_perp_req")]
    pub mm_perp_req: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Subaccount {
    #[serde(rename = "collaterals")]
    pub collaterals: Vec<Collateral>,
    #[serde(rename = "collaterals_initial_margin")]
    pub collaterals_initial_margin: String,
    #[serde(rename = "collaterals_maintenance_margin")]
    pub collaterals_maintenance_margin: String,
    #[serde(rename = "collaterals_value")]
    pub collaterals_value: String,
    #[serde(rename = "currency")]
    pub currency: Vec<String>,
    #[serde(rename = "failed_to_fetch")]
    pub failed_to_fetch: bool,
    #[serde(rename = "initial_margin")]
    pub initial_margin: String,
    #[serde(rename = "is_under_liquidation")]
    pub is_under_liquidation: bool,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "maintenance_margin")]
    pub maintenance_margin: String,
    #[serde(rename = "manager_id")]
    pub manager_id: serde_json::Number,
    #[serde(rename = "margin_type")]
    pub margin_type: String,
    #[serde(rename = "mm_credits")]
    pub mm_credits: String,
    #[serde(rename = "open_orders")]
    pub open_orders: Vec<Order>,
    #[serde(rename = "open_orders_margin")]
    pub open_orders_margin: String,
    #[serde(rename = "positions")]
    pub positions: Vec<Position>,
    #[serde(rename = "positions_initial_margin")]
    pub positions_initial_margin: String,
    #[serde(rename = "positions_maintenance_margin")]
    pub positions_maintenance_margin: String,
    #[serde(rename = "positions_value")]
    pub positions_value: String,
    #[serde(rename = "projected_margin_change")]
    pub projected_margin_change: String,
    #[serde(rename = "risk_universe_id")]
    pub risk_universe_id: serde_json::Number,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "subaccount_value")]
    pub subaccount_value: String,
    #[serde(rename = "vault_deposit_holds")]
    pub vault_deposit_holds: Vec<VaultDepositHold>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for Subaccount {
    fn default() -> Self {
        Self {
            collaterals: Default::default(),
            collaterals_initial_margin: Default::default(),
            collaterals_maintenance_margin: Default::default(),
            collaterals_value: Default::default(),
            currency: Default::default(),
            failed_to_fetch: Default::default(),
            initial_margin: Default::default(),
            is_under_liquidation: Default::default(),
            label: Default::default(),
            maintenance_margin: Default::default(),
            manager_id: 0.into(),
            margin_type: Default::default(),
            mm_credits: Default::default(),
            open_orders: Default::default(),
            open_orders_margin: Default::default(),
            positions: Default::default(),
            positions_initial_margin: Default::default(),
            positions_maintenance_margin: Default::default(),
            positions_value: Default::default(),
            projected_margin_change: Default::default(),
            risk_universe_id: 0.into(),
            subaccount_id: 0.into(),
            subaccount_value: Default::default(),
            vault_deposit_holds: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SubaccountValueEntry {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "initial_margin")]
    pub initial_margin: String,
    #[serde(rename = "maintenance_margin")]
    pub maintenance_margin: String,
    #[serde(rename = "margin_type")]
    pub margin_type: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "subaccount_value")]
    pub subaccount_value: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SubaccountValueEntry {
    fn default() -> Self {
        Self {
            currency: Default::default(),
            initial_margin: Default::default(),
            maintenance_margin: Default::default(),
            margin_type: Default::default(),
            subaccount_id: 0.into(),
            subaccount_value: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SubaccountValueHistoryResult {
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "subaccount_value_history")]
    pub subaccount_value_history: Vec<SubaccountValueEntry>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TickerSlimSnapshot {
    #[serde(rename = "A")]
    pub a: String,
    #[serde(rename = "B")]
    pub b: String,
    #[serde(rename = "I")]
    pub i: String,
    #[serde(rename = "M")]
    pub m: String,
    #[serde(rename = "a")]
    pub a_value: String,
    #[serde(rename = "b")]
    pub b_value: String,
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub f: Option<String>,
    #[serde(rename = "maxp")]
    pub maxp: String,
    #[serde(rename = "minp")]
    pub minp: String,
    #[serde(rename = "option_pricing", default, skip_serializing_if = "Option::is_none")]
    pub option_pricing: Option<OptionPricing>,
    #[serde(rename = "stats")]
    pub stats: DailyTradingStatistics,
    #[serde(rename = "t")]
    pub t: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for TickerSlimSnapshot {
    fn default() -> Self {
        Self {
            a: Default::default(),
            b: Default::default(),
            i: Default::default(),
            m: Default::default(),
            a_value: Default::default(),
            b_value: Default::default(),
            f: Default::default(),
            maxp: Default::default(),
            minp: Default::default(),
            option_pricing: Default::default(),
            stats: Default::default(),
            t: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Trade {
    #[serde(rename = "batch_status", default, skip_serializing_if = "Option::is_none")]
    pub batch_status: Option<BatchStatus>,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "expected_rebate")]
    pub expected_rebate: String,
    #[serde(rename = "extra_fee")]
    pub extra_fee: String,
    #[serde(rename = "index_price")]
    pub index_price: String,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "is_transfer")]
    pub is_transfer: bool,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "liquidity_role")]
    pub liquidity_role: LiquidityRole,
    #[serde(rename = "mark_price")]
    pub mark_price: String,
    #[serde(rename = "op_uuid")]
    pub op_uuid: String,
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "quote_id", skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(rename = "realized_pnl")]
    pub realized_pnl: String,
    #[serde(rename = "realized_pnl_excl_fees")]
    pub realized_pnl_excl_fees: String,
    #[serde(rename = "rfq_id", skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "trade_amount")]
    pub trade_amount: String,
    #[serde(rename = "trade_fee")]
    pub trade_fee: String,
    #[serde(rename = "trade_id")]
    pub trade_id: String,
    #[serde(rename = "trade_price")]
    pub trade_price: String,
    #[serde(rename = "tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for Trade {
    fn default() -> Self {
        Self {
            batch_status: Default::default(),
            direction: Default::default(),
            expected_rebate: Default::default(),
            extra_fee: Default::default(),
            index_price: Default::default(),
            instrument_name: Default::default(),
            is_transfer: Default::default(),
            label: Default::default(),
            liquidity_role: Default::default(),
            mark_price: Default::default(),
            op_uuid: Default::default(),
            order_id: Default::default(),
            quote_id: Default::default(),
            realized_pnl: Default::default(),
            realized_pnl_excl_fees: Default::default(),
            rfq_id: Default::default(),
            subaccount_id: 0.into(),
            timestamp: 0.into(),
            trade_amount: Default::default(),
            trade_fee: Default::default(),
            trade_id: Default::default(),
            trade_price: Default::default(),
            tx_hash: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TradeHistoryResponse {
    #[serde(rename = "batch_status", default, skip_serializing_if = "Option::is_none")]
    pub batch_status: Option<BatchStatus>,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "expected_rebate")]
    pub expected_rebate: String,
    #[serde(rename = "extra_fee")]
    pub extra_fee: String,
    #[serde(rename = "index_price")]
    pub index_price: String,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "is_transfer")]
    pub is_transfer: bool,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "liquidity_role")]
    pub liquidity_role: LiquidityRole,
    #[serde(rename = "mark_price")]
    pub mark_price: String,
    #[serde(rename = "op_uuid")]
    pub op_uuid: String,
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "quote_id", skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(rename = "realized_pnl")]
    pub realized_pnl: String,
    #[serde(rename = "realized_pnl_excl_fees")]
    pub realized_pnl_excl_fees: String,
    #[serde(rename = "rfq_id", skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "trade_amount")]
    pub trade_amount: String,
    #[serde(rename = "trade_fee")]
    pub trade_fee: String,
    #[serde(rename = "trade_id")]
    pub trade_id: String,
    #[serde(rename = "trade_price")]
    pub trade_price: String,
    #[serde(rename = "tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for TradeHistoryResponse {
    fn default() -> Self {
        Self {
            batch_status: Default::default(),
            direction: Default::default(),
            expected_rebate: Default::default(),
            extra_fee: Default::default(),
            index_price: Default::default(),
            instrument_name: Default::default(),
            is_transfer: Default::default(),
            label: Default::default(),
            liquidity_role: Default::default(),
            mark_price: Default::default(),
            op_uuid: Default::default(),
            order_id: Default::default(),
            quote_id: Default::default(),
            realized_pnl: Default::default(),
            realized_pnl_excl_fees: Default::default(),
            rfq_id: Default::default(),
            subaccount_id: 0.into(),
            timestamp: 0.into(),
            trade_amount: Default::default(),
            trade_fee: Default::default(),
            trade_id: Default::default(),
            trade_price: Default::default(),
            tx_hash: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TradingviewCandle {
    #[serde(rename = "close_price")]
    pub close_price: String,
    #[serde(rename = "high_price")]
    pub high_price: String,
    #[serde(rename = "low_price")]
    pub low_price: String,
    #[serde(rename = "open_price")]
    pub open_price: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "timestamp_bucket")]
    pub timestamp_bucket: serde_json::Number,
    #[serde(rename = "volume_contracts")]
    pub volume_contracts: String,
    #[serde(rename = "volume_usd")]
    pub volume_usd: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for TradingviewCandle {
    fn default() -> Self {
        Self {
            close_price: Default::default(),
            high_price: Default::default(),
            low_price: Default::default(),
            open_price: Default::default(),
            timestamp: 0.into(),
            timestamp_bucket: 0.into(),
            volume_contracts: Default::default(),
            volume_usd: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TransferActionDataResponse {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "max_fee_usd")]
    pub max_fee_usd: String,
    #[serde(rename = "new_subaccount_manager")]
    pub new_subaccount_manager: serde_json::Number,
    #[serde(rename = "sub_id")]
    pub sub_id: String,
    #[serde(rename = "to_subaccount_id")]
    pub to_subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for TransferActionDataResponse {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            asset: Default::default(),
            max_fee_usd: Default::default(),
            new_subaccount_manager: 0.into(),
            sub_id: Default::default(),
            to_subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TransferEntry {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "batch_status")]
    pub batch_status: BatchStatus,
    #[serde(rename = "batch_uuid")]
    pub batch_uuid: String,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "from_subaccount_id")]
    pub from_subaccount_id: serde_json::Number,
    #[serde(rename = "from_wallet")]
    pub from_wallet: String,
    #[serde(rename = "is_outgoing")]
    pub is_outgoing: bool,
    #[serde(rename = "operation_id")]
    pub operation_id: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "to_subaccount_id")]
    pub to_subaccount_id: serde_json::Number,
    #[serde(rename = "to_wallet")]
    pub to_wallet: String,
    #[serde(rename = "tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for TransferEntry {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            asset: Default::default(),
            batch_status: Default::default(),
            batch_uuid: Default::default(),
            fee: Default::default(),
            from_subaccount_id: 0.into(),
            from_wallet: Default::default(),
            is_outgoing: Default::default(),
            operation_id: Default::default(),
            timestamp: 0.into(),
            to_subaccount_id: 0.into(),
            to_wallet: Default::default(),
            tx_hash: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferHistoryResult {
    #[serde(rename = "transfers")]
    pub transfers: Vec<TransferEntry>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferPositionsDebugResponse {
    #[serde(rename = "maker_result")]
    pub maker_result: TransferPositionsMakerDebugResponse,
    #[serde(rename = "taker_result")]
    pub taker_result: TransferPositionsTakerDebugResponse,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TransferPositionsMakerActionInputData {
    #[serde(rename = "data")]
    pub data: RfqMakerActionDataResponse,
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "signer")]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for TransferPositionsMakerActionInputData {
    fn default() -> Self {
        Self {
            data: Default::default(),
            expiry: 0.into(),
            module: Default::default(),
            nonce: Default::default(),
            owner: Default::default(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferPositionsMakerDebugResponse {
    #[serde(rename = "action_hash")]
    pub action_hash: String,
    #[serde(rename = "action_typehash")]
    pub action_typehash: String,
    #[serde(rename = "domain_separator")]
    pub domain_separator: String,
    #[serde(rename = "encoded_data")]
    pub encoded_data: String,
    #[serde(rename = "encoded_data_hashed")]
    pub encoded_data_hashed: String,
    #[serde(rename = "expected_signer")]
    pub expected_signer: String,
    #[serde(rename = "input_data")]
    pub input_data: TransferPositionsMakerActionInputData,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "recovered_signer", default, skip_serializing_if = "Option::is_none")]
    pub recovered_signer: Option<String>,
    #[serde(rename = "typed_data_hash")]
    pub typed_data_hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferPositionsRequest {
    #[serde(rename = "maker_params")]
    pub maker_params: SignedTransferQuoteRequest,
    #[serde(rename = "taker_params")]
    pub taker_params: SignedTransferQuoteRequest,
    #[serde(rename = "wallet")]
    pub wallet: Address,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferPositionsResponse {
    #[serde(rename = "maker_quote")]
    pub maker_quote: Quote,
    #[serde(rename = "taker_quote")]
    pub taker_quote: Quote,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TransferPositionsTakerActionInputData {
    #[serde(rename = "data")]
    pub data: RfqTakerActionDataResponse,
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "signer")]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for TransferPositionsTakerActionInputData {
    fn default() -> Self {
        Self {
            data: Default::default(),
            expiry: 0.into(),
            module: Default::default(),
            nonce: Default::default(),
            owner: Default::default(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferPositionsTakerDebugResponse {
    #[serde(rename = "action_hash")]
    pub action_hash: String,
    #[serde(rename = "action_typehash")]
    pub action_typehash: String,
    #[serde(rename = "domain_separator")]
    pub domain_separator: String,
    #[serde(rename = "encoded_data")]
    pub encoded_data: String,
    #[serde(rename = "encoded_data_hashed")]
    pub encoded_data_hashed: String,
    #[serde(rename = "expected_signer")]
    pub expected_signer: String,
    #[serde(rename = "input_data")]
    pub input_data: TransferPositionsTakerActionInputData,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "recovered_signer", default, skip_serializing_if = "Option::is_none")]
    pub recovered_signer: Option<String>,
    #[serde(rename = "typed_data_hash")]
    pub typed_data_hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TransferSpotActionInputData {
    #[serde(rename = "data")]
    pub data: TransferActionDataResponse,
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "signer")]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for TransferSpotActionInputData {
    fn default() -> Self {
        Self {
            data: Default::default(),
            expiry: 0.into(),
            module: Default::default(),
            nonce: Default::default(),
            owner: Default::default(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferSpotDebugResponse {
    #[serde(rename = "action_hash")]
    pub action_hash: String,
    #[serde(rename = "action_typehash")]
    pub action_typehash: String,
    #[serde(rename = "domain_separator")]
    pub domain_separator: String,
    #[serde(rename = "encoded_data")]
    pub encoded_data: String,
    #[serde(rename = "encoded_data_hashed")]
    pub encoded_data_hashed: String,
    #[serde(rename = "expected_signer")]
    pub expected_signer: String,
    #[serde(rename = "input_data")]
    pub input_data: TransferSpotActionInputData,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "recovered_signer", default, skip_serializing_if = "Option::is_none")]
    pub recovered_signer: Option<String>,
    #[serde(rename = "typed_data_hash")]
    pub typed_data_hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TransferSpotExternalActionInputData {
    #[serde(rename = "data")]
    pub data: ExternalTransferActionDataResponse,
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "signer")]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for TransferSpotExternalActionInputData {
    fn default() -> Self {
        Self {
            data: Default::default(),
            expiry: 0.into(),
            module: Default::default(),
            nonce: Default::default(),
            owner: Default::default(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferSpotExternalDebugResponse {
    #[serde(rename = "action_hash")]
    pub action_hash: String,
    #[serde(rename = "action_typehash")]
    pub action_typehash: String,
    #[serde(rename = "domain_separator")]
    pub domain_separator: String,
    #[serde(rename = "encoded_data")]
    pub encoded_data: String,
    #[serde(rename = "encoded_data_hashed")]
    pub encoded_data_hashed: String,
    #[serde(rename = "expected_signer")]
    pub expected_signer: String,
    #[serde(rename = "input_data")]
    pub input_data: TransferSpotExternalActionInputData,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "recovered_signer", default, skip_serializing_if = "Option::is_none")]
    pub recovered_signer: Option<String>,
    #[serde(rename = "typed_data_hash")]
    pub typed_data_hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UniverseManagers {
    #[serde(rename = "pm", default, skip_serializing_if = "Option::is_none")]
    pub pm: Option<serde_json::Number>,
    #[serde(rename = "risk_universe_id")]
    pub risk_universe_id: serde_json::Number,
    #[serde(rename = "risk_universe_name", default, skip_serializing_if = "Option::is_none")]
    pub risk_universe_name: Option<String>,
    #[serde(rename = "sm", default, skip_serializing_if = "Option::is_none")]
    pub sm: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for UniverseManagers {
    fn default() -> Self {
        Self {
            pm: Default::default(),
            risk_universe_id: 0.into(),
            risk_universe_name: Default::default(),
            sm: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateVaultInfoRequest {
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "mtm_cap", default, skip_serializing_if = "Option::is_none")]
    pub mtm_cap: Option<String>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "whitelist_only", default, skip_serializing_if = "Option::is_none")]
    pub whitelist_only: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for UpdateVaultInfoRequest {
    fn default() -> Self {
        Self {
            description: Default::default(),
            mtm_cap: Default::default(),
            name: Default::default(),
            subaccount_id: 0.into(),
            whitelist_only: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateWhitelistedRecipientsActionDataResponse {
    #[serde(rename = "add")]
    pub add: Vec<String>,
    #[serde(rename = "remove")]
    pub remove: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateWhitelistedRecipientsActionInputData {
    #[serde(rename = "data")]
    pub data: UpdateWhitelistedRecipientsActionDataResponse,
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "signer")]
    pub signer: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for UpdateWhitelistedRecipientsActionInputData {
    fn default() -> Self {
        Self {
            data: Default::default(),
            expiry: 0.into(),
            module: Default::default(),
            nonce: Default::default(),
            owner: Default::default(),
            signer: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateWhitelistedRecipientsDebugResponse {
    #[serde(rename = "action_hash")]
    pub action_hash: String,
    #[serde(rename = "action_typehash")]
    pub action_typehash: String,
    #[serde(rename = "domain_separator")]
    pub domain_separator: String,
    #[serde(rename = "encoded_data")]
    pub encoded_data: String,
    #[serde(rename = "encoded_data_hashed")]
    pub encoded_data_hashed: String,
    #[serde(rename = "expected_signer")]
    pub expected_signer: String,
    #[serde(rename = "input_data")]
    pub input_data: UpdateWhitelistedRecipientsActionInputData,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "recovered_signer", default, skip_serializing_if = "Option::is_none")]
    pub recovered_signer: Option<String>,
    #[serde(rename = "typed_data_hash")]
    pub typed_data_hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateWhitelistedRecipientsRequest {
    #[serde(rename = "add")]
    pub add: Vec<String>,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "remove")]
    pub remove: Vec<String>,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: String,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for UpdateWhitelistedRecipientsRequest {
    fn default() -> Self {
        Self {
            add: Default::default(),
            nonce: Default::default(),
            remove: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateWhitelistedRecipientsResponse {
    #[serde(rename = "op_uuid")]
    pub op_uuid: String,
    #[serde(rename = "operation_id")]
    pub operation_id: serde_json::Number,
    #[serde(rename = "whitelisted_recipients")]
    pub whitelisted_recipients: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for UpdateWhitelistedRecipientsResponse {
    fn default() -> Self {
        Self {
            op_uuid: Default::default(),
            operation_id: 0.into(),
            whitelisted_recipients: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Vault {
    #[serde(rename = "benchmark_price", default, skip_serializing_if = "Option::is_none")]
    pub benchmark_price: Option<String>,
    #[serde(rename = "curator")]
    pub curator: Address,
    #[serde(rename = "curator_shares")]
    pub curator_shares: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "mtm_cap", default, skip_serializing_if = "Option::is_none")]
    pub mtm_cap: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nav_benchmark", default, skip_serializing_if = "Option::is_none")]
    pub nav_benchmark: Option<String>,
    #[serde(rename = "nav_usd", default, skip_serializing_if = "Option::is_none")]
    pub nav_usd: Option<String>,
    #[serde(rename = "protocol")]
    pub protocol: ProtocolVault,
    #[serde(rename = "simulated_share_price_usd", default, skip_serializing_if = "Option::is_none")]
    pub simulated_share_price_usd: Option<String>,
    #[serde(rename = "whitelist_only")]
    pub whitelist_only: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VaultActionResponse {
    #[serde(rename = "after_shares")]
    pub after_shares: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "before_shares")]
    pub before_shares: String,
    #[serde(rename = "creation_timestamp_ms")]
    pub creation_timestamp_ms: serde_json::Number,
    #[serde(rename = "entry_price")]
    pub entry_price: String,
    #[serde(rename = "error_reason")]
    pub error_reason: String,
    #[serde(rename = "event_ts")]
    pub event_ts: serde_json::Number,
    #[serde(rename = "event_type")]
    pub event_type: String,
    #[serde(rename = "exit_price")]
    pub exit_price: String,
    #[serde(rename = "operation_id")]
    pub operation_id: serde_json::Number,
    #[serde(rename = "operation_uuid")]
    pub operation_uuid: String,
    #[serde(rename = "share_price")]
    pub share_price: String,
    #[serde(rename = "shares_delta")]
    pub shares_delta: String,
    #[serde(rename = "shares_requested")]
    pub shares_requested: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "user_action_hash")]
    pub user_action_hash: String,
    #[serde(rename = "vault_nonce")]
    pub vault_nonce: String,
    #[serde(rename = "vault_subaccount_id")]
    pub vault_subaccount_id: serde_json::Number,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for VaultActionResponse {
    fn default() -> Self {
        Self {
            after_shares: Default::default(),
            amount: Default::default(),
            before_shares: Default::default(),
            creation_timestamp_ms: 0.into(),
            entry_price: Default::default(),
            error_reason: Default::default(),
            event_ts: 0.into(),
            event_type: Default::default(),
            exit_price: Default::default(),
            operation_id: 0.into(),
            operation_uuid: Default::default(),
            share_price: Default::default(),
            shares_delta: Default::default(),
            shares_requested: Default::default(),
            status: Default::default(),
            user_action_hash: Default::default(),
            vault_nonce: Default::default(),
            vault_subaccount_id: 0.into(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VaultCancelResponse {
    #[serde(rename = "cancelled_request_ids")]
    pub cancelled_request_ids: Vec<VaultRequestId>,
    #[serde(rename = "op_uuid")]
    pub op_uuid: String,
    #[serde(rename = "operation_id")]
    pub operation_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for VaultCancelResponse {
    fn default() -> Self {
        Self {
            cancelled_request_ids: Default::default(),
            op_uuid: Default::default(),
            operation_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    #[serde(rename = "benchmark_asset", default, skip_serializing_if = "Option::is_none")]
    pub benchmark_asset: Option<Address>,
    #[serde(rename = "cooldown_sec")]
    pub cooldown_sec: serde_json::Number,
    #[serde(rename = "deposit_spot_asset")]
    pub deposit_spot_asset: Address,
    #[serde(rename = "management_fee_bps")]
    pub management_fee_bps: serde_json::Number,
    #[serde(rename = "max_slippage_bps")]
    pub max_slippage_bps: serde_json::Number,
    #[serde(rename = "performance_fee_bps")]
    pub performance_fee_bps: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            benchmark_asset: Default::default(),
            cooldown_sec: 0.into(),
            deposit_spot_asset: Default::default(),
            management_fee_bps: 0.into(),
            max_slippage_bps: 0.into(),
            performance_fee_bps: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VaultCreateResponse {
    #[serde(rename = "op_uuid")]
    pub op_uuid: String,
    #[serde(rename = "operation_id")]
    pub operation_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for VaultCreateResponse {
    fn default() -> Self {
        Self {
            op_uuid: Default::default(),
            operation_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VaultDepositHold {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset_name")]
    pub asset_name: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "vault_id")]
    pub vault_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for VaultDepositHold {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            asset_name: Default::default(),
            currency: Default::default(),
            vault_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VaultForceBurnResponse {
    #[serde(rename = "op_uuid")]
    pub op_uuid: String,
    #[serde(rename = "operation_id")]
    pub operation_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for VaultForceBurnResponse {
    fn default() -> Self {
        Self {
            op_uuid: Default::default(),
            operation_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultIdsResponse {
    #[serde(rename = "subaccount_ids")]
    pub subaccount_ids: Vec<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VaultPerformanceHistoryResult {
    #[serde(rename = "points")]
    pub points: Vec<VaultPerformancePointResponse>,
    #[serde(rename = "resolution")]
    pub resolution: PerformanceResolution,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for VaultPerformanceHistoryResult {
    fn default() -> Self {
        Self {
            points: Default::default(),
            resolution: Default::default(),
            subaccount_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VaultPerformancePointResponse {
    #[serde(rename = "benchmark_price", skip_serializing_if = "Option::is_none")]
    pub benchmark_price: Option<String>,
    #[serde(rename = "curator_shares")]
    pub curator_shares: String,
    #[serde(rename = "global_hwm")]
    pub global_hwm: String,
    #[serde(rename = "nav", skip_serializing_if = "Option::is_none")]
    pub nav: Option<String>,
    #[serde(rename = "nav_benchmark", skip_serializing_if = "Option::is_none")]
    pub nav_benchmark: Option<String>,
    #[serde(rename = "share_price")]
    pub share_price: String,
    #[serde(rename = "total_shares")]
    pub total_shares: String,
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for VaultPerformancePointResponse {
    fn default() -> Self {
        Self {
            benchmark_price: Default::default(),
            curator_shares: Default::default(),
            global_hwm: Default::default(),
            nav: Default::default(),
            nav_benchmark: Default::default(),
            share_price: Default::default(),
            total_shares: Default::default(),
            ts: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultRequestAckResponse {
    #[serde(rename = "request_id")]
    pub request_id: VaultRequestId,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VaultRequestId {
    #[serde(rename = "vault_nonce")]
    pub vault_nonce: String,
    #[serde(rename = "vault_subaccount_id")]
    pub vault_subaccount_id: serde_json::Number,
    #[serde(rename = "wallet")]
    pub wallet: Address,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for VaultRequestId {
    fn default() -> Self {
        Self {
            vault_nonce: Default::default(),
            vault_subaccount_id: 0.into(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VaultRequestResponse {
    #[serde(rename = "creation_timestamp_ms")]
    pub creation_timestamp_ms: serde_json::Number,
    #[serde(rename = "id")]
    pub id: VaultRequestId,
    #[serde(rename = "signed_action")]
    pub signed_action: SignedAction,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "user_action_hash")]
    pub user_action_hash: String,
    #[serde(rename = "wallet")]
    pub wallet: Address,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for VaultRequestResponse {
    fn default() -> Self {
        Self {
            creation_timestamp_ms: 0.into(),
            id: Default::default(),
            signed_action: Default::default(),
            subaccount_id: 0.into(),
            user_action_hash: Default::default(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VaultSettleResponse {
    #[serde(rename = "op_uuid")]
    pub op_uuid: String,
    #[serde(rename = "operation_id")]
    pub operation_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for VaultSettleResponse {
    fn default() -> Self {
        Self {
            op_uuid: Default::default(),
            operation_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultShareEntryResponse {
    #[serde(rename = "shares")]
    pub shares: String,
    #[serde(rename = "vault")]
    pub vault: Vault,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultSharesResponse {
    #[serde(rename = "vaults")]
    pub vaults: Vec<VaultShareEntryResponse>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultsResponse {
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "vaults")]
    pub vaults: Vec<Vault>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VolFeedDataResponse {
    #[serde(rename = "confidence")]
    pub confidence: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "deadline")]
    pub deadline: serde_json::Number,
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(rename = "signatures")]
    pub signatures: OracleSignatureDataResponse,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "vol_data")]
    pub vol_data: VolSVIParamDataResponse,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for VolFeedDataResponse {
    fn default() -> Self {
        Self {
            confidence: Default::default(),
            currency: Default::default(),
            deadline: 0.into(),
            expiry: 0.into(),
            signatures: Default::default(),
            timestamp: 0.into(),
            vol_data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VolSVIParamDataResponse {
    #[serde(rename = "SVI_a")]
    pub svi_a: String,
    #[serde(rename = "SVI_b")]
    pub svi_b: String,
    #[serde(rename = "SVI_fwd")]
    pub svi_fwd: String,
    #[serde(rename = "SVI_m")]
    pub svi_m: String,
    #[serde(rename = "SVI_refTau")]
    pub svi_ref_tau: String,
    #[serde(rename = "SVI_rho")]
    pub svi_rho: String,
    #[serde(rename = "SVI_sigma")]
    pub svi_sigma: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WithdrawAddressEntry {
    #[serde(rename = "address_kind")]
    pub address_kind: AddressKind,
    #[serde(rename = "chain_id")]
    pub chain_id: serde_json::Number,
    #[serde(rename = "recipient")]
    pub recipient: String,
    #[serde(rename = "recipient_display")]
    pub recipient_display: String,
    #[serde(rename = "whitelisted")]
    pub whitelisted: bool,
    #[serde(rename = "withdraw_address")]
    pub withdraw_address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for WithdrawAddressEntry {
    fn default() -> Self {
        Self {
            address_kind: Default::default(),
            chain_id: 0.into(),
            recipient: Default::default(),
            recipient_display: Default::default(),
            whitelisted: Default::default(),
            withdraw_address: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WithdrawalEntry {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "batch_status")]
    pub batch_status: BatchStatus,
    #[serde(rename = "batch_uuid")]
    pub batch_uuid: String,
    #[serde(rename = "erc20_address")]
    pub erc20_address: String,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "forward", default, skip_serializing_if = "Option::is_none")]
    pub forward: Option<WithdrawalForward>,
    #[serde(rename = "operation_id")]
    pub operation_id: String,
    #[serde(rename = "recipient")]
    pub recipient: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for WithdrawalEntry {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            asset: Default::default(),
            batch_status: Default::default(),
            batch_uuid: Default::default(),
            erc20_address: Default::default(),
            fee: Default::default(),
            forward: Default::default(),
            operation_id: Default::default(),
            recipient: Default::default(),
            subaccount_id: 0.into(),
            timestamp: 0.into(),
            tx_hash: Default::default(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WithdrawalForward {
    #[serde(rename = "amount_forwarded", skip_serializing_if = "Option::is_none")]
    pub amount_forwarded: Option<String>,
    #[serde(rename = "chain_id")]
    pub chain_id: serde_json::Number,
    #[serde(rename = "destination_tx", default, skip_serializing_if = "Option::is_none")]
    pub destination_tx: Option<String>,
    #[serde(rename = "forward_tx", default, skip_serializing_if = "Option::is_none")]
    pub forward_tx: Option<String>,
    #[serde(rename = "hold_reason", default, skip_serializing_if = "Option::is_none")]
    pub hold_reason: Option<String>,
    #[serde(rename = "message_id", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "rail", default, skip_serializing_if = "Option::is_none")]
    pub rail: Option<String>,
    #[serde(rename = "recipient")]
    pub recipient: String,
    #[serde(rename = "status")]
    pub status: WithdrawalForwardStatus,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for WithdrawalForward {
    fn default() -> Self {
        Self {
            amount_forwarded: Default::default(),
            chain_id: 0.into(),
            destination_tx: Default::default(),
            forward_tx: Default::default(),
            hold_reason: Default::default(),
            message_id: Default::default(),
            rail: Default::default(),
            recipient: Default::default(),
            status: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WithdrawalHistoryResult {
    #[serde(rename = "withdrawals")]
    pub withdrawals: Vec<WithdrawalEntry>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LoginRequest {
    #[serde(rename = "signature", default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<serde_json::Number>,
    #[serde(rename = "wallet", default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetCancelOnDisconnectRequest {
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "wallet", default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SubscribeParams {
    #[serde(rename = "channels")]
    pub channels: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SubscribeResult {
    #[serde(rename = "current_subscriptions")]
    pub current_subscriptions: Vec<String>,
    #[serde(rename = "status")]
    pub status: serde_json::Map<String,serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UnsubscribeParams {
    #[serde(rename = "channels", default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UnsubscribeResult {
    #[serde(rename = "remaining_subscriptions")]
    pub remaining_subscriptions: Vec<String>,
    #[serde(rename = "status")]
    pub status: serde_json::Map<String,serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AuctionDetailsSubaccountBalances {
    #[serde(rename = "{key}", default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuctionDetails {
    #[serde(rename = "currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "estimated_bid_price")]
    pub estimated_bid_price: String,
    #[serde(rename = "estimated_discount_pnl")]
    pub estimated_discount_pnl: String,
    #[serde(rename = "estimated_mtm")]
    pub estimated_mtm: String,
    #[serde(rename = "estimated_percent_bid")]
    pub estimated_percent_bid: String,
    #[serde(rename = "margin_type")]
    pub margin_type: String,
    #[serde(rename = "min_price_limit")]
    pub min_price_limit: String,
    #[serde(rename = "risk_universe_id")]
    pub risk_universe_id: serde_json::Number,
    #[serde(rename = "subaccount_balances")]
    pub subaccount_balances: AuctionDetailsSubaccountBalances,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for AuctionDetails {
    fn default() -> Self {
        Self {
            currency: Default::default(),
            estimated_bid_price: Default::default(),
            estimated_discount_pnl: Default::default(),
            estimated_mtm: Default::default(),
            estimated_percent_bid: Default::default(),
            margin_type: Default::default(),
            min_price_limit: Default::default(),
            risk_universe_id: 0.into(),
            subaccount_balances: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuctionResult {
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<AuctionDetails>,
    #[serde(rename = "state")]
    pub state: AuctionStateType,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for AuctionResult {
    fn default() -> Self {
        Self {
            details: Default::default(),
            state: Default::default(),
            subaccount_id: 0.into(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BalanceUpdate {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "new_balance")]
    pub new_balance: String,
    #[serde(rename = "previous_balance")]
    pub previous_balance: String,
    #[serde(rename = "update_type")]
    pub update_type: BalanceUpdateType,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BestQuoteChannelResult {
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<RPCError>,
    #[serde(rename = "result", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<RfqGetBestQuoteResponse>,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MarginWatchResult {
    #[serde(rename = "collaterals")]
    pub collaterals: Vec<serde_json::Value>,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "initial_margin")]
    pub initial_margin: String,
    #[serde(rename = "maintenance_margin")]
    pub maintenance_margin: String,
    #[serde(rename = "margin_type")]
    pub margin_type: String,
    #[serde(rename = "positions")]
    pub positions: Vec<serde_json::Value>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "subaccount_value")]
    pub subaccount_value: String,
    #[serde(rename = "valuation_timestamp")]
    pub valuation_timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for MarginWatchResult {
    fn default() -> Self {
        Self {
            collaterals: Default::default(),
            currency: Default::default(),
            initial_margin: Default::default(),
            maintenance_margin: Default::default(),
            margin_type: Default::default(),
            positions: Default::default(),
            subaccount_id: 0.into(),
            subaccount_value: Default::default(),
            valuation_timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderbookPayloadDoc {
    #[serde(rename = "asks")]
    pub asks: Vec<Vec<String>>,
    #[serde(rename = "bids")]
    pub bids: Vec<Vec<String>>,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "publish_id")]
    pub publish_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for OrderbookPayloadDoc {
    fn default() -> Self {
        Self {
            asks: Default::default(),
            bids: Default::default(),
            instrument_name: Default::default(),
            publish_id: 0.into(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicTrade {
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "expected_rebate")]
    pub expected_rebate: String,
    #[serde(rename = "extra_fee")]
    pub extra_fee: String,
    #[serde(rename = "index_price")]
    pub index_price: String,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "liquidity_role")]
    pub liquidity_role: LiquidityRole2,
    #[serde(rename = "mark_price")]
    pub mark_price: String,
    #[serde(rename = "quote_id", default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(rename = "realized_pnl")]
    pub realized_pnl: String,
    #[serde(rename = "realized_pnl_excl_fees")]
    pub realized_pnl_excl_fees: String,
    #[serde(rename = "rfq_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "trade_amount")]
    pub trade_amount: String,
    #[serde(rename = "trade_fee")]
    pub trade_fee: String,
    #[serde(rename = "trade_id")]
    pub trade_id: String,
    #[serde(rename = "trade_price")]
    pub trade_price: String,
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PublicTrade {
    fn default() -> Self {
        Self {
            direction: Default::default(),
            expected_rebate: Default::default(),
            extra_fee: Default::default(),
            index_price: Default::default(),
            instrument_name: Default::default(),
            liquidity_role: Default::default(),
            mark_price: Default::default(),
            quote_id: Default::default(),
            realized_pnl: Default::default(),
            realized_pnl_excl_fees: Default::default(),
            rfq_id: Default::default(),
            subaccount_id: 0.into(),
            timestamp: 0.into(),
            trade_amount: Default::default(),
            trade_fee: Default::default(),
            trade_id: Default::default(),
            trade_price: Default::default(),
            wallet: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QuotePublishResult {
    #[serde(rename = "batch_status", default, skip_serializing_if = "Option::is_none")]
    pub batch_status: Option<BatchStatus>,
    #[serde(rename = "cancel_reason", default, skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<RFQCancelReason>,
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: serde_json::Number,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "extra_fee")]
    pub extra_fee: String,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "fill_pct")]
    pub fill_pct: String,
    #[serde(rename = "is_transfer")]
    pub is_transfer: bool,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "last_update_timestamp")]
    pub last_update_timestamp: serde_json::Number,
    #[serde(rename = "legs")]
    pub legs: Vec<PricedLegParamsAndResponse>,
    #[serde(rename = "legs_hash")]
    pub legs_hash: String,
    #[serde(rename = "liquidity_role")]
    pub liquidity_role: LiquidityRole,
    #[serde(rename = "max_fee")]
    pub max_fee: String,
    #[serde(rename = "mmp")]
    pub mmp: bool,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_expiry_sec", default = "zero_number")]
    pub signature_expiry_sec: serde_json::Number,
    #[serde(rename = "signer", default)]
    pub signer: String,
    #[serde(rename = "status")]
    pub status: RFQStatus,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: serde_json::Number,
    #[serde(rename = "tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for QuotePublishResult {
    fn default() -> Self {
        Self {
            batch_status: Default::default(),
            cancel_reason: Default::default(),
            creation_timestamp: 0.into(),
            direction: Default::default(),
            extra_fee: Default::default(),
            fee: Default::default(),
            fill_pct: Default::default(),
            is_transfer: Default::default(),
            label: Default::default(),
            last_update_timestamp: 0.into(),
            legs: Default::default(),
            legs_hash: Default::default(),
            liquidity_role: Default::default(),
            max_fee: Default::default(),
            mmp: Default::default(),
            nonce: Default::default(),
            quote_id: Default::default(),
            rfq_id: Default::default(),
            signature: Default::default(),
            signature_expiry_sec: 0.into(),
            signer: Default::default(),
            status: Default::default(),
            subaccount_id: 0.into(),
            tx_hash: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpotFeedEntry {
    #[serde(rename = "confidence")]
    pub confidence: String,
    #[serde(rename = "confidence_prev_daily")]
    pub confidence_prev_daily: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "price_prev_daily")]
    pub price_prev_daily: String,
    #[serde(rename = "timestamp_prev_daily")]
    pub timestamp_prev_daily: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SpotFeedEntry {
    fn default() -> Self {
        Self {
            confidence: Default::default(),
            confidence_prev_daily: Default::default(),
            price: Default::default(),
            price_prev_daily: Default::default(),
            timestamp_prev_daily: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SpotFeedPayloadFeeds {
    #[serde(rename = "{key}", default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SpotFeedEntry>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpotFeedPayload {
    #[serde(rename = "feeds")]
    pub feeds: SpotFeedPayloadFeeds,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SpotFeedPayload {
    fn default() -> Self {
        Self {
            feeds: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TickerSlimPayload {
    #[serde(rename = "instrument_ticker")]
    pub instrument_ticker: TickerSlimSnapshot,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for TickerSlimPayload {
    fn default() -> Self {
        Self {
            instrument_ticker: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

pub struct TypedDeriveClient(DeriveClient);
impl DeriveClient {
    pub fn typed(self) -> TypedDeriveClient { TypedDeriveClient(self) }
}
impl TypedDeriveClient {
    pub fn private_burn_vault_shares(&self, request: &BurnSharesRequest) -> Result<VaultSettleResponse> {
        serde_json::from_value(self.0.private_burn_vault_shares(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_cancel(&self, request: &CancelOrderRequest) -> Result<Order> {
        serde_json::from_value(self.0.private_cancel(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_cancel_algo_order(&self, request: &CancelAlgoOrderRequest) -> Result<Order> {
        serde_json::from_value(self.0.private_cancel_algo_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_cancel_all(&self, request: &CancelAllRequest) -> Result<CancelAllResponse> {
        serde_json::from_value(self.0.private_cancel_all(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_cancel_all_algo_orders(&self, request: &CancelAllAlgoOrdersRequest) -> Result<CancelAllAlgoOrdersResponse> {
        serde_json::from_value(self.0.private_cancel_all_algo_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_cancel_all_trigger_orders(&self, request: &CancelAllTriggerOrdersRequest) -> Result<CancelAllTriggerOrdersResponse> {
        serde_json::from_value(self.0.private_cancel_all_trigger_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_cancel_all_vault_requests(&self, request: &CancelVaultRequestRequest) -> Result<VaultCancelResponse> {
        serde_json::from_value(self.0.private_cancel_all_vault_requests(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_cancel_batch_quotes(&self, request: &CancelBatchQuotesRequest) -> Result<CancelBatchResult> {
        serde_json::from_value(self.0.private_cancel_batch_quotes(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_cancel_batch_rfqs(&self, request: &CancelBatchRfqsRequest) -> Result<CancelBatchRfqsResponse> {
        serde_json::from_value(self.0.private_cancel_batch_rfqs(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_cancel_by_instrument(&self, request: &CancelByInstrumentRequest) -> Result<CancelByInstrumentResponse> {
        serde_json::from_value(self.0.private_cancel_by_instrument(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_cancel_by_label(&self, request: &CancelByLabelRequest) -> Result<CancelByLabelResponse> {
        serde_json::from_value(self.0.private_cancel_by_label(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_cancel_by_nonce(&self, request: &CancelByNonceRequest) -> Result<CancelByNonceResponse> {
        serde_json::from_value(self.0.private_cancel_by_nonce(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_cancel_quote(&self, request: &CancelQuoteRequest) -> Result<Quote> {
        serde_json::from_value(self.0.private_cancel_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_cancel_rfq(&self, request: &CancelRfqRequest) -> Result<CancelRfqResponse> {
        serde_json::from_value(self.0.private_cancel_rfq(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_cancel_trigger_order(&self, request: &CancelTriggerOrderRequest) -> Result<Order> {
        serde_json::from_value(self.0.private_cancel_trigger_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_change_subaccount_label(&self, request: &ChangeSubaccountLabelRequest) -> Result<PrivateChangeSubaccountLabelResponse> {
        serde_json::from_value(self.0.private_change_subaccount_label(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_create_vault(&self, request: &CreateVaultRequest) -> Result<VaultCreateResponse> {
        serde_json::from_value(self.0.private_create_vault(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_delete_subaccount(&self, request: &DeleteSubaccountRequest) -> Result<DeleteSubaccountResponse> {
        serde_json::from_value(self.0.private_delete_subaccount(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_edit_session_key(&self, request: &EditSessionKeyRequest) -> Result<SessionKey> {
        serde_json::from_value(self.0.private_edit_session_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_execute_quote(&self, request: &ExecuteQuoteRequest) -> Result<QuoteExecuteResponse> {
        serde_json::from_value(self.0.private_execute_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_force_burn(&self, request: &ForceBurnRequest) -> Result<VaultForceBurnResponse> {
        serde_json::from_value(self.0.private_force_burn(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_account(&self, request: &GetAccountRequest) -> Result<PrivateGetAccountResponse> {
        serde_json::from_value(self.0.private_get_account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_algo_orders(&self, request: &GetAlgoOrdersRequest) -> Result<Vec<Order>> {
        serde_json::from_value(self.0.private_get_algo_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_all_portfolios(&self, request: &GetAllPortfoliosRequest) -> Result<Vec<Subaccount>> {
        serde_json::from_value(self.0.private_get_all_portfolios(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_collaterals(&self, request: &GetCollateralsRequest) -> Result<PrivateGetCollateralsResponse> {
        serde_json::from_value(self.0.private_get_collaterals(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_curated_vaults(&self, request: &GetCuratedVaultsRequest) -> Result<VaultIdsResponse> {
        serde_json::from_value(self.0.private_get_curated_vaults(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_deposit_history(&self, request: &GetDepositHistoryRequest) -> Result<DepositHistoryResult> {
        serde_json::from_value(self.0.private_get_deposit_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_erc20_transfer_history(&self, request: &GetErc20TransferHistoryRequest) -> Result<TransferHistoryResult> {
        serde_json::from_value(self.0.private_get_erc20_transfer_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_funding_history(&self, request: &GetFundingHistoryRequest) -> Result<PerpSettlementHistoryResponse> {
        serde_json::from_value(self.0.private_get_funding_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_interest_history(&self, request: &GetInterestHistoryRequest) -> Result<InterestHistoryResult> {
        serde_json::from_value(self.0.private_get_interest_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_live_burn_requests(&self, request: &GetLiveBurnRequestsRequest) -> Result<MultipleVaultRequestsResponse> {
        serde_json::from_value(self.0.private_get_live_burn_requests(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_live_mint_requests(&self, request: &GetLiveMintRequestsRequest) -> Result<MultipleVaultRequestsResponse> {
        serde_json::from_value(self.0.private_get_live_mint_requests(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_live_vault_requests(&self, request: &GetLiveVaultRequestsRequest) -> Result<MultipleVaultRequestsResponse> {
        serde_json::from_value(self.0.private_get_live_vault_requests(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_maker_rfq_history(&self, request: &GetMakerRfqHistoryRequest) -> Result<RFQPollResponse> {
        serde_json::from_value(self.0.private_get_maker_rfq_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_margin(&self, request: &PrivateGetMarginRequest) -> Result<GetMarginResponse> {
        serde_json::from_value(self.0.private_get_margin(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_mmp_config(&self, request: &MmpScopeRequest) -> Result<Vec<MmpConfigResult>> {
        serde_json::from_value(self.0.private_get_mmp_config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_open_orders(&self, request: &GetOpenOrdersRequest) -> Result<AggregatedOrdersResult> {
        serde_json::from_value(self.0.private_get_open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_option_settlement_history(&self, request: &GetOptionSettlementHistoryParams) -> Result<OptionSettlementHistoryResponse> {
        serde_json::from_value(self.0.private_get_option_settlement_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_order(&self, request: &GetOrderRequest) -> Result<Order> {
        serde_json::from_value(self.0.private_get_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_order_history(&self, request: &GetOrderHistoryRequest) -> Result<PaginatedOrdersResult> {
        serde_json::from_value(self.0.private_get_order_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_positions(&self, request: &GetPositionsRequest) -> Result<PrivateGetPositionsResponse> {
        serde_json::from_value(self.0.private_get_positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_quotes(&self, request: &GetQuotesRequest) -> Result<QuoteGetResponse> {
        serde_json::from_value(self.0.private_get_quotes(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_rfqs(&self, request: &GetRfqsRequest) -> Result<RFQGetResponse> {
        serde_json::from_value(self.0.private_get_rfqs(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_shareholder_vaults(&self, request: &GetShareholderVaultsRequest) -> Result<VaultIdsResponse> {
        serde_json::from_value(self.0.private_get_shareholder_vaults(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_subaccount(&self, request: &GetSubaccountRequest) -> Result<Subaccount> {
        serde_json::from_value(self.0.private_get_subaccount(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_subaccount_value_history(&self, request: &GetSubaccountValueHistoryRequest) -> Result<SubaccountValueHistoryResult> {
        serde_json::from_value(self.0.private_get_subaccount_value_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_subaccounts(&self, request: &GetSubaccountsRequest) -> Result<PrivateGetSubaccountsResponse> {
        serde_json::from_value(self.0.private_get_subaccounts(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_trade_history(&self, request: &GetTradeHistoryRequest) -> Result<PaginatedTradesResult> {
        serde_json::from_value(self.0.private_get_trade_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_trigger_orders(&self, request: &GetTriggerOrdersRequest) -> Result<AggregatedTriggerOrdersResult> {
        serde_json::from_value(self.0.private_get_trigger_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_vault_request_history(&self, request: &GetVaultRequestHistoryRequest) -> Result<PaginatedVaultRequestHistory> {
        serde_json::from_value(self.0.private_get_vault_request_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_vault_shares(&self, request: &GetVaultSharesRequest) -> Result<VaultSharesResponse> {
        serde_json::from_value(self.0.private_get_vault_shares(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_withdraw_addresses(&self, request: &GetWithdrawAddressesParams) -> Result<GetWithdrawAddressesResult> {
        serde_json::from_value(self.0.private_get_withdraw_addresses(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_get_withdrawal_history(&self, request: &GetWithdrawalHistoryRequest) -> Result<WithdrawalHistoryResult> {
        serde_json::from_value(self.0.private_get_withdrawal_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_liquidate(&self, request: &PrivateLiquidateRequest) -> Result<PrivateLiquidateResponse> {
        serde_json::from_value(self.0.private_liquidate(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_mint_vault_shares(&self, request: &MintSharesRequest) -> Result<VaultSettleResponse> {
        serde_json::from_value(self.0.private_mint_vault_shares(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_order(&self, request: &CreateOrderRequest) -> Result<OrderCreatedResponse> {
        serde_json::from_value(self.0.private_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_order_debug(&self, request: &CreateOrderRequest) -> Result<OrderDebugResponse> {
        serde_json::from_value(self.0.private_order_debug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_order_quote(&self, request: &OrderQuoteRequest) -> Result<OrderQuoteResponse> {
        serde_json::from_value(self.0.private_order_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_poll_quotes(&self, request: &PollQuotesRequest) -> Result<QuotePollResponse> {
        serde_json::from_value(self.0.private_poll_quotes(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_poll_rfqs(&self, request: &PollRfqsRequest) -> Result<RFQPollResponse> {
        serde_json::from_value(self.0.private_poll_rfqs(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_register_bridge_deposit(&self, request: &RegisterBridgeDepositParams) -> Result<PendingDepositBridgeOrigin> {
        serde_json::from_value(self.0.private_register_bridge_deposit(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_register_withdraw_address(&self, request: &RegisterWithdrawAddressParams) -> Result<RegisterWithdrawAddressResult> {
        serde_json::from_value(self.0.private_register_withdraw_address(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_reject_deposit_request(&self, request: &RejectDepositRequestRequest) -> Result<VaultRequestAckResponse> {
        serde_json::from_value(self.0.private_reject_deposit_request(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_replace(&self, request: &ReplaceOrderRequest) -> Result<ReplaceOrderResponse> {
        serde_json::from_value(self.0.private_replace(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_replace_quote(&self, request: &ReplaceQuoteRequest) -> Result<QuoteReplaceResponse> {
        serde_json::from_value(self.0.private_replace_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_request_vault_deposit(&self, request: &RequestVaultDepositRequest) -> Result<VaultRequestAckResponse> {
        serde_json::from_value(self.0.private_request_vault_deposit(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_request_vault_withdraw(&self, request: &RequestVaultWithdrawRequest) -> Result<VaultRequestAckResponse> {
        serde_json::from_value(self.0.private_request_vault_withdraw(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_reset_mmp(&self, request: &MmpScopeRequest) -> Result<ResetMmpResponse> {
        serde_json::from_value(self.0.private_reset_mmp(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_rfq_get_best_quote(&self, request: &RfqGetBestQuoteRequest) -> Result<RfqGetBestQuoteResponse> {
        serde_json::from_value(self.0.private_rfq_get_best_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_send_quote(&self, request: &SendQuoteRequest) -> Result<Quote> {
        serde_json::from_value(self.0.private_send_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_send_rfq(&self, request: &SendRfqRequest) -> Result<Rfq> {
        serde_json::from_value(self.0.private_send_rfq(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_session_keys(&self, request: &SessionKeysRequest) -> Result<PrivateSessionKeysResponse> {
        serde_json::from_value(self.0.private_session_keys(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_set_mmp_config(&self, request: &SetMmpConfigRequest) -> Result<SetMmpConfigResponse> {
        serde_json::from_value(self.0.private_set_mmp_config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_set_session_key(&self, request: &SetSessionKeyRequest) -> Result<PrivateSetSessionKeyResponse> {
        serde_json::from_value(self.0.private_set_session_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_set_session_key_debug(&self, request: &SetSessionKeyRequest) -> Result<SetSessionKeyDebugResponse> {
        serde_json::from_value(self.0.private_set_session_key_debug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_transfer_positions(&self, request: &TransferPositionsRequest) -> Result<TransferPositionsResponse> {
        serde_json::from_value(self.0.private_transfer_positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_transfer_positions_debug(&self, request: &TransferPositionsRequest) -> Result<TransferPositionsDebugResponse> {
        serde_json::from_value(self.0.private_transfer_positions_debug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_transfer_spot(&self, request: &PrivateTransferSpotRequest) -> Result<PrivateTransferSpotResponse> {
        serde_json::from_value(self.0.private_transfer_spot(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_transfer_spot_debug(&self, request: &PrivateTransferSpotRequest) -> Result<TransferSpotDebugResponse> {
        serde_json::from_value(self.0.private_transfer_spot_debug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_transfer_spot_external(&self, request: &PrivateTransferSpotExternalRequest) -> Result<PrivateTransferSpotExternalResponse> {
        serde_json::from_value(self.0.private_transfer_spot_external(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_transfer_spot_external_debug(&self, request: &PrivateTransferSpotExternalRequest) -> Result<TransferSpotExternalDebugResponse> {
        serde_json::from_value(self.0.private_transfer_spot_external_debug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_update_vault_info(&self, request: &UpdateVaultInfoRequest) -> Result<OffchainAckResponse> {
        serde_json::from_value(self.0.private_update_vault_info(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_update_whitelisted_recipients(&self, request: &UpdateWhitelistedRecipientsRequest) -> Result<UpdateWhitelistedRecipientsResponse> {
        serde_json::from_value(self.0.private_update_whitelisted_recipients(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_update_whitelisted_recipients_debug(&self, request: &UpdateWhitelistedRecipientsRequest) -> Result<UpdateWhitelistedRecipientsDebugResponse> {
        serde_json::from_value(self.0.private_update_whitelisted_recipients_debug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn private_withdraw(&self, request: &PrivateWithdrawRequest) -> Result<PrivateWithdrawResponse> {
        serde_json::from_value(self.0.private_withdraw(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_decode_action(&self, request: &DecodeActionRequest) -> Result<DecodeActionResponse> {
        serde_json::from_value(self.0.public_decode_action(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_execute_quote_debug(&self, request: &PublicExecuteQuoteDebugRequest) -> Result<QuoteExecuteDebugResult> {
        serde_json::from_value(self.0.public_execute_quote_debug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_all_currencies(&self, request: &EmptyRequest) -> Result<Vec<Currency>> {
        serde_json::from_value(self.0.public_get_all_currencies(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_all_instruments(&self, request: &GetAllInstrumentsRequest) -> Result<GetAllInstrumentsResponse> {
        serde_json::from_value(self.0.public_get_all_instruments(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_all_live_instruments(&self, request: &EmptyRequest) -> Result<Vec<String>> {
        serde_json::from_value(self.0.public_get_all_live_instruments(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_all_referral_codes(&self, request: &GetAllReferralCodesParams) -> Result<Vec<Referrer>> {
        serde_json::from_value(self.0.public_get_all_referral_codes(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_currency(&self, request: &GetCurrencyRequest) -> Result<Currency> {
        serde_json::from_value(self.0.public_get_currency(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_funding_rate_history(&self, request: &GetFundingRateHistoryRequest) -> Result<FundingRateHistoryResult> {
        serde_json::from_value(self.0.public_get_funding_rate_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_index_chart_data(&self, request: &GetIndexChartDataRequest) -> Result<Vec<IndexCandle>> {
        serde_json::from_value(self.0.public_get_index_chart_data(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_instrument(&self, request: &GetInstrumentRequest) -> Result<Instrument> {
        serde_json::from_value(self.0.public_get_instrument(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_interest_rate_history(&self, request: &GetInterestRateHistoryRequest) -> Result<InterestRateHistoryResult> {
        serde_json::from_value(self.0.public_get_interest_rate_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_latest_signed_feeds(&self, request: &GetLatestSignedFeedsRequest) -> Result<GetLatestSignedFeedsResponse> {
        serde_json::from_value(self.0.public_get_latest_signed_feeds(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_liquidation_history(&self, request: &GetLiquidationHistoryRequest) -> Result<LiquidationHistoryResult> {
        serde_json::from_value(self.0.public_get_liquidation_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_live_auctions(&self, request: &EmptyRequest) -> Result<PublicGetLiveAuctionsResponse> {
        serde_json::from_value(self.0.public_get_live_auctions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_live_incidents(&self, request: &GetLiveIncidentsParams) -> Result<GetLiveIncidentsResponse> {
        serde_json::from_value(self.0.public_get_live_incidents(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_maker_program_scores(&self, request: &GetMakerProgramScoresParams) -> Result<GetMakerProgramScoresResponse> {
        serde_json::from_value(self.0.public_get_maker_program_scores(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_maker_programs(&self, request: &GetMakerProgramsParams) -> Result<Vec<ProgramResponse>> {
        serde_json::from_value(self.0.public_get_maker_programs(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_margin(&self, request: &PublicGetMarginRequest) -> Result<GetMarginResponse> {
        serde_json::from_value(self.0.public_get_margin(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_onchain_action_history(&self, request: &GetOnchainActionHistoryParams) -> Result<GetOnchainActionHistoryResponse> {
        serde_json::from_value(self.0.public_get_onchain_action_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_option_settlement_prices(&self, request: &GetOptionSettlementPricesRequest) -> Result<OptionSettlementPricesResult> {
        serde_json::from_value(self.0.public_get_option_settlement_prices(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_pending_deposits(&self, request: &GetPendingDepositsParams) -> Result<GetPendingDepositsResult> {
        serde_json::from_value(self.0.public_get_pending_deposits(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_perp_impact_twap(&self, request: &GetPerpImpactTwapRequest) -> Result<PerpImpactTwapResult> {
        serde_json::from_value(self.0.public_get_perp_impact_twap(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_rate_limits(&self, request: &EmptyRequest) -> Result<RateLimitResult> {
        serde_json::from_value(self.0.public_get_rate_limits(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_referral_performance(&self, request: &GetReferralPerformanceParams) -> Result<GetReferralPerformanceResult> {
        serde_json::from_value(self.0.public_get_referral_performance(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_risk_universes(&self, request: &EmptyRequest) -> Result<Vec<RiskUniverse>> {
        serde_json::from_value(self.0.public_get_risk_universes(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_subaccounts(&self, request: &PublicGetSubaccountsRequest) -> Result<PrivateGetSubaccountsResponse> {
        serde_json::from_value(self.0.public_get_subaccounts(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_ticker(&self, request: &GetTickerRequest) -> Result<TickerSlimSnapshot> {
        serde_json::from_value(self.0.public_get_ticker(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_tickers(&self, request: &GetTickersRequest) -> Result<GetTickersResponse> {
        serde_json::from_value(self.0.public_get_tickers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_time(&self, request: &EmptyRequest) -> Result<serde_json::Number> {
        serde_json::from_value(self.0.public_get_time(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_trade_history(&self, request: &GetPublicTradeHistoryRequest) -> Result<PublicTradesResult> {
        serde_json::from_value(self.0.public_get_trade_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_tradingview_chart_data(&self, request: &GetTradingviewChartDataRequest) -> Result<Vec<TradingviewCandle>> {
        serde_json::from_value(self.0.public_get_tradingview_chart_data(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_transaction(&self, request: &GetTransactionParams) -> Result<GetTransactionResult> {
        serde_json::from_value(self.0.public_get_transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_vault(&self, request: &GetVaultRequest) -> Result<Vault> {
        serde_json::from_value(self.0.public_get_vault(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_vault_action_history(&self, request: &GetVaultActionHistoryRequest) -> Result<PaginatedVaultActionHistory> {
        serde_json::from_value(self.0.public_get_vault_action_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_vault_performance_history(&self, request: &GetVaultPerformanceHistoryRequest) -> Result<VaultPerformanceHistoryResult> {
        serde_json::from_value(self.0.public_get_vault_performance_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_vaults(&self, request: &GetVaultsRequest) -> Result<VaultsResponse> {
        serde_json::from_value(self.0.public_get_vaults(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_wallet(&self, request: &PublicGetWalletRequest) -> Result<PublicGetWalletResponse> {
        serde_json::from_value(self.0.public_get_wallet(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_get_wallets_from_session_key(&self, request: &GetWalletsFromSessionKeyRequest) -> Result<PublicGetWalletsFromSessionKeyResponse> {
        serde_json::from_value(self.0.public_get_wallets_from_session_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_margin_watch(&self, request: &MarginWatchRequest) -> Result<PublicMarginWatchResponse> {
        serde_json::from_value(self.0.public_margin_watch(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_order_quote(&self, request: &OrderQuoteRequest) -> Result<OrderQuoteResponse> {
        serde_json::from_value(self.0.public_order_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_register_deposit_address(&self, request: &RegisterDepositAddressParams) -> Result<RegisterDepositAddressResult> {
        serde_json::from_value(self.0.public_register_deposit_address(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_send_quote_debug(&self, request: &PublicSendQuoteDebugRequest) -> Result<QuoteSendDebugResult> {
        serde_json::from_value(self.0.public_send_quote_debug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub fn public_withdraw_debug(&self, request: &PublicWithdrawDebugRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.public_withdraw_debug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedDeriveClient(crate::AsyncClient<DeriveClient>);
#[cfg(feature = "async")]
impl TypedDeriveClient {
    pub fn into_async(self) -> AsyncTypedDeriveClient { AsyncTypedDeriveClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedDeriveClient {
    pub async fn private_burn_vault_shares(&self, request: &BurnSharesRequest) -> Result<VaultSettleResponse> {
        serde_json::from_value(self.0.private_burn_vault_shares(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_cancel(&self, request: &CancelOrderRequest) -> Result<Order> {
        serde_json::from_value(self.0.private_cancel(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_cancel_algo_order(&self, request: &CancelAlgoOrderRequest) -> Result<Order> {
        serde_json::from_value(self.0.private_cancel_algo_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_cancel_all(&self, request: &CancelAllRequest) -> Result<CancelAllResponse> {
        serde_json::from_value(self.0.private_cancel_all(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_cancel_all_algo_orders(&self, request: &CancelAllAlgoOrdersRequest) -> Result<CancelAllAlgoOrdersResponse> {
        serde_json::from_value(self.0.private_cancel_all_algo_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_cancel_all_trigger_orders(&self, request: &CancelAllTriggerOrdersRequest) -> Result<CancelAllTriggerOrdersResponse> {
        serde_json::from_value(self.0.private_cancel_all_trigger_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_cancel_all_vault_requests(&self, request: &CancelVaultRequestRequest) -> Result<VaultCancelResponse> {
        serde_json::from_value(self.0.private_cancel_all_vault_requests(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_cancel_batch_quotes(&self, request: &CancelBatchQuotesRequest) -> Result<CancelBatchResult> {
        serde_json::from_value(self.0.private_cancel_batch_quotes(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_cancel_batch_rfqs(&self, request: &CancelBatchRfqsRequest) -> Result<CancelBatchRfqsResponse> {
        serde_json::from_value(self.0.private_cancel_batch_rfqs(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_cancel_by_instrument(&self, request: &CancelByInstrumentRequest) -> Result<CancelByInstrumentResponse> {
        serde_json::from_value(self.0.private_cancel_by_instrument(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_cancel_by_label(&self, request: &CancelByLabelRequest) -> Result<CancelByLabelResponse> {
        serde_json::from_value(self.0.private_cancel_by_label(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_cancel_by_nonce(&self, request: &CancelByNonceRequest) -> Result<CancelByNonceResponse> {
        serde_json::from_value(self.0.private_cancel_by_nonce(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_cancel_quote(&self, request: &CancelQuoteRequest) -> Result<Quote> {
        serde_json::from_value(self.0.private_cancel_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_cancel_rfq(&self, request: &CancelRfqRequest) -> Result<CancelRfqResponse> {
        serde_json::from_value(self.0.private_cancel_rfq(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_cancel_trigger_order(&self, request: &CancelTriggerOrderRequest) -> Result<Order> {
        serde_json::from_value(self.0.private_cancel_trigger_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_change_subaccount_label(&self, request: &ChangeSubaccountLabelRequest) -> Result<PrivateChangeSubaccountLabelResponse> {
        serde_json::from_value(self.0.private_change_subaccount_label(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_create_vault(&self, request: &CreateVaultRequest) -> Result<VaultCreateResponse> {
        serde_json::from_value(self.0.private_create_vault(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_delete_subaccount(&self, request: &DeleteSubaccountRequest) -> Result<DeleteSubaccountResponse> {
        serde_json::from_value(self.0.private_delete_subaccount(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_edit_session_key(&self, request: &EditSessionKeyRequest) -> Result<SessionKey> {
        serde_json::from_value(self.0.private_edit_session_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_execute_quote(&self, request: &ExecuteQuoteRequest) -> Result<QuoteExecuteResponse> {
        serde_json::from_value(self.0.private_execute_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_force_burn(&self, request: &ForceBurnRequest) -> Result<VaultForceBurnResponse> {
        serde_json::from_value(self.0.private_force_burn(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_account(&self, request: &GetAccountRequest) -> Result<PrivateGetAccountResponse> {
        serde_json::from_value(self.0.private_get_account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_algo_orders(&self, request: &GetAlgoOrdersRequest) -> Result<Vec<Order>> {
        serde_json::from_value(self.0.private_get_algo_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_all_portfolios(&self, request: &GetAllPortfoliosRequest) -> Result<Vec<Subaccount>> {
        serde_json::from_value(self.0.private_get_all_portfolios(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_collaterals(&self, request: &GetCollateralsRequest) -> Result<PrivateGetCollateralsResponse> {
        serde_json::from_value(self.0.private_get_collaterals(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_curated_vaults(&self, request: &GetCuratedVaultsRequest) -> Result<VaultIdsResponse> {
        serde_json::from_value(self.0.private_get_curated_vaults(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_deposit_history(&self, request: &GetDepositHistoryRequest) -> Result<DepositHistoryResult> {
        serde_json::from_value(self.0.private_get_deposit_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_erc20_transfer_history(&self, request: &GetErc20TransferHistoryRequest) -> Result<TransferHistoryResult> {
        serde_json::from_value(self.0.private_get_erc20_transfer_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_funding_history(&self, request: &GetFundingHistoryRequest) -> Result<PerpSettlementHistoryResponse> {
        serde_json::from_value(self.0.private_get_funding_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_interest_history(&self, request: &GetInterestHistoryRequest) -> Result<InterestHistoryResult> {
        serde_json::from_value(self.0.private_get_interest_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_live_burn_requests(&self, request: &GetLiveBurnRequestsRequest) -> Result<MultipleVaultRequestsResponse> {
        serde_json::from_value(self.0.private_get_live_burn_requests(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_live_mint_requests(&self, request: &GetLiveMintRequestsRequest) -> Result<MultipleVaultRequestsResponse> {
        serde_json::from_value(self.0.private_get_live_mint_requests(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_live_vault_requests(&self, request: &GetLiveVaultRequestsRequest) -> Result<MultipleVaultRequestsResponse> {
        serde_json::from_value(self.0.private_get_live_vault_requests(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_maker_rfq_history(&self, request: &GetMakerRfqHistoryRequest) -> Result<RFQPollResponse> {
        serde_json::from_value(self.0.private_get_maker_rfq_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_margin(&self, request: &PrivateGetMarginRequest) -> Result<GetMarginResponse> {
        serde_json::from_value(self.0.private_get_margin(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_mmp_config(&self, request: &MmpScopeRequest) -> Result<Vec<MmpConfigResult>> {
        serde_json::from_value(self.0.private_get_mmp_config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_open_orders(&self, request: &GetOpenOrdersRequest) -> Result<AggregatedOrdersResult> {
        serde_json::from_value(self.0.private_get_open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_option_settlement_history(&self, request: &GetOptionSettlementHistoryParams) -> Result<OptionSettlementHistoryResponse> {
        serde_json::from_value(self.0.private_get_option_settlement_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_order(&self, request: &GetOrderRequest) -> Result<Order> {
        serde_json::from_value(self.0.private_get_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_order_history(&self, request: &GetOrderHistoryRequest) -> Result<PaginatedOrdersResult> {
        serde_json::from_value(self.0.private_get_order_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_positions(&self, request: &GetPositionsRequest) -> Result<PrivateGetPositionsResponse> {
        serde_json::from_value(self.0.private_get_positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_quotes(&self, request: &GetQuotesRequest) -> Result<QuoteGetResponse> {
        serde_json::from_value(self.0.private_get_quotes(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_rfqs(&self, request: &GetRfqsRequest) -> Result<RFQGetResponse> {
        serde_json::from_value(self.0.private_get_rfqs(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_shareholder_vaults(&self, request: &GetShareholderVaultsRequest) -> Result<VaultIdsResponse> {
        serde_json::from_value(self.0.private_get_shareholder_vaults(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_subaccount(&self, request: &GetSubaccountRequest) -> Result<Subaccount> {
        serde_json::from_value(self.0.private_get_subaccount(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_subaccount_value_history(&self, request: &GetSubaccountValueHistoryRequest) -> Result<SubaccountValueHistoryResult> {
        serde_json::from_value(self.0.private_get_subaccount_value_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_subaccounts(&self, request: &GetSubaccountsRequest) -> Result<PrivateGetSubaccountsResponse> {
        serde_json::from_value(self.0.private_get_subaccounts(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_trade_history(&self, request: &GetTradeHistoryRequest) -> Result<PaginatedTradesResult> {
        serde_json::from_value(self.0.private_get_trade_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_trigger_orders(&self, request: &GetTriggerOrdersRequest) -> Result<AggregatedTriggerOrdersResult> {
        serde_json::from_value(self.0.private_get_trigger_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_vault_request_history(&self, request: &GetVaultRequestHistoryRequest) -> Result<PaginatedVaultRequestHistory> {
        serde_json::from_value(self.0.private_get_vault_request_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_vault_shares(&self, request: &GetVaultSharesRequest) -> Result<VaultSharesResponse> {
        serde_json::from_value(self.0.private_get_vault_shares(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_withdraw_addresses(&self, request: &GetWithdrawAddressesParams) -> Result<GetWithdrawAddressesResult> {
        serde_json::from_value(self.0.private_get_withdraw_addresses(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_get_withdrawal_history(&self, request: &GetWithdrawalHistoryRequest) -> Result<WithdrawalHistoryResult> {
        serde_json::from_value(self.0.private_get_withdrawal_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_liquidate(&self, request: &PrivateLiquidateRequest) -> Result<PrivateLiquidateResponse> {
        serde_json::from_value(self.0.private_liquidate(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_mint_vault_shares(&self, request: &MintSharesRequest) -> Result<VaultSettleResponse> {
        serde_json::from_value(self.0.private_mint_vault_shares(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_order(&self, request: &CreateOrderRequest) -> Result<OrderCreatedResponse> {
        serde_json::from_value(self.0.private_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_order_debug(&self, request: &CreateOrderRequest) -> Result<OrderDebugResponse> {
        serde_json::from_value(self.0.private_order_debug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_order_quote(&self, request: &OrderQuoteRequest) -> Result<OrderQuoteResponse> {
        serde_json::from_value(self.0.private_order_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_poll_quotes(&self, request: &PollQuotesRequest) -> Result<QuotePollResponse> {
        serde_json::from_value(self.0.private_poll_quotes(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_poll_rfqs(&self, request: &PollRfqsRequest) -> Result<RFQPollResponse> {
        serde_json::from_value(self.0.private_poll_rfqs(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_register_bridge_deposit(&self, request: &RegisterBridgeDepositParams) -> Result<PendingDepositBridgeOrigin> {
        serde_json::from_value(self.0.private_register_bridge_deposit(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_register_withdraw_address(&self, request: &RegisterWithdrawAddressParams) -> Result<RegisterWithdrawAddressResult> {
        serde_json::from_value(self.0.private_register_withdraw_address(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_reject_deposit_request(&self, request: &RejectDepositRequestRequest) -> Result<VaultRequestAckResponse> {
        serde_json::from_value(self.0.private_reject_deposit_request(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_replace(&self, request: &ReplaceOrderRequest) -> Result<ReplaceOrderResponse> {
        serde_json::from_value(self.0.private_replace(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_replace_quote(&self, request: &ReplaceQuoteRequest) -> Result<QuoteReplaceResponse> {
        serde_json::from_value(self.0.private_replace_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_request_vault_deposit(&self, request: &RequestVaultDepositRequest) -> Result<VaultRequestAckResponse> {
        serde_json::from_value(self.0.private_request_vault_deposit(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_request_vault_withdraw(&self, request: &RequestVaultWithdrawRequest) -> Result<VaultRequestAckResponse> {
        serde_json::from_value(self.0.private_request_vault_withdraw(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_reset_mmp(&self, request: &MmpScopeRequest) -> Result<ResetMmpResponse> {
        serde_json::from_value(self.0.private_reset_mmp(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_rfq_get_best_quote(&self, request: &RfqGetBestQuoteRequest) -> Result<RfqGetBestQuoteResponse> {
        serde_json::from_value(self.0.private_rfq_get_best_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_send_quote(&self, request: &SendQuoteRequest) -> Result<Quote> {
        serde_json::from_value(self.0.private_send_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_send_rfq(&self, request: &SendRfqRequest) -> Result<Rfq> {
        serde_json::from_value(self.0.private_send_rfq(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_session_keys(&self, request: &SessionKeysRequest) -> Result<PrivateSessionKeysResponse> {
        serde_json::from_value(self.0.private_session_keys(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_set_mmp_config(&self, request: &SetMmpConfigRequest) -> Result<SetMmpConfigResponse> {
        serde_json::from_value(self.0.private_set_mmp_config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_set_session_key(&self, request: &SetSessionKeyRequest) -> Result<PrivateSetSessionKeyResponse> {
        serde_json::from_value(self.0.private_set_session_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_set_session_key_debug(&self, request: &SetSessionKeyRequest) -> Result<SetSessionKeyDebugResponse> {
        serde_json::from_value(self.0.private_set_session_key_debug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_transfer_positions(&self, request: &TransferPositionsRequest) -> Result<TransferPositionsResponse> {
        serde_json::from_value(self.0.private_transfer_positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_transfer_positions_debug(&self, request: &TransferPositionsRequest) -> Result<TransferPositionsDebugResponse> {
        serde_json::from_value(self.0.private_transfer_positions_debug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_transfer_spot(&self, request: &PrivateTransferSpotRequest) -> Result<PrivateTransferSpotResponse> {
        serde_json::from_value(self.0.private_transfer_spot(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_transfer_spot_debug(&self, request: &PrivateTransferSpotRequest) -> Result<TransferSpotDebugResponse> {
        serde_json::from_value(self.0.private_transfer_spot_debug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_transfer_spot_external(&self, request: &PrivateTransferSpotExternalRequest) -> Result<PrivateTransferSpotExternalResponse> {
        serde_json::from_value(self.0.private_transfer_spot_external(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_transfer_spot_external_debug(&self, request: &PrivateTransferSpotExternalRequest) -> Result<TransferSpotExternalDebugResponse> {
        serde_json::from_value(self.0.private_transfer_spot_external_debug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_update_vault_info(&self, request: &UpdateVaultInfoRequest) -> Result<OffchainAckResponse> {
        serde_json::from_value(self.0.private_update_vault_info(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_update_whitelisted_recipients(&self, request: &UpdateWhitelistedRecipientsRequest) -> Result<UpdateWhitelistedRecipientsResponse> {
        serde_json::from_value(self.0.private_update_whitelisted_recipients(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_update_whitelisted_recipients_debug(&self, request: &UpdateWhitelistedRecipientsRequest) -> Result<UpdateWhitelistedRecipientsDebugResponse> {
        serde_json::from_value(self.0.private_update_whitelisted_recipients_debug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn private_withdraw(&self, request: &PrivateWithdrawRequest) -> Result<PrivateWithdrawResponse> {
        serde_json::from_value(self.0.private_withdraw(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_decode_action(&self, request: &DecodeActionRequest) -> Result<DecodeActionResponse> {
        serde_json::from_value(self.0.public_decode_action(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_execute_quote_debug(&self, request: &PublicExecuteQuoteDebugRequest) -> Result<QuoteExecuteDebugResult> {
        serde_json::from_value(self.0.public_execute_quote_debug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_all_currencies(&self, request: &EmptyRequest) -> Result<Vec<Currency>> {
        serde_json::from_value(self.0.public_get_all_currencies(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_all_instruments(&self, request: &GetAllInstrumentsRequest) -> Result<GetAllInstrumentsResponse> {
        serde_json::from_value(self.0.public_get_all_instruments(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_all_live_instruments(&self, request: &EmptyRequest) -> Result<Vec<String>> {
        serde_json::from_value(self.0.public_get_all_live_instruments(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_all_referral_codes(&self, request: &GetAllReferralCodesParams) -> Result<Vec<Referrer>> {
        serde_json::from_value(self.0.public_get_all_referral_codes(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_currency(&self, request: &GetCurrencyRequest) -> Result<Currency> {
        serde_json::from_value(self.0.public_get_currency(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_funding_rate_history(&self, request: &GetFundingRateHistoryRequest) -> Result<FundingRateHistoryResult> {
        serde_json::from_value(self.0.public_get_funding_rate_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_index_chart_data(&self, request: &GetIndexChartDataRequest) -> Result<Vec<IndexCandle>> {
        serde_json::from_value(self.0.public_get_index_chart_data(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_instrument(&self, request: &GetInstrumentRequest) -> Result<Instrument> {
        serde_json::from_value(self.0.public_get_instrument(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_interest_rate_history(&self, request: &GetInterestRateHistoryRequest) -> Result<InterestRateHistoryResult> {
        serde_json::from_value(self.0.public_get_interest_rate_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_latest_signed_feeds(&self, request: &GetLatestSignedFeedsRequest) -> Result<GetLatestSignedFeedsResponse> {
        serde_json::from_value(self.0.public_get_latest_signed_feeds(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_liquidation_history(&self, request: &GetLiquidationHistoryRequest) -> Result<LiquidationHistoryResult> {
        serde_json::from_value(self.0.public_get_liquidation_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_live_auctions(&self, request: &EmptyRequest) -> Result<PublicGetLiveAuctionsResponse> {
        serde_json::from_value(self.0.public_get_live_auctions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_live_incidents(&self, request: &GetLiveIncidentsParams) -> Result<GetLiveIncidentsResponse> {
        serde_json::from_value(self.0.public_get_live_incidents(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_maker_program_scores(&self, request: &GetMakerProgramScoresParams) -> Result<GetMakerProgramScoresResponse> {
        serde_json::from_value(self.0.public_get_maker_program_scores(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_maker_programs(&self, request: &GetMakerProgramsParams) -> Result<Vec<ProgramResponse>> {
        serde_json::from_value(self.0.public_get_maker_programs(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_margin(&self, request: &PublicGetMarginRequest) -> Result<GetMarginResponse> {
        serde_json::from_value(self.0.public_get_margin(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_onchain_action_history(&self, request: &GetOnchainActionHistoryParams) -> Result<GetOnchainActionHistoryResponse> {
        serde_json::from_value(self.0.public_get_onchain_action_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_option_settlement_prices(&self, request: &GetOptionSettlementPricesRequest) -> Result<OptionSettlementPricesResult> {
        serde_json::from_value(self.0.public_get_option_settlement_prices(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_pending_deposits(&self, request: &GetPendingDepositsParams) -> Result<GetPendingDepositsResult> {
        serde_json::from_value(self.0.public_get_pending_deposits(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_perp_impact_twap(&self, request: &GetPerpImpactTwapRequest) -> Result<PerpImpactTwapResult> {
        serde_json::from_value(self.0.public_get_perp_impact_twap(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_rate_limits(&self, request: &EmptyRequest) -> Result<RateLimitResult> {
        serde_json::from_value(self.0.public_get_rate_limits(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_referral_performance(&self, request: &GetReferralPerformanceParams) -> Result<GetReferralPerformanceResult> {
        serde_json::from_value(self.0.public_get_referral_performance(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_risk_universes(&self, request: &EmptyRequest) -> Result<Vec<RiskUniverse>> {
        serde_json::from_value(self.0.public_get_risk_universes(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_subaccounts(&self, request: &PublicGetSubaccountsRequest) -> Result<PrivateGetSubaccountsResponse> {
        serde_json::from_value(self.0.public_get_subaccounts(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_ticker(&self, request: &GetTickerRequest) -> Result<TickerSlimSnapshot> {
        serde_json::from_value(self.0.public_get_ticker(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_tickers(&self, request: &GetTickersRequest) -> Result<GetTickersResponse> {
        serde_json::from_value(self.0.public_get_tickers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_time(&self, request: &EmptyRequest) -> Result<serde_json::Number> {
        serde_json::from_value(self.0.public_get_time(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_trade_history(&self, request: &GetPublicTradeHistoryRequest) -> Result<PublicTradesResult> {
        serde_json::from_value(self.0.public_get_trade_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_tradingview_chart_data(&self, request: &GetTradingviewChartDataRequest) -> Result<Vec<TradingviewCandle>> {
        serde_json::from_value(self.0.public_get_tradingview_chart_data(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_transaction(&self, request: &GetTransactionParams) -> Result<GetTransactionResult> {
        serde_json::from_value(self.0.public_get_transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_vault(&self, request: &GetVaultRequest) -> Result<Vault> {
        serde_json::from_value(self.0.public_get_vault(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_vault_action_history(&self, request: &GetVaultActionHistoryRequest) -> Result<PaginatedVaultActionHistory> {
        serde_json::from_value(self.0.public_get_vault_action_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_vault_performance_history(&self, request: &GetVaultPerformanceHistoryRequest) -> Result<VaultPerformanceHistoryResult> {
        serde_json::from_value(self.0.public_get_vault_performance_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_vaults(&self, request: &GetVaultsRequest) -> Result<VaultsResponse> {
        serde_json::from_value(self.0.public_get_vaults(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_wallet(&self, request: &PublicGetWalletRequest) -> Result<PublicGetWalletResponse> {
        serde_json::from_value(self.0.public_get_wallet(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_get_wallets_from_session_key(&self, request: &GetWalletsFromSessionKeyRequest) -> Result<PublicGetWalletsFromSessionKeyResponse> {
        serde_json::from_value(self.0.public_get_wallets_from_session_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_margin_watch(&self, request: &MarginWatchRequest) -> Result<PublicMarginWatchResponse> {
        serde_json::from_value(self.0.public_margin_watch(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_order_quote(&self, request: &OrderQuoteRequest) -> Result<OrderQuoteResponse> {
        serde_json::from_value(self.0.public_order_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_register_deposit_address(&self, request: &RegisterDepositAddressParams) -> Result<RegisterDepositAddressResult> {
        serde_json::from_value(self.0.public_register_deposit_address(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_send_quote_debug(&self, request: &PublicSendQuoteDebugRequest) -> Result<QuoteSendDebugResult> {
        serde_json::from_value(self.0.public_send_quote_debug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
    pub async fn public_withdraw_debug(&self, request: &PublicWithdrawDebugRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.public_withdraw_debug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Derive response model"))
    }
}
