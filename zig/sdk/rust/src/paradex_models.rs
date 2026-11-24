// Generated from schema/paradex.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{ParadexClient, Error, Result};

fn zero_number() -> serde_json::Number { 0.into() }
pub type ResponsesAccountKind = String;
pub type ResponsesAlgoType = String;
pub type ResponsesBlockTradeStatus = String;
pub type ResponsesBlockTradeType = String;
pub type ResponsesDepositAddressStatus = String;
pub type ResponsesErrorCode = String;
pub type ResponsesFillFlag = String;
pub type ResponsesFillType = String;
pub type ResponsesMarketEventType = String;
pub type ResponsesOrderFlag = String;
pub type ResponsesOrderInstruction = String;
pub type ResponsesOrderSide = String;
pub type ResponsesOrderStatus = String;
pub type ResponsesOrderType = String;
pub type ResponsesRfqVenueState = String;
pub type ResponsesSTPMode = String;
pub type ResponsesSystemStatus = String;
pub type ResponsesTraderRole = String;
pub type ResponsesVaultKind = String;
pub type ResponsesVaultStatus = String;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ActivateSubkeyParams {
    #[serde(rename = "body")]
    pub body: RequestsActivateSubkey,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ArchiveSubAccountParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlockOffersCancelParams {
    #[serde(rename = "block_trade_id")]
    pub block_trade_id: String,
    #[serde(rename = "offer_id")]
    pub offer_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlockOffersCreateParams {
    #[serde(rename = "block_trade_id")]
    pub block_trade_id: String,
    #[serde(rename = "body")]
    pub body: RequestsBlockOfferRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlockOffersExecuteParams {
    #[serde(rename = "block_trade_id")]
    pub block_trade_id: String,
    #[serde(rename = "offer_id")]
    pub offer_id: String,
    #[serde(rename = "body")]
    pub body: RequestsBlockExecuteRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlockOffersGetParams {
    #[serde(rename = "block_trade_id")]
    pub block_trade_id: String,
    #[serde(rename = "offer_id")]
    pub offer_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlockOffersListParams {
    #[serde(rename = "block_trade_id")]
    pub block_trade_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlockTradesCancelParams {
    #[serde(rename = "block_trade_id")]
    pub block_trade_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlockTradesCreateParams {
    #[serde(rename = "body")]
    pub body: RequestsBlockTradeRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlockTradesExecuteParams {
    #[serde(rename = "block_trade_id")]
    pub block_trade_id: String,
    #[serde(rename = "body")]
    pub body: RequestsBlockExecuteRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlockTradesGetParams {
    #[serde(rename = "block_trade_id")]
    pub block_trade_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlockTradesListParams {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAlgoOrderParams {
    #[serde(rename = "algo_id")]
    pub algo_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateAlgoOrderParams {
    #[serde(rename = "body")]
    pub body: RequestsAlgoOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateDepositAddressParams {
    #[serde(rename = "body")]
    pub body: RequestsCreateDepositAddressReq,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateTokenParams {
    #[serde(rename = "body")]
    pub body: RequestsCreateToken,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateXpTransferV2Params {
    #[serde(rename = "body")]
    pub body: RequestsCreateXPTransferRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteAccountMmpParams {
    #[serde(rename = "base_asset")]
    pub base_asset: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct EvmAuthParams {
    #[serde(rename = "PARADEX-STARKNET-ACCOUNT")]
    pub paradex_starknet_account: String,
    #[serde(rename = "PARADEX-EVM-SIGNATURE")]
    pub paradex_evm_signature: String,
    #[serde(rename = "PARADEX-SIWE-MESSAGE")]
    pub paradex_siwe_message: String,
    #[serde(rename = "PARADEX-AUTHORIZE-ISOLATED-MARKETS", default, skip_serializing_if = "Option::is_none")]
    pub paradex_authorize_isolated_markets: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct EvmOnboardingParams {
    #[serde(rename = "PARADEX-STARKNET-ACCOUNT")]
    pub paradex_starknet_account: String,
    #[serde(rename = "PARADEX-EVM-SIGNATURE")]
    pub paradex_evm_signature: String,
    #[serde(rename = "PARADEX-SIWE-MESSAGE")]
    pub paradex_siwe_message: String,
    #[serde(rename = "body")]
    pub body: RequestsOnboarding,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountHistoryParams {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountMarginParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountMmpParams {
    #[serde(rename = "base_asset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountParams {
    #[serde(rename = "subaccount_address", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountProfileParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountSettingsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountsInfoParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAlgoOrderByIdParams {
    #[serde(rename = "algo_id")]
    pub algo_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAlgoOrdersHistoryParams {
    #[serde(rename = "client_id", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "end_at", default, skip_serializing_if = "Option::is_none")]
    pub end_at: Option<serde_json::Number>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "start_at", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<serde_json::Number>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAnnouncementsParams {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "end_at", default, skip_serializing_if = "Option::is_none")]
    pub end_at: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "start_at", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBalanceParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBboInteractiveParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBboParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetComplianceActionsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetDepositAddressParams {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetDimeConfigParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFillsParams {
    #[serde(rename = "asset_kind", default, skip_serializing_if = "Option::is_none")]
    pub asset_kind: Option<String>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "end_at", default, skip_serializing_if = "Option::is_none")]
    pub end_at: Option<serde_json::Number>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "start_at", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFundingDataParams {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "end_at", default, skip_serializing_if = "Option::is_none")]
    pub end_at: Option<serde_json::Number>,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "start_at", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFundingParams {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "end_at", default, skip_serializing_if = "Option::is_none")]
    pub end_at: Option<serde_json::Number>,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "start_at", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetGlobalStakingBalanceHistoryParams {
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetImpactPriceParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "size")]
    pub size: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetImpactPriceParams {
    fn default() -> Self {
        Self {
            market: Default::default(),
            size: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInsuranceFundParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetJwtParams {
    #[serde(rename = "PARADEX-STARKNET-ACCOUNT")]
    pub paradex_starknet_account: String,
    #[serde(rename = "PARADEX-STARKNET-SIGNATURE")]
    pub paradex_starknet_signature: String,
    #[serde(rename = "PARADEX-TIMESTAMP")]
    pub paradex_timestamp: String,
    #[serde(rename = "PARADEX-SIGNATURE-EXPIRATION", default, skip_serializing_if = "Option::is_none")]
    pub paradex_signature_expiration: Option<String>,
    #[serde(rename = "PARADEX-AUTHORIZE-ISOLATED-MARKETS", default, skip_serializing_if = "Option::is_none")]
    pub paradex_authorize_isolated_markets: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLiquidationsParams {
    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<serde_json::Number>,
    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginConfigParams {
    #[serde(rename = "base_asset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketSettlementPriceParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketsHistoryParams {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketsParams {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "asset_kind", default, skip_serializing_if = "Option::is_none")]
    pub asset_kind: Option<String>,
    #[serde(rename = "base_asset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketsSummaryParams {
    #[serde(rename = "asset_kind", default, skip_serializing_if = "Option::is_none")]
    pub asset_kind: Option<String>,
    #[serde(rename = "base_asset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    #[serde(rename = "end", default, skip_serializing_if = "Option::is_none")]
    pub end: Option<serde_json::Number>,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "start", default, skip_serializing_if = "Option::is_none")]
    pub start: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOnboardingParams {
    #[serde(rename = "public_key", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "eth_address", default, skip_serializing_if = "Option::is_none")]
    pub eth_address: Option<String>,
    #[serde(rename = "account_signer_type")]
    pub account_signer_type: String,
    #[serde(rename = "vault_operator_index", default, skip_serializing_if = "Option::is_none")]
    pub vault_operator_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOpenAlgoOrdersParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOpenOrdersParams {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderbookInteractiveParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "depth", default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<serde_json::Number>,
    #[serde(rename = "price_tick", default, skip_serializing_if = "Option::is_none")]
    pub price_tick: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderbookParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "depth", default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<serde_json::Number>,
    #[serde(rename = "price_tick", default, skip_serializing_if = "Option::is_none")]
    pub price_tick: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrdersParams {
    #[serde(rename = "client_id", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "end_at", default, skip_serializing_if = "Option::is_none")]
    pub end_at: Option<serde_json::Number>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "start_at", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<serde_json::Number>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPortfolioMarginConfigParams {
    #[serde(rename = "base_asset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPositionsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPublicXpTransfersV2Params {
    #[serde(rename = "start_at", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<serde_json::Number>,
    #[serde(rename = "end_at", default, skip_serializing_if = "Option::is_none")]
    pub end_at: Option<serde_json::Number>,
    #[serde(rename = "season", default, skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetReferralQrCodeParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetReferralsConfigParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetReferralsSummaryParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetStakingBalanceHistoryParams {
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetStakingConfigParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetStakingHistoryParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetStakingSummaryParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSubAccountsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSubkeyJwtParams {
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "PARADEX-STARKNET-ACCOUNT")]
    pub paradex_starknet_account: String,
    #[serde(rename = "PARADEX-STARKNET-SIGNATURE")]
    pub paradex_starknet_signature: String,
    #[serde(rename = "PARADEX-TIMESTAMP")]
    pub paradex_timestamp: String,
    #[serde(rename = "PARADEX-SIGNATURE-EXPIRATION", default, skip_serializing_if = "Option::is_none")]
    pub paradex_signature_expiration: Option<String>,
    #[serde(rename = "PARADEX-AUTHORIZE-ISOLATED-MARKETS", default, skip_serializing_if = "Option::is_none")]
    pub paradex_authorize_isolated_markets: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSubkeyParams {
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSystemConfigParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSystemStateParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSystemStatsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSystemTimeParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTransferFeeConfigV2Params {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTransfersParams {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "end_at", default, skip_serializing_if = "Option::is_none")]
    pub end_at: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "start_at", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<serde_json::Number>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetVolumeTiersParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetXpBalanceV2Params {
    #[serde(rename = "season", default, skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetXpTransferByIdV2Params {
    #[serde(rename = "transfer_id")]
    pub transfer_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetXpTransfersV2Params {
    #[serde(rename = "role", default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "season", default, skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct JwksParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListDepositNetworksParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListSubkeysParams {
    #[serde(rename = "with_revoked", default, skip_serializing_if = "Option::is_none")]
    pub with_revoked: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListTokensParams {
    #[serde(rename = "kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "with_invalid", default, skip_serializing_if = "Option::is_none")]
    pub with_invalid: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MarketsKlinesParams {
    #[serde(rename = "end_at")]
    pub end_at: serde_json::Number,
    #[serde(rename = "price_kind", default, skip_serializing_if = "Option::is_none")]
    pub price_kind: Option<String>,
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(rename = "start_at")]
    pub start_at: serde_json::Number,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for MarketsKlinesParams {
    fn default() -> Self {
        Self {
            end_at: 0.into(),
            price_kind: Default::default(),
            resolution: Default::default(),
            start_at: 0.into(),
            symbol: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Messagesv1NftPrice {
    #[serde(rename = "currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "decimals", default, skip_serializing_if = "Option::is_none")]
    pub decimals: Option<serde_json::Number>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OnboardingParams {
    #[serde(rename = "PARADEX-ETHEREUM-ACCOUNT")]
    pub paradex_ethereum_account: String,
    #[serde(rename = "PARADEX-STARKNET-ACCOUNT")]
    pub paradex_starknet_account: String,
    #[serde(rename = "PARADEX-STARKNET-SIGNATURE")]
    pub paradex_starknet_signature: String,
    #[serde(rename = "PARADEX-PARENT-SIWE-MESSAGE", default, skip_serializing_if = "Option::is_none")]
    pub paradex_parent_siwe_message: Option<String>,
    #[serde(rename = "PARADEX-PARENT-SIGNATURE", default, skip_serializing_if = "Option::is_none")]
    pub paradex_parent_signature: Option<String>,
    #[serde(rename = "body")]
    pub body: RequestsOnboarding,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrdersCancelAllParams {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrdersCancelBatchParams {
    #[serde(rename = "body")]
    pub body: RequestsCancelOrderBatchRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrdersCancelByClientIdParams {
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrdersCancelParams {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrdersGetByClientIdParams {
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrdersGetParams {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrdersModifyParams {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "body")]
    pub body: RequestsModifyOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrdersNewBatchParams {
    #[serde(rename = "body")]
    pub body: Vec<RequestsOrderRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrdersNewParams {
    #[serde(rename = "body")]
    pub body: RequestsOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PostAccountTerminalTokenParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RecordComplianceActionParams {
    #[serde(rename = "body")]
    pub body: RequestsRecordComplianceActionRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RefreshAccountInventoryParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RegisterSubkeyParams {
    #[serde(rename = "body")]
    pub body: RequestsCreateSubkey,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestsAccountMarginRequest {
    #[serde(rename = "leverage")]
    pub leverage: serde_json::Number,
    #[serde(rename = "margin_type")]
    pub margin_type: String,
    #[serde(rename = "on_behalf_of_account", default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of_account: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RequestsAccountMarginRequest {
    fn default() -> Self {
        Self {
            leverage: 0.into(),
            margin_type: Default::default(),
            on_behalf_of_account: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestsAccountMmpConfigRequest {
    #[serde(rename = "delta_limit", default, skip_serializing_if = "Option::is_none")]
    pub delta_limit: Option<String>,
    #[serde(rename = "frozen_time_ms")]
    pub frozen_time_ms: serde_json::Number,
    #[serde(rename = "interval_ms")]
    pub interval_ms: serde_json::Number,
    #[serde(rename = "protect_block_trades", default, skip_serializing_if = "Option::is_none")]
    pub protect_block_trades: Option<bool>,
    #[serde(rename = "size_limit", default, skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<String>,
    #[serde(rename = "vega_limit", default, skip_serializing_if = "Option::is_none")]
    pub vega_limit: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RequestsAccountMmpConfigRequest {
    fn default() -> Self {
        Self {
            delta_limit: Default::default(),
            frozen_time_ms: 0.into(),
            interval_ms: 0.into(),
            protect_block_trades: Default::default(),
            size_limit: Default::default(),
            vega_limit: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsAccountMmpResetRequest {
    #[serde(rename = "base_asset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestsActivateSubkey {
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "signature", default)]
    pub signature: Vec<String>,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RequestsActivateSubkey {
    fn default() -> Self {
        Self {
            account_id: Default::default(),
            name: Default::default(),
            public_key: Default::default(),
            signature: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsActivateSubkeyResponse {
    #[serde(rename = "encrypted_key", default, skip_serializing_if = "Option::is_none")]
    pub encrypted_key: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestsAlgoOrderRequest {
    #[serde(rename = "algo_type")]
    pub algo_type: String,
    #[serde(rename = "dime_discount", default, skip_serializing_if = "Option::is_none")]
    pub dime_discount: Option<bool>,
    #[serde(rename = "duration_seconds")]
    pub duration_seconds: serde_json::Number,
    #[serde(rename = "frequency", default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<serde_json::Number>,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "on_behalf_of_account", default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of_account: Option<String>,
    #[serde(rename = "recv_window", default, skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<serde_json::Number>,
    #[serde(rename = "side")]
    pub side: ResponsesOrderSide,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_timestamp", default = "zero_number")]
    pub signature_timestamp: serde_json::Number,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "type")]
    pub type_: ResponsesOrderType,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RequestsAlgoOrderRequest {
    fn default() -> Self {
        Self {
            algo_type: Default::default(),
            dime_discount: Default::default(),
            duration_seconds: 0.into(),
            frequency: Default::default(),
            market: Default::default(),
            on_behalf_of_account: Default::default(),
            recv_window: Default::default(),
            side: Default::default(),
            signature: Default::default(),
            signature_timestamp: 0.into(),
            size: Default::default(),
            type_: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsBlockExecuteRequest {
    #[serde(rename = "execution_nonce")]
    pub execution_nonce: String,
    #[serde(rename = "selected_offers", default, skip_serializing_if = "Option::is_none")]
    pub selected_offers: Option<Vec<String>>,
    #[serde(rename = "signatures")]
    pub signatures: std::collections::BTreeMap<String,ResponsesBlockTradeSignature>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsBlockOfferInfo {
    #[serde(rename = "offerer_order")]
    pub offerer_order: ResponsesBlockTradeOrder,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsBlockOfferRequest {
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "offering_account")]
    pub offering_account: String,
    #[serde(rename = "signature", default)]
    pub signature: ResponsesBlockTradeSignature,
    #[serde(rename = "trades")]
    pub trades: std::collections::BTreeMap<String,RequestsBlockOfferInfo>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsBlockTradeInfo {
    #[serde(rename = "maker_order", default, skip_serializing_if = "Option::is_none")]
    pub maker_order: Option<ResponsesBlockTradeOrder>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "taker_order", default, skip_serializing_if = "Option::is_none")]
    pub taker_order: Option<ResponsesBlockTradeOrder>,
    #[serde(rename = "trade_constraints", default, skip_serializing_if = "Option::is_none")]
    pub trade_constraints: Option<ResponsesBlockTradeConstraints>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestsBlockTradeRequest {
    #[serde(rename = "block_expiration")]
    pub block_expiration: serde_json::Number,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "required_signers")]
    pub required_signers: Vec<String>,
    #[serde(rename = "signatures")]
    pub signatures: std::collections::BTreeMap<String,ResponsesBlockTradeSignature>,
    #[serde(rename = "trades")]
    pub trades: std::collections::BTreeMap<String,RequestsBlockTradeInfo>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RequestsBlockTradeRequest {
    fn default() -> Self {
        Self {
            block_expiration: 0.into(),
            nonce: Default::default(),
            required_signers: Default::default(),
            signatures: Default::default(),
            trades: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsCancelOrderBatchRequest {
    #[serde(rename = "client_order_ids", default, skip_serializing_if = "Option::is_none")]
    pub client_order_ids: Option<Vec<String>>,
    #[serde(rename = "order_ids", default, skip_serializing_if = "Option::is_none")]
    pub order_ids: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsCreateDepositAddressReq {
    #[serde(rename = "network")]
    pub network: String,
    #[serde(rename = "token", default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsCreateSubkey {
    #[serde(rename = "allowed_cidrs", default, skip_serializing_if = "Option::is_none")]
    pub allowed_cidrs: Option<Vec<String>>,
    #[serde(rename = "encrypted_key", default, skip_serializing_if = "Option::is_none")]
    pub encrypted_key: Option<String>,
    #[serde(rename = "eph_public_key", default, skip_serializing_if = "Option::is_none")]
    pub eph_public_key: Option<String>,
    #[serde(rename = "evm_signature", default, skip_serializing_if = "Option::is_none")]
    pub evm_signature: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "signature", default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<Vec<String>>,
    #[serde(rename = "signature_expiry", default, skip_serializing_if = "Option::is_none")]
    pub signature_expiry: Option<serde_json::Number>,
    #[serde(rename = "signature_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub signature_timestamp: Option<serde_json::Number>,
    #[serde(rename = "siwe_message", default, skip_serializing_if = "Option::is_none")]
    pub siwe_message: Option<String>,
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestsCreateToken {
    #[serde(rename = "allowed_cidrs", default, skip_serializing_if = "Option::is_none")]
    pub allowed_cidrs: Option<Vec<String>>,
    #[serde(rename = "expiry_duration")]
    pub expiry_duration: serde_json::Number,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RequestsCreateToken {
    fn default() -> Self {
        Self {
            allowed_cidrs: Default::default(),
            expiry_duration: 0.into(),
            name: Default::default(),
            token_type: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsCreateVault {
    #[serde(rename = "curator_name", default, skip_serializing_if = "Option::is_none")]
    pub curator_name: Option<String>,
    #[serde(rename = "deposit_tx_signature", default, skip_serializing_if = "Option::is_none")]
    pub deposit_tx_signature: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lockup_period", default, skip_serializing_if = "Option::is_none")]
    pub lockup_period: Option<serde_json::Number>,
    #[serde(rename = "max_tvl", default, skip_serializing_if = "Option::is_none")]
    pub max_tvl: Option<serde_json::Number>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "operator_eth_address", default, skip_serializing_if = "Option::is_none")]
    pub operator_eth_address: Option<String>,
    #[serde(rename = "operator_index", default, skip_serializing_if = "Option::is_none")]
    pub operator_index: Option<serde_json::Number>,
    #[serde(rename = "operator_signer_type", default, skip_serializing_if = "Option::is_none")]
    pub operator_signer_type: Option<String>,
    #[serde(rename = "profit_share", default, skip_serializing_if = "Option::is_none")]
    pub profit_share: Option<serde_json::Number>,
    #[serde(rename = "public_key", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "show_balances", default, skip_serializing_if = "Option::is_none")]
    pub show_balances: Option<bool>,
    #[serde(rename = "show_positions", default, skip_serializing_if = "Option::is_none")]
    pub show_positions: Option<bool>,
    #[serde(rename = "strategy_description", default, skip_serializing_if = "Option::is_none")]
    pub strategy_description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestsCreateXPTransferRequest {
    #[serde(rename = "amount")]
    pub amount: serde_json::Number,
    #[serde(rename = "is_private", default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(rename = "recipient")]
    pub recipient: String,
    #[serde(rename = "season", default, skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RequestsCreateXPTransferRequest {
    fn default() -> Self {
        Self {
            amount: 0.into(),
            is_private: Default::default(),
            recipient: Default::default(),
            season: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestsModifyOrderRequest {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "on_behalf_of_account", default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of_account: Option<String>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_timestamp", default = "zero_number")]
    pub signature_timestamp: serde_json::Number,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "trigger_price", default, skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RequestsModifyOrderRequest {
    fn default() -> Self {
        Self {
            id: Default::default(),
            market: Default::default(),
            on_behalf_of_account: Default::default(),
            price: Default::default(),
            side: Default::default(),
            signature: Default::default(),
            signature_timestamp: 0.into(),
            size: Default::default(),
            trigger_price: Default::default(),
            type_: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsOnboarding {
    #[serde(rename = "marketing_code", default, skip_serializing_if = "Option::is_none")]
    pub marketing_code: Option<String>,
    #[serde(rename = "portfolio_margin", default, skip_serializing_if = "Option::is_none")]
    pub portfolio_margin: Option<bool>,
    #[serde(rename = "public_key", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "referral_code", default, skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(rename = "utm", default, skip_serializing_if = "Option::is_none")]
    pub utm: Option<RequestsUtm>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestsOrderRequest {
    #[serde(rename = "client_id", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "dime_discount", default, skip_serializing_if = "Option::is_none")]
    pub dime_discount: Option<bool>,
    #[serde(rename = "flags", default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<ResponsesOrderFlag>>,
    #[serde(rename = "instruction")]
    pub instruction: ResponsesOrderInstruction,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "on_behalf_of_account", default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of_account: Option<String>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "recv_window", default, skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<serde_json::Number>,
    #[serde(rename = "side")]
    pub side: ResponsesOrderSide,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signature_timestamp", default = "zero_number")]
    pub signature_timestamp: serde_json::Number,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "stp", default, skip_serializing_if = "Option::is_none")]
    pub stp: Option<String>,
    #[serde(rename = "trigger_price", default, skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "type")]
    pub type_: ResponsesOrderType,
    #[serde(rename = "vwap_price", default, skip_serializing_if = "Option::is_none")]
    pub vwap_price: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for RequestsOrderRequest {
    fn default() -> Self {
        Self {
            client_id: Default::default(),
            dime_discount: Default::default(),
            flags: Default::default(),
            instruction: Default::default(),
            market: Default::default(),
            on_behalf_of_account: Default::default(),
            price: Default::default(),
            recv_window: Default::default(),
            side: Default::default(),
            signature: Default::default(),
            signature_timestamp: 0.into(),
            size: Default::default(),
            stp: Default::default(),
            trigger_price: Default::default(),
            type_: Default::default(),
            vwap_price: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsRecordComplianceActionRequest {
    #[serde(rename = "action_type")]
    pub action_type: String,
    #[serde(rename = "action_version")]
    pub action_version: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsRfqCreateRequest {
    #[serde(rename = "counterparties", default, skip_serializing_if = "Option::is_none")]
    pub counterparties: Option<Vec<String>>,
    #[serde(rename = "is_anonymous", default, skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "legs")]
    pub legs: Vec<RequestsRfqLeg>,
    #[serde(rename = "max_slippage", default, skip_serializing_if = "Option::is_none")]
    pub max_slippage: Option<String>,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "strategy", default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsRfqExecuteRequest {
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsRfqLeg {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "ratio")]
    pub ratio: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsUpdateAccountMaxSlippageRequest {
    #[serde(rename = "max_slippage")]
    pub max_slippage: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsUpdateAccountProfileRequest {
    #[serde(rename = "ai_agent_url", default, skip_serializing_if = "Option::is_none")]
    pub ai_agent_url: Option<String>,
    #[serde(rename = "announcements", default, skip_serializing_if = "Option::is_none")]
    pub announcements: Option<bool>,
    #[serde(rename = "fill_sound", default, skip_serializing_if = "Option::is_none")]
    pub fill_sound: Option<bool>,
    #[serde(rename = "fills", default, skip_serializing_if = "Option::is_none")]
    pub fills: Option<bool>,
    #[serde(rename = "is_username_private", default, skip_serializing_if = "Option::is_none")]
    pub is_username_private: Option<bool>,
    #[serde(rename = "max_slippage", default, skip_serializing_if = "Option::is_none")]
    pub max_slippage: Option<String>,
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<bool>,
    #[serde(rename = "referral_code", default, skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(rename = "referred_by", default, skip_serializing_if = "Option::is_none")]
    pub referred_by: Option<String>,
    #[serde(rename = "size_currency_display", default, skip_serializing_if = "Option::is_none")]
    pub size_currency_display: Option<String>,
    #[serde(rename = "tap_share_rate", default, skip_serializing_if = "Option::is_none")]
    pub tap_share_rate: Option<String>,
    #[serde(rename = "trading_value_display", default, skip_serializing_if = "Option::is_none")]
    pub trading_value_display: Option<String>,
    #[serde(rename = "transfers", default, skip_serializing_if = "Option::is_none")]
    pub transfers: Option<bool>,
    #[serde(rename = "username", default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "xp_share_rate", default, skip_serializing_if = "Option::is_none")]
    pub xp_share_rate: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsUpdateAllowedCIDRs {
    #[serde(rename = "allowed_cidrs", default, skip_serializing_if = "Option::is_none")]
    pub allowed_cidrs: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsUpdateNotificationPreferencesRequest {
    #[serde(rename = "announcements", default, skip_serializing_if = "Option::is_none")]
    pub announcements: Option<bool>,
    #[serde(rename = "fill_sound", default, skip_serializing_if = "Option::is_none")]
    pub fill_sound: Option<bool>,
    #[serde(rename = "fills", default, skip_serializing_if = "Option::is_none")]
    pub fills: Option<bool>,
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<bool>,
    #[serde(rename = "transfers", default, skip_serializing_if = "Option::is_none")]
    pub transfers: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsUpdateSizeCurrencyDisplayRequest {
    #[serde(rename = "size_currency_display")]
    pub size_currency_display: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsUpdateSubkey {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsUpdateSubkeyBlockTradeSigning {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsUpdateTradingValueDisplayRequest {
    #[serde(rename = "trading_value_display")]
    pub trading_value_display: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestsUtm {
    #[serde(rename = "campaign", default, skip_serializing_if = "Option::is_none")]
    pub campaign: Option<String>,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResetAccountMmpParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<RequestsAccountMmpResetRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAccountAnalytics {
    #[serde(rename = "account_address", default, skip_serializing_if = "Option::is_none")]
    pub account_address: Option<String>,
    #[serde(rename = "annualized_volatility", default, skip_serializing_if = "Option::is_none")]
    pub annualized_volatility: Option<ResponsesWindowedMetrics>,
    #[serde(rename = "apr", default, skip_serializing_if = "Option::is_none")]
    pub apr: Option<ResponsesWindowedMetrics>,
    #[serde(rename = "apr_simple_itd", default, skip_serializing_if = "Option::is_none")]
    pub apr_simple_itd: Option<String>,
    #[serde(rename = "cum_return", default, skip_serializing_if = "Option::is_none")]
    pub cum_return: Option<String>,
    #[serde(rename = "cumulative_pnl_series", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_pnl_series: Option<Vec<ResponsesAccountDailyPnlPoint>>,
    #[serde(rename = "cumulative_pnl_usd", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_pnl_usd: Option<String>,
    #[serde(rename = "end_value", default, skip_serializing_if = "Option::is_none")]
    pub end_value: Option<String>,
    #[serde(rename = "hourly_cumulative_pnl_series", default, skip_serializing_if = "Option::is_none")]
    pub hourly_cumulative_pnl_series: Option<Vec<ResponsesAccountHourlyPnlPoint>>,
    #[serde(rename = "max_dd", default, skip_serializing_if = "Option::is_none")]
    pub max_dd: Option<ResponsesWindowedMetrics>,
    #[serde(rename = "max_drawdown_to_date", default, skip_serializing_if = "Option::is_none")]
    pub max_drawdown_to_date: Option<String>,
    #[serde(rename = "monthly_pnl_last_12m", default, skip_serializing_if = "Option::is_none")]
    pub monthly_pnl_last_12m: Option<Vec<ResponsesAccountMonthlyPnlEntry>>,
    #[serde(rename = "pnl", default, skip_serializing_if = "Option::is_none")]
    pub pnl: Option<ResponsesWindowedMetrics>,
    #[serde(rename = "return", default, skip_serializing_if = "Option::is_none")]
    pub return_: Option<ResponsesWindowedMetrics>,
    #[serde(rename = "sharpe", default, skip_serializing_if = "Option::is_none")]
    pub sharpe: Option<ResponsesWindowedMetrics>,
    #[serde(rename = "sharpe_annualized_itd", default, skip_serializing_if = "Option::is_none")]
    pub sharpe_annualized_itd: Option<String>,
    #[serde(rename = "strategy_name", default, skip_serializing_if = "Option::is_none")]
    pub strategy_name: Option<String>,
    #[serde(rename = "vol_annualized_itd", default, skip_serializing_if = "Option::is_none")]
    pub vol_annualized_itd: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAccountDailyPnlPoint {
    #[serde(rename = "day", default, skip_serializing_if = "Option::is_none")]
    pub day: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAccountHistoricalDataResp {
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<serde_json::Number>>,
    #[serde(rename = "timestamps", default, skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Vec<serde_json::Number>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAccountHourlyPnlPoint {
    #[serde(rename = "at", default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAccountInfoResponse {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "archived_at", default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<serde_json::Number>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(rename = "derivation_path", default, skip_serializing_if = "Option::is_none")]
    pub derivation_path: Option<String>,
    #[serde(rename = "fees", default, skip_serializing_if = "Option::is_none")]
    pub fees: Option<ResponsesFees>,
    #[serde(rename = "has_ecosystem_nft", default, skip_serializing_if = "Option::is_none")]
    pub has_ecosystem_nft: Option<bool>,
    #[serde(rename = "isolated_market", default, skip_serializing_if = "Option::is_none")]
    pub isolated_market: Option<String>,
    #[serde(rename = "kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ResponsesAccountKind>,
    #[serde(rename = "parent_account", default, skip_serializing_if = "Option::is_none")]
    pub parent_account: Option<String>,
    #[serde(rename = "public_key", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "username", default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "volume_tiers", default, skip_serializing_if = "Option::is_none")]
    pub volume_tiers: Option<std::collections::BTreeMap<String,ResponsesVolumeTierKindInfo>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAccountMarginEntry {
    #[serde(rename = "isolated_margin_leverage", default, skip_serializing_if = "Option::is_none")]
    pub isolated_margin_leverage: Option<serde_json::Number>,
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    pub leverage: Option<serde_json::Number>,
    #[serde(rename = "margin_type", default, skip_serializing_if = "Option::is_none")]
    pub margin_type: Option<String>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAccountMmpConfigResp {
    #[serde(rename = "base_asset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    #[serde(rename = "delta_limit", default, skip_serializing_if = "Option::is_none")]
    pub delta_limit: Option<String>,
    #[serde(rename = "frozen_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub frozen_time_ms: Option<serde_json::Number>,
    #[serde(rename = "frozen_until", default, skip_serializing_if = "Option::is_none")]
    pub frozen_until: Option<serde_json::Number>,
    #[serde(rename = "interval_ms", default, skip_serializing_if = "Option::is_none")]
    pub interval_ms: Option<serde_json::Number>,
    #[serde(rename = "is_frozen", default, skip_serializing_if = "Option::is_none")]
    pub is_frozen: Option<bool>,
    #[serde(rename = "protect_block_trades", default, skip_serializing_if = "Option::is_none")]
    pub protect_block_trades: Option<bool>,
    #[serde(rename = "size_limit", default, skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<String>,
    #[serde(rename = "updated_at", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<serde_json::Number>,
    #[serde(rename = "vega_limit", default, skip_serializing_if = "Option::is_none")]
    pub vega_limit: Option<String>,
    #[serde(rename = "window_delta", default, skip_serializing_if = "Option::is_none")]
    pub window_delta: Option<String>,
    #[serde(rename = "window_size", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
    #[serde(rename = "window_vega", default, skip_serializing_if = "Option::is_none")]
    pub window_vega: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAccountMmpResetResp {
    #[serde(rename = "results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ResponsesAccountMmpResetResult>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAccountMmpResetResult {
    #[serde(rename = "base_asset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    #[serde(rename = "reset_at", default, skip_serializing_if = "Option::is_none")]
    pub reset_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAccountMonthlyPnlEntry {
    #[serde(rename = "month", default, skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,
    #[serde(rename = "month_return_canonical", default, skip_serializing_if = "Option::is_none")]
    pub month_return_canonical: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAccountProfileResp {
    #[serde(rename = "ai_agent_url", default, skip_serializing_if = "Option::is_none")]
    pub ai_agent_url: Option<String>,
    #[serde(rename = "discord", default, skip_serializing_if = "Option::is_none")]
    pub discord: Option<ResponsesDiscordProfile>,
    #[serde(rename = "is_username_private", default, skip_serializing_if = "Option::is_none")]
    pub is_username_private: Option<bool>,
    #[serde(rename = "last_seen_notification", default, skip_serializing_if = "Option::is_none")]
    pub last_seen_notification: Option<serde_json::Number>,
    #[serde(rename = "market_max_slippage", default, skip_serializing_if = "Option::is_none")]
    pub market_max_slippage: Option<std::collections::BTreeMap<String,String>>,
    #[serde(rename = "marketed_by", default, skip_serializing_if = "Option::is_none")]
    pub marketed_by: Option<String>,
    #[serde(rename = "nfts", default, skip_serializing_if = "Option::is_none")]
    pub nfts: Option<Vec<ResponsesNft>>,
    #[serde(rename = "notifications", default, skip_serializing_if = "Option::is_none")]
    pub notifications: Option<ResponsesNotificationPreferencesResp>,
    #[serde(rename = "referral", default, skip_serializing_if = "Option::is_none")]
    pub referral: Option<ResponsesReferralConfigResp>,
    #[serde(rename = "referral_code", default, skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(rename = "referred_by", default, skip_serializing_if = "Option::is_none")]
    pub referred_by: Option<String>,
    #[serde(rename = "size_currency_display", default, skip_serializing_if = "Option::is_none")]
    pub size_currency_display: Option<String>,
    #[serde(rename = "tap_share_rate", default, skip_serializing_if = "Option::is_none")]
    pub tap_share_rate: Option<ResponsesShareRateResp>,
    #[serde(rename = "tap_status", default, skip_serializing_if = "Option::is_none")]
    pub tap_status: Option<String>,
    #[serde(rename = "twitter", default, skip_serializing_if = "Option::is_none")]
    pub twitter: Option<ResponsesTwitterProfile>,
    #[serde(rename = "twitter_following", default, skip_serializing_if = "Option::is_none")]
    pub twitter_following: Option<std::collections::BTreeMap<String,bool>>,
    #[serde(rename = "username", default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "xp_share_rate", default, skip_serializing_if = "Option::is_none")]
    pub xp_share_rate: Option<ResponsesShareRateResp>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAccountSettingsResp {
    #[serde(rename = "trading_value_display", default, skip_serializing_if = "Option::is_none")]
    pub trading_value_display: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAccountSummaryResponse {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "account_value", default, skip_serializing_if = "Option::is_none")]
    pub account_value: Option<String>,
    #[serde(rename = "free_collateral", default, skip_serializing_if = "Option::is_none")]
    pub free_collateral: Option<String>,
    #[serde(rename = "initial_margin_requirement", default, skip_serializing_if = "Option::is_none")]
    pub initial_margin_requirement: Option<String>,
    #[serde(rename = "last_seen_notification", default, skip_serializing_if = "Option::is_none")]
    pub last_seen_notification: Option<serde_json::Number>,
    #[serde(rename = "maintenance_margin_requirement", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_margin_requirement: Option<String>,
    #[serde(rename = "margin_cushion", default, skip_serializing_if = "Option::is_none")]
    pub margin_cushion: Option<String>,
    #[serde(rename = "seq_no", default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<serde_json::Number>,
    #[serde(rename = "settlement_asset", default, skip_serializing_if = "Option::is_none")]
    pub settlement_asset: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "total_collateral", default, skip_serializing_if = "Option::is_none")]
    pub total_collateral: Option<String>,
    #[serde(rename = "updated_at", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<serde_json::Number>,
    #[serde(rename = "withdrawal_lock", default, skip_serializing_if = "Option::is_none")]
    pub withdrawal_lock: Option<ResponsesWithdrawalLockResponse>,
    #[serde(rename = "withdrawal_lock_rejection", default, skip_serializing_if = "Option::is_none")]
    pub withdrawal_lock_rejection: Option<ResponsesWithdrawalLockRejectionResponse>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAlgoOrderResp {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "algo_params", default, skip_serializing_if = "Option::is_none")]
    pub algo_params: Option<std::collections::BTreeMap<String,String>>,
    #[serde(rename = "algo_type", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<ResponsesAlgoType>,
    #[serde(rename = "avg_fill_price", default, skip_serializing_if = "Option::is_none")]
    pub avg_fill_price: Option<String>,
    #[serde(rename = "cancel_reason", default, skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(rename = "end_at", default, skip_serializing_if = "Option::is_none")]
    pub end_at: Option<serde_json::Number>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "last_updated_at", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<serde_json::Number>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "remaining_size", default, skip_serializing_if = "Option::is_none")]
    pub remaining_size: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<ResponsesOrderSide>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponsesOrderStatus>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesApiToken {
    #[serde(rename = "allowed_cidrs", default, skip_serializing_if = "Option::is_none")]
    pub allowed_cidrs: Option<Vec<String>>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(rename = "expiry_at", default, skip_serializing_if = "Option::is_none")]
    pub expiry_at: Option<serde_json::Number>,
    #[serde(rename = "kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "last_updated_at", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<serde_json::Number>,
    #[serde(rename = "lookup_id", default, skip_serializing_if = "Option::is_none")]
    pub lookup_id: Option<String>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "revoked_at", default, skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<serde_json::Number>,
    #[serde(rename = "token_id", default, skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAskBidArray {
    #[serde(rename = "asks", default, skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<Vec<String>>>,
    #[serde(rename = "best_ask_api", default, skip_serializing_if = "Option::is_none")]
    pub best_ask_api: Option<Vec<String>>,
    #[serde(rename = "best_ask_interactive", default, skip_serializing_if = "Option::is_none")]
    pub best_ask_interactive: Option<Vec<String>>,
    #[serde(rename = "best_bid_api", default, skip_serializing_if = "Option::is_none")]
    pub best_bid_api: Option<Vec<String>>,
    #[serde(rename = "best_bid_interactive", default, skip_serializing_if = "Option::is_none")]
    pub best_bid_interactive: Option<Vec<String>>,
    #[serde(rename = "bids", default, skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<Vec<String>>>,
    #[serde(rename = "last_updated_at", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<serde_json::Number>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "seq_no", default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesAuthResp {
    #[serde(rename = "jwt_token", default, skip_serializing_if = "Option::is_none")]
    pub jwt_token: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesBBOResp {
    #[serde(rename = "ask", default, skip_serializing_if = "Option::is_none")]
    pub ask: Option<String>,
    #[serde(rename = "ask_size", default, skip_serializing_if = "Option::is_none")]
    pub ask_size: Option<String>,
    #[serde(rename = "bid", default, skip_serializing_if = "Option::is_none")]
    pub bid: Option<String>,
    #[serde(rename = "bid_size", default, skip_serializing_if = "Option::is_none")]
    pub bid_size: Option<String>,
    #[serde(rename = "last_updated_at", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<serde_json::Number>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "seq_no", default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesBatchResponse {
    #[serde(rename = "errors", default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ResponsesErrorResponse>>,
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<ResponsesOrderResp>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesBlockTradeConstraints {
    #[serde(rename = "max_price", default, skip_serializing_if = "Option::is_none")]
    pub max_price: Option<String>,
    #[serde(rename = "max_size", default, skip_serializing_if = "Option::is_none")]
    pub max_size: Option<String>,
    #[serde(rename = "min_price", default, skip_serializing_if = "Option::is_none")]
    pub min_price: Option<String>,
    #[serde(rename = "min_size", default, skip_serializing_if = "Option::is_none")]
    pub min_size: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesBlockTradeDetailFullResponse {
    #[serde(rename = "block_expiration", default, skip_serializing_if = "Option::is_none")]
    pub block_expiration: Option<serde_json::Number>,
    #[serde(rename = "block_id", default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(rename = "block_type", default, skip_serializing_if = "Option::is_none")]
    pub block_type: Option<ResponsesBlockTradeType>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(rename = "failure_reason", default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "initiator", default, skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    #[serde(rename = "last_updated_at", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<serde_json::Number>,
    #[serde(rename = "nonce", default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(rename = "parent_block_id", default, skip_serializing_if = "Option::is_none")]
    pub parent_block_id: Option<String>,
    #[serde(rename = "required_signers", default, skip_serializing_if = "Option::is_none")]
    pub required_signers: Option<Vec<String>>,
    #[serde(rename = "signatures", default, skip_serializing_if = "Option::is_none")]
    pub signatures: Option<std::collections::BTreeMap<String,ResponsesBlockTradeSignature>>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponsesBlockTradeStatus>,
    #[serde(rename = "trades", default, skip_serializing_if = "Option::is_none")]
    pub trades: Option<std::collections::BTreeMap<String,ResponsesBlockTradeDetailResponse>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesBlockTradeDetailResponse {
    #[serde(rename = "executed_at", default, skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<serde_json::Number>,
    #[serde(rename = "failure_reason", default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "maker_account", default, skip_serializing_if = "Option::is_none")]
    pub maker_account: Option<String>,
    #[serde(rename = "maker_fill", default, skip_serializing_if = "Option::is_none")]
    pub maker_fill: Option<ResponsesFillResult>,
    #[serde(rename = "maker_order", default, skip_serializing_if = "Option::is_none")]
    pub maker_order: Option<ResponsesBlockTradeOrder>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "taker_account", default, skip_serializing_if = "Option::is_none")]
    pub taker_account: Option<String>,
    #[serde(rename = "taker_fill", default, skip_serializing_if = "Option::is_none")]
    pub taker_fill: Option<ResponsesFillResult>,
    #[serde(rename = "taker_order", default, skip_serializing_if = "Option::is_none")]
    pub taker_order: Option<ResponsesBlockTradeOrder>,
    #[serde(rename = "trade_constraints", default, skip_serializing_if = "Option::is_none")]
    pub trade_constraints: Option<ResponsesBlockTradeConstraints>,
    #[serde(rename = "trade_id", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesBlockTradeOrder {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "client_id", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "flags", default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<ResponsesOrderFlag>>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "side")]
    pub side: ResponsesOrderSide,
    #[serde(rename = "signature", default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "signature_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub signature_timestamp: Option<serde_json::Number>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "type")]
    pub type_: ResponsesOrderType,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ResponsesBlockTradeSignature {
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "signature_data")]
    pub signature_data: String,
    #[serde(rename = "signature_expiration")]
    pub signature_expiration: serde_json::Number,
    #[serde(rename = "signature_timestamp")]
    pub signature_timestamp: serde_json::Number,
    #[serde(rename = "signature_type")]
    pub signature_type: String,
    #[serde(rename = "signer_account")]
    pub signer_account: String,
    #[serde(rename = "signer_public_key", default, skip_serializing_if = "Option::is_none")]
    pub signer_public_key: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ResponsesBlockTradeSignature {
    fn default() -> Self {
        Self {
            nonce: Default::default(),
            signature_data: Default::default(),
            signature_expiration: 0.into(),
            signature_timestamp: 0.into(),
            signature_type: Default::default(),
            signer_account: Default::default(),
            signer_public_key: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesBridgedToken {
    #[serde(rename = "decimals", default, skip_serializing_if = "Option::is_none")]
    pub decimals: Option<serde_json::Number>,
    #[serde(rename = "l1_bridge_address", default, skip_serializing_if = "Option::is_none")]
    pub l1_bridge_address: Option<String>,
    #[serde(rename = "l1_token_address", default, skip_serializing_if = "Option::is_none")]
    pub l1_token_address: Option<String>,
    #[serde(rename = "l2_bridge_address", default, skip_serializing_if = "Option::is_none")]
    pub l2_bridge_address: Option<String>,
    #[serde(rename = "l2_token_address", default, skip_serializing_if = "Option::is_none")]
    pub l2_token_address: Option<String>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesCancelOrderBatchResponse {
    #[serde(rename = "results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ResponsesCancelOrderResult>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesCancelOrderResult {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "client_id", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesCreateTokenResponse {
    #[serde(rename = "api_token", default, skip_serializing_if = "Option::is_none")]
    pub api_token: Option<ResponsesApiToken>,
    #[serde(rename = "token", default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesCreateTransferResponse {
    #[serde(rename = "transfer", default, skip_serializing_if = "Option::is_none")]
    pub transfer: Option<ResponsesXPTransfer>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesDepositAddressDetailsResp {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "amount_credited", default, skip_serializing_if = "Option::is_none")]
    pub amount_credited: Option<String>,
    #[serde(rename = "amount_received", default, skip_serializing_if = "Option::is_none")]
    pub amount_received: Option<String>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(rename = "destination_tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub destination_tx_hash: Option<String>,
    #[serde(rename = "expires_at", default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<serde_json::Number>,
    #[serde(rename = "fail_reason", default, skip_serializing_if = "Option::is_none")]
    pub fail_reason: Option<String>,
    #[serde(rename = "fee", default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "network", default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "source_tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub source_tx_hash: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponsesDepositAddressStatus>,
    #[serde(rename = "token", default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesDepositAddressResp {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "avg_completion_time", default, skip_serializing_if = "Option::is_none")]
    pub avg_completion_time: Option<String>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(rename = "expires_at", default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<serde_json::Number>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "max_amount", default, skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<String>,
    #[serde(rename = "min_amount", default, skip_serializing_if = "Option::is_none")]
    pub min_amount: Option<String>,
    #[serde(rename = "network", default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponsesDepositAddressStatus>,
    #[serde(rename = "token", default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesDepositNetwork {
    #[serde(rename = "max_amount", default, skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<String>,
    #[serde(rename = "min_amount", default, skip_serializing_if = "Option::is_none")]
    pub min_amount: Option<String>,
    #[serde(rename = "network", default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "token", default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesDepositNetworksResp {
    #[serde(rename = "results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ResponsesDepositNetwork>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesDimeAllocationResp {
    #[serde(rename = "addresses", default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    #[serde(rename = "balance", default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesDimeConfigResponse {
    #[serde(rename = "allocations", default, skip_serializing_if = "Option::is_none")]
    pub allocations: Option<Vec<ResponsesDimeAllocationResp>>,
    #[serde(rename = "circulating_supply", default, skip_serializing_if = "Option::is_none")]
    pub circulating_supply: Option<String>,
    #[serde(rename = "staked_supply", default, skip_serializing_if = "Option::is_none")]
    pub staked_supply: Option<String>,
    #[serde(rename = "total_supply", default, skip_serializing_if = "Option::is_none")]
    pub total_supply: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesDiscordProfile {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "image_url", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "username", default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesErrorResponse {
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponsesErrorCode>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesFees {
    #[serde(rename = "commission_rate", default, skip_serializing_if = "Option::is_none")]
    pub commission_rate: Option<String>,
    #[serde(rename = "dated_option_maker_rate", default, skip_serializing_if = "Option::is_none")]
    pub dated_option_maker_rate: Option<String>,
    #[serde(rename = "dated_option_taker_rate", default, skip_serializing_if = "Option::is_none")]
    pub dated_option_taker_rate: Option<String>,
    #[serde(rename = "discount_rate", default, skip_serializing_if = "Option::is_none")]
    pub discount_rate: Option<String>,
    #[serde(rename = "maker_rate", default, skip_serializing_if = "Option::is_none")]
    pub maker_rate: Option<String>,
    #[serde(rename = "perp_option_maker_rate", default, skip_serializing_if = "Option::is_none")]
    pub perp_option_maker_rate: Option<String>,
    #[serde(rename = "perp_option_taker_rate", default, skip_serializing_if = "Option::is_none")]
    pub perp_option_taker_rate: Option<String>,
    #[serde(rename = "spot_maker_rate", default, skip_serializing_if = "Option::is_none")]
    pub spot_maker_rate: Option<String>,
    #[serde(rename = "spot_taker_rate", default, skip_serializing_if = "Option::is_none")]
    pub spot_taker_rate: Option<String>,
    #[serde(rename = "taker_rate", default, skip_serializing_if = "Option::is_none")]
    pub taker_rate: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesFillResult {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "client_id", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(rename = "fee", default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename = "fee_currency", default, skip_serializing_if = "Option::is_none")]
    pub fee_currency: Option<String>,
    #[serde(rename = "fill_type", default, skip_serializing_if = "Option::is_none")]
    pub fill_type: Option<ResponsesFillType>,
    #[serde(rename = "flags", default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<ResponsesFillFlag>>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<ResponsesTraderRole>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "order_id", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "orderbook_seq_no", default, skip_serializing_if = "Option::is_none")]
    pub orderbook_seq_no: Option<serde_json::Number>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "realized_funding", default, skip_serializing_if = "Option::is_none")]
    pub realized_funding: Option<String>,
    #[serde(rename = "realized_pnl", default, skip_serializing_if = "Option::is_none")]
    pub realized_pnl: Option<String>,
    #[serde(rename = "remaining_size", default, skip_serializing_if = "Option::is_none")]
    pub remaining_size: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<ResponsesOrderSide>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "underlying_price", default, skip_serializing_if = "Option::is_none")]
    pub underlying_price: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesGetAccountMarginConfigsResp {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "configs", default, skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<ResponsesAccountMarginEntry>>,
    #[serde(rename = "margin_methodology", default, skip_serializing_if = "Option::is_none")]
    pub margin_methodology: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesGetAccountMmpConfigsResp {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ResponsesAccountMmpConfigResp>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesGetAccountsInfoResponse {
    #[serde(rename = "results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ResponsesAccountInfoResponse>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesGetOnboardingResp {
    #[serde(rename = "account_signer_type", default, skip_serializing_if = "Option::is_none")]
    pub account_signer_type: Option<String>,
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "derivation_info", default, skip_serializing_if = "Option::is_none")]
    pub derivation_info: Option<ResponsesOnboardingDerivationInfo>,
    #[serde(rename = "exists", default, skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesGetSubAccountsResponse {
    #[serde(rename = "results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ResponsesAccountInfoResponse>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesGetXPBalanceResponseV2 {
    #[serde(rename = "earned_xp", default, skip_serializing_if = "Option::is_none")]
    pub earned_xp: Option<serde_json::Number>,
    #[serde(rename = "transferrable_xp", default, skip_serializing_if = "Option::is_none")]
    pub transferrable_xp: Option<serde_json::Number>,
    #[serde(rename = "xp_delta", default, skip_serializing_if = "Option::is_none")]
    pub xp_delta: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesImpactPriceResp {
    #[serde(rename = "impact_price", default, skip_serializing_if = "Option::is_none")]
    pub impact_price: Option<String>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesInsuranceAccountResp {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "account_value", default, skip_serializing_if = "Option::is_none")]
    pub account_value: Option<String>,
    #[serde(rename = "settlement_asset", default, skip_serializing_if = "Option::is_none")]
    pub settlement_asset: Option<String>,
    #[serde(rename = "updated_at", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesJwkKey {
    #[serde(rename = "alg", default, skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,
    #[serde(rename = "crv", default, skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,
    #[serde(rename = "kid", default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(rename = "kty", default, skip_serializing_if = "Option::is_none")]
    pub kty: Option<String>,
    #[serde(rename = "use", default, skip_serializing_if = "Option::is_none")]
    pub use_: Option<String>,
    #[serde(rename = "x", default, skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(rename = "y", default, skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesJwksResponse {
    #[serde(rename = "keys", default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<ResponsesJwkKey>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesMarketEvent {
    #[serde(rename = "changed_fields", default, skip_serializing_if = "Option::is_none")]
    pub changed_fields: Option<ResponsesMarketHistoryFields>,
    #[serde(rename = "event_type", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<ResponsesMarketEventType>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "updated_at", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesMarketHistoryFields {
    #[serde(rename = "asset_kind", default, skip_serializing_if = "Option::is_none")]
    pub asset_kind: Option<String>,
    #[serde(rename = "base_currency", default, skip_serializing_if = "Option::is_none")]
    pub base_currency: Option<String>,
    #[serde(rename = "clamp_rate", default, skip_serializing_if = "Option::is_none")]
    pub clamp_rate: Option<serde_json::Number>,
    #[serde(rename = "delta1_cross_margin_params", default, skip_serializing_if = "Option::is_none")]
    pub delta1_cross_margin_params: Option<serde_json::Value>,
    #[serde(rename = "expiry_at", default, skip_serializing_if = "Option::is_none")]
    pub expiry_at: Option<serde_json::Number>,
    #[serde(rename = "fee_config", default, skip_serializing_if = "Option::is_none")]
    pub fee_config: Option<serde_json::Value>,
    #[serde(rename = "funding_multiplier", default, skip_serializing_if = "Option::is_none")]
    pub funding_multiplier: Option<serde_json::Number>,
    #[serde(rename = "funding_period_hours", default, skip_serializing_if = "Option::is_none")]
    pub funding_period_hours: Option<serde_json::Number>,
    #[serde(rename = "interest_rate", default, skip_serializing_if = "Option::is_none")]
    pub interest_rate: Option<serde_json::Number>,
    #[serde(rename = "market_kind", default, skip_serializing_if = "Option::is_none")]
    pub market_kind: Option<String>,
    #[serde(rename = "max_funding_rate", default, skip_serializing_if = "Option::is_none")]
    pub max_funding_rate: Option<serde_json::Number>,
    #[serde(rename = "max_funding_rate_change", default, skip_serializing_if = "Option::is_none")]
    pub max_funding_rate_change: Option<serde_json::Number>,
    #[serde(rename = "max_open_orders", default, skip_serializing_if = "Option::is_none")]
    pub max_open_orders: Option<serde_json::Number>,
    #[serde(rename = "max_order_size", default, skip_serializing_if = "Option::is_none")]
    pub max_order_size: Option<String>,
    #[serde(rename = "max_slippage", default, skip_serializing_if = "Option::is_none")]
    pub max_slippage: Option<serde_json::Number>,
    #[serde(rename = "max_tob_spread", default, skip_serializing_if = "Option::is_none")]
    pub max_tob_spread: Option<serde_json::Number>,
    #[serde(rename = "min_notional", default, skip_serializing_if = "Option::is_none")]
    pub min_notional: Option<String>,
    #[serde(rename = "open_at", default, skip_serializing_if = "Option::is_none")]
    pub open_at: Option<serde_json::Number>,
    #[serde(rename = "option_cross_margin_params", default, skip_serializing_if = "Option::is_none")]
    pub option_cross_margin_params: Option<serde_json::Value>,
    #[serde(rename = "option_type", default, skip_serializing_if = "Option::is_none")]
    pub option_type: Option<String>,
    #[serde(rename = "oracle_ewma_factor", default, skip_serializing_if = "Option::is_none")]
    pub oracle_ewma_factor: Option<serde_json::Number>,
    #[serde(rename = "order_size_increment", default, skip_serializing_if = "Option::is_none")]
    pub order_size_increment: Option<String>,
    #[serde(rename = "position_limit", default, skip_serializing_if = "Option::is_none")]
    pub position_limit: Option<String>,
    #[serde(rename = "price_bands_width", default, skip_serializing_if = "Option::is_none")]
    pub price_bands_width: Option<serde_json::Number>,
    #[serde(rename = "price_tick_size", default, skip_serializing_if = "Option::is_none")]
    pub price_tick_size: Option<String>,
    #[serde(rename = "quote_currency", default, skip_serializing_if = "Option::is_none")]
    pub quote_currency: Option<String>,
    #[serde(rename = "settlement_currency", default, skip_serializing_if = "Option::is_none")]
    pub settlement_currency: Option<String>,
    #[serde(rename = "settlement_price", default, skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<serde_json::Number>,
    #[serde(rename = "strike_price", default, skip_serializing_if = "Option::is_none")]
    pub strike_price: Option<String>,
    #[serde(rename = "underlying_twap_price", default, skip_serializing_if = "Option::is_none")]
    pub underlying_twap_price: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesMarketHistoryResponse {
    #[serde(rename = "results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ResponsesMarketEvent>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesMarketSettlementResp {
    #[serde(rename = "expiry_at", default, skip_serializing_if = "Option::is_none")]
    pub expiry_at: Option<serde_json::Number>,
    #[serde(rename = "settlement_price", default, skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "underlying_twap_price", default, skip_serializing_if = "Option::is_none")]
    pub underlying_twap_price: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesNft {
    #[serde(rename = "collection_address", default, skip_serializing_if = "Option::is_none")]
    pub collection_address: Option<String>,
    #[serde(rename = "collection_name", default, skip_serializing_if = "Option::is_none")]
    pub collection_name: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "image_url", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<Messagesv1NftPrice>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesNotificationPreferencesResp {
    #[serde(rename = "announcements", default, skip_serializing_if = "Option::is_none")]
    pub announcements: Option<bool>,
    #[serde(rename = "fill_sound", default, skip_serializing_if = "Option::is_none")]
    pub fill_sound: Option<bool>,
    #[serde(rename = "fills", default, skip_serializing_if = "Option::is_none")]
    pub fills: Option<bool>,
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<bool>,
    #[serde(rename = "transfers", default, skip_serializing_if = "Option::is_none")]
    pub transfers: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesOnboardingDerivationInfo {
    #[serde(rename = "class_hash", default, skip_serializing_if = "Option::is_none")]
    pub class_hash: Option<String>,
    #[serde(rename = "proxy_class_hash", default, skip_serializing_if = "Option::is_none")]
    pub proxy_class_hash: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesOrderResp {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "avg_fill_price", default, skip_serializing_if = "Option::is_none")]
    pub avg_fill_price: Option<String>,
    #[serde(rename = "cancel_reason", default, skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,
    #[serde(rename = "client_id", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(rename = "flags", default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<ResponsesOrderFlag>>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "instruction", default, skip_serializing_if = "Option::is_none")]
    pub instruction: Option<ResponsesOrderInstruction>,
    #[serde(rename = "last_updated_at", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<serde_json::Number>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "published_at", default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<serde_json::Number>,
    #[serde(rename = "received_at", default, skip_serializing_if = "Option::is_none")]
    pub received_at: Option<serde_json::Number>,
    #[serde(rename = "remaining_size", default, skip_serializing_if = "Option::is_none")]
    pub remaining_size: Option<String>,
    #[serde(rename = "request_info", default, skip_serializing_if = "Option::is_none")]
    pub request_info: Option<ResponsesRequestInfo>,
    #[serde(rename = "seq_no", default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<serde_json::Number>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<ResponsesOrderSide>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponsesOrderStatus>,
    #[serde(rename = "stp", default, skip_serializing_if = "Option::is_none")]
    pub stp: Option<ResponsesSTPMode>,
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<serde_json::Number>,
    #[serde(rename = "trigger_price", default, skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ResponsesOrderType>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesParadigmEnableResponse {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subkey_public_key", default, skip_serializing_if = "Option::is_none")]
    pub subkey_public_key: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesReferralConfigResp {
    #[serde(rename = "commission_rate", default, skip_serializing_if = "Option::is_none")]
    pub commission_rate: Option<String>,
    #[serde(rename = "commission_volume_cap", default, skip_serializing_if = "Option::is_none")]
    pub commission_volume_cap: Option<String>,
    #[serde(rename = "discount_rate", default, skip_serializing_if = "Option::is_none")]
    pub discount_rate: Option<String>,
    #[serde(rename = "discount_volume_cap", default, skip_serializing_if = "Option::is_none")]
    pub discount_volume_cap: Option<String>,
    #[serde(rename = "minimum_volume", default, skip_serializing_if = "Option::is_none")]
    pub minimum_volume: Option<String>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "points_bonus_rate", default, skip_serializing_if = "Option::is_none")]
    pub points_bonus_rate: Option<String>,
    #[serde(rename = "points_bonus_volume_cap", default, skip_serializing_if = "Option::is_none")]
    pub points_bonus_volume_cap: Option<String>,
    #[serde(rename = "referral_type", default, skip_serializing_if = "Option::is_none")]
    pub referral_type: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRefreshInventoryResp {
    #[serde(rename = "nfts", default, skip_serializing_if = "Option::is_none")]
    pub nfts: Option<Vec<ResponsesNft>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRequestInfo {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "request_type", default, skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRevokeSubkeyResponse {
    #[serde(rename = "public_key", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRevokeTokenResponse {
    #[serde(rename = "lookup_id", default, skip_serializing_if = "Option::is_none")]
    pub lookup_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRfqAcceptedResponse {
    #[serde(rename = "accepted_at", default, skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<serde_json::Number>,
    #[serde(rename = "request_id", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRfqBboLegResult {
    #[serde(rename = "best_ask_iv", default, skip_serializing_if = "Option::is_none")]
    pub best_ask_iv: Option<String>,
    #[serde(rename = "best_ask_price", default, skip_serializing_if = "Option::is_none")]
    pub best_ask_price: Option<String>,
    #[serde(rename = "best_ask_quantity", default, skip_serializing_if = "Option::is_none")]
    pub best_ask_quantity: Option<String>,
    #[serde(rename = "best_bid_iv", default, skip_serializing_if = "Option::is_none")]
    pub best_bid_iv: Option<String>,
    #[serde(rename = "best_bid_price", default, skip_serializing_if = "Option::is_none")]
    pub best_bid_price: Option<String>,
    #[serde(rename = "best_bid_quantity", default, skip_serializing_if = "Option::is_none")]
    pub best_bid_quantity: Option<String>,
    #[serde(rename = "mark_price", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "mark_price_iv", default, skip_serializing_if = "Option::is_none")]
    pub mark_price_iv: Option<String>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRfqBboResult {
    #[serde(rename = "best_ask_price", default, skip_serializing_if = "Option::is_none")]
    pub best_ask_price: Option<String>,
    #[serde(rename = "best_ask_quantity", default, skip_serializing_if = "Option::is_none")]
    pub best_ask_quantity: Option<String>,
    #[serde(rename = "best_bid_price", default, skip_serializing_if = "Option::is_none")]
    pub best_bid_price: Option<String>,
    #[serde(rename = "best_bid_quantity", default, skip_serializing_if = "Option::is_none")]
    pub best_bid_quantity: Option<String>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(rename = "legs", default, skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<ResponsesRfqBboLegResult>>,
    #[serde(rename = "mark_price", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "max_price", default, skip_serializing_if = "Option::is_none")]
    pub max_price: Option<String>,
    #[serde(rename = "min_price", default, skip_serializing_if = "Option::is_none")]
    pub min_price: Option<String>,
    #[serde(rename = "rfq_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRfqDraftResult {
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "legs", default, skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<ResponsesRfqLegResult>>,
    #[serde(rename = "quantity", default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "strategy", default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(rename = "updated_at", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRfqLegResult {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "ratio", default, skip_serializing_if = "Option::is_none")]
    pub ratio: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRfqMarketInfo {
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "venue_size_increment", default, skip_serializing_if = "Option::is_none")]
    pub venue_size_increment: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRfqMarketsResult {
    #[serde(rename = "markets", default, skip_serializing_if = "Option::is_none")]
    pub markets: Option<Vec<String>>,
    #[serde(rename = "markets_info", default, skip_serializing_if = "Option::is_none")]
    pub markets_info: Option<Vec<ResponsesRfqMarketInfo>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRfqPlatformStateResult {
    #[serde(rename = "next_platform_state", default, skip_serializing_if = "Option::is_none")]
    pub next_platform_state: Option<ResponsesRfqVenueState>,
    #[serde(rename = "next_platform_state_at", default, skip_serializing_if = "Option::is_none")]
    pub next_platform_state_at: Option<serde_json::Number>,
    #[serde(rename = "platform_state", default, skip_serializing_if = "Option::is_none")]
    pub platform_state: Option<ResponsesRfqVenueState>,
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ResponsesRfqVenueState>,
    #[serde(rename = "venue_state", default, skip_serializing_if = "Option::is_none")]
    pub venue_state: Option<ResponsesRfqVenueState>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRfqResult {
    #[serde(rename = "closed_reason", default, skip_serializing_if = "Option::is_none")]
    pub closed_reason: Option<String>,
    #[serde(rename = "counterparties", default, skip_serializing_if = "Option::is_none")]
    pub counterparties: Option<Vec<String>>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(rename = "expires_at", default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<serde_json::Number>,
    #[serde(rename = "filled_quantity", default, skip_serializing_if = "Option::is_none")]
    pub filled_quantity: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "legs", default, skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<ResponsesRfqLegResult>>,
    #[serde(rename = "mark_price", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "quantity", default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRfqResults {
    #[serde(rename = "next", default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(rename = "results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ResponsesRfqResult>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesShareRateResp {
    #[serde(rename = "last_updated_at", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<serde_json::Number>,
    #[serde(rename = "share_rate", default, skip_serializing_if = "Option::is_none")]
    pub share_rate: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesStakingConfigResponse {
    #[serde(rename = "dime_fee_discount_pct", default, skip_serializing_if = "Option::is_none")]
    pub dime_fee_discount_pct: Option<String>,
    #[serde(rename = "tiers", default, skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<ResponsesStakingTierResp>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesStakingSummaryResponse {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "active_stakes", default, skip_serializing_if = "Option::is_none")]
    pub active_stakes: Option<serde_json::Number>,
    #[serde(rename = "staking_tier", default, skip_serializing_if = "Option::is_none")]
    pub staking_tier: Option<String>,
    #[serde(rename = "total_staked", default, skip_serializing_if = "Option::is_none")]
    pub total_staked: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesStakingTierResp {
    #[serde(rename = "fee_rates", default, skip_serializing_if = "Option::is_none")]
    pub fee_rates: Option<std::collections::BTreeMap<String,String>>,
    #[serde(rename = "min_staked", default, skip_serializing_if = "Option::is_none")]
    pub min_staked: Option<String>,
    #[serde(rename = "tier_name", default, skip_serializing_if = "Option::is_none")]
    pub tier_name: Option<String>,
    #[serde(rename = "xp_multiplier", default, skip_serializing_if = "Option::is_none")]
    pub xp_multiplier: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesStrategy {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "visibility", default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ResponsesVisibility>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesSubkey {
    #[serde(rename = "account_id", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "allowed_cidrs", default, skip_serializing_if = "Option::is_none")]
    pub allowed_cidrs: Option<Vec<String>>,
    #[serde(rename = "block_trade_signing_enabled", default, skip_serializing_if = "Option::is_none")]
    pub block_trade_signing_enabled: Option<bool>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Number>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "public_key", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "revoked_at", default, skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<serde_json::Number>,
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "updated_at", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesSystemConfigResponse {
    #[serde(rename = "assets_limit_per_account", default, skip_serializing_if = "Option::is_none")]
    pub assets_limit_per_account: Option<serde_json::Number>,
    #[serde(rename = "block_explorer_url", default, skip_serializing_if = "Option::is_none")]
    pub block_explorer_url: Option<String>,
    #[serde(rename = "bridged_tokens", default, skip_serializing_if = "Option::is_none")]
    pub bridged_tokens: Option<Vec<ResponsesBridgedToken>>,
    #[serde(rename = "dime_staking_contract_address", default, skip_serializing_if = "Option::is_none")]
    pub dime_staking_contract_address: Option<String>,
    #[serde(rename = "environment", default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(rename = "l1_chain_id", default, skip_serializing_if = "Option::is_none")]
    pub l1_chain_id: Option<String>,
    #[serde(rename = "l1_core_contract_address", default, skip_serializing_if = "Option::is_none")]
    pub l1_core_contract_address: Option<String>,
    #[serde(rename = "l1_operator_address", default, skip_serializing_if = "Option::is_none")]
    pub l1_operator_address: Option<String>,
    #[serde(rename = "l1_relayer_address", default, skip_serializing_if = "Option::is_none")]
    pub l1_relayer_address: Option<String>,
    #[serde(rename = "l2_relayer_address", default, skip_serializing_if = "Option::is_none")]
    pub l2_relayer_address: Option<String>,
    #[serde(rename = "liquidation_fee", default, skip_serializing_if = "Option::is_none")]
    pub liquidation_fee: Option<String>,
    #[serde(rename = "oracle_address", default, skip_serializing_if = "Option::is_none")]
    pub oracle_address: Option<String>,
    #[serde(rename = "paraclear_account_hash", default, skip_serializing_if = "Option::is_none")]
    pub paraclear_account_hash: Option<String>,
    #[serde(rename = "paraclear_account_proxy_hash", default, skip_serializing_if = "Option::is_none")]
    pub paraclear_account_proxy_hash: Option<String>,
    #[serde(rename = "paraclear_address", default, skip_serializing_if = "Option::is_none")]
    pub paraclear_address: Option<String>,
    #[serde(rename = "paraclear_decimals", default, skip_serializing_if = "Option::is_none")]
    pub paraclear_decimals: Option<serde_json::Number>,
    #[serde(rename = "paraclear_evm_account_hash", default, skip_serializing_if = "Option::is_none")]
    pub paraclear_evm_account_hash: Option<String>,
    #[serde(rename = "partial_liquidation_buffer", default, skip_serializing_if = "Option::is_none")]
    pub partial_liquidation_buffer: Option<String>,
    #[serde(rename = "partial_liquidation_share_increment", default, skip_serializing_if = "Option::is_none")]
    pub partial_liquidation_share_increment: Option<String>,
    #[serde(rename = "rfq_default_max_slippage", default, skip_serializing_if = "Option::is_none")]
    pub rfq_default_max_slippage: Option<String>,
    #[serde(rename = "starknet_chain_id", default, skip_serializing_if = "Option::is_none")]
    pub starknet_chain_id: Option<String>,
    #[serde(rename = "starknet_fullnode_rpc_base_url", default, skip_serializing_if = "Option::is_none")]
    pub starknet_fullnode_rpc_base_url: Option<String>,
    #[serde(rename = "starknet_fullnode_rpc_url", default, skip_serializing_if = "Option::is_none")]
    pub starknet_fullnode_rpc_url: Option<String>,
    #[serde(rename = "starknet_gateway_url", default, skip_serializing_if = "Option::is_none")]
    pub starknet_gateway_url: Option<String>,
    #[serde(rename = "universal_deployer_address", default, skip_serializing_if = "Option::is_none")]
    pub universal_deployer_address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesSystemStateResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponsesSystemStatus>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesSystemStatsResponse {
    #[serde(rename = "lifetime_volume", default, skip_serializing_if = "Option::is_none")]
    pub lifetime_volume: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesSystemTimeResponse {
    #[serde(rename = "server_time", default, skip_serializing_if = "Option::is_none")]
    pub server_time: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesTwitterProfile {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "image_url", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "username", default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesUpdateAccountMarginConfigResp {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    pub leverage: Option<serde_json::Number>,
    #[serde(rename = "margin_type", default, skip_serializing_if = "Option::is_none")]
    pub margin_type: Option<String>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesVaultAnalytics {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "annualized_volatility", default, skip_serializing_if = "Option::is_none")]
    pub annualized_volatility: Option<ResponsesWindowedMetrics>,
    #[serde(rename = "apr", default, skip_serializing_if = "Option::is_none")]
    pub apr: Option<ResponsesWindowedMetrics>,
    #[serde(rename = "as_of", default, skip_serializing_if = "Option::is_none")]
    pub as_of: Option<String>,
    #[serde(rename = "best_period_return", default, skip_serializing_if = "Option::is_none")]
    pub best_period_return: Option<ResponsesWindowedMetrics>,
    #[serde(rename = "max_drawdown", default, skip_serializing_if = "Option::is_none")]
    pub max_drawdown: Option<ResponsesWindowedMetrics>,
    #[serde(rename = "monthly_pnl_last_12m", default, skip_serializing_if = "Option::is_none")]
    pub monthly_pnl_last_12m: Option<Vec<ResponsesVaultMonthlyPnlEntry>>,
    #[serde(rename = "pct_positive", default, skip_serializing_if = "Option::is_none")]
    pub pct_positive: Option<ResponsesWindowedMetrics>,
    #[serde(rename = "pnl", default, skip_serializing_if = "Option::is_none")]
    pub pnl: Option<ResponsesWindowedMetrics>,
    #[serde(rename = "recovery_hours", default, skip_serializing_if = "Option::is_none")]
    pub recovery_hours: Option<ResponsesWindowedMetrics>,
    #[serde(rename = "sharpe_ratio", default, skip_serializing_if = "Option::is_none")]
    pub sharpe_ratio: Option<ResponsesWindowedMetrics>,
    #[serde(rename = "top_holders", default, skip_serializing_if = "Option::is_none")]
    pub top_holders: Option<Vec<ResponsesVaultTopHolder>>,
    #[serde(rename = "worst_period_return", default, skip_serializing_if = "Option::is_none")]
    pub worst_period_return: Option<ResponsesWindowedMetrics>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesVaultAnalyticsResp {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "as_of", default, skip_serializing_if = "Option::is_none")]
    pub as_of: Option<String>,
    #[serde(rename = "substrategies", default, skip_serializing_if = "Option::is_none")]
    pub substrategies: Option<Vec<ResponsesAccountAnalytics>>,
    #[serde(rename = "vault_level", default, skip_serializing_if = "Option::is_none")]
    pub vault_level: Option<ResponsesVaultAnalytics>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesVaultMonthlyPnlEntry {
    #[serde(rename = "month", default, skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,
    #[serde(rename = "month_return_pct", default, skip_serializing_if = "Option::is_none")]
    pub month_return_pct: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesVaultResp {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(rename = "curator_name", default, skip_serializing_if = "Option::is_none")]
    pub curator_name: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ResponsesVaultKind>,
    #[serde(rename = "last_updated_at", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<serde_json::Number>,
    #[serde(rename = "lockup_period", default, skip_serializing_if = "Option::is_none")]
    pub lockup_period: Option<serde_json::Number>,
    #[serde(rename = "max_tvl", default, skip_serializing_if = "Option::is_none")]
    pub max_tvl: Option<serde_json::Number>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "operator_account", default, skip_serializing_if = "Option::is_none")]
    pub operator_account: Option<String>,
    #[serde(rename = "owner_account", default, skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "profit_share", default, skip_serializing_if = "Option::is_none")]
    pub profit_share: Option<serde_json::Number>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponsesVaultStatus>,
    #[serde(rename = "strategies", default, skip_serializing_if = "Option::is_none")]
    pub strategies: Option<Vec<ResponsesStrategy>>,
    #[serde(rename = "strategy_description", default, skip_serializing_if = "Option::is_none")]
    pub strategy_description: Option<String>,
    #[serde(rename = "token_address", default, skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    #[serde(rename = "visibility", default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ResponsesVisibility>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesVaultTopHolder {
    #[serde(rename = "percentage", default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    #[serde(rename = "rank", default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<serde_json::Number>,
    #[serde(rename = "value_usdc", default, skip_serializing_if = "Option::is_none")]
    pub value_usdc: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesVaultsConfigResponse {
    #[serde(rename = "max_lockup_period_days", default, skip_serializing_if = "Option::is_none")]
    pub max_lockup_period_days: Option<String>,
    #[serde(rename = "max_profit_share_percentage", default, skip_serializing_if = "Option::is_none")]
    pub max_profit_share_percentage: Option<String>,
    #[serde(rename = "min_initial_deposit", default, skip_serializing_if = "Option::is_none")]
    pub min_initial_deposit: Option<String>,
    #[serde(rename = "min_lockup_period_days", default, skip_serializing_if = "Option::is_none")]
    pub min_lockup_period_days: Option<String>,
    #[serde(rename = "min_owner_share_percentage", default, skip_serializing_if = "Option::is_none")]
    pub min_owner_share_percentage: Option<String>,
    #[serde(rename = "profit_share_period", default, skip_serializing_if = "Option::is_none")]
    pub profit_share_period: Option<String>,
    #[serde(rename = "vault_factory_address", default, skip_serializing_if = "Option::is_none")]
    pub vault_factory_address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesVisibility {
    #[serde(rename = "balances", default, skip_serializing_if = "Option::is_none")]
    pub balances: Option<bool>,
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesVolumeTierKindInfo {
    #[serde(rename = "tier", default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesWindowedMetrics {
    #[serde(rename = "window_1y", default, skip_serializing_if = "Option::is_none")]
    pub window_1y: Option<String>,
    #[serde(rename = "window_24h", default, skip_serializing_if = "Option::is_none")]
    pub window_24h: Option<String>,
    #[serde(rename = "window_30d", default, skip_serializing_if = "Option::is_none")]
    pub window_30d: Option<String>,
    #[serde(rename = "window_7d", default, skip_serializing_if = "Option::is_none")]
    pub window_7d: Option<String>,
    #[serde(rename = "window_90d", default, skip_serializing_if = "Option::is_none")]
    pub window_90d: Option<String>,
    #[serde(rename = "window_all", default, skip_serializing_if = "Option::is_none")]
    pub window_all: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesWithdrawalLockRejectionResponse {
    #[serde(rename = "amount", default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "rejected_at", default, skip_serializing_if = "Option::is_none")]
    pub rejected_at: Option<serde_json::Number>,
    #[serde(rename = "request_id", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesWithdrawalLockResponse {
    #[serde(rename = "amount", default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "expires_at", default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<serde_json::Number>,
    #[serde(rename = "request_id", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponsesXPTransfer {
    #[serde(rename = "amount", default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<serde_json::Number>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(rename = "fee", default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<serde_json::Number>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "is_private", default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(rename = "recipient", default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(rename = "season", default, skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,
    #[serde(rename = "sender", default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RevokeSubkeyParams {
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RevokeTokenParams {
    #[serde(rename = "lookup_id")]
    pub lookup_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqBboParams {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqCreateParams {
    #[serde(rename = "body")]
    pub body: RequestsRfqCreateRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqDeleteParams {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqDraftCreateParams {
    #[serde(rename = "body")]
    pub body: RequestsRfqCreateRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqDraftDeleteParams {
    #[serde(rename = "draft_id")]
    pub draft_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqDraftListParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqEnableParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqExecuteParams {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "body")]
    pub body: RequestsRfqExecuteRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqListParams {
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqMarketsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RfqPlatformStateParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ServicesTransferFeeConfigResponse {
    #[serde(rename = "fee_percentage", default, skip_serializing_if = "Option::is_none")]
    pub fee_percentage: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradebustsParams {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "end_at", default, skip_serializing_if = "Option::is_none")]
    pub end_at: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "start_at", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesParams {
    #[serde(rename = "base_asset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "end_at", default, skip_serializing_if = "Option::is_none")]
    pub end_at: Option<serde_json::Number>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "start_at", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<serde_json::Number>,
    #[serde(rename = "trade_type", default, skip_serializing_if = "Option::is_none")]
    pub trade_type: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransactionsParams {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "end_at", default, skip_serializing_if = "Option::is_none")]
    pub end_at: Option<serde_json::Number>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "start_at", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateAccountMarketMaxSlippageParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "body")]
    pub body: RequestsUpdateAccountMaxSlippageRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateAccountMaxSlippageParams {
    #[serde(rename = "body")]
    pub body: UpdateAccountMaxSlippageParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateAccountMaxSlippageParamsBody {
    #[serde(rename = "max_slippage")]
    pub max_slippage: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateAccountNotificationPreferencesParams {
    #[serde(rename = "body")]
    pub body: RequestsUpdateNotificationPreferencesRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateAccountProfileParams {
    #[serde(rename = "body")]
    pub body: RequestsUpdateAccountProfileRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateAccountProfileReferralCodeParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateAccountProfileUsernameParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateAccountReferrerParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateAccountSizeCurrencyDisplayParams {
    #[serde(rename = "body")]
    pub body: RequestsUpdateSizeCurrencyDisplayRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateAccountTradingValueDisplayParams {
    #[serde(rename = "body")]
    pub body: RequestsUpdateTradingValueDisplayRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateLastSeenParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateSubkeyAllowedCidrsParams {
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "body")]
    pub body: RequestsUpdateAllowedCIDRs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateSubkeyBlockTradeSigningParams {
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "body")]
    pub body: RequestsUpdateSubkeyBlockTradeSigning,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateSubkeyParams {
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "body")]
    pub body: RequestsUpdateSubkey,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateTokenAllowedCidrsParams {
    #[serde(rename = "lookup_id")]
    pub lookup_id: String,
    #[serde(rename = "body")]
    pub body: RequestsUpdateAllowedCIDRs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpsertAccountMarginParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "body")]
    pub body: RequestsAccountMarginRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpsertAccountMmpParams {
    #[serde(rename = "base_asset")]
    pub base_asset: String,
    #[serde(rename = "body")]
    pub body: RequestsAccountMmpConfigRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultsCreateParams {
    #[serde(rename = "PARADEX-PARENT-ACCOUNT")]
    pub paradex_parent_account: String,
    #[serde(rename = "PARADEX-STARKNET-SIGNATURE")]
    pub paradex_starknet_signature: String,
    #[serde(rename = "PARADEX-STARKNET-ACCOUNT")]
    pub paradex_starknet_account: String,
    #[serde(rename = "PARADEX-STARKNET-ACCOUNT-DERIVATION-PATH")]
    pub paradex_starknet_account_derivation_path: String,
    #[serde(rename = "body")]
    pub body: RequestsCreateVault,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultsGetAccountSummaryParams {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultsGetAnalyticsParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultsGetBalanceParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "strategy", default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultsGetConfigParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultsGetHistoricalDataParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "strategy", default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultsGetMineParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultsGetParams {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultsGetPositionsParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "strategy", default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultsGetSummaryParams {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VaultsGetTransfersParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedParadexClient(ParadexClient);
impl ParadexClient {
    pub fn typed(self) -> TypedParadexClient { TypedParadexClient(self) }
}
impl TypedParadexClient {
    pub fn activate_subkey(&self, request: &ActivateSubkeyParams) -> Result<RequestsActivateSubkeyResponse> {
        serde_json::from_value(self.0.activate_subkey(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn archive_sub_account(&self, request: &ArchiveSubAccountParams) -> Result<ResponsesAccountInfoResponse> {
        serde_json::from_value(self.0.archive_sub_account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn block_offers_cancel(&self, request: &BlockOffersCancelParams) -> Result<std::collections::BTreeMap<String, String>> {
        serde_json::from_value(self.0.block_offers_cancel(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn block_offers_create(&self, request: &BlockOffersCreateParams) -> Result<ResponsesBlockTradeDetailFullResponse> {
        serde_json::from_value(self.0.block_offers_create(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn block_offers_execute(&self, request: &BlockOffersExecuteParams) -> Result<ResponsesBlockTradeDetailFullResponse> {
        serde_json::from_value(self.0.block_offers_execute(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn block_offers_get(&self, request: &BlockOffersGetParams) -> Result<ResponsesBlockTradeDetailFullResponse> {
        serde_json::from_value(self.0.block_offers_get(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn block_offers_list(&self, request: &BlockOffersListParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.block_offers_list(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn block_trades_cancel(&self, request: &BlockTradesCancelParams) -> Result<std::collections::BTreeMap<String, String>> {
        serde_json::from_value(self.0.block_trades_cancel(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn block_trades_create(&self, request: &BlockTradesCreateParams) -> Result<ResponsesBlockTradeDetailFullResponse> {
        serde_json::from_value(self.0.block_trades_create(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn block_trades_execute(&self, request: &BlockTradesExecuteParams) -> Result<ResponsesBlockTradeDetailFullResponse> {
        serde_json::from_value(self.0.block_trades_execute(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn block_trades_get(&self, request: &BlockTradesGetParams) -> Result<ResponsesBlockTradeDetailFullResponse> {
        serde_json::from_value(self.0.block_trades_get(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn block_trades_list(&self, request: &BlockTradesListParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.block_trades_list(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn cancel_algo_order(&self, request: &CancelAlgoOrderParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.cancel_algo_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn create_algo_order(&self, request: &CreateAlgoOrderParams) -> Result<ResponsesAlgoOrderResp> {
        serde_json::from_value(self.0.create_algo_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn create_deposit_address(&self, request: &CreateDepositAddressParams) -> Result<ResponsesDepositAddressResp> {
        serde_json::from_value(self.0.create_deposit_address(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn create_token(&self, request: &CreateTokenParams) -> Result<ResponsesCreateTokenResponse> {
        serde_json::from_value(self.0.create_token(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn create_xp_transfer_v2(&self, request: &CreateXpTransferV2Params) -> Result<ResponsesCreateTransferResponse> {
        serde_json::from_value(self.0.create_xp_transfer_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn delete_account_mmp(&self, request: &DeleteAccountMmpParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_account_mmp(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn evm_auth(&self, request: &EvmAuthParams) -> Result<ResponsesAuthResp> {
        serde_json::from_value(self.0.evm_auth(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn evm_onboarding(&self, request: &EvmOnboardingParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.evm_onboarding(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_account(&self, request: &GetAccountParams) -> Result<ResponsesAccountSummaryResponse> {
        serde_json::from_value(self.0.get_account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_account_history(&self, request: &GetAccountHistoryParams) -> Result<ResponsesAccountHistoricalDataResp> {
        serde_json::from_value(self.0.get_account_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_account_margin(&self, request: &GetAccountMarginParams) -> Result<ResponsesGetAccountMarginConfigsResp> {
        serde_json::from_value(self.0.get_account_margin(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_account_mmp(&self, request: &GetAccountMmpParams) -> Result<ResponsesGetAccountMmpConfigsResp> {
        serde_json::from_value(self.0.get_account_mmp(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_account_profile(&self, request: &GetAccountProfileParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.get_account_profile(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_account_settings(&self, request: &GetAccountSettingsParams) -> Result<ResponsesAccountSettingsResp> {
        serde_json::from_value(self.0.get_account_settings(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_accounts(&self, request: &GetAccountsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_accounts(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_accounts_info(&self, request: &GetAccountsInfoParams) -> Result<ResponsesGetAccountsInfoResponse> {
        serde_json::from_value(self.0.get_accounts_info(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_algo_order_by_id(&self, request: &GetAlgoOrderByIdParams) -> Result<ResponsesAlgoOrderResp> {
        serde_json::from_value(self.0.get_algo_order_by_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_algo_orders_history(&self, request: &GetAlgoOrdersHistoryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_algo_orders_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_announcements(&self, request: &GetAnnouncementsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_announcements(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_balance(&self, request: &GetBalanceParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_balance(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_bbo(&self, request: &GetBboParams) -> Result<ResponsesBBOResp> {
        serde_json::from_value(self.0.get_bbo(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_bbo_interactive(&self, request: &GetBboInteractiveParams) -> Result<ResponsesBBOResp> {
        serde_json::from_value(self.0.get_bbo_interactive(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_compliance_actions(&self, request: &GetComplianceActionsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_compliance_actions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_deposit_address(&self, request: &GetDepositAddressParams) -> Result<ResponsesDepositAddressDetailsResp> {
        serde_json::from_value(self.0.get_deposit_address(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_dime_config(&self, request: &GetDimeConfigParams) -> Result<ResponsesDimeConfigResponse> {
        serde_json::from_value(self.0.get_dime_config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_fills(&self, request: &GetFillsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_funding(&self, request: &GetFundingParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_funding(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_funding_data(&self, request: &GetFundingDataParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_funding_data(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_global_staking_balance_history(&self, request: &GetGlobalStakingBalanceHistoryParams) -> Result<ResponsesAccountHistoricalDataResp> {
        serde_json::from_value(self.0.get_global_staking_balance_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_impact_price(&self, request: &GetImpactPriceParams) -> Result<ResponsesImpactPriceResp> {
        serde_json::from_value(self.0.get_impact_price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_insurance_fund(&self, request: &GetInsuranceFundParams) -> Result<ResponsesInsuranceAccountResp> {
        serde_json::from_value(self.0.get_insurance_fund(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_jwt(&self, request: &GetJwtParams) -> Result<ResponsesAuthResp> {
        serde_json::from_value(self.0.get_jwt(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_liquidations(&self, request: &GetLiquidationsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_liquidations(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_margin_config(&self, request: &GetMarginConfigParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_margin_config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_market_settlement_price(&self, request: &GetMarketSettlementPriceParams) -> Result<ResponsesMarketSettlementResp> {
        serde_json::from_value(self.0.get_market_settlement_price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_markets(&self, request: &GetMarketsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_markets_history(&self, request: &GetMarketsHistoryParams) -> Result<ResponsesMarketHistoryResponse> {
        serde_json::from_value(self.0.get_markets_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_markets_summary(&self, request: &GetMarketsSummaryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_markets_summary(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_onboarding(&self, request: &GetOnboardingParams) -> Result<ResponsesGetOnboardingResp> {
        serde_json::from_value(self.0.get_onboarding(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_open_algo_orders(&self, request: &GetOpenAlgoOrdersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_open_algo_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_open_orders(&self, request: &GetOpenOrdersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_orderbook(&self, request: &GetOrderbookParams) -> Result<ResponsesAskBidArray> {
        serde_json::from_value(self.0.get_orderbook(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_orderbook_interactive(&self, request: &GetOrderbookInteractiveParams) -> Result<ResponsesAskBidArray> {
        serde_json::from_value(self.0.get_orderbook_interactive(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_orders(&self, request: &GetOrdersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_portfolio_margin_config(&self, request: &GetPortfolioMarginConfigParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_portfolio_margin_config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_positions(&self, request: &GetPositionsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_public_xp_transfers_v2(&self, request: &GetPublicXpTransfersV2Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_public_xp_transfers_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_referral_qr_code(&self, request: &GetReferralQrCodeParams) -> Result<String> {
        serde_json::from_value(self.0.get_referral_qr_code(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_referrals_config(&self, request: &GetReferralsConfigParams) -> Result<serde_json::Map<String, serde_json::Value>> {
        serde_json::from_value(self.0.get_referrals_config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_referrals_summary(&self, request: &GetReferralsSummaryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_referrals_summary(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_staking_balance_history(&self, request: &GetStakingBalanceHistoryParams) -> Result<ResponsesAccountHistoricalDataResp> {
        serde_json::from_value(self.0.get_staking_balance_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_staking_config(&self, request: &GetStakingConfigParams) -> Result<ResponsesStakingConfigResponse> {
        serde_json::from_value(self.0.get_staking_config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_staking_history(&self, request: &GetStakingHistoryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_staking_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_staking_summary(&self, request: &GetStakingSummaryParams) -> Result<ResponsesStakingSummaryResponse> {
        serde_json::from_value(self.0.get_staking_summary(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_sub_accounts(&self, request: &GetSubAccountsParams) -> Result<ResponsesGetSubAccountsResponse> {
        serde_json::from_value(self.0.get_sub_accounts(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_subkey(&self, request: &GetSubkeyParams) -> Result<ResponsesSubkey> {
        serde_json::from_value(self.0.get_subkey(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_subkey_jwt(&self, request: &GetSubkeyJwtParams) -> Result<ResponsesAuthResp> {
        serde_json::from_value(self.0.get_subkey_jwt(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_system_config(&self, request: &GetSystemConfigParams) -> Result<ResponsesSystemConfigResponse> {
        serde_json::from_value(self.0.get_system_config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_system_state(&self, request: &GetSystemStateParams) -> Result<ResponsesSystemStateResponse> {
        serde_json::from_value(self.0.get_system_state(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_system_stats(&self, request: &GetSystemStatsParams) -> Result<ResponsesSystemStatsResponse> {
        serde_json::from_value(self.0.get_system_stats(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_system_time(&self, request: &GetSystemTimeParams) -> Result<ResponsesSystemTimeResponse> {
        serde_json::from_value(self.0.get_system_time(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_transfer_fee_config_v2(&self, request: &GetTransferFeeConfigV2Params) -> Result<ServicesTransferFeeConfigResponse> {
        serde_json::from_value(self.0.get_transfer_fee_config_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_transfers(&self, request: &GetTransfersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_transfers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_volume_tiers(&self, request: &GetVolumeTiersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_volume_tiers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_xp_balance_v2(&self, request: &GetXpBalanceV2Params) -> Result<ResponsesGetXPBalanceResponseV2> {
        serde_json::from_value(self.0.get_xp_balance_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_xp_transfer_by_id_v2(&self, request: &GetXpTransferByIdV2Params) -> Result<ResponsesXPTransfer> {
        serde_json::from_value(self.0.get_xp_transfer_by_id_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn get_xp_transfers_v2(&self, request: &GetXpTransfersV2Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_xp_transfers_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn jwks(&self, request: &JwksParams) -> Result<ResponsesJwksResponse> {
        serde_json::from_value(self.0.jwks(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn list_deposit_networks(&self, request: &ListDepositNetworksParams) -> Result<ResponsesDepositNetworksResp> {
        serde_json::from_value(self.0.list_deposit_networks(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn list_subkeys(&self, request: &ListSubkeysParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_subkeys(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn list_tokens(&self, request: &ListTokensParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_tokens(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn markets_klines(&self, request: &MarketsKlinesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.markets_klines(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn onboarding(&self, request: &OnboardingParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.onboarding(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn orders_cancel(&self, request: &OrdersCancelParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.orders_cancel(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn orders_cancel_all(&self, request: &OrdersCancelAllParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.orders_cancel_all(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn orders_cancel_batch(&self, request: &OrdersCancelBatchParams) -> Result<ResponsesCancelOrderBatchResponse> {
        serde_json::from_value(self.0.orders_cancel_batch(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn orders_cancel_by_client_id(&self, request: &OrdersCancelByClientIdParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.orders_cancel_by_client_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn orders_get(&self, request: &OrdersGetParams) -> Result<ResponsesOrderResp> {
        serde_json::from_value(self.0.orders_get(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn orders_get_by_client_id(&self, request: &OrdersGetByClientIdParams) -> Result<ResponsesOrderResp> {
        serde_json::from_value(self.0.orders_get_by_client_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn orders_modify(&self, request: &OrdersModifyParams) -> Result<ResponsesOrderResp> {
        serde_json::from_value(self.0.orders_modify(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn orders_new(&self, request: &OrdersNewParams) -> Result<ResponsesOrderResp> {
        serde_json::from_value(self.0.orders_new(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn orders_new_batch(&self, request: &OrdersNewBatchParams) -> Result<ResponsesBatchResponse> {
        serde_json::from_value(self.0.orders_new_batch(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn post_account_terminal_token(&self, request: &PostAccountTerminalTokenParams) -> Result<ResponsesAuthResp> {
        serde_json::from_value(self.0.post_account_terminal_token(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn record_compliance_action(&self, request: &RecordComplianceActionParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.record_compliance_action(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn refresh_account_inventory(&self, request: &RefreshAccountInventoryParams) -> Result<ResponsesRefreshInventoryResp> {
        serde_json::from_value(self.0.refresh_account_inventory(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn register_subkey(&self, request: &RegisterSubkeyParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.register_subkey(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn reset_account_mmp(&self, request: &ResetAccountMmpParams) -> Result<ResponsesAccountMmpResetResp> {
        serde_json::from_value(self.0.reset_account_mmp(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn revoke_subkey(&self, request: &RevokeSubkeyParams) -> Result<ResponsesRevokeSubkeyResponse> {
        serde_json::from_value(self.0.revoke_subkey(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn revoke_token(&self, request: &RevokeTokenParams) -> Result<ResponsesRevokeTokenResponse> {
        serde_json::from_value(self.0.revoke_token(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn rfq_bbo(&self, request: &RfqBboParams) -> Result<ResponsesRfqBboResult> {
        serde_json::from_value(self.0.rfq_bbo(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn rfq_create(&self, request: &RfqCreateParams) -> Result<ResponsesRfqAcceptedResponse> {
        serde_json::from_value(self.0.rfq_create(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn rfq_delete(&self, request: &RfqDeleteParams) -> Result<ResponsesRfqAcceptedResponse> {
        serde_json::from_value(self.0.rfq_delete(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn rfq_draft_create(&self, request: &RfqDraftCreateParams) -> Result<ResponsesRfqDraftResult> {
        serde_json::from_value(self.0.rfq_draft_create(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn rfq_draft_delete(&self, request: &RfqDraftDeleteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.rfq_draft_delete(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn rfq_draft_list(&self, request: &RfqDraftListParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.rfq_draft_list(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn rfq_enable(&self, request: &RfqEnableParams) -> Result<ResponsesParadigmEnableResponse> {
        serde_json::from_value(self.0.rfq_enable(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn rfq_execute(&self, request: &RfqExecuteParams) -> Result<ResponsesRfqAcceptedResponse> {
        serde_json::from_value(self.0.rfq_execute(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn rfq_list(&self, request: &RfqListParams) -> Result<ResponsesRfqResults> {
        serde_json::from_value(self.0.rfq_list(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn rfq_markets(&self, request: &RfqMarketsParams) -> Result<ResponsesRfqMarketsResult> {
        serde_json::from_value(self.0.rfq_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn rfq_platform_state(&self, request: &RfqPlatformStateParams) -> Result<ResponsesRfqPlatformStateResult> {
        serde_json::from_value(self.0.rfq_platform_state(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn tradebusts(&self, request: &TradebustsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.tradebusts(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn trades(&self, request: &TradesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn transactions(&self, request: &TransactionsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.transactions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn update_account_market_max_slippage(&self, request: &UpdateAccountMarketMaxSlippageParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_market_max_slippage(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn update_account_max_slippage(&self, request: &UpdateAccountMaxSlippageParams) -> Result<serde_json::Map<String, serde_json::Value>> {
        serde_json::from_value(self.0.update_account_max_slippage(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn update_account_notification_preferences(&self, request: &UpdateAccountNotificationPreferencesParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_notification_preferences(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn update_account_profile(&self, request: &UpdateAccountProfileParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_profile(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn update_account_profile_referral_code(&self, request: &UpdateAccountProfileReferralCodeParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_profile_referral_code(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn update_account_profile_username(&self, request: &UpdateAccountProfileUsernameParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_profile_username(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn update_account_referrer(&self, request: &UpdateAccountReferrerParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_referrer(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn update_account_size_currency_display(&self, request: &UpdateAccountSizeCurrencyDisplayParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_size_currency_display(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn update_account_trading_value_display(&self, request: &UpdateAccountTradingValueDisplayParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_trading_value_display(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn update_last_seen(&self, request: &UpdateLastSeenParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_last_seen(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn update_subkey(&self, request: &UpdateSubkeyParams) -> Result<ResponsesSubkey> {
        serde_json::from_value(self.0.update_subkey(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn update_subkey_allowed_cidrs(&self, request: &UpdateSubkeyAllowedCidrsParams) -> Result<ResponsesSubkey> {
        serde_json::from_value(self.0.update_subkey_allowed_cidrs(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn update_subkey_block_trade_signing(&self, request: &UpdateSubkeyBlockTradeSigningParams) -> Result<ResponsesSubkey> {
        serde_json::from_value(self.0.update_subkey_block_trade_signing(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn update_token_allowed_cidrs(&self, request: &UpdateTokenAllowedCidrsParams) -> Result<ResponsesApiToken> {
        serde_json::from_value(self.0.update_token_allowed_cidrs(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn upsert_account_margin(&self, request: &UpsertAccountMarginParams) -> Result<ResponsesUpdateAccountMarginConfigResp> {
        serde_json::from_value(self.0.upsert_account_margin(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn upsert_account_mmp(&self, request: &UpsertAccountMmpParams) -> Result<ResponsesAccountMmpConfigResp> {
        serde_json::from_value(self.0.upsert_account_mmp(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn vaults_create(&self, request: &VaultsCreateParams) -> Result<ResponsesVaultResp> {
        serde_json::from_value(self.0.vaults_create(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn vaults_get(&self, request: &VaultsGetParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn vaults_get_account_summary(&self, request: &VaultsGetAccountSummaryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get_account_summary(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn vaults_get_analytics(&self, request: &VaultsGetAnalyticsParams) -> Result<ResponsesVaultAnalyticsResp> {
        serde_json::from_value(self.0.vaults_get_analytics(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn vaults_get_balance(&self, request: &VaultsGetBalanceParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get_balance(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn vaults_get_config(&self, request: &VaultsGetConfigParams) -> Result<ResponsesVaultsConfigResponse> {
        serde_json::from_value(self.0.vaults_get_config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn vaults_get_historical_data(&self, request: &VaultsGetHistoricalDataParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get_historical_data(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn vaults_get_mine(&self, request: &VaultsGetMineParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get_mine(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn vaults_get_positions(&self, request: &VaultsGetPositionsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get_positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn vaults_get_summary(&self, request: &VaultsGetSummaryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get_summary(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub fn vaults_get_transfers(&self, request: &VaultsGetTransfersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get_transfers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedParadexClient(crate::AsyncClient<ParadexClient>);
#[cfg(feature = "async")]
impl TypedParadexClient {
    pub fn into_async(self) -> AsyncTypedParadexClient { AsyncTypedParadexClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedParadexClient {
    pub async fn activate_subkey(&self, request: &ActivateSubkeyParams) -> Result<RequestsActivateSubkeyResponse> {
        serde_json::from_value(self.0.activate_subkey(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn archive_sub_account(&self, request: &ArchiveSubAccountParams) -> Result<ResponsesAccountInfoResponse> {
        serde_json::from_value(self.0.archive_sub_account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn block_offers_cancel(&self, request: &BlockOffersCancelParams) -> Result<std::collections::BTreeMap<String, String>> {
        serde_json::from_value(self.0.block_offers_cancel(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn block_offers_create(&self, request: &BlockOffersCreateParams) -> Result<ResponsesBlockTradeDetailFullResponse> {
        serde_json::from_value(self.0.block_offers_create(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn block_offers_execute(&self, request: &BlockOffersExecuteParams) -> Result<ResponsesBlockTradeDetailFullResponse> {
        serde_json::from_value(self.0.block_offers_execute(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn block_offers_get(&self, request: &BlockOffersGetParams) -> Result<ResponsesBlockTradeDetailFullResponse> {
        serde_json::from_value(self.0.block_offers_get(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn block_offers_list(&self, request: &BlockOffersListParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.block_offers_list(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn block_trades_cancel(&self, request: &BlockTradesCancelParams) -> Result<std::collections::BTreeMap<String, String>> {
        serde_json::from_value(self.0.block_trades_cancel(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn block_trades_create(&self, request: &BlockTradesCreateParams) -> Result<ResponsesBlockTradeDetailFullResponse> {
        serde_json::from_value(self.0.block_trades_create(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn block_trades_execute(&self, request: &BlockTradesExecuteParams) -> Result<ResponsesBlockTradeDetailFullResponse> {
        serde_json::from_value(self.0.block_trades_execute(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn block_trades_get(&self, request: &BlockTradesGetParams) -> Result<ResponsesBlockTradeDetailFullResponse> {
        serde_json::from_value(self.0.block_trades_get(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn block_trades_list(&self, request: &BlockTradesListParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.block_trades_list(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn cancel_algo_order(&self, request: &CancelAlgoOrderParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.cancel_algo_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn create_algo_order(&self, request: &CreateAlgoOrderParams) -> Result<ResponsesAlgoOrderResp> {
        serde_json::from_value(self.0.create_algo_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn create_deposit_address(&self, request: &CreateDepositAddressParams) -> Result<ResponsesDepositAddressResp> {
        serde_json::from_value(self.0.create_deposit_address(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn create_token(&self, request: &CreateTokenParams) -> Result<ResponsesCreateTokenResponse> {
        serde_json::from_value(self.0.create_token(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn create_xp_transfer_v2(&self, request: &CreateXpTransferV2Params) -> Result<ResponsesCreateTransferResponse> {
        serde_json::from_value(self.0.create_xp_transfer_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn delete_account_mmp(&self, request: &DeleteAccountMmpParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_account_mmp(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn evm_auth(&self, request: &EvmAuthParams) -> Result<ResponsesAuthResp> {
        serde_json::from_value(self.0.evm_auth(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn evm_onboarding(&self, request: &EvmOnboardingParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.evm_onboarding(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_account(&self, request: &GetAccountParams) -> Result<ResponsesAccountSummaryResponse> {
        serde_json::from_value(self.0.get_account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_account_history(&self, request: &GetAccountHistoryParams) -> Result<ResponsesAccountHistoricalDataResp> {
        serde_json::from_value(self.0.get_account_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_account_margin(&self, request: &GetAccountMarginParams) -> Result<ResponsesGetAccountMarginConfigsResp> {
        serde_json::from_value(self.0.get_account_margin(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_account_mmp(&self, request: &GetAccountMmpParams) -> Result<ResponsesGetAccountMmpConfigsResp> {
        serde_json::from_value(self.0.get_account_mmp(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_account_profile(&self, request: &GetAccountProfileParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.get_account_profile(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_account_settings(&self, request: &GetAccountSettingsParams) -> Result<ResponsesAccountSettingsResp> {
        serde_json::from_value(self.0.get_account_settings(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_accounts(&self, request: &GetAccountsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_accounts(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_accounts_info(&self, request: &GetAccountsInfoParams) -> Result<ResponsesGetAccountsInfoResponse> {
        serde_json::from_value(self.0.get_accounts_info(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_algo_order_by_id(&self, request: &GetAlgoOrderByIdParams) -> Result<ResponsesAlgoOrderResp> {
        serde_json::from_value(self.0.get_algo_order_by_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_algo_orders_history(&self, request: &GetAlgoOrdersHistoryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_algo_orders_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_announcements(&self, request: &GetAnnouncementsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_announcements(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_balance(&self, request: &GetBalanceParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_balance(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_bbo(&self, request: &GetBboParams) -> Result<ResponsesBBOResp> {
        serde_json::from_value(self.0.get_bbo(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_bbo_interactive(&self, request: &GetBboInteractiveParams) -> Result<ResponsesBBOResp> {
        serde_json::from_value(self.0.get_bbo_interactive(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_compliance_actions(&self, request: &GetComplianceActionsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_compliance_actions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_deposit_address(&self, request: &GetDepositAddressParams) -> Result<ResponsesDepositAddressDetailsResp> {
        serde_json::from_value(self.0.get_deposit_address(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_dime_config(&self, request: &GetDimeConfigParams) -> Result<ResponsesDimeConfigResponse> {
        serde_json::from_value(self.0.get_dime_config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_fills(&self, request: &GetFillsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_funding(&self, request: &GetFundingParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_funding(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_funding_data(&self, request: &GetFundingDataParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_funding_data(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_global_staking_balance_history(&self, request: &GetGlobalStakingBalanceHistoryParams) -> Result<ResponsesAccountHistoricalDataResp> {
        serde_json::from_value(self.0.get_global_staking_balance_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_impact_price(&self, request: &GetImpactPriceParams) -> Result<ResponsesImpactPriceResp> {
        serde_json::from_value(self.0.get_impact_price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_insurance_fund(&self, request: &GetInsuranceFundParams) -> Result<ResponsesInsuranceAccountResp> {
        serde_json::from_value(self.0.get_insurance_fund(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_jwt(&self, request: &GetJwtParams) -> Result<ResponsesAuthResp> {
        serde_json::from_value(self.0.get_jwt(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_liquidations(&self, request: &GetLiquidationsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_liquidations(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_margin_config(&self, request: &GetMarginConfigParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_margin_config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_market_settlement_price(&self, request: &GetMarketSettlementPriceParams) -> Result<ResponsesMarketSettlementResp> {
        serde_json::from_value(self.0.get_market_settlement_price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_markets(&self, request: &GetMarketsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_markets_history(&self, request: &GetMarketsHistoryParams) -> Result<ResponsesMarketHistoryResponse> {
        serde_json::from_value(self.0.get_markets_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_markets_summary(&self, request: &GetMarketsSummaryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_markets_summary(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_onboarding(&self, request: &GetOnboardingParams) -> Result<ResponsesGetOnboardingResp> {
        serde_json::from_value(self.0.get_onboarding(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_open_algo_orders(&self, request: &GetOpenAlgoOrdersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_open_algo_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_open_orders(&self, request: &GetOpenOrdersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_orderbook(&self, request: &GetOrderbookParams) -> Result<ResponsesAskBidArray> {
        serde_json::from_value(self.0.get_orderbook(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_orderbook_interactive(&self, request: &GetOrderbookInteractiveParams) -> Result<ResponsesAskBidArray> {
        serde_json::from_value(self.0.get_orderbook_interactive(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_orders(&self, request: &GetOrdersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_portfolio_margin_config(&self, request: &GetPortfolioMarginConfigParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_portfolio_margin_config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_positions(&self, request: &GetPositionsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_public_xp_transfers_v2(&self, request: &GetPublicXpTransfersV2Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_public_xp_transfers_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_referral_qr_code(&self, request: &GetReferralQrCodeParams) -> Result<String> {
        serde_json::from_value(self.0.get_referral_qr_code(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_referrals_config(&self, request: &GetReferralsConfigParams) -> Result<serde_json::Map<String, serde_json::Value>> {
        serde_json::from_value(self.0.get_referrals_config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_referrals_summary(&self, request: &GetReferralsSummaryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_referrals_summary(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_staking_balance_history(&self, request: &GetStakingBalanceHistoryParams) -> Result<ResponsesAccountHistoricalDataResp> {
        serde_json::from_value(self.0.get_staking_balance_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_staking_config(&self, request: &GetStakingConfigParams) -> Result<ResponsesStakingConfigResponse> {
        serde_json::from_value(self.0.get_staking_config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_staking_history(&self, request: &GetStakingHistoryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_staking_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_staking_summary(&self, request: &GetStakingSummaryParams) -> Result<ResponsesStakingSummaryResponse> {
        serde_json::from_value(self.0.get_staking_summary(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_sub_accounts(&self, request: &GetSubAccountsParams) -> Result<ResponsesGetSubAccountsResponse> {
        serde_json::from_value(self.0.get_sub_accounts(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_subkey(&self, request: &GetSubkeyParams) -> Result<ResponsesSubkey> {
        serde_json::from_value(self.0.get_subkey(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_subkey_jwt(&self, request: &GetSubkeyJwtParams) -> Result<ResponsesAuthResp> {
        serde_json::from_value(self.0.get_subkey_jwt(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_system_config(&self, request: &GetSystemConfigParams) -> Result<ResponsesSystemConfigResponse> {
        serde_json::from_value(self.0.get_system_config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_system_state(&self, request: &GetSystemStateParams) -> Result<ResponsesSystemStateResponse> {
        serde_json::from_value(self.0.get_system_state(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_system_stats(&self, request: &GetSystemStatsParams) -> Result<ResponsesSystemStatsResponse> {
        serde_json::from_value(self.0.get_system_stats(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_system_time(&self, request: &GetSystemTimeParams) -> Result<ResponsesSystemTimeResponse> {
        serde_json::from_value(self.0.get_system_time(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_transfer_fee_config_v2(&self, request: &GetTransferFeeConfigV2Params) -> Result<ServicesTransferFeeConfigResponse> {
        serde_json::from_value(self.0.get_transfer_fee_config_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_transfers(&self, request: &GetTransfersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_transfers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_volume_tiers(&self, request: &GetVolumeTiersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_volume_tiers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_xp_balance_v2(&self, request: &GetXpBalanceV2Params) -> Result<ResponsesGetXPBalanceResponseV2> {
        serde_json::from_value(self.0.get_xp_balance_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_xp_transfer_by_id_v2(&self, request: &GetXpTransferByIdV2Params) -> Result<ResponsesXPTransfer> {
        serde_json::from_value(self.0.get_xp_transfer_by_id_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn get_xp_transfers_v2(&self, request: &GetXpTransfersV2Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_xp_transfers_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn jwks(&self, request: &JwksParams) -> Result<ResponsesJwksResponse> {
        serde_json::from_value(self.0.jwks(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn list_deposit_networks(&self, request: &ListDepositNetworksParams) -> Result<ResponsesDepositNetworksResp> {
        serde_json::from_value(self.0.list_deposit_networks(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn list_subkeys(&self, request: &ListSubkeysParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_subkeys(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn list_tokens(&self, request: &ListTokensParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_tokens(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn markets_klines(&self, request: &MarketsKlinesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.markets_klines(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn onboarding(&self, request: &OnboardingParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.onboarding(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn orders_cancel(&self, request: &OrdersCancelParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.orders_cancel(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn orders_cancel_all(&self, request: &OrdersCancelAllParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.orders_cancel_all(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn orders_cancel_batch(&self, request: &OrdersCancelBatchParams) -> Result<ResponsesCancelOrderBatchResponse> {
        serde_json::from_value(self.0.orders_cancel_batch(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn orders_cancel_by_client_id(&self, request: &OrdersCancelByClientIdParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.orders_cancel_by_client_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn orders_get(&self, request: &OrdersGetParams) -> Result<ResponsesOrderResp> {
        serde_json::from_value(self.0.orders_get(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn orders_get_by_client_id(&self, request: &OrdersGetByClientIdParams) -> Result<ResponsesOrderResp> {
        serde_json::from_value(self.0.orders_get_by_client_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn orders_modify(&self, request: &OrdersModifyParams) -> Result<ResponsesOrderResp> {
        serde_json::from_value(self.0.orders_modify(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn orders_new(&self, request: &OrdersNewParams) -> Result<ResponsesOrderResp> {
        serde_json::from_value(self.0.orders_new(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn orders_new_batch(&self, request: &OrdersNewBatchParams) -> Result<ResponsesBatchResponse> {
        serde_json::from_value(self.0.orders_new_batch(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn post_account_terminal_token(&self, request: &PostAccountTerminalTokenParams) -> Result<ResponsesAuthResp> {
        serde_json::from_value(self.0.post_account_terminal_token(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn record_compliance_action(&self, request: &RecordComplianceActionParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.record_compliance_action(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn refresh_account_inventory(&self, request: &RefreshAccountInventoryParams) -> Result<ResponsesRefreshInventoryResp> {
        serde_json::from_value(self.0.refresh_account_inventory(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn register_subkey(&self, request: &RegisterSubkeyParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.register_subkey(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn reset_account_mmp(&self, request: &ResetAccountMmpParams) -> Result<ResponsesAccountMmpResetResp> {
        serde_json::from_value(self.0.reset_account_mmp(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn revoke_subkey(&self, request: &RevokeSubkeyParams) -> Result<ResponsesRevokeSubkeyResponse> {
        serde_json::from_value(self.0.revoke_subkey(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn revoke_token(&self, request: &RevokeTokenParams) -> Result<ResponsesRevokeTokenResponse> {
        serde_json::from_value(self.0.revoke_token(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn rfq_bbo(&self, request: &RfqBboParams) -> Result<ResponsesRfqBboResult> {
        serde_json::from_value(self.0.rfq_bbo(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn rfq_create(&self, request: &RfqCreateParams) -> Result<ResponsesRfqAcceptedResponse> {
        serde_json::from_value(self.0.rfq_create(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn rfq_delete(&self, request: &RfqDeleteParams) -> Result<ResponsesRfqAcceptedResponse> {
        serde_json::from_value(self.0.rfq_delete(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn rfq_draft_create(&self, request: &RfqDraftCreateParams) -> Result<ResponsesRfqDraftResult> {
        serde_json::from_value(self.0.rfq_draft_create(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn rfq_draft_delete(&self, request: &RfqDraftDeleteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.rfq_draft_delete(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn rfq_draft_list(&self, request: &RfqDraftListParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.rfq_draft_list(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn rfq_enable(&self, request: &RfqEnableParams) -> Result<ResponsesParadigmEnableResponse> {
        serde_json::from_value(self.0.rfq_enable(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn rfq_execute(&self, request: &RfqExecuteParams) -> Result<ResponsesRfqAcceptedResponse> {
        serde_json::from_value(self.0.rfq_execute(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn rfq_list(&self, request: &RfqListParams) -> Result<ResponsesRfqResults> {
        serde_json::from_value(self.0.rfq_list(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn rfq_markets(&self, request: &RfqMarketsParams) -> Result<ResponsesRfqMarketsResult> {
        serde_json::from_value(self.0.rfq_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn rfq_platform_state(&self, request: &RfqPlatformStateParams) -> Result<ResponsesRfqPlatformStateResult> {
        serde_json::from_value(self.0.rfq_platform_state(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn tradebusts(&self, request: &TradebustsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.tradebusts(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn trades(&self, request: &TradesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn transactions(&self, request: &TransactionsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.transactions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn update_account_market_max_slippage(&self, request: &UpdateAccountMarketMaxSlippageParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_market_max_slippage(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn update_account_max_slippage(&self, request: &UpdateAccountMaxSlippageParams) -> Result<serde_json::Map<String, serde_json::Value>> {
        serde_json::from_value(self.0.update_account_max_slippage(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn update_account_notification_preferences(&self, request: &UpdateAccountNotificationPreferencesParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_notification_preferences(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn update_account_profile(&self, request: &UpdateAccountProfileParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_profile(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn update_account_profile_referral_code(&self, request: &UpdateAccountProfileReferralCodeParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_profile_referral_code(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn update_account_profile_username(&self, request: &UpdateAccountProfileUsernameParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_profile_username(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn update_account_referrer(&self, request: &UpdateAccountReferrerParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_referrer(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn update_account_size_currency_display(&self, request: &UpdateAccountSizeCurrencyDisplayParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_size_currency_display(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn update_account_trading_value_display(&self, request: &UpdateAccountTradingValueDisplayParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_account_trading_value_display(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn update_last_seen(&self, request: &UpdateLastSeenParams) -> Result<ResponsesAccountProfileResp> {
        serde_json::from_value(self.0.update_last_seen(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn update_subkey(&self, request: &UpdateSubkeyParams) -> Result<ResponsesSubkey> {
        serde_json::from_value(self.0.update_subkey(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn update_subkey_allowed_cidrs(&self, request: &UpdateSubkeyAllowedCidrsParams) -> Result<ResponsesSubkey> {
        serde_json::from_value(self.0.update_subkey_allowed_cidrs(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn update_subkey_block_trade_signing(&self, request: &UpdateSubkeyBlockTradeSigningParams) -> Result<ResponsesSubkey> {
        serde_json::from_value(self.0.update_subkey_block_trade_signing(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn update_token_allowed_cidrs(&self, request: &UpdateTokenAllowedCidrsParams) -> Result<ResponsesApiToken> {
        serde_json::from_value(self.0.update_token_allowed_cidrs(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn upsert_account_margin(&self, request: &UpsertAccountMarginParams) -> Result<ResponsesUpdateAccountMarginConfigResp> {
        serde_json::from_value(self.0.upsert_account_margin(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn upsert_account_mmp(&self, request: &UpsertAccountMmpParams) -> Result<ResponsesAccountMmpConfigResp> {
        serde_json::from_value(self.0.upsert_account_mmp(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn vaults_create(&self, request: &VaultsCreateParams) -> Result<ResponsesVaultResp> {
        serde_json::from_value(self.0.vaults_create(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn vaults_get(&self, request: &VaultsGetParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn vaults_get_account_summary(&self, request: &VaultsGetAccountSummaryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get_account_summary(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn vaults_get_analytics(&self, request: &VaultsGetAnalyticsParams) -> Result<ResponsesVaultAnalyticsResp> {
        serde_json::from_value(self.0.vaults_get_analytics(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn vaults_get_balance(&self, request: &VaultsGetBalanceParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get_balance(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn vaults_get_config(&self, request: &VaultsGetConfigParams) -> Result<ResponsesVaultsConfigResponse> {
        serde_json::from_value(self.0.vaults_get_config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn vaults_get_historical_data(&self, request: &VaultsGetHistoricalDataParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get_historical_data(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn vaults_get_mine(&self, request: &VaultsGetMineParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get_mine(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn vaults_get_positions(&self, request: &VaultsGetPositionsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get_positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn vaults_get_summary(&self, request: &VaultsGetSummaryParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get_summary(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
    pub async fn vaults_get_transfers(&self, request: &VaultsGetTransfersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.vaults_get_transfers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paradex response model"))
    }
}
