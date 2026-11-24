// Generated from schema/polymarket_perps.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{PolymarketPerpsClient, Error, Result};

pub type CancelChaseResponse = serde_json::Value;
pub type PerpsCancelResponse = serde_json::Value;
pub type CancelTwapResponse = serde_json::Value;
pub type ControlTwapResponse = serde_json::Value;
pub type CreateChaseResponse = serde_json::Value;
pub type PerpsOrderResponse = serde_json::Value;
pub type CreateTwapResponse = serde_json::Value;
pub type GetAssetsResponse = Vec<PerpsAsset>;
pub type GetBalancesResponse = Vec<PerpsBalance>;
pub type GetBboResponse = Vec<PerpsBBO>;
pub type GetChasesResponse = serde_json::Value;
pub type GetConfigResponse = Vec<PerpsAccountConfig>;
pub type GetInstrumentsResponse = Vec<PerpsInstrument>;
pub type GetLimitTiersResponse = Vec<PerpsLimitTier>;
pub type GetOpenOrdersResponse = Vec<PerpsOrderData>;
pub type GetOrdersResponse = Vec<PerpsOrderData>;
pub type GetStatisticsResponse = Vec<PerpsStatistic>;
pub type GetTickersResponse = Vec<PerpsTicker>;
pub type GetTwapsResponse = serde_json::Value;
pub type PerpsModifyResponse = serde_json::Value;
pub type PerpsAccount = String;
pub type PerpsAccountBackstopsCursor = String;
pub type PerpsAccountMakerShare7d = String;
pub type PerpsAccountOi = String;
pub type PerpsAccountValue = String;
pub type PerpsActionsBurst = serde_json::Number;
pub type PerpsActionsBurstLimit = serde_json::Number;
pub type PerpsActionsPerMinute = serde_json::Number;
pub type PerpsActionsPerMinuteLimit = serde_json::Number;
pub type PerpsActiveChildrenCount = serde_json::Number;
pub type PerpsAddress = String;
pub type PerpsAdl = bool;
pub type PerpsAmount = String;
pub type PerpsArts = serde_json::Number;
pub type PerpsAssetField = String;
pub type PerpsAutoCancelDailyLimit = serde_json::Number;
pub type PerpsAutoCancelDeadline = serde_json::Number;
pub type PerpsAutoCancelNextReset = serde_json::Number;
pub type PerpsAutoCancelTriggered = serde_json::Number;
pub type PerpsAvailableOrderMargin = String;
pub type PerpsBackstopCashAfter = String;
pub type PerpsBackstopCashBefore = String;
pub type PerpsBackstopTransferred = String;
pub type PerpsBalanceField = String;
pub type PerpsBaseAsset = String;
pub type PerpsBestAskPrice = String;
pub type PerpsBestAskQuantity = String;
pub type PerpsBestBidPrice = String;
pub type PerpsBestBidQuantity = String;
pub type PerpsBlp = bool;
pub type PerpsBlpActive = bool;
pub type PerpsBlpCursor = serde_json::Number;
pub type PerpsBlpSort = String;
pub type PerpsBuilderApprovalVersion = serde_json::Number;
pub type PerpsBuilderFee = String;
pub type PerpsBuilderFeeRate = String;
pub type PerpsBuy = bool;
pub type PerpsCancelOnly = bool;
pub type PerpsCancelTs = serde_json::Number;
pub type PerpsCategory = String;
pub type PerpsChainId = serde_json::Number;
pub type PerpsChildrenCount = serde_json::Number;
pub type PerpsChildrenFees = String;
pub type PerpsChildrenVolume = String;
pub type PerpsCloseOnly = bool;
pub type PerpsCode = String;
pub type PerpsCoid = String;
pub type PerpsCollateralRatio = String;
pub type PerpsCompetitionFirstFillAt = serde_json::Number;
pub type PerpsCompetitionId = serde_json::Number;
pub type PerpsCompetitionReadError = String;
pub type PerpsCompetitionReadTimestamp = serde_json::Number;
pub type PerpsCompetitionRowState = String;
pub type PerpsCompetitionScore = String;
pub type PerpsCompetitionStatus = String;
pub type PerpsConfirmations = serde_json::Number;
pub type PerpsConfirmedTimestamp = serde_json::Number;
pub type PerpsContract = String;
pub type PerpsCounterparty = String;
pub type PerpsCreatedTimestamp = serde_json::Number;
pub type PerpsCross = bool;
pub type PerpsCts = serde_json::Number;
pub type PerpsCumulativeFunding = String;
pub type PerpsCursor = String;
pub type PerpsDate = String;
pub type PerpsDecimals = serde_json::Number;
pub type PerpsDepositStatus = String;
pub type PerpsDepth = serde_json::Number;
pub type PerpsDirection = String;
pub type PerpsDisplaySymbol = String;
pub type PerpsDurableSourceSeq = serde_json::Number;
pub type PerpsEndDate = String;
pub type PerpsEndTimestamp = serde_json::Number;
pub type PerpsEngineVersion = String;
pub type PerpsEntityId = serde_json::Number;
pub type PerpsEntityMakerShare7d = String;
pub type PerpsEntityName = String;
pub type PerpsEntryPrice = String;
pub type PerpsEquity = String;
pub type PerpsError = String;
pub type PerpsExchangeFees = String;
pub type PerpsExchangeOpenInterest = String;
pub type PerpsExchangeVolume = String;
pub type PerpsExp = serde_json::Number;
pub type PerpsExpiry = serde_json::Number;
pub type PerpsFea = String;
pub type PerpsFee = String;
pub type PerpsFeeShareRate = String;
pub type PerpsFeeTierField = serde_json::Number;
pub type PerpsFid = serde_json::Number;
pub type PerpsFill = String;
pub type PerpsFillsCursor = String;
pub type PerpsFr = String;
pub type PerpsFrom = String;
pub type PerpsFua = String;
pub type PerpsFund = String;
pub type PerpsFundingInterval = String;
pub type PerpsFundingRateField = String;
pub type PerpsGenericResponse = serde_json::Value;
pub type PerpsGrp = String;
pub type PerpsHash = String;
pub type PerpsHistorySequence = serde_json::Number;
pub type PerpsIid = serde_json::Number;
pub type PerpsInLiquidation = bool;
pub type PerpsIndexPrice = String;
pub type PerpsInitialMargin = String;
pub type PerpsInstrumentId = serde_json::Number;
pub type PerpsInstrumentType = String;
pub type PerpsInterval = String;
pub type PerpsIsolatedOnly = bool;
pub type PerpsKind = String;
pub type PerpsKline = Vec<serde_json::Value>;
pub type PerpsLabel = String;
pub type PerpsLastPrice = String;
pub type PerpsLev = serde_json::Number;
pub type PerpsLevel = Vec<String>;
pub type PerpsLeverage = serde_json::Number;
pub type PerpsLeverageBatchResponse = serde_json::Value;
pub type PerpsLid = serde_json::Number;
pub type PerpsLimit = serde_json::Number;
pub type PerpsLiq = bool;
pub type PerpsLiquidationFee = String;
pub type PerpsLiquidationPrice = String;
pub type PerpsLowerBound = String;
pub type PerpsMaintenanceMarginAmount = String;
pub type PerpsMakerFeeRate = String;
pub type PerpsMakerShare7d = String;
pub type PerpsMakerVolume7d = String;
pub type PerpsMarginAmount = String;
pub type PerpsMarkPoint = Vec<serde_json::Value>;
pub type PerpsMarkPrice = String;
pub type PerpsMarket = bool;
pub type PerpsMaxLeverage = serde_json::Number;
pub type PerpsMaxLimitNotional = String;
pub type PerpsMaxMarketNotional = String;
pub type PerpsMaxOrderCount = serde_json::Number;
pub type PerpsMessagesPerMinute = serde_json::Number;
pub type PerpsMgn = String;
pub type PerpsMidPrice = String;
pub type PerpsMinNotional = String;
pub type PerpsMinVolume = String;
pub type PerpsMinVolume30d = String;
pub type PerpsMore = bool;
pub type PerpsName = String;
pub type PerpsNextFunding = serde_json::Number;
pub type PerpsNotifCursor = String;
pub type PerpsNotifId = String;
pub type PerpsNotificationsReadRequest = serde_json::Value;
pub type PerpsNotional = String;
pub type PerpsOffset = serde_json::Number;
pub type PerpsOid = serde_json::Number;
pub type PerpsOpAmount = String;
pub type PerpsOpenInterest = String;
pub type PerpsOpenInterestTimestamp = serde_json::Number;
pub type PerpsOpenOrders = serde_json::Number;
pub type PerpsOpenOrdersLimit = serde_json::Number;
pub type PerpsOpenPrice = String;
pub type PerpsOrderId = serde_json::Number;
pub type PerpsOwner = String;
pub type PerpsP = String;
pub type PerpsParentOid = serde_json::Number;
pub type PerpsPep = String;
pub type PerpsPnl = String;
pub type PerpsPnlInterval = String;
pub type PerpsPo = bool;
pub type PerpsPositionSnapshotStatus = String;
pub type PerpsPositionValue = String;
pub type PerpsPrice = String;
pub type PerpsPriceBounds = String;
pub type PerpsPriceDecimals = serde_json::Number;
pub type PerpsProxy = String;
pub type PerpsPsz = String;
pub type PerpsQty = String;
pub type PerpsQuantity = String;
pub type PerpsQuantityDecimals = serde_json::Number;
pub type PerpsQuoteAsset = String;
pub type PerpsRank = serde_json::Number;
pub type PerpsRateBurstLimit = serde_json::Number;
pub type PerpsRatePerMinuteLimit = serde_json::Number;
pub type PerpsRateReset = serde_json::Number;
pub type PerpsReadAt = serde_json::Number;
pub type PerpsRef = String;
pub type PerpsReferralsLeft = serde_json::Number;
pub type PerpsRequiredConfirmations = serde_json::Number;
pub type PerpsRest = String;
pub type PerpsReturnOnEquity = String;
pub type PerpsRewardAmount = String;
pub type PerpsRewardDistributed = bool;
pub type PerpsRewardPool = String;
pub type PerpsRo = bool;
pub type PerpsRoi = String;
pub type PerpsSalt = serde_json::Number;
pub type PerpsSecret = String;
pub type PerpsSequence = serde_json::Number;
pub type PerpsSettlementFill = bool;
pub type PerpsSettlementInsuranceDebit = String;
pub type PerpsSettlementPrice = String;
pub type PerpsSettlementSequence = serde_json::Number;
pub type PerpsSettlementTimestamp = serde_json::Number;
pub type PerpsSide = String;
pub type PerpsSig = String;
pub type PerpsSinceSeq = serde_json::Number;
pub type PerpsSize = String;
pub type PerpsSlipBps = serde_json::Number;
pub type PerpsSort = String;
pub type PerpsSortBy = String;
pub type PerpsSource = String;
pub type PerpsSt = String;
pub type PerpsStartDate = String;
pub type PerpsStartTimestamp = serde_json::Number;
pub type PerpsSymbol = String;
pub type PerpsSz = String;
pub type PerpsTaker = bool;
pub type PerpsTakerFeeRate = String;
pub type PerpsTakerVolume7d = String;
pub type PerpsTid = serde_json::Number;
pub type PerpsTif = String;
pub type PerpsTimeField = serde_json::Number;
pub type PerpsTimestamp = serde_json::Number;
pub type PerpsTo = String;
pub type PerpsToken = String;
pub type PerpsTotal = serde_json::Number;
pub type PerpsTotalAccountValue = String;
pub type PerpsTotalFee = String;
pub type PerpsTotalInitialMargin = String;
pub type PerpsTotalMaintenanceMargin = String;
pub type PerpsTotalPositionValue = String;
pub type PerpsTpsl = String;
pub type PerpsTrScope = String;
pub type PerpsTransferId = serde_json::Number;
pub type PerpsTrp = String;
pub type PerpsTs = serde_json::Number;
pub type PerpsUiLiveTime = serde_json::Number;
pub type PerpsUnread = serde_json::Number;
pub type PerpsUnrealizedPnl = String;
pub type PerpsUpdateTimestamp = serde_json::Number;
pub type PerpsUts = serde_json::Number;
pub type PerpsValue = String;
pub type PerpsVersion = String;
pub type PerpsVolume = String;
pub type PerpsVolume7d = String;
pub type PerpsWeight = String;
pub type PerpsWindow = String;
pub type PerpsWithdrawId = serde_json::Number;
pub type PerpsWithdrawable = String;
pub type PerpsWithdrawalFee = String;
pub type PerpsWithdrawalStatus = String;
pub type UpdateMarginResponse = serde_json::Value;
pub type CancelOrdersByCoidResponse = Vec<PerpsCancelResponse>;
pub type CancelOrdersResponse = Vec<PerpsCancelResponse>;
pub type CreateOrdersResponse = Vec<PerpsOrderResponse>;
pub type ModifyOrdersByCoidResponse = Vec<PerpsModifyResponse>;
pub type ModifyOrdersResponse = Vec<PerpsModifyResponse>;
pub type PerpsKlines = Vec<PerpsKline>;
pub type UpdateLeveragesResponse = Vec<PerpsLeverageBatchResponse>;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApplyReferralParams {
    #[serde(rename = "body")]
    pub body: Box<PerpsAccountReferralRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApplyReferralResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApproveBuilderParams {
    #[serde(rename = "body")]
    pub body: Box<PerpsBuilderApprovalRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApproveBuilderResponse {
    #[serde(rename = "trader")]
    pub trader: String,
    #[serde(rename = "builder")]
    pub builder: String,
    #[serde(rename = "max_fee_rate")]
    pub max_fee_rate: String,
    #[serde(rename = "approval_version")]
    pub approval_version: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "sequence")]
    pub sequence: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ApproveBuilderResponse {
    fn default() -> Self {
        Self {
            trader: Default::default(),
            builder: Default::default(),
            max_fee_rate: Default::default(),
            approval_version: 0.into(),
            timestamp: 0.into(),
            sequence: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAllOrdersParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsCancelAllRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAllOrdersResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelChaseParams {
    #[serde(rename = "body")]
    pub body: CancelChaseParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelChaseParamsBody {
    #[serde(rename = "op")]
    pub op: CancelChaseParamsBodyOp,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(rename = "exp", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelChaseParamsBodyOp {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: CancelChaseParamsBodyOpArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelChaseParamsBodyOpArgs {
    #[serde(rename = "chid")]
    pub chid: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelChaseParamsBodyOpArgs {
    fn default() -> Self {
        Self {
            chid: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrdersByCoidParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsCancelByCoidRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrdersParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsCancelRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelTwapParams {
    #[serde(rename = "body")]
    pub body: CancelTwapParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelTwapParamsBody {
    #[serde(rename = "op")]
    pub op: CancelTwapParamsBodyOp,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(rename = "exp", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelTwapParamsBodyOp {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: CancelTwapParamsBodyOpArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelTwapParamsBodyOpArgs {
    #[serde(rename = "twid")]
    pub twid: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelTwapParamsBodyOpArgs {
    fn default() -> Self {
        Self {
            twid: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CheckInviteCodeParams {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CheckInviteCodeResponse {
    #[serde(rename = "valid")]
    pub valid: bool,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ControlTwapParams {
    #[serde(rename = "body")]
    pub body: ControlTwapParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ControlTwapParamsBody {
    #[serde(rename = "op")]
    pub op: ControlTwapParamsBodyOp,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(rename = "exp", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ControlTwapParamsBodyOp {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: ControlTwapParamsBodyOpArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ControlTwapParamsBodyOpArgs {
    #[serde(rename = "twid")]
    pub twid: serde_json::Number,
    #[serde(rename = "act")]
    pub act: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ControlTwapParamsBodyOpArgs {
    fn default() -> Self {
        Self {
            twid: 0.into(),
            act: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateAccountInviteParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsCreateInviteRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateAccountInviteResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "referrals_available")]
    pub referrals_available: serde_json::Number,
    #[serde(rename = "cooldown_ms", skip_serializing_if = "Option::is_none")]
    pub cooldown_ms: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateAccountInviteResponse {
    fn default() -> Self {
        Self {
            status: Default::default(),
            code: Default::default(),
            referrals_available: 0.into(),
            cooldown_ms: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateChaseParams {
    #[serde(rename = "body")]
    pub body: CreateChaseParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateChaseParamsBody {
    #[serde(rename = "op")]
    pub op: CreateChaseParamsBodyOp,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(rename = "exp", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateChaseParamsBodyOp {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: CreateChaseParamsBodyOpArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateChaseParamsBodyOpArgs {
    #[serde(rename = "iid")]
    pub iid: serde_json::Number,
    #[serde(rename = "buy")]
    pub buy: bool,
    #[serde(rename = "qty")]
    pub qty: String,
    #[serde(rename = "lim", default, skip_serializing_if = "Option::is_none")]
    pub lim: Option<String>,
    #[serde(rename = "max_dist", default, skip_serializing_if = "Option::is_none")]
    pub max_dist: Option<String>,
    #[serde(rename = "max_dist_bps", default, skip_serializing_if = "Option::is_none")]
    pub max_dist_bps: Option<serde_json::Number>,
    #[serde(rename = "po", default, skip_serializing_if = "Option::is_none")]
    pub po: Option<bool>,
    #[serde(rename = "ro", default, skip_serializing_if = "Option::is_none")]
    pub ro: Option<bool>,
    #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateChaseParamsBodyOpArgs {
    fn default() -> Self {
        Self {
            iid: 0.into(),
            buy: Default::default(),
            qty: Default::default(),
            lim: Default::default(),
            max_dist: Default::default(),
            max_dist_bps: Default::default(),
            po: Default::default(),
            ro: Default::default(),
            c: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateOrdersParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsOrderRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateProxyParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsProxyRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateProxyResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateTwapParams {
    #[serde(rename = "body")]
    pub body: CreateTwapParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateTwapParamsBody {
    #[serde(rename = "op")]
    pub op: CreateTwapParamsBodyOp,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(rename = "exp", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateTwapParamsBodyOp {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: CreateTwapParamsBodyOpArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateTwapParamsBodyOpArgs {
    #[serde(rename = "iid")]
    pub iid: serde_json::Number,
    #[serde(rename = "buy")]
    pub buy: bool,
    #[serde(rename = "qty")]
    pub qty: String,
    #[serde(rename = "dur", default, skip_serializing_if = "Option::is_none")]
    pub dur: Option<serde_json::Number>,
    #[serde(rename = "ivl", default, skip_serializing_if = "Option::is_none")]
    pub ivl: Option<serde_json::Number>,
    #[serde(rename = "rnd", default, skip_serializing_if = "Option::is_none")]
    pub rnd: Option<bool>,
    #[serde(rename = "slip_bps", default, skip_serializing_if = "Option::is_none")]
    pub slip_bps: Option<serde_json::Number>,
    #[serde(rename = "min_px", default, skip_serializing_if = "Option::is_none")]
    pub min_px: Option<String>,
    #[serde(rename = "max_px", default, skip_serializing_if = "Option::is_none")]
    pub max_px: Option<String>,
    #[serde(rename = "ro", default, skip_serializing_if = "Option::is_none")]
    pub ro: Option<bool>,
    #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateTwapParamsBodyOpArgs {
    fn default() -> Self {
        Self {
            iid: 0.into(),
            buy: Default::default(),
            qty: Default::default(),
            dur: Default::default(),
            ivl: Default::default(),
            rnd: Default::default(),
            slip_bps: Default::default(),
            min_px: Default::default(),
            max_px: Default::default(),
            ro: Default::default(),
            c: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteProxyParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsDeleteProxyRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteProxyResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountBackstopsParams {
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountBackstopsResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsAccountBackstop>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(rename = "coverage", default, skip_serializing_if = "Option::is_none")]
    pub coverage: Option<GetAccountBackstopsResponseCoverage>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountBackstopsResponseCoverage {
    #[serde(rename = "notes")]
    pub notes: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountFundingParams {
    #[serde(rename = "instrument_id", default, skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<serde_json::Number>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountFundingResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsAccountFundingData>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountLimitsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetAccountLimitsResponse {
    #[serde(rename = "actions_per_minute")]
    pub actions_per_minute: serde_json::Number,
    #[serde(rename = "actions_per_minute_limit")]
    pub actions_per_minute_limit: serde_json::Number,
    #[serde(rename = "actions_burst")]
    pub actions_burst: serde_json::Number,
    #[serde(rename = "actions_burst_limit")]
    pub actions_burst_limit: serde_json::Number,
    #[serde(rename = "open_orders")]
    pub open_orders: serde_json::Number,
    #[serde(rename = "open_orders_limit")]
    pub open_orders_limit: serde_json::Number,
    #[serde(rename = "reset")]
    pub reset: serde_json::Number,
    #[serde(rename = "messages_per_minute")]
    pub messages_per_minute: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetAccountLimitsResponse {
    fn default() -> Self {
        Self {
            actions_per_minute: 0.into(),
            actions_per_minute_limit: 0.into(),
            actions_burst: 0.into(),
            actions_burst_limit: 0.into(),
            open_orders: 0.into(),
            open_orders_limit: 0.into(),
            reset: 0.into(),
            messages_per_minute: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountNotificationsParams {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "since_seq", default, skip_serializing_if = "Option::is_none")]
    pub since_seq: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetAccountNotificationsResponse {
    #[serde(rename = "items")]
    pub items: Vec<PerpsNotificationItem>,
    #[serde(rename = "unread")]
    pub unread: serde_json::Number,
    #[serde(rename = "durable_source_seq")]
    pub durable_source_seq: serde_json::Number,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "next_cursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetAccountNotificationsResponse {
    fn default() -> Self {
        Self {
            items: Default::default(),
            unread: 0.into(),
            durable_source_seq: 0.into(),
            has_more: Default::default(),
            next_cursor: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountReferralParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetAccountReferralResponse {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "parent", default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "children_count")]
    pub children_count: serde_json::Number,
    #[serde(rename = "active_children_count")]
    pub active_children_count: serde_json::Number,
    #[serde(rename = "children_volume")]
    pub children_volume: String,
    #[serde(rename = "children_fees")]
    pub children_fees: String,
    #[serde(rename = "referrals_left")]
    pub referrals_left: serde_json::Number,
    #[serde(rename = "fee_share_rate")]
    pub fee_share_rate: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetAccountReferralResponse {
    fn default() -> Self {
        Self {
            code: Default::default(),
            parent: Default::default(),
            children_count: 0.into(),
            active_children_count: 0.into(),
            children_volume: Default::default(),
            children_fees: Default::default(),
            referrals_left: 0.into(),
            fee_share_rate: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountRewardsParams {
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "start_date", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "end_date", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountRewardsResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsAccountReward>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountStatsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountStatsResponse {
    #[serde(rename = "volume_7d")]
    pub volume_7d: String,
    #[serde(rename = "taker_volume_7d")]
    pub taker_volume_7d: String,
    #[serde(rename = "maker_volume_7d")]
    pub maker_volume_7d: String,
    #[serde(rename = "account_maker_share_7d")]
    pub account_maker_share_7d: String,
    #[serde(rename = "entity_maker_share_7d", default, skip_serializing_if = "Option::is_none")]
    pub entity_maker_share_7d: Option<String>,
    #[serde(rename = "entity_id", default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<serde_json::Number>,
    #[serde(rename = "entity_name", default, skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAssetsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAutoCancelParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetAutoCancelResponse {
    #[serde(rename = "deadline")]
    pub deadline: serde_json::Number,
    #[serde(rename = "triggered")]
    pub triggered: serde_json::Number,
    #[serde(rename = "daily_limit")]
    pub daily_limit: serde_json::Number,
    #[serde(rename = "next_reset")]
    pub next_reset: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetAutoCancelResponse {
    fn default() -> Self {
        Self {
            deadline: 0.into(),
            triggered: 0.into(),
            daily_limit: 0.into(),
            next_reset: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBalancesParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBboParams {
    #[serde(rename = "instrument_id", default, skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBlpEnrollmentParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBlpEnrollmentResponse {
    #[serde(rename = "blp")]
    pub blp: bool,
    #[serde(rename = "instruments")]
    pub instruments: Vec<PerpsBlpInstrument>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBlpLiquidationsParams {
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<serde_json::Number>,
    #[serde(rename = "sort", default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBlpLiquidationsResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsBlpLiquidation>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetBookParams {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "depth", default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetBookParams {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            depth: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetBookResponse {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "bids")]
    pub bids: Vec<PerpsLevel>,
    #[serde(rename = "asks")]
    pub asks: Vec<PerpsLevel>,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "sequence")]
    pub sequence: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetBookResponse {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            bids: Default::default(),
            asks: Default::default(),
            timestamp: 0.into(),
            sequence: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBuilderApprovalsParams {
    #[serde(rename = "builder", default, skip_serializing_if = "Option::is_none")]
    pub builder: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBuilderApprovalsResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsBuilderApproval>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBuilderEarningsParams {
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "as_of_sequence", default, skip_serializing_if = "Option::is_none")]
    pub as_of_sequence: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetBuilderEarningsResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsBuilderEarning>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: serde_json::Number,
    #[serde(rename = "as_of_sequence")]
    pub as_of_sequence: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetBuilderEarningsResponse {
    fn default() -> Self {
        Self {
            data: Default::default(),
            more: Default::default(),
            cursor: Default::default(),
            start_timestamp: 0.into(),
            end_timestamp: 0.into(),
            as_of_sequence: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBuilderEarningsSummaryParams {
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "as_of_sequence", default, skip_serializing_if = "Option::is_none")]
    pub as_of_sequence: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetBuilderEarningsSummaryResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsBuilderEarningsAsset>,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: serde_json::Number,
    #[serde(rename = "as_of_sequence")]
    pub as_of_sequence: serde_json::Number,
    #[serde(rename = "trader_count")]
    pub trader_count: serde_json::Number,
    #[serde(rename = "active_approval_count")]
    pub active_approval_count: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetBuilderEarningsSummaryResponse {
    fn default() -> Self {
        Self {
            data: Default::default(),
            start_timestamp: 0.into(),
            end_timestamp: 0.into(),
            as_of_sequence: 0.into(),
            trader_count: 0.into(),
            active_approval_count: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBuilderParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBuilderResponse {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "registered")]
    pub registered: bool,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "admission_enabled")]
    pub admission_enabled: bool,
    #[serde(rename = "max_fee_rate")]
    pub max_fee_rate: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetChasesParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetCompetitionAccountParams {
    #[serde(rename = "competition_id")]
    pub competition_id: serde_json::Number,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetCompetitionAccountParams {
    fn default() -> Self {
        Self {
            competition_id: 0.into(),
            address: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetCompetitionAccountResponse {
    #[serde(rename = "competition_id")]
    pub competition_id: serde_json::Number,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "updated_at", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<serde_json::Number>,
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "rank", default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<serde_json::Number>,
    #[serde(rename = "qualified")]
    pub qualified: bool,
    #[serde(rename = "score")]
    pub score: String,
    #[serde(rename = "pnl")]
    pub pnl: String,
    #[serde(rename = "realized_pnl")]
    pub realized_pnl: String,
    #[serde(rename = "unrealized_pnl")]
    pub unrealized_pnl: String,
    #[serde(rename = "fees")]
    pub fees: String,
    #[serde(rename = "funding")]
    pub funding: String,
    #[serde(rename = "volume")]
    pub volume: String,
    #[serde(rename = "boosted_volume")]
    pub boosted_volume: String,
    #[serde(rename = "first_fill_at", default, skip_serializing_if = "Option::is_none")]
    pub first_fill_at: Option<serde_json::Number>,
    #[serde(rename = "min_volume")]
    pub min_volume: String,
    #[serde(rename = "volume_to_qualify")]
    pub volume_to_qualify: String,
    #[serde(rename = "qualification_progress")]
    pub qualification_progress: String,
    #[serde(rename = "prize", default, skip_serializing_if = "Option::is_none")]
    pub prize: Option<String>,
    #[serde(rename = "spots_from_prize_zone", default, skip_serializing_if = "Option::is_none")]
    pub spots_from_prize_zone: Option<serde_json::Number>,
    #[serde(rename = "score_to_prize_zone", default, skip_serializing_if = "Option::is_none")]
    pub score_to_prize_zone: Option<String>,
    #[serde(rename = "row_state")]
    pub row_state: String,
    #[serde(rename = "payout_tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub payout_tx_hash: Option<String>,
    #[serde(rename = "paid_at", default, skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<serde_json::Number>,
    #[serde(rename = "locked_positions")]
    pub locked_positions: Vec<PerpsCompetitionLockedPosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetCompetitionAccountResponse {
    fn default() -> Self {
        Self {
            competition_id: 0.into(),
            status: Default::default(),
            updated_at: Default::default(),
            account: Default::default(),
            rank: Default::default(),
            qualified: Default::default(),
            score: Default::default(),
            pnl: Default::default(),
            realized_pnl: Default::default(),
            unrealized_pnl: Default::default(),
            fees: Default::default(),
            funding: Default::default(),
            volume: Default::default(),
            boosted_volume: Default::default(),
            first_fill_at: Default::default(),
            min_volume: Default::default(),
            volume_to_qualify: Default::default(),
            qualification_progress: Default::default(),
            prize: Default::default(),
            spots_from_prize_zone: Default::default(),
            score_to_prize_zone: Default::default(),
            row_state: Default::default(),
            payout_tx_hash: Default::default(),
            paid_at: Default::default(),
            locked_positions: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetCompetitionLeaderboardParams {
    #[serde(rename = "competition_id")]
    pub competition_id: serde_json::Number,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetCompetitionLeaderboardParams {
    fn default() -> Self {
        Self {
            competition_id: 0.into(),
            limit: Default::default(),
            offset: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetCompetitionLeaderboardResponse {
    #[serde(rename = "competition_id")]
    pub competition_id: serde_json::Number,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "updated_at", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<serde_json::Number>,
    #[serde(rename = "total")]
    pub total: serde_json::Number,
    #[serde(rename = "offset")]
    pub offset: serde_json::Number,
    #[serde(rename = "final_results")]
    pub final_results: bool,
    #[serde(rename = "entries")]
    pub entries: Vec<PerpsCompetitionEntry>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetCompetitionLeaderboardResponse {
    fn default() -> Self {
        Self {
            competition_id: 0.into(),
            status: Default::default(),
            updated_at: Default::default(),
            total: 0.into(),
            offset: 0.into(),
            final_results: Default::default(),
            entries: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetCompetitionParams {
    #[serde(rename = "competition_id")]
    pub competition_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetCompetitionParams {
    fn default() -> Self {
        Self {
            competition_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetCompetitionResponse {
    #[serde(rename = "competition_id")]
    pub competition_id: serde_json::Number,
    #[serde(rename = "number")]
    pub number: serde_json::Number,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "sector_label")]
    pub sector_label: String,
    #[serde(rename = "boost_description")]
    pub boost_description: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "start_time")]
    pub start_time: serde_json::Number,
    #[serde(rename = "end_time")]
    pub end_time: serde_json::Number,
    #[serde(rename = "all_markets")]
    pub all_markets: bool,
    #[serde(rename = "markets")]
    pub markets: Vec<PerpsCompetitionMarket>,
    #[serde(rename = "min_volume")]
    pub min_volume: String,
    #[serde(rename = "count_unrealized")]
    pub count_unrealized: bool,
    #[serde(rename = "prize_pool")]
    pub prize_pool: String,
    #[serde(rename = "prize_asset_id")]
    pub prize_asset_id: serde_json::Number,
    #[serde(rename = "prize_tiers")]
    pub prize_tiers: Vec<PerpsCompetitionPrizeTier>,
    #[serde(rename = "paid_places")]
    pub paid_places: serde_json::Number,
    #[serde(rename = "internal")]
    pub internal: bool,
    #[serde(rename = "start_trading_instrument_id", default, skip_serializing_if = "Option::is_none")]
    pub start_trading_instrument_id: Option<serde_json::Number>,
    #[serde(rename = "updated_at", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<serde_json::Number>,
    #[serde(rename = "ranked")]
    pub ranked: serde_json::Number,
    #[serde(rename = "participants")]
    pub participants: serde_json::Number,
    #[serde(rename = "prize_cutoff_score", default, skip_serializing_if = "Option::is_none")]
    pub prize_cutoff_score: Option<String>,
    #[serde(rename = "final_results")]
    pub final_results: bool,
    #[serde(rename = "scoring_version")]
    pub scoring_version: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetCompetitionResponse {
    fn default() -> Self {
        Self {
            competition_id: 0.into(),
            number: 0.into(),
            name: Default::default(),
            sector_label: Default::default(),
            boost_description: Default::default(),
            status: Default::default(),
            start_time: 0.into(),
            end_time: 0.into(),
            all_markets: Default::default(),
            markets: Default::default(),
            min_volume: Default::default(),
            count_unrealized: Default::default(),
            prize_pool: Default::default(),
            prize_asset_id: 0.into(),
            prize_tiers: Default::default(),
            paid_places: 0.into(),
            internal: Default::default(),
            start_trading_instrument_id: Default::default(),
            updated_at: Default::default(),
            ranked: 0.into(),
            participants: 0.into(),
            prize_cutoff_score: Default::default(),
            final_results: Default::default(),
            scoring_version: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCompetitionWidgetParams {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetCompetitionWidgetResponse {
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "competition_id", default, skip_serializing_if = "Option::is_none")]
    pub competition_id: Option<serde_json::Number>,
    #[serde(rename = "number", default, skip_serializing_if = "Option::is_none")]
    pub number: Option<serde_json::Number>,
    #[serde(rename = "end_time", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<serde_json::Number>,
    #[serde(rename = "rank", default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<serde_json::Number>,
    #[serde(rename = "qualified", default, skip_serializing_if = "Option::is_none")]
    pub qualified: Option<bool>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(rename = "min_volume", default, skip_serializing_if = "Option::is_none")]
    pub min_volume: Option<String>,
    #[serde(rename = "qualification_progress", default, skip_serializing_if = "Option::is_none")]
    pub qualification_progress: Option<String>,
    #[serde(rename = "spots_from_prize_zone", default, skip_serializing_if = "Option::is_none")]
    pub spots_from_prize_zone: Option<serde_json::Number>,
    #[serde(rename = "updated_at", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetCompetitionWidgetResponse {
    fn default() -> Self {
        Self {
            timestamp: 0.into(),
            competition_id: Default::default(),
            number: Default::default(),
            end_time: Default::default(),
            rank: Default::default(),
            qualified: Default::default(),
            volume: Default::default(),
            min_volume: Default::default(),
            qualification_progress: Default::default(),
            spots_from_prize_zone: Default::default(),
            updated_at: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCompetitionsParams {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetCompetitionsResponse {
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "live_competition_id", default, skip_serializing_if = "Option::is_none")]
    pub live_competition_id: Option<serde_json::Number>,
    #[serde(rename = "landing_competition_id", default, skip_serializing_if = "Option::is_none")]
    pub landing_competition_id: Option<serde_json::Number>,
    #[serde(rename = "competitions")]
    pub competitions: Vec<PerpsCompetitionSummary>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetCompetitionsResponse {
    fn default() -> Self {
        Self {
            timestamp: 0.into(),
            live_competition_id: Default::default(),
            landing_competition_id: Default::default(),
            competitions: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetConfigParams {
    #[serde(rename = "instrument_id", default, skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCredentialsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCredentialsResponse {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "keys")]
    pub keys: Vec<PerpsKey>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetDepositsParams {
    #[serde(rename = "deposit_status", default, skip_serializing_if = "Option::is_none")]
    pub deposit_status: Option<String>,
    #[serde(rename = "hash", default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetDepositsResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsDeposit>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetEquityHistoryParams {
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetEquityHistoryParams {
    fn default() -> Self {
        Self {
            interval: Default::default(),
            start_timestamp: 0.into(),
            end_timestamp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEquityHistoryResponse {
    #[serde(rename = "data")]
    pub data: Vec<Vec<serde_json::Value>>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetExchangeParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetExchangeResponse {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "chain_id")]
    pub chain_id: serde_json::Number,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "cancel_only")]
    pub cancel_only: bool,
    #[serde(rename = "engine_version")]
    pub engine_version: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetExchangeResponse {
    fn default() -> Self {
        Self {
            name: Default::default(),
            version: Default::default(),
            chain_id: 0.into(),
            contract: Default::default(),
            cancel_only: Default::default(),
            engine_version: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetExchangeStatisticsParams {
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetExchangeStatisticsParams {
    fn default() -> Self {
        Self {
            start_timestamp: 0.into(),
            end_timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetExchangeStatisticsResponse {
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: serde_json::Number,
    #[serde(rename = "volume")]
    pub volume: String,
    #[serde(rename = "open_interest", skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<String>,
    #[serde(rename = "open_interest_timestamp", skip_serializing_if = "Option::is_none")]
    pub open_interest_timestamp: Option<serde_json::Number>,
    #[serde(rename = "fees")]
    pub fees: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetExchangeStatisticsResponse {
    fn default() -> Self {
        Self {
            start_timestamp: 0.into(),
            end_timestamp: 0.into(),
            volume: Default::default(),
            open_interest: Default::default(),
            open_interest_timestamp: Default::default(),
            fees: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFillsParams {
    #[serde(rename = "instrument_id", default, skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<serde_json::Number>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "sort", default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFillsResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsAccountTradeData>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetFundingHistoryParams {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetFundingHistoryParams {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            start_timestamp: Default::default(),
            end_timestamp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFundingHistoryResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsFundingRate>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetIndexParams {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetIndexResponse {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "index_price")]
    pub index_price: String,
    #[serde(rename = "constituents")]
    pub constituents: Vec<PerpsIndexConstituent>,
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetIndexResponse {
    fn default() -> Self {
        Self {
            asset: Default::default(),
            index_price: Default::default(),
            constituents: Default::default(),
            ts: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInfoFeesParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInfoFeesResponse {
    #[serde(rename = "fee_schedule")]
    pub fee_schedule: Vec<PerpsFeeScheduleEntry>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInstrumentsParams {
    #[serde(rename = "instrument_id", default, skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<serde_json::Number>,
    #[serde(rename = "instrument_type", default, skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<String>,
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInternalTransfersParams {
    #[serde(rename = "direction", default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetInternalTransfersResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsInternalTransfer>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetKlinesParams {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetKlinesParams {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            interval: Default::default(),
            start_timestamp: 0.into(),
            end_timestamp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetKlinesResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsKline>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLeaderboardParams {
    #[serde(rename = "window", default, skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Number>,
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetLeaderboardResponse {
    #[serde(rename = "window")]
    pub window: String,
    #[serde(rename = "sort_by")]
    pub sort_by: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "total")]
    pub total: serde_json::Number,
    #[serde(rename = "entries")]
    pub entries: Vec<PerpsLeaderboardEntry>,
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<Box<PerpsLeaderboardAccount>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetLeaderboardResponse {
    fn default() -> Self {
        Self {
            window: Default::default(),
            sort_by: Default::default(),
            timestamp: 0.into(),
            total: 0.into(),
            entries: Default::default(),
            account: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLimitTiersParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMarkHistoryParams {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetMarkHistoryParams {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            interval: Default::default(),
            start_timestamp: 0.into(),
            end_timestamp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarkHistoryResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsMarkPoint>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOpenOrdersParams {
    #[serde(rename = "instrument_id", default, skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrdersParams {
    #[serde(rename = "order_id", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<serde_json::Number>,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "instrument_id", default, skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<serde_json::Number>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPingParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPingResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPnlHistoryParams {
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetPnlHistoryParams {
    fn default() -> Self {
        Self {
            interval: Default::default(),
            start_timestamp: 0.into(),
            end_timestamp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPnlHistoryResponse {
    #[serde(rename = "data")]
    pub data: Vec<Vec<serde_json::Value>>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPortfolioParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPortfolioResponse {
    #[serde(rename = "positions")]
    pub positions: Vec<PerpsPortfolioPosition>,
    #[serde(rename = "margin")]
    pub margin: Box<PerpsMarginSummary>,
    #[serde(rename = "withdrawable")]
    pub withdrawable: String,
    #[serde(rename = "in_liquidation")]
    pub in_liquidation: bool,
    #[serde(rename = "fee_tier")]
    pub fee_tier: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetPortfolioResponse {
    fn default() -> Self {
        Self {
            positions: Default::default(),
            margin: Default::default(),
            withdrawable: Default::default(),
            in_liquidation: Default::default(),
            fee_tier: 0.into(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPublicPortfolioParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPublicPortfolioResponse {
    #[serde(rename = "positions")]
    pub positions: Vec<PerpsPublicPortfolioPosition>,
    #[serde(rename = "equity")]
    pub equity: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetPublicPortfolioResponse {
    fn default() -> Self {
        Self {
            positions: Default::default(),
            equity: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPublicPositionFillsParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "sort", default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetPublicPositionFillsParams {
    fn default() -> Self {
        Self {
            address: Default::default(),
            instrument_id: 0.into(),
            cursor: Default::default(),
            sort: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPublicPositionFillsResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsAccountTradeData>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPublicPositionSnapshotsParams {
    #[serde(rename = "body")]
    pub body: Box<PerpsPositionSnapshotsRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPublicPositionSnapshotsResponse {
    #[serde(rename = "history_as_of_at")]
    pub history_as_of_at: serde_json::Number,
    #[serde(rename = "active")]
    pub active: Vec<PerpsPositionSnapshotResult>,
    #[serde(rename = "history")]
    pub history: Vec<PerpsPositionSnapshotResult>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetPublicPositionSnapshotsResponse {
    fn default() -> Self {
        Self {
            history_as_of_at: 0.into(),
            active: Default::default(),
            history: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRegisteredParams {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRegisteredResponse {
    #[serde(rename = "registered")]
    pub registered: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetStatisticsParams {
    #[serde(rename = "instrument_id", default, skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTickersParams {
    #[serde(rename = "instrument_id", default, skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTimeParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetTimeResponse {
    #[serde(rename = "time")]
    pub time: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetTimeResponse {
    fn default() -> Self {
        Self {
            time: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetTradesParams {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetTradesParams {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            start_timestamp: Default::default(),
            end_timestamp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTradesResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsTradeData>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTwapsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWithdrawalsParams {
    #[serde(rename = "withdrawal_status", default, skip_serializing_if = "Option::is_none")]
    pub withdrawal_status: Option<String>,
    #[serde(rename = "hash", default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "start_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<serde_json::Number>,
    #[serde(rename = "end_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWithdrawalsResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsWithdrawal>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct InternalTransferParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsInternalTransferRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InternalTransferResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "transfer_id")]
    pub transfer_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for InternalTransferResponse {
    fn default() -> Self {
        Self {
            status: Default::default(),
            transfer_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarkAccountNotificationsReadParams {
    #[serde(rename = "body")]
    pub body: Box<PerpsNotificationsReadRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarkAccountNotificationsReadResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ModifyOrdersByCoidParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsModifyByCoidRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ModifyOrdersParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsModifyRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsAccountBackstop {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(rename = "cross")]
    pub cross: bool,
    #[serde(rename = "instrument_id", skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<serde_json::Number>,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "transferred")]
    pub transferred: String,
    #[serde(rename = "cash_before")]
    pub cash_before: String,
    #[serde(rename = "cash_after")]
    pub cash_after: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "sequence")]
    pub sequence: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsAccountBackstop {
    fn default() -> Self {
        Self {
            id: 0.into(),
            cross: Default::default(),
            instrument_id: Default::default(),
            asset: Default::default(),
            transferred: Default::default(),
            cash_before: Default::default(),
            cash_after: Default::default(),
            timestamp: 0.into(),
            sequence: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsAccountBackstops {
    #[serde(rename = "data")]
    pub data: Vec<PerpsAccountBackstop>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(rename = "coverage", default, skip_serializing_if = "Option::is_none")]
    pub coverage: Option<PerpsAccountBackstopsCoverage>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsAccountBackstopsCoverage {
    #[serde(rename = "notes")]
    pub notes: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsAccountConfig {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "leverage")]
    pub leverage: serde_json::Number,
    #[serde(rename = "cross")]
    pub cross: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsAccountConfig {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            leverage: 0.into(),
            cross: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsAccountFundingData {
    #[serde(rename = "id")]
    pub id: serde_json::Number,
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "funding_rate")]
    pub funding_rate: String,
    #[serde(rename = "funding_asset")]
    pub funding_asset: String,
    #[serde(rename = "funding")]
    pub funding: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsAccountFundingData {
    fn default() -> Self {
        Self {
            id: 0.into(),
            instrument_id: 0.into(),
            size: Default::default(),
            funding_rate: Default::default(),
            funding_asset: Default::default(),
            funding: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsAccountFundingHistory {
    #[serde(rename = "data")]
    pub data: Vec<PerpsAccountFundingData>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsAccountLimits {
    #[serde(rename = "actions_per_minute")]
    pub actions_per_minute: serde_json::Number,
    #[serde(rename = "actions_per_minute_limit")]
    pub actions_per_minute_limit: serde_json::Number,
    #[serde(rename = "actions_burst")]
    pub actions_burst: serde_json::Number,
    #[serde(rename = "actions_burst_limit")]
    pub actions_burst_limit: serde_json::Number,
    #[serde(rename = "open_orders")]
    pub open_orders: serde_json::Number,
    #[serde(rename = "open_orders_limit")]
    pub open_orders_limit: serde_json::Number,
    #[serde(rename = "reset")]
    pub reset: serde_json::Number,
    #[serde(rename = "messages_per_minute")]
    pub messages_per_minute: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsAccountLimits {
    fn default() -> Self {
        Self {
            actions_per_minute: 0.into(),
            actions_per_minute_limit: 0.into(),
            actions_burst: 0.into(),
            actions_burst_limit: 0.into(),
            open_orders: 0.into(),
            open_orders_limit: 0.into(),
            reset: 0.into(),
            messages_per_minute: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsAccountReferral {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "parent", default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "children_count")]
    pub children_count: serde_json::Number,
    #[serde(rename = "active_children_count")]
    pub active_children_count: serde_json::Number,
    #[serde(rename = "children_volume")]
    pub children_volume: String,
    #[serde(rename = "children_fees")]
    pub children_fees: String,
    #[serde(rename = "referrals_left")]
    pub referrals_left: serde_json::Number,
    #[serde(rename = "fee_share_rate")]
    pub fee_share_rate: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsAccountReferral {
    fn default() -> Self {
        Self {
            code: Default::default(),
            parent: Default::default(),
            children_count: 0.into(),
            active_children_count: 0.into(),
            children_volume: Default::default(),
            children_fees: Default::default(),
            referrals_left: 0.into(),
            fee_share_rate: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsAccountReferralRequest {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsAccountReward {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "maker_share_7d")]
    pub maker_share_7d: String,
    #[serde(rename = "reward_distributed")]
    pub reward_distributed: bool,
    #[serde(rename = "breakdown")]
    pub breakdown: Vec<PerpsAccountRewardInstrument>,
    #[serde(rename = "oi_rewards", default, skip_serializing_if = "Option::is_none")]
    pub oi_rewards: Option<Box<PerpsAccountRewardOi>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsAccountRewardInstrument {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "reward_pool")]
    pub reward_pool: String,
    #[serde(rename = "reward_amount")]
    pub reward_amount: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsAccountRewardInstrument {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            reward_pool: Default::default(),
            reward_amount: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsAccountRewardOi {
    #[serde(rename = "account_oi")]
    pub account_oi: String,
    #[serde(rename = "reward_amount")]
    pub reward_amount: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsAccountRewards {
    #[serde(rename = "data")]
    pub data: Vec<PerpsAccountReward>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsAccountStats {
    #[serde(rename = "volume_7d")]
    pub volume_7d: String,
    #[serde(rename = "taker_volume_7d")]
    pub taker_volume_7d: String,
    #[serde(rename = "maker_volume_7d")]
    pub maker_volume_7d: String,
    #[serde(rename = "account_maker_share_7d")]
    pub account_maker_share_7d: String,
    #[serde(rename = "entity_maker_share_7d", default, skip_serializing_if = "Option::is_none")]
    pub entity_maker_share_7d: Option<String>,
    #[serde(rename = "entity_id", default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<serde_json::Number>,
    #[serde(rename = "entity_name", default, skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsAccountTradeData {
    #[serde(rename = "trade_id")]
    pub trade_id: serde_json::Number,
    #[serde(rename = "order_id")]
    pub order_id: serde_json::Number,
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "taker")]
    pub taker: bool,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "fee_asset")]
    pub fee_asset: String,
    #[serde(rename = "previous_size")]
    pub previous_size: String,
    #[serde(rename = "previous_entry_price")]
    pub previous_entry_price: String,
    #[serde(rename = "pnl")]
    pub pnl: String,
    #[serde(rename = "liquidation")]
    pub liquidation: bool,
    #[serde(rename = "adl")]
    pub adl: bool,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "builder_fee")]
    pub builder_fee: String,
    #[serde(rename = "total_fee")]
    pub total_fee: String,
    #[serde(rename = "builder", default, skip_serializing_if = "Option::is_none")]
    pub builder: Option<Box<PerpsBuilder>>,
    #[serde(rename = "settlement")]
    pub settlement: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsAccountTradeData {
    fn default() -> Self {
        Self {
            trade_id: 0.into(),
            order_id: 0.into(),
            instrument_id: 0.into(),
            side: Default::default(),
            price: Default::default(),
            quantity: Default::default(),
            taker: Default::default(),
            fee: Default::default(),
            fee_asset: Default::default(),
            previous_size: Default::default(),
            previous_entry_price: Default::default(),
            pnl: Default::default(),
            liquidation: Default::default(),
            adl: Default::default(),
            timestamp: 0.into(),
            hash: Default::default(),
            builder_fee: Default::default(),
            total_fee: Default::default(),
            builder: Default::default(),
            settlement: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsAccountTrades {
    #[serde(rename = "data")]
    pub data: Vec<PerpsAccountTradeData>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsAsset {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "decimals")]
    pub decimals: serde_json::Number,
    #[serde(rename = "collateral_ratio")]
    pub collateral_ratio: String,
    #[serde(rename = "withdrawal_fee")]
    pub withdrawal_fee: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsAsset {
    fn default() -> Self {
        Self {
            asset: Default::default(),
            address: Default::default(),
            decimals: 0.into(),
            collateral_ratio: Default::default(),
            withdrawal_fee: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsAutoCancelRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpAutoCancel>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsAutoCancelResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "deadline")]
    pub deadline: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsAutoCancelResponse {
    fn default() -> Self {
        Self {
            status: Default::default(),
            deadline: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsAutoCancelStatus {
    #[serde(rename = "deadline")]
    pub deadline: serde_json::Number,
    #[serde(rename = "triggered")]
    pub triggered: serde_json::Number,
    #[serde(rename = "daily_limit")]
    pub daily_limit: serde_json::Number,
    #[serde(rename = "next_reset")]
    pub next_reset: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsAutoCancelStatus {
    fn default() -> Self {
        Self {
            deadline: 0.into(),
            triggered: 0.into(),
            daily_limit: 0.into(),
            next_reset: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsBBO {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "bid_price")]
    pub bid_price: String,
    #[serde(rename = "bid_quantity")]
    pub bid_quantity: String,
    #[serde(rename = "ask_price")]
    pub ask_price: String,
    #[serde(rename = "ask_quantity")]
    pub ask_quantity: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsBBO {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            bid_price: Default::default(),
            bid_quantity: Default::default(),
            ask_price: Default::default(),
            ask_quantity: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsBalance {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsBaseOp {
    #[serde(rename = "sig")]
    pub sig: String,
    #[serde(rename = "salt")]
    pub salt: serde_json::Number,
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsBaseOp {
    fn default() -> Self {
        Self {
            sig: Default::default(),
            salt: 0.into(),
            ts: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsBlpEnrollRequest {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsBlpEnrollRequest {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            active: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsBlpEnrollment {
    #[serde(rename = "blp")]
    pub blp: bool,
    #[serde(rename = "instruments")]
    pub instruments: Vec<PerpsBlpInstrument>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsBlpInstrument {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsBlpInstrument {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            symbol: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsBlpLiquidation {
    #[serde(rename = "liquidation_id")]
    pub liquidation_id: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "cross")]
    pub cross: bool,
    #[serde(rename = "margin")]
    pub margin: String,
    #[serde(rename = "trades")]
    pub trades: Vec<PerpsBlpLiquidationTrade>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsBlpLiquidation {
    fn default() -> Self {
        Self {
            liquidation_id: 0.into(),
            timestamp: 0.into(),
            cross: Default::default(),
            margin: Default::default(),
            trades: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsBlpLiquidationTrade {
    #[serde(rename = "trade_id")]
    pub trade_id: serde_json::Number,
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsBlpLiquidationTrade {
    fn default() -> Self {
        Self {
            trade_id: 0.into(),
            instrument_id: 0.into(),
            side: Default::default(),
            quantity: Default::default(),
            price: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsBlpLiquidations {
    #[serde(rename = "data")]
    pub data: Vec<PerpsBlpLiquidation>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsBook {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "bids")]
    pub bids: Vec<PerpsLevel>,
    #[serde(rename = "asks")]
    pub asks: Vec<PerpsLevel>,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "sequence")]
    pub sequence: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsBook {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            bids: Default::default(),
            asks: Default::default(),
            timestamp: 0.into(),
            sequence: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsBuilder {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "fee_rate")]
    pub fee_rate: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsBuilderApproval {
    #[serde(rename = "trader")]
    pub trader: String,
    #[serde(rename = "builder")]
    pub builder: String,
    #[serde(rename = "max_fee_rate")]
    pub max_fee_rate: String,
    #[serde(rename = "approval_version")]
    pub approval_version: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "sequence")]
    pub sequence: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsBuilderApproval {
    fn default() -> Self {
        Self {
            trader: Default::default(),
            builder: Default::default(),
            max_fee_rate: Default::default(),
            approval_version: 0.into(),
            timestamp: 0.into(),
            sequence: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsBuilderApprovalRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpApproveBuilder>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsBuilderApprovals {
    #[serde(rename = "data")]
    pub data: Vec<PerpsBuilderApproval>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsBuilderEarning {
    #[serde(rename = "earning_id")]
    pub earning_id: String,
    #[serde(rename = "trade_id")]
    pub trade_id: serde_json::Number,
    #[serde(rename = "order_id")]
    pub order_id: serde_json::Number,
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "sequence")]
    pub sequence: serde_json::Number,
    #[serde(rename = "notional")]
    pub notional: String,
    #[serde(rename = "fee_asset")]
    pub fee_asset: String,
    #[serde(rename = "builder_fee")]
    pub builder_fee: String,
    #[serde(rename = "fee_rate")]
    pub fee_rate: String,
    #[serde(rename = "trader")]
    pub trader: String,
    #[serde(rename = "buy")]
    pub buy: bool,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "total_fee")]
    pub total_fee: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsBuilderEarning {
    fn default() -> Self {
        Self {
            earning_id: Default::default(),
            trade_id: 0.into(),
            order_id: 0.into(),
            instrument_id: 0.into(),
            side: Default::default(),
            timestamp: 0.into(),
            sequence: 0.into(),
            notional: Default::default(),
            fee_asset: Default::default(),
            builder_fee: Default::default(),
            fee_rate: Default::default(),
            trader: Default::default(),
            buy: Default::default(),
            price: Default::default(),
            quantity: Default::default(),
            client_order_id: Default::default(),
            fee: Default::default(),
            total_fee: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsBuilderEarnings {
    #[serde(rename = "data")]
    pub data: Vec<PerpsBuilderEarning>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: serde_json::Number,
    #[serde(rename = "as_of_sequence")]
    pub as_of_sequence: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsBuilderEarnings {
    fn default() -> Self {
        Self {
            data: Default::default(),
            more: Default::default(),
            cursor: Default::default(),
            start_timestamp: 0.into(),
            end_timestamp: 0.into(),
            as_of_sequence: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsBuilderEarningsAsset {
    #[serde(rename = "fee_asset")]
    pub fee_asset: String,
    #[serde(rename = "fill_count")]
    pub fill_count: serde_json::Number,
    #[serde(rename = "notional")]
    pub notional: String,
    #[serde(rename = "builder_fee")]
    pub builder_fee: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsBuilderEarningsAsset {
    fn default() -> Self {
        Self {
            fee_asset: Default::default(),
            fill_count: 0.into(),
            notional: Default::default(),
            builder_fee: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsBuilderEarningsSummary {
    #[serde(rename = "data")]
    pub data: Vec<PerpsBuilderEarningsAsset>,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: serde_json::Number,
    #[serde(rename = "as_of_sequence")]
    pub as_of_sequence: serde_json::Number,
    #[serde(rename = "trader_count")]
    pub trader_count: serde_json::Number,
    #[serde(rename = "active_approval_count")]
    pub active_approval_count: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsBuilderEarningsSummary {
    fn default() -> Self {
        Self {
            data: Default::default(),
            start_timestamp: 0.into(),
            end_timestamp: 0.into(),
            as_of_sequence: 0.into(),
            trader_count: 0.into(),
            active_approval_count: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsBuilderStatus {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "registered")]
    pub registered: bool,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "admission_enabled")]
    pub admission_enabled: bool,
    #[serde(rename = "max_fee_rate")]
    pub max_fee_rate: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsCancelAccepted {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "oid")]
    pub oid: serde_json::Number,
    #[serde(rename = "coid", default, skip_serializing_if = "Option::is_none")]
    pub coid: Option<String>,
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCancelAccepted {
    fn default() -> Self {
        Self {
            status: Default::default(),
            oid: 0.into(),
            coid: Default::default(),
            ts: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCancelAllRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpCancelAll>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(rename = "exp", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCancelByCoidRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpCancelOrdersByCoid>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(rename = "exp", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsCancelRejected {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "oid", default, skip_serializing_if = "Option::is_none")]
    pub oid: Option<serde_json::Number>,
    #[serde(rename = "coid", default, skip_serializing_if = "Option::is_none")]
    pub coid: Option<String>,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCancelRejected {
    fn default() -> Self {
        Self {
            status: Default::default(),
            oid: Default::default(),
            coid: Default::default(),
            error: Default::default(),
            ts: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCancelRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpCancelOrders>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(rename = "exp", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsCompetitionAccount {
    #[serde(rename = "competition_id")]
    pub competition_id: serde_json::Number,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "updated_at", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<serde_json::Number>,
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "rank", default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<serde_json::Number>,
    #[serde(rename = "qualified")]
    pub qualified: bool,
    #[serde(rename = "score")]
    pub score: String,
    #[serde(rename = "pnl")]
    pub pnl: String,
    #[serde(rename = "realized_pnl")]
    pub realized_pnl: String,
    #[serde(rename = "unrealized_pnl")]
    pub unrealized_pnl: String,
    #[serde(rename = "fees")]
    pub fees: String,
    #[serde(rename = "funding")]
    pub funding: String,
    #[serde(rename = "volume")]
    pub volume: String,
    #[serde(rename = "boosted_volume")]
    pub boosted_volume: String,
    #[serde(rename = "first_fill_at", default, skip_serializing_if = "Option::is_none")]
    pub first_fill_at: Option<serde_json::Number>,
    #[serde(rename = "min_volume")]
    pub min_volume: String,
    #[serde(rename = "volume_to_qualify")]
    pub volume_to_qualify: String,
    #[serde(rename = "qualification_progress")]
    pub qualification_progress: String,
    #[serde(rename = "prize", default, skip_serializing_if = "Option::is_none")]
    pub prize: Option<String>,
    #[serde(rename = "spots_from_prize_zone", default, skip_serializing_if = "Option::is_none")]
    pub spots_from_prize_zone: Option<serde_json::Number>,
    #[serde(rename = "score_to_prize_zone", default, skip_serializing_if = "Option::is_none")]
    pub score_to_prize_zone: Option<String>,
    #[serde(rename = "row_state")]
    pub row_state: String,
    #[serde(rename = "payout_tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub payout_tx_hash: Option<String>,
    #[serde(rename = "paid_at", default, skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<serde_json::Number>,
    #[serde(rename = "locked_positions")]
    pub locked_positions: Vec<PerpsCompetitionLockedPosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCompetitionAccount {
    fn default() -> Self {
        Self {
            competition_id: 0.into(),
            status: Default::default(),
            updated_at: Default::default(),
            account: Default::default(),
            rank: Default::default(),
            qualified: Default::default(),
            score: Default::default(),
            pnl: Default::default(),
            realized_pnl: Default::default(),
            unrealized_pnl: Default::default(),
            fees: Default::default(),
            funding: Default::default(),
            volume: Default::default(),
            boosted_volume: Default::default(),
            first_fill_at: Default::default(),
            min_volume: Default::default(),
            volume_to_qualify: Default::default(),
            qualification_progress: Default::default(),
            prize: Default::default(),
            spots_from_prize_zone: Default::default(),
            score_to_prize_zone: Default::default(),
            row_state: Default::default(),
            payout_tx_hash: Default::default(),
            paid_at: Default::default(),
            locked_positions: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsCompetitionDetail {
    #[serde(rename = "competition_id")]
    pub competition_id: serde_json::Number,
    #[serde(rename = "number")]
    pub number: serde_json::Number,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "sector_label")]
    pub sector_label: String,
    #[serde(rename = "boost_description")]
    pub boost_description: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "start_time")]
    pub start_time: serde_json::Number,
    #[serde(rename = "end_time")]
    pub end_time: serde_json::Number,
    #[serde(rename = "all_markets")]
    pub all_markets: bool,
    #[serde(rename = "markets")]
    pub markets: Vec<PerpsCompetitionMarket>,
    #[serde(rename = "min_volume")]
    pub min_volume: String,
    #[serde(rename = "count_unrealized")]
    pub count_unrealized: bool,
    #[serde(rename = "prize_pool")]
    pub prize_pool: String,
    #[serde(rename = "prize_asset_id")]
    pub prize_asset_id: serde_json::Number,
    #[serde(rename = "prize_tiers")]
    pub prize_tiers: Vec<PerpsCompetitionPrizeTier>,
    #[serde(rename = "paid_places")]
    pub paid_places: serde_json::Number,
    #[serde(rename = "internal")]
    pub internal: bool,
    #[serde(rename = "start_trading_instrument_id", default, skip_serializing_if = "Option::is_none")]
    pub start_trading_instrument_id: Option<serde_json::Number>,
    #[serde(rename = "updated_at", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<serde_json::Number>,
    #[serde(rename = "ranked")]
    pub ranked: serde_json::Number,
    #[serde(rename = "participants")]
    pub participants: serde_json::Number,
    #[serde(rename = "prize_cutoff_score", default, skip_serializing_if = "Option::is_none")]
    pub prize_cutoff_score: Option<String>,
    #[serde(rename = "final_results")]
    pub final_results: bool,
    #[serde(rename = "scoring_version")]
    pub scoring_version: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCompetitionDetail {
    fn default() -> Self {
        Self {
            competition_id: 0.into(),
            number: 0.into(),
            name: Default::default(),
            sector_label: Default::default(),
            boost_description: Default::default(),
            status: Default::default(),
            start_time: 0.into(),
            end_time: 0.into(),
            all_markets: Default::default(),
            markets: Default::default(),
            min_volume: Default::default(),
            count_unrealized: Default::default(),
            prize_pool: Default::default(),
            prize_asset_id: 0.into(),
            prize_tiers: Default::default(),
            paid_places: 0.into(),
            internal: Default::default(),
            start_trading_instrument_id: Default::default(),
            updated_at: Default::default(),
            ranked: 0.into(),
            participants: 0.into(),
            prize_cutoff_score: Default::default(),
            final_results: Default::default(),
            scoring_version: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCompetitionEntry {
    #[serde(rename = "rank", default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<serde_json::Number>,
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "score")]
    pub score: String,
    #[serde(rename = "pnl")]
    pub pnl: String,
    #[serde(rename = "realized_pnl")]
    pub realized_pnl: String,
    #[serde(rename = "unrealized_pnl")]
    pub unrealized_pnl: String,
    #[serde(rename = "volume")]
    pub volume: String,
    #[serde(rename = "boosted_volume")]
    pub boosted_volume: String,
    #[serde(rename = "first_fill_at", default, skip_serializing_if = "Option::is_none")]
    pub first_fill_at: Option<serde_json::Number>,
    #[serde(rename = "prize", default, skip_serializing_if = "Option::is_none")]
    pub prize: Option<String>,
    #[serde(rename = "row_state")]
    pub row_state: String,
    #[serde(rename = "payout_tx_hash", default, skip_serializing_if = "Option::is_none")]
    pub payout_tx_hash: Option<String>,
    #[serde(rename = "paid_at", default, skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsCompetitionLeaderboard {
    #[serde(rename = "competition_id")]
    pub competition_id: serde_json::Number,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "updated_at", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<serde_json::Number>,
    #[serde(rename = "total")]
    pub total: serde_json::Number,
    #[serde(rename = "offset")]
    pub offset: serde_json::Number,
    #[serde(rename = "final_results")]
    pub final_results: bool,
    #[serde(rename = "entries")]
    pub entries: Vec<PerpsCompetitionEntry>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCompetitionLeaderboard {
    fn default() -> Self {
        Self {
            competition_id: 0.into(),
            status: Default::default(),
            updated_at: Default::default(),
            total: 0.into(),
            offset: 0.into(),
            final_results: Default::default(),
            entries: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsCompetitionList {
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "live_competition_id", default, skip_serializing_if = "Option::is_none")]
    pub live_competition_id: Option<serde_json::Number>,
    #[serde(rename = "landing_competition_id", default, skip_serializing_if = "Option::is_none")]
    pub landing_competition_id: Option<serde_json::Number>,
    #[serde(rename = "competitions")]
    pub competitions: Vec<PerpsCompetitionSummary>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCompetitionList {
    fn default() -> Self {
        Self {
            timestamp: 0.into(),
            live_competition_id: Default::default(),
            landing_competition_id: Default::default(),
            competitions: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsCompetitionLockedPosition {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "size_at_start")]
    pub size_at_start: String,
    #[serde(rename = "released_at", default, skip_serializing_if = "Option::is_none")]
    pub released_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCompetitionLockedPosition {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            size_at_start: Default::default(),
            released_at: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsCompetitionMarket {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "boost")]
    pub boost: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCompetitionMarket {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            boost: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsCompetitionPrizeTier {
    #[serde(rename = "from_rank")]
    pub from_rank: serde_json::Number,
    #[serde(rename = "to_rank")]
    pub to_rank: serde_json::Number,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCompetitionPrizeTier {
    fn default() -> Self {
        Self {
            from_rank: 0.into(),
            to_rank: 0.into(),
            amount: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCompetitionReadError400 {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "arts", default, skip_serializing_if = "Option::is_none")]
    pub arts: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCompetitionReadError404 {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCompetitionReadError429 {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCompetitionReadError500 {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCompetitionReadError503 {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsCompetitionSummary {
    #[serde(rename = "competition_id")]
    pub competition_id: serde_json::Number,
    #[serde(rename = "number")]
    pub number: serde_json::Number,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "sector_label")]
    pub sector_label: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "start_time")]
    pub start_time: serde_json::Number,
    #[serde(rename = "end_time")]
    pub end_time: serde_json::Number,
    #[serde(rename = "prize_pool")]
    pub prize_pool: String,
    #[serde(rename = "prize_asset_id")]
    pub prize_asset_id: serde_json::Number,
    #[serde(rename = "internal")]
    pub internal: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCompetitionSummary {
    fn default() -> Self {
        Self {
            competition_id: 0.into(),
            number: 0.into(),
            name: Default::default(),
            sector_label: Default::default(),
            status: Default::default(),
            start_time: 0.into(),
            end_time: 0.into(),
            prize_pool: Default::default(),
            prize_asset_id: 0.into(),
            internal: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsCompetitionWidget {
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "competition_id", default, skip_serializing_if = "Option::is_none")]
    pub competition_id: Option<serde_json::Number>,
    #[serde(rename = "number", default, skip_serializing_if = "Option::is_none")]
    pub number: Option<serde_json::Number>,
    #[serde(rename = "end_time", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<serde_json::Number>,
    #[serde(rename = "rank", default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<serde_json::Number>,
    #[serde(rename = "qualified", default, skip_serializing_if = "Option::is_none")]
    pub qualified: Option<bool>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(rename = "min_volume", default, skip_serializing_if = "Option::is_none")]
    pub min_volume: Option<String>,
    #[serde(rename = "qualification_progress", default, skip_serializing_if = "Option::is_none")]
    pub qualification_progress: Option<String>,
    #[serde(rename = "spots_from_prize_zone", default, skip_serializing_if = "Option::is_none")]
    pub spots_from_prize_zone: Option<serde_json::Number>,
    #[serde(rename = "updated_at", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCompetitionWidget {
    fn default() -> Self {
        Self {
            timestamp: 0.into(),
            competition_id: Default::default(),
            number: Default::default(),
            end_time: Default::default(),
            rank: Default::default(),
            qualified: Default::default(),
            volume: Default::default(),
            min_volume: Default::default(),
            qualification_progress: Default::default(),
            spots_from_prize_zone: Default::default(),
            updated_at: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCreateInviteRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsCreateInviteResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "referrals_available")]
    pub referrals_available: serde_json::Number,
    #[serde(rename = "cooldown_ms", skip_serializing_if = "Option::is_none")]
    pub cooldown_ms: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCreateInviteResponse {
    fn default() -> Self {
        Self {
            status: Default::default(),
            code: Default::default(),
            referrals_available: 0.into(),
            cooldown_ms: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsCreateOrder {
    #[serde(rename = "iid")]
    pub iid: serde_json::Number,
    #[serde(rename = "buy")]
    pub buy: bool,
    #[serde(rename = "p", default, skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[serde(rename = "qty")]
    pub qty: String,
    #[serde(rename = "tif", default, skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,
    #[serde(rename = "po", default, skip_serializing_if = "Option::is_none")]
    pub po: Option<bool>,
    #[serde(rename = "ro", default, skip_serializing_if = "Option::is_none")]
    pub ro: Option<bool>,
    #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(rename = "tr", default, skip_serializing_if = "Option::is_none")]
    pub tr: Option<PerpsCreateOrderTr>,
    #[serde(rename = "builder", default, skip_serializing_if = "Option::is_none")]
    pub builder: Option<Box<PerpsBuilder>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCreateOrder {
    fn default() -> Self {
        Self {
            iid: 0.into(),
            buy: Default::default(),
            p: Default::default(),
            qty: Default::default(),
            tif: Default::default(),
            po: Default::default(),
            ro: Default::default(),
            c: Default::default(),
            tr: Default::default(),
            builder: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCreateOrderTr {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<bool>,
    #[serde(rename = "trp", default, skip_serializing_if = "Option::is_none")]
    pub trp: Option<String>,
    #[serde(rename = "tpsl", default, skip_serializing_if = "Option::is_none")]
    pub tpsl: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCredentials {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "keys")]
    pub keys: Vec<PerpsKey>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsDeleteProxyRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpDeleteProxy>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsDeposit {
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "confirmations")]
    pub confirmations: serde_json::Number,
    #[serde(rename = "required_confirmations")]
    pub required_confirmations: serde_json::Number,
    #[serde(rename = "created_timestamp")]
    pub created_timestamp: serde_json::Number,
    #[serde(rename = "confirmed_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub confirmed_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsDeposit {
    fn default() -> Self {
        Self {
            hash: Default::default(),
            asset: Default::default(),
            amount: Default::default(),
            from: Default::default(),
            to: Default::default(),
            status: Default::default(),
            confirmations: 0.into(),
            required_confirmations: 0.into(),
            created_timestamp: 0.into(),
            confirmed_timestamp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsDeposits {
    #[serde(rename = "data")]
    pub data: Vec<PerpsDeposit>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsEquityHistory {
    #[serde(rename = "data")]
    pub data: Vec<Vec<serde_json::Value>>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsError400 {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsError401 {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsError404 {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsError408 {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsError413 {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsError429 {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsError500 {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsError503 {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsExchange {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "chain_id")]
    pub chain_id: serde_json::Number,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "cancel_only")]
    pub cancel_only: bool,
    #[serde(rename = "engine_version")]
    pub engine_version: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsExchange {
    fn default() -> Self {
        Self {
            name: Default::default(),
            version: Default::default(),
            chain_id: 0.into(),
            contract: Default::default(),
            cancel_only: Default::default(),
            engine_version: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsExchangeStatistics {
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: serde_json::Number,
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: serde_json::Number,
    #[serde(rename = "volume")]
    pub volume: String,
    #[serde(rename = "open_interest", skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<String>,
    #[serde(rename = "open_interest_timestamp", skip_serializing_if = "Option::is_none")]
    pub open_interest_timestamp: Option<serde_json::Number>,
    #[serde(rename = "fees")]
    pub fees: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsExchangeStatistics {
    fn default() -> Self {
        Self {
            start_timestamp: 0.into(),
            end_timestamp: 0.into(),
            volume: Default::default(),
            open_interest: Default::default(),
            open_interest_timestamp: Default::default(),
            fees: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsFeeScheduleEntry {
    #[serde(rename = "instrument_type")]
    pub instrument_type: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "taker_fee_rate")]
    pub taker_fee_rate: String,
    #[serde(rename = "maker_fee_rate")]
    pub maker_fee_rate: String,
    #[serde(rename = "tiers")]
    pub tiers: Vec<PerpsFeeTier>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsFeeTier {
    #[serde(rename = "min_volume_30d")]
    pub min_volume_30d: String,
    #[serde(rename = "taker_fee_rate")]
    pub taker_fee_rate: String,
    #[serde(rename = "maker_fee_rate")]
    pub maker_fee_rate: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsFeesInfo {
    #[serde(rename = "fee_schedule")]
    pub fee_schedule: Vec<PerpsFeeScheduleEntry>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsFundingHistory {
    #[serde(rename = "data")]
    pub data: Vec<PerpsFundingRate>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsFundingRate {
    #[serde(rename = "funding_rate")]
    pub funding_rate: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsFundingRate {
    fn default() -> Self {
        Self {
            funding_rate: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGenericAccepted {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGenericRejected {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsIndex {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "index_price")]
    pub index_price: String,
    #[serde(rename = "constituents")]
    pub constituents: Vec<PerpsIndexConstituent>,
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsIndex {
    fn default() -> Self {
        Self {
            asset: Default::default(),
            index_price: Default::default(),
            constituents: Default::default(),
            ts: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsIndexConstituent {
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "weight")]
    pub weight: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsInstrument {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "instrument_type")]
    pub instrument_type: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "isolated_only")]
    pub isolated_only: bool,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "display_symbol", default, skip_serializing_if = "Option::is_none")]
    pub display_symbol: Option<String>,
    #[serde(rename = "base_asset")]
    pub base_asset: String,
    #[serde(rename = "quote_asset")]
    pub quote_asset: String,
    #[serde(rename = "funding_interval")]
    pub funding_interval: String,
    #[serde(rename = "quantity_decimals")]
    pub quantity_decimals: serde_json::Number,
    #[serde(rename = "price_decimals")]
    pub price_decimals: serde_json::Number,
    #[serde(rename = "price_bounds")]
    pub price_bounds: String,
    #[serde(rename = "liquidation_fee")]
    pub liquidation_fee: String,
    #[serde(rename = "max_order_count")]
    pub max_order_count: serde_json::Number,
    #[serde(rename = "min_notional")]
    pub min_notional: String,
    #[serde(rename = "max_market_notional")]
    pub max_market_notional: String,
    #[serde(rename = "max_limit_notional")]
    pub max_limit_notional: String,
    #[serde(rename = "max_leverage")]
    pub max_leverage: serde_json::Number,
    #[serde(rename = "risk_tiers")]
    pub risk_tiers: Vec<PerpsRiskTier>,
    #[serde(rename = "ui_live_time", skip_serializing_if = "Option::is_none")]
    pub ui_live_time: Option<serde_json::Number>,
    #[serde(rename = "close_only")]
    pub close_only: bool,
    #[serde(rename = "settlement", default, skip_serializing_if = "Option::is_none")]
    pub settlement: Option<Box<PerpsInstrumentSettlement>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsInstrument {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            instrument_type: Default::default(),
            category: Default::default(),
            isolated_only: Default::default(),
            symbol: Default::default(),
            display_symbol: Default::default(),
            base_asset: Default::default(),
            quote_asset: Default::default(),
            funding_interval: Default::default(),
            quantity_decimals: 0.into(),
            price_decimals: 0.into(),
            price_bounds: Default::default(),
            liquidation_fee: Default::default(),
            max_order_count: 0.into(),
            min_notional: Default::default(),
            max_market_notional: Default::default(),
            max_limit_notional: Default::default(),
            max_leverage: 0.into(),
            risk_tiers: Default::default(),
            ui_live_time: Default::default(),
            close_only: Default::default(),
            settlement: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsInstrumentSettlement {
    #[serde(rename = "sequence")]
    pub sequence: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "insurance_debit")]
    pub insurance_debit: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsInstrumentSettlement {
    fn default() -> Self {
        Self {
            sequence: 0.into(),
            timestamp: 0.into(),
            price: Default::default(),
            insurance_debit: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsInternalTransfer {
    #[serde(rename = "transfer_id")]
    pub transfer_id: serde_json::Number,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "direction")]
    pub direction: String,
    #[serde(rename = "counterparty")]
    pub counterparty: String,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "created_timestamp")]
    pub created_timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsInternalTransfer {
    fn default() -> Self {
        Self {
            transfer_id: 0.into(),
            asset: Default::default(),
            amount: Default::default(),
            direction: Default::default(),
            counterparty: Default::default(),
            label: Default::default(),
            created_timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsInternalTransferAccepted {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "transfer_id")]
    pub transfer_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsInternalTransferAccepted {
    fn default() -> Self {
        Self {
            status: Default::default(),
            transfer_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsInternalTransferRejected {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "transfer_id")]
    pub transfer_id: serde_json::Number,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsInternalTransferRejected {
    fn default() -> Self {
        Self {
            status: Default::default(),
            transfer_id: 0.into(),
            error: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsInternalTransferRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpInternalTransfer>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsInternalTransfers {
    #[serde(rename = "data")]
    pub data: Vec<PerpsInternalTransfer>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsInviteCheckResponse {
    #[serde(rename = "valid")]
    pub valid: bool,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsKey {
    #[serde(rename = "proxy")]
    pub proxy: String,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsKey {
    fn default() -> Self {
        Self {
            proxy: Default::default(),
            label: Default::default(),
            expiry: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsKlinesResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsKline>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsLeaderboard {
    #[serde(rename = "window")]
    pub window: String,
    #[serde(rename = "sort_by")]
    pub sort_by: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "total")]
    pub total: serde_json::Number,
    #[serde(rename = "entries")]
    pub entries: Vec<PerpsLeaderboardEntry>,
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<Box<PerpsLeaderboardAccount>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsLeaderboard {
    fn default() -> Self {
        Self {
            window: Default::default(),
            sort_by: Default::default(),
            timestamp: 0.into(),
            total: 0.into(),
            entries: Default::default(),
            account: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsLeaderboardAccount {
    #[serde(rename = "rank", default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<serde_json::Number>,
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "pnl")]
    pub pnl: String,
    #[serde(rename = "notional")]
    pub notional: String,
    #[serde(rename = "account_value")]
    pub account_value: String,
    #[serde(rename = "roi")]
    pub roi: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsLeaderboardEntry {
    #[serde(rename = "rank")]
    pub rank: serde_json::Number,
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "pnl")]
    pub pnl: String,
    #[serde(rename = "notional")]
    pub notional: String,
    #[serde(rename = "account_value")]
    pub account_value: String,
    #[serde(rename = "roi")]
    pub roi: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsLeaderboardEntry {
    fn default() -> Self {
        Self {
            rank: 0.into(),
            account: Default::default(),
            pnl: Default::default(),
            notional: Default::default(),
            account_value: Default::default(),
            roi: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsLeverageBatchRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpUpdateLeverages>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsLeverageRejected {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsLeverageRejected {
    fn default() -> Self {
        Self {
            status: Default::default(),
            instrument_id: 0.into(),
            error: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsLeverageRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpUpdateLeverage>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsLeverageResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "leverage")]
    pub leverage: serde_json::Number,
    #[serde(rename = "cross")]
    pub cross: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsLeverageResponse {
    fn default() -> Self {
        Self {
            status: Default::default(),
            instrument_id: 0.into(),
            leverage: 0.into(),
            cross: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsLimitTier {
    #[serde(rename = "min_volume_14d")]
    pub min_volume_14d: String,
    #[serde(rename = "rate_per_minute_limit")]
    pub rate_per_minute_limit: serde_json::Number,
    #[serde(rename = "rate_burst_limit")]
    pub rate_burst_limit: serde_json::Number,
    #[serde(rename = "actions_per_minute_limit")]
    pub actions_per_minute_limit: serde_json::Number,
    #[serde(rename = "actions_burst_limit")]
    pub actions_burst_limit: serde_json::Number,
    #[serde(rename = "open_orders_limit")]
    pub open_orders_limit: serde_json::Number,
    #[serde(rename = "messages_per_minute")]
    pub messages_per_minute: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsLimitTier {
    fn default() -> Self {
        Self {
            min_volume_14d: Default::default(),
            rate_per_minute_limit: 0.into(),
            rate_burst_limit: 0.into(),
            actions_per_minute_limit: 0.into(),
            actions_burst_limit: 0.into(),
            open_orders_limit: 0.into(),
            messages_per_minute: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpUpdateMargin>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginSummary {
    #[serde(rename = "total_account_value")]
    pub total_account_value: String,
    #[serde(rename = "available_order_margin")]
    pub available_order_margin: String,
    #[serde(rename = "total_initial_margin")]
    pub total_initial_margin: String,
    #[serde(rename = "total_maintenance_margin")]
    pub total_maintenance_margin: String,
    #[serde(rename = "total_position_value")]
    pub total_position_value: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarkHistoryResponse {
    #[serde(rename = "data")]
    pub data: Vec<PerpsMarkPoint>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsModifyAccepted {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "order")]
    pub order: Box<PerpsOrder>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsModifyByCoidRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpModifyOrdersByCoid>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(rename = "exp", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsModifyOrder {
    #[serde(rename = "oid")]
    pub oid: serde_json::Number,
    #[serde(rename = "p")]
    pub p: String,
    #[serde(rename = "qty")]
    pub qty: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsModifyOrder {
    fn default() -> Self {
        Self {
            oid: 0.into(),
            p: Default::default(),
            qty: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsModifyOrderByCoid {
    #[serde(rename = "coid")]
    pub coid: String,
    #[serde(rename = "p")]
    pub p: String,
    #[serde(rename = "qty")]
    pub qty: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsModifyRejected {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "oid", default, skip_serializing_if = "Option::is_none")]
    pub oid: Option<serde_json::Number>,
    #[serde(rename = "coid", default, skip_serializing_if = "Option::is_none")]
    pub coid: Option<String>,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsModifyRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpModifyOrders>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(rename = "exp", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsNotificationItem {
    #[serde(rename = "notification")]
    pub notification: serde_json::Map<String,serde_json::Value>,
    #[serde(rename = "read_at", skip_serializing_if = "Option::is_none")]
    pub read_at: Option<serde_json::Number>,
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsNotificationItem {
    fn default() -> Self {
        Self {
            notification: Default::default(),
            read_at: Default::default(),
            ts: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsNotificationsResponse {
    #[serde(rename = "items")]
    pub items: Vec<PerpsNotificationItem>,
    #[serde(rename = "unread")]
    pub unread: serde_json::Number,
    #[serde(rename = "durable_source_seq")]
    pub durable_source_seq: serde_json::Number,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "next_cursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsNotificationsResponse {
    fn default() -> Self {
        Self {
            items: Default::default(),
            unread: 0.into(),
            durable_source_seq: 0.into(),
            has_more: Default::default(),
            next_cursor: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpApproveBuilder {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: PerpsOpApproveBuilderArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsOpApproveBuilderArgs {
    #[serde(rename = "builder")]
    pub builder: String,
    #[serde(rename = "max_fee_rate")]
    pub max_fee_rate: String,
    #[serde(rename = "approval_version")]
    pub approval_version: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsOpApproveBuilderArgs {
    fn default() -> Self {
        Self {
            builder: Default::default(),
            max_fee_rate: Default::default(),
            approval_version: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpAutoCancel {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: PerpsOpAutoCancelArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsOpAutoCancelArgs {
    #[serde(rename = "time")]
    pub time: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsOpAutoCancelArgs {
    fn default() -> Self {
        Self {
            time: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpCancelAll {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: PerpsOpCancelAllArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpCancelAllArgs {
    #[serde(rename = "iid", default, skip_serializing_if = "Option::is_none")]
    pub iid: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpCancelOrders {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: Vec<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpCancelOrdersByCoid {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpCreateOrders {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: Vec<PerpsCreateOrder>,
    #[serde(rename = "grp", default, skip_serializing_if = "Option::is_none")]
    pub grp: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpCreateProxy {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: PerpsOpCreateProxyArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsOpCreateProxyArgs {
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "proxy")]
    pub proxy: String,
    #[serde(rename = "expiry")]
    pub expiry: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsOpCreateProxyArgs {
    fn default() -> Self {
        Self {
            owner: Default::default(),
            proxy: Default::default(),
            expiry: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpDeleteProxy {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: PerpsOpDeleteProxyArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpDeleteProxyArgs {
    #[serde(rename = "proxy")]
    pub proxy: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpInternalTransfer {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: PerpsOpInternalTransferArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpInternalTransferArgs {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpModifyOrders {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: Vec<PerpsModifyOrder>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpModifyOrdersByCoid {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: Vec<PerpsModifyOrderByCoid>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpUpdateLeverage {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: PerpsOpUpdateLeverageArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsOpUpdateLeverageArgs {
    #[serde(rename = "iid")]
    pub iid: serde_json::Number,
    #[serde(rename = "lev")]
    pub lev: serde_json::Number,
    #[serde(rename = "cross")]
    pub cross: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsOpUpdateLeverageArgs {
    fn default() -> Self {
        Self {
            iid: 0.into(),
            lev: 0.into(),
            cross: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpUpdateLeverages {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: Vec<PerpsOpUpdateLeveragesArgsItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsOpUpdateLeveragesArgsItem {
    #[serde(rename = "iid")]
    pub iid: serde_json::Number,
    #[serde(rename = "lev")]
    pub lev: serde_json::Number,
    #[serde(rename = "cross")]
    pub cross: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsOpUpdateLeveragesArgsItem {
    fn default() -> Self {
        Self {
            iid: 0.into(),
            lev: 0.into(),
            cross: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpUpdateMargin {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: PerpsOpUpdateMarginArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsOpUpdateMarginArgs {
    #[serde(rename = "iid")]
    pub iid: serde_json::Number,
    #[serde(rename = "amt")]
    pub amt: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsOpUpdateMarginArgs {
    fn default() -> Self {
        Self {
            iid: 0.into(),
            amt: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpWithdraw {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: PerpsOpWithdrawArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOpWithdrawArgs {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsOrder {
    #[serde(rename = "oid")]
    pub oid: serde_json::Number,
    #[serde(rename = "iid")]
    pub iid: serde_json::Number,
    #[serde(rename = "buy")]
    pub buy: bool,
    #[serde(rename = "p")]
    pub p: String,
    #[serde(rename = "qty")]
    pub qty: String,
    #[serde(rename = "tif")]
    pub tif: String,
    #[serde(rename = "po")]
    pub po: bool,
    #[serde(rename = "ro")]
    pub ro: bool,
    #[serde(rename = "rest")]
    pub rest: String,
    #[serde(rename = "fill")]
    pub fill: String,
    #[serde(rename = "cts")]
    pub cts: serde_json::Number,
    #[serde(rename = "uts")]
    pub uts: serde_json::Number,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "coid", default, skip_serializing_if = "Option::is_none")]
    pub coid: Option<String>,
    #[serde(rename = "builder", default, skip_serializing_if = "Option::is_none")]
    pub builder: Option<Box<PerpsBuilder>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsOrder {
    fn default() -> Self {
        Self {
            oid: 0.into(),
            iid: 0.into(),
            buy: Default::default(),
            p: Default::default(),
            qty: Default::default(),
            tif: Default::default(),
            po: Default::default(),
            ro: Default::default(),
            rest: Default::default(),
            fill: Default::default(),
            cts: 0.into(),
            uts: 0.into(),
            status: Default::default(),
            coid: Default::default(),
            builder: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsOrderAccepted {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "oid")]
    pub oid: serde_json::Number,
    #[serde(rename = "coid", default, skip_serializing_if = "Option::is_none")]
    pub coid: Option<String>,
    #[serde(rename = "builder", default, skip_serializing_if = "Option::is_none")]
    pub builder: Option<Box<PerpsBuilder>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsOrderAccepted {
    fn default() -> Self {
        Self {
            status: Default::default(),
            oid: 0.into(),
            coid: Default::default(),
            builder: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsOrderData {
    #[serde(rename = "order_id")]
    pub order_id: serde_json::Number,
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "buy")]
    pub buy: bool,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "tif")]
    pub tif: String,
    #[serde(rename = "post_only")]
    pub post_only: bool,
    #[serde(rename = "ro")]
    pub ro: bool,
    #[serde(rename = "resting_quantity")]
    pub resting_quantity: String,
    #[serde(rename = "filled_quantity")]
    pub filled_quantity: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "created_timestamp")]
    pub created_timestamp: serde_json::Number,
    #[serde(rename = "updated_timestamp")]
    pub updated_timestamp: serde_json::Number,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "tpsl", default, skip_serializing_if = "Option::is_none")]
    pub tpsl: Option<Box<PerpsTpSlOrderFields>>,
    #[serde(rename = "builder", default, skip_serializing_if = "Option::is_none")]
    pub builder: Option<Box<PerpsBuilder>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsOrderData {
    fn default() -> Self {
        Self {
            order_id: 0.into(),
            instrument_id: 0.into(),
            buy: Default::default(),
            price: Default::default(),
            quantity: Default::default(),
            tif: Default::default(),
            post_only: Default::default(),
            ro: Default::default(),
            resting_quantity: Default::default(),
            filled_quantity: Default::default(),
            status: Default::default(),
            created_timestamp: 0.into(),
            updated_timestamp: 0.into(),
            client_order_id: Default::default(),
            tpsl: Default::default(),
            builder: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOrderRejected {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "oid", default, skip_serializing_if = "Option::is_none")]
    pub oid: Option<serde_json::Number>,
    #[serde(rename = "coid", default, skip_serializing_if = "Option::is_none")]
    pub coid: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOrderRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpCreateOrders>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(rename = "exp", default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsPnlHistory {
    #[serde(rename = "data")]
    pub data: Vec<Vec<serde_json::Value>>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsPortfolio {
    #[serde(rename = "positions")]
    pub positions: Vec<PerpsPortfolioPosition>,
    #[serde(rename = "margin")]
    pub margin: Box<PerpsMarginSummary>,
    #[serde(rename = "withdrawable")]
    pub withdrawable: String,
    #[serde(rename = "in_liquidation")]
    pub in_liquidation: bool,
    #[serde(rename = "fee_tier")]
    pub fee_tier: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsPortfolio {
    fn default() -> Self {
        Self {
            positions: Default::default(),
            margin: Default::default(),
            withdrawable: Default::default(),
            in_liquidation: Default::default(),
            fee_tier: 0.into(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsPortfolioPosition {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "entry_price")]
    pub entry_price: String,
    #[serde(rename = "leverage")]
    pub leverage: serde_json::Number,
    #[serde(rename = "cross")]
    pub cross: bool,
    #[serde(rename = "initial_margin")]
    pub initial_margin: String,
    #[serde(rename = "maintenance_margin")]
    pub maintenance_margin: String,
    #[serde(rename = "position_value")]
    pub position_value: String,
    #[serde(rename = "liquidation_price")]
    pub liquidation_price: String,
    #[serde(rename = "unrealized_pnl")]
    pub unrealized_pnl: String,
    #[serde(rename = "return_on_equity")]
    pub return_on_equity: String,
    #[serde(rename = "cumulative_funding")]
    pub cumulative_funding: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsPortfolioPosition {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            symbol: Default::default(),
            size: Default::default(),
            entry_price: Default::default(),
            leverage: 0.into(),
            cross: Default::default(),
            initial_margin: Default::default(),
            maintenance_margin: Default::default(),
            position_value: Default::default(),
            liquidation_price: Default::default(),
            unrealized_pnl: Default::default(),
            return_on_equity: Default::default(),
            cumulative_funding: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsPositionCardSnapshot {
    #[serde(rename = "position_cycle_id")]
    pub position_cycle_id: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "started_at")]
    pub started_at: serde_json::Number,
    #[serde(rename = "as_of_at")]
    pub as_of_at: serde_json::Number,
    #[serde(rename = "is_closed")]
    pub is_closed: bool,
    #[serde(rename = "size_after")]
    pub size_after: String,
    #[serde(rename = "entry_price")]
    pub entry_price: String,
    #[serde(rename = "as_of_price")]
    pub as_of_price: String,
    #[serde(rename = "pnl")]
    pub pnl: String,
    #[serde(rename = "pnl_percent", skip_serializing_if = "Option::is_none")]
    pub pnl_percent: Option<String>,
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: Option<serde_json::Number>,
    #[serde(rename = "chart")]
    pub chart: Box<PerpsPositionSnapshotChart>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsPositionCardSnapshot {
    fn default() -> Self {
        Self {
            position_cycle_id: Default::default(),
            side: Default::default(),
            started_at: 0.into(),
            as_of_at: 0.into(),
            is_closed: Default::default(),
            size_after: Default::default(),
            entry_price: Default::default(),
            as_of_price: Default::default(),
            pnl: Default::default(),
            pnl_percent: Default::default(),
            leverage: Default::default(),
            chart: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsPositionSnapshotCandle {
    #[serde(rename = "position")]
    pub position: serde_json::Number,
    #[serde(rename = "open")]
    pub open: serde_json::Number,
    #[serde(rename = "high")]
    pub high: serde_json::Number,
    #[serde(rename = "low")]
    pub low: serde_json::Number,
    #[serde(rename = "close")]
    pub close: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsPositionSnapshotCandle {
    fn default() -> Self {
        Self {
            position: 0.into(),
            open: 0.into(),
            high: 0.into(),
            low: 0.into(),
            close: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsPositionSnapshotChart {
    #[serde(rename = "candles")]
    pub candles: Vec<PerpsPositionSnapshotCandle>,
    #[serde(rename = "markers")]
    pub markers: Vec<PerpsPositionSnapshotMarker>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsPositionSnapshotFill {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "trade_id")]
    pub trade_id: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsPositionSnapshotFill {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            trade_id: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsPositionSnapshotMarker {
    #[serde(rename = "position")]
    pub position: serde_json::Number,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsPositionSnapshotMarker {
    fn default() -> Self {
        Self {
            position: 0.into(),
            kind: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsPositionSnapshotResult {
    #[serde(rename = "instrument_id", default, skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<serde_json::Number>,
    #[serde(rename = "trade_id", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "snapshot", default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Box<PerpsPositionCardSnapshot>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsPositionSnapshots {
    #[serde(rename = "history_as_of_at")]
    pub history_as_of_at: serde_json::Number,
    #[serde(rename = "active")]
    pub active: Vec<PerpsPositionSnapshotResult>,
    #[serde(rename = "history")]
    pub history: Vec<PerpsPositionSnapshotResult>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsPositionSnapshots {
    fn default() -> Self {
        Self {
            history_as_of_at: 0.into(),
            active: Default::default(),
            history: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsPositionSnapshotsRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "active_instrument_ids", default, skip_serializing_if = "Option::is_none")]
    pub active_instrument_ids: Option<Vec<serde_json::Number>>,
    #[serde(rename = "history_fills", default, skip_serializing_if = "Option::is_none")]
    pub history_fills: Option<Vec<PerpsPositionSnapshotFill>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsProxyRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpCreateProxy>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsProxyResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsPublicPortfolio {
    #[serde(rename = "positions")]
    pub positions: Vec<PerpsPublicPortfolioPosition>,
    #[serde(rename = "equity")]
    pub equity: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsPublicPortfolio {
    fn default() -> Self {
        Self {
            positions: Default::default(),
            equity: Default::default(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsPublicPortfolioPosition {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "entry_price")]
    pub entry_price: String,
    #[serde(rename = "unrealized_pnl")]
    pub unrealized_pnl: String,
    #[serde(rename = "return_on_equity")]
    pub return_on_equity: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsPublicPortfolioPosition {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            symbol: Default::default(),
            size: Default::default(),
            entry_price: Default::default(),
            unrealized_pnl: Default::default(),
            return_on_equity: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsRegisteredResponse {
    #[serde(rename = "registered")]
    pub registered: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsRiskTier {
    #[serde(rename = "lower_bound")]
    pub lower_bound: String,
    #[serde(rename = "max_leverage")]
    pub max_leverage: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsRiskTier {
    fn default() -> Self {
        Self {
            lower_bound: Default::default(),
            max_leverage: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsStatistic {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "volume")]
    pub volume: String,
    #[serde(rename = "open_price")]
    pub open_price: String,
    #[serde(rename = "klines")]
    pub klines: PerpsKlines,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsStatistic {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            symbol: Default::default(),
            volume: Default::default(),
            open_price: Default::default(),
            klines: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsTicker {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "index_price")]
    pub index_price: String,
    #[serde(rename = "mark_price")]
    pub mark_price: String,
    #[serde(rename = "last_price")]
    pub last_price: String,
    #[serde(rename = "mid_price")]
    pub mid_price: String,
    #[serde(rename = "open_interest")]
    pub open_interest: String,
    #[serde(rename = "funding_rate")]
    pub funding_rate: String,
    #[serde(rename = "next_funding")]
    pub next_funding: serde_json::Number,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsTicker {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            symbol: Default::default(),
            index_price: Default::default(),
            mark_price: Default::default(),
            last_price: Default::default(),
            mid_price: Default::default(),
            open_interest: Default::default(),
            funding_rate: Default::default(),
            next_funding: 0.into(),
            timestamp: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsTickerData {
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "index_price")]
    pub index_price: String,
    #[serde(rename = "mark_price")]
    pub mark_price: String,
    #[serde(rename = "last_price")]
    pub last_price: String,
    #[serde(rename = "mid_price")]
    pub mid_price: String,
    #[serde(rename = "open_interest")]
    pub open_interest: String,
    #[serde(rename = "funding_rate")]
    pub funding_rate: String,
    #[serde(rename = "next_funding")]
    pub next_funding: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsTickerData {
    fn default() -> Self {
        Self {
            instrument_id: 0.into(),
            symbol: Default::default(),
            index_price: Default::default(),
            mark_price: Default::default(),
            last_price: Default::default(),
            mid_price: Default::default(),
            open_interest: Default::default(),
            funding_rate: Default::default(),
            next_funding: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsTime {
    #[serde(rename = "time")]
    pub time: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsTime {
    fn default() -> Self {
        Self {
            time: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsTpSlOrderFields {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "trp")]
    pub trp: String,
    #[serde(rename = "parent_oid", default, skip_serializing_if = "Option::is_none")]
    pub parent_oid: Option<serde_json::Number>,
    #[serde(rename = "armed_qty", default, skip_serializing_if = "Option::is_none")]
    pub armed_qty: Option<String>,
    #[serde(rename = "slip_bps", default, skip_serializing_if = "Option::is_none")]
    pub slip_bps: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsTradeData {
    #[serde(rename = "trade_id")]
    pub trade_id: serde_json::Number,
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "timestamp")]
    pub timestamp: serde_json::Number,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "settlement")]
    pub settlement: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsTradeData {
    fn default() -> Self {
        Self {
            trade_id: 0.into(),
            instrument_id: 0.into(),
            side: Default::default(),
            price: Default::default(),
            quantity: Default::default(),
            timestamp: 0.into(),
            hash: Default::default(),
            settlement: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsTrades {
    #[serde(rename = "data")]
    pub data: Vec<PerpsTradeData>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsWithdrawAccepted {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "withdraw_id")]
    pub withdraw_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsWithdrawAccepted {
    fn default() -> Self {
        Self {
            status: Default::default(),
            withdraw_id: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsWithdrawRejected {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "withdraw_id")]
    pub withdraw_id: serde_json::Number,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsWithdrawRejected {
    fn default() -> Self {
        Self {
            status: Default::default(),
            withdraw_id: 0.into(),
            error: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsWithdrawRequest {
    #[serde(rename = "op")]
    pub op: Box<PerpsOpWithdraw>,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsWithdrawal {
    #[serde(rename = "withdraw_id")]
    pub withdraw_id: serde_json::Number,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "confirmations")]
    pub confirmations: serde_json::Number,
    #[serde(rename = "required_confirmations")]
    pub required_confirmations: serde_json::Number,
    #[serde(rename = "created_timestamp")]
    pub created_timestamp: serde_json::Number,
    #[serde(rename = "confirmed_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub confirmed_timestamp: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsWithdrawal {
    fn default() -> Self {
        Self {
            withdraw_id: 0.into(),
            asset: Default::default(),
            amount: Default::default(),
            to: Default::default(),
            fee: Default::default(),
            status: Default::default(),
            hash: Default::default(),
            confirmations: 0.into(),
            required_confirmations: 0.into(),
            created_timestamp: 0.into(),
            confirmed_timestamp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsWithdrawals {
    #[serde(rename = "data")]
    pub data: Vec<PerpsWithdrawal>,
    #[serde(rename = "more")]
    pub more: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PostBlpEnrollParams {
    #[serde(rename = "body")]
    pub body: Box<PerpsBlpEnrollRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PostBlpEnrollResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetAutoCancelParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsAutoCancelRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SetAutoCancelResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "deadline")]
    pub deadline: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SetAutoCancelResponse {
    fn default() -> Self {
        Self {
            status: Default::default(),
            deadline: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateBuilderParams {
    #[serde(rename = "body")]
    pub body: UpdateBuilderParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateBuilderParamsBody {
    #[serde(rename = "op")]
    pub op: UpdateBuilderParamsBodyOp,
    #[serde(rename = "sig", default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[serde(rename = "salt", default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateBuilderParamsBodyOp {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "args")]
    pub args: UpdateBuilderParamsBodyOpArgs,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateBuilderParamsBodyOpArgs {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateBuilderResponse {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "registered")]
    pub registered: bool,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "admission_enabled")]
    pub admission_enabled: bool,
    #[serde(rename = "max_fee_rate")]
    pub max_fee_rate: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateLeverageParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsLeverageRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateLeverageResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "instrument_id")]
    pub instrument_id: serde_json::Number,
    #[serde(rename = "leverage")]
    pub leverage: serde_json::Number,
    #[serde(rename = "cross")]
    pub cross: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for UpdateLeverageResponse {
    fn default() -> Self {
        Self {
            status: Default::default(),
            instrument_id: 0.into(),
            leverage: 0.into(),
            cross: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateLeveragesParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsLeverageBatchRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateMarginParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsMarginRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WithdrawParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsWithdrawRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WithdrawResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "withdraw_id")]
    pub withdraw_id: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for WithdrawResponse {
    fn default() -> Self {
        Self {
            status: Default::default(),
            withdraw_id: 0.into(),
            extra: Default::default(),
        }
    }
}

pub struct TypedPolymarketPerpsClient(PolymarketPerpsClient);
impl PolymarketPerpsClient {
    pub fn typed(self) -> TypedPolymarketPerpsClient { TypedPolymarketPerpsClient(self) }
}
impl TypedPolymarketPerpsClient {
    pub fn apply_referral(&self, request: &ApplyReferralParams) -> Result<Box<PerpsGenericAccepted>> {
        serde_json::from_value(self.0.apply_referral(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn approve_builder(&self, request: &ApproveBuilderParams) -> Result<Box<PerpsBuilderApproval>> {
        serde_json::from_value(self.0.approve_builder(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn cancel_all_orders(&self, request: &CancelAllOrdersParams) -> Result<Box<PerpsGenericAccepted>> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn cancel_chase(&self, request: &CancelChaseParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.cancel_chase(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn cancel_orders(&self, request: &CancelOrdersParams) -> Result<Vec<PerpsCancelResponse>> {
        serde_json::from_value(self.0.cancel_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn cancel_orders_by_coid(&self, request: &CancelOrdersByCoidParams) -> Result<Vec<PerpsCancelResponse>> {
        serde_json::from_value(self.0.cancel_orders_by_coid(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn cancel_twap(&self, request: &CancelTwapParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.cancel_twap(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn check_invite_code(&self, request: &CheckInviteCodeParams) -> Result<Box<PerpsInviteCheckResponse>> {
        serde_json::from_value(self.0.check_invite_code(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn control_twap(&self, request: &ControlTwapParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.control_twap(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn create_account_invite(&self, request: &CreateAccountInviteParams) -> Result<Box<PerpsCreateInviteResponse>> {
        serde_json::from_value(self.0.create_account_invite(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn create_chase(&self, request: &CreateChaseParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.create_chase(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn create_orders(&self, request: &CreateOrdersParams) -> Result<Vec<PerpsOrderResponse>> {
        serde_json::from_value(self.0.create_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn create_proxy(&self, request: &CreateProxyParams) -> Result<Box<PerpsProxyResponse>> {
        serde_json::from_value(self.0.create_proxy(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn create_twap(&self, request: &CreateTwapParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.create_twap(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn delete_proxy(&self, request: &DeleteProxyParams) -> Result<Box<PerpsGenericAccepted>> {
        serde_json::from_value(self.0.delete_proxy(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_account_backstops(&self, request: &GetAccountBackstopsParams) -> Result<Box<PerpsAccountBackstops>> {
        serde_json::from_value(self.0.get_account_backstops(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_account_funding(&self, request: &GetAccountFundingParams) -> Result<Box<PerpsAccountFundingHistory>> {
        serde_json::from_value(self.0.get_account_funding(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_account_limits(&self, request: &GetAccountLimitsParams) -> Result<Box<PerpsAccountLimits>> {
        serde_json::from_value(self.0.get_account_limits(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_account_notifications(&self, request: &GetAccountNotificationsParams) -> Result<Box<PerpsNotificationsResponse>> {
        serde_json::from_value(self.0.get_account_notifications(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_account_referral(&self, request: &GetAccountReferralParams) -> Result<Box<PerpsAccountReferral>> {
        serde_json::from_value(self.0.get_account_referral(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_account_rewards(&self, request: &GetAccountRewardsParams) -> Result<Box<PerpsAccountRewards>> {
        serde_json::from_value(self.0.get_account_rewards(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_account_stats(&self, request: &GetAccountStatsParams) -> Result<Box<PerpsAccountStats>> {
        serde_json::from_value(self.0.get_account_stats(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_assets(&self, request: &GetAssetsParams) -> Result<Vec<PerpsAsset>> {
        serde_json::from_value(self.0.get_assets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_auto_cancel(&self, request: &GetAutoCancelParams) -> Result<Box<PerpsAutoCancelStatus>> {
        serde_json::from_value(self.0.get_auto_cancel(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_balances(&self, request: &GetBalancesParams) -> Result<Vec<PerpsBalance>> {
        serde_json::from_value(self.0.get_balances(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_bbo(&self, request: &GetBboParams) -> Result<Vec<PerpsBBO>> {
        serde_json::from_value(self.0.get_bbo(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_blp_enrollment(&self, request: &GetBlpEnrollmentParams) -> Result<Box<PerpsBlpEnrollment>> {
        serde_json::from_value(self.0.get_blp_enrollment(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_blp_liquidations(&self, request: &GetBlpLiquidationsParams) -> Result<Box<PerpsBlpLiquidations>> {
        serde_json::from_value(self.0.get_blp_liquidations(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_book(&self, request: &GetBookParams) -> Result<Box<PerpsBook>> {
        serde_json::from_value(self.0.get_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_builder(&self, request: &GetBuilderParams) -> Result<GetBuilderResponse> {
        serde_json::from_value(self.0.get_builder(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_builder_approvals(&self, request: &GetBuilderApprovalsParams) -> Result<Box<PerpsBuilderApprovals>> {
        serde_json::from_value(self.0.get_builder_approvals(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_builder_earnings(&self, request: &GetBuilderEarningsParams) -> Result<Box<PerpsBuilderEarnings>> {
        serde_json::from_value(self.0.get_builder_earnings(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_builder_earnings_summary(&self, request: &GetBuilderEarningsSummaryParams) -> Result<Box<PerpsBuilderEarningsSummary>> {
        serde_json::from_value(self.0.get_builder_earnings_summary(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_chases(&self, request: &GetChasesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_chases(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_competition(&self, request: &GetCompetitionParams) -> Result<Box<PerpsCompetitionDetail>> {
        serde_json::from_value(self.0.get_competition(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_competition_account(&self, request: &GetCompetitionAccountParams) -> Result<Box<PerpsCompetitionAccount>> {
        serde_json::from_value(self.0.get_competition_account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_competition_leaderboard(&self, request: &GetCompetitionLeaderboardParams) -> Result<Box<PerpsCompetitionLeaderboard>> {
        serde_json::from_value(self.0.get_competition_leaderboard(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_competition_widget(&self, request: &GetCompetitionWidgetParams) -> Result<Box<PerpsCompetitionWidget>> {
        serde_json::from_value(self.0.get_competition_widget(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_competitions(&self, request: &GetCompetitionsParams) -> Result<Box<PerpsCompetitionList>> {
        serde_json::from_value(self.0.get_competitions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_config(&self, request: &GetConfigParams) -> Result<Vec<PerpsAccountConfig>> {
        serde_json::from_value(self.0.get_config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_credentials(&self, request: &GetCredentialsParams) -> Result<Box<PerpsCredentials>> {
        serde_json::from_value(self.0.get_credentials(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_deposits(&self, request: &GetDepositsParams) -> Result<Box<PerpsDeposits>> {
        serde_json::from_value(self.0.get_deposits(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_equity_history(&self, request: &GetEquityHistoryParams) -> Result<Box<PerpsEquityHistory>> {
        serde_json::from_value(self.0.get_equity_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_exchange(&self, request: &GetExchangeParams) -> Result<Box<PerpsExchange>> {
        serde_json::from_value(self.0.get_exchange(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_exchange_statistics(&self, request: &GetExchangeStatisticsParams) -> Result<Box<PerpsExchangeStatistics>> {
        serde_json::from_value(self.0.get_exchange_statistics(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_fills(&self, request: &GetFillsParams) -> Result<Box<PerpsAccountTrades>> {
        serde_json::from_value(self.0.get_fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_funding_history(&self, request: &GetFundingHistoryParams) -> Result<Box<PerpsFundingHistory>> {
        serde_json::from_value(self.0.get_funding_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_index(&self, request: &GetIndexParams) -> Result<Box<PerpsIndex>> {
        serde_json::from_value(self.0.get_index(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_info_fees(&self, request: &GetInfoFeesParams) -> Result<Box<PerpsFeesInfo>> {
        serde_json::from_value(self.0.get_info_fees(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_instruments(&self, request: &GetInstrumentsParams) -> Result<Vec<PerpsInstrument>> {
        serde_json::from_value(self.0.get_instruments(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_internal_transfers(&self, request: &GetInternalTransfersParams) -> Result<Box<PerpsInternalTransfers>> {
        serde_json::from_value(self.0.get_internal_transfers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_klines(&self, request: &GetKlinesParams) -> Result<Box<PerpsKlinesResponse>> {
        serde_json::from_value(self.0.get_klines(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_leaderboard(&self, request: &GetLeaderboardParams) -> Result<Box<PerpsLeaderboard>> {
        serde_json::from_value(self.0.get_leaderboard(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_limit_tiers(&self, request: &GetLimitTiersParams) -> Result<Vec<PerpsLimitTier>> {
        serde_json::from_value(self.0.get_limit_tiers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_mark_history(&self, request: &GetMarkHistoryParams) -> Result<Box<PerpsMarkHistoryResponse>> {
        serde_json::from_value(self.0.get_mark_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_open_orders(&self, request: &GetOpenOrdersParams) -> Result<Vec<PerpsOrderData>> {
        serde_json::from_value(self.0.get_open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_orders(&self, request: &GetOrdersParams) -> Result<Vec<PerpsOrderData>> {
        serde_json::from_value(self.0.get_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_ping(&self, request: &GetPingParams) -> Result<GetPingResponse> {
        serde_json::from_value(self.0.get_ping(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_pnl_history(&self, request: &GetPnlHistoryParams) -> Result<Box<PerpsPnlHistory>> {
        serde_json::from_value(self.0.get_pnl_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_portfolio(&self, request: &GetPortfolioParams) -> Result<Box<PerpsPortfolio>> {
        serde_json::from_value(self.0.get_portfolio(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_public_portfolio(&self, request: &GetPublicPortfolioParams) -> Result<Box<PerpsPublicPortfolio>> {
        serde_json::from_value(self.0.get_public_portfolio(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_public_position_fills(&self, request: &GetPublicPositionFillsParams) -> Result<Box<PerpsAccountTrades>> {
        serde_json::from_value(self.0.get_public_position_fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_public_position_snapshots(&self, request: &GetPublicPositionSnapshotsParams) -> Result<Box<PerpsPositionSnapshots>> {
        serde_json::from_value(self.0.get_public_position_snapshots(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_registered(&self, request: &GetRegisteredParams) -> Result<Box<PerpsRegisteredResponse>> {
        serde_json::from_value(self.0.get_registered(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_statistics(&self, request: &GetStatisticsParams) -> Result<Vec<PerpsStatistic>> {
        serde_json::from_value(self.0.get_statistics(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_tickers(&self, request: &GetTickersParams) -> Result<Vec<PerpsTicker>> {
        serde_json::from_value(self.0.get_tickers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_time(&self, request: &GetTimeParams) -> Result<Box<PerpsTime>> {
        serde_json::from_value(self.0.get_time(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_trades(&self, request: &GetTradesParams) -> Result<Box<PerpsTrades>> {
        serde_json::from_value(self.0.get_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_twaps(&self, request: &GetTwapsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_twaps(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn get_withdrawals(&self, request: &GetWithdrawalsParams) -> Result<Box<PerpsWithdrawals>> {
        serde_json::from_value(self.0.get_withdrawals(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn internal_transfer(&self, request: &InternalTransferParams) -> Result<Box<PerpsInternalTransferAccepted>> {
        serde_json::from_value(self.0.internal_transfer(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn mark_account_notifications_read(&self, request: &MarkAccountNotificationsReadParams) -> Result<Box<PerpsGenericAccepted>> {
        serde_json::from_value(self.0.mark_account_notifications_read(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn modify_orders(&self, request: &ModifyOrdersParams) -> Result<Vec<PerpsModifyResponse>> {
        serde_json::from_value(self.0.modify_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn modify_orders_by_coid(&self, request: &ModifyOrdersByCoidParams) -> Result<Vec<PerpsModifyResponse>> {
        serde_json::from_value(self.0.modify_orders_by_coid(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn post_blp_enroll(&self, request: &PostBlpEnrollParams) -> Result<Box<PerpsGenericAccepted>> {
        serde_json::from_value(self.0.post_blp_enroll(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn set_auto_cancel(&self, request: &SetAutoCancelParams) -> Result<Box<PerpsAutoCancelResponse>> {
        serde_json::from_value(self.0.set_auto_cancel(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn update_builder(&self, request: &UpdateBuilderParams) -> Result<UpdateBuilderResponse> {
        serde_json::from_value(self.0.update_builder(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn update_leverage(&self, request: &UpdateLeverageParams) -> Result<Box<PerpsLeverageResponse>> {
        serde_json::from_value(self.0.update_leverage(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn update_leverages(&self, request: &UpdateLeveragesParams) -> Result<Vec<PerpsLeverageBatchResponse>> {
        serde_json::from_value(self.0.update_leverages(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn update_margin(&self, request: &UpdateMarginParams) -> Result<Box<PerpsGenericResponse>> {
        serde_json::from_value(self.0.update_margin(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub fn withdraw(&self, request: &WithdrawParams) -> Result<Box<PerpsWithdrawAccepted>> {
        serde_json::from_value(self.0.withdraw(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedPolymarketPerpsClient(crate::AsyncClient<PolymarketPerpsClient>);
#[cfg(feature = "async")]
impl TypedPolymarketPerpsClient {
    pub fn into_async(self) -> AsyncTypedPolymarketPerpsClient { AsyncTypedPolymarketPerpsClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedPolymarketPerpsClient {
    pub async fn apply_referral(&self, request: &ApplyReferralParams) -> Result<Box<PerpsGenericAccepted>> {
        serde_json::from_value(self.0.apply_referral(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn approve_builder(&self, request: &ApproveBuilderParams) -> Result<Box<PerpsBuilderApproval>> {
        serde_json::from_value(self.0.approve_builder(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn cancel_all_orders(&self, request: &CancelAllOrdersParams) -> Result<Box<PerpsGenericAccepted>> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn cancel_chase(&self, request: &CancelChaseParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.cancel_chase(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn cancel_orders(&self, request: &CancelOrdersParams) -> Result<Vec<PerpsCancelResponse>> {
        serde_json::from_value(self.0.cancel_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn cancel_orders_by_coid(&self, request: &CancelOrdersByCoidParams) -> Result<Vec<PerpsCancelResponse>> {
        serde_json::from_value(self.0.cancel_orders_by_coid(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn cancel_twap(&self, request: &CancelTwapParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.cancel_twap(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn check_invite_code(&self, request: &CheckInviteCodeParams) -> Result<Box<PerpsInviteCheckResponse>> {
        serde_json::from_value(self.0.check_invite_code(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn control_twap(&self, request: &ControlTwapParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.control_twap(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn create_account_invite(&self, request: &CreateAccountInviteParams) -> Result<Box<PerpsCreateInviteResponse>> {
        serde_json::from_value(self.0.create_account_invite(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn create_chase(&self, request: &CreateChaseParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.create_chase(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn create_orders(&self, request: &CreateOrdersParams) -> Result<Vec<PerpsOrderResponse>> {
        serde_json::from_value(self.0.create_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn create_proxy(&self, request: &CreateProxyParams) -> Result<Box<PerpsProxyResponse>> {
        serde_json::from_value(self.0.create_proxy(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn create_twap(&self, request: &CreateTwapParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.create_twap(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn delete_proxy(&self, request: &DeleteProxyParams) -> Result<Box<PerpsGenericAccepted>> {
        serde_json::from_value(self.0.delete_proxy(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_account_backstops(&self, request: &GetAccountBackstopsParams) -> Result<Box<PerpsAccountBackstops>> {
        serde_json::from_value(self.0.get_account_backstops(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_account_funding(&self, request: &GetAccountFundingParams) -> Result<Box<PerpsAccountFundingHistory>> {
        serde_json::from_value(self.0.get_account_funding(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_account_limits(&self, request: &GetAccountLimitsParams) -> Result<Box<PerpsAccountLimits>> {
        serde_json::from_value(self.0.get_account_limits(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_account_notifications(&self, request: &GetAccountNotificationsParams) -> Result<Box<PerpsNotificationsResponse>> {
        serde_json::from_value(self.0.get_account_notifications(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_account_referral(&self, request: &GetAccountReferralParams) -> Result<Box<PerpsAccountReferral>> {
        serde_json::from_value(self.0.get_account_referral(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_account_rewards(&self, request: &GetAccountRewardsParams) -> Result<Box<PerpsAccountRewards>> {
        serde_json::from_value(self.0.get_account_rewards(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_account_stats(&self, request: &GetAccountStatsParams) -> Result<Box<PerpsAccountStats>> {
        serde_json::from_value(self.0.get_account_stats(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_assets(&self, request: &GetAssetsParams) -> Result<Vec<PerpsAsset>> {
        serde_json::from_value(self.0.get_assets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_auto_cancel(&self, request: &GetAutoCancelParams) -> Result<Box<PerpsAutoCancelStatus>> {
        serde_json::from_value(self.0.get_auto_cancel(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_balances(&self, request: &GetBalancesParams) -> Result<Vec<PerpsBalance>> {
        serde_json::from_value(self.0.get_balances(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_bbo(&self, request: &GetBboParams) -> Result<Vec<PerpsBBO>> {
        serde_json::from_value(self.0.get_bbo(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_blp_enrollment(&self, request: &GetBlpEnrollmentParams) -> Result<Box<PerpsBlpEnrollment>> {
        serde_json::from_value(self.0.get_blp_enrollment(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_blp_liquidations(&self, request: &GetBlpLiquidationsParams) -> Result<Box<PerpsBlpLiquidations>> {
        serde_json::from_value(self.0.get_blp_liquidations(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_book(&self, request: &GetBookParams) -> Result<Box<PerpsBook>> {
        serde_json::from_value(self.0.get_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_builder(&self, request: &GetBuilderParams) -> Result<GetBuilderResponse> {
        serde_json::from_value(self.0.get_builder(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_builder_approvals(&self, request: &GetBuilderApprovalsParams) -> Result<Box<PerpsBuilderApprovals>> {
        serde_json::from_value(self.0.get_builder_approvals(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_builder_earnings(&self, request: &GetBuilderEarningsParams) -> Result<Box<PerpsBuilderEarnings>> {
        serde_json::from_value(self.0.get_builder_earnings(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_builder_earnings_summary(&self, request: &GetBuilderEarningsSummaryParams) -> Result<Box<PerpsBuilderEarningsSummary>> {
        serde_json::from_value(self.0.get_builder_earnings_summary(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_chases(&self, request: &GetChasesParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_chases(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_competition(&self, request: &GetCompetitionParams) -> Result<Box<PerpsCompetitionDetail>> {
        serde_json::from_value(self.0.get_competition(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_competition_account(&self, request: &GetCompetitionAccountParams) -> Result<Box<PerpsCompetitionAccount>> {
        serde_json::from_value(self.0.get_competition_account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_competition_leaderboard(&self, request: &GetCompetitionLeaderboardParams) -> Result<Box<PerpsCompetitionLeaderboard>> {
        serde_json::from_value(self.0.get_competition_leaderboard(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_competition_widget(&self, request: &GetCompetitionWidgetParams) -> Result<Box<PerpsCompetitionWidget>> {
        serde_json::from_value(self.0.get_competition_widget(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_competitions(&self, request: &GetCompetitionsParams) -> Result<Box<PerpsCompetitionList>> {
        serde_json::from_value(self.0.get_competitions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_config(&self, request: &GetConfigParams) -> Result<Vec<PerpsAccountConfig>> {
        serde_json::from_value(self.0.get_config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_credentials(&self, request: &GetCredentialsParams) -> Result<Box<PerpsCredentials>> {
        serde_json::from_value(self.0.get_credentials(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_deposits(&self, request: &GetDepositsParams) -> Result<Box<PerpsDeposits>> {
        serde_json::from_value(self.0.get_deposits(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_equity_history(&self, request: &GetEquityHistoryParams) -> Result<Box<PerpsEquityHistory>> {
        serde_json::from_value(self.0.get_equity_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_exchange(&self, request: &GetExchangeParams) -> Result<Box<PerpsExchange>> {
        serde_json::from_value(self.0.get_exchange(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_exchange_statistics(&self, request: &GetExchangeStatisticsParams) -> Result<Box<PerpsExchangeStatistics>> {
        serde_json::from_value(self.0.get_exchange_statistics(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_fills(&self, request: &GetFillsParams) -> Result<Box<PerpsAccountTrades>> {
        serde_json::from_value(self.0.get_fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_funding_history(&self, request: &GetFundingHistoryParams) -> Result<Box<PerpsFundingHistory>> {
        serde_json::from_value(self.0.get_funding_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_index(&self, request: &GetIndexParams) -> Result<Box<PerpsIndex>> {
        serde_json::from_value(self.0.get_index(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_info_fees(&self, request: &GetInfoFeesParams) -> Result<Box<PerpsFeesInfo>> {
        serde_json::from_value(self.0.get_info_fees(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_instruments(&self, request: &GetInstrumentsParams) -> Result<Vec<PerpsInstrument>> {
        serde_json::from_value(self.0.get_instruments(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_internal_transfers(&self, request: &GetInternalTransfersParams) -> Result<Box<PerpsInternalTransfers>> {
        serde_json::from_value(self.0.get_internal_transfers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_klines(&self, request: &GetKlinesParams) -> Result<Box<PerpsKlinesResponse>> {
        serde_json::from_value(self.0.get_klines(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_leaderboard(&self, request: &GetLeaderboardParams) -> Result<Box<PerpsLeaderboard>> {
        serde_json::from_value(self.0.get_leaderboard(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_limit_tiers(&self, request: &GetLimitTiersParams) -> Result<Vec<PerpsLimitTier>> {
        serde_json::from_value(self.0.get_limit_tiers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_mark_history(&self, request: &GetMarkHistoryParams) -> Result<Box<PerpsMarkHistoryResponse>> {
        serde_json::from_value(self.0.get_mark_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_open_orders(&self, request: &GetOpenOrdersParams) -> Result<Vec<PerpsOrderData>> {
        serde_json::from_value(self.0.get_open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_orders(&self, request: &GetOrdersParams) -> Result<Vec<PerpsOrderData>> {
        serde_json::from_value(self.0.get_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_ping(&self, request: &GetPingParams) -> Result<GetPingResponse> {
        serde_json::from_value(self.0.get_ping(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_pnl_history(&self, request: &GetPnlHistoryParams) -> Result<Box<PerpsPnlHistory>> {
        serde_json::from_value(self.0.get_pnl_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_portfolio(&self, request: &GetPortfolioParams) -> Result<Box<PerpsPortfolio>> {
        serde_json::from_value(self.0.get_portfolio(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_public_portfolio(&self, request: &GetPublicPortfolioParams) -> Result<Box<PerpsPublicPortfolio>> {
        serde_json::from_value(self.0.get_public_portfolio(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_public_position_fills(&self, request: &GetPublicPositionFillsParams) -> Result<Box<PerpsAccountTrades>> {
        serde_json::from_value(self.0.get_public_position_fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_public_position_snapshots(&self, request: &GetPublicPositionSnapshotsParams) -> Result<Box<PerpsPositionSnapshots>> {
        serde_json::from_value(self.0.get_public_position_snapshots(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_registered(&self, request: &GetRegisteredParams) -> Result<Box<PerpsRegisteredResponse>> {
        serde_json::from_value(self.0.get_registered(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_statistics(&self, request: &GetStatisticsParams) -> Result<Vec<PerpsStatistic>> {
        serde_json::from_value(self.0.get_statistics(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_tickers(&self, request: &GetTickersParams) -> Result<Vec<PerpsTicker>> {
        serde_json::from_value(self.0.get_tickers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_time(&self, request: &GetTimeParams) -> Result<Box<PerpsTime>> {
        serde_json::from_value(self.0.get_time(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_trades(&self, request: &GetTradesParams) -> Result<Box<PerpsTrades>> {
        serde_json::from_value(self.0.get_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_twaps(&self, request: &GetTwapsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_twaps(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn get_withdrawals(&self, request: &GetWithdrawalsParams) -> Result<Box<PerpsWithdrawals>> {
        serde_json::from_value(self.0.get_withdrawals(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn internal_transfer(&self, request: &InternalTransferParams) -> Result<Box<PerpsInternalTransferAccepted>> {
        serde_json::from_value(self.0.internal_transfer(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn mark_account_notifications_read(&self, request: &MarkAccountNotificationsReadParams) -> Result<Box<PerpsGenericAccepted>> {
        serde_json::from_value(self.0.mark_account_notifications_read(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn modify_orders(&self, request: &ModifyOrdersParams) -> Result<Vec<PerpsModifyResponse>> {
        serde_json::from_value(self.0.modify_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn modify_orders_by_coid(&self, request: &ModifyOrdersByCoidParams) -> Result<Vec<PerpsModifyResponse>> {
        serde_json::from_value(self.0.modify_orders_by_coid(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn post_blp_enroll(&self, request: &PostBlpEnrollParams) -> Result<Box<PerpsGenericAccepted>> {
        serde_json::from_value(self.0.post_blp_enroll(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn set_auto_cancel(&self, request: &SetAutoCancelParams) -> Result<Box<PerpsAutoCancelResponse>> {
        serde_json::from_value(self.0.set_auto_cancel(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn update_builder(&self, request: &UpdateBuilderParams) -> Result<UpdateBuilderResponse> {
        serde_json::from_value(self.0.update_builder(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn update_leverage(&self, request: &UpdateLeverageParams) -> Result<Box<PerpsLeverageResponse>> {
        serde_json::from_value(self.0.update_leverage(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn update_leverages(&self, request: &UpdateLeveragesParams) -> Result<Vec<PerpsLeverageBatchResponse>> {
        serde_json::from_value(self.0.update_leverages(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn update_margin(&self, request: &UpdateMarginParams) -> Result<Box<PerpsGenericResponse>> {
        serde_json::from_value(self.0.update_margin(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
    pub async fn withdraw(&self, request: &WithdrawParams) -> Result<Box<PerpsWithdrawAccepted>> {
        serde_json::from_value(self.0.withdraw(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PolymarketPerps response model"))
    }
}
