// Generated from src/exchanges/polymarket/schema/polymarket.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{PolymarketClient, Error, Result};

pub type BridgeAddress = String;
pub type BuilderGatewayAcceptRfqResponse = serde_json::Value;
pub type BuilderGatewayCreateRfqResponse = serde_json::Value;
pub type BuilderGatewayGetRfqResponse = serde_json::Value;
pub type ClobClobRewards = serde_json::Map<String,serde_json::Value>;
pub type ClobDeleteApiKeyResponse = String;
pub type ClobDropNotificationsResponse = String;
pub type ClobGetBooksGetResponse = Vec<ClobOrderBookSummary>;
pub type ClobGetBooksPostResponse = Vec<ClobOrderBookSummary>;
pub type ClobGetBuilderApiKeysResponse = Vec<String>;
pub type ClobGetCurrentRebatedFeesResponse = Vec<ClobRebatedFees>;
pub type ClobGetLastTradesPricesGetResponse = Vec<ClobGetLastTradesPricesGetResponseValueItem>;
pub type ClobGetLastTradesPricesPostResponse = Vec<ClobGetLastTradesPricesPostResponseValueItem>;
pub type ClobGetMarketsLiveActivityResponse = Vec<ClobLiveActivityMarket>;
pub type ClobGetMidpointsGetResponse = std::collections::BTreeMap<String,String>;
pub type ClobGetMidpointsPostResponse = std::collections::BTreeMap<String,String>;
pub type ClobGetNotificationsResponse = Vec<ClobNotification>;
pub type ClobGetOrdersScoringResponse = std::collections::BTreeMap<String,bool>;
pub type ClobGetPricesGetResponse = std::collections::BTreeMap<String,std::collections::BTreeMap<String,serde_json::Number>>;
pub type ClobGetPricesPostResponse = std::collections::BTreeMap<String,std::collections::BTreeMap<String,serde_json::Number>>;
pub type ClobGetRewardPercentagesForUserResponse = std::collections::BTreeMap<String,serde_json::Number>;
pub type ClobGetSpreadsResponse = std::collections::BTreeMap<String,String>;
pub type ClobGetTimeResponse = serde_json::Number;
pub type ClobGetTotalEarningsForUserForDayResponse = Vec<ClobTotalUserEarning>;
pub type ClobListSessionSignersResponse = serde_json::Value;
pub type ClobOrdersScoringResponse = std::collections::BTreeMap<String,bool>;
pub type ClobPostOrdersResponse = Vec<ClobSendOrderResponse>;
pub type ClobPostOrdersScoringResponse = std::collections::BTreeMap<String,bool>;
pub type ClobRevokeBuilderApiKeyResponse = String;
pub type CollateralReturnPlanResponse = serde_json::Value;
pub type CollateralReturnSubmitResponse = serde_json::Value;
pub type CombosRfqDirection = String;
pub type CombosRfqRFQStatus = String;
pub type CombosRfqSide = String;
pub type CombosRfqSignatureType = serde_json::Number;
pub type DataErrorCode = String;
pub type DataGetLeaderboardResponse = serde_json::Value;
pub type DataLeaderboardResponse = serde_json::Value;
pub type DataTokenModule = String;
pub type GammaGetAbridgedMarketsResponse = Vec<GammaMarket>;
pub type GammaGetCommentsByIdResponse = Vec<GammaComment>;
pub type GammaGetCommentsByUserAddressResponse = Vec<GammaComment>;
pub type GammaGetEventTagsResponse = Vec<GammaTag>;
pub type GammaGetGammaStatusResponse = serde_json::Value;
pub type GammaGetMarketTagsResponse = Vec<GammaTag>;
pub type GammaGetMarketsInformationResponse = Vec<GammaMarket>;
pub type GammaGetRelatedTagsByIdResponse = Vec<GammaRelatedTag>;
pub type GammaGetRelatedTagsBySlugResponse = Vec<GammaRelatedTag>;
pub type GammaGetSportsMetadataResponse = Vec<GammaSportsMetadata>;
pub type GammaGetTagsRelatedToATagByIdResponse = Vec<GammaTag>;
pub type GammaGetTagsRelatedToATagBySlugResponse = Vec<GammaTag>;
pub type GammaListCommentsResponse = Vec<GammaComment>;
pub type GammaListEventCreatorsResponse = Vec<GammaEventCreator>;
pub type GammaListEventsResponse = Vec<GammaEvent>;
pub type GammaListMarketsResponse = Vec<GammaMarket>;
pub type GammaListSeriesResponse = Vec<GammaSeries>;
pub type GammaListSportEventsResultsResponse = Vec<GammaEvent>;
pub type GammaListTagsResponse = Vec<GammaTag>;
pub type GammaListTeamsResponse = Vec<GammaTeam>;
pub type RelayerAddress = String;
pub type RelayerAuthorizationsResponse = serde_json::Value;
pub type RelayerGetExecuteParamsResponse = serde_json::Value;
pub type RelayerGetRelayerApiKeysResponse = Vec<RelayerRelayerApiKey>;
pub type RelayerGetTransactionResponse = Vec<RelayerRelayerTransaction>;
pub type RelayerGetTransactionsResponse = Vec<RelayerRelayerTransaction>;
pub type RelayerRevocationsResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeDepositRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeDepositResponse {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<BridgeDepositResponseAddress>,
    #[serde(rename = "note", default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeDepositResponseAddress {
    #[serde(rename = "evm", default, skip_serializing_if = "Option::is_none")]
    pub evm: Option<String>,
    #[serde(rename = "svm", default, skip_serializing_if = "Option::is_none")]
    pub svm: Option<String>,
    #[serde(rename = "btc", default, skip_serializing_if = "Option::is_none")]
    pub btc: Option<String>,
    #[serde(rename = "tron", default, skip_serializing_if = "Option::is_none")]
    pub tron: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeErrorResponse {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeFeeBreakdown {
    #[serde(rename = "appFeeLabel", default, skip_serializing_if = "Option::is_none")]
    pub app_fee_label: Option<String>,
    #[serde(rename = "appFeePercent", default, skip_serializing_if = "Option::is_none")]
    pub app_fee_percent: Option<serde_json::Number>,
    #[serde(rename = "appFeeUsd", default, skip_serializing_if = "Option::is_none")]
    pub app_fee_usd: Option<serde_json::Number>,
    #[serde(rename = "fillCostPercent", default, skip_serializing_if = "Option::is_none")]
    pub fill_cost_percent: Option<serde_json::Number>,
    #[serde(rename = "fillCostUsd", default, skip_serializing_if = "Option::is_none")]
    pub fill_cost_usd: Option<serde_json::Number>,
    #[serde(rename = "gasUsd", default, skip_serializing_if = "Option::is_none")]
    pub gas_usd: Option<serde_json::Number>,
    #[serde(rename = "maxSlippage", default, skip_serializing_if = "Option::is_none")]
    pub max_slippage: Option<serde_json::Number>,
    #[serde(rename = "minReceived", default, skip_serializing_if = "Option::is_none")]
    pub min_received: Option<serde_json::Number>,
    #[serde(rename = "swapImpact", default, skip_serializing_if = "Option::is_none")]
    pub swap_impact: Option<serde_json::Number>,
    #[serde(rename = "swapImpactUsd", default, skip_serializing_if = "Option::is_none")]
    pub swap_impact_usd: Option<serde_json::Number>,
    #[serde(rename = "totalImpact", default, skip_serializing_if = "Option::is_none")]
    pub total_impact: Option<serde_json::Number>,
    #[serde(rename = "totalImpactUsd", default, skip_serializing_if = "Option::is_none")]
    pub total_impact_usd: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeGetStatusAddressParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "paginate", default, skip_serializing_if = "Option::is_none")]
    pub paginate: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeGetStatusAddressResponse {
    #[serde(rename = "transactions")]
    pub transactions: Vec<BridgeTransaction>,
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeGetSupportedAssetsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeGetSupportedAssetsResponse {
    #[serde(rename = "supportedAssets", default, skip_serializing_if = "Option::is_none")]
    pub supported_assets: Option<Vec<BridgeSupportedAsset>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgePostDepositParams {
    #[serde(rename = "X-Builder-Code", default, skip_serializing_if = "Option::is_none")]
    pub x_builder_code: Option<String>,
    #[serde(rename = "body")]
    pub body: Box<BridgeDepositRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgePostDepositResponse {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<BridgePostDepositResponseAddress>,
    #[serde(rename = "note", default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgePostDepositResponseAddress {
    #[serde(rename = "evm", default, skip_serializing_if = "Option::is_none")]
    pub evm: Option<String>,
    #[serde(rename = "svm", default, skip_serializing_if = "Option::is_none")]
    pub svm: Option<String>,
    #[serde(rename = "btc", default, skip_serializing_if = "Option::is_none")]
    pub btc: Option<String>,
    #[serde(rename = "tron", default, skip_serializing_if = "Option::is_none")]
    pub tron: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgePostQuoteParams {
    #[serde(rename = "body")]
    pub body: Box<BridgeQuoteRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgePostQuoteResponse {
    #[serde(rename = "estCheckoutTimeMs", default, skip_serializing_if = "Option::is_none")]
    pub est_checkout_time_ms: Option<serde_json::Number>,
    #[serde(rename = "estFeeBreakdown", default, skip_serializing_if = "Option::is_none")]
    pub est_fee_breakdown: Option<Box<BridgeFeeBreakdown>>,
    #[serde(rename = "estInputUsd", default, skip_serializing_if = "Option::is_none")]
    pub est_input_usd: Option<serde_json::Number>,
    #[serde(rename = "estOutputUsd", default, skip_serializing_if = "Option::is_none")]
    pub est_output_usd: Option<serde_json::Number>,
    #[serde(rename = "estToTokenBaseUnit", default, skip_serializing_if = "Option::is_none")]
    pub est_to_token_base_unit: Option<String>,
    #[serde(rename = "quoteId", default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgePostWithdrawParams {
    #[serde(rename = "X-Builder-Code", default, skip_serializing_if = "Option::is_none")]
    pub x_builder_code: Option<String>,
    #[serde(rename = "body")]
    pub body: Box<BridgeWithdrawalRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgePostWithdrawResponse {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<BridgePostWithdrawResponseAddress>,
    #[serde(rename = "note", default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgePostWithdrawResponseAddress {
    #[serde(rename = "evm", default, skip_serializing_if = "Option::is_none")]
    pub evm: Option<String>,
    #[serde(rename = "svm", default, skip_serializing_if = "Option::is_none")]
    pub svm: Option<String>,
    #[serde(rename = "btc", default, skip_serializing_if = "Option::is_none")]
    pub btc: Option<String>,
    #[serde(rename = "tron", default, skip_serializing_if = "Option::is_none")]
    pub tron: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeQuoteRequest {
    #[serde(rename = "fromAmountBaseUnit")]
    pub from_amount_base_unit: String,
    #[serde(rename = "fromChainId")]
    pub from_chain_id: String,
    #[serde(rename = "fromTokenAddress")]
    pub from_token_address: String,
    #[serde(rename = "recipientAddress")]
    pub recipient_address: String,
    #[serde(rename = "toChainId")]
    pub to_chain_id: String,
    #[serde(rename = "toTokenAddress")]
    pub to_token_address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeQuoteResponse {
    #[serde(rename = "estCheckoutTimeMs", default, skip_serializing_if = "Option::is_none")]
    pub est_checkout_time_ms: Option<serde_json::Number>,
    #[serde(rename = "estFeeBreakdown", default, skip_serializing_if = "Option::is_none")]
    pub est_fee_breakdown: Option<Box<BridgeFeeBreakdown>>,
    #[serde(rename = "estInputUsd", default, skip_serializing_if = "Option::is_none")]
    pub est_input_usd: Option<serde_json::Number>,
    #[serde(rename = "estOutputUsd", default, skip_serializing_if = "Option::is_none")]
    pub est_output_usd: Option<serde_json::Number>,
    #[serde(rename = "estToTokenBaseUnit", default, skip_serializing_if = "Option::is_none")]
    pub est_to_token_base_unit: Option<String>,
    #[serde(rename = "quoteId", default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeSupportedAsset {
    #[serde(rename = "chainId", default, skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    #[serde(rename = "chainName", default, skip_serializing_if = "Option::is_none")]
    pub chain_name: Option<String>,
    #[serde(rename = "token", default, skip_serializing_if = "Option::is_none")]
    pub token: Option<Box<BridgeToken>>,
    #[serde(rename = "minCheckoutUsd", default, skip_serializing_if = "Option::is_none")]
    pub min_checkout_usd: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeSupportedAssetsResponse {
    #[serde(rename = "supportedAssets", default, skip_serializing_if = "Option::is_none")]
    pub supported_assets: Option<Vec<BridgeSupportedAsset>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeToken {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "decimals", default, skip_serializing_if = "Option::is_none")]
    pub decimals: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeTransaction {
    #[serde(rename = "fromChainId", default, skip_serializing_if = "Option::is_none")]
    pub from_chain_id: Option<String>,
    #[serde(rename = "fromTokenAddress", default, skip_serializing_if = "Option::is_none")]
    pub from_token_address: Option<String>,
    #[serde(rename = "fromAmountBaseUnit", default, skip_serializing_if = "Option::is_none")]
    pub from_amount_base_unit: Option<String>,
    #[serde(rename = "toChainId", default, skip_serializing_if = "Option::is_none")]
    pub to_chain_id: Option<String>,
    #[serde(rename = "toTokenAddress", default, skip_serializing_if = "Option::is_none")]
    pub to_token_address: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "txHash", default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "createdTimeMs", default, skip_serializing_if = "Option::is_none")]
    pub created_time_ms: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeTransactionStatusResponse {
    #[serde(rename = "transactions")]
    pub transactions: Vec<BridgeTransaction>,
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BridgeWithdrawalRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "toChainId")]
    pub to_chain_id: String,
    #[serde(rename = "toTokenAddress")]
    pub to_token_address: String,
    #[serde(rename = "recipientAddr")]
    pub recipient_addr: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuilderGatewayAcceptRfqParams {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "body")]
    pub body: BuilderGatewayAcceptRfqParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuilderGatewayAcceptRfqParamsBody {
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "signed_order")]
    pub signed_order: serde_json::Map<String,serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuilderGatewayCreateRfqParams {
    #[serde(rename = "body")]
    pub body: BuilderGatewayCreateRfqParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BuilderGatewayCreateRfqParamsBody {
    #[serde(rename = "direction")]
    pub direction: String,
    #[serde(rename = "leg_position_ids")]
    pub leg_position_ids: Vec<String>,
    #[serde(rename = "maker_address")]
    pub maker_address: String,
    #[serde(rename = "requested_size")]
    pub requested_size: BuilderGatewayCreateRfqParamsBodyRequestedSize,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "signature_type")]
    pub signature_type: serde_json::Number,
    #[serde(rename = "signer_address")]
    pub signer_address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for BuilderGatewayCreateRfqParamsBody {
    fn default() -> Self {
        Self {
            direction: Default::default(),
            leg_position_ids: Default::default(),
            maker_address: Default::default(),
            requested_size: Default::default(),
            side: Default::default(),
            signature_type: 0.into(),
            signer_address: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuilderGatewayCreateRfqParamsBodyRequestedSize {
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "value_e6")]
    pub value_e6: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuilderGatewayGetRfqParams {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobApiKeyResponse {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(rename = "passphrase")]
    pub passphrase: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobApiKeysResponse {
    #[serde(rename = "apiKeys")]
    pub api_keys: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobAssetEarning {
    #[serde(rename = "asset_address")]
    pub asset_address: String,
    #[serde(rename = "earnings")]
    pub earnings: serde_json::Number,
    #[serde(rename = "asset_rate", default, skip_serializing_if = "Option::is_none")]
    pub asset_rate: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobAssetEarning {
    fn default() -> Self {
        Self {
            asset_address: Default::default(),
            earnings: 0.into(),
            asset_rate: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobBalanceAllowanceResponse {
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "allowances")]
    pub allowances: std::collections::BTreeMap<String,String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobBatchPricesHistoryRequest {
    #[serde(rename = "markets")]
    pub markets: Vec<String>,
    #[serde(rename = "start_ts", default, skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<serde_json::Number>,
    #[serde(rename = "end_ts", default, skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<serde_json::Number>,
    #[serde(rename = "interval", default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "fidelity", default, skip_serializing_if = "Option::is_none")]
    pub fidelity: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobBatchPricesHistoryResponse {
    #[serde(rename = "history", default, skip_serializing_if = "Option::is_none")]
    pub history: Option<std::collections::BTreeMap<String,Vec<ClobMarketPrice>>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobBookRequest {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobBuilderApiKeyResponse {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(rename = "passphrase")]
    pub passphrase: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobBuilderTrade {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "tradeType")]
    pub trade_type: String,
    #[serde(rename = "takerOrderHash")]
    pub taker_order_hash: String,
    #[serde(rename = "builder")]
    pub builder: String,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "sizeUsdc")]
    pub size_usdc: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "outcome")]
    pub outcome: String,
    #[serde(rename = "outcomeIndex")]
    pub outcome_index: serde_json::Number,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "matchTime")]
    pub match_time: String,
    #[serde(rename = "bucketIndex")]
    pub bucket_index: serde_json::Number,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "feeUsdc")]
    pub fee_usdc: String,
    #[serde(rename = "err_msg", default, skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobBuilderTrade {
    fn default() -> Self {
        Self {
            id: Default::default(),
            trade_type: Default::default(),
            taker_order_hash: Default::default(),
            builder: Default::default(),
            market: Default::default(),
            asset_id: Default::default(),
            side: Default::default(),
            size: Default::default(),
            size_usdc: Default::default(),
            price: Default::default(),
            status: Default::default(),
            outcome: Default::default(),
            outcome_index: 0.into(),
            owner: Default::default(),
            maker: Default::default(),
            transaction_hash: Default::default(),
            match_time: Default::default(),
            bucket_index: 0.into(),
            fee: Default::default(),
            fee_usdc: Default::default(),
            err_msg: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobBuilderTradesResponse {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "data")]
    pub data: Vec<ClobBuilderTrade>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobBuilderTradesResponse {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            next_cursor: Default::default(),
            count: 0.into(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobCancelAllOrdersParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobCancelAllOrdersResponse {
    #[serde(rename = "canceled")]
    pub canceled: Vec<String>,
    #[serde(rename = "not_canceled")]
    pub not_canceled: std::collections::BTreeMap<String,String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobCancelMarketOrdersParams {
    #[serde(rename = "body")]
    pub body: Box<ClobOrderMarketCancelParams>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobCancelMarketOrdersResponse {
    #[serde(rename = "canceled")]
    pub canceled: Vec<String>,
    #[serde(rename = "not_canceled")]
    pub not_canceled: std::collections::BTreeMap<String,String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobCancelOrderParams {
    #[serde(rename = "body")]
    pub body: Box<ClobCancelOrderPayload>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobCancelOrderPayload {
    #[serde(rename = "orderID")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobCancelOrderResponse {
    #[serde(rename = "canceled")]
    pub canceled: Vec<String>,
    #[serde(rename = "not_canceled")]
    pub not_canceled: std::collections::BTreeMap<String,String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobCancelOrdersParams {
    #[serde(rename = "body")]
    pub body: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobCancelOrdersResponse {
    #[serde(rename = "canceled")]
    pub canceled: Vec<String>,
    #[serde(rename = "not_canceled")]
    pub not_canceled: std::collections::BTreeMap<String,String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobClobMarketDetails {
    #[serde(rename = "gst", default, skip_serializing_if = "Option::is_none")]
    pub gst: Option<String>,
    #[serde(rename = "r", default, skip_serializing_if = "Option::is_none")]
    pub r: Option<Box<ClobClobRewards>>,
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<Vec<ClobClobToken>>,
    #[serde(rename = "mos", default, skip_serializing_if = "Option::is_none")]
    pub mos: Option<serde_json::Number>,
    #[serde(rename = "mts", default, skip_serializing_if = "Option::is_none")]
    pub mts: Option<serde_json::Number>,
    #[serde(rename = "mbf", default, skip_serializing_if = "Option::is_none")]
    pub mbf: Option<serde_json::Number>,
    #[serde(rename = "tbf", default, skip_serializing_if = "Option::is_none")]
    pub tbf: Option<serde_json::Number>,
    #[serde(rename = "rfqe", default, skip_serializing_if = "Option::is_none")]
    pub rfqe: Option<bool>,
    #[serde(rename = "itode", default, skip_serializing_if = "Option::is_none")]
    pub itode: Option<bool>,
    #[serde(rename = "ibce", default, skip_serializing_if = "Option::is_none")]
    pub ibce: Option<bool>,
    #[serde(rename = "fd", default, skip_serializing_if = "Option::is_none")]
    pub fd: Option<Box<ClobFeeDetails>>,
    #[serde(rename = "oas", default, skip_serializing_if = "Option::is_none")]
    pub oas: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobClobToken {
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
    #[serde(rename = "o", default, skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobClosedOnlyResponse {
    #[serde(rename = "closed_only")]
    pub closed_only: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobCreateApiKeyParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobCreateApiKeyResponse {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(rename = "passphrase")]
    pub passphrase: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobCreateBuilderApiKeyParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobCreateBuilderApiKeyResponse {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(rename = "passphrase")]
    pub passphrase: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobCurrentReward {
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "rewards_max_spread", default, skip_serializing_if = "Option::is_none")]
    pub rewards_max_spread: Option<serde_json::Number>,
    #[serde(rename = "rewards_min_size", default, skip_serializing_if = "Option::is_none")]
    pub rewards_min_size: Option<serde_json::Number>,
    #[serde(rename = "rewards_config", default, skip_serializing_if = "Option::is_none")]
    pub rewards_config: Option<Vec<ClobCurrentRewardConfig>>,
    #[serde(rename = "sponsored_daily_rate", default, skip_serializing_if = "Option::is_none")]
    pub sponsored_daily_rate: Option<serde_json::Number>,
    #[serde(rename = "sponsors_count", default, skip_serializing_if = "Option::is_none")]
    pub sponsors_count: Option<serde_json::Number>,
    #[serde(rename = "native_daily_rate", default, skip_serializing_if = "Option::is_none")]
    pub native_daily_rate: Option<serde_json::Number>,
    #[serde(rename = "total_daily_rate", default, skip_serializing_if = "Option::is_none")]
    pub total_daily_rate: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobCurrentRewardConfig {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Number>,
    #[serde(rename = "asset_address")]
    pub asset_address: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "end_date", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "rate_per_day")]
    pub rate_per_day: serde_json::Number,
    #[serde(rename = "total_rewards", default, skip_serializing_if = "Option::is_none")]
    pub total_rewards: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobCurrentRewardConfig {
    fn default() -> Self {
        Self {
            id: Default::default(),
            asset_address: Default::default(),
            start_date: Default::default(),
            end_date: Default::default(),
            rate_per_day: 0.into(),
            total_rewards: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobDeleteApiKeyParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobDeriveApiKeyParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobDeriveApiKeyResponse {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(rename = "passphrase")]
    pub passphrase: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobDropNotificationsParams {
    #[serde(rename = "ids")]
    pub ids: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobErrorResponse {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "retry_after_seconds", default, skip_serializing_if = "Option::is_none")]
    pub retry_after_seconds: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobFeeDetails {
    #[serde(rename = "r", default, skip_serializing_if = "Option::is_none")]
    pub r: Option<serde_json::Number>,
    #[serde(rename = "e", default, skip_serializing_if = "Option::is_none")]
    pub e: Option<serde_json::Number>,
    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobFeeRate {
    #[serde(rename = "base_fee")]
    pub base_fee: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobFeeRate {
    fn default() -> Self {
        Self {
            base_fee: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetApiKeysParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetApiKeysResponse {
    #[serde(rename = "apiKeys")]
    pub api_keys: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetBalanceAllowanceParams {
    #[serde(rename = "asset_type")]
    pub asset_type: String,
    #[serde(rename = "token_id", default, skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(rename = "signature_type", default, skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetBalanceAllowanceResponse {
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "allowances")]
    pub allowances: std::collections::BTreeMap<String,String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetBatchPricesHistoryParams {
    #[serde(rename = "body")]
    pub body: Box<ClobBatchPricesHistoryRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetBatchPricesHistoryResponse {
    #[serde(rename = "history", default, skip_serializing_if = "Option::is_none")]
    pub history: Option<std::collections::BTreeMap<String,Vec<ClobMarketPrice>>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetBookParams {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetBookResponse {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "asset_id")]
    pub asset_id: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "bids")]
    pub bids: Vec<ClobOrderSummary>,
    #[serde(rename = "asks")]
    pub asks: Vec<ClobOrderSummary>,
    #[serde(rename = "min_order_size")]
    pub min_order_size: String,
    #[serde(rename = "tick_size")]
    pub tick_size: String,
    #[serde(rename = "neg_risk")]
    pub neg_risk: bool,
    #[serde(rename = "last_trade_price")]
    pub last_trade_price: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetBooksGetParams {
    #[serde(rename = "token_ids")]
    pub token_ids: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetBooksPostParams {
    #[serde(rename = "body")]
    pub body: Vec<ClobBookRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetBuilderApiKeysParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetBuilderTradesParams {
    #[serde(rename = "builder_code")]
    pub builder_code: String,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "asset_id", default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "before", default, skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(rename = "after", default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetBuilderTradesResponse {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "data")]
    pub data: Vec<ClobBuilderTrade>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetBuilderTradesResponse {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            next_cursor: Default::default(),
            count: 0.into(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetClobMarketInfoParams {
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetClobMarketInfoResponse {
    #[serde(rename = "gst", default, skip_serializing_if = "Option::is_none")]
    pub gst: Option<String>,
    #[serde(rename = "r", default, skip_serializing_if = "Option::is_none")]
    pub r: Option<Box<ClobClobRewards>>,
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<Vec<ClobClobToken>>,
    #[serde(rename = "mos", default, skip_serializing_if = "Option::is_none")]
    pub mos: Option<serde_json::Number>,
    #[serde(rename = "mts", default, skip_serializing_if = "Option::is_none")]
    pub mts: Option<serde_json::Number>,
    #[serde(rename = "mbf", default, skip_serializing_if = "Option::is_none")]
    pub mbf: Option<serde_json::Number>,
    #[serde(rename = "tbf", default, skip_serializing_if = "Option::is_none")]
    pub tbf: Option<serde_json::Number>,
    #[serde(rename = "rfqe", default, skip_serializing_if = "Option::is_none")]
    pub rfqe: Option<bool>,
    #[serde(rename = "itode", default, skip_serializing_if = "Option::is_none")]
    pub itode: Option<bool>,
    #[serde(rename = "ibce", default, skip_serializing_if = "Option::is_none")]
    pub ibce: Option<bool>,
    #[serde(rename = "fd", default, skip_serializing_if = "Option::is_none")]
    pub fd: Option<Box<ClobFeeDetails>>,
    #[serde(rename = "oas", default, skip_serializing_if = "Option::is_none")]
    pub oas: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetClosedOnlyModeParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetClosedOnlyModeResponse {
    #[serde(rename = "closed_only")]
    pub closed_only: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetCurrentRebatedFeesParams {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "maker_address")]
    pub maker_address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetCurrentRewardsParams {
    #[serde(rename = "sponsored", default, skip_serializing_if = "Option::is_none")]
    pub sponsored: Option<bool>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetCurrentRewardsResponse {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "data")]
    pub data: Vec<ClobCurrentReward>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetCurrentRewardsResponse {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            count: 0.into(),
            next_cursor: Default::default(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetEarningsForUserForDayParams {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "signature_type", default, skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<serde_json::Number>,
    #[serde(rename = "maker_address", default, skip_serializing_if = "Option::is_none")]
    pub maker_address: Option<String>,
    #[serde(rename = "sponsored", default, skip_serializing_if = "Option::is_none")]
    pub sponsored: Option<bool>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetEarningsForUserForDayResponse {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "data")]
    pub data: Vec<ClobUserEarning>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetEarningsForUserForDayResponse {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            count: 0.into(),
            next_cursor: Default::default(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetFeeRateByPathParams {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetFeeRateByPathResponse {
    #[serde(rename = "base_fee")]
    pub base_fee: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetFeeRateByPathResponse {
    fn default() -> Self {
        Self {
            base_fee: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetFeeRateParams {
    #[serde(rename = "token_id", default, skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetFeeRateResponse {
    #[serde(rename = "base_fee")]
    pub base_fee: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetFeeRateResponse {
    fn default() -> Self {
        Self {
            base_fee: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetLastTradePriceParams {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetLastTradePriceResponse {
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetLastTradesPricesGetParams {
    #[serde(rename = "token_ids")]
    pub token_ids: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetLastTradesPricesGetResponseItem {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetLastTradesPricesGetResponseValueItem {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetLastTradesPricesPostParams {
    #[serde(rename = "body")]
    pub body: Vec<ClobBookRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetLastTradesPricesPostResponseItem {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetLastTradesPricesPostResponseValueItem {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetMarketByTokenParams {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetMarketByTokenResponse {
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "primary_token_id")]
    pub primary_token_id: String,
    #[serde(rename = "secondary_token_id")]
    pub secondary_token_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetMarketLiveActivityParams {
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetMarketLiveActivityResponse {
    #[serde(rename = "condition_id", default, skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Number>,
    #[serde(rename = "question", default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(rename = "market_slug", default, skip_serializing_if = "Option::is_none")]
    pub market_slug: Option<String>,
    #[serde(rename = "event_slug", default, skip_serializing_if = "Option::is_none")]
    pub event_slug: Option<String>,
    #[serde(rename = "series_slug", default, skip_serializing_if = "Option::is_none")]
    pub series_slug: Option<String>,
    #[serde(rename = "icon", default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetMarketsLiveActivityParams {
    #[serde(rename = "body")]
    pub body: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetMidpointParams {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetMidpointResponse {
    #[serde(rename = "mid_price")]
    pub mid_price: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetMidpointsGetParams {
    #[serde(rename = "token_ids")]
    pub token_ids: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetMidpointsPostParams {
    #[serde(rename = "body")]
    pub body: Vec<ClobBookRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetMultiMarketsParams {
    #[serde(rename = "q", default, skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(rename = "tag_slug", default, skip_serializing_if = "Option::is_none")]
    pub tag_slug: Option<String>,
    #[serde(rename = "event_id", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "event_title", default, skip_serializing_if = "Option::is_none")]
    pub event_title: Option<String>,
    #[serde(rename = "order_by", default, skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(rename = "position", default, skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "min_volume_24hr", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_24hr: Option<serde_json::Number>,
    #[serde(rename = "max_volume_24hr", default, skip_serializing_if = "Option::is_none")]
    pub max_volume_24hr: Option<serde_json::Number>,
    #[serde(rename = "min_spread", default, skip_serializing_if = "Option::is_none")]
    pub min_spread: Option<serde_json::Number>,
    #[serde(rename = "max_spread", default, skip_serializing_if = "Option::is_none")]
    pub max_spread: Option<serde_json::Number>,
    #[serde(rename = "min_price", default, skip_serializing_if = "Option::is_none")]
    pub min_price: Option<serde_json::Number>,
    #[serde(rename = "max_price", default, skip_serializing_if = "Option::is_none")]
    pub max_price: Option<serde_json::Number>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetMultiMarketsResponse {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "data")]
    pub data: Vec<ClobMultiMarketInfo>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetMultiMarketsResponse {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            count: 0.into(),
            next_cursor: Default::default(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetNegRiskByPathParams {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetNegRiskByPathResponse {
    #[serde(rename = "neg_risk")]
    pub neg_risk: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetNegRiskParams {
    #[serde(rename = "token_id", default, skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetNegRiskResponse {
    #[serde(rename = "neg_risk")]
    pub neg_risk: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetNotificationsParams {
    #[serde(rename = "signature_type")]
    pub signature_type: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetNotificationsParams {
    fn default() -> Self {
        Self {
            signature_type: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetOrderParams {
    #[serde(rename = "orderID")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetOrderResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "maker_address")]
    pub maker_address: String,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "asset_id")]
    pub asset_id: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "original_size")]
    pub original_size: String,
    #[serde(rename = "size_matched")]
    pub size_matched: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "outcome")]
    pub outcome: String,
    #[serde(rename = "expiration")]
    pub expiration: String,
    #[serde(rename = "order_type")]
    pub order_type: String,
    #[serde(rename = "associate_trades", default, skip_serializing_if = "Option::is_none")]
    pub associate_trades: Option<Vec<String>>,
    #[serde(rename = "created_at")]
    pub created_at: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetOrderResponse {
    fn default() -> Self {
        Self {
            id: Default::default(),
            status: Default::default(),
            owner: Default::default(),
            maker_address: Default::default(),
            market: Default::default(),
            asset_id: Default::default(),
            side: Default::default(),
            original_size: Default::default(),
            size_matched: Default::default(),
            price: Default::default(),
            outcome: Default::default(),
            expiration: Default::default(),
            order_type: Default::default(),
            associate_trades: Default::default(),
            created_at: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetOrderScoringParams {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetOrderScoringResponse {
    #[serde(rename = "scoring")]
    pub scoring: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetOrdersParams {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "asset_id", default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetOrdersResponse {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "data")]
    pub data: Vec<ClobOpenOrder>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetOrdersResponse {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            next_cursor: Default::default(),
            count: 0.into(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetOrdersScoringParams {
    #[serde(rename = "order_ids")]
    pub order_ids: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetPriceParams {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetPriceResponse {
    #[serde(rename = "price")]
    pub price: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetPriceResponse {
    fn default() -> Self {
        Self {
            price: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetPricesGetParams {
    #[serde(rename = "token_ids")]
    pub token_ids: String,
    #[serde(rename = "sides")]
    pub sides: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetPricesHistoryParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "startTs", default, skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<serde_json::Number>,
    #[serde(rename = "endTs", default, skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<serde_json::Number>,
    #[serde(rename = "interval", default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "fidelity", default, skip_serializing_if = "Option::is_none")]
    pub fidelity: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetPricesHistoryResponse {
    #[serde(rename = "history", default, skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<ClobMarketPrice>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetPricesPostParams {
    #[serde(rename = "body")]
    pub body: Vec<ClobBookRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetRateLimitTierParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetRateLimitTierResponse {
    #[serde(rename = "tier")]
    pub tier: String,
    #[serde(rename = "usdc_volume", default, skip_serializing_if = "Option::is_none")]
    pub usdc_volume: Option<String>,
    #[serde(rename = "next_tier_threshold", default, skip_serializing_if = "Option::is_none")]
    pub next_tier_threshold: Option<serde_json::Number>,
    #[serde(rename = "period")]
    pub period: serde_json::Number,
    #[serde(rename = "tier_details")]
    pub tier_details: Box<ClobRateLimitTierDetails>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetRateLimitTierResponse {
    fn default() -> Self {
        Self {
            tier: Default::default(),
            usdc_volume: Default::default(),
            next_tier_threshold: Default::default(),
            period: 0.into(),
            tier_details: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetRawRewardsForMarketParams {
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "sponsored", default, skip_serializing_if = "Option::is_none")]
    pub sponsored: Option<bool>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetRawRewardsForMarketResponse {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "data")]
    pub data: Vec<ClobMarketReward>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetRawRewardsForMarketResponse {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            count: 0.into(),
            next_cursor: Default::default(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetRewardPercentagesForUserParams {
    #[serde(rename = "signature_type", default, skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<serde_json::Number>,
    #[serde(rename = "maker_address", default, skip_serializing_if = "Option::is_none")]
    pub maker_address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetSamplingMarketsParams {
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetSamplingMarketsResponse {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<serde_json::Number>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ClobMarket>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetSamplingSimplifiedMarketsParams {
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetSamplingSimplifiedMarketsResponse {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<serde_json::Number>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ClobSimplifiedMarket>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetSimplifiedMarketsParams {
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetSimplifiedMarketsResponse {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<serde_json::Number>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ClobSimplifiedMarket>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetSpreadParams {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetSpreadResponse {
    #[serde(rename = "spread")]
    pub spread: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetSpreadsParams {
    #[serde(rename = "body")]
    pub body: Vec<ClobBookRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetTickSizeByPathParams {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetTickSizeByPathResponse {
    #[serde(rename = "minimum_tick_size")]
    pub minimum_tick_size: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetTickSizeByPathResponse {
    fn default() -> Self {
        Self {
            minimum_tick_size: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetTickSizeParams {
    #[serde(rename = "token_id", default, skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetTickSizeResponse {
    #[serde(rename = "minimum_tick_size")]
    pub minimum_tick_size: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetTickSizeResponse {
    fn default() -> Self {
        Self {
            minimum_tick_size: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetTimeParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetTotalEarningsForUserForDayParams {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "signature_type", default, skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<serde_json::Number>,
    #[serde(rename = "maker_address", default, skip_serializing_if = "Option::is_none")]
    pub maker_address: Option<String>,
    #[serde(rename = "sponsored", default, skip_serializing_if = "Option::is_none")]
    pub sponsored: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetTradesParams {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "maker_address")]
    pub maker_address: String,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "asset_id", default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "before", default, skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(rename = "after", default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetTradesResponse {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "data")]
    pub data: Vec<ClobTrade>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetTradesResponse {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            next_cursor: Default::default(),
            count: 0.into(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetUpdateBalanceAllowanceParams {
    #[serde(rename = "asset_type")]
    pub asset_type: String,
    #[serde(rename = "token_id", default, skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(rename = "signature_type", default, skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetUpdateBalanceAllowanceResponse {
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "allowances")]
    pub allowances: std::collections::BTreeMap<String,String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobGetUserEarningsAndMarketsConfigParams {
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "signature_type", default, skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<serde_json::Number>,
    #[serde(rename = "maker_address", default, skip_serializing_if = "Option::is_none")]
    pub maker_address: Option<String>,
    #[serde(rename = "sponsored", default, skip_serializing_if = "Option::is_none")]
    pub sponsored: Option<bool>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "q", default, skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(rename = "tag_slug", default, skip_serializing_if = "Option::is_none")]
    pub tag_slug: Option<String>,
    #[serde(rename = "favorite_markets", default, skip_serializing_if = "Option::is_none")]
    pub favorite_markets: Option<bool>,
    #[serde(rename = "no_competition", default, skip_serializing_if = "Option::is_none")]
    pub no_competition: Option<bool>,
    #[serde(rename = "only_mergeable", default, skip_serializing_if = "Option::is_none")]
    pub only_mergeable: Option<bool>,
    #[serde(rename = "only_open_orders", default, skip_serializing_if = "Option::is_none")]
    pub only_open_orders: Option<bool>,
    #[serde(rename = "only_open_positions", default, skip_serializing_if = "Option::is_none")]
    pub only_open_positions: Option<bool>,
    #[serde(rename = "order_by", default, skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(rename = "position", default, skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobGetUserEarningsAndMarketsConfigResponse {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "total_count", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<serde_json::Number>,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "data")]
    pub data: Vec<ClobUserRewardsMarket>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobGetUserEarningsAndMarketsConfigResponse {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            count: 0.into(),
            total_count: Default::default(),
            next_cursor: Default::default(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobHeartbeatErrorResponse {
    #[serde(rename = "error_msg")]
    pub error_msg: String,
    #[serde(rename = "heartbeat_id")]
    pub heartbeat_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobHeartbeatRequest {
    #[serde(rename = "heartbeat_id")]
    pub heartbeat_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobHeartbeatResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobHeartbeatV1Response {
    #[serde(rename = "heartbeat_id")]
    pub heartbeat_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobListSessionSignersParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobLiveActivityMarket {
    #[serde(rename = "condition_id", default, skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Number>,
    #[serde(rename = "question", default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(rename = "market_slug", default, skip_serializing_if = "Option::is_none")]
    pub market_slug: Option<String>,
    #[serde(rename = "event_slug", default, skip_serializing_if = "Option::is_none")]
    pub event_slug: Option<String>,
    #[serde(rename = "series_slug", default, skip_serializing_if = "Option::is_none")]
    pub series_slug: Option<String>,
    #[serde(rename = "icon", default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobMarket {
    #[serde(rename = "enable_order_book", default, skip_serializing_if = "Option::is_none")]
    pub enable_order_book: Option<bool>,
    #[serde(rename = "active", default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "archived", default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "accepting_orders", default, skip_serializing_if = "Option::is_none")]
    pub accepting_orders: Option<bool>,
    #[serde(rename = "accepting_order_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub accepting_order_timestamp: Option<String>,
    #[serde(rename = "minimum_order_size", default, skip_serializing_if = "Option::is_none")]
    pub minimum_order_size: Option<serde_json::Number>,
    #[serde(rename = "minimum_tick_size", default, skip_serializing_if = "Option::is_none")]
    pub minimum_tick_size: Option<serde_json::Number>,
    #[serde(rename = "condition_id", default, skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    #[serde(rename = "question_id", default, skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    #[serde(rename = "question", default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "market_slug", default, skip_serializing_if = "Option::is_none")]
    pub market_slug: Option<String>,
    #[serde(rename = "end_date_iso", default, skip_serializing_if = "Option::is_none")]
    pub end_date_iso: Option<String>,
    #[serde(rename = "game_start_time", default, skip_serializing_if = "Option::is_none")]
    pub game_start_time: Option<String>,
    #[serde(rename = "seconds_delay", default, skip_serializing_if = "Option::is_none")]
    pub seconds_delay: Option<serde_json::Number>,
    #[serde(rename = "fpmm", default, skip_serializing_if = "Option::is_none")]
    pub fpmm: Option<String>,
    #[serde(rename = "maker_base_fee", default, skip_serializing_if = "Option::is_none")]
    pub maker_base_fee: Option<serde_json::Number>,
    #[serde(rename = "taker_base_fee", default, skip_serializing_if = "Option::is_none")]
    pub taker_base_fee: Option<serde_json::Number>,
    #[serde(rename = "notifications_enabled", default, skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
    #[serde(rename = "neg_risk", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk: Option<bool>,
    #[serde(rename = "neg_risk_market_id", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk_market_id: Option<String>,
    #[serde(rename = "neg_risk_request_id", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk_request_id: Option<String>,
    #[serde(rename = "icon", default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "rewards", default, skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Box<ClobRewards>>,
    #[serde(rename = "is_50_50_outcome", default, skip_serializing_if = "Option::is_none")]
    pub is_50_50_outcome: Option<bool>,
    #[serde(rename = "tokens", default, skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<ClobToken>>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobMarketByTokenResponse {
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "primary_token_id")]
    pub primary_token_id: String,
    #[serde(rename = "secondary_token_id")]
    pub secondary_token_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobMarketPrice {
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<serde_json::Number>,
    #[serde(rename = "p", default, skip_serializing_if = "Option::is_none")]
    pub p: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobMarketReward {
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "question")]
    pub question: String,
    #[serde(rename = "market_slug", default, skip_serializing_if = "Option::is_none")]
    pub market_slug: Option<String>,
    #[serde(rename = "event_slug", default, skip_serializing_if = "Option::is_none")]
    pub event_slug: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "rewards_max_spread", default, skip_serializing_if = "Option::is_none")]
    pub rewards_max_spread: Option<serde_json::Number>,
    #[serde(rename = "rewards_min_size", default, skip_serializing_if = "Option::is_none")]
    pub rewards_min_size: Option<serde_json::Number>,
    #[serde(rename = "market_competitiveness", default, skip_serializing_if = "Option::is_none")]
    pub market_competitiveness: Option<serde_json::Number>,
    #[serde(rename = "tokens")]
    pub tokens: Vec<ClobRewardsToken>,
    #[serde(rename = "rewards_config", default, skip_serializing_if = "Option::is_none")]
    pub rewards_config: Option<Vec<ClobRewardsConfig>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobMultiMarketInfo {
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "event_id", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "event_slug", default, skip_serializing_if = "Option::is_none")]
    pub event_slug: Option<String>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "group_item_title", default, skip_serializing_if = "Option::is_none")]
    pub group_item_title: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "market_competitiveness", default, skip_serializing_if = "Option::is_none")]
    pub market_competitiveness: Option<serde_json::Number>,
    #[serde(rename = "market_id")]
    pub market_id: String,
    #[serde(rename = "market_slug", default, skip_serializing_if = "Option::is_none")]
    pub market_slug: Option<String>,
    #[serde(rename = "one_day_price_change", default, skip_serializing_if = "Option::is_none")]
    pub one_day_price_change: Option<serde_json::Number>,
    #[serde(rename = "question")]
    pub question: String,
    #[serde(rename = "rewards_max_spread", default, skip_serializing_if = "Option::is_none")]
    pub rewards_max_spread: Option<serde_json::Number>,
    #[serde(rename = "rewards_min_size", default, skip_serializing_if = "Option::is_none")]
    pub rewards_min_size: Option<serde_json::Number>,
    #[serde(rename = "spread", default, skip_serializing_if = "Option::is_none")]
    pub spread: Option<serde_json::Number>,
    #[serde(rename = "end_date", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "tokens")]
    pub tokens: Vec<ClobRewardsToken>,
    #[serde(rename = "volume_24hr", default, skip_serializing_if = "Option::is_none")]
    pub volume_24hr: Option<serde_json::Number>,
    #[serde(rename = "rewards_config", default, skip_serializing_if = "Option::is_none")]
    pub rewards_config: Option<Vec<ClobRewardsConfig>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobNegRisk {
    #[serde(rename = "neg_risk")]
    pub neg_risk: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobNotification {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Number,
    #[serde(rename = "payload")]
    pub payload: serde_json::Map<String,serde_json::Value>,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobNotification {
    fn default() -> Self {
        Self {
            id: 0.into(),
            owner: Default::default(),
            type_: 0.into(),
            payload: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobOpenOrder {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "maker_address")]
    pub maker_address: String,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "asset_id")]
    pub asset_id: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "original_size")]
    pub original_size: String,
    #[serde(rename = "size_matched")]
    pub size_matched: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "outcome")]
    pub outcome: String,
    #[serde(rename = "expiration")]
    pub expiration: String,
    #[serde(rename = "order_type")]
    pub order_type: String,
    #[serde(rename = "associate_trades", default, skip_serializing_if = "Option::is_none")]
    pub associate_trades: Option<Vec<String>>,
    #[serde(rename = "created_at")]
    pub created_at: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobOpenOrder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            status: Default::default(),
            owner: Default::default(),
            maker_address: Default::default(),
            market: Default::default(),
            asset_id: Default::default(),
            side: Default::default(),
            original_size: Default::default(),
            size_matched: Default::default(),
            price: Default::default(),
            outcome: Default::default(),
            expiration: Default::default(),
            order_type: Default::default(),
            associate_trades: Default::default(),
            created_at: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobOrder {
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "signer")]
    pub signer: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "makerAmount")]
    pub maker_amount: String,
    #[serde(rename = "takerAmount")]
    pub taker_amount: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "expiration")]
    pub expiration: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(rename = "builder")]
    pub builder: String,
    #[serde(rename = "signature", default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "salt")]
    pub salt: serde_json::Number,
    #[serde(rename = "signatureType")]
    pub signature_type: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobOrder {
    fn default() -> Self {
        Self {
            maker: Default::default(),
            signer: Default::default(),
            token_id: Default::default(),
            maker_amount: Default::default(),
            taker_amount: Default::default(),
            side: Default::default(),
            expiration: Default::default(),
            timestamp: Default::default(),
            metadata: Default::default(),
            builder: Default::default(),
            signature: Default::default(),
            salt: 0.into(),
            signature_type: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobOrderBookSummary {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "asset_id")]
    pub asset_id: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "bids")]
    pub bids: Vec<ClobOrderSummary>,
    #[serde(rename = "asks")]
    pub asks: Vec<ClobOrderSummary>,
    #[serde(rename = "min_order_size")]
    pub min_order_size: String,
    #[serde(rename = "tick_size")]
    pub tick_size: String,
    #[serde(rename = "neg_risk")]
    pub neg_risk: bool,
    #[serde(rename = "last_trade_price")]
    pub last_trade_price: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobOrderMarketCancelParams {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "asset_id")]
    pub asset_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobOrderScoringResponse {
    #[serde(rename = "scoring")]
    pub scoring: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobOrderSummary {
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobOrdersResponse {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "data")]
    pub data: Vec<ClobOpenOrder>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobOrdersResponse {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            next_cursor: Default::default(),
            count: 0.into(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobPaginatedCurrentReward {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "data")]
    pub data: Vec<ClobCurrentReward>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobPaginatedCurrentReward {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            count: 0.into(),
            next_cursor: Default::default(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobPaginatedMarketReward {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "data")]
    pub data: Vec<ClobMarketReward>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobPaginatedMarketReward {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            count: 0.into(),
            next_cursor: Default::default(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobPaginatedMarkets {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<serde_json::Number>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ClobMarket>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobPaginatedMultiMarketInfo {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "data")]
    pub data: Vec<ClobMultiMarketInfo>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobPaginatedMultiMarketInfo {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            count: 0.into(),
            next_cursor: Default::default(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobPaginatedSimplifiedMarkets {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<serde_json::Number>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ClobSimplifiedMarket>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobPaginatedUserEarnings {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "data")]
    pub data: Vec<ClobUserEarning>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobPaginatedUserEarnings {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            count: 0.into(),
            next_cursor: Default::default(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobPaginatedUserRewardsMarkets {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "total_count", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<serde_json::Number>,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "data")]
    pub data: Vec<ClobUserRewardsMarket>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobPaginatedUserRewardsMarkets {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            count: 0.into(),
            total_count: Default::default(),
            next_cursor: Default::default(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobPostOrderParams {
    #[serde(rename = "body")]
    pub body: Box<ClobSendOrder>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobPostOrderResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "orderID")]
    pub order_id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "makingAmount", default, skip_serializing_if = "Option::is_none")]
    pub making_amount: Option<String>,
    #[serde(rename = "takingAmount", default, skip_serializing_if = "Option::is_none")]
    pub taking_amount: Option<String>,
    #[serde(rename = "transactionsHashes", default, skip_serializing_if = "Option::is_none")]
    pub transactions_hashes: Option<Vec<String>>,
    #[serde(rename = "tradeIDs", default, skip_serializing_if = "Option::is_none")]
    pub trade_i_ds: Option<Vec<String>>,
    #[serde(rename = "errorMsg", default, skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobPostOrdersParams {
    #[serde(rename = "body")]
    pub body: Vec<ClobSendOrder>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobPostOrdersScoringParams {
    #[serde(rename = "body")]
    pub body: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobPricesHistoryResponse {
    #[serde(rename = "history", default, skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<ClobMarketPrice>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobRateLimitTierDetails {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "volume_threshold", default, skip_serializing_if = "Option::is_none")]
    pub volume_threshold: Option<serde_json::Number>,
    #[serde(rename = "cancel_rate_per_sec", default, skip_serializing_if = "Option::is_none")]
    pub cancel_rate_per_sec: Option<serde_json::Number>,
    #[serde(rename = "order_rate_per_sec", default, skip_serializing_if = "Option::is_none")]
    pub order_rate_per_sec: Option<serde_json::Number>,
    #[serde(rename = "order_burst", default, skip_serializing_if = "Option::is_none")]
    pub order_burst: Option<serde_json::Number>,
    #[serde(rename = "cancel_burst", default, skip_serializing_if = "Option::is_none")]
    pub cancel_burst: Option<serde_json::Number>,
    #[serde(rename = "allow_negative", default, skip_serializing_if = "Option::is_none")]
    pub allow_negative: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobRateLimitTierResponse {
    #[serde(rename = "tier")]
    pub tier: String,
    #[serde(rename = "usdc_volume", default, skip_serializing_if = "Option::is_none")]
    pub usdc_volume: Option<String>,
    #[serde(rename = "next_tier_threshold", default, skip_serializing_if = "Option::is_none")]
    pub next_tier_threshold: Option<serde_json::Number>,
    #[serde(rename = "period")]
    pub period: serde_json::Number,
    #[serde(rename = "tier_details")]
    pub tier_details: Box<ClobRateLimitTierDetails>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobRateLimitTierResponse {
    fn default() -> Self {
        Self {
            tier: Default::default(),
            usdc_volume: Default::default(),
            next_tier_threshold: Default::default(),
            period: 0.into(),
            tier_details: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobRebatedFees {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "asset_address")]
    pub asset_address: String,
    #[serde(rename = "maker_address")]
    pub maker_address: String,
    #[serde(rename = "rebated_fees_usdc")]
    pub rebated_fees_usdc: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobRevokeBuilderApiKeyParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobRewards {
    #[serde(rename = "rates", default, skip_serializing_if = "Option::is_none")]
    pub rates: Option<Vec<ClobRewardsRatesItem>>,
    #[serde(rename = "min_size", default, skip_serializing_if = "Option::is_none")]
    pub min_size: Option<serde_json::Number>,
    #[serde(rename = "max_spread", default, skip_serializing_if = "Option::is_none")]
    pub max_spread: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobRewardsConfig {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Number>,
    #[serde(rename = "asset_address")]
    pub asset_address: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "end_date", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "rate_per_day")]
    pub rate_per_day: serde_json::Number,
    #[serde(rename = "total_rewards", default, skip_serializing_if = "Option::is_none")]
    pub total_rewards: Option<serde_json::Number>,
    #[serde(rename = "remaining_reward_amount", default, skip_serializing_if = "Option::is_none")]
    pub remaining_reward_amount: Option<serde_json::Number>,
    #[serde(rename = "total_days", default, skip_serializing_if = "Option::is_none")]
    pub total_days: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobRewardsConfig {
    fn default() -> Self {
        Self {
            id: Default::default(),
            asset_address: Default::default(),
            start_date: Default::default(),
            end_date: Default::default(),
            rate_per_day: 0.into(),
            total_rewards: Default::default(),
            remaining_reward_amount: Default::default(),
            total_days: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobRewardsRatesItem {
    #[serde(rename = "asset_address", default, skip_serializing_if = "Option::is_none")]
    pub asset_address: Option<String>,
    #[serde(rename = "rewards_daily_rate", default, skip_serializing_if = "Option::is_none")]
    pub rewards_daily_rate: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobRewardsToken {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(rename = "outcome")]
    pub outcome: String,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobSendHeartbeatParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobSendHeartbeatResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobSendHeartbeatV1Params {
    #[serde(rename = "body")]
    pub body: Box<ClobHeartbeatRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobSendHeartbeatV1Response {
    #[serde(rename = "heartbeat_id")]
    pub heartbeat_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobSendOrder {
    #[serde(rename = "order")]
    pub order: Box<ClobOrder>,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(rename = "deferExec", default, skip_serializing_if = "Option::is_none")]
    pub defer_exec: Option<bool>,
    #[serde(rename = "postOnly", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobSendOrderResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "orderID")]
    pub order_id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "makingAmount", default, skip_serializing_if = "Option::is_none")]
    pub making_amount: Option<String>,
    #[serde(rename = "takingAmount", default, skip_serializing_if = "Option::is_none")]
    pub taking_amount: Option<String>,
    #[serde(rename = "transactionsHashes", default, skip_serializing_if = "Option::is_none")]
    pub transactions_hashes: Option<Vec<String>>,
    #[serde(rename = "tradeIDs", default, skip_serializing_if = "Option::is_none")]
    pub trade_i_ds: Option<Vec<String>>,
    #[serde(rename = "errorMsg", default, skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobSimplifiedMarket {
    #[serde(rename = "condition_id", default, skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    #[serde(rename = "rewards", default, skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Box<ClobRewards>>,
    #[serde(rename = "tokens", default, skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<ClobToken>>,
    #[serde(rename = "active", default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "archived", default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "accepting_orders", default, skip_serializing_if = "Option::is_none")]
    pub accepting_orders: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobTickSize {
    #[serde(rename = "minimum_tick_size")]
    pub minimum_tick_size: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobTickSize {
    fn default() -> Self {
        Self {
            minimum_tick_size: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobToken {
    #[serde(rename = "token_id", default, skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(rename = "outcome", default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Number>,
    #[serde(rename = "winner", default, skip_serializing_if = "Option::is_none")]
    pub winner: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobTotalUserEarning {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "asset_address")]
    pub asset_address: String,
    #[serde(rename = "maker_address")]
    pub maker_address: String,
    #[serde(rename = "earnings")]
    pub earnings: serde_json::Number,
    #[serde(rename = "native_earnings", default, skip_serializing_if = "Option::is_none")]
    pub native_earnings: Option<serde_json::Number>,
    #[serde(rename = "sponsored_earnings", default, skip_serializing_if = "Option::is_none")]
    pub sponsored_earnings: Option<serde_json::Number>,
    #[serde(rename = "asset_rate")]
    pub asset_rate: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobTotalUserEarning {
    fn default() -> Self {
        Self {
            date: Default::default(),
            asset_address: Default::default(),
            maker_address: Default::default(),
            earnings: 0.into(),
            native_earnings: Default::default(),
            sponsored_earnings: Default::default(),
            asset_rate: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobTrade {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "taker_order_id")]
    pub taker_order_id: String,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "asset_id")]
    pub asset_id: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "fee_rate_bps", default, skip_serializing_if = "Option::is_none")]
    pub fee_rate_bps: Option<String>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "match_time")]
    pub match_time: String,
    #[serde(rename = "match_time_nano", default, skip_serializing_if = "Option::is_none")]
    pub match_time_nano: Option<String>,
    #[serde(rename = "last_update")]
    pub last_update: String,
    #[serde(rename = "outcome")]
    pub outcome: String,
    #[serde(rename = "bucket_index")]
    pub bucket_index: serde_json::Number,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "maker_address")]
    pub maker_address: String,
    #[serde(rename = "transaction_hash", default, skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "err_msg", default, skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    #[serde(rename = "maker_orders", default, skip_serializing_if = "Option::is_none")]
    pub maker_orders: Option<Vec<ClobTradeMakerOrdersItem>>,
    #[serde(rename = "trader_side")]
    pub trader_side: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobTrade {
    fn default() -> Self {
        Self {
            id: Default::default(),
            taker_order_id: Default::default(),
            market: Default::default(),
            asset_id: Default::default(),
            side: Default::default(),
            size: Default::default(),
            fee_rate_bps: Default::default(),
            price: Default::default(),
            status: Default::default(),
            match_time: Default::default(),
            match_time_nano: Default::default(),
            last_update: Default::default(),
            outcome: Default::default(),
            bucket_index: 0.into(),
            owner: Default::default(),
            maker_address: Default::default(),
            transaction_hash: Default::default(),
            err_msg: Default::default(),
            maker_orders: Default::default(),
            trader_side: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobTradeMakerOrdersItem {
    #[serde(rename = "order_id", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "maker_address", default, skip_serializing_if = "Option::is_none")]
    pub maker_address: Option<String>,
    #[serde(rename = "matched_amount", default, skip_serializing_if = "Option::is_none")]
    pub matched_amount: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "fee_rate_bps", default, skip_serializing_if = "Option::is_none")]
    pub fee_rate_bps: Option<String>,
    #[serde(rename = "asset_id", default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "outcome", default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobTradesResponse {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    #[serde(rename = "count")]
    pub count: serde_json::Number,
    #[serde(rename = "data")]
    pub data: Vec<ClobTrade>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobTradesResponse {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            next_cursor: Default::default(),
            count: 0.into(),
            data: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobUpdateBalanceAllowanceParams {
    #[serde(rename = "asset_type")]
    pub asset_type: String,
    #[serde(rename = "token_id", default, skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(rename = "signature_type", default, skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobUpdateBalanceAllowanceResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClobUserEarning {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "asset_address")]
    pub asset_address: String,
    #[serde(rename = "maker_address")]
    pub maker_address: String,
    #[serde(rename = "earnings")]
    pub earnings: serde_json::Number,
    #[serde(rename = "asset_rate", default, skip_serializing_if = "Option::is_none")]
    pub asset_rate: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClobUserEarning {
    fn default() -> Self {
        Self {
            date: Default::default(),
            condition_id: Default::default(),
            asset_address: Default::default(),
            maker_address: Default::default(),
            earnings: 0.into(),
            asset_rate: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClobUserRewardsMarket {
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "market_id")]
    pub market_id: String,
    #[serde(rename = "event_id", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "question")]
    pub question: String,
    #[serde(rename = "market_slug", default, skip_serializing_if = "Option::is_none")]
    pub market_slug: Option<String>,
    #[serde(rename = "event_slug", default, skip_serializing_if = "Option::is_none")]
    pub event_slug: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "rewards_max_spread", default, skip_serializing_if = "Option::is_none")]
    pub rewards_max_spread: Option<serde_json::Number>,
    #[serde(rename = "rewards_min_size", default, skip_serializing_if = "Option::is_none")]
    pub rewards_min_size: Option<serde_json::Number>,
    #[serde(rename = "volume_24hr", default, skip_serializing_if = "Option::is_none")]
    pub volume_24hr: Option<serde_json::Number>,
    #[serde(rename = "spread", default, skip_serializing_if = "Option::is_none")]
    pub spread: Option<serde_json::Number>,
    #[serde(rename = "market_competitiveness", default, skip_serializing_if = "Option::is_none")]
    pub market_competitiveness: Option<serde_json::Number>,
    #[serde(rename = "tokens")]
    pub tokens: Vec<ClobRewardsToken>,
    #[serde(rename = "rewards_config", default, skip_serializing_if = "Option::is_none")]
    pub rewards_config: Option<Vec<ClobCurrentRewardConfig>>,
    #[serde(rename = "maker_address", default, skip_serializing_if = "Option::is_none")]
    pub maker_address: Option<String>,
    #[serde(rename = "earning_percentage", default, skip_serializing_if = "Option::is_none")]
    pub earning_percentage: Option<serde_json::Number>,
    #[serde(rename = "earnings", default, skip_serializing_if = "Option::is_none")]
    pub earnings: Option<Vec<ClobAssetEarning>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CollateralReturnPlanParams {
    #[serde(rename = "body")]
    pub body: CollateralReturnPlanParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CollateralReturnPlanParamsBody {
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CollateralReturnSubmitParams {
    #[serde(rename = "body")]
    pub body: CollateralReturnSubmitParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CollateralReturnSubmitParamsBody {
    #[serde(rename = "plan_hash")]
    pub plan_hash: String,
    #[serde(rename = "envelope")]
    pub envelope: serde_json::Map<String,serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqCancelMakerQuoteParams {
    #[serde(rename = "body")]
    pub body: Box<CombosRfqCancelQuoteRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqCancelMakerQuoteResponse {
    #[serde(rename = "request")]
    pub request: Box<CombosRfqRFQRequest>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "competition_started_at", default, skip_serializing_if = "Option::is_none")]
    pub competition_started_at: Option<serde_json::Number>,
    #[serde(rename = "competition_ends_at", default, skip_serializing_if = "Option::is_none")]
    pub competition_ends_at: Option<serde_json::Number>,
    #[serde(rename = "confirmation_started_at", default, skip_serializing_if = "Option::is_none")]
    pub confirmation_started_at: Option<serde_json::Number>,
    #[serde(rename = "confirmation_ends_at", default, skip_serializing_if = "Option::is_none")]
    pub confirmation_ends_at: Option<serde_json::Number>,
    #[serde(rename = "quote_id", default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(rename = "bundle", default, skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Box<CombosRfqFillBundle>>,
    #[serde(rename = "maker_confirmations", default, skip_serializing_if = "Option::is_none")]
    pub maker_confirmations: Option<Vec<CombosRfqMakerConfirmationSnapshot>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CombosRfqCancelQuoteRequest {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "signer_address")]
    pub signer_address: String,
    #[serde(rename = "maker_address")]
    pub maker_address: String,
    #[serde(rename = "signature_type")]
    pub signature_type: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CombosRfqCancelQuoteRequest {
    fn default() -> Self {
        Self {
            rfq_id: Default::default(),
            quote_id: Default::default(),
            signer_address: Default::default(),
            maker_address: Default::default(),
            signature_type: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CombosRfqComboMarket {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "position_ids")]
    pub position_ids: Vec<String>,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "outcomes")]
    pub outcomes: Vec<String>,
    #[serde(rename = "outcome_prices")]
    pub outcome_prices: Vec<String>,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "volume")]
    pub volume: serde_json::Number,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CombosRfqComboMarket {
    fn default() -> Self {
        Self {
            id: Default::default(),
            condition_id: Default::default(),
            position_ids: Default::default(),
            slug: Default::default(),
            title: Default::default(),
            outcomes: Default::default(),
            outcome_prices: Default::default(),
            image: Default::default(),
            volume: 0.into(),
            tags: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqComboMarketsResponse {
    #[serde(rename = "markets")]
    pub markets: Vec<CombosRfqComboMarket>,
    #[serde(rename = "next_cursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqErrorResponse {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CombosRfqExchangeV3Order {
    #[serde(rename = "salt")]
    pub salt: String,
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "signer")]
    pub signer: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "makerAmount")]
    pub maker_amount: String,
    #[serde(rename = "takerAmount")]
    pub taker_amount: String,
    #[serde(rename = "side")]
    pub side: serde_json::Number,
    #[serde(rename = "signatureType")]
    pub signature_type: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(rename = "builder", default, skip_serializing_if = "Option::is_none")]
    pub builder: Option<String>,
    #[serde(rename = "signature", default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CombosRfqExchangeV3Order {
    fn default() -> Self {
        Self {
            salt: Default::default(),
            maker: Default::default(),
            signer: Default::default(),
            token_id: Default::default(),
            maker_amount: Default::default(),
            taker_amount: Default::default(),
            side: 0.into(),
            signature_type: 0.into(),
            timestamp: Default::default(),
            metadata: Default::default(),
            builder: Default::default(),
            signature: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqExecutionHandoff {
    #[serde(rename = "execution_id")]
    pub execution_id: String,
    #[serde(rename = "request")]
    pub request: Box<CombosRfqRFQRequest>,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "bundle")]
    pub bundle: Box<CombosRfqFillBundle>,
    #[serde(rename = "requester_acceptance")]
    pub requester_acceptance: Box<CombosRfqRequesterAcceptance>,
    #[serde(rename = "maker_quotes")]
    pub maker_quotes: Vec<CombosRfqQuote>,
    #[serde(rename = "reservations", default, skip_serializing_if = "Option::is_none")]
    pub reservations: Option<Vec<CombosRfqWalletReservation>>,
    #[serde(rename = "ready_at", default, skip_serializing_if = "Option::is_none")]
    pub ready_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqFillAllocation {
    #[serde(rename = "maker_quote_id")]
    pub maker_quote_id: String,
    #[serde(rename = "signer_address")]
    pub signer_address: String,
    #[serde(rename = "maker_address")]
    pub maker_address: String,
    #[serde(rename = "size_e6")]
    pub size_e6: String,
    #[serde(rename = "price_e6")]
    pub price_e6: String,
    #[serde(rename = "received_at", default, skip_serializing_if = "Option::is_none")]
    pub received_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqFillBundle {
    #[serde(rename = "requested_shares_e6")]
    pub requested_shares_e6: String,
    #[serde(rename = "requested_notional_e6", default, skip_serializing_if = "Option::is_none")]
    pub requested_notional_e6: Option<String>,
    #[serde(rename = "blended_price_e6")]
    pub blended_price_e6: String,
    #[serde(rename = "allocations")]
    pub allocations: Vec<CombosRfqFillAllocation>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqGetComboMarketsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "exclude", default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqGetComboMarketsResponse {
    #[serde(rename = "markets")]
    pub markets: Vec<CombosRfqComboMarket>,
    #[serde(rename = "next_cursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CombosRfqMakerConfirmationRequest {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "signer_address")]
    pub signer_address: String,
    #[serde(rename = "maker_address")]
    pub maker_address: String,
    #[serde(rename = "signature_type")]
    pub signature_type: serde_json::Number,
    #[serde(rename = "decision")]
    pub decision: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CombosRfqMakerConfirmationRequest {
    fn default() -> Self {
        Self {
            rfq_id: Default::default(),
            quote_id: Default::default(),
            signer_address: Default::default(),
            maker_address: Default::default(),
            signature_type: 0.into(),
            decision: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqMakerConfirmationResult {
    #[serde(rename = "snapshot", default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Box<CombosRfqRFQSnapshot>>,
    #[serde(rename = "execution", default, skip_serializing_if = "Option::is_none")]
    pub execution: Option<Box<CombosRfqExecutionHandoff>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqMakerConfirmationSnapshot {
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "signer_address")]
    pub signer_address: String,
    #[serde(rename = "maker_address")]
    pub maker_address: String,
    #[serde(rename = "decision", default, skip_serializing_if = "Option::is_none")]
    pub decision: Option<String>,
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "responded_at", default, skip_serializing_if = "Option::is_none")]
    pub responded_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CombosRfqQuote {
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "auth_address", default, skip_serializing_if = "Option::is_none")]
    pub auth_address: Option<String>,
    #[serde(rename = "signer_address")]
    pub signer_address: String,
    #[serde(rename = "maker_address")]
    pub maker_address: String,
    #[serde(rename = "signature_type")]
    pub signature_type: serde_json::Number,
    #[serde(rename = "price_e6")]
    pub price_e6: String,
    #[serde(rename = "size_e6")]
    pub size_e6: String,
    #[serde(rename = "valid_until", default, skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<serde_json::Number>,
    #[serde(rename = "signed_order")]
    pub signed_order: Box<CombosRfqExchangeV3Order>,
    #[serde(rename = "received_at", default, skip_serializing_if = "Option::is_none")]
    pub received_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CombosRfqQuote {
    fn default() -> Self {
        Self {
            quote_id: Default::default(),
            rfq_id: Default::default(),
            auth_address: Default::default(),
            signer_address: Default::default(),
            maker_address: Default::default(),
            signature_type: 0.into(),
            price_e6: Default::default(),
            size_e6: Default::default(),
            valid_until: Default::default(),
            signed_order: Default::default(),
            received_at: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqRFQRequest {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "auth_address", default, skip_serializing_if = "Option::is_none")]
    pub auth_address: Option<String>,
    #[serde(rename = "signer_address", default, skip_serializing_if = "Option::is_none")]
    pub signer_address: Option<String>,
    #[serde(rename = "maker_address", default, skip_serializing_if = "Option::is_none")]
    pub maker_address: Option<String>,
    #[serde(rename = "signature_type", default, skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<serde_json::Number>,
    #[serde(rename = "requestor_public_id", default, skip_serializing_if = "Option::is_none")]
    pub requestor_public_id: Option<String>,
    #[serde(rename = "leg_position_ids")]
    pub leg_position_ids: Vec<String>,
    #[serde(rename = "condition_id", default, skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    #[serde(rename = "yes_position_id", default, skip_serializing_if = "Option::is_none")]
    pub yes_position_id: Option<String>,
    #[serde(rename = "no_position_id", default, skip_serializing_if = "Option::is_none")]
    pub no_position_id: Option<String>,
    #[serde(rename = "direction")]
    pub direction: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "requested_size", default, skip_serializing_if = "Option::is_none")]
    pub requested_size: Option<Box<CombosRfqRequestedSize>>,
    #[serde(rename = "created_at", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqRFQSnapshot {
    #[serde(rename = "request")]
    pub request: Box<CombosRfqRFQRequest>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "competition_started_at", default, skip_serializing_if = "Option::is_none")]
    pub competition_started_at: Option<serde_json::Number>,
    #[serde(rename = "competition_ends_at", default, skip_serializing_if = "Option::is_none")]
    pub competition_ends_at: Option<serde_json::Number>,
    #[serde(rename = "confirmation_started_at", default, skip_serializing_if = "Option::is_none")]
    pub confirmation_started_at: Option<serde_json::Number>,
    #[serde(rename = "confirmation_ends_at", default, skip_serializing_if = "Option::is_none")]
    pub confirmation_ends_at: Option<serde_json::Number>,
    #[serde(rename = "quote_id", default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(rename = "bundle", default, skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Box<CombosRfqFillBundle>>,
    #[serde(rename = "maker_confirmations", default, skip_serializing_if = "Option::is_none")]
    pub maker_confirmations: Option<Vec<CombosRfqMakerConfirmationSnapshot>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqRequestedSize {
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "value_e6")]
    pub value_e6: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqRequesterAcceptance {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "auth_address", default, skip_serializing_if = "Option::is_none")]
    pub auth_address: Option<String>,
    #[serde(rename = "signer_address", default, skip_serializing_if = "Option::is_none")]
    pub signer_address: Option<String>,
    #[serde(rename = "maker_address", default, skip_serializing_if = "Option::is_none")]
    pub maker_address: Option<String>,
    #[serde(rename = "signature_type", default, skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<serde_json::Number>,
    #[serde(rename = "signed_order")]
    pub signed_order: Box<CombosRfqExchangeV3Order>,
    #[serde(rename = "accepted_at", default, skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqSubmitMakerConfirmationParams {
    #[serde(rename = "body")]
    pub body: Box<CombosRfqMakerConfirmationRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqSubmitMakerConfirmationResponse {
    #[serde(rename = "snapshot", default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Box<CombosRfqRFQSnapshot>>,
    #[serde(rename = "execution", default, skip_serializing_if = "Option::is_none")]
    pub execution: Option<Box<CombosRfqExecutionHandoff>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqSubmitMakerQuoteParams {
    #[serde(rename = "body")]
    pub body: Box<CombosRfqQuote>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqSubmitMakerQuoteResponse {
    #[serde(rename = "request")]
    pub request: Box<CombosRfqRFQRequest>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "competition_started_at", default, skip_serializing_if = "Option::is_none")]
    pub competition_started_at: Option<serde_json::Number>,
    #[serde(rename = "competition_ends_at", default, skip_serializing_if = "Option::is_none")]
    pub competition_ends_at: Option<serde_json::Number>,
    #[serde(rename = "confirmation_started_at", default, skip_serializing_if = "Option::is_none")]
    pub confirmation_started_at: Option<serde_json::Number>,
    #[serde(rename = "confirmation_ends_at", default, skip_serializing_if = "Option::is_none")]
    pub confirmation_ends_at: Option<serde_json::Number>,
    #[serde(rename = "quote_id", default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(rename = "bundle", default, skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Box<CombosRfqFillBundle>>,
    #[serde(rename = "maker_confirmations", default, skip_serializing_if = "Option::is_none")]
    pub maker_confirmations: Option<Vec<CombosRfqMakerConfirmationSnapshot>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CombosRfqWalletAssetDelta {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "asset_id")]
    pub asset_id: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CombosRfqWalletReservation {
    #[serde(rename = "action_id")]
    pub action_id: String,
    #[serde(rename = "user")]
    pub user: String,
    #[serde(rename = "wallet_nonce")]
    pub wallet_nonce: serde_json::Number,
    #[serde(rename = "deltas")]
    pub deltas: Vec<CombosRfqWalletAssetDelta>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CombosRfqWalletReservation {
    fn default() -> Self {
        Self {
            action_id: Default::default(),
            user: Default::default(),
            wallet_nonce: 0.into(),
            deltas: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataActivity {
    #[serde(rename = "bio")]
    pub bio: String,
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "event_slug")]
    pub event_slug: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "is_combo", default, skip_serializing_if = "Option::is_none")]
    pub is_combo: Option<bool>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "outcome")]
    pub outcome: String,
    #[serde(rename = "outcome_index")]
    pub outcome_index: serde_json::Number,
    #[serde(rename = "price")]
    pub price: serde_json::Number,
    #[serde(rename = "profile_image")]
    pub profile_image: String,
    #[serde(rename = "profile_image_optimized")]
    pub profile_image_optimized: String,
    #[serde(rename = "proxy_wallet")]
    pub proxy_wallet: String,
    #[serde(rename = "pseudonym")]
    pub pseudonym: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "size")]
    pub size: serde_json::Number,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(rename = "transaction_hash")]
    pub transaction_hash: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "usdc_size")]
    pub usdc_size: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataActivity {
    fn default() -> Self {
        Self {
            bio: Default::default(),
            condition_id: Default::default(),
            event_slug: Default::default(),
            icon: Default::default(),
            is_combo: Default::default(),
            name: Default::default(),
            outcome: Default::default(),
            outcome_index: 0.into(),
            price: 0.into(),
            profile_image: Default::default(),
            profile_image_optimized: Default::default(),
            proxy_wallet: Default::default(),
            pseudonym: Default::default(),
            side: Default::default(),
            size: 0.into(),
            slug: Default::default(),
            timestamp: 0.into(),
            title: Default::default(),
            token_id: Default::default(),
            transaction_hash: Default::default(),
            type_: Default::default(),
            usdc_size: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataActivityPage {
    #[serde(rename = "data")]
    pub data: Vec<DataActivity>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataApprovalContract {
    #[serde(rename = "amount", default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "approved")]
    pub approved: bool,
    #[serde(rename = "feature")]
    pub feature: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "spender")]
    pub spender: String,
    #[serde(rename = "standard")]
    pub standard: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataApprovals {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "chain_id")]
    pub chain_id: serde_json::Number,
    #[serde(rename = "checked_at")]
    pub checked_at: String,
    #[serde(rename = "contracts")]
    pub contracts: Vec<DataApprovalContract>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataApprovals {
    fn default() -> Self {
        Self {
            address: Default::default(),
            chain_id: 0.into(),
            checked_at: Default::default(),
            contracts: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataBiggestWinner {
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "event_id")]
    pub event_id: serde_json::Number,
    #[serde(rename = "event_slug")]
    pub event_slug: String,
    #[serde(rename = "event_title")]
    pub event_title: String,
    #[serde(rename = "final_value")]
    pub final_value: serde_json::Number,
    #[serde(rename = "initial_value")]
    pub initial_value: serde_json::Number,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "pnl")]
    pub pnl: serde_json::Number,
    #[serde(rename = "position_id")]
    pub position_id: String,
    #[serde(rename = "profile_image")]
    pub profile_image: String,
    #[serde(rename = "resolved_at")]
    pub resolved_at: serde_json::Number,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "user_name")]
    pub user_name: String,
    #[serde(rename = "win_rank")]
    pub win_rank: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataBiggestWinner {
    fn default() -> Self {
        Self {
            condition_id: Default::default(),
            event_id: 0.into(),
            event_slug: Default::default(),
            event_title: Default::default(),
            final_value: 0.into(),
            initial_value: 0.into(),
            kind: Default::default(),
            pnl: 0.into(),
            position_id: Default::default(),
            profile_image: Default::default(),
            resolved_at: 0.into(),
            user_id: Default::default(),
            user_name: Default::default(),
            win_rank: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataBiggestWinnersPage {
    #[serde(rename = "data")]
    pub data: Vec<DataBiggestWinner>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataBuilderStanding {
    #[serde(rename = "active_users")]
    pub active_users: serde_json::Number,
    #[serde(rename = "builder_code")]
    pub builder_code: String,
    #[serde(rename = "builder_name")]
    pub builder_name: String,
    #[serde(rename = "profile_image")]
    pub profile_image: String,
    #[serde(rename = "rank")]
    pub rank: serde_json::Number,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "volume")]
    pub volume: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataBuilderStanding {
    fn default() -> Self {
        Self {
            active_users: 0.into(),
            builder_code: Default::default(),
            builder_name: Default::default(),
            profile_image: Default::default(),
            rank: 0.into(),
            verified: Default::default(),
            volume: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataBuilderVolumePoint {
    #[serde(rename = "active_users")]
    pub active_users: serde_json::Number,
    #[serde(rename = "builder_code")]
    pub builder_code: String,
    #[serde(rename = "builder_name")]
    pub builder_name: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "profile_image")]
    pub profile_image: String,
    #[serde(rename = "rank")]
    pub rank: serde_json::Number,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "volume")]
    pub volume: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataBuilderVolumePoint {
    fn default() -> Self {
        Self {
            active_users: 0.into(),
            builder_code: Default::default(),
            builder_name: Default::default(),
            date: Default::default(),
            profile_image: Default::default(),
            rank: 0.into(),
            verified: Default::default(),
            volume: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataBuildersLeaderboardPage {
    #[serde(rename = "data")]
    pub data: Vec<DataBuilderStanding>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataComboActivity {
    #[serde(rename = "amount_usdc", default, skip_serializing_if = "Option::is_none")]
    pub amount_usdc: Option<serde_json::Number>,
    #[serde(rename = "block_number")]
    pub block_number: serde_json::Number,
    #[serde(rename = "combo_condition_id")]
    pub combo_condition_id: String,
    #[serde(rename = "combo_position_id")]
    pub combo_position_id: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "legs")]
    pub legs: Vec<DataComboLeg>,
    #[serde(rename = "payout_usdc", default, skip_serializing_if = "Option::is_none")]
    pub payout_usdc: Option<serde_json::Number>,
    #[serde(rename = "proxy_wallet")]
    pub proxy_wallet: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "transaction_hash")]
    pub transaction_hash: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataComboActivity {
    fn default() -> Self {
        Self {
            amount_usdc: Default::default(),
            block_number: 0.into(),
            combo_condition_id: Default::default(),
            combo_position_id: Default::default(),
            id: Default::default(),
            legs: Default::default(),
            payout_usdc: Default::default(),
            proxy_wallet: Default::default(),
            timestamp: 0.into(),
            transaction_hash: Default::default(),
            type_: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataComboActivityPage {
    #[serde(rename = "data")]
    pub data: Vec<DataComboActivity>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataComboLeg {
    #[serde(rename = "leg_condition_id")]
    pub leg_condition_id: String,
    #[serde(rename = "leg_current_price")]
    pub leg_current_price: serde_json::Number,
    #[serde(rename = "leg_index")]
    pub leg_index: serde_json::Number,
    #[serde(rename = "leg_outcome_index")]
    pub leg_outcome_index: serde_json::Number,
    #[serde(rename = "leg_outcome_label")]
    pub leg_outcome_label: String,
    #[serde(rename = "leg_position_id")]
    pub leg_position_id: String,
    #[serde(rename = "leg_resolved_at", default, skip_serializing_if = "Option::is_none")]
    pub leg_resolved_at: Option<String>,
    #[serde(rename = "leg_status")]
    pub leg_status: String,
    #[serde(rename = "market")]
    pub market: Box<DataComboLegMarket>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataComboLeg {
    fn default() -> Self {
        Self {
            leg_condition_id: Default::default(),
            leg_current_price: 0.into(),
            leg_index: 0.into(),
            leg_outcome_index: 0.into(),
            leg_outcome_label: Default::default(),
            leg_position_id: Default::default(),
            leg_resolved_at: Default::default(),
            leg_status: Default::default(),
            market: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataComboLegEvent {
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "event_image")]
    pub event_image: String,
    #[serde(rename = "event_slug")]
    pub event_slug: String,
    #[serde(rename = "event_title")]
    pub event_title: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataComboLegMarket {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "end_date")]
    pub end_date: String,
    #[serde(rename = "event")]
    pub event: Box<DataComboLegEvent>,
    #[serde(rename = "group_item_title", default, skip_serializing_if = "Option::is_none")]
    pub group_item_title: Option<String>,
    #[serde(rename = "icon_url")]
    pub icon_url: String,
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "line", default, skip_serializing_if = "Option::is_none")]
    pub line: Option<serde_json::Number>,
    #[serde(rename = "market_id")]
    pub market_id: String,
    #[serde(rename = "outcome")]
    pub outcome: String,
    #[serde(rename = "outcomes", default, skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<Vec<String>>,
    #[serde(rename = "question", default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "sports_market_type", default, skip_serializing_if = "Option::is_none")]
    pub sports_market_type: Option<String>,
    #[serde(rename = "subcategory")]
    pub subcategory: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataComboPosition {
    #[serde(rename = "combo_condition_id")]
    pub combo_condition_id: String,
    #[serde(rename = "combo_position_id")]
    pub combo_position_id: String,
    #[serde(rename = "current_size")]
    pub current_size: serde_json::Number,
    #[serde(rename = "entry_avg_price_usdc")]
    pub entry_avg_price_usdc: serde_json::Number,
    #[serde(rename = "entry_cost_usdc")]
    pub entry_cost_usdc: serde_json::Number,
    #[serde(rename = "entry_fees_usdc")]
    pub entry_fees_usdc: serde_json::Number,
    #[serde(rename = "first_entry_at")]
    pub first_entry_at: String,
    #[serde(rename = "first_entry_at_micros", default, skip_serializing_if = "Option::is_none")]
    pub first_entry_at_micros: Option<serde_json::Number>,
    #[serde(rename = "gross_entry_cost_usdc")]
    pub gross_entry_cost_usdc: serde_json::Number,
    #[serde(rename = "legs")]
    pub legs: Vec<DataComboLeg>,
    #[serde(rename = "legs_pending")]
    pub legs_pending: serde_json::Number,
    #[serde(rename = "legs_resolved")]
    pub legs_resolved: serde_json::Number,
    #[serde(rename = "legs_total")]
    pub legs_total: serde_json::Number,
    #[serde(rename = "outcome_index")]
    pub outcome_index: serde_json::Number,
    #[serde(rename = "outcome_label")]
    pub outcome_label: String,
    #[serde(rename = "proxy_wallet")]
    pub proxy_wallet: String,
    #[serde(rename = "realized_payout_usdc")]
    pub realized_payout_usdc: serde_json::Number,
    #[serde(rename = "redeemable")]
    pub redeemable: bool,
    #[serde(rename = "resolved_at", default, skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "updated_at_micros")]
    pub updated_at_micros: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataComboPosition {
    fn default() -> Self {
        Self {
            combo_condition_id: Default::default(),
            combo_position_id: Default::default(),
            current_size: 0.into(),
            entry_avg_price_usdc: 0.into(),
            entry_cost_usdc: 0.into(),
            entry_fees_usdc: 0.into(),
            first_entry_at: Default::default(),
            first_entry_at_micros: Default::default(),
            gross_entry_cost_usdc: 0.into(),
            legs: Default::default(),
            legs_pending: 0.into(),
            legs_resolved: 0.into(),
            legs_total: 0.into(),
            outcome_index: 0.into(),
            outcome_label: Default::default(),
            proxy_wallet: Default::default(),
            realized_payout_usdc: 0.into(),
            redeemable: Default::default(),
            resolved_at: Default::default(),
            status: Default::default(),
            updated_at: Default::default(),
            updated_at_micros: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataComboPositionsPage {
    #[serde(rename = "data")]
    pub data: Vec<DataComboPosition>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataConditionVolume {
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "taker_volume")]
    pub taker_volume: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataConditionVolume {
    fn default() -> Self {
        Self {
            condition_id: Default::default(),
            taker_volume: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataCursorLag {
    #[serde(rename = "behind_max")]
    pub behind_max: serde_json::Number,
    #[serde(rename = "block")]
    pub block: serde_json::Number,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataCursorLag {
    fn default() -> Self {
        Self {
            behind_max: 0.into(),
            block: 0.into(),
            source: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataEnvelopeApprovals {
    #[serde(rename = "data")]
    pub data: DataEnvelopeApprovalsData,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataEnvelopeApprovalsData {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "chain_id")]
    pub chain_id: serde_json::Number,
    #[serde(rename = "checked_at")]
    pub checked_at: String,
    #[serde(rename = "contracts")]
    pub contracts: Vec<DataApprovalContract>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataEnvelopeApprovalsData {
    fn default() -> Self {
        Self {
            address: Default::default(),
            chain_id: 0.into(),
            checked_at: Default::default(),
            contracts: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataEnvelopeLiveVolume {
    #[serde(rename = "data")]
    pub data: DataEnvelopeLiveVolumeData,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataEnvelopeLiveVolumeData {
    #[serde(rename = "conditions")]
    pub conditions: Vec<DataConditionVolume>,
    #[serde(rename = "taker_volume_total")]
    pub taker_volume_total: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataEnvelopeLiveVolumeData {
    fn default() -> Self {
        Self {
            conditions: Default::default(),
            taker_volume_total: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataEnvelopeOptionLeaderboardUserEntry {
    #[serde(rename = "data")]
    pub data: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataEnvelopeOptionUserStats {
    #[serde(rename = "data")]
    pub data: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataEnvelopePortfolioValue {
    #[serde(rename = "data")]
    pub data: DataEnvelopePortfolioValueData,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataEnvelopePortfolioValueData {
    #[serde(rename = "proxy_wallet")]
    pub proxy_wallet: String,
    #[serde(rename = "value")]
    pub value: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataEnvelopePortfolioValueData {
    fn default() -> Self {
        Self {
            proxy_wallet: Default::default(),
            value: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataEnvelopeServiceStatus {
    #[serde(rename = "data")]
    pub data: DataEnvelopeServiceStatusData,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataEnvelopeServiceStatusData {
    #[serde(rename = "age_seconds")]
    pub age_seconds: serde_json::Number,
    #[serde(rename = "computed_at")]
    pub computed_at: String,
    #[serde(rename = "ingestion")]
    pub ingestion: Box<DataIngestionFreshness>,
    #[serde(rename = "serving")]
    pub serving: Box<DataServingFreshness>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataEnvelopeServiceStatusData {
    fn default() -> Self {
        Self {
            age_seconds: 0.into(),
            computed_at: Default::default(),
            ingestion: Default::default(),
            serving: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataEnvelopeUserPnlSeries {
    #[serde(rename = "data")]
    pub data: DataEnvelopeUserPnlSeriesData,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataEnvelopeUserPnlSeriesData {
    #[serde(rename = "fidelity")]
    pub fidelity: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "points")]
    pub points: Vec<DataUserPnlPoint>,
    #[serde(rename = "proxy_wallet")]
    pub proxy_wallet: String,
    #[serde(rename = "source_fidelity")]
    pub source_fidelity: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataEnvelopeUserVolume {
    #[serde(rename = "data")]
    pub data: DataEnvelopeUserVolumeData,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataEnvelopeUserVolumeData {
    #[serde(rename = "trade_count")]
    pub trade_count: serde_json::Number,
    #[serde(rename = "volume")]
    pub volume: serde_json::Number,
    #[serde(rename = "volume_usdc")]
    pub volume_usdc: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataEnvelopeUserVolumeData {
    fn default() -> Self {
        Self {
            trade_count: 0.into(),
            volume: 0.into(),
            volume_usdc: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataEnvelopeVecBuilderVolumePoint {
    #[serde(rename = "data")]
    pub data: Vec<DataEnvelopeVecBuilderVolumePointDataItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataEnvelopeVecBuilderVolumePointDataItem {
    #[serde(rename = "active_users")]
    pub active_users: serde_json::Number,
    #[serde(rename = "builder_code")]
    pub builder_code: String,
    #[serde(rename = "builder_name")]
    pub builder_name: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "profile_image")]
    pub profile_image: String,
    #[serde(rename = "rank")]
    pub rank: serde_json::Number,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "volume")]
    pub volume: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataEnvelopeVecBuilderVolumePointDataItem {
    fn default() -> Self {
        Self {
            active_users: 0.into(),
            builder_code: Default::default(),
            builder_name: Default::default(),
            date: Default::default(),
            profile_image: Default::default(),
            rank: 0.into(),
            verified: Default::default(),
            volume: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataEnvelopeVecOpenInterest {
    #[serde(rename = "data")]
    pub data: Vec<DataEnvelopeVecOpenInterestDataItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataEnvelopeVecOpenInterestDataItem {
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "value")]
    pub value: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataEnvelopeVecOpenInterestDataItem {
    fn default() -> Self {
        Self {
            condition_id: Default::default(),
            value: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataEnvelopeVecResolution {
    #[serde(rename = "data")]
    pub data: Vec<DataEnvelopeVecResolutionDataItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataEnvelopeVecResolutionDataItem {
    #[serde(rename = "condition_id", default, skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    #[serde(rename = "expected_settlement_time", default, skip_serializing_if = "Option::is_none")]
    pub expected_settlement_time: Option<String>,
    #[serde(rename = "extended_review")]
    pub extended_review: bool,
    #[serde(rename = "last_update_timestamp")]
    pub last_update_timestamp: String,
    #[serde(rename = "log_index")]
    pub log_index: String,
    #[serde(rename = "market_type", default, skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
    #[serde(rename = "new_version_q")]
    pub new_version_q: bool,
    #[serde(rename = "payouts", default, skip_serializing_if = "Option::is_none")]
    pub payouts: Option<Vec<serde_json::Number>>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "proposed_price", default, skip_serializing_if = "Option::is_none")]
    pub proposed_price: Option<String>,
    #[serde(rename = "question_id", default, skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    #[serde(rename = "reporter", default, skip_serializing_if = "Option::is_none")]
    pub reporter: Option<String>,
    #[serde(rename = "reproposed_price", default, skip_serializing_if = "Option::is_none")]
    pub reproposed_price: Option<String>,
    #[serde(rename = "resolution_source", default, skip_serializing_if = "Option::is_none")]
    pub resolution_source: Option<String>,
    #[serde(rename = "resolved_at", default, skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    #[serde(rename = "resolved_block", default, skip_serializing_if = "Option::is_none")]
    pub resolved_block: Option<serde_json::Number>,
    #[serde(rename = "settlement_time_basis", default, skip_serializing_if = "Option::is_none")]
    pub settlement_time_basis: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "transaction_hash")]
    pub transaction_hash: String,
    #[serde(rename = "was_arbitrated", default, skip_serializing_if = "Option::is_none")]
    pub was_arbitrated: Option<bool>,
    #[serde(rename = "was_disputed")]
    pub was_disputed: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataEnvelopeVecTokenRef {
    #[serde(rename = "data")]
    pub data: Vec<DataEnvelopeVecTokenRefDataItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataEnvelopeVecTokenRefDataItem {
    #[serde(rename = "clob_index", default, skip_serializing_if = "Option::is_none")]
    pub clob_index: Option<serde_json::Number>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "event_id", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<serde_json::Number>,
    #[serde(rename = "event_slug", default, skip_serializing_if = "Option::is_none")]
    pub event_slug: Option<String>,
    #[serde(rename = "final_price", default, skip_serializing_if = "Option::is_none")]
    pub final_price: Option<serde_json::Number>,
    #[serde(rename = "market_slug", default, skip_serializing_if = "Option::is_none")]
    pub market_slug: Option<String>,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "neg_risk", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk: Option<bool>,
    #[serde(rename = "neg_risk_market_id", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk_market_id: Option<String>,
    #[serde(rename = "opposite_token_id", default, skip_serializing_if = "Option::is_none")]
    pub opposite_token_id: Option<String>,
    #[serde(rename = "outcome", default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[serde(rename = "outcome_index")]
    pub outcome_index: serde_json::Number,
    #[serde(rename = "question_index", default, skip_serializing_if = "Option::is_none")]
    pub question_index: Option<serde_json::Number>,
    #[serde(rename = "resolved")]
    pub resolved: bool,
    #[serde(rename = "structural_condition_id", default, skip_serializing_if = "Option::is_none")]
    pub structural_condition_id: Option<String>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataEnvelopeVecTokenRefDataItem {
    fn default() -> Self {
        Self {
            clob_index: Default::default(),
            closed: Default::default(),
            condition_id: Default::default(),
            event_id: Default::default(),
            event_slug: Default::default(),
            final_price: Default::default(),
            market_slug: Default::default(),
            module: Default::default(),
            neg_risk: Default::default(),
            neg_risk_market_id: Default::default(),
            opposite_token_id: Default::default(),
            outcome: Default::default(),
            outcome_index: 0.into(),
            question_index: Default::default(),
            resolved: Default::default(),
            structural_condition_id: Default::default(),
            title: Default::default(),
            token_id: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataErrorResponse {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "parameter", default, skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    #[serde(rename = "retryable")]
    pub retryable: bool,
    #[serde(rename = "trace_id")]
    pub trace_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetActivityParams {
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "condition", default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "event_id", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "start", default, skip_serializing_if = "Option::is_none")]
    pub start: Option<serde_json::Number>,
    #[serde(rename = "end", default, skip_serializing_if = "Option::is_none")]
    pub end: Option<serde_json::Number>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sort_direction", default, skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
    #[serde(rename = "exclude_deposits_withdrawals", default, skip_serializing_if = "Option::is_none")]
    pub exclude_deposits_withdrawals: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetActivityResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataActivity>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetApprovalsParams {
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetApprovalsResponse {
    #[serde(rename = "data")]
    pub data: DataGetApprovalsResponseData,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataGetApprovalsResponseData {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "chain_id")]
    pub chain_id: serde_json::Number,
    #[serde(rename = "checked_at")]
    pub checked_at: String,
    #[serde(rename = "contracts")]
    pub contracts: Vec<DataApprovalContract>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataGetApprovalsResponseData {
    fn default() -> Self {
        Self {
            address: Default::default(),
            chain_id: 0.into(),
            checked_at: Default::default(),
            contracts: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetBiggestWinnersParams {
    #[serde(rename = "time_period", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<String>,
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetBiggestWinnersResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataBiggestWinner>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetBuildersLeaderboardParams {
    #[serde(rename = "time_period", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetBuildersLeaderboardResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataBuilderStanding>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetBuildersVolumeParams {
    #[serde(rename = "interval", default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetBuildersVolumeResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataGetBuildersVolumeResponseDataItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataGetBuildersVolumeResponseDataItem {
    #[serde(rename = "active_users")]
    pub active_users: serde_json::Number,
    #[serde(rename = "builder_code")]
    pub builder_code: String,
    #[serde(rename = "builder_name")]
    pub builder_name: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "profile_image")]
    pub profile_image: String,
    #[serde(rename = "rank")]
    pub rank: serde_json::Number,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "volume")]
    pub volume: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataGetBuildersVolumeResponseDataItem {
    fn default() -> Self {
        Self {
            active_users: 0.into(),
            builder_code: Default::default(),
            builder_name: Default::default(),
            date: Default::default(),
            profile_image: Default::default(),
            rank: 0.into(),
            verified: Default::default(),
            volume: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetComboPositionsParams {
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "condition", default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sort_direction", default, skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
    #[serde(rename = "updated_after", default, skip_serializing_if = "Option::is_none")]
    pub updated_after: Option<serde_json::Number>,
    #[serde(rename = "updated_before", default, skip_serializing_if = "Option::is_none")]
    pub updated_before: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetComboPositionsResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataComboPosition>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetCombosParams {
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "condition", default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetCombosResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataComboActivity>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetHoldersParams {
    #[serde(rename = "condition", default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "min_balance", default, skip_serializing_if = "Option::is_none")]
    pub min_balance: Option<serde_json::Number>,
    #[serde(rename = "include_pnl", default, skip_serializing_if = "Option::is_none")]
    pub include_pnl: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetHoldersResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataMetaHolder>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetLeaderboardParams {
    #[serde(rename = "time_period", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<String>,
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetLiveVolumeParams {
    #[serde(rename = "event_id", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetLiveVolumeResponse {
    #[serde(rename = "data")]
    pub data: DataGetLiveVolumeResponseData,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataGetLiveVolumeResponseData {
    #[serde(rename = "conditions")]
    pub conditions: Vec<DataConditionVolume>,
    #[serde(rename = "taker_volume_total")]
    pub taker_volume_total: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataGetLiveVolumeResponseData {
    fn default() -> Self {
        Self {
            conditions: Default::default(),
            taker_volume_total: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetOiParams {
    #[serde(rename = "condition", default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetOiResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataGetOiResponseDataItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataGetOiResponseDataItem {
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "value")]
    pub value: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataGetOiResponseDataItem {
    fn default() -> Self {
        Self {
            condition_id: Default::default(),
            value: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetPositionsParams {
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "condition", default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "event_id", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "filter_type", default, skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    #[serde(rename = "filter_amount", default, skip_serializing_if = "Option::is_none")]
    pub filter_amount: Option<serde_json::Number>,
    #[serde(rename = "include_archived", default, skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "start", default, skip_serializing_if = "Option::is_none")]
    pub start: Option<serde_json::Number>,
    #[serde(rename = "end", default, skip_serializing_if = "Option::is_none")]
    pub end: Option<serde_json::Number>,
    #[serde(rename = "sort_direction", default, skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetPositionsResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataPosition>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetPricesHistoryParams {
    #[serde(rename = "token_id", default, skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(rename = "start", default, skip_serializing_if = "Option::is_none")]
    pub start: Option<serde_json::Number>,
    #[serde(rename = "end", default, skip_serializing_if = "Option::is_none")]
    pub end: Option<serde_json::Number>,
    #[serde(rename = "interval", default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "bucket_seconds", default, skip_serializing_if = "Option::is_none")]
    pub bucket_seconds: Option<serde_json::Number>,
    #[serde(rename = "as_of", default, skip_serializing_if = "Option::is_none")]
    pub as_of: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetPricesHistoryResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataPricePoint>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetResolutionsParams {
    #[serde(rename = "question_id", default, skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    #[serde(rename = "condition", default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "event_id", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetResolutionsResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataGetResolutionsResponseDataItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetResolutionsResponseDataItem {
    #[serde(rename = "condition_id", default, skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    #[serde(rename = "expected_settlement_time", default, skip_serializing_if = "Option::is_none")]
    pub expected_settlement_time: Option<String>,
    #[serde(rename = "extended_review")]
    pub extended_review: bool,
    #[serde(rename = "last_update_timestamp")]
    pub last_update_timestamp: String,
    #[serde(rename = "log_index")]
    pub log_index: String,
    #[serde(rename = "market_type", default, skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
    #[serde(rename = "new_version_q")]
    pub new_version_q: bool,
    #[serde(rename = "payouts", default, skip_serializing_if = "Option::is_none")]
    pub payouts: Option<Vec<serde_json::Number>>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "proposed_price", default, skip_serializing_if = "Option::is_none")]
    pub proposed_price: Option<String>,
    #[serde(rename = "question_id", default, skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    #[serde(rename = "reporter", default, skip_serializing_if = "Option::is_none")]
    pub reporter: Option<String>,
    #[serde(rename = "reproposed_price", default, skip_serializing_if = "Option::is_none")]
    pub reproposed_price: Option<String>,
    #[serde(rename = "resolution_source", default, skip_serializing_if = "Option::is_none")]
    pub resolution_source: Option<String>,
    #[serde(rename = "resolved_at", default, skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    #[serde(rename = "resolved_block", default, skip_serializing_if = "Option::is_none")]
    pub resolved_block: Option<serde_json::Number>,
    #[serde(rename = "settlement_time_basis", default, skip_serializing_if = "Option::is_none")]
    pub settlement_time_basis: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "transaction_hash")]
    pub transaction_hash: String,
    #[serde(rename = "was_arbitrated", default, skip_serializing_if = "Option::is_none")]
    pub was_arbitrated: Option<bool>,
    #[serde(rename = "was_disputed")]
    pub was_disputed: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetStatusParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetStatusResponse {
    #[serde(rename = "data")]
    pub data: DataGetStatusResponseData,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataGetStatusResponseData {
    #[serde(rename = "age_seconds")]
    pub age_seconds: serde_json::Number,
    #[serde(rename = "computed_at")]
    pub computed_at: String,
    #[serde(rename = "ingestion")]
    pub ingestion: Box<DataIngestionFreshness>,
    #[serde(rename = "serving")]
    pub serving: Box<DataServingFreshness>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataGetStatusResponseData {
    fn default() -> Self {
        Self {
            age_seconds: 0.into(),
            computed_at: Default::default(),
            ingestion: Default::default(),
            serving: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetTokensParams {
    #[serde(rename = "token_id", default, skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(rename = "condition", default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetTokensResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataGetTokensResponseDataItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataGetTokensResponseDataItem {
    #[serde(rename = "clob_index", default, skip_serializing_if = "Option::is_none")]
    pub clob_index: Option<serde_json::Number>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "event_id", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<serde_json::Number>,
    #[serde(rename = "event_slug", default, skip_serializing_if = "Option::is_none")]
    pub event_slug: Option<String>,
    #[serde(rename = "final_price", default, skip_serializing_if = "Option::is_none")]
    pub final_price: Option<serde_json::Number>,
    #[serde(rename = "market_slug", default, skip_serializing_if = "Option::is_none")]
    pub market_slug: Option<String>,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "neg_risk", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk: Option<bool>,
    #[serde(rename = "neg_risk_market_id", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk_market_id: Option<String>,
    #[serde(rename = "opposite_token_id", default, skip_serializing_if = "Option::is_none")]
    pub opposite_token_id: Option<String>,
    #[serde(rename = "outcome", default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[serde(rename = "outcome_index")]
    pub outcome_index: serde_json::Number,
    #[serde(rename = "question_index", default, skip_serializing_if = "Option::is_none")]
    pub question_index: Option<serde_json::Number>,
    #[serde(rename = "resolved")]
    pub resolved: bool,
    #[serde(rename = "structural_condition_id", default, skip_serializing_if = "Option::is_none")]
    pub structural_condition_id: Option<String>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataGetTokensResponseDataItem {
    fn default() -> Self {
        Self {
            clob_index: Default::default(),
            closed: Default::default(),
            condition_id: Default::default(),
            event_id: Default::default(),
            event_slug: Default::default(),
            final_price: Default::default(),
            market_slug: Default::default(),
            module: Default::default(),
            neg_risk: Default::default(),
            neg_risk_market_id: Default::default(),
            opposite_token_id: Default::default(),
            outcome: Default::default(),
            outcome_index: 0.into(),
            question_index: Default::default(),
            resolved: Default::default(),
            structural_condition_id: Default::default(),
            title: Default::default(),
            token_id: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetTradesParams {
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "taker_only", default, skip_serializing_if = "Option::is_none")]
    pub taker_only: Option<bool>,
    #[serde(rename = "filter_type", default, skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    #[serde(rename = "filter_amount", default, skip_serializing_if = "Option::is_none")]
    pub filter_amount: Option<serde_json::Number>,
    #[serde(rename = "start", default, skip_serializing_if = "Option::is_none")]
    pub start: Option<serde_json::Number>,
    #[serde(rename = "end", default, skip_serializing_if = "Option::is_none")]
    pub end: Option<serde_json::Number>,
    #[serde(rename = "condition", default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "event_id", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetTradesResponse {
    #[serde(rename = "data")]
    pub data: Vec<DataTrade>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetUserPnlV2Params {
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "interval", default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "fidelity", default, skip_serializing_if = "Option::is_none")]
    pub fidelity: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetUserPnlV2Response {
    #[serde(rename = "data")]
    pub data: DataGetUserPnlV2ResponseData,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetUserPnlV2ResponseData {
    #[serde(rename = "fidelity")]
    pub fidelity: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "points")]
    pub points: Vec<DataUserPnlPoint>,
    #[serde(rename = "proxy_wallet")]
    pub proxy_wallet: String,
    #[serde(rename = "source_fidelity")]
    pub source_fidelity: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetUserStatsParams {
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetUserStatsResponse {
    #[serde(rename = "data")]
    pub data: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetUserVolumeParams {
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "start", default, skip_serializing_if = "Option::is_none")]
    pub start: Option<serde_json::Number>,
    #[serde(rename = "end", default, skip_serializing_if = "Option::is_none")]
    pub end: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetUserVolumeResponse {
    #[serde(rename = "data")]
    pub data: DataGetUserVolumeResponseData,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataGetUserVolumeResponseData {
    #[serde(rename = "trade_count")]
    pub trade_count: serde_json::Number,
    #[serde(rename = "volume")]
    pub volume: serde_json::Number,
    #[serde(rename = "volume_usdc")]
    pub volume_usdc: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataGetUserVolumeResponseData {
    fn default() -> Self {
        Self {
            trade_count: 0.into(),
            volume: 0.into(),
            volume_usdc: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetValueParams {
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "condition", default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataGetValueResponse {
    #[serde(rename = "data")]
    pub data: DataGetValueResponseData,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataGetValueResponseData {
    #[serde(rename = "proxy_wallet")]
    pub proxy_wallet: String,
    #[serde(rename = "value")]
    pub value: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataGetValueResponseData {
    fn default() -> Self {
        Self {
            proxy_wallet: Default::default(),
            value: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataHolder {
    #[serde(rename = "amount")]
    pub amount: serde_json::Number,
    #[serde(rename = "avg_price", default, skip_serializing_if = "Option::is_none")]
    pub avg_price: Option<serde_json::Number>,
    #[serde(rename = "bio")]
    pub bio: String,
    #[serde(rename = "current_price", default, skip_serializing_if = "Option::is_none")]
    pub current_price: Option<serde_json::Number>,
    #[serde(rename = "current_value", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<serde_json::Number>,
    #[serde(rename = "display_username_public")]
    pub display_username_public: bool,
    #[serde(rename = "entry_cost_usdc", default, skip_serializing_if = "Option::is_none")]
    pub entry_cost_usdc: Option<serde_json::Number>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "outcome_index")]
    pub outcome_index: serde_json::Number,
    #[serde(rename = "profile_image")]
    pub profile_image: String,
    #[serde(rename = "profile_image_optimized")]
    pub profile_image_optimized: String,
    #[serde(rename = "proxy_wallet")]
    pub proxy_wallet: String,
    #[serde(rename = "pseudonym")]
    pub pseudonym: String,
    #[serde(rename = "realized_pnl", default, skip_serializing_if = "Option::is_none")]
    pub realized_pnl: Option<serde_json::Number>,
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(rename = "total_pnl", default, skip_serializing_if = "Option::is_none")]
    pub total_pnl: Option<serde_json::Number>,
    #[serde(rename = "unrealized_pnl", default, skip_serializing_if = "Option::is_none")]
    pub unrealized_pnl: Option<serde_json::Number>,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataHolder {
    fn default() -> Self {
        Self {
            amount: 0.into(),
            avg_price: Default::default(),
            bio: Default::default(),
            current_price: Default::default(),
            current_value: Default::default(),
            display_username_public: Default::default(),
            entry_cost_usdc: Default::default(),
            name: Default::default(),
            outcome_index: 0.into(),
            profile_image: Default::default(),
            profile_image_optimized: Default::default(),
            proxy_wallet: Default::default(),
            pseudonym: Default::default(),
            realized_pnl: Default::default(),
            token_id: Default::default(),
            total_pnl: Default::default(),
            unrealized_pnl: Default::default(),
            verified: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataHoldersPage {
    #[serde(rename = "data")]
    pub data: Vec<DataMetaHolder>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataIngestionFreshness {
    #[serde(rename = "chain_id")]
    pub chain_id: serde_json::Number,
    #[serde(rename = "cursors")]
    pub cursors: serde_json::Number,
    #[serde(rename = "lagging")]
    pub lagging: Vec<DataCursorLag>,
    #[serde(rename = "max_synced_block", default, skip_serializing_if = "Option::is_none")]
    pub max_synced_block: Option<serde_json::Number>,
    #[serde(rename = "min_synced_block", default, skip_serializing_if = "Option::is_none")]
    pub min_synced_block: Option<serde_json::Number>,
    #[serde(rename = "most_lagged", default, skip_serializing_if = "Option::is_none")]
    pub most_lagged: Option<serde_json::Value>,
    #[serde(rename = "network", default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataIngestionFreshness {
    fn default() -> Self {
        Self {
            chain_id: 0.into(),
            cursors: 0.into(),
            lagging: Default::default(),
            max_synced_block: Default::default(),
            min_synced_block: Default::default(),
            most_lagged: Default::default(),
            network: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataLeaderboardEntry {
    #[serde(rename = "pnl")]
    pub pnl: serde_json::Number,
    #[serde(rename = "profile_image")]
    pub profile_image: String,
    #[serde(rename = "rank")]
    pub rank: serde_json::Number,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "user_name")]
    pub user_name: String,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "volume")]
    pub volume: serde_json::Number,
    #[serde(rename = "x_username")]
    pub x_username: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataLeaderboardEntry {
    fn default() -> Self {
        Self {
            pnl: 0.into(),
            profile_image: Default::default(),
            rank: 0.into(),
            user_id: Default::default(),
            user_name: Default::default(),
            verified: Default::default(),
            volume: 0.into(),
            x_username: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataLeaderboardPage {
    #[serde(rename = "data")]
    pub data: Vec<DataLeaderboardEntry>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataLeaderboardUserEntry {
    #[serde(rename = "pnl")]
    pub pnl: serde_json::Number,
    #[serde(rename = "profile_image")]
    pub profile_image: String,
    #[serde(rename = "rank_pnl", default, skip_serializing_if = "Option::is_none")]
    pub rank_pnl: Option<serde_json::Number>,
    #[serde(rename = "rank_volume", default, skip_serializing_if = "Option::is_none")]
    pub rank_volume: Option<serde_json::Number>,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "user_name")]
    pub user_name: String,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "volume")]
    pub volume: serde_json::Number,
    #[serde(rename = "x_username")]
    pub x_username: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataLeaderboardUserEntry {
    fn default() -> Self {
        Self {
            pnl: 0.into(),
            profile_image: Default::default(),
            rank_pnl: Default::default(),
            rank_volume: Default::default(),
            user_id: Default::default(),
            user_name: Default::default(),
            verified: Default::default(),
            volume: 0.into(),
            x_username: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataLiveVolume {
    #[serde(rename = "conditions")]
    pub conditions: Vec<DataConditionVolume>,
    #[serde(rename = "taker_volume_total")]
    pub taker_volume_total: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataLiveVolume {
    fn default() -> Self {
        Self {
            conditions: Default::default(),
            taker_volume_total: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataMetaHolder {
    #[serde(rename = "holders")]
    pub holders: Vec<DataHolder>,
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataOpenInterest {
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "value")]
    pub value: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataOpenInterest {
    fn default() -> Self {
        Self {
            condition_id: Default::default(),
            value: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataPagination {
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "offset")]
    pub offset: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataPagination {
    fn default() -> Self {
        Self {
            has_more: Default::default(),
            limit: 0.into(),
            next_cursor: Default::default(),
            offset: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataPortfolioValue {
    #[serde(rename = "proxy_wallet")]
    pub proxy_wallet: String,
    #[serde(rename = "value")]
    pub value: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataPortfolioValue {
    fn default() -> Self {
        Self {
            proxy_wallet: Default::default(),
            value: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataPosition {
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "avg_price")]
    pub avg_price: serde_json::Number,
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "current_price")]
    pub current_price: serde_json::Number,
    #[serde(rename = "current_size")]
    pub current_size: serde_json::Number,
    #[serde(rename = "current_value")]
    pub current_value: serde_json::Number,
    #[serde(rename = "end_date")]
    pub end_date: String,
    #[serde(rename = "entry_cost_usdc")]
    pub entry_cost_usdc: serde_json::Number,
    #[serde(rename = "entry_fees_usdc")]
    pub entry_fees_usdc: serde_json::Number,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "event_slug")]
    pub event_slug: String,
    #[serde(rename = "first_entry_at")]
    pub first_entry_at: serde_json::Number,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "last_event_at")]
    pub last_event_at: serde_json::Number,
    #[serde(rename = "mergeable")]
    pub mergeable: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "negative_risk")]
    pub negative_risk: bool,
    #[serde(rename = "opposite_outcome")]
    pub opposite_outcome: String,
    #[serde(rename = "opposite_token_id")]
    pub opposite_token_id: String,
    #[serde(rename = "outcome")]
    pub outcome: String,
    #[serde(rename = "outcome_index")]
    pub outcome_index: serde_json::Number,
    #[serde(rename = "percent_pnl")]
    pub percent_pnl: serde_json::Number,
    #[serde(rename = "percent_realized_pnl")]
    pub percent_realized_pnl: serde_json::Number,
    #[serde(rename = "profile_image")]
    pub profile_image: String,
    #[serde(rename = "proxy_wallet")]
    pub proxy_wallet: String,
    #[serde(rename = "realized_pnl")]
    pub realized_pnl: serde_json::Number,
    #[serde(rename = "redeemable")]
    pub redeemable: bool,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(rename = "total_cost_usdc")]
    pub total_cost_usdc: serde_json::Number,
    #[serde(rename = "total_pnl")]
    pub total_pnl: serde_json::Number,
    #[serde(rename = "total_size")]
    pub total_size: serde_json::Number,
    #[serde(rename = "unrealized_pnl")]
    pub unrealized_pnl: serde_json::Number,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataPosition {
    fn default() -> Self {
        Self {
            archived: Default::default(),
            avg_price: 0.into(),
            condition_id: Default::default(),
            current_price: 0.into(),
            current_size: 0.into(),
            current_value: 0.into(),
            end_date: Default::default(),
            entry_cost_usdc: 0.into(),
            entry_fees_usdc: 0.into(),
            event_id: Default::default(),
            event_slug: Default::default(),
            first_entry_at: 0.into(),
            icon: Default::default(),
            last_event_at: 0.into(),
            mergeable: Default::default(),
            name: Default::default(),
            negative_risk: Default::default(),
            opposite_outcome: Default::default(),
            opposite_token_id: Default::default(),
            outcome: Default::default(),
            outcome_index: 0.into(),
            percent_pnl: 0.into(),
            percent_realized_pnl: 0.into(),
            profile_image: Default::default(),
            proxy_wallet: Default::default(),
            realized_pnl: 0.into(),
            redeemable: Default::default(),
            slug: Default::default(),
            status: Default::default(),
            title: Default::default(),
            token_id: Default::default(),
            total_cost_usdc: 0.into(),
            total_pnl: 0.into(),
            total_size: 0.into(),
            unrealized_pnl: 0.into(),
            verified: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataPositionsPage {
    #[serde(rename = "data")]
    pub data: Vec<DataPosition>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataPricePoint {
    #[serde(rename = "price")]
    pub price: serde_json::Number,
    #[serde(rename = "resolution_seconds")]
    pub resolution_seconds: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataPricePoint {
    fn default() -> Self {
        Self {
            price: 0.into(),
            resolution_seconds: 0.into(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataPricesHistoryPage {
    #[serde(rename = "data")]
    pub data: Vec<DataPricePoint>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataResolution {
    #[serde(rename = "condition_id", default, skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    #[serde(rename = "expected_settlement_time", default, skip_serializing_if = "Option::is_none")]
    pub expected_settlement_time: Option<String>,
    #[serde(rename = "extended_review")]
    pub extended_review: bool,
    #[serde(rename = "last_update_timestamp")]
    pub last_update_timestamp: String,
    #[serde(rename = "log_index")]
    pub log_index: String,
    #[serde(rename = "market_type", default, skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
    #[serde(rename = "new_version_q")]
    pub new_version_q: bool,
    #[serde(rename = "payouts", default, skip_serializing_if = "Option::is_none")]
    pub payouts: Option<Vec<serde_json::Number>>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "proposed_price", default, skip_serializing_if = "Option::is_none")]
    pub proposed_price: Option<String>,
    #[serde(rename = "question_id", default, skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    #[serde(rename = "reporter", default, skip_serializing_if = "Option::is_none")]
    pub reporter: Option<String>,
    #[serde(rename = "reproposed_price", default, skip_serializing_if = "Option::is_none")]
    pub reproposed_price: Option<String>,
    #[serde(rename = "resolution_source", default, skip_serializing_if = "Option::is_none")]
    pub resolution_source: Option<String>,
    #[serde(rename = "resolved_at", default, skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    #[serde(rename = "resolved_block", default, skip_serializing_if = "Option::is_none")]
    pub resolved_block: Option<serde_json::Number>,
    #[serde(rename = "settlement_time_basis", default, skip_serializing_if = "Option::is_none")]
    pub settlement_time_basis: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "transaction_hash")]
    pub transaction_hash: String,
    #[serde(rename = "was_arbitrated", default, skip_serializing_if = "Option::is_none")]
    pub was_arbitrated: Option<bool>,
    #[serde(rename = "was_disputed")]
    pub was_disputed: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataServiceStatus {
    #[serde(rename = "age_seconds")]
    pub age_seconds: serde_json::Number,
    #[serde(rename = "computed_at")]
    pub computed_at: String,
    #[serde(rename = "ingestion")]
    pub ingestion: Box<DataIngestionFreshness>,
    #[serde(rename = "serving")]
    pub serving: Box<DataServingFreshness>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataServiceStatus {
    fn default() -> Self {
        Self {
            age_seconds: 0.into(),
            computed_at: Default::default(),
            ingestion: Default::default(),
            serving: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataServingFreshness {
    #[serde(rename = "lag_seconds", default, skip_serializing_if = "Option::is_none")]
    pub lag_seconds: Option<serde_json::Number>,
    #[serde(rename = "mechanisms")]
    pub mechanisms: Vec<DataServingMechanism>,
    #[serde(rename = "worst", default, skip_serializing_if = "Option::is_none")]
    pub worst: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataServingMechanism {
    #[serde(rename = "age_seconds")]
    pub age_seconds: serde_json::Number,
    #[serde(rename = "blocks_behind", default, skip_serializing_if = "Option::is_none")]
    pub blocks_behind: Option<serde_json::Number>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataServingMechanism {
    fn default() -> Self {
        Self {
            age_seconds: 0.into(),
            blocks_behind: Default::default(),
            name: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataTokenRef {
    #[serde(rename = "clob_index", default, skip_serializing_if = "Option::is_none")]
    pub clob_index: Option<serde_json::Number>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "event_id", default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<serde_json::Number>,
    #[serde(rename = "event_slug", default, skip_serializing_if = "Option::is_none")]
    pub event_slug: Option<String>,
    #[serde(rename = "final_price", default, skip_serializing_if = "Option::is_none")]
    pub final_price: Option<serde_json::Number>,
    #[serde(rename = "market_slug", default, skip_serializing_if = "Option::is_none")]
    pub market_slug: Option<String>,
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "neg_risk", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk: Option<bool>,
    #[serde(rename = "neg_risk_market_id", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk_market_id: Option<String>,
    #[serde(rename = "opposite_token_id", default, skip_serializing_if = "Option::is_none")]
    pub opposite_token_id: Option<String>,
    #[serde(rename = "outcome", default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[serde(rename = "outcome_index")]
    pub outcome_index: serde_json::Number,
    #[serde(rename = "question_index", default, skip_serializing_if = "Option::is_none")]
    pub question_index: Option<serde_json::Number>,
    #[serde(rename = "resolved")]
    pub resolved: bool,
    #[serde(rename = "structural_condition_id", default, skip_serializing_if = "Option::is_none")]
    pub structural_condition_id: Option<String>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataTokenRef {
    fn default() -> Self {
        Self {
            clob_index: Default::default(),
            closed: Default::default(),
            condition_id: Default::default(),
            event_id: Default::default(),
            event_slug: Default::default(),
            final_price: Default::default(),
            market_slug: Default::default(),
            module: Default::default(),
            neg_risk: Default::default(),
            neg_risk_market_id: Default::default(),
            opposite_token_id: Default::default(),
            outcome: Default::default(),
            outcome_index: 0.into(),
            question_index: Default::default(),
            resolved: Default::default(),
            structural_condition_id: Default::default(),
            title: Default::default(),
            token_id: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataTrade {
    #[serde(rename = "bio")]
    pub bio: String,
    #[serde(rename = "condition_id")]
    pub condition_id: String,
    #[serde(rename = "event_slug")]
    pub event_slug: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "outcome")]
    pub outcome: String,
    #[serde(rename = "outcome_index")]
    pub outcome_index: serde_json::Number,
    #[serde(rename = "price")]
    pub price: serde_json::Number,
    #[serde(rename = "profile_image")]
    pub profile_image: String,
    #[serde(rename = "profile_image_optimized")]
    pub profile_image_optimized: String,
    #[serde(rename = "proxy_wallet")]
    pub proxy_wallet: String,
    #[serde(rename = "pseudonym")]
    pub pseudonym: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "size")]
    pub size: serde_json::Number,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(rename = "transaction_hash")]
    pub transaction_hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataTrade {
    fn default() -> Self {
        Self {
            bio: Default::default(),
            condition_id: Default::default(),
            event_slug: Default::default(),
            icon: Default::default(),
            name: Default::default(),
            outcome: Default::default(),
            outcome_index: 0.into(),
            price: 0.into(),
            profile_image: Default::default(),
            profile_image_optimized: Default::default(),
            proxy_wallet: Default::default(),
            pseudonym: Default::default(),
            side: Default::default(),
            size: 0.into(),
            slug: Default::default(),
            timestamp: 0.into(),
            title: Default::default(),
            token_id: Default::default(),
            transaction_hash: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataTradesPage {
    #[serde(rename = "data")]
    pub data: Vec<DataTrade>,
    #[serde(rename = "pagination")]
    pub pagination: Box<DataPagination>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataUserPnlPoint {
    #[serde(rename = "cashflow_net", default, skip_serializing_if = "Option::is_none")]
    pub cashflow_net: Option<serde_json::Number>,
    #[serde(rename = "deposits", default, skip_serializing_if = "Option::is_none")]
    pub deposits: Option<serde_json::Number>,
    #[serde(rename = "economic_pnl", default, skip_serializing_if = "Option::is_none")]
    pub economic_pnl: Option<serde_json::Number>,
    #[serde(rename = "fees", default, skip_serializing_if = "Option::is_none")]
    pub fees: Option<serde_json::Number>,
    #[serde(rename = "fees_paid", default, skip_serializing_if = "Option::is_none")]
    pub fees_paid: Option<serde_json::Number>,
    #[serde(rename = "fees_refunded", default, skip_serializing_if = "Option::is_none")]
    pub fees_refunded: Option<serde_json::Number>,
    #[serde(rename = "maker_rebate", default, skip_serializing_if = "Option::is_none")]
    pub maker_rebate: Option<serde_json::Number>,
    #[serde(rename = "position_pnl", default, skip_serializing_if = "Option::is_none")]
    pub position_pnl: Option<serde_json::Number>,
    #[serde(rename = "realized_combo_pnl")]
    pub realized_combo_pnl: serde_json::Number,
    #[serde(rename = "realized_lp_pnl")]
    pub realized_lp_pnl: serde_json::Number,
    #[serde(rename = "realized_market_pnl")]
    pub realized_market_pnl: serde_json::Number,
    #[serde(rename = "realized_pnl")]
    pub realized_pnl: serde_json::Number,
    #[serde(rename = "referral_income", default, skip_serializing_if = "Option::is_none")]
    pub referral_income: Option<serde_json::Number>,
    #[serde(rename = "reward_income", default, skip_serializing_if = "Option::is_none")]
    pub reward_income: Option<serde_json::Number>,
    #[serde(rename = "settled_pnl", default, skip_serializing_if = "Option::is_none")]
    pub settled_pnl: Option<serde_json::Number>,
    #[serde(rename = "source_block")]
    pub source_block: serde_json::Number,
    #[serde(rename = "sponsored_income", default, skip_serializing_if = "Option::is_none")]
    pub sponsored_income: Option<serde_json::Number>,
    #[serde(rename = "taker_rebate", default, skip_serializing_if = "Option::is_none")]
    pub taker_rebate: Option<serde_json::Number>,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "trade_count")]
    pub trade_count: serde_json::Number,
    #[serde(rename = "trade_pnl", default, skip_serializing_if = "Option::is_none")]
    pub trade_pnl: Option<serde_json::Number>,
    #[serde(rename = "unrealized_pnl", default, skip_serializing_if = "Option::is_none")]
    pub unrealized_pnl: Option<serde_json::Number>,
    #[serde(rename = "volume")]
    pub volume: serde_json::Number,
    #[serde(rename = "volume_usdc")]
    pub volume_usdc: serde_json::Number,
    #[serde(rename = "wallet_income", default, skip_serializing_if = "Option::is_none")]
    pub wallet_income: Option<serde_json::Number>,
    #[serde(rename = "withdrawals", default, skip_serializing_if = "Option::is_none")]
    pub withdrawals: Option<serde_json::Number>,
    #[serde(rename = "yield_income", default, skip_serializing_if = "Option::is_none")]
    pub yield_income: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataUserPnlPoint {
    fn default() -> Self {
        Self {
            cashflow_net: Default::default(),
            deposits: Default::default(),
            economic_pnl: Default::default(),
            fees: Default::default(),
            fees_paid: Default::default(),
            fees_refunded: Default::default(),
            maker_rebate: Default::default(),
            position_pnl: Default::default(),
            realized_combo_pnl: 0.into(),
            realized_lp_pnl: 0.into(),
            realized_market_pnl: 0.into(),
            realized_pnl: 0.into(),
            referral_income: Default::default(),
            reward_income: Default::default(),
            settled_pnl: Default::default(),
            source_block: 0.into(),
            sponsored_income: Default::default(),
            taker_rebate: Default::default(),
            timestamp: 0.into(),
            trade_count: 0.into(),
            trade_pnl: Default::default(),
            unrealized_pnl: Default::default(),
            volume: 0.into(),
            volume_usdc: 0.into(),
            wallet_income: Default::default(),
            withdrawals: Default::default(),
            yield_income: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DataUserPnlSeries {
    #[serde(rename = "fidelity")]
    pub fidelity: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "points")]
    pub points: Vec<DataUserPnlPoint>,
    #[serde(rename = "proxy_wallet")]
    pub proxy_wallet: String,
    #[serde(rename = "source_fidelity")]
    pub source_fidelity: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataUserStats {
    #[serde(rename = "all_time_pnl", default, skip_serializing_if = "Option::is_none")]
    pub all_time_pnl: Option<serde_json::Value>,
    #[serde(rename = "biggest_win")]
    pub biggest_win: serde_json::Number,
    #[serde(rename = "join_date", default, skip_serializing_if = "Option::is_none")]
    pub join_date: Option<serde_json::Number>,
    #[serde(rename = "proxy_wallet")]
    pub proxy_wallet: String,
    #[serde(rename = "trades")]
    pub trades: serde_json::Number,
    #[serde(rename = "views")]
    pub views: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataUserStats {
    fn default() -> Self {
        Self {
            all_time_pnl: Default::default(),
            biggest_win: 0.into(),
            join_date: Default::default(),
            proxy_wallet: Default::default(),
            trades: 0.into(),
            views: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataUserVolume {
    #[serde(rename = "trade_count")]
    pub trade_count: serde_json::Number,
    #[serde(rename = "volume")]
    pub volume: serde_json::Number,
    #[serde(rename = "volume_usdc")]
    pub volume_usdc: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DataUserVolume {
    fn default() -> Self {
        Self {
            trade_count: 0.into(),
            volume: 0.into(),
            volume_usdc: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaCategory {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "parentCategory", default, skip_serializing_if = "Option::is_none")]
    pub parent_category: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "publishedAt", default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaChat {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "channelId", default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "channelName", default, skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "channelImage", default, skip_serializing_if = "Option::is_none")]
    pub channel_image: Option<String>,
    #[serde(rename = "live", default, skip_serializing_if = "Option::is_none")]
    pub live: Option<bool>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaCollection {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "subtitle", default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "collectionType", default, skip_serializing_if = "Option::is_none")]
    pub collection_type: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "icon", default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "headerImage", default, skip_serializing_if = "Option::is_none")]
    pub header_image: Option<String>,
    #[serde(rename = "layout", default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    #[serde(rename = "active", default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "archived", default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "new", default, skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    #[serde(rename = "featured", default, skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(rename = "restricted", default, skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[serde(rename = "isTemplate", default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(rename = "templateVariables", default, skip_serializing_if = "Option::is_none")]
    pub template_variables: Option<String>,
    #[serde(rename = "publishedAt", default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "commentsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub comments_enabled: Option<bool>,
    #[serde(rename = "imageOptimized", default, skip_serializing_if = "Option::is_none")]
    pub image_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "iconOptimized", default, skip_serializing_if = "Option::is_none")]
    pub icon_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "headerImageOptimized", default, skip_serializing_if = "Option::is_none")]
    pub header_image_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaComment {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "parentEntityType", default, skip_serializing_if = "Option::is_none")]
    pub parent_entity_type: Option<String>,
    #[serde(rename = "parentEntityID", default, skip_serializing_if = "Option::is_none")]
    pub parent_entity_id: Option<serde_json::Number>,
    #[serde(rename = "parentCommentID", default, skip_serializing_if = "Option::is_none")]
    pub parent_comment_id: Option<String>,
    #[serde(rename = "userAddress", default, skip_serializing_if = "Option::is_none")]
    pub user_address: Option<String>,
    #[serde(rename = "replyAddress", default, skip_serializing_if = "Option::is_none")]
    pub reply_address: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "profile", default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<Box<GammaCommentProfile>>,
    #[serde(rename = "reactions", default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<GammaReaction>>,
    #[serde(rename = "reportCount", default, skip_serializing_if = "Option::is_none")]
    pub report_count: Option<serde_json::Number>,
    #[serde(rename = "reactionCount", default, skip_serializing_if = "Option::is_none")]
    pub reaction_count: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaCommentPosition {
    #[serde(rename = "tokenId", default, skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(rename = "positionSize", default, skip_serializing_if = "Option::is_none")]
    pub position_size: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaCommentProfile {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pseudonym", default, skip_serializing_if = "Option::is_none")]
    pub pseudonym: Option<String>,
    #[serde(rename = "displayUsernamePublic", default, skip_serializing_if = "Option::is_none")]
    pub display_username_public: Option<bool>,
    #[serde(rename = "bio", default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(rename = "isMod", default, skip_serializing_if = "Option::is_none")]
    pub is_mod: Option<bool>,
    #[serde(rename = "isCreator", default, skip_serializing_if = "Option::is_none")]
    pub is_creator: Option<bool>,
    #[serde(rename = "proxyWallet", default, skip_serializing_if = "Option::is_none")]
    pub proxy_wallet: Option<String>,
    #[serde(rename = "baseAddress", default, skip_serializing_if = "Option::is_none")]
    pub base_address: Option<String>,
    #[serde(rename = "profileImage", default, skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>,
    #[serde(rename = "profileImageOptimized", default, skip_serializing_if = "Option::is_none")]
    pub profile_image_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<GammaCommentPosition>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaCount {
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaEvent {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "subtitle", default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "resolutionSource", default, skip_serializing_if = "Option::is_none")]
    pub resolution_source: Option<String>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "creationDate", default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "icon", default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "active", default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "archived", default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "new", default, skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    #[serde(rename = "featured", default, skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(rename = "restricted", default, skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<serde_json::Number>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<serde_json::Number>,
    #[serde(rename = "openInterest", default, skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<serde_json::Number>,
    #[serde(rename = "sortBy", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "subcategory", default, skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    #[serde(rename = "isTemplate", default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(rename = "templateVariables", default, skip_serializing_if = "Option::is_none")]
    pub template_variables: Option<String>,
    #[serde(rename = "published_at", default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "commentsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub comments_enabled: Option<bool>,
    #[serde(rename = "competitive", default, skip_serializing_if = "Option::is_none")]
    pub competitive: Option<serde_json::Number>,
    #[serde(rename = "volume24hr", default, skip_serializing_if = "Option::is_none")]
    pub volume24hr: Option<serde_json::Number>,
    #[serde(rename = "volume1wk", default, skip_serializing_if = "Option::is_none")]
    pub volume1wk: Option<serde_json::Number>,
    #[serde(rename = "volume1mo", default, skip_serializing_if = "Option::is_none")]
    pub volume1mo: Option<serde_json::Number>,
    #[serde(rename = "volume1yr", default, skip_serializing_if = "Option::is_none")]
    pub volume1yr: Option<serde_json::Number>,
    #[serde(rename = "featuredImage", default, skip_serializing_if = "Option::is_none")]
    pub featured_image: Option<String>,
    #[serde(rename = "disqusThread", default, skip_serializing_if = "Option::is_none")]
    pub disqus_thread: Option<String>,
    #[serde(rename = "parentEvent", default, skip_serializing_if = "Option::is_none")]
    pub parent_event: Option<String>,
    #[serde(rename = "enableOrderBook", default, skip_serializing_if = "Option::is_none")]
    pub enable_order_book: Option<bool>,
    #[serde(rename = "liquidityClob", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_clob: Option<serde_json::Number>,
    #[serde(rename = "negRisk", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk: Option<bool>,
    #[serde(rename = "negRiskMarketID", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk_market_id: Option<String>,
    #[serde(rename = "negRiskFeeBips", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk_fee_bips: Option<serde_json::Number>,
    #[serde(rename = "commentCount", default, skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<serde_json::Number>,
    #[serde(rename = "imageOptimized", default, skip_serializing_if = "Option::is_none")]
    pub image_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "iconOptimized", default, skip_serializing_if = "Option::is_none")]
    pub icon_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "featuredImageOptimized", default, skip_serializing_if = "Option::is_none")]
    pub featured_image_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "subEvents", default, skip_serializing_if = "Option::is_none")]
    pub sub_events: Option<Vec<String>>,
    #[serde(rename = "markets", default, skip_serializing_if = "Option::is_none")]
    pub markets: Option<Vec<GammaMarket>>,
    #[serde(rename = "series", default, skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<GammaSeries>>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<GammaCategory>>,
    #[serde(rename = "collections", default, skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<GammaCollection>>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GammaTag>>,
    #[serde(rename = "cyom", default, skip_serializing_if = "Option::is_none")]
    pub cyom: Option<bool>,
    #[serde(rename = "closedTime", default, skip_serializing_if = "Option::is_none")]
    pub closed_time: Option<String>,
    #[serde(rename = "showAllOutcomes", default, skip_serializing_if = "Option::is_none")]
    pub show_all_outcomes: Option<bool>,
    #[serde(rename = "showMarketImages", default, skip_serializing_if = "Option::is_none")]
    pub show_market_images: Option<bool>,
    #[serde(rename = "automaticallyResolved", default, skip_serializing_if = "Option::is_none")]
    pub automatically_resolved: Option<bool>,
    #[serde(rename = "enableNegRisk", default, skip_serializing_if = "Option::is_none")]
    pub enable_neg_risk: Option<bool>,
    #[serde(rename = "automaticallyActive", default, skip_serializing_if = "Option::is_none")]
    pub automatically_active: Option<bool>,
    #[serde(rename = "eventDate", default, skip_serializing_if = "Option::is_none")]
    pub event_date: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "eventWeek", default, skip_serializing_if = "Option::is_none")]
    pub event_week: Option<serde_json::Number>,
    #[serde(rename = "seriesSlug", default, skip_serializing_if = "Option::is_none")]
    pub series_slug: Option<String>,
    #[serde(rename = "score", default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    #[serde(rename = "elapsed", default, skip_serializing_if = "Option::is_none")]
    pub elapsed: Option<String>,
    #[serde(rename = "period", default, skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(rename = "live", default, skip_serializing_if = "Option::is_none")]
    pub live: Option<bool>,
    #[serde(rename = "ended", default, skip_serializing_if = "Option::is_none")]
    pub ended: Option<bool>,
    #[serde(rename = "finishedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub finished_timestamp: Option<String>,
    #[serde(rename = "gmpChartMode", default, skip_serializing_if = "Option::is_none")]
    pub gmp_chart_mode: Option<String>,
    #[serde(rename = "eventCreators", default, skip_serializing_if = "Option::is_none")]
    pub event_creators: Option<Vec<GammaEventCreator>>,
    #[serde(rename = "tweetCount", default, skip_serializing_if = "Option::is_none")]
    pub tweet_count: Option<serde_json::Number>,
    #[serde(rename = "chats", default, skip_serializing_if = "Option::is_none")]
    pub chats: Option<Vec<GammaChat>>,
    #[serde(rename = "featuredOrder", default, skip_serializing_if = "Option::is_none")]
    pub featured_order: Option<serde_json::Number>,
    #[serde(rename = "estimateValue", default, skip_serializing_if = "Option::is_none")]
    pub estimate_value: Option<bool>,
    #[serde(rename = "cantEstimate", default, skip_serializing_if = "Option::is_none")]
    pub cant_estimate: Option<bool>,
    #[serde(rename = "estimatedValue", default, skip_serializing_if = "Option::is_none")]
    pub estimated_value: Option<String>,
    #[serde(rename = "templates", default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<GammaTemplate>>,
    #[serde(rename = "spreadsMainLine", default, skip_serializing_if = "Option::is_none")]
    pub spreads_main_line: Option<serde_json::Number>,
    #[serde(rename = "totalsMainLine", default, skip_serializing_if = "Option::is_none")]
    pub totals_main_line: Option<serde_json::Number>,
    #[serde(rename = "carouselMap", default, skip_serializing_if = "Option::is_none")]
    pub carousel_map: Option<String>,
    #[serde(rename = "pendingDeployment", default, skip_serializing_if = "Option::is_none")]
    pub pending_deployment: Option<bool>,
    #[serde(rename = "deploying", default, skip_serializing_if = "Option::is_none")]
    pub deploying: Option<bool>,
    #[serde(rename = "deployingTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub deploying_timestamp: Option<String>,
    #[serde(rename = "scheduledDeploymentTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_deployment_timestamp: Option<String>,
    #[serde(rename = "gameStatus", default, skip_serializing_if = "Option::is_none")]
    pub game_status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaEventCreator {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "creatorName", default, skip_serializing_if = "Option::is_none")]
    pub creator_name: Option<String>,
    #[serde(rename = "creatorHandle", default, skip_serializing_if = "Option::is_none")]
    pub creator_handle: Option<String>,
    #[serde(rename = "creatorUrl", default, skip_serializing_if = "Option::is_none")]
    pub creator_url: Option<String>,
    #[serde(rename = "creatorImage", default, skip_serializing_if = "Option::is_none")]
    pub creator_image: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaEventTweetCount {
    #[serde(rename = "tweetCount", default, skip_serializing_if = "Option::is_none")]
    pub tweet_count: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaEventsPagination {
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GammaEvent>>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<GammaPagination>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaFeeSchedule {
    #[serde(rename = "exponent", default, skip_serializing_if = "Option::is_none")]
    pub exponent: Option<serde_json::Number>,
    #[serde(rename = "rate", default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<serde_json::Number>,
    #[serde(rename = "takerOnly", default, skip_serializing_if = "Option::is_none")]
    pub taker_only: Option<bool>,
    #[serde(rename = "rebateRate", default, skip_serializing_if = "Option::is_none")]
    pub rebate_rate: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetAbridgedMarketsParams {
    #[serde(rename = "body")]
    pub body: Box<GammaMarketsInformationBody>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetCommentsByIdParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(rename = "get_positions", default, skip_serializing_if = "Option::is_none")]
    pub get_positions: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetCommentsByIdParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            get_positions: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetCommentsByUserAddressParams {
    #[serde(rename = "user_address")]
    pub user_address: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Number>,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "ascending", default, skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetEventBySlugParams {
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "include_chat", default, skip_serializing_if = "Option::is_none")]
    pub include_chat: Option<bool>,
    #[serde(rename = "include_template", default, skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetEventBySlugResponse {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "subtitle", default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "resolutionSource", default, skip_serializing_if = "Option::is_none")]
    pub resolution_source: Option<String>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "creationDate", default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "icon", default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "active", default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "archived", default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "new", default, skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    #[serde(rename = "featured", default, skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(rename = "restricted", default, skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<serde_json::Number>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<serde_json::Number>,
    #[serde(rename = "openInterest", default, skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<serde_json::Number>,
    #[serde(rename = "sortBy", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "subcategory", default, skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    #[serde(rename = "isTemplate", default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(rename = "templateVariables", default, skip_serializing_if = "Option::is_none")]
    pub template_variables: Option<String>,
    #[serde(rename = "published_at", default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "commentsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub comments_enabled: Option<bool>,
    #[serde(rename = "competitive", default, skip_serializing_if = "Option::is_none")]
    pub competitive: Option<serde_json::Number>,
    #[serde(rename = "volume24hr", default, skip_serializing_if = "Option::is_none")]
    pub volume24hr: Option<serde_json::Number>,
    #[serde(rename = "volume1wk", default, skip_serializing_if = "Option::is_none")]
    pub volume1wk: Option<serde_json::Number>,
    #[serde(rename = "volume1mo", default, skip_serializing_if = "Option::is_none")]
    pub volume1mo: Option<serde_json::Number>,
    #[serde(rename = "volume1yr", default, skip_serializing_if = "Option::is_none")]
    pub volume1yr: Option<serde_json::Number>,
    #[serde(rename = "featuredImage", default, skip_serializing_if = "Option::is_none")]
    pub featured_image: Option<String>,
    #[serde(rename = "disqusThread", default, skip_serializing_if = "Option::is_none")]
    pub disqus_thread: Option<String>,
    #[serde(rename = "parentEvent", default, skip_serializing_if = "Option::is_none")]
    pub parent_event: Option<String>,
    #[serde(rename = "enableOrderBook", default, skip_serializing_if = "Option::is_none")]
    pub enable_order_book: Option<bool>,
    #[serde(rename = "liquidityClob", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_clob: Option<serde_json::Number>,
    #[serde(rename = "negRisk", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk: Option<bool>,
    #[serde(rename = "negRiskMarketID", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk_market_id: Option<String>,
    #[serde(rename = "negRiskFeeBips", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk_fee_bips: Option<serde_json::Number>,
    #[serde(rename = "commentCount", default, skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<serde_json::Number>,
    #[serde(rename = "imageOptimized", default, skip_serializing_if = "Option::is_none")]
    pub image_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "iconOptimized", default, skip_serializing_if = "Option::is_none")]
    pub icon_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "featuredImageOptimized", default, skip_serializing_if = "Option::is_none")]
    pub featured_image_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "subEvents", default, skip_serializing_if = "Option::is_none")]
    pub sub_events: Option<Vec<String>>,
    #[serde(rename = "markets", default, skip_serializing_if = "Option::is_none")]
    pub markets: Option<Vec<GammaMarket>>,
    #[serde(rename = "series", default, skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<GammaSeries>>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<GammaCategory>>,
    #[serde(rename = "collections", default, skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<GammaCollection>>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GammaTag>>,
    #[serde(rename = "cyom", default, skip_serializing_if = "Option::is_none")]
    pub cyom: Option<bool>,
    #[serde(rename = "closedTime", default, skip_serializing_if = "Option::is_none")]
    pub closed_time: Option<String>,
    #[serde(rename = "showAllOutcomes", default, skip_serializing_if = "Option::is_none")]
    pub show_all_outcomes: Option<bool>,
    #[serde(rename = "showMarketImages", default, skip_serializing_if = "Option::is_none")]
    pub show_market_images: Option<bool>,
    #[serde(rename = "automaticallyResolved", default, skip_serializing_if = "Option::is_none")]
    pub automatically_resolved: Option<bool>,
    #[serde(rename = "enableNegRisk", default, skip_serializing_if = "Option::is_none")]
    pub enable_neg_risk: Option<bool>,
    #[serde(rename = "automaticallyActive", default, skip_serializing_if = "Option::is_none")]
    pub automatically_active: Option<bool>,
    #[serde(rename = "eventDate", default, skip_serializing_if = "Option::is_none")]
    pub event_date: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "eventWeek", default, skip_serializing_if = "Option::is_none")]
    pub event_week: Option<serde_json::Number>,
    #[serde(rename = "seriesSlug", default, skip_serializing_if = "Option::is_none")]
    pub series_slug: Option<String>,
    #[serde(rename = "score", default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    #[serde(rename = "elapsed", default, skip_serializing_if = "Option::is_none")]
    pub elapsed: Option<String>,
    #[serde(rename = "period", default, skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(rename = "live", default, skip_serializing_if = "Option::is_none")]
    pub live: Option<bool>,
    #[serde(rename = "ended", default, skip_serializing_if = "Option::is_none")]
    pub ended: Option<bool>,
    #[serde(rename = "finishedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub finished_timestamp: Option<String>,
    #[serde(rename = "gmpChartMode", default, skip_serializing_if = "Option::is_none")]
    pub gmp_chart_mode: Option<String>,
    #[serde(rename = "eventCreators", default, skip_serializing_if = "Option::is_none")]
    pub event_creators: Option<Vec<GammaEventCreator>>,
    #[serde(rename = "tweetCount", default, skip_serializing_if = "Option::is_none")]
    pub tweet_count: Option<serde_json::Number>,
    #[serde(rename = "chats", default, skip_serializing_if = "Option::is_none")]
    pub chats: Option<Vec<GammaChat>>,
    #[serde(rename = "featuredOrder", default, skip_serializing_if = "Option::is_none")]
    pub featured_order: Option<serde_json::Number>,
    #[serde(rename = "estimateValue", default, skip_serializing_if = "Option::is_none")]
    pub estimate_value: Option<bool>,
    #[serde(rename = "cantEstimate", default, skip_serializing_if = "Option::is_none")]
    pub cant_estimate: Option<bool>,
    #[serde(rename = "estimatedValue", default, skip_serializing_if = "Option::is_none")]
    pub estimated_value: Option<String>,
    #[serde(rename = "templates", default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<GammaTemplate>>,
    #[serde(rename = "spreadsMainLine", default, skip_serializing_if = "Option::is_none")]
    pub spreads_main_line: Option<serde_json::Number>,
    #[serde(rename = "totalsMainLine", default, skip_serializing_if = "Option::is_none")]
    pub totals_main_line: Option<serde_json::Number>,
    #[serde(rename = "carouselMap", default, skip_serializing_if = "Option::is_none")]
    pub carousel_map: Option<String>,
    #[serde(rename = "pendingDeployment", default, skip_serializing_if = "Option::is_none")]
    pub pending_deployment: Option<bool>,
    #[serde(rename = "deploying", default, skip_serializing_if = "Option::is_none")]
    pub deploying: Option<bool>,
    #[serde(rename = "deployingTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub deploying_timestamp: Option<String>,
    #[serde(rename = "scheduledDeploymentTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_deployment_timestamp: Option<String>,
    #[serde(rename = "gameStatus", default, skip_serializing_if = "Option::is_none")]
    pub game_status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetEventCommentsCountParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetEventCommentsCountParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetEventCommentsCountResponse {
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetEventCreatorParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetEventCreatorParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetEventCreatorResponse {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "creatorName", default, skip_serializing_if = "Option::is_none")]
    pub creator_name: Option<String>,
    #[serde(rename = "creatorHandle", default, skip_serializing_if = "Option::is_none")]
    pub creator_handle: Option<String>,
    #[serde(rename = "creatorUrl", default, skip_serializing_if = "Option::is_none")]
    pub creator_url: Option<String>,
    #[serde(rename = "creatorImage", default, skip_serializing_if = "Option::is_none")]
    pub creator_image: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetEventParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(rename = "include_chat", default, skip_serializing_if = "Option::is_none")]
    pub include_chat: Option<bool>,
    #[serde(rename = "include_template", default, skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetEventParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            include_chat: Default::default(),
            include_template: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetEventResponse {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "subtitle", default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "resolutionSource", default, skip_serializing_if = "Option::is_none")]
    pub resolution_source: Option<String>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "creationDate", default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "icon", default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "active", default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "archived", default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "new", default, skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    #[serde(rename = "featured", default, skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(rename = "restricted", default, skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<serde_json::Number>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<serde_json::Number>,
    #[serde(rename = "openInterest", default, skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<serde_json::Number>,
    #[serde(rename = "sortBy", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "subcategory", default, skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    #[serde(rename = "isTemplate", default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(rename = "templateVariables", default, skip_serializing_if = "Option::is_none")]
    pub template_variables: Option<String>,
    #[serde(rename = "published_at", default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "commentsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub comments_enabled: Option<bool>,
    #[serde(rename = "competitive", default, skip_serializing_if = "Option::is_none")]
    pub competitive: Option<serde_json::Number>,
    #[serde(rename = "volume24hr", default, skip_serializing_if = "Option::is_none")]
    pub volume24hr: Option<serde_json::Number>,
    #[serde(rename = "volume1wk", default, skip_serializing_if = "Option::is_none")]
    pub volume1wk: Option<serde_json::Number>,
    #[serde(rename = "volume1mo", default, skip_serializing_if = "Option::is_none")]
    pub volume1mo: Option<serde_json::Number>,
    #[serde(rename = "volume1yr", default, skip_serializing_if = "Option::is_none")]
    pub volume1yr: Option<serde_json::Number>,
    #[serde(rename = "featuredImage", default, skip_serializing_if = "Option::is_none")]
    pub featured_image: Option<String>,
    #[serde(rename = "disqusThread", default, skip_serializing_if = "Option::is_none")]
    pub disqus_thread: Option<String>,
    #[serde(rename = "parentEvent", default, skip_serializing_if = "Option::is_none")]
    pub parent_event: Option<String>,
    #[serde(rename = "enableOrderBook", default, skip_serializing_if = "Option::is_none")]
    pub enable_order_book: Option<bool>,
    #[serde(rename = "liquidityClob", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_clob: Option<serde_json::Number>,
    #[serde(rename = "negRisk", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk: Option<bool>,
    #[serde(rename = "negRiskMarketID", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk_market_id: Option<String>,
    #[serde(rename = "negRiskFeeBips", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk_fee_bips: Option<serde_json::Number>,
    #[serde(rename = "commentCount", default, skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<serde_json::Number>,
    #[serde(rename = "imageOptimized", default, skip_serializing_if = "Option::is_none")]
    pub image_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "iconOptimized", default, skip_serializing_if = "Option::is_none")]
    pub icon_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "featuredImageOptimized", default, skip_serializing_if = "Option::is_none")]
    pub featured_image_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "subEvents", default, skip_serializing_if = "Option::is_none")]
    pub sub_events: Option<Vec<String>>,
    #[serde(rename = "markets", default, skip_serializing_if = "Option::is_none")]
    pub markets: Option<Vec<GammaMarket>>,
    #[serde(rename = "series", default, skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<GammaSeries>>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<GammaCategory>>,
    #[serde(rename = "collections", default, skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<GammaCollection>>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GammaTag>>,
    #[serde(rename = "cyom", default, skip_serializing_if = "Option::is_none")]
    pub cyom: Option<bool>,
    #[serde(rename = "closedTime", default, skip_serializing_if = "Option::is_none")]
    pub closed_time: Option<String>,
    #[serde(rename = "showAllOutcomes", default, skip_serializing_if = "Option::is_none")]
    pub show_all_outcomes: Option<bool>,
    #[serde(rename = "showMarketImages", default, skip_serializing_if = "Option::is_none")]
    pub show_market_images: Option<bool>,
    #[serde(rename = "automaticallyResolved", default, skip_serializing_if = "Option::is_none")]
    pub automatically_resolved: Option<bool>,
    #[serde(rename = "enableNegRisk", default, skip_serializing_if = "Option::is_none")]
    pub enable_neg_risk: Option<bool>,
    #[serde(rename = "automaticallyActive", default, skip_serializing_if = "Option::is_none")]
    pub automatically_active: Option<bool>,
    #[serde(rename = "eventDate", default, skip_serializing_if = "Option::is_none")]
    pub event_date: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "eventWeek", default, skip_serializing_if = "Option::is_none")]
    pub event_week: Option<serde_json::Number>,
    #[serde(rename = "seriesSlug", default, skip_serializing_if = "Option::is_none")]
    pub series_slug: Option<String>,
    #[serde(rename = "score", default, skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    #[serde(rename = "elapsed", default, skip_serializing_if = "Option::is_none")]
    pub elapsed: Option<String>,
    #[serde(rename = "period", default, skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(rename = "live", default, skip_serializing_if = "Option::is_none")]
    pub live: Option<bool>,
    #[serde(rename = "ended", default, skip_serializing_if = "Option::is_none")]
    pub ended: Option<bool>,
    #[serde(rename = "finishedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub finished_timestamp: Option<String>,
    #[serde(rename = "gmpChartMode", default, skip_serializing_if = "Option::is_none")]
    pub gmp_chart_mode: Option<String>,
    #[serde(rename = "eventCreators", default, skip_serializing_if = "Option::is_none")]
    pub event_creators: Option<Vec<GammaEventCreator>>,
    #[serde(rename = "tweetCount", default, skip_serializing_if = "Option::is_none")]
    pub tweet_count: Option<serde_json::Number>,
    #[serde(rename = "chats", default, skip_serializing_if = "Option::is_none")]
    pub chats: Option<Vec<GammaChat>>,
    #[serde(rename = "featuredOrder", default, skip_serializing_if = "Option::is_none")]
    pub featured_order: Option<serde_json::Number>,
    #[serde(rename = "estimateValue", default, skip_serializing_if = "Option::is_none")]
    pub estimate_value: Option<bool>,
    #[serde(rename = "cantEstimate", default, skip_serializing_if = "Option::is_none")]
    pub cant_estimate: Option<bool>,
    #[serde(rename = "estimatedValue", default, skip_serializing_if = "Option::is_none")]
    pub estimated_value: Option<String>,
    #[serde(rename = "templates", default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<GammaTemplate>>,
    #[serde(rename = "spreadsMainLine", default, skip_serializing_if = "Option::is_none")]
    pub spreads_main_line: Option<serde_json::Number>,
    #[serde(rename = "totalsMainLine", default, skip_serializing_if = "Option::is_none")]
    pub totals_main_line: Option<serde_json::Number>,
    #[serde(rename = "carouselMap", default, skip_serializing_if = "Option::is_none")]
    pub carousel_map: Option<String>,
    #[serde(rename = "pendingDeployment", default, skip_serializing_if = "Option::is_none")]
    pub pending_deployment: Option<bool>,
    #[serde(rename = "deploying", default, skip_serializing_if = "Option::is_none")]
    pub deploying: Option<bool>,
    #[serde(rename = "deployingTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub deploying_timestamp: Option<String>,
    #[serde(rename = "scheduledDeploymentTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_deployment_timestamp: Option<String>,
    #[serde(rename = "gameStatus", default, skip_serializing_if = "Option::is_none")]
    pub game_status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetEventTagsParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetEventTagsParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetEventTweetCountParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetEventTweetCountParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetEventTweetCountResponse {
    #[serde(rename = "tweetCount", default, skip_serializing_if = "Option::is_none")]
    pub tweet_count: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetGammaStatusParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetMarketBySlugParams {
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "include_tag", default, skip_serializing_if = "Option::is_none")]
    pub include_tag: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetMarketBySlugResponse {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "question", default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(rename = "conditionId", default, skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "twitterCardImage", default, skip_serializing_if = "Option::is_none")]
    pub twitter_card_image: Option<String>,
    #[serde(rename = "resolutionSource", default, skip_serializing_if = "Option::is_none")]
    pub resolution_source: Option<String>,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    #[serde(rename = "sponsorName", default, skip_serializing_if = "Option::is_none")]
    pub sponsor_name: Option<String>,
    #[serde(rename = "sponsorImage", default, skip_serializing_if = "Option::is_none")]
    pub sponsor_image: Option<String>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "xAxisValue", default, skip_serializing_if = "Option::is_none")]
    pub x_axis_value: Option<String>,
    #[serde(rename = "yAxisValue", default, skip_serializing_if = "Option::is_none")]
    pub y_axis_value: Option<String>,
    #[serde(rename = "denominationToken", default, skip_serializing_if = "Option::is_none")]
    pub denomination_token: Option<String>,
    #[serde(rename = "fee", default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "icon", default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "lowerBound", default, skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<String>,
    #[serde(rename = "upperBound", default, skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "outcomes", default, skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<String>,
    #[serde(rename = "outcomePrices", default, skip_serializing_if = "Option::is_none")]
    pub outcome_prices: Option<String>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(rename = "active", default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "marketType", default, skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
    #[serde(rename = "formatType", default, skip_serializing_if = "Option::is_none")]
    pub format_type: Option<String>,
    #[serde(rename = "lowerBoundDate", default, skip_serializing_if = "Option::is_none")]
    pub lower_bound_date: Option<String>,
    #[serde(rename = "upperBoundDate", default, skip_serializing_if = "Option::is_none")]
    pub upper_bound_date: Option<String>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<serde_json::Number>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<serde_json::Number>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "closedTime", default, skip_serializing_if = "Option::is_none")]
    pub closed_time: Option<String>,
    #[serde(rename = "wideFormat", default, skip_serializing_if = "Option::is_none")]
    pub wide_format: Option<bool>,
    #[serde(rename = "new", default, skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    #[serde(rename = "mailchimpTag", default, skip_serializing_if = "Option::is_none")]
    pub mailchimp_tag: Option<String>,
    #[serde(rename = "featured", default, skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(rename = "archived", default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "resolvedBy", default, skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<String>,
    #[serde(rename = "restricted", default, skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[serde(rename = "marketGroup", default, skip_serializing_if = "Option::is_none")]
    pub market_group: Option<serde_json::Number>,
    #[serde(rename = "groupItemTitle", default, skip_serializing_if = "Option::is_none")]
    pub group_item_title: Option<String>,
    #[serde(rename = "groupItemThreshold", default, skip_serializing_if = "Option::is_none")]
    pub group_item_threshold: Option<String>,
    #[serde(rename = "questionID", default, skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    #[serde(rename = "umaEndDate", default, skip_serializing_if = "Option::is_none")]
    pub uma_end_date: Option<String>,
    #[serde(rename = "enableOrderBook", default, skip_serializing_if = "Option::is_none")]
    pub enable_order_book: Option<bool>,
    #[serde(rename = "orderPriceMinTickSize", default, skip_serializing_if = "Option::is_none")]
    pub order_price_min_tick_size: Option<serde_json::Number>,
    #[serde(rename = "orderMinSize", default, skip_serializing_if = "Option::is_none")]
    pub order_min_size: Option<serde_json::Number>,
    #[serde(rename = "umaResolutionStatus", default, skip_serializing_if = "Option::is_none")]
    pub uma_resolution_status: Option<String>,
    #[serde(rename = "curationOrder", default, skip_serializing_if = "Option::is_none")]
    pub curation_order: Option<serde_json::Number>,
    #[serde(rename = "volumeNum", default, skip_serializing_if = "Option::is_none")]
    pub volume_num: Option<serde_json::Number>,
    #[serde(rename = "liquidityNum", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_num: Option<serde_json::Number>,
    #[serde(rename = "endDateIso", default, skip_serializing_if = "Option::is_none")]
    pub end_date_iso: Option<String>,
    #[serde(rename = "startDateIso", default, skip_serializing_if = "Option::is_none")]
    pub start_date_iso: Option<String>,
    #[serde(rename = "umaEndDateIso", default, skip_serializing_if = "Option::is_none")]
    pub uma_end_date_iso: Option<String>,
    #[serde(rename = "hasReviewedDates", default, skip_serializing_if = "Option::is_none")]
    pub has_reviewed_dates: Option<bool>,
    #[serde(rename = "readyForCron", default, skip_serializing_if = "Option::is_none")]
    pub ready_for_cron: Option<bool>,
    #[serde(rename = "commentsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub comments_enabled: Option<bool>,
    #[serde(rename = "volume24hr", default, skip_serializing_if = "Option::is_none")]
    pub volume24hr: Option<serde_json::Number>,
    #[serde(rename = "volume1wk", default, skip_serializing_if = "Option::is_none")]
    pub volume1wk: Option<serde_json::Number>,
    #[serde(rename = "volume1mo", default, skip_serializing_if = "Option::is_none")]
    pub volume1mo: Option<serde_json::Number>,
    #[serde(rename = "volume1yr", default, skip_serializing_if = "Option::is_none")]
    pub volume1yr: Option<serde_json::Number>,
    #[serde(rename = "gameStartTime", default, skip_serializing_if = "Option::is_none")]
    pub game_start_time: Option<String>,
    #[serde(rename = "secondsDelay", default, skip_serializing_if = "Option::is_none")]
    pub seconds_delay: Option<serde_json::Number>,
    #[serde(rename = "version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "clobTokenIds", default, skip_serializing_if = "Option::is_none")]
    pub clob_token_ids: Option<String>,
    #[serde(rename = "positionIds", default, skip_serializing_if = "Option::is_none")]
    pub position_ids: Option<Vec<String>>,
    #[serde(rename = "disqusThread", default, skip_serializing_if = "Option::is_none")]
    pub disqus_thread: Option<String>,
    #[serde(rename = "shortOutcomes", default, skip_serializing_if = "Option::is_none")]
    pub short_outcomes: Option<String>,
    #[serde(rename = "teamAID", default, skip_serializing_if = "Option::is_none")]
    pub team_aid: Option<String>,
    #[serde(rename = "teamBID", default, skip_serializing_if = "Option::is_none")]
    pub team_bid: Option<String>,
    #[serde(rename = "umaBond", default, skip_serializing_if = "Option::is_none")]
    pub uma_bond: Option<String>,
    #[serde(rename = "umaReward", default, skip_serializing_if = "Option::is_none")]
    pub uma_reward: Option<String>,
    #[serde(rename = "volume24hrClob", default, skip_serializing_if = "Option::is_none")]
    pub volume24hr_clob: Option<serde_json::Number>,
    #[serde(rename = "volume1wkClob", default, skip_serializing_if = "Option::is_none")]
    pub volume1wk_clob: Option<serde_json::Number>,
    #[serde(rename = "volume1moClob", default, skip_serializing_if = "Option::is_none")]
    pub volume1mo_clob: Option<serde_json::Number>,
    #[serde(rename = "volume1yrClob", default, skip_serializing_if = "Option::is_none")]
    pub volume1yr_clob: Option<serde_json::Number>,
    #[serde(rename = "volumeClob", default, skip_serializing_if = "Option::is_none")]
    pub volume_clob: Option<serde_json::Number>,
    #[serde(rename = "liquidityClob", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_clob: Option<serde_json::Number>,
    #[serde(rename = "makerBaseFee", default, skip_serializing_if = "Option::is_none")]
    pub maker_base_fee: Option<serde_json::Number>,
    #[serde(rename = "takerBaseFee", default, skip_serializing_if = "Option::is_none")]
    pub taker_base_fee: Option<serde_json::Number>,
    #[serde(rename = "customLiveness", default, skip_serializing_if = "Option::is_none")]
    pub custom_liveness: Option<serde_json::Number>,
    #[serde(rename = "acceptingOrders", default, skip_serializing_if = "Option::is_none")]
    pub accepting_orders: Option<bool>,
    #[serde(rename = "notificationsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
    #[serde(rename = "score", default, skip_serializing_if = "Option::is_none")]
    pub score: Option<serde_json::Number>,
    #[serde(rename = "imageOptimized", default, skip_serializing_if = "Option::is_none")]
    pub image_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "iconOptimized", default, skip_serializing_if = "Option::is_none")]
    pub icon_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GammaEvent>>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<GammaCategory>>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GammaTag>>,
    #[serde(rename = "creator", default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(rename = "ready", default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[serde(rename = "funded", default, skip_serializing_if = "Option::is_none")]
    pub funded: Option<bool>,
    #[serde(rename = "pastSlugs", default, skip_serializing_if = "Option::is_none")]
    pub past_slugs: Option<String>,
    #[serde(rename = "readyTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub ready_timestamp: Option<String>,
    #[serde(rename = "fundedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub funded_timestamp: Option<String>,
    #[serde(rename = "acceptingOrdersTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub accepting_orders_timestamp: Option<String>,
    #[serde(rename = "competitive", default, skip_serializing_if = "Option::is_none")]
    pub competitive: Option<serde_json::Number>,
    #[serde(rename = "rewardsMinSize", default, skip_serializing_if = "Option::is_none")]
    pub rewards_min_size: Option<serde_json::Number>,
    #[serde(rename = "rewardsMaxSpread", default, skip_serializing_if = "Option::is_none")]
    pub rewards_max_spread: Option<serde_json::Number>,
    #[serde(rename = "spread", default, skip_serializing_if = "Option::is_none")]
    pub spread: Option<serde_json::Number>,
    #[serde(rename = "automaticallyResolved", default, skip_serializing_if = "Option::is_none")]
    pub automatically_resolved: Option<bool>,
    #[serde(rename = "oneDayPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_day_price_change: Option<serde_json::Number>,
    #[serde(rename = "oneHourPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_hour_price_change: Option<serde_json::Number>,
    #[serde(rename = "oneWeekPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_week_price_change: Option<serde_json::Number>,
    #[serde(rename = "oneMonthPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_month_price_change: Option<serde_json::Number>,
    #[serde(rename = "oneYearPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_year_price_change: Option<serde_json::Number>,
    #[serde(rename = "lastTradePrice", default, skip_serializing_if = "Option::is_none")]
    pub last_trade_price: Option<serde_json::Number>,
    #[serde(rename = "bestBid", default, skip_serializing_if = "Option::is_none")]
    pub best_bid: Option<serde_json::Number>,
    #[serde(rename = "bestAsk", default, skip_serializing_if = "Option::is_none")]
    pub best_ask: Option<serde_json::Number>,
    #[serde(rename = "automaticallyActive", default, skip_serializing_if = "Option::is_none")]
    pub automatically_active: Option<bool>,
    #[serde(rename = "clearBookOnStart", default, skip_serializing_if = "Option::is_none")]
    pub clear_book_on_start: Option<bool>,
    #[serde(rename = "chartColor", default, skip_serializing_if = "Option::is_none")]
    pub chart_color: Option<String>,
    #[serde(rename = "seriesColor", default, skip_serializing_if = "Option::is_none")]
    pub series_color: Option<String>,
    #[serde(rename = "showGmpSeries", default, skip_serializing_if = "Option::is_none")]
    pub show_gmp_series: Option<bool>,
    #[serde(rename = "showGmpOutcome", default, skip_serializing_if = "Option::is_none")]
    pub show_gmp_outcome: Option<bool>,
    #[serde(rename = "manualActivation", default, skip_serializing_if = "Option::is_none")]
    pub manual_activation: Option<bool>,
    #[serde(rename = "negRiskOther", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk_other: Option<bool>,
    #[serde(rename = "gameId", default, skip_serializing_if = "Option::is_none")]
    pub game_id: Option<String>,
    #[serde(rename = "groupItemRange", default, skip_serializing_if = "Option::is_none")]
    pub group_item_range: Option<String>,
    #[serde(rename = "sportsMarketType", default, skip_serializing_if = "Option::is_none")]
    pub sports_market_type: Option<String>,
    #[serde(rename = "line", default, skip_serializing_if = "Option::is_none")]
    pub line: Option<serde_json::Number>,
    #[serde(rename = "umaResolutionStatuses", default, skip_serializing_if = "Option::is_none")]
    pub uma_resolution_statuses: Option<String>,
    #[serde(rename = "pendingDeployment", default, skip_serializing_if = "Option::is_none")]
    pub pending_deployment: Option<bool>,
    #[serde(rename = "deploying", default, skip_serializing_if = "Option::is_none")]
    pub deploying: Option<bool>,
    #[serde(rename = "deployingTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub deploying_timestamp: Option<String>,
    #[serde(rename = "scheduledDeploymentTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_deployment_timestamp: Option<String>,
    #[serde(rename = "rfqEnabled", default, skip_serializing_if = "Option::is_none")]
    pub rfq_enabled: Option<bool>,
    #[serde(rename = "eventStartTime", default, skip_serializing_if = "Option::is_none")]
    pub event_start_time: Option<String>,
    #[serde(rename = "feesEnabled", default, skip_serializing_if = "Option::is_none")]
    pub fees_enabled: Option<bool>,
    #[serde(rename = "feeSchedule", default, skip_serializing_if = "Option::is_none")]
    pub fee_schedule: Option<Box<GammaFeeSchedule>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetMarketDescriptionParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetMarketDescriptionParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetMarketDescriptionResponse {
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetMarketParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(rename = "include_tag", default, skip_serializing_if = "Option::is_none")]
    pub include_tag: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetMarketParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            include_tag: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetMarketResponse {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "question", default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(rename = "conditionId", default, skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "twitterCardImage", default, skip_serializing_if = "Option::is_none")]
    pub twitter_card_image: Option<String>,
    #[serde(rename = "resolutionSource", default, skip_serializing_if = "Option::is_none")]
    pub resolution_source: Option<String>,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    #[serde(rename = "sponsorName", default, skip_serializing_if = "Option::is_none")]
    pub sponsor_name: Option<String>,
    #[serde(rename = "sponsorImage", default, skip_serializing_if = "Option::is_none")]
    pub sponsor_image: Option<String>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "xAxisValue", default, skip_serializing_if = "Option::is_none")]
    pub x_axis_value: Option<String>,
    #[serde(rename = "yAxisValue", default, skip_serializing_if = "Option::is_none")]
    pub y_axis_value: Option<String>,
    #[serde(rename = "denominationToken", default, skip_serializing_if = "Option::is_none")]
    pub denomination_token: Option<String>,
    #[serde(rename = "fee", default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "icon", default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "lowerBound", default, skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<String>,
    #[serde(rename = "upperBound", default, skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "outcomes", default, skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<String>,
    #[serde(rename = "outcomePrices", default, skip_serializing_if = "Option::is_none")]
    pub outcome_prices: Option<String>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(rename = "active", default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "marketType", default, skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
    #[serde(rename = "formatType", default, skip_serializing_if = "Option::is_none")]
    pub format_type: Option<String>,
    #[serde(rename = "lowerBoundDate", default, skip_serializing_if = "Option::is_none")]
    pub lower_bound_date: Option<String>,
    #[serde(rename = "upperBoundDate", default, skip_serializing_if = "Option::is_none")]
    pub upper_bound_date: Option<String>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<serde_json::Number>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<serde_json::Number>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "closedTime", default, skip_serializing_if = "Option::is_none")]
    pub closed_time: Option<String>,
    #[serde(rename = "wideFormat", default, skip_serializing_if = "Option::is_none")]
    pub wide_format: Option<bool>,
    #[serde(rename = "new", default, skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    #[serde(rename = "mailchimpTag", default, skip_serializing_if = "Option::is_none")]
    pub mailchimp_tag: Option<String>,
    #[serde(rename = "featured", default, skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(rename = "archived", default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "resolvedBy", default, skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<String>,
    #[serde(rename = "restricted", default, skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[serde(rename = "marketGroup", default, skip_serializing_if = "Option::is_none")]
    pub market_group: Option<serde_json::Number>,
    #[serde(rename = "groupItemTitle", default, skip_serializing_if = "Option::is_none")]
    pub group_item_title: Option<String>,
    #[serde(rename = "groupItemThreshold", default, skip_serializing_if = "Option::is_none")]
    pub group_item_threshold: Option<String>,
    #[serde(rename = "questionID", default, skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    #[serde(rename = "umaEndDate", default, skip_serializing_if = "Option::is_none")]
    pub uma_end_date: Option<String>,
    #[serde(rename = "enableOrderBook", default, skip_serializing_if = "Option::is_none")]
    pub enable_order_book: Option<bool>,
    #[serde(rename = "orderPriceMinTickSize", default, skip_serializing_if = "Option::is_none")]
    pub order_price_min_tick_size: Option<serde_json::Number>,
    #[serde(rename = "orderMinSize", default, skip_serializing_if = "Option::is_none")]
    pub order_min_size: Option<serde_json::Number>,
    #[serde(rename = "umaResolutionStatus", default, skip_serializing_if = "Option::is_none")]
    pub uma_resolution_status: Option<String>,
    #[serde(rename = "curationOrder", default, skip_serializing_if = "Option::is_none")]
    pub curation_order: Option<serde_json::Number>,
    #[serde(rename = "volumeNum", default, skip_serializing_if = "Option::is_none")]
    pub volume_num: Option<serde_json::Number>,
    #[serde(rename = "liquidityNum", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_num: Option<serde_json::Number>,
    #[serde(rename = "endDateIso", default, skip_serializing_if = "Option::is_none")]
    pub end_date_iso: Option<String>,
    #[serde(rename = "startDateIso", default, skip_serializing_if = "Option::is_none")]
    pub start_date_iso: Option<String>,
    #[serde(rename = "umaEndDateIso", default, skip_serializing_if = "Option::is_none")]
    pub uma_end_date_iso: Option<String>,
    #[serde(rename = "hasReviewedDates", default, skip_serializing_if = "Option::is_none")]
    pub has_reviewed_dates: Option<bool>,
    #[serde(rename = "readyForCron", default, skip_serializing_if = "Option::is_none")]
    pub ready_for_cron: Option<bool>,
    #[serde(rename = "commentsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub comments_enabled: Option<bool>,
    #[serde(rename = "volume24hr", default, skip_serializing_if = "Option::is_none")]
    pub volume24hr: Option<serde_json::Number>,
    #[serde(rename = "volume1wk", default, skip_serializing_if = "Option::is_none")]
    pub volume1wk: Option<serde_json::Number>,
    #[serde(rename = "volume1mo", default, skip_serializing_if = "Option::is_none")]
    pub volume1mo: Option<serde_json::Number>,
    #[serde(rename = "volume1yr", default, skip_serializing_if = "Option::is_none")]
    pub volume1yr: Option<serde_json::Number>,
    #[serde(rename = "gameStartTime", default, skip_serializing_if = "Option::is_none")]
    pub game_start_time: Option<String>,
    #[serde(rename = "secondsDelay", default, skip_serializing_if = "Option::is_none")]
    pub seconds_delay: Option<serde_json::Number>,
    #[serde(rename = "version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "clobTokenIds", default, skip_serializing_if = "Option::is_none")]
    pub clob_token_ids: Option<String>,
    #[serde(rename = "positionIds", default, skip_serializing_if = "Option::is_none")]
    pub position_ids: Option<Vec<String>>,
    #[serde(rename = "disqusThread", default, skip_serializing_if = "Option::is_none")]
    pub disqus_thread: Option<String>,
    #[serde(rename = "shortOutcomes", default, skip_serializing_if = "Option::is_none")]
    pub short_outcomes: Option<String>,
    #[serde(rename = "teamAID", default, skip_serializing_if = "Option::is_none")]
    pub team_aid: Option<String>,
    #[serde(rename = "teamBID", default, skip_serializing_if = "Option::is_none")]
    pub team_bid: Option<String>,
    #[serde(rename = "umaBond", default, skip_serializing_if = "Option::is_none")]
    pub uma_bond: Option<String>,
    #[serde(rename = "umaReward", default, skip_serializing_if = "Option::is_none")]
    pub uma_reward: Option<String>,
    #[serde(rename = "volume24hrClob", default, skip_serializing_if = "Option::is_none")]
    pub volume24hr_clob: Option<serde_json::Number>,
    #[serde(rename = "volume1wkClob", default, skip_serializing_if = "Option::is_none")]
    pub volume1wk_clob: Option<serde_json::Number>,
    #[serde(rename = "volume1moClob", default, skip_serializing_if = "Option::is_none")]
    pub volume1mo_clob: Option<serde_json::Number>,
    #[serde(rename = "volume1yrClob", default, skip_serializing_if = "Option::is_none")]
    pub volume1yr_clob: Option<serde_json::Number>,
    #[serde(rename = "volumeClob", default, skip_serializing_if = "Option::is_none")]
    pub volume_clob: Option<serde_json::Number>,
    #[serde(rename = "liquidityClob", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_clob: Option<serde_json::Number>,
    #[serde(rename = "makerBaseFee", default, skip_serializing_if = "Option::is_none")]
    pub maker_base_fee: Option<serde_json::Number>,
    #[serde(rename = "takerBaseFee", default, skip_serializing_if = "Option::is_none")]
    pub taker_base_fee: Option<serde_json::Number>,
    #[serde(rename = "customLiveness", default, skip_serializing_if = "Option::is_none")]
    pub custom_liveness: Option<serde_json::Number>,
    #[serde(rename = "acceptingOrders", default, skip_serializing_if = "Option::is_none")]
    pub accepting_orders: Option<bool>,
    #[serde(rename = "notificationsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
    #[serde(rename = "score", default, skip_serializing_if = "Option::is_none")]
    pub score: Option<serde_json::Number>,
    #[serde(rename = "imageOptimized", default, skip_serializing_if = "Option::is_none")]
    pub image_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "iconOptimized", default, skip_serializing_if = "Option::is_none")]
    pub icon_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GammaEvent>>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<GammaCategory>>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GammaTag>>,
    #[serde(rename = "creator", default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(rename = "ready", default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[serde(rename = "funded", default, skip_serializing_if = "Option::is_none")]
    pub funded: Option<bool>,
    #[serde(rename = "pastSlugs", default, skip_serializing_if = "Option::is_none")]
    pub past_slugs: Option<String>,
    #[serde(rename = "readyTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub ready_timestamp: Option<String>,
    #[serde(rename = "fundedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub funded_timestamp: Option<String>,
    #[serde(rename = "acceptingOrdersTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub accepting_orders_timestamp: Option<String>,
    #[serde(rename = "competitive", default, skip_serializing_if = "Option::is_none")]
    pub competitive: Option<serde_json::Number>,
    #[serde(rename = "rewardsMinSize", default, skip_serializing_if = "Option::is_none")]
    pub rewards_min_size: Option<serde_json::Number>,
    #[serde(rename = "rewardsMaxSpread", default, skip_serializing_if = "Option::is_none")]
    pub rewards_max_spread: Option<serde_json::Number>,
    #[serde(rename = "spread", default, skip_serializing_if = "Option::is_none")]
    pub spread: Option<serde_json::Number>,
    #[serde(rename = "automaticallyResolved", default, skip_serializing_if = "Option::is_none")]
    pub automatically_resolved: Option<bool>,
    #[serde(rename = "oneDayPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_day_price_change: Option<serde_json::Number>,
    #[serde(rename = "oneHourPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_hour_price_change: Option<serde_json::Number>,
    #[serde(rename = "oneWeekPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_week_price_change: Option<serde_json::Number>,
    #[serde(rename = "oneMonthPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_month_price_change: Option<serde_json::Number>,
    #[serde(rename = "oneYearPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_year_price_change: Option<serde_json::Number>,
    #[serde(rename = "lastTradePrice", default, skip_serializing_if = "Option::is_none")]
    pub last_trade_price: Option<serde_json::Number>,
    #[serde(rename = "bestBid", default, skip_serializing_if = "Option::is_none")]
    pub best_bid: Option<serde_json::Number>,
    #[serde(rename = "bestAsk", default, skip_serializing_if = "Option::is_none")]
    pub best_ask: Option<serde_json::Number>,
    #[serde(rename = "automaticallyActive", default, skip_serializing_if = "Option::is_none")]
    pub automatically_active: Option<bool>,
    #[serde(rename = "clearBookOnStart", default, skip_serializing_if = "Option::is_none")]
    pub clear_book_on_start: Option<bool>,
    #[serde(rename = "chartColor", default, skip_serializing_if = "Option::is_none")]
    pub chart_color: Option<String>,
    #[serde(rename = "seriesColor", default, skip_serializing_if = "Option::is_none")]
    pub series_color: Option<String>,
    #[serde(rename = "showGmpSeries", default, skip_serializing_if = "Option::is_none")]
    pub show_gmp_series: Option<bool>,
    #[serde(rename = "showGmpOutcome", default, skip_serializing_if = "Option::is_none")]
    pub show_gmp_outcome: Option<bool>,
    #[serde(rename = "manualActivation", default, skip_serializing_if = "Option::is_none")]
    pub manual_activation: Option<bool>,
    #[serde(rename = "negRiskOther", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk_other: Option<bool>,
    #[serde(rename = "gameId", default, skip_serializing_if = "Option::is_none")]
    pub game_id: Option<String>,
    #[serde(rename = "groupItemRange", default, skip_serializing_if = "Option::is_none")]
    pub group_item_range: Option<String>,
    #[serde(rename = "sportsMarketType", default, skip_serializing_if = "Option::is_none")]
    pub sports_market_type: Option<String>,
    #[serde(rename = "line", default, skip_serializing_if = "Option::is_none")]
    pub line: Option<serde_json::Number>,
    #[serde(rename = "umaResolutionStatuses", default, skip_serializing_if = "Option::is_none")]
    pub uma_resolution_statuses: Option<String>,
    #[serde(rename = "pendingDeployment", default, skip_serializing_if = "Option::is_none")]
    pub pending_deployment: Option<bool>,
    #[serde(rename = "deploying", default, skip_serializing_if = "Option::is_none")]
    pub deploying: Option<bool>,
    #[serde(rename = "deployingTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub deploying_timestamp: Option<String>,
    #[serde(rename = "scheduledDeploymentTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_deployment_timestamp: Option<String>,
    #[serde(rename = "rfqEnabled", default, skip_serializing_if = "Option::is_none")]
    pub rfq_enabled: Option<bool>,
    #[serde(rename = "eventStartTime", default, skip_serializing_if = "Option::is_none")]
    pub event_start_time: Option<String>,
    #[serde(rename = "feesEnabled", default, skip_serializing_if = "Option::is_none")]
    pub fees_enabled: Option<bool>,
    #[serde(rename = "feeSchedule", default, skip_serializing_if = "Option::is_none")]
    pub fee_schedule: Option<Box<GammaFeeSchedule>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetMarketTagsParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetMarketTagsParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetMarketsInformationParams {
    #[serde(rename = "body")]
    pub body: Box<GammaMarketsInformationBody>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetPublicProfileByUserAddressParams {
    #[serde(rename = "user_address")]
    pub user_address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetPublicProfileByUserAddressResponse {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<serde_json::Number>,
    #[serde(rename = "referral", default, skip_serializing_if = "Option::is_none")]
    pub referral: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<serde_json::Number>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<serde_json::Number>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "utmSource", default, skip_serializing_if = "Option::is_none")]
    pub utm_source: Option<String>,
    #[serde(rename = "utmMedium", default, skip_serializing_if = "Option::is_none")]
    pub utm_medium: Option<String>,
    #[serde(rename = "utmCampaign", default, skip_serializing_if = "Option::is_none")]
    pub utm_campaign: Option<String>,
    #[serde(rename = "utmContent", default, skip_serializing_if = "Option::is_none")]
    pub utm_content: Option<String>,
    #[serde(rename = "utmTerm", default, skip_serializing_if = "Option::is_none")]
    pub utm_term: Option<String>,
    #[serde(rename = "walletActivated", default, skip_serializing_if = "Option::is_none")]
    pub wallet_activated: Option<bool>,
    #[serde(rename = "pseudonym", default, skip_serializing_if = "Option::is_none")]
    pub pseudonym: Option<String>,
    #[serde(rename = "displayUsernamePublic", default, skip_serializing_if = "Option::is_none")]
    pub display_username_public: Option<bool>,
    #[serde(rename = "profileImage", default, skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>,
    #[serde(rename = "bio", default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(rename = "proxyWallet", default, skip_serializing_if = "Option::is_none")]
    pub proxy_wallet: Option<String>,
    #[serde(rename = "profileImageOptimized", default, skip_serializing_if = "Option::is_none")]
    pub profile_image_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "isCloseOnly", default, skip_serializing_if = "Option::is_none")]
    pub is_close_only: Option<bool>,
    #[serde(rename = "isCertReq", default, skip_serializing_if = "Option::is_none")]
    pub is_cert_req: Option<bool>,
    #[serde(rename = "certReqDate", default, skip_serializing_if = "Option::is_none")]
    pub cert_req_date: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetPublicProfileParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetPublicProfileResponse {
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "proxyWallet", default, skip_serializing_if = "Option::is_none")]
    pub proxy_wallet: Option<String>,
    #[serde(rename = "profileImage", default, skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>,
    #[serde(rename = "displayUsernamePublic", default, skip_serializing_if = "Option::is_none")]
    pub display_username_public: Option<bool>,
    #[serde(rename = "bio", default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(rename = "pseudonym", default, skip_serializing_if = "Option::is_none")]
    pub pseudonym: Option<String>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "users", default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<GammaPublicProfileUser>>,
    #[serde(rename = "xUsername", default, skip_serializing_if = "Option::is_none")]
    pub x_username: Option<String>,
    #[serde(rename = "verifiedBadge", default, skip_serializing_if = "Option::is_none")]
    pub verified_badge: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetRelatedTagsByIdParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(rename = "omit_empty", default, skip_serializing_if = "Option::is_none")]
    pub omit_empty: Option<bool>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetRelatedTagsByIdParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            omit_empty: Default::default(),
            status: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetRelatedTagsBySlugParams {
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "omit_empty", default, skip_serializing_if = "Option::is_none")]
    pub omit_empty: Option<bool>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetSeriesCommentsCountParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetSeriesCommentsCountParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetSeriesCommentsCountResponse {
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetSeriesParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(rename = "include_chat", default, skip_serializing_if = "Option::is_none")]
    pub include_chat: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetSeriesParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            include_chat: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetSeriesResponse {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "subtitle", default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "seriesType", default, skip_serializing_if = "Option::is_none")]
    pub series_type: Option<String>,
    #[serde(rename = "recurrence", default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "icon", default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "layout", default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    #[serde(rename = "active", default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "archived", default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "new", default, skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    #[serde(rename = "featured", default, skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(rename = "restricted", default, skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[serde(rename = "isTemplate", default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(rename = "templateVariables", default, skip_serializing_if = "Option::is_none")]
    pub template_variables: Option<bool>,
    #[serde(rename = "publishedAt", default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "commentsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub comments_enabled: Option<bool>,
    #[serde(rename = "competitive", default, skip_serializing_if = "Option::is_none")]
    pub competitive: Option<String>,
    #[serde(rename = "volume24hr", default, skip_serializing_if = "Option::is_none")]
    pub volume24hr: Option<serde_json::Number>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<serde_json::Number>,
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<serde_json::Number>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "pythTokenID", default, skip_serializing_if = "Option::is_none")]
    pub pyth_token_id: Option<String>,
    #[serde(rename = "cgAssetName", default, skip_serializing_if = "Option::is_none")]
    pub cg_asset_name: Option<String>,
    #[serde(rename = "score", default, skip_serializing_if = "Option::is_none")]
    pub score: Option<serde_json::Number>,
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GammaEvent>>,
    #[serde(rename = "collections", default, skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<GammaCollection>>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<GammaCategory>>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GammaTag>>,
    #[serde(rename = "commentCount", default, skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<serde_json::Number>,
    #[serde(rename = "chats", default, skip_serializing_if = "Option::is_none")]
    pub chats: Option<Vec<GammaChat>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetSeriesSummaryByIdParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetSeriesSummaryByIdParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetSeriesSummaryByIdResponse {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "eventDates", default, skip_serializing_if = "Option::is_none")]
    pub event_dates: Option<Vec<String>>,
    #[serde(rename = "eventWeeks", default, skip_serializing_if = "Option::is_none")]
    pub event_weeks: Option<Vec<serde_json::Number>>,
    #[serde(rename = "earliest_open_week", default, skip_serializing_if = "Option::is_none")]
    pub earliest_open_week: Option<serde_json::Number>,
    #[serde(rename = "earliest_open_date", default, skip_serializing_if = "Option::is_none")]
    pub earliest_open_date: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetSeriesSummaryBySlugParams {
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetSeriesSummaryBySlugResponse {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "eventDates", default, skip_serializing_if = "Option::is_none")]
    pub event_dates: Option<Vec<String>>,
    #[serde(rename = "eventWeeks", default, skip_serializing_if = "Option::is_none")]
    pub event_weeks: Option<Vec<serde_json::Number>>,
    #[serde(rename = "earliest_open_week", default, skip_serializing_if = "Option::is_none")]
    pub earliest_open_week: Option<serde_json::Number>,
    #[serde(rename = "earliest_open_date", default, skip_serializing_if = "Option::is_none")]
    pub earliest_open_date: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetSportsMarketTypesParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetSportsMarketTypesResponse {
    #[serde(rename = "marketTypes", default, skip_serializing_if = "Option::is_none")]
    pub market_types: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetSportsMetadataParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetTagBySlugParams {
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "include_template", default, skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetTagBySlugResponse {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "forceShow", default, skip_serializing_if = "Option::is_none")]
    pub force_show: Option<bool>,
    #[serde(rename = "publishedAt", default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<serde_json::Number>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<serde_json::Number>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "forceHide", default, skip_serializing_if = "Option::is_none")]
    pub force_hide: Option<bool>,
    #[serde(rename = "isCarousel", default, skip_serializing_if = "Option::is_none")]
    pub is_carousel: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetTagParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(rename = "include_template", default, skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetTagParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            include_template: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetTagResponse {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "forceShow", default, skip_serializing_if = "Option::is_none")]
    pub force_show: Option<bool>,
    #[serde(rename = "publishedAt", default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<serde_json::Number>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<serde_json::Number>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "forceHide", default, skip_serializing_if = "Option::is_none")]
    pub force_hide: Option<bool>,
    #[serde(rename = "isCarousel", default, skip_serializing_if = "Option::is_none")]
    pub is_carousel: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetTagsRelatedToATagByIdParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(rename = "omit_empty", default, skip_serializing_if = "Option::is_none")]
    pub omit_empty: Option<bool>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetTagsRelatedToATagByIdParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            omit_empty: Default::default(),
            status: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetTagsRelatedToATagBySlugParams {
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "omit_empty", default, skip_serializing_if = "Option::is_none")]
    pub omit_empty: Option<bool>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GammaGetTeamParams {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GammaGetTeamParams {
    fn default() -> Self {
        Self {
            id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaGetTeamResponse {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Number>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "league", default, skip_serializing_if = "Option::is_none")]
    pub league: Option<String>,
    #[serde(rename = "record", default, skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
    #[serde(rename = "logo", default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "abbreviation", default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[serde(rename = "alias", default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaImageOptimization {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "imageUrlSource", default, skip_serializing_if = "Option::is_none")]
    pub image_url_source: Option<String>,
    #[serde(rename = "imageUrlOptimized", default, skip_serializing_if = "Option::is_none")]
    pub image_url_optimized: Option<String>,
    #[serde(rename = "imageSizeKbSource", default, skip_serializing_if = "Option::is_none")]
    pub image_size_kb_source: Option<serde_json::Number>,
    #[serde(rename = "imageSizeKbOptimized", default, skip_serializing_if = "Option::is_none")]
    pub image_size_kb_optimized: Option<serde_json::Number>,
    #[serde(rename = "imageOptimizedComplete", default, skip_serializing_if = "Option::is_none")]
    pub image_optimized_complete: Option<bool>,
    #[serde(rename = "imageOptimizedLastUpdated", default, skip_serializing_if = "Option::is_none")]
    pub image_optimized_last_updated: Option<String>,
    #[serde(rename = "relID", default, skip_serializing_if = "Option::is_none")]
    pub rel_id: Option<serde_json::Number>,
    #[serde(rename = "field", default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "relname", default, skip_serializing_if = "Option::is_none")]
    pub relname: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaInternalError {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaKeysetEventsResponse {
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GammaEvent>>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaKeysetMarketsResponse {
    #[serde(rename = "markets", default, skip_serializing_if = "Option::is_none")]
    pub markets: Option<Vec<GammaMarket>>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaListCommentsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Number>,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "ascending", default, skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(rename = "parent_entity_type", default, skip_serializing_if = "Option::is_none")]
    pub parent_entity_type: Option<String>,
    #[serde(rename = "parent_entity_id", default, skip_serializing_if = "Option::is_none")]
    pub parent_entity_id: Option<serde_json::Number>,
    #[serde(rename = "get_positions", default, skip_serializing_if = "Option::is_none")]
    pub get_positions: Option<bool>,
    #[serde(rename = "holders_only", default, skip_serializing_if = "Option::is_none")]
    pub holders_only: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaListEventCreatorsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Number>,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "ascending", default, skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(rename = "creator_name", default, skip_serializing_if = "Option::is_none")]
    pub creator_name: Option<String>,
    #[serde(rename = "creator_handle", default, skip_serializing_if = "Option::is_none")]
    pub creator_handle: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaListEventsKeysetParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "ascending", default, skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(rename = "after_cursor", default, skip_serializing_if = "Option::is_none")]
    pub after_cursor: Option<String>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Number>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<serde_json::Number>>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<Vec<String>>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<serde_json::Value>,
    #[serde(rename = "live", default, skip_serializing_if = "Option::is_none")]
    pub live: Option<bool>,
    #[serde(rename = "featured", default, skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(rename = "cyom", default, skip_serializing_if = "Option::is_none")]
    pub cyom: Option<bool>,
    #[serde(rename = "title_search", default, skip_serializing_if = "Option::is_none")]
    pub title_search: Option<String>,
    #[serde(rename = "liquidity_min", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_min: Option<serde_json::Number>,
    #[serde(rename = "liquidity_max", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_max: Option<serde_json::Number>,
    #[serde(rename = "volume_min", default, skip_serializing_if = "Option::is_none")]
    pub volume_min: Option<serde_json::Number>,
    #[serde(rename = "volume_max", default, skip_serializing_if = "Option::is_none")]
    pub volume_max: Option<serde_json::Number>,
    #[serde(rename = "start_date_min", default, skip_serializing_if = "Option::is_none")]
    pub start_date_min: Option<String>,
    #[serde(rename = "start_date_max", default, skip_serializing_if = "Option::is_none")]
    pub start_date_max: Option<String>,
    #[serde(rename = "end_date_min", default, skip_serializing_if = "Option::is_none")]
    pub end_date_min: Option<String>,
    #[serde(rename = "end_date_max", default, skip_serializing_if = "Option::is_none")]
    pub end_date_max: Option<String>,
    #[serde(rename = "start_time_min", default, skip_serializing_if = "Option::is_none")]
    pub start_time_min: Option<String>,
    #[serde(rename = "start_time_max", default, skip_serializing_if = "Option::is_none")]
    pub start_time_max: Option<String>,
    #[serde(rename = "tag_id", default, skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<serde_json::Number>>,
    #[serde(rename = "tag_slug", default, skip_serializing_if = "Option::is_none")]
    pub tag_slug: Option<String>,
    #[serde(rename = "exclude_tag_id", default, skip_serializing_if = "Option::is_none")]
    pub exclude_tag_id: Option<Vec<serde_json::Number>>,
    #[serde(rename = "related_tags", default, skip_serializing_if = "Option::is_none")]
    pub related_tags: Option<bool>,
    #[serde(rename = "tag_match", default, skip_serializing_if = "Option::is_none")]
    pub tag_match: Option<String>,
    #[serde(rename = "series_id", default, skip_serializing_if = "Option::is_none")]
    pub series_id: Option<Vec<serde_json::Number>>,
    #[serde(rename = "game_id", default, skip_serializing_if = "Option::is_none")]
    pub game_id: Option<Vec<serde_json::Number>>,
    #[serde(rename = "event_date", default, skip_serializing_if = "Option::is_none")]
    pub event_date: Option<String>,
    #[serde(rename = "event_week", default, skip_serializing_if = "Option::is_none")]
    pub event_week: Option<serde_json::Number>,
    #[serde(rename = "featured_order", default, skip_serializing_if = "Option::is_none")]
    pub featured_order: Option<bool>,
    #[serde(rename = "recurrence", default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(rename = "created_by", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Vec<String>>,
    #[serde(rename = "parent_event_id", default, skip_serializing_if = "Option::is_none")]
    pub parent_event_id: Option<serde_json::Number>,
    #[serde(rename = "include_children", default, skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
    #[serde(rename = "partner_slug", default, skip_serializing_if = "Option::is_none")]
    pub partner_slug: Option<String>,
    #[serde(rename = "include_chat", default, skip_serializing_if = "Option::is_none")]
    pub include_chat: Option<bool>,
    #[serde(rename = "include_template", default, skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,
    #[serde(rename = "include_best_lines", default, skip_serializing_if = "Option::is_none")]
    pub include_best_lines: Option<bool>,
    #[serde(rename = "locale", default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaListEventsKeysetResponse {
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GammaEvent>>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaListEventsPaginationParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Number>,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "ascending", default, skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(rename = "include_chat", default, skip_serializing_if = "Option::is_none")]
    pub include_chat: Option<bool>,
    #[serde(rename = "include_template", default, skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,
    #[serde(rename = "recurrence", default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaListEventsPaginationResponse {
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GammaEvent>>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<GammaPagination>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaListEventsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Number>,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "ascending", default, skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<serde_json::Number>>,
    #[serde(rename = "tag_id", default, skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<serde_json::Number>,
    #[serde(rename = "exclude_tag_id", default, skip_serializing_if = "Option::is_none")]
    pub exclude_tag_id: Option<Vec<serde_json::Number>>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<Vec<String>>,
    #[serde(rename = "tag_slug", default, skip_serializing_if = "Option::is_none")]
    pub tag_slug: Option<String>,
    #[serde(rename = "related_tags", default, skip_serializing_if = "Option::is_none")]
    pub related_tags: Option<bool>,
    #[serde(rename = "active", default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "archived", default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "featured", default, skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(rename = "cyom", default, skip_serializing_if = "Option::is_none")]
    pub cyom: Option<bool>,
    #[serde(rename = "include_chat", default, skip_serializing_if = "Option::is_none")]
    pub include_chat: Option<bool>,
    #[serde(rename = "include_template", default, skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,
    #[serde(rename = "recurrence", default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<serde_json::Value>,
    #[serde(rename = "liquidity_min", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_min: Option<serde_json::Number>,
    #[serde(rename = "liquidity_max", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_max: Option<serde_json::Number>,
    #[serde(rename = "volume_min", default, skip_serializing_if = "Option::is_none")]
    pub volume_min: Option<serde_json::Number>,
    #[serde(rename = "volume_max", default, skip_serializing_if = "Option::is_none")]
    pub volume_max: Option<serde_json::Number>,
    #[serde(rename = "start_date_min", default, skip_serializing_if = "Option::is_none")]
    pub start_date_min: Option<String>,
    #[serde(rename = "start_date_max", default, skip_serializing_if = "Option::is_none")]
    pub start_date_max: Option<String>,
    #[serde(rename = "end_date_min", default, skip_serializing_if = "Option::is_none")]
    pub end_date_min: Option<String>,
    #[serde(rename = "end_date_max", default, skip_serializing_if = "Option::is_none")]
    pub end_date_max: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaListMarketsKeysetParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "ascending", default, skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(rename = "after_cursor", default, skip_serializing_if = "Option::is_none")]
    pub after_cursor: Option<String>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Number>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<serde_json::Number>>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<Vec<String>>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<serde_json::Value>,
    #[serde(rename = "decimalized", default, skip_serializing_if = "Option::is_none")]
    pub decimalized: Option<bool>,
    #[serde(rename = "clob_token_ids", default, skip_serializing_if = "Option::is_none")]
    pub clob_token_ids: Option<Vec<String>>,
    #[serde(rename = "condition_ids", default, skip_serializing_if = "Option::is_none")]
    pub condition_ids: Option<Vec<String>>,
    #[serde(rename = "question_ids", default, skip_serializing_if = "Option::is_none")]
    pub question_ids: Option<Vec<String>>,
    #[serde(rename = "liquidity_num_min", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_num_min: Option<serde_json::Number>,
    #[serde(rename = "liquidity_num_max", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_num_max: Option<serde_json::Number>,
    #[serde(rename = "volume_num_min", default, skip_serializing_if = "Option::is_none")]
    pub volume_num_min: Option<serde_json::Number>,
    #[serde(rename = "volume_num_max", default, skip_serializing_if = "Option::is_none")]
    pub volume_num_max: Option<serde_json::Number>,
    #[serde(rename = "start_date_min", default, skip_serializing_if = "Option::is_none")]
    pub start_date_min: Option<String>,
    #[serde(rename = "start_date_max", default, skip_serializing_if = "Option::is_none")]
    pub start_date_max: Option<String>,
    #[serde(rename = "end_date_min", default, skip_serializing_if = "Option::is_none")]
    pub end_date_min: Option<String>,
    #[serde(rename = "end_date_max", default, skip_serializing_if = "Option::is_none")]
    pub end_date_max: Option<String>,
    #[serde(rename = "tag_id", default, skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<serde_json::Number>>,
    #[serde(rename = "related_tags", default, skip_serializing_if = "Option::is_none")]
    pub related_tags: Option<bool>,
    #[serde(rename = "tag_match", default, skip_serializing_if = "Option::is_none")]
    pub tag_match: Option<String>,
    #[serde(rename = "cyom", default, skip_serializing_if = "Option::is_none")]
    pub cyom: Option<bool>,
    #[serde(rename = "rfq_enabled", default, skip_serializing_if = "Option::is_none")]
    pub rfq_enabled: Option<bool>,
    #[serde(rename = "uma_resolution_status", default, skip_serializing_if = "Option::is_none")]
    pub uma_resolution_status: Option<String>,
    #[serde(rename = "game_id", default, skip_serializing_if = "Option::is_none")]
    pub game_id: Option<String>,
    #[serde(rename = "sports_market_types", default, skip_serializing_if = "Option::is_none")]
    pub sports_market_types: Option<Vec<String>>,
    #[serde(rename = "include_tag", default, skip_serializing_if = "Option::is_none")]
    pub include_tag: Option<bool>,
    #[serde(rename = "locale", default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaListMarketsKeysetResponse {
    #[serde(rename = "markets", default, skip_serializing_if = "Option::is_none")]
    pub markets: Option<Vec<GammaMarket>>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaListMarketsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Number>,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "ascending", default, skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<serde_json::Number>>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<Vec<String>>,
    #[serde(rename = "clob_token_ids", default, skip_serializing_if = "Option::is_none")]
    pub clob_token_ids: Option<Vec<String>>,
    #[serde(rename = "condition_ids", default, skip_serializing_if = "Option::is_none")]
    pub condition_ids: Option<Vec<String>>,
    #[serde(rename = "liquidity_num_min", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_num_min: Option<serde_json::Number>,
    #[serde(rename = "liquidity_num_max", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_num_max: Option<serde_json::Number>,
    #[serde(rename = "volume_num_min", default, skip_serializing_if = "Option::is_none")]
    pub volume_num_min: Option<serde_json::Number>,
    #[serde(rename = "volume_num_max", default, skip_serializing_if = "Option::is_none")]
    pub volume_num_max: Option<serde_json::Number>,
    #[serde(rename = "start_date_min", default, skip_serializing_if = "Option::is_none")]
    pub start_date_min: Option<String>,
    #[serde(rename = "start_date_max", default, skip_serializing_if = "Option::is_none")]
    pub start_date_max: Option<String>,
    #[serde(rename = "end_date_min", default, skip_serializing_if = "Option::is_none")]
    pub end_date_min: Option<String>,
    #[serde(rename = "end_date_max", default, skip_serializing_if = "Option::is_none")]
    pub end_date_max: Option<String>,
    #[serde(rename = "tag_id", default, skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<serde_json::Number>,
    #[serde(rename = "related_tags", default, skip_serializing_if = "Option::is_none")]
    pub related_tags: Option<bool>,
    #[serde(rename = "cyom", default, skip_serializing_if = "Option::is_none")]
    pub cyom: Option<bool>,
    #[serde(rename = "uma_resolution_status", default, skip_serializing_if = "Option::is_none")]
    pub uma_resolution_status: Option<String>,
    #[serde(rename = "game_id", default, skip_serializing_if = "Option::is_none")]
    pub game_id: Option<String>,
    #[serde(rename = "sports_market_types", default, skip_serializing_if = "Option::is_none")]
    pub sports_market_types: Option<Vec<String>>,
    #[serde(rename = "rewards_min_size", default, skip_serializing_if = "Option::is_none")]
    pub rewards_min_size: Option<serde_json::Number>,
    #[serde(rename = "question_ids", default, skip_serializing_if = "Option::is_none")]
    pub question_ids: Option<Vec<String>>,
    #[serde(rename = "include_tag", default, skip_serializing_if = "Option::is_none")]
    pub include_tag: Option<bool>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaListSeriesParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Number>,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "ascending", default, skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<Vec<String>>,
    #[serde(rename = "categories_ids", default, skip_serializing_if = "Option::is_none")]
    pub categories_ids: Option<Vec<serde_json::Number>>,
    #[serde(rename = "categories_labels", default, skip_serializing_if = "Option::is_none")]
    pub categories_labels: Option<Vec<String>>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "include_chat", default, skip_serializing_if = "Option::is_none")]
    pub include_chat: Option<bool>,
    #[serde(rename = "recurrence", default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(rename = "exclude_events", default, skip_serializing_if = "Option::is_none")]
    pub exclude_events: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaListSportEventsResultsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Number>,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "ascending", default, skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaListTagsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Number>,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "ascending", default, skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(rename = "include_template", default, skip_serializing_if = "Option::is_none")]
    pub include_template: Option<bool>,
    #[serde(rename = "is_carousel", default, skip_serializing_if = "Option::is_none")]
    pub is_carousel: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaListTeamsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Number>,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "ascending", default, skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(rename = "league", default, skip_serializing_if = "Option::is_none")]
    pub league: Option<Vec<String>>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<String>>,
    #[serde(rename = "abbreviation", default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaMarket {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "question", default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(rename = "conditionId", default, skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "twitterCardImage", default, skip_serializing_if = "Option::is_none")]
    pub twitter_card_image: Option<String>,
    #[serde(rename = "resolutionSource", default, skip_serializing_if = "Option::is_none")]
    pub resolution_source: Option<String>,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    #[serde(rename = "sponsorName", default, skip_serializing_if = "Option::is_none")]
    pub sponsor_name: Option<String>,
    #[serde(rename = "sponsorImage", default, skip_serializing_if = "Option::is_none")]
    pub sponsor_image: Option<String>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "xAxisValue", default, skip_serializing_if = "Option::is_none")]
    pub x_axis_value: Option<String>,
    #[serde(rename = "yAxisValue", default, skip_serializing_if = "Option::is_none")]
    pub y_axis_value: Option<String>,
    #[serde(rename = "denominationToken", default, skip_serializing_if = "Option::is_none")]
    pub denomination_token: Option<String>,
    #[serde(rename = "fee", default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "icon", default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "lowerBound", default, skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<String>,
    #[serde(rename = "upperBound", default, skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "outcomes", default, skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<String>,
    #[serde(rename = "outcomePrices", default, skip_serializing_if = "Option::is_none")]
    pub outcome_prices: Option<String>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(rename = "active", default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "marketType", default, skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
    #[serde(rename = "formatType", default, skip_serializing_if = "Option::is_none")]
    pub format_type: Option<String>,
    #[serde(rename = "lowerBoundDate", default, skip_serializing_if = "Option::is_none")]
    pub lower_bound_date: Option<String>,
    #[serde(rename = "upperBoundDate", default, skip_serializing_if = "Option::is_none")]
    pub upper_bound_date: Option<String>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<serde_json::Number>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<serde_json::Number>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "closedTime", default, skip_serializing_if = "Option::is_none")]
    pub closed_time: Option<String>,
    #[serde(rename = "wideFormat", default, skip_serializing_if = "Option::is_none")]
    pub wide_format: Option<bool>,
    #[serde(rename = "new", default, skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    #[serde(rename = "mailchimpTag", default, skip_serializing_if = "Option::is_none")]
    pub mailchimp_tag: Option<String>,
    #[serde(rename = "featured", default, skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(rename = "archived", default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "resolvedBy", default, skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<String>,
    #[serde(rename = "restricted", default, skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[serde(rename = "marketGroup", default, skip_serializing_if = "Option::is_none")]
    pub market_group: Option<serde_json::Number>,
    #[serde(rename = "groupItemTitle", default, skip_serializing_if = "Option::is_none")]
    pub group_item_title: Option<String>,
    #[serde(rename = "groupItemThreshold", default, skip_serializing_if = "Option::is_none")]
    pub group_item_threshold: Option<String>,
    #[serde(rename = "questionID", default, skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    #[serde(rename = "umaEndDate", default, skip_serializing_if = "Option::is_none")]
    pub uma_end_date: Option<String>,
    #[serde(rename = "enableOrderBook", default, skip_serializing_if = "Option::is_none")]
    pub enable_order_book: Option<bool>,
    #[serde(rename = "orderPriceMinTickSize", default, skip_serializing_if = "Option::is_none")]
    pub order_price_min_tick_size: Option<serde_json::Number>,
    #[serde(rename = "orderMinSize", default, skip_serializing_if = "Option::is_none")]
    pub order_min_size: Option<serde_json::Number>,
    #[serde(rename = "umaResolutionStatus", default, skip_serializing_if = "Option::is_none")]
    pub uma_resolution_status: Option<String>,
    #[serde(rename = "curationOrder", default, skip_serializing_if = "Option::is_none")]
    pub curation_order: Option<serde_json::Number>,
    #[serde(rename = "volumeNum", default, skip_serializing_if = "Option::is_none")]
    pub volume_num: Option<serde_json::Number>,
    #[serde(rename = "liquidityNum", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_num: Option<serde_json::Number>,
    #[serde(rename = "endDateIso", default, skip_serializing_if = "Option::is_none")]
    pub end_date_iso: Option<String>,
    #[serde(rename = "startDateIso", default, skip_serializing_if = "Option::is_none")]
    pub start_date_iso: Option<String>,
    #[serde(rename = "umaEndDateIso", default, skip_serializing_if = "Option::is_none")]
    pub uma_end_date_iso: Option<String>,
    #[serde(rename = "hasReviewedDates", default, skip_serializing_if = "Option::is_none")]
    pub has_reviewed_dates: Option<bool>,
    #[serde(rename = "readyForCron", default, skip_serializing_if = "Option::is_none")]
    pub ready_for_cron: Option<bool>,
    #[serde(rename = "commentsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub comments_enabled: Option<bool>,
    #[serde(rename = "volume24hr", default, skip_serializing_if = "Option::is_none")]
    pub volume24hr: Option<serde_json::Number>,
    #[serde(rename = "volume1wk", default, skip_serializing_if = "Option::is_none")]
    pub volume1wk: Option<serde_json::Number>,
    #[serde(rename = "volume1mo", default, skip_serializing_if = "Option::is_none")]
    pub volume1mo: Option<serde_json::Number>,
    #[serde(rename = "volume1yr", default, skip_serializing_if = "Option::is_none")]
    pub volume1yr: Option<serde_json::Number>,
    #[serde(rename = "gameStartTime", default, skip_serializing_if = "Option::is_none")]
    pub game_start_time: Option<String>,
    #[serde(rename = "secondsDelay", default, skip_serializing_if = "Option::is_none")]
    pub seconds_delay: Option<serde_json::Number>,
    #[serde(rename = "version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "clobTokenIds", default, skip_serializing_if = "Option::is_none")]
    pub clob_token_ids: Option<String>,
    #[serde(rename = "positionIds", default, skip_serializing_if = "Option::is_none")]
    pub position_ids: Option<Vec<String>>,
    #[serde(rename = "disqusThread", default, skip_serializing_if = "Option::is_none")]
    pub disqus_thread: Option<String>,
    #[serde(rename = "shortOutcomes", default, skip_serializing_if = "Option::is_none")]
    pub short_outcomes: Option<String>,
    #[serde(rename = "teamAID", default, skip_serializing_if = "Option::is_none")]
    pub team_aid: Option<String>,
    #[serde(rename = "teamBID", default, skip_serializing_if = "Option::is_none")]
    pub team_bid: Option<String>,
    #[serde(rename = "umaBond", default, skip_serializing_if = "Option::is_none")]
    pub uma_bond: Option<String>,
    #[serde(rename = "umaReward", default, skip_serializing_if = "Option::is_none")]
    pub uma_reward: Option<String>,
    #[serde(rename = "volume24hrClob", default, skip_serializing_if = "Option::is_none")]
    pub volume24hr_clob: Option<serde_json::Number>,
    #[serde(rename = "volume1wkClob", default, skip_serializing_if = "Option::is_none")]
    pub volume1wk_clob: Option<serde_json::Number>,
    #[serde(rename = "volume1moClob", default, skip_serializing_if = "Option::is_none")]
    pub volume1mo_clob: Option<serde_json::Number>,
    #[serde(rename = "volume1yrClob", default, skip_serializing_if = "Option::is_none")]
    pub volume1yr_clob: Option<serde_json::Number>,
    #[serde(rename = "volumeClob", default, skip_serializing_if = "Option::is_none")]
    pub volume_clob: Option<serde_json::Number>,
    #[serde(rename = "liquidityClob", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_clob: Option<serde_json::Number>,
    #[serde(rename = "makerBaseFee", default, skip_serializing_if = "Option::is_none")]
    pub maker_base_fee: Option<serde_json::Number>,
    #[serde(rename = "takerBaseFee", default, skip_serializing_if = "Option::is_none")]
    pub taker_base_fee: Option<serde_json::Number>,
    #[serde(rename = "customLiveness", default, skip_serializing_if = "Option::is_none")]
    pub custom_liveness: Option<serde_json::Number>,
    #[serde(rename = "acceptingOrders", default, skip_serializing_if = "Option::is_none")]
    pub accepting_orders: Option<bool>,
    #[serde(rename = "notificationsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
    #[serde(rename = "score", default, skip_serializing_if = "Option::is_none")]
    pub score: Option<serde_json::Number>,
    #[serde(rename = "imageOptimized", default, skip_serializing_if = "Option::is_none")]
    pub image_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "iconOptimized", default, skip_serializing_if = "Option::is_none")]
    pub icon_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GammaEvent>>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<GammaCategory>>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GammaTag>>,
    #[serde(rename = "creator", default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(rename = "ready", default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[serde(rename = "funded", default, skip_serializing_if = "Option::is_none")]
    pub funded: Option<bool>,
    #[serde(rename = "pastSlugs", default, skip_serializing_if = "Option::is_none")]
    pub past_slugs: Option<String>,
    #[serde(rename = "readyTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub ready_timestamp: Option<String>,
    #[serde(rename = "fundedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub funded_timestamp: Option<String>,
    #[serde(rename = "acceptingOrdersTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub accepting_orders_timestamp: Option<String>,
    #[serde(rename = "competitive", default, skip_serializing_if = "Option::is_none")]
    pub competitive: Option<serde_json::Number>,
    #[serde(rename = "rewardsMinSize", default, skip_serializing_if = "Option::is_none")]
    pub rewards_min_size: Option<serde_json::Number>,
    #[serde(rename = "rewardsMaxSpread", default, skip_serializing_if = "Option::is_none")]
    pub rewards_max_spread: Option<serde_json::Number>,
    #[serde(rename = "spread", default, skip_serializing_if = "Option::is_none")]
    pub spread: Option<serde_json::Number>,
    #[serde(rename = "automaticallyResolved", default, skip_serializing_if = "Option::is_none")]
    pub automatically_resolved: Option<bool>,
    #[serde(rename = "oneDayPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_day_price_change: Option<serde_json::Number>,
    #[serde(rename = "oneHourPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_hour_price_change: Option<serde_json::Number>,
    #[serde(rename = "oneWeekPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_week_price_change: Option<serde_json::Number>,
    #[serde(rename = "oneMonthPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_month_price_change: Option<serde_json::Number>,
    #[serde(rename = "oneYearPriceChange", default, skip_serializing_if = "Option::is_none")]
    pub one_year_price_change: Option<serde_json::Number>,
    #[serde(rename = "lastTradePrice", default, skip_serializing_if = "Option::is_none")]
    pub last_trade_price: Option<serde_json::Number>,
    #[serde(rename = "bestBid", default, skip_serializing_if = "Option::is_none")]
    pub best_bid: Option<serde_json::Number>,
    #[serde(rename = "bestAsk", default, skip_serializing_if = "Option::is_none")]
    pub best_ask: Option<serde_json::Number>,
    #[serde(rename = "automaticallyActive", default, skip_serializing_if = "Option::is_none")]
    pub automatically_active: Option<bool>,
    #[serde(rename = "clearBookOnStart", default, skip_serializing_if = "Option::is_none")]
    pub clear_book_on_start: Option<bool>,
    #[serde(rename = "chartColor", default, skip_serializing_if = "Option::is_none")]
    pub chart_color: Option<String>,
    #[serde(rename = "seriesColor", default, skip_serializing_if = "Option::is_none")]
    pub series_color: Option<String>,
    #[serde(rename = "showGmpSeries", default, skip_serializing_if = "Option::is_none")]
    pub show_gmp_series: Option<bool>,
    #[serde(rename = "showGmpOutcome", default, skip_serializing_if = "Option::is_none")]
    pub show_gmp_outcome: Option<bool>,
    #[serde(rename = "manualActivation", default, skip_serializing_if = "Option::is_none")]
    pub manual_activation: Option<bool>,
    #[serde(rename = "negRiskOther", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk_other: Option<bool>,
    #[serde(rename = "gameId", default, skip_serializing_if = "Option::is_none")]
    pub game_id: Option<String>,
    #[serde(rename = "groupItemRange", default, skip_serializing_if = "Option::is_none")]
    pub group_item_range: Option<String>,
    #[serde(rename = "sportsMarketType", default, skip_serializing_if = "Option::is_none")]
    pub sports_market_type: Option<String>,
    #[serde(rename = "line", default, skip_serializing_if = "Option::is_none")]
    pub line: Option<serde_json::Number>,
    #[serde(rename = "umaResolutionStatuses", default, skip_serializing_if = "Option::is_none")]
    pub uma_resolution_statuses: Option<String>,
    #[serde(rename = "pendingDeployment", default, skip_serializing_if = "Option::is_none")]
    pub pending_deployment: Option<bool>,
    #[serde(rename = "deploying", default, skip_serializing_if = "Option::is_none")]
    pub deploying: Option<bool>,
    #[serde(rename = "deployingTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub deploying_timestamp: Option<String>,
    #[serde(rename = "scheduledDeploymentTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_deployment_timestamp: Option<String>,
    #[serde(rename = "rfqEnabled", default, skip_serializing_if = "Option::is_none")]
    pub rfq_enabled: Option<bool>,
    #[serde(rename = "eventStartTime", default, skip_serializing_if = "Option::is_none")]
    pub event_start_time: Option<String>,
    #[serde(rename = "feesEnabled", default, skip_serializing_if = "Option::is_none")]
    pub fees_enabled: Option<bool>,
    #[serde(rename = "feeSchedule", default, skip_serializing_if = "Option::is_none")]
    pub fee_schedule: Option<Box<GammaFeeSchedule>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaMarketDescription {
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaMarketsInformationBody {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<serde_json::Number>>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<Vec<String>>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "clobTokenIds", default, skip_serializing_if = "Option::is_none")]
    pub clob_token_ids: Option<Vec<String>>,
    #[serde(rename = "conditionIds", default, skip_serializing_if = "Option::is_none")]
    pub condition_ids: Option<Vec<String>>,
    #[serde(rename = "liquidityNumMin", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_num_min: Option<serde_json::Number>,
    #[serde(rename = "liquidityNumMax", default, skip_serializing_if = "Option::is_none")]
    pub liquidity_num_max: Option<serde_json::Number>,
    #[serde(rename = "volumeNumMin", default, skip_serializing_if = "Option::is_none")]
    pub volume_num_min: Option<serde_json::Number>,
    #[serde(rename = "volumeNumMax", default, skip_serializing_if = "Option::is_none")]
    pub volume_num_max: Option<serde_json::Number>,
    #[serde(rename = "startDateMin", default, skip_serializing_if = "Option::is_none")]
    pub start_date_min: Option<String>,
    #[serde(rename = "startDateMax", default, skip_serializing_if = "Option::is_none")]
    pub start_date_max: Option<String>,
    #[serde(rename = "endDateMin", default, skip_serializing_if = "Option::is_none")]
    pub end_date_min: Option<String>,
    #[serde(rename = "endDateMax", default, skip_serializing_if = "Option::is_none")]
    pub end_date_max: Option<String>,
    #[serde(rename = "relatedTags", default, skip_serializing_if = "Option::is_none")]
    pub related_tags: Option<bool>,
    #[serde(rename = "tagId", default, skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<serde_json::Number>,
    #[serde(rename = "cyom", default, skip_serializing_if = "Option::is_none")]
    pub cyom: Option<bool>,
    #[serde(rename = "umaResolutionStatus", default, skip_serializing_if = "Option::is_none")]
    pub uma_resolution_status: Option<String>,
    #[serde(rename = "gameId", default, skip_serializing_if = "Option::is_none")]
    pub game_id: Option<String>,
    #[serde(rename = "sportsMarketTypes", default, skip_serializing_if = "Option::is_none")]
    pub sports_market_types: Option<Vec<String>>,
    #[serde(rename = "rewardsMinSize", default, skip_serializing_if = "Option::is_none")]
    pub rewards_min_size: Option<serde_json::Number>,
    #[serde(rename = "questionIds", default, skip_serializing_if = "Option::is_none")]
    pub question_ids: Option<Vec<String>>,
    #[serde(rename = "includeTags", default, skip_serializing_if = "Option::is_none")]
    pub include_tags: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaPagination {
    #[serde(rename = "hasMore", default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(rename = "totalResults", default, skip_serializing_if = "Option::is_none")]
    pub total_results: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaProfile {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<serde_json::Number>,
    #[serde(rename = "referral", default, skip_serializing_if = "Option::is_none")]
    pub referral: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<serde_json::Number>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<serde_json::Number>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "utmSource", default, skip_serializing_if = "Option::is_none")]
    pub utm_source: Option<String>,
    #[serde(rename = "utmMedium", default, skip_serializing_if = "Option::is_none")]
    pub utm_medium: Option<String>,
    #[serde(rename = "utmCampaign", default, skip_serializing_if = "Option::is_none")]
    pub utm_campaign: Option<String>,
    #[serde(rename = "utmContent", default, skip_serializing_if = "Option::is_none")]
    pub utm_content: Option<String>,
    #[serde(rename = "utmTerm", default, skip_serializing_if = "Option::is_none")]
    pub utm_term: Option<String>,
    #[serde(rename = "walletActivated", default, skip_serializing_if = "Option::is_none")]
    pub wallet_activated: Option<bool>,
    #[serde(rename = "pseudonym", default, skip_serializing_if = "Option::is_none")]
    pub pseudonym: Option<String>,
    #[serde(rename = "displayUsernamePublic", default, skip_serializing_if = "Option::is_none")]
    pub display_username_public: Option<bool>,
    #[serde(rename = "profileImage", default, skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>,
    #[serde(rename = "bio", default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(rename = "proxyWallet", default, skip_serializing_if = "Option::is_none")]
    pub proxy_wallet: Option<String>,
    #[serde(rename = "profileImageOptimized", default, skip_serializing_if = "Option::is_none")]
    pub profile_image_optimized: Option<Box<GammaImageOptimization>>,
    #[serde(rename = "isCloseOnly", default, skip_serializing_if = "Option::is_none")]
    pub is_close_only: Option<bool>,
    #[serde(rename = "isCertReq", default, skip_serializing_if = "Option::is_none")]
    pub is_cert_req: Option<bool>,
    #[serde(rename = "certReqDate", default, skip_serializing_if = "Option::is_none")]
    pub cert_req_date: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaPublicProfileError {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaPublicProfileResponse {
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "proxyWallet", default, skip_serializing_if = "Option::is_none")]
    pub proxy_wallet: Option<String>,
    #[serde(rename = "profileImage", default, skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>,
    #[serde(rename = "displayUsernamePublic", default, skip_serializing_if = "Option::is_none")]
    pub display_username_public: Option<bool>,
    #[serde(rename = "bio", default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(rename = "pseudonym", default, skip_serializing_if = "Option::is_none")]
    pub pseudonym: Option<String>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "users", default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<GammaPublicProfileUser>>,
    #[serde(rename = "xUsername", default, skip_serializing_if = "Option::is_none")]
    pub x_username: Option<String>,
    #[serde(rename = "verifiedBadge", default, skip_serializing_if = "Option::is_none")]
    pub verified_badge: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaPublicProfileUser {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "creator", default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<bool>,
    #[serde(rename = "mod", default, skip_serializing_if = "Option::is_none")]
    pub mod_: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaPublicSearchParams {
    #[serde(rename = "q")]
    pub q: String,
    #[serde(rename = "cache", default, skip_serializing_if = "Option::is_none")]
    pub cache: Option<bool>,
    #[serde(rename = "events_status", default, skip_serializing_if = "Option::is_none")]
    pub events_status: Option<String>,
    #[serde(rename = "limit_per_type", default, skip_serializing_if = "Option::is_none")]
    pub limit_per_type: Option<serde_json::Number>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Number>,
    #[serde(rename = "events_tag", default, skip_serializing_if = "Option::is_none")]
    pub events_tag: Option<Vec<String>>,
    #[serde(rename = "keep_closed_markets", default, skip_serializing_if = "Option::is_none")]
    pub keep_closed_markets: Option<serde_json::Number>,
    #[serde(rename = "sort", default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(rename = "ascending", default, skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(rename = "search_tags", default, skip_serializing_if = "Option::is_none")]
    pub search_tags: Option<bool>,
    #[serde(rename = "search_profiles", default, skip_serializing_if = "Option::is_none")]
    pub search_profiles: Option<bool>,
    #[serde(rename = "recurrence", default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(rename = "exclude_tag_id", default, skip_serializing_if = "Option::is_none")]
    pub exclude_tag_id: Option<Vec<serde_json::Number>>,
    #[serde(rename = "optimized", default, skip_serializing_if = "Option::is_none")]
    pub optimized: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaPublicSearchResponse {
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GammaEvent>>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GammaSearchTag>>,
    #[serde(rename = "profiles", default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<GammaProfile>>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<GammaPagination>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaReaction {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "commentID", default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<serde_json::Number>,
    #[serde(rename = "reactionType", default, skip_serializing_if = "Option::is_none")]
    pub reaction_type: Option<String>,
    #[serde(rename = "icon", default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "userAddress", default, skip_serializing_if = "Option::is_none")]
    pub user_address: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "profile", default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<Box<GammaCommentProfile>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaRelatedTag {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "tagID", default, skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<serde_json::Number>,
    #[serde(rename = "relatedTagID", default, skip_serializing_if = "Option::is_none")]
    pub related_tag_id: Option<serde_json::Number>,
    #[serde(rename = "rank", default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaSearch {
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GammaEvent>>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GammaSearchTag>>,
    #[serde(rename = "profiles", default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<GammaProfile>>,
    #[serde(rename = "pagination", default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<GammaPagination>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaSearchTag {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "event_count", default, skip_serializing_if = "Option::is_none")]
    pub event_count: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaSeries {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "subtitle", default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "seriesType", default, skip_serializing_if = "Option::is_none")]
    pub series_type: Option<String>,
    #[serde(rename = "recurrence", default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "icon", default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "layout", default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    #[serde(rename = "active", default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(rename = "archived", default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "new", default, skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    #[serde(rename = "featured", default, skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(rename = "restricted", default, skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[serde(rename = "isTemplate", default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(rename = "templateVariables", default, skip_serializing_if = "Option::is_none")]
    pub template_variables: Option<bool>,
    #[serde(rename = "publishedAt", default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "commentsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub comments_enabled: Option<bool>,
    #[serde(rename = "competitive", default, skip_serializing_if = "Option::is_none")]
    pub competitive: Option<String>,
    #[serde(rename = "volume24hr", default, skip_serializing_if = "Option::is_none")]
    pub volume24hr: Option<serde_json::Number>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<serde_json::Number>,
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<serde_json::Number>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "pythTokenID", default, skip_serializing_if = "Option::is_none")]
    pub pyth_token_id: Option<String>,
    #[serde(rename = "cgAssetName", default, skip_serializing_if = "Option::is_none")]
    pub cg_asset_name: Option<String>,
    #[serde(rename = "score", default, skip_serializing_if = "Option::is_none")]
    pub score: Option<serde_json::Number>,
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GammaEvent>>,
    #[serde(rename = "collections", default, skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<GammaCollection>>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<GammaCategory>>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GammaTag>>,
    #[serde(rename = "commentCount", default, skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<serde_json::Number>,
    #[serde(rename = "chats", default, skip_serializing_if = "Option::is_none")]
    pub chats: Option<Vec<GammaChat>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaSeriesSummary {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "eventDates", default, skip_serializing_if = "Option::is_none")]
    pub event_dates: Option<Vec<String>>,
    #[serde(rename = "eventWeeks", default, skip_serializing_if = "Option::is_none")]
    pub event_weeks: Option<Vec<serde_json::Number>>,
    #[serde(rename = "earliest_open_week", default, skip_serializing_if = "Option::is_none")]
    pub earliest_open_week: Option<serde_json::Number>,
    #[serde(rename = "earliest_open_date", default, skip_serializing_if = "Option::is_none")]
    pub earliest_open_date: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaServiceUnavailableError {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaSportsMarketTypesResponse {
    #[serde(rename = "marketTypes", default, skip_serializing_if = "Option::is_none")]
    pub market_types: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaSportsMetadata {
    #[serde(rename = "sport", default, skip_serializing_if = "Option::is_none")]
    pub sport: Option<String>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "resolution", default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(rename = "ordering", default, skip_serializing_if = "Option::is_none")]
    pub ordering: Option<String>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(rename = "series", default, skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaTag {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "slug", default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "forceShow", default, skip_serializing_if = "Option::is_none")]
    pub force_show: Option<bool>,
    #[serde(rename = "publishedAt", default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<serde_json::Number>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<serde_json::Number>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "forceHide", default, skip_serializing_if = "Option::is_none")]
    pub force_hide: Option<bool>,
    #[serde(rename = "isCarousel", default, skip_serializing_if = "Option::is_none")]
    pub is_carousel: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaTeam {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Number>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "league", default, skip_serializing_if = "Option::is_none")]
    pub league: Option<String>,
    #[serde(rename = "record", default, skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
    #[serde(rename = "logo", default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "abbreviation", default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[serde(rename = "alias", default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaTemplate {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "eventTitle", default, skip_serializing_if = "Option::is_none")]
    pub event_title: Option<String>,
    #[serde(rename = "eventSlug", default, skip_serializing_if = "Option::is_none")]
    pub event_slug: Option<String>,
    #[serde(rename = "eventImage", default, skip_serializing_if = "Option::is_none")]
    pub event_image: Option<String>,
    #[serde(rename = "marketTitle", default, skip_serializing_if = "Option::is_none")]
    pub market_title: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "resolutionSource", default, skip_serializing_if = "Option::is_none")]
    pub resolution_source: Option<String>,
    #[serde(rename = "negRisk", default, skip_serializing_if = "Option::is_none")]
    pub neg_risk: Option<bool>,
    #[serde(rename = "sortBy", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "showMarketImages", default, skip_serializing_if = "Option::is_none")]
    pub show_market_images: Option<bool>,
    #[serde(rename = "seriesSlug", default, skip_serializing_if = "Option::is_none")]
    pub series_slug: Option<String>,
    #[serde(rename = "outcomes", default, skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GammaValidationError {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerAuthorizationsParams {
    #[serde(rename = "Idempotency-Key")]
    pub idempotency_key: String,
    #[serde(rename = "body")]
    pub body: RelayerAuthorizationsParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerAuthorizationsParamsBody {
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    #[serde(rename = "validUntil")]
    pub valid_until: String,
    #[serde(rename = "deadline")]
    pub deadline: String,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "sessionSignerAddress")]
    pub session_signer_address: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "walletAddress")]
    pub wallet_address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerDeployedResponse {
    #[serde(rename = "deployed", default, skip_serializing_if = "Option::is_none")]
    pub deployed: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerErrorResponse {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerGetDeployedParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerGetDeployedResponse {
    #[serde(rename = "deployed", default, skip_serializing_if = "Option::is_none")]
    pub deployed: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerGetExecuteParamsParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerGetNonceParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerGetNonceResponse {
    #[serde(rename = "nonce", default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerGetRelayPayloadParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerGetRelayPayloadResponse {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "nonce", default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerGetRelayerApiKeysParams {
    #[serde(rename = "RELAYER_API_KEY", default, skip_serializing_if = "Option::is_none")]
    pub relayer_api_key: Option<String>,
    #[serde(rename = "RELAYER_API_KEY_ADDRESS", default, skip_serializing_if = "Option::is_none")]
    pub relayer_api_key_address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerGetTransactionParams {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerGetTransactionsParams {
    #[serde(rename = "POLY_BUILDER_API_KEY", default, skip_serializing_if = "Option::is_none")]
    pub poly_builder_api_key: Option<String>,
    #[serde(rename = "POLY_BUILDER_TIMESTAMP", default, skip_serializing_if = "Option::is_none")]
    pub poly_builder_timestamp: Option<String>,
    #[serde(rename = "POLY_BUILDER_PASSPHRASE", default, skip_serializing_if = "Option::is_none")]
    pub poly_builder_passphrase: Option<String>,
    #[serde(rename = "POLY_BUILDER_SIGNATURE", default, skip_serializing_if = "Option::is_none")]
    pub poly_builder_signature: Option<String>,
    #[serde(rename = "RELAYER_API_KEY", default, skip_serializing_if = "Option::is_none")]
    pub relayer_api_key: Option<String>,
    #[serde(rename = "RELAYER_API_KEY_ADDRESS", default, skip_serializing_if = "Option::is_none")]
    pub relayer_api_key_address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerNonceResponse {
    #[serde(rename = "nonce", default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerPostSubmitParams {
    #[serde(rename = "POLY_BUILDER_API_KEY", default, skip_serializing_if = "Option::is_none")]
    pub poly_builder_api_key: Option<String>,
    #[serde(rename = "POLY_BUILDER_TIMESTAMP", default, skip_serializing_if = "Option::is_none")]
    pub poly_builder_timestamp: Option<String>,
    #[serde(rename = "POLY_BUILDER_PASSPHRASE", default, skip_serializing_if = "Option::is_none")]
    pub poly_builder_passphrase: Option<String>,
    #[serde(rename = "POLY_BUILDER_SIGNATURE", default, skip_serializing_if = "Option::is_none")]
    pub poly_builder_signature: Option<String>,
    #[serde(rename = "RELAYER_API_KEY", default, skip_serializing_if = "Option::is_none")]
    pub relayer_api_key: Option<String>,
    #[serde(rename = "RELAYER_API_KEY_ADDRESS", default, skip_serializing_if = "Option::is_none")]
    pub relayer_api_key_address: Option<String>,
    #[serde(rename = "body")]
    pub body: Box<RelayerSubmitRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerPostSubmitResponse {
    #[serde(rename = "transactionID", default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerRelayPayloadResponse {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "nonce", default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerRelayerApiKey {
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerRelayerTransaction {
    #[serde(rename = "transactionID", default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "transactionHash", default, skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(rename = "proxyAddress", default, skip_serializing_if = "Option::is_none")]
    pub proxy_address: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "nonce", default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "signature", default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerRevocationsParams {
    #[serde(rename = "Idempotency-Key")]
    pub idempotency_key: String,
    #[serde(rename = "body")]
    pub body: RelayerRevocationsParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerRevocationsParamsBody {
    #[serde(rename = "deadline")]
    pub deadline: String,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "sessionSignerAddress")]
    pub session_signer_address: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "walletAddress")]
    pub wallet_address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerSignatureParams {
    #[serde(rename = "gasPrice", default, skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
    #[serde(rename = "operation", default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "safeTxnGas", default, skip_serializing_if = "Option::is_none")]
    pub safe_txn_gas: Option<String>,
    #[serde(rename = "baseGas", default, skip_serializing_if = "Option::is_none")]
    pub base_gas: Option<String>,
    #[serde(rename = "gasToken", default, skip_serializing_if = "Option::is_none")]
    pub gas_token: Option<String>,
    #[serde(rename = "refundReceiver", default, skip_serializing_if = "Option::is_none")]
    pub refund_receiver: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerSubmitRequest {
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "proxyWallet")]
    pub proxy_wallet: String,
    #[serde(rename = "data")]
    pub data: String,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "signatureParams")]
    pub signature_params: Box<RelayerSignatureParams>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelayerSubmitResponse {
    #[serde(rename = "transactionID", default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedPolymarketClient(PolymarketClient);
impl PolymarketClient {
    pub fn typed(self) -> TypedPolymarketClient { TypedPolymarketClient(self) }
}
impl TypedPolymarketClient {
    pub fn bridge_get_status_address(&self, request: &BridgeGetStatusAddressParams) -> Result<Box<BridgeTransactionStatusResponse>> {
        serde_json::from_value(self.0.bridge_get_status_address(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn bridge_get_supported_assets(&self, request: &BridgeGetSupportedAssetsParams) -> Result<Box<BridgeSupportedAssetsResponse>> {
        serde_json::from_value(self.0.bridge_get_supported_assets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn bridge_post_deposit(&self, request: &BridgePostDepositParams) -> Result<Box<BridgeDepositResponse>> {
        serde_json::from_value(self.0.bridge_post_deposit(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn bridge_post_quote(&self, request: &BridgePostQuoteParams) -> Result<Box<BridgeQuoteResponse>> {
        serde_json::from_value(self.0.bridge_post_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn bridge_post_withdraw(&self, request: &BridgePostWithdrawParams) -> Result<Box<BridgeDepositResponse>> {
        serde_json::from_value(self.0.bridge_post_withdraw(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn builder_gateway_accept_rfq(&self, request: &BuilderGatewayAcceptRfqParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.builder_gateway_accept_rfq(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn builder_gateway_create_rfq(&self, request: &BuilderGatewayCreateRfqParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.builder_gateway_create_rfq(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn builder_gateway_get_rfq(&self, request: &BuilderGatewayGetRfqParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.builder_gateway_get_rfq(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_cancel_all_orders(&self, request: &ClobCancelAllOrdersParams) -> Result<Box<ClobCancelOrdersResponse>> {
        serde_json::from_value(self.0.clob_cancel_all_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_cancel_market_orders(&self, request: &ClobCancelMarketOrdersParams) -> Result<Box<ClobCancelOrdersResponse>> {
        serde_json::from_value(self.0.clob_cancel_market_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_cancel_order(&self, request: &ClobCancelOrderParams) -> Result<Box<ClobCancelOrdersResponse>> {
        serde_json::from_value(self.0.clob_cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_cancel_orders(&self, request: &ClobCancelOrdersParams) -> Result<Box<ClobCancelOrdersResponse>> {
        serde_json::from_value(self.0.clob_cancel_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_create_api_key(&self, request: &ClobCreateApiKeyParams) -> Result<Box<ClobApiKeyResponse>> {
        serde_json::from_value(self.0.clob_create_api_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_create_builder_api_key(&self, request: &ClobCreateBuilderApiKeyParams) -> Result<Box<ClobBuilderApiKeyResponse>> {
        serde_json::from_value(self.0.clob_create_builder_api_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_delete_api_key(&self, request: &ClobDeleteApiKeyParams) -> Result<String> {
        serde_json::from_value(self.0.clob_delete_api_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_derive_api_key(&self, request: &ClobDeriveApiKeyParams) -> Result<Box<ClobApiKeyResponse>> {
        serde_json::from_value(self.0.clob_derive_api_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_drop_notifications(&self, request: &ClobDropNotificationsParams) -> Result<String> {
        serde_json::from_value(self.0.clob_drop_notifications(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_api_keys(&self, request: &ClobGetApiKeysParams) -> Result<Box<ClobApiKeysResponse>> {
        serde_json::from_value(self.0.clob_get_api_keys(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_balance_allowance(&self, request: &ClobGetBalanceAllowanceParams) -> Result<Box<ClobBalanceAllowanceResponse>> {
        serde_json::from_value(self.0.clob_get_balance_allowance(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_batch_prices_history(&self, request: &ClobGetBatchPricesHistoryParams) -> Result<Box<ClobBatchPricesHistoryResponse>> {
        serde_json::from_value(self.0.clob_get_batch_prices_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_book(&self, request: &ClobGetBookParams) -> Result<Box<ClobOrderBookSummary>> {
        serde_json::from_value(self.0.clob_get_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_books_get(&self, request: &ClobGetBooksGetParams) -> Result<Vec<ClobOrderBookSummary>> {
        serde_json::from_value(self.0.clob_get_books_get(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_books_post(&self, request: &ClobGetBooksPostParams) -> Result<Vec<ClobOrderBookSummary>> {
        serde_json::from_value(self.0.clob_get_books_post(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_builder_api_keys(&self, request: &ClobGetBuilderApiKeysParams) -> Result<Vec<String>> {
        serde_json::from_value(self.0.clob_get_builder_api_keys(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_builder_trades(&self, request: &ClobGetBuilderTradesParams) -> Result<Box<ClobBuilderTradesResponse>> {
        serde_json::from_value(self.0.clob_get_builder_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_clob_market_info(&self, request: &ClobGetClobMarketInfoParams) -> Result<Box<ClobClobMarketDetails>> {
        serde_json::from_value(self.0.clob_get_clob_market_info(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_closed_only_mode(&self, request: &ClobGetClosedOnlyModeParams) -> Result<Box<ClobClosedOnlyResponse>> {
        serde_json::from_value(self.0.clob_get_closed_only_mode(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_current_rebated_fees(&self, request: &ClobGetCurrentRebatedFeesParams) -> Result<Vec<ClobRebatedFees>> {
        serde_json::from_value(self.0.clob_get_current_rebated_fees(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_current_rewards(&self, request: &ClobGetCurrentRewardsParams) -> Result<Box<ClobPaginatedCurrentReward>> {
        serde_json::from_value(self.0.clob_get_current_rewards(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_earnings_for_user_for_day(&self, request: &ClobGetEarningsForUserForDayParams) -> Result<Box<ClobPaginatedUserEarnings>> {
        serde_json::from_value(self.0.clob_get_earnings_for_user_for_day(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_fee_rate(&self, request: &ClobGetFeeRateParams) -> Result<Box<ClobFeeRate>> {
        serde_json::from_value(self.0.clob_get_fee_rate(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_fee_rate_by_path(&self, request: &ClobGetFeeRateByPathParams) -> Result<Box<ClobFeeRate>> {
        serde_json::from_value(self.0.clob_get_fee_rate_by_path(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_last_trade_price(&self, request: &ClobGetLastTradePriceParams) -> Result<ClobGetLastTradePriceResponse> {
        serde_json::from_value(self.0.clob_get_last_trade_price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_last_trades_prices_get(&self, request: &ClobGetLastTradesPricesGetParams) -> Result<Vec<ClobGetLastTradesPricesGetResponseItem>> {
        serde_json::from_value(self.0.clob_get_last_trades_prices_get(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_last_trades_prices_post(&self, request: &ClobGetLastTradesPricesPostParams) -> Result<Vec<ClobGetLastTradesPricesPostResponseItem>> {
        serde_json::from_value(self.0.clob_get_last_trades_prices_post(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_market_by_token(&self, request: &ClobGetMarketByTokenParams) -> Result<Box<ClobMarketByTokenResponse>> {
        serde_json::from_value(self.0.clob_get_market_by_token(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_market_live_activity(&self, request: &ClobGetMarketLiveActivityParams) -> Result<Box<ClobLiveActivityMarket>> {
        serde_json::from_value(self.0.clob_get_market_live_activity(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_markets_live_activity(&self, request: &ClobGetMarketsLiveActivityParams) -> Result<Vec<ClobLiveActivityMarket>> {
        serde_json::from_value(self.0.clob_get_markets_live_activity(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_midpoint(&self, request: &ClobGetMidpointParams) -> Result<ClobGetMidpointResponse> {
        serde_json::from_value(self.0.clob_get_midpoint(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_midpoints_get(&self, request: &ClobGetMidpointsGetParams) -> Result<std::collections::BTreeMap<String,String>> {
        serde_json::from_value(self.0.clob_get_midpoints_get(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_midpoints_post(&self, request: &ClobGetMidpointsPostParams) -> Result<std::collections::BTreeMap<String,String>> {
        serde_json::from_value(self.0.clob_get_midpoints_post(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_multi_markets(&self, request: &ClobGetMultiMarketsParams) -> Result<Box<ClobPaginatedMultiMarketInfo>> {
        serde_json::from_value(self.0.clob_get_multi_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_neg_risk(&self, request: &ClobGetNegRiskParams) -> Result<Box<ClobNegRisk>> {
        serde_json::from_value(self.0.clob_get_neg_risk(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_neg_risk_by_path(&self, request: &ClobGetNegRiskByPathParams) -> Result<Box<ClobNegRisk>> {
        serde_json::from_value(self.0.clob_get_neg_risk_by_path(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_notifications(&self, request: &ClobGetNotificationsParams) -> Result<Vec<ClobNotification>> {
        serde_json::from_value(self.0.clob_get_notifications(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_order(&self, request: &ClobGetOrderParams) -> Result<Box<ClobOpenOrder>> {
        serde_json::from_value(self.0.clob_get_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_order_scoring(&self, request: &ClobGetOrderScoringParams) -> Result<Box<ClobOrderScoringResponse>> {
        serde_json::from_value(self.0.clob_get_order_scoring(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_orders(&self, request: &ClobGetOrdersParams) -> Result<Box<ClobOrdersResponse>> {
        serde_json::from_value(self.0.clob_get_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_orders_scoring(&self, request: &ClobGetOrdersScoringParams) -> Result<Box<ClobOrdersScoringResponse>> {
        serde_json::from_value(self.0.clob_get_orders_scoring(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_price(&self, request: &ClobGetPriceParams) -> Result<ClobGetPriceResponse> {
        serde_json::from_value(self.0.clob_get_price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_prices_get(&self, request: &ClobGetPricesGetParams) -> Result<std::collections::BTreeMap<String,std::collections::BTreeMap<String,serde_json::Number>>> {
        serde_json::from_value(self.0.clob_get_prices_get(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_prices_history(&self, request: &ClobGetPricesHistoryParams) -> Result<Box<ClobPricesHistoryResponse>> {
        serde_json::from_value(self.0.clob_get_prices_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_prices_post(&self, request: &ClobGetPricesPostParams) -> Result<std::collections::BTreeMap<String,std::collections::BTreeMap<String,serde_json::Number>>> {
        serde_json::from_value(self.0.clob_get_prices_post(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_rate_limit_tier(&self, request: &ClobGetRateLimitTierParams) -> Result<Box<ClobRateLimitTierResponse>> {
        serde_json::from_value(self.0.clob_get_rate_limit_tier(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_raw_rewards_for_market(&self, request: &ClobGetRawRewardsForMarketParams) -> Result<Box<ClobPaginatedMarketReward>> {
        serde_json::from_value(self.0.clob_get_raw_rewards_for_market(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_reward_percentages_for_user(&self, request: &ClobGetRewardPercentagesForUserParams) -> Result<std::collections::BTreeMap<String,serde_json::Number>> {
        serde_json::from_value(self.0.clob_get_reward_percentages_for_user(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_sampling_markets(&self, request: &ClobGetSamplingMarketsParams) -> Result<Box<ClobPaginatedMarkets>> {
        serde_json::from_value(self.0.clob_get_sampling_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_sampling_simplified_markets(&self, request: &ClobGetSamplingSimplifiedMarketsParams) -> Result<Box<ClobPaginatedSimplifiedMarkets>> {
        serde_json::from_value(self.0.clob_get_sampling_simplified_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_simplified_markets(&self, request: &ClobGetSimplifiedMarketsParams) -> Result<Box<ClobPaginatedSimplifiedMarkets>> {
        serde_json::from_value(self.0.clob_get_simplified_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_spread(&self, request: &ClobGetSpreadParams) -> Result<ClobGetSpreadResponse> {
        serde_json::from_value(self.0.clob_get_spread(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_spreads(&self, request: &ClobGetSpreadsParams) -> Result<std::collections::BTreeMap<String,String>> {
        serde_json::from_value(self.0.clob_get_spreads(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_tick_size(&self, request: &ClobGetTickSizeParams) -> Result<Box<ClobTickSize>> {
        serde_json::from_value(self.0.clob_get_tick_size(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_tick_size_by_path(&self, request: &ClobGetTickSizeByPathParams) -> Result<Box<ClobTickSize>> {
        serde_json::from_value(self.0.clob_get_tick_size_by_path(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_time(&self, request: &ClobGetTimeParams) -> Result<serde_json::Number> {
        serde_json::from_value(self.0.clob_get_time(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_total_earnings_for_user_for_day(&self, request: &ClobGetTotalEarningsForUserForDayParams) -> Result<Vec<ClobTotalUserEarning>> {
        serde_json::from_value(self.0.clob_get_total_earnings_for_user_for_day(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_trades(&self, request: &ClobGetTradesParams) -> Result<Box<ClobTradesResponse>> {
        serde_json::from_value(self.0.clob_get_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_update_balance_allowance(&self, request: &ClobGetUpdateBalanceAllowanceParams) -> Result<Box<ClobBalanceAllowanceResponse>> {
        serde_json::from_value(self.0.clob_get_update_balance_allowance(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_get_user_earnings_and_markets_config(&self, request: &ClobGetUserEarningsAndMarketsConfigParams) -> Result<Box<ClobPaginatedUserRewardsMarkets>> {
        serde_json::from_value(self.0.clob_get_user_earnings_and_markets_config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_list_session_signers(&self, request: &ClobListSessionSignersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.clob_list_session_signers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_post_order(&self, request: &ClobPostOrderParams) -> Result<Box<ClobSendOrderResponse>> {
        serde_json::from_value(self.0.clob_post_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_post_orders(&self, request: &ClobPostOrdersParams) -> Result<Vec<ClobSendOrderResponse>> {
        serde_json::from_value(self.0.clob_post_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_post_orders_scoring(&self, request: &ClobPostOrdersScoringParams) -> Result<Box<ClobOrdersScoringResponse>> {
        serde_json::from_value(self.0.clob_post_orders_scoring(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_revoke_builder_api_key(&self, request: &ClobRevokeBuilderApiKeyParams) -> Result<String> {
        serde_json::from_value(self.0.clob_revoke_builder_api_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_send_heartbeat(&self, request: &ClobSendHeartbeatParams) -> Result<Box<ClobHeartbeatResponse>> {
        serde_json::from_value(self.0.clob_send_heartbeat(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_send_heartbeat_v1(&self, request: &ClobSendHeartbeatV1Params) -> Result<Box<ClobHeartbeatV1Response>> {
        serde_json::from_value(self.0.clob_send_heartbeat_v1(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn clob_update_balance_allowance(&self, request: &ClobUpdateBalanceAllowanceParams) -> Result<serde_json::Map<String,serde_json::Value>> {
        serde_json::from_value(self.0.clob_update_balance_allowance(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn collateral_return_plan(&self, request: &CollateralReturnPlanParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.collateral_return_plan(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn collateral_return_submit(&self, request: &CollateralReturnSubmitParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.collateral_return_submit(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn combos_rfq_cancel_maker_quote(&self, request: &CombosRfqCancelMakerQuoteParams) -> Result<Box<CombosRfqRFQSnapshot>> {
        serde_json::from_value(self.0.combos_rfq_cancel_maker_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn combos_rfq_get_combo_markets(&self, request: &CombosRfqGetComboMarketsParams) -> Result<Box<CombosRfqComboMarketsResponse>> {
        serde_json::from_value(self.0.combos_rfq_get_combo_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn combos_rfq_submit_maker_confirmation(&self, request: &CombosRfqSubmitMakerConfirmationParams) -> Result<Box<CombosRfqMakerConfirmationResult>> {
        serde_json::from_value(self.0.combos_rfq_submit_maker_confirmation(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn combos_rfq_submit_maker_quote(&self, request: &CombosRfqSubmitMakerQuoteParams) -> Result<Box<CombosRfqRFQSnapshot>> {
        serde_json::from_value(self.0.combos_rfq_submit_maker_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_activity(&self, request: &DataGetActivityParams) -> Result<Box<DataActivityPage>> {
        serde_json::from_value(self.0.data_get_activity(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_approvals(&self, request: &DataGetApprovalsParams) -> Result<Box<DataEnvelopeApprovals>> {
        serde_json::from_value(self.0.data_get_approvals(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_biggest_winners(&self, request: &DataGetBiggestWinnersParams) -> Result<Box<DataBiggestWinnersPage>> {
        serde_json::from_value(self.0.data_get_biggest_winners(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_builders_leaderboard(&self, request: &DataGetBuildersLeaderboardParams) -> Result<Box<DataBuildersLeaderboardPage>> {
        serde_json::from_value(self.0.data_get_builders_leaderboard(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_builders_volume(&self, request: &DataGetBuildersVolumeParams) -> Result<Box<DataEnvelopeVecBuilderVolumePoint>> {
        serde_json::from_value(self.0.data_get_builders_volume(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_combo_positions(&self, request: &DataGetComboPositionsParams) -> Result<Box<DataComboPositionsPage>> {
        serde_json::from_value(self.0.data_get_combo_positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_combos(&self, request: &DataGetCombosParams) -> Result<Box<DataComboActivityPage>> {
        serde_json::from_value(self.0.data_get_combos(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_holders(&self, request: &DataGetHoldersParams) -> Result<Box<DataHoldersPage>> {
        serde_json::from_value(self.0.data_get_holders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_leaderboard(&self, request: &DataGetLeaderboardParams) -> Result<Box<DataLeaderboardResponse>> {
        serde_json::from_value(self.0.data_get_leaderboard(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_live_volume(&self, request: &DataGetLiveVolumeParams) -> Result<Box<DataEnvelopeLiveVolume>> {
        serde_json::from_value(self.0.data_get_live_volume(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_oi(&self, request: &DataGetOiParams) -> Result<Box<DataEnvelopeVecOpenInterest>> {
        serde_json::from_value(self.0.data_get_oi(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_positions(&self, request: &DataGetPositionsParams) -> Result<Box<DataPositionsPage>> {
        serde_json::from_value(self.0.data_get_positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_prices_history(&self, request: &DataGetPricesHistoryParams) -> Result<Box<DataPricesHistoryPage>> {
        serde_json::from_value(self.0.data_get_prices_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_resolutions(&self, request: &DataGetResolutionsParams) -> Result<Box<DataEnvelopeVecResolution>> {
        serde_json::from_value(self.0.data_get_resolutions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_status(&self, request: &DataGetStatusParams) -> Result<Box<DataEnvelopeServiceStatus>> {
        serde_json::from_value(self.0.data_get_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_tokens(&self, request: &DataGetTokensParams) -> Result<Box<DataEnvelopeVecTokenRef>> {
        serde_json::from_value(self.0.data_get_tokens(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_trades(&self, request: &DataGetTradesParams) -> Result<Box<DataTradesPage>> {
        serde_json::from_value(self.0.data_get_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_user_pnl_v2(&self, request: &DataGetUserPnlV2Params) -> Result<Box<DataEnvelopeUserPnlSeries>> {
        serde_json::from_value(self.0.data_get_user_pnl_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_user_stats(&self, request: &DataGetUserStatsParams) -> Result<Box<DataEnvelopeOptionUserStats>> {
        serde_json::from_value(self.0.data_get_user_stats(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_user_volume(&self, request: &DataGetUserVolumeParams) -> Result<Box<DataEnvelopeUserVolume>> {
        serde_json::from_value(self.0.data_get_user_volume(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn data_get_value(&self, request: &DataGetValueParams) -> Result<Box<DataEnvelopePortfolioValue>> {
        serde_json::from_value(self.0.data_get_value(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_abridged_markets(&self, request: &GammaGetAbridgedMarketsParams) -> Result<Vec<GammaMarket>> {
        serde_json::from_value(self.0.gamma_get_abridged_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_comments_by_id(&self, request: &GammaGetCommentsByIdParams) -> Result<Vec<GammaComment>> {
        serde_json::from_value(self.0.gamma_get_comments_by_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_comments_by_user_address(&self, request: &GammaGetCommentsByUserAddressParams) -> Result<Vec<GammaComment>> {
        serde_json::from_value(self.0.gamma_get_comments_by_user_address(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_event(&self, request: &GammaGetEventParams) -> Result<Box<GammaEvent>> {
        serde_json::from_value(self.0.gamma_get_event(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_event_by_slug(&self, request: &GammaGetEventBySlugParams) -> Result<Box<GammaEvent>> {
        serde_json::from_value(self.0.gamma_get_event_by_slug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_event_comments_count(&self, request: &GammaGetEventCommentsCountParams) -> Result<Box<GammaCount>> {
        serde_json::from_value(self.0.gamma_get_event_comments_count(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_event_creator(&self, request: &GammaGetEventCreatorParams) -> Result<Box<GammaEventCreator>> {
        serde_json::from_value(self.0.gamma_get_event_creator(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_event_tags(&self, request: &GammaGetEventTagsParams) -> Result<Vec<GammaTag>> {
        serde_json::from_value(self.0.gamma_get_event_tags(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_event_tweet_count(&self, request: &GammaGetEventTweetCountParams) -> Result<Box<GammaEventTweetCount>> {
        serde_json::from_value(self.0.gamma_get_event_tweet_count(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_gamma_status(&self, request: &GammaGetGammaStatusParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.gamma_get_gamma_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_market(&self, request: &GammaGetMarketParams) -> Result<Box<GammaMarket>> {
        serde_json::from_value(self.0.gamma_get_market(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_market_by_slug(&self, request: &GammaGetMarketBySlugParams) -> Result<Box<GammaMarket>> {
        serde_json::from_value(self.0.gamma_get_market_by_slug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_market_description(&self, request: &GammaGetMarketDescriptionParams) -> Result<Box<GammaMarketDescription>> {
        serde_json::from_value(self.0.gamma_get_market_description(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_market_tags(&self, request: &GammaGetMarketTagsParams) -> Result<Vec<GammaTag>> {
        serde_json::from_value(self.0.gamma_get_market_tags(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_markets_information(&self, request: &GammaGetMarketsInformationParams) -> Result<Vec<GammaMarket>> {
        serde_json::from_value(self.0.gamma_get_markets_information(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_public_profile(&self, request: &GammaGetPublicProfileParams) -> Result<Box<GammaPublicProfileResponse>> {
        serde_json::from_value(self.0.gamma_get_public_profile(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_public_profile_by_user_address(&self, request: &GammaGetPublicProfileByUserAddressParams) -> Result<Box<GammaProfile>> {
        serde_json::from_value(self.0.gamma_get_public_profile_by_user_address(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_related_tags_by_id(&self, request: &GammaGetRelatedTagsByIdParams) -> Result<Vec<GammaRelatedTag>> {
        serde_json::from_value(self.0.gamma_get_related_tags_by_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_related_tags_by_slug(&self, request: &GammaGetRelatedTagsBySlugParams) -> Result<Vec<GammaRelatedTag>> {
        serde_json::from_value(self.0.gamma_get_related_tags_by_slug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_series(&self, request: &GammaGetSeriesParams) -> Result<Box<GammaSeries>> {
        serde_json::from_value(self.0.gamma_get_series(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_series_comments_count(&self, request: &GammaGetSeriesCommentsCountParams) -> Result<Box<GammaCount>> {
        serde_json::from_value(self.0.gamma_get_series_comments_count(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_series_summary_by_id(&self, request: &GammaGetSeriesSummaryByIdParams) -> Result<Box<GammaSeriesSummary>> {
        serde_json::from_value(self.0.gamma_get_series_summary_by_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_series_summary_by_slug(&self, request: &GammaGetSeriesSummaryBySlugParams) -> Result<Box<GammaSeriesSummary>> {
        serde_json::from_value(self.0.gamma_get_series_summary_by_slug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_sports_market_types(&self, request: &GammaGetSportsMarketTypesParams) -> Result<Box<GammaSportsMarketTypesResponse>> {
        serde_json::from_value(self.0.gamma_get_sports_market_types(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_sports_metadata(&self, request: &GammaGetSportsMetadataParams) -> Result<Vec<GammaSportsMetadata>> {
        serde_json::from_value(self.0.gamma_get_sports_metadata(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_tag(&self, request: &GammaGetTagParams) -> Result<Box<GammaTag>> {
        serde_json::from_value(self.0.gamma_get_tag(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_tag_by_slug(&self, request: &GammaGetTagBySlugParams) -> Result<Box<GammaTag>> {
        serde_json::from_value(self.0.gamma_get_tag_by_slug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_tags_related_to_a_tag_by_id(&self, request: &GammaGetTagsRelatedToATagByIdParams) -> Result<Vec<GammaTag>> {
        serde_json::from_value(self.0.gamma_get_tags_related_to_a_tag_by_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_tags_related_to_a_tag_by_slug(&self, request: &GammaGetTagsRelatedToATagBySlugParams) -> Result<Vec<GammaTag>> {
        serde_json::from_value(self.0.gamma_get_tags_related_to_a_tag_by_slug(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_get_team(&self, request: &GammaGetTeamParams) -> Result<Box<GammaTeam>> {
        serde_json::from_value(self.0.gamma_get_team(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_list_comments(&self, request: &GammaListCommentsParams) -> Result<Vec<GammaComment>> {
        serde_json::from_value(self.0.gamma_list_comments(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_list_event_creators(&self, request: &GammaListEventCreatorsParams) -> Result<Vec<GammaEventCreator>> {
        serde_json::from_value(self.0.gamma_list_event_creators(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_list_events(&self, request: &GammaListEventsParams) -> Result<Vec<GammaEvent>> {
        serde_json::from_value(self.0.gamma_list_events(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_list_events_keyset(&self, request: &GammaListEventsKeysetParams) -> Result<Box<GammaKeysetEventsResponse>> {
        serde_json::from_value(self.0.gamma_list_events_keyset(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_list_events_pagination(&self, request: &GammaListEventsPaginationParams) -> Result<Box<GammaEventsPagination>> {
        serde_json::from_value(self.0.gamma_list_events_pagination(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_list_markets(&self, request: &GammaListMarketsParams) -> Result<Vec<GammaMarket>> {
        serde_json::from_value(self.0.gamma_list_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_list_markets_keyset(&self, request: &GammaListMarketsKeysetParams) -> Result<Box<GammaKeysetMarketsResponse>> {
        serde_json::from_value(self.0.gamma_list_markets_keyset(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_list_series(&self, request: &GammaListSeriesParams) -> Result<Vec<GammaSeries>> {
        serde_json::from_value(self.0.gamma_list_series(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_list_sport_events_results(&self, request: &GammaListSportEventsResultsParams) -> Result<Vec<GammaEvent>> {
        serde_json::from_value(self.0.gamma_list_sport_events_results(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_list_tags(&self, request: &GammaListTagsParams) -> Result<Vec<GammaTag>> {
        serde_json::from_value(self.0.gamma_list_tags(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_list_teams(&self, request: &GammaListTeamsParams) -> Result<Vec<GammaTeam>> {
        serde_json::from_value(self.0.gamma_list_teams(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn gamma_public_search(&self, request: &GammaPublicSearchParams) -> Result<Box<GammaSearch>> {
        serde_json::from_value(self.0.gamma_public_search(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn relayer_authorizations(&self, request: &RelayerAuthorizationsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.relayer_authorizations(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn relayer_get_deployed(&self, request: &RelayerGetDeployedParams) -> Result<Box<RelayerDeployedResponse>> {
        serde_json::from_value(self.0.relayer_get_deployed(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn relayer_get_execute_params(&self, request: &RelayerGetExecuteParamsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.relayer_get_execute_params(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn relayer_get_nonce(&self, request: &RelayerGetNonceParams) -> Result<Box<RelayerNonceResponse>> {
        serde_json::from_value(self.0.relayer_get_nonce(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn relayer_get_relay_payload(&self, request: &RelayerGetRelayPayloadParams) -> Result<Box<RelayerRelayPayloadResponse>> {
        serde_json::from_value(self.0.relayer_get_relay_payload(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn relayer_get_relayer_api_keys(&self, request: &RelayerGetRelayerApiKeysParams) -> Result<Vec<RelayerRelayerApiKey>> {
        serde_json::from_value(self.0.relayer_get_relayer_api_keys(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn relayer_get_transaction(&self, request: &RelayerGetTransactionParams) -> Result<Vec<RelayerRelayerTransaction>> {
        serde_json::from_value(self.0.relayer_get_transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn relayer_get_transactions(&self, request: &RelayerGetTransactionsParams) -> Result<Vec<RelayerRelayerTransaction>> {
        serde_json::from_value(self.0.relayer_get_transactions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn relayer_post_submit(&self, request: &RelayerPostSubmitParams) -> Result<Box<RelayerSubmitResponse>> {
        serde_json::from_value(self.0.relayer_post_submit(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub fn relayer_revocations(&self, request: &RelayerRevocationsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.relayer_revocations(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedPolymarketClient(crate::AsyncClient<PolymarketClient>);
#[cfg(feature = "async")]
impl TypedPolymarketClient {
    pub fn into_async(self) -> AsyncTypedPolymarketClient { AsyncTypedPolymarketClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedPolymarketClient {
    pub async fn bridge_get_status_address(&self, request: &BridgeGetStatusAddressParams) -> Result<Box<BridgeTransactionStatusResponse>> {
        serde_json::from_value(self.0.bridge_get_status_address(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn bridge_get_supported_assets(&self, request: &BridgeGetSupportedAssetsParams) -> Result<Box<BridgeSupportedAssetsResponse>> {
        serde_json::from_value(self.0.bridge_get_supported_assets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn bridge_post_deposit(&self, request: &BridgePostDepositParams) -> Result<Box<BridgeDepositResponse>> {
        serde_json::from_value(self.0.bridge_post_deposit(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn bridge_post_quote(&self, request: &BridgePostQuoteParams) -> Result<Box<BridgeQuoteResponse>> {
        serde_json::from_value(self.0.bridge_post_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn bridge_post_withdraw(&self, request: &BridgePostWithdrawParams) -> Result<Box<BridgeDepositResponse>> {
        serde_json::from_value(self.0.bridge_post_withdraw(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn builder_gateway_accept_rfq(&self, request: &BuilderGatewayAcceptRfqParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.builder_gateway_accept_rfq(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn builder_gateway_create_rfq(&self, request: &BuilderGatewayCreateRfqParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.builder_gateway_create_rfq(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn builder_gateway_get_rfq(&self, request: &BuilderGatewayGetRfqParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.builder_gateway_get_rfq(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_cancel_all_orders(&self, request: &ClobCancelAllOrdersParams) -> Result<Box<ClobCancelOrdersResponse>> {
        serde_json::from_value(self.0.clob_cancel_all_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_cancel_market_orders(&self, request: &ClobCancelMarketOrdersParams) -> Result<Box<ClobCancelOrdersResponse>> {
        serde_json::from_value(self.0.clob_cancel_market_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_cancel_order(&self, request: &ClobCancelOrderParams) -> Result<Box<ClobCancelOrdersResponse>> {
        serde_json::from_value(self.0.clob_cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_cancel_orders(&self, request: &ClobCancelOrdersParams) -> Result<Box<ClobCancelOrdersResponse>> {
        serde_json::from_value(self.0.clob_cancel_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_create_api_key(&self, request: &ClobCreateApiKeyParams) -> Result<Box<ClobApiKeyResponse>> {
        serde_json::from_value(self.0.clob_create_api_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_create_builder_api_key(&self, request: &ClobCreateBuilderApiKeyParams) -> Result<Box<ClobBuilderApiKeyResponse>> {
        serde_json::from_value(self.0.clob_create_builder_api_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_delete_api_key(&self, request: &ClobDeleteApiKeyParams) -> Result<String> {
        serde_json::from_value(self.0.clob_delete_api_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_derive_api_key(&self, request: &ClobDeriveApiKeyParams) -> Result<Box<ClobApiKeyResponse>> {
        serde_json::from_value(self.0.clob_derive_api_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_drop_notifications(&self, request: &ClobDropNotificationsParams) -> Result<String> {
        serde_json::from_value(self.0.clob_drop_notifications(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_api_keys(&self, request: &ClobGetApiKeysParams) -> Result<Box<ClobApiKeysResponse>> {
        serde_json::from_value(self.0.clob_get_api_keys(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_balance_allowance(&self, request: &ClobGetBalanceAllowanceParams) -> Result<Box<ClobBalanceAllowanceResponse>> {
        serde_json::from_value(self.0.clob_get_balance_allowance(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_batch_prices_history(&self, request: &ClobGetBatchPricesHistoryParams) -> Result<Box<ClobBatchPricesHistoryResponse>> {
        serde_json::from_value(self.0.clob_get_batch_prices_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_book(&self, request: &ClobGetBookParams) -> Result<Box<ClobOrderBookSummary>> {
        serde_json::from_value(self.0.clob_get_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_books_get(&self, request: &ClobGetBooksGetParams) -> Result<Vec<ClobOrderBookSummary>> {
        serde_json::from_value(self.0.clob_get_books_get(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_books_post(&self, request: &ClobGetBooksPostParams) -> Result<Vec<ClobOrderBookSummary>> {
        serde_json::from_value(self.0.clob_get_books_post(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_builder_api_keys(&self, request: &ClobGetBuilderApiKeysParams) -> Result<Vec<String>> {
        serde_json::from_value(self.0.clob_get_builder_api_keys(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_builder_trades(&self, request: &ClobGetBuilderTradesParams) -> Result<Box<ClobBuilderTradesResponse>> {
        serde_json::from_value(self.0.clob_get_builder_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_clob_market_info(&self, request: &ClobGetClobMarketInfoParams) -> Result<Box<ClobClobMarketDetails>> {
        serde_json::from_value(self.0.clob_get_clob_market_info(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_closed_only_mode(&self, request: &ClobGetClosedOnlyModeParams) -> Result<Box<ClobClosedOnlyResponse>> {
        serde_json::from_value(self.0.clob_get_closed_only_mode(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_current_rebated_fees(&self, request: &ClobGetCurrentRebatedFeesParams) -> Result<Vec<ClobRebatedFees>> {
        serde_json::from_value(self.0.clob_get_current_rebated_fees(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_current_rewards(&self, request: &ClobGetCurrentRewardsParams) -> Result<Box<ClobPaginatedCurrentReward>> {
        serde_json::from_value(self.0.clob_get_current_rewards(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_earnings_for_user_for_day(&self, request: &ClobGetEarningsForUserForDayParams) -> Result<Box<ClobPaginatedUserEarnings>> {
        serde_json::from_value(self.0.clob_get_earnings_for_user_for_day(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_fee_rate(&self, request: &ClobGetFeeRateParams) -> Result<Box<ClobFeeRate>> {
        serde_json::from_value(self.0.clob_get_fee_rate(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_fee_rate_by_path(&self, request: &ClobGetFeeRateByPathParams) -> Result<Box<ClobFeeRate>> {
        serde_json::from_value(self.0.clob_get_fee_rate_by_path(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_last_trade_price(&self, request: &ClobGetLastTradePriceParams) -> Result<ClobGetLastTradePriceResponse> {
        serde_json::from_value(self.0.clob_get_last_trade_price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_last_trades_prices_get(&self, request: &ClobGetLastTradesPricesGetParams) -> Result<Vec<ClobGetLastTradesPricesGetResponseItem>> {
        serde_json::from_value(self.0.clob_get_last_trades_prices_get(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_last_trades_prices_post(&self, request: &ClobGetLastTradesPricesPostParams) -> Result<Vec<ClobGetLastTradesPricesPostResponseItem>> {
        serde_json::from_value(self.0.clob_get_last_trades_prices_post(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_market_by_token(&self, request: &ClobGetMarketByTokenParams) -> Result<Box<ClobMarketByTokenResponse>> {
        serde_json::from_value(self.0.clob_get_market_by_token(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_market_live_activity(&self, request: &ClobGetMarketLiveActivityParams) -> Result<Box<ClobLiveActivityMarket>> {
        serde_json::from_value(self.0.clob_get_market_live_activity(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_markets_live_activity(&self, request: &ClobGetMarketsLiveActivityParams) -> Result<Vec<ClobLiveActivityMarket>> {
        serde_json::from_value(self.0.clob_get_markets_live_activity(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_midpoint(&self, request: &ClobGetMidpointParams) -> Result<ClobGetMidpointResponse> {
        serde_json::from_value(self.0.clob_get_midpoint(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_midpoints_get(&self, request: &ClobGetMidpointsGetParams) -> Result<std::collections::BTreeMap<String,String>> {
        serde_json::from_value(self.0.clob_get_midpoints_get(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_midpoints_post(&self, request: &ClobGetMidpointsPostParams) -> Result<std::collections::BTreeMap<String,String>> {
        serde_json::from_value(self.0.clob_get_midpoints_post(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_multi_markets(&self, request: &ClobGetMultiMarketsParams) -> Result<Box<ClobPaginatedMultiMarketInfo>> {
        serde_json::from_value(self.0.clob_get_multi_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_neg_risk(&self, request: &ClobGetNegRiskParams) -> Result<Box<ClobNegRisk>> {
        serde_json::from_value(self.0.clob_get_neg_risk(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_neg_risk_by_path(&self, request: &ClobGetNegRiskByPathParams) -> Result<Box<ClobNegRisk>> {
        serde_json::from_value(self.0.clob_get_neg_risk_by_path(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_notifications(&self, request: &ClobGetNotificationsParams) -> Result<Vec<ClobNotification>> {
        serde_json::from_value(self.0.clob_get_notifications(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_order(&self, request: &ClobGetOrderParams) -> Result<Box<ClobOpenOrder>> {
        serde_json::from_value(self.0.clob_get_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_order_scoring(&self, request: &ClobGetOrderScoringParams) -> Result<Box<ClobOrderScoringResponse>> {
        serde_json::from_value(self.0.clob_get_order_scoring(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_orders(&self, request: &ClobGetOrdersParams) -> Result<Box<ClobOrdersResponse>> {
        serde_json::from_value(self.0.clob_get_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_orders_scoring(&self, request: &ClobGetOrdersScoringParams) -> Result<Box<ClobOrdersScoringResponse>> {
        serde_json::from_value(self.0.clob_get_orders_scoring(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_price(&self, request: &ClobGetPriceParams) -> Result<ClobGetPriceResponse> {
        serde_json::from_value(self.0.clob_get_price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_prices_get(&self, request: &ClobGetPricesGetParams) -> Result<std::collections::BTreeMap<String,std::collections::BTreeMap<String,serde_json::Number>>> {
        serde_json::from_value(self.0.clob_get_prices_get(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_prices_history(&self, request: &ClobGetPricesHistoryParams) -> Result<Box<ClobPricesHistoryResponse>> {
        serde_json::from_value(self.0.clob_get_prices_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_prices_post(&self, request: &ClobGetPricesPostParams) -> Result<std::collections::BTreeMap<String,std::collections::BTreeMap<String,serde_json::Number>>> {
        serde_json::from_value(self.0.clob_get_prices_post(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_rate_limit_tier(&self, request: &ClobGetRateLimitTierParams) -> Result<Box<ClobRateLimitTierResponse>> {
        serde_json::from_value(self.0.clob_get_rate_limit_tier(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_raw_rewards_for_market(&self, request: &ClobGetRawRewardsForMarketParams) -> Result<Box<ClobPaginatedMarketReward>> {
        serde_json::from_value(self.0.clob_get_raw_rewards_for_market(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_reward_percentages_for_user(&self, request: &ClobGetRewardPercentagesForUserParams) -> Result<std::collections::BTreeMap<String,serde_json::Number>> {
        serde_json::from_value(self.0.clob_get_reward_percentages_for_user(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_sampling_markets(&self, request: &ClobGetSamplingMarketsParams) -> Result<Box<ClobPaginatedMarkets>> {
        serde_json::from_value(self.0.clob_get_sampling_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_sampling_simplified_markets(&self, request: &ClobGetSamplingSimplifiedMarketsParams) -> Result<Box<ClobPaginatedSimplifiedMarkets>> {
        serde_json::from_value(self.0.clob_get_sampling_simplified_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_simplified_markets(&self, request: &ClobGetSimplifiedMarketsParams) -> Result<Box<ClobPaginatedSimplifiedMarkets>> {
        serde_json::from_value(self.0.clob_get_simplified_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_spread(&self, request: &ClobGetSpreadParams) -> Result<ClobGetSpreadResponse> {
        serde_json::from_value(self.0.clob_get_spread(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_spreads(&self, request: &ClobGetSpreadsParams) -> Result<std::collections::BTreeMap<String,String>> {
        serde_json::from_value(self.0.clob_get_spreads(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_tick_size(&self, request: &ClobGetTickSizeParams) -> Result<Box<ClobTickSize>> {
        serde_json::from_value(self.0.clob_get_tick_size(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_tick_size_by_path(&self, request: &ClobGetTickSizeByPathParams) -> Result<Box<ClobTickSize>> {
        serde_json::from_value(self.0.clob_get_tick_size_by_path(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_time(&self, request: &ClobGetTimeParams) -> Result<serde_json::Number> {
        serde_json::from_value(self.0.clob_get_time(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_total_earnings_for_user_for_day(&self, request: &ClobGetTotalEarningsForUserForDayParams) -> Result<Vec<ClobTotalUserEarning>> {
        serde_json::from_value(self.0.clob_get_total_earnings_for_user_for_day(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_trades(&self, request: &ClobGetTradesParams) -> Result<Box<ClobTradesResponse>> {
        serde_json::from_value(self.0.clob_get_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_update_balance_allowance(&self, request: &ClobGetUpdateBalanceAllowanceParams) -> Result<Box<ClobBalanceAllowanceResponse>> {
        serde_json::from_value(self.0.clob_get_update_balance_allowance(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_get_user_earnings_and_markets_config(&self, request: &ClobGetUserEarningsAndMarketsConfigParams) -> Result<Box<ClobPaginatedUserRewardsMarkets>> {
        serde_json::from_value(self.0.clob_get_user_earnings_and_markets_config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_list_session_signers(&self, request: &ClobListSessionSignersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.clob_list_session_signers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_post_order(&self, request: &ClobPostOrderParams) -> Result<Box<ClobSendOrderResponse>> {
        serde_json::from_value(self.0.clob_post_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_post_orders(&self, request: &ClobPostOrdersParams) -> Result<Vec<ClobSendOrderResponse>> {
        serde_json::from_value(self.0.clob_post_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_post_orders_scoring(&self, request: &ClobPostOrdersScoringParams) -> Result<Box<ClobOrdersScoringResponse>> {
        serde_json::from_value(self.0.clob_post_orders_scoring(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_revoke_builder_api_key(&self, request: &ClobRevokeBuilderApiKeyParams) -> Result<String> {
        serde_json::from_value(self.0.clob_revoke_builder_api_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_send_heartbeat(&self, request: &ClobSendHeartbeatParams) -> Result<Box<ClobHeartbeatResponse>> {
        serde_json::from_value(self.0.clob_send_heartbeat(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_send_heartbeat_v1(&self, request: &ClobSendHeartbeatV1Params) -> Result<Box<ClobHeartbeatV1Response>> {
        serde_json::from_value(self.0.clob_send_heartbeat_v1(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn clob_update_balance_allowance(&self, request: &ClobUpdateBalanceAllowanceParams) -> Result<serde_json::Map<String,serde_json::Value>> {
        serde_json::from_value(self.0.clob_update_balance_allowance(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn collateral_return_plan(&self, request: &CollateralReturnPlanParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.collateral_return_plan(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn collateral_return_submit(&self, request: &CollateralReturnSubmitParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.collateral_return_submit(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn combos_rfq_cancel_maker_quote(&self, request: &CombosRfqCancelMakerQuoteParams) -> Result<Box<CombosRfqRFQSnapshot>> {
        serde_json::from_value(self.0.combos_rfq_cancel_maker_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn combos_rfq_get_combo_markets(&self, request: &CombosRfqGetComboMarketsParams) -> Result<Box<CombosRfqComboMarketsResponse>> {
        serde_json::from_value(self.0.combos_rfq_get_combo_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn combos_rfq_submit_maker_confirmation(&self, request: &CombosRfqSubmitMakerConfirmationParams) -> Result<Box<CombosRfqMakerConfirmationResult>> {
        serde_json::from_value(self.0.combos_rfq_submit_maker_confirmation(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn combos_rfq_submit_maker_quote(&self, request: &CombosRfqSubmitMakerQuoteParams) -> Result<Box<CombosRfqRFQSnapshot>> {
        serde_json::from_value(self.0.combos_rfq_submit_maker_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_activity(&self, request: &DataGetActivityParams) -> Result<Box<DataActivityPage>> {
        serde_json::from_value(self.0.data_get_activity(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_approvals(&self, request: &DataGetApprovalsParams) -> Result<Box<DataEnvelopeApprovals>> {
        serde_json::from_value(self.0.data_get_approvals(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_biggest_winners(&self, request: &DataGetBiggestWinnersParams) -> Result<Box<DataBiggestWinnersPage>> {
        serde_json::from_value(self.0.data_get_biggest_winners(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_builders_leaderboard(&self, request: &DataGetBuildersLeaderboardParams) -> Result<Box<DataBuildersLeaderboardPage>> {
        serde_json::from_value(self.0.data_get_builders_leaderboard(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_builders_volume(&self, request: &DataGetBuildersVolumeParams) -> Result<Box<DataEnvelopeVecBuilderVolumePoint>> {
        serde_json::from_value(self.0.data_get_builders_volume(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_combo_positions(&self, request: &DataGetComboPositionsParams) -> Result<Box<DataComboPositionsPage>> {
        serde_json::from_value(self.0.data_get_combo_positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_combos(&self, request: &DataGetCombosParams) -> Result<Box<DataComboActivityPage>> {
        serde_json::from_value(self.0.data_get_combos(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_holders(&self, request: &DataGetHoldersParams) -> Result<Box<DataHoldersPage>> {
        serde_json::from_value(self.0.data_get_holders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_leaderboard(&self, request: &DataGetLeaderboardParams) -> Result<Box<DataLeaderboardResponse>> {
        serde_json::from_value(self.0.data_get_leaderboard(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_live_volume(&self, request: &DataGetLiveVolumeParams) -> Result<Box<DataEnvelopeLiveVolume>> {
        serde_json::from_value(self.0.data_get_live_volume(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_oi(&self, request: &DataGetOiParams) -> Result<Box<DataEnvelopeVecOpenInterest>> {
        serde_json::from_value(self.0.data_get_oi(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_positions(&self, request: &DataGetPositionsParams) -> Result<Box<DataPositionsPage>> {
        serde_json::from_value(self.0.data_get_positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_prices_history(&self, request: &DataGetPricesHistoryParams) -> Result<Box<DataPricesHistoryPage>> {
        serde_json::from_value(self.0.data_get_prices_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_resolutions(&self, request: &DataGetResolutionsParams) -> Result<Box<DataEnvelopeVecResolution>> {
        serde_json::from_value(self.0.data_get_resolutions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_status(&self, request: &DataGetStatusParams) -> Result<Box<DataEnvelopeServiceStatus>> {
        serde_json::from_value(self.0.data_get_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_tokens(&self, request: &DataGetTokensParams) -> Result<Box<DataEnvelopeVecTokenRef>> {
        serde_json::from_value(self.0.data_get_tokens(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_trades(&self, request: &DataGetTradesParams) -> Result<Box<DataTradesPage>> {
        serde_json::from_value(self.0.data_get_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_user_pnl_v2(&self, request: &DataGetUserPnlV2Params) -> Result<Box<DataEnvelopeUserPnlSeries>> {
        serde_json::from_value(self.0.data_get_user_pnl_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_user_stats(&self, request: &DataGetUserStatsParams) -> Result<Box<DataEnvelopeOptionUserStats>> {
        serde_json::from_value(self.0.data_get_user_stats(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_user_volume(&self, request: &DataGetUserVolumeParams) -> Result<Box<DataEnvelopeUserVolume>> {
        serde_json::from_value(self.0.data_get_user_volume(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn data_get_value(&self, request: &DataGetValueParams) -> Result<Box<DataEnvelopePortfolioValue>> {
        serde_json::from_value(self.0.data_get_value(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_abridged_markets(&self, request: &GammaGetAbridgedMarketsParams) -> Result<Vec<GammaMarket>> {
        serde_json::from_value(self.0.gamma_get_abridged_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_comments_by_id(&self, request: &GammaGetCommentsByIdParams) -> Result<Vec<GammaComment>> {
        serde_json::from_value(self.0.gamma_get_comments_by_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_comments_by_user_address(&self, request: &GammaGetCommentsByUserAddressParams) -> Result<Vec<GammaComment>> {
        serde_json::from_value(self.0.gamma_get_comments_by_user_address(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_event(&self, request: &GammaGetEventParams) -> Result<Box<GammaEvent>> {
        serde_json::from_value(self.0.gamma_get_event(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_event_by_slug(&self, request: &GammaGetEventBySlugParams) -> Result<Box<GammaEvent>> {
        serde_json::from_value(self.0.gamma_get_event_by_slug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_event_comments_count(&self, request: &GammaGetEventCommentsCountParams) -> Result<Box<GammaCount>> {
        serde_json::from_value(self.0.gamma_get_event_comments_count(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_event_creator(&self, request: &GammaGetEventCreatorParams) -> Result<Box<GammaEventCreator>> {
        serde_json::from_value(self.0.gamma_get_event_creator(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_event_tags(&self, request: &GammaGetEventTagsParams) -> Result<Vec<GammaTag>> {
        serde_json::from_value(self.0.gamma_get_event_tags(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_event_tweet_count(&self, request: &GammaGetEventTweetCountParams) -> Result<Box<GammaEventTweetCount>> {
        serde_json::from_value(self.0.gamma_get_event_tweet_count(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_gamma_status(&self, request: &GammaGetGammaStatusParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.gamma_get_gamma_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_market(&self, request: &GammaGetMarketParams) -> Result<Box<GammaMarket>> {
        serde_json::from_value(self.0.gamma_get_market(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_market_by_slug(&self, request: &GammaGetMarketBySlugParams) -> Result<Box<GammaMarket>> {
        serde_json::from_value(self.0.gamma_get_market_by_slug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_market_description(&self, request: &GammaGetMarketDescriptionParams) -> Result<Box<GammaMarketDescription>> {
        serde_json::from_value(self.0.gamma_get_market_description(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_market_tags(&self, request: &GammaGetMarketTagsParams) -> Result<Vec<GammaTag>> {
        serde_json::from_value(self.0.gamma_get_market_tags(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_markets_information(&self, request: &GammaGetMarketsInformationParams) -> Result<Vec<GammaMarket>> {
        serde_json::from_value(self.0.gamma_get_markets_information(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_public_profile(&self, request: &GammaGetPublicProfileParams) -> Result<Box<GammaPublicProfileResponse>> {
        serde_json::from_value(self.0.gamma_get_public_profile(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_public_profile_by_user_address(&self, request: &GammaGetPublicProfileByUserAddressParams) -> Result<Box<GammaProfile>> {
        serde_json::from_value(self.0.gamma_get_public_profile_by_user_address(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_related_tags_by_id(&self, request: &GammaGetRelatedTagsByIdParams) -> Result<Vec<GammaRelatedTag>> {
        serde_json::from_value(self.0.gamma_get_related_tags_by_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_related_tags_by_slug(&self, request: &GammaGetRelatedTagsBySlugParams) -> Result<Vec<GammaRelatedTag>> {
        serde_json::from_value(self.0.gamma_get_related_tags_by_slug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_series(&self, request: &GammaGetSeriesParams) -> Result<Box<GammaSeries>> {
        serde_json::from_value(self.0.gamma_get_series(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_series_comments_count(&self, request: &GammaGetSeriesCommentsCountParams) -> Result<Box<GammaCount>> {
        serde_json::from_value(self.0.gamma_get_series_comments_count(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_series_summary_by_id(&self, request: &GammaGetSeriesSummaryByIdParams) -> Result<Box<GammaSeriesSummary>> {
        serde_json::from_value(self.0.gamma_get_series_summary_by_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_series_summary_by_slug(&self, request: &GammaGetSeriesSummaryBySlugParams) -> Result<Box<GammaSeriesSummary>> {
        serde_json::from_value(self.0.gamma_get_series_summary_by_slug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_sports_market_types(&self, request: &GammaGetSportsMarketTypesParams) -> Result<Box<GammaSportsMarketTypesResponse>> {
        serde_json::from_value(self.0.gamma_get_sports_market_types(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_sports_metadata(&self, request: &GammaGetSportsMetadataParams) -> Result<Vec<GammaSportsMetadata>> {
        serde_json::from_value(self.0.gamma_get_sports_metadata(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_tag(&self, request: &GammaGetTagParams) -> Result<Box<GammaTag>> {
        serde_json::from_value(self.0.gamma_get_tag(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_tag_by_slug(&self, request: &GammaGetTagBySlugParams) -> Result<Box<GammaTag>> {
        serde_json::from_value(self.0.gamma_get_tag_by_slug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_tags_related_to_a_tag_by_id(&self, request: &GammaGetTagsRelatedToATagByIdParams) -> Result<Vec<GammaTag>> {
        serde_json::from_value(self.0.gamma_get_tags_related_to_a_tag_by_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_tags_related_to_a_tag_by_slug(&self, request: &GammaGetTagsRelatedToATagBySlugParams) -> Result<Vec<GammaTag>> {
        serde_json::from_value(self.0.gamma_get_tags_related_to_a_tag_by_slug(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_get_team(&self, request: &GammaGetTeamParams) -> Result<Box<GammaTeam>> {
        serde_json::from_value(self.0.gamma_get_team(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_list_comments(&self, request: &GammaListCommentsParams) -> Result<Vec<GammaComment>> {
        serde_json::from_value(self.0.gamma_list_comments(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_list_event_creators(&self, request: &GammaListEventCreatorsParams) -> Result<Vec<GammaEventCreator>> {
        serde_json::from_value(self.0.gamma_list_event_creators(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_list_events(&self, request: &GammaListEventsParams) -> Result<Vec<GammaEvent>> {
        serde_json::from_value(self.0.gamma_list_events(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_list_events_keyset(&self, request: &GammaListEventsKeysetParams) -> Result<Box<GammaKeysetEventsResponse>> {
        serde_json::from_value(self.0.gamma_list_events_keyset(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_list_events_pagination(&self, request: &GammaListEventsPaginationParams) -> Result<Box<GammaEventsPagination>> {
        serde_json::from_value(self.0.gamma_list_events_pagination(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_list_markets(&self, request: &GammaListMarketsParams) -> Result<Vec<GammaMarket>> {
        serde_json::from_value(self.0.gamma_list_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_list_markets_keyset(&self, request: &GammaListMarketsKeysetParams) -> Result<Box<GammaKeysetMarketsResponse>> {
        serde_json::from_value(self.0.gamma_list_markets_keyset(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_list_series(&self, request: &GammaListSeriesParams) -> Result<Vec<GammaSeries>> {
        serde_json::from_value(self.0.gamma_list_series(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_list_sport_events_results(&self, request: &GammaListSportEventsResultsParams) -> Result<Vec<GammaEvent>> {
        serde_json::from_value(self.0.gamma_list_sport_events_results(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_list_tags(&self, request: &GammaListTagsParams) -> Result<Vec<GammaTag>> {
        serde_json::from_value(self.0.gamma_list_tags(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_list_teams(&self, request: &GammaListTeamsParams) -> Result<Vec<GammaTeam>> {
        serde_json::from_value(self.0.gamma_list_teams(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn gamma_public_search(&self, request: &GammaPublicSearchParams) -> Result<Box<GammaSearch>> {
        serde_json::from_value(self.0.gamma_public_search(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn relayer_authorizations(&self, request: &RelayerAuthorizationsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.relayer_authorizations(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn relayer_get_deployed(&self, request: &RelayerGetDeployedParams) -> Result<Box<RelayerDeployedResponse>> {
        serde_json::from_value(self.0.relayer_get_deployed(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn relayer_get_execute_params(&self, request: &RelayerGetExecuteParamsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.relayer_get_execute_params(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn relayer_get_nonce(&self, request: &RelayerGetNonceParams) -> Result<Box<RelayerNonceResponse>> {
        serde_json::from_value(self.0.relayer_get_nonce(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn relayer_get_relay_payload(&self, request: &RelayerGetRelayPayloadParams) -> Result<Box<RelayerRelayPayloadResponse>> {
        serde_json::from_value(self.0.relayer_get_relay_payload(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn relayer_get_relayer_api_keys(&self, request: &RelayerGetRelayerApiKeysParams) -> Result<Vec<RelayerRelayerApiKey>> {
        serde_json::from_value(self.0.relayer_get_relayer_api_keys(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn relayer_get_transaction(&self, request: &RelayerGetTransactionParams) -> Result<Vec<RelayerRelayerTransaction>> {
        serde_json::from_value(self.0.relayer_get_transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn relayer_get_transactions(&self, request: &RelayerGetTransactionsParams) -> Result<Vec<RelayerRelayerTransaction>> {
        serde_json::from_value(self.0.relayer_get_transactions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn relayer_post_submit(&self, request: &RelayerPostSubmitParams) -> Result<Box<RelayerSubmitResponse>> {
        serde_json::from_value(self.0.relayer_post_submit(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
    pub async fn relayer_revocations(&self, request: &RelayerRevocationsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.relayer_revocations(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Polymarket response model"))
    }
}
