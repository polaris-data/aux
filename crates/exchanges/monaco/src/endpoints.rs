use crate::*;
use aux_rs_core::{
    Endpoint, Error as ClientError, HttpMethod, HttpRequest, HttpResponse, HttpTransport, Public,
};
use serde::{Deserialize, Serialize};
/// Access marker for endpoints that require an active Monaco session.
pub struct SessionRequired;
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TradingPairsRequest {
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page: Option<u64>,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_size: Option<u64>,
    #[serde(
        rename = "marketType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub market_type: Option<String>,
    #[serde(rename = "baseToken", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub base_token: Option<String>,
    #[serde(
        rename = "quoteToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_token: Option<String>,
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub is_active: Option<bool>,
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub category: Option<String>,
}
impl Endpoint<Monaco> for TradingPairsRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = ListTradingPairsResponse;
    type Access = Public;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let mut query = Vec::new();
        if let Some(value) = &self.page {
            if *value < 1 {
                return Err(Error::InvalidRequest("page below minimum"));
            }
            if *value > 10000 {
                return Err(Error::InvalidRequest("page above maximum"));
            }
            query.push(("page".into(), value.to_string()));
        }
        if let Some(value) = &self.page_size {
            if *value < 1 {
                return Err(Error::InvalidRequest("pageSize below minimum"));
            }
            if *value > 100 {
                return Err(Error::InvalidRequest("pageSize above maximum"));
            }
            query.push(("pageSize".into(), value.to_string()));
        }
        if let Some(value) = &self.market_type {
            one_of(value, &["SPOT", "MARGIN"])?;
            query.push(("marketType".into(), value.to_string()));
        }
        if let Some(value) = &self.base_token {
            query.push(("baseToken".into(), value.to_string()));
        }
        if let Some(value) = &self.quote_token {
            query.push(("quoteToken".into(), value.to_string()));
        }
        if let Some(value) = &self.is_active {
            query.push(("isActive".into(), value.to_string()));
        }
        if let Some(value) = &self.category {
            query.push(("category".into(), value.to_string()));
        }
        wire(e, HttpMethod::Get, "/api/v1/market/pairs", &query, None)
    }
    fn decode(r: HttpResponse) -> Result<ListTradingPairsResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, A: Sync> MonacoClient<T, A> {
    pub async fn trading_pairs(
        &self,
        request: TradingPairsRequest,
    ) -> Result<ListTradingPairsResponse, ClientError<Error>> {
        self.request(request).await
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TradingPairRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
}
impl Endpoint<Monaco> for TradingPairRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = GetTradingPairResponse;
    type Access = Public;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let query = Vec::new();
        uuid(&self.trading_pair_id)?;
        wire(
            e,
            HttpMethod::Get,
            &format!(
                "/api/v1/market/pairs/{}",
                aux_rs_core::url_component(&self.trading_pair_id)
            ),
            &query,
            None,
        )
    }
    fn decode(r: HttpResponse) -> Result<GetTradingPairResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, A: Sync> MonacoClient<T, A> {
    pub async fn trading_pair(
        &self,
        request: TradingPairRequest,
    ) -> Result<GetTradingPairResponse, ClientError<Error>> {
        self.request(request).await
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MarketMetadataRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
}
impl Endpoint<Monaco> for MarketMetadataRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = GetMarketMetadataResponse;
    type Access = Public;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let query = Vec::new();
        uuid(&self.trading_pair_id)?;
        wire(
            e,
            HttpMethod::Get,
            &format!(
                "/api/v1/market/pairs/{}/metadata",
                aux_rs_core::url_component(&self.trading_pair_id)
            ),
            &query,
            None,
        )
    }
    fn decode(r: HttpResponse) -> Result<GetMarketMetadataResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, A: Sync> MonacoClient<T, A> {
    pub async fn market_metadata(
        &self,
        request: MarketMetadataRequest,
    ) -> Result<GetMarketMetadataResponse, ClientError<Error>> {
        self.request(request).await
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OrderBookRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
    #[serde(rename = "levels", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub levels: Option<u64>,
    #[serde(
        rename = "tradingMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_mode: Option<String>,
    #[serde(rename = "magnitude", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub magnitude: Option<String>,
    #[serde(
        rename = "denomination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub denomination: Option<String>,
}
impl Endpoint<Monaco> for OrderBookRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = GetOrderbookResponse;
    type Access = Public;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let mut query = Vec::new();
        uuid(&self.trading_pair_id)?;
        if let Some(value) = &self.levels {
            if *value < 1 {
                return Err(Error::InvalidRequest("levels below minimum"));
            }
            if *value > 1000 {
                return Err(Error::InvalidRequest("levels above maximum"));
            }
            query.push(("levels".into(), value.to_string()));
        }
        if let Some(value) = &self.trading_mode {
            one_of(value, &["SPOT", "MARGIN"])?;
            query.push(("tradingMode".into(), value.to_string()));
        }
        if let Some(value) = &self.magnitude {
            query.push(("magnitude".into(), value.to_string()));
        }
        if let Some(value) = &self.denomination {
            query.push(("denomination".into(), value.to_string()));
        }
        wire(
            e,
            HttpMethod::Get,
            &format!(
                "/api/v1/orderbook/{}",
                aux_rs_core::url_component(&self.trading_pair_id)
            ),
            &query,
            None,
        )
    }
    fn decode(r: HttpResponse) -> Result<GetOrderbookResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, A: Sync> MonacoClient<T, A> {
    pub async fn order_book(
        &self,
        request: OrderBookRequest,
    ) -> Result<GetOrderbookResponse, ClientError<Error>> {
        self.request(request).await
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TradesRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_size: Option<u64>,
    #[serde(rename = "pageToken", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_token: Option<String>,
}
impl Endpoint<Monaco> for TradesRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = GetTradesResponse;
    type Access = Public;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let mut query = Vec::new();
        uuid(&self.trading_pair_id)?;
        if let Some(value) = &self.page_size {
            if *value < 1 {
                return Err(Error::InvalidRequest("pageSize below minimum"));
            }
            if *value > 1000 {
                return Err(Error::InvalidRequest("pageSize above maximum"));
            }
            query.push(("pageSize".into(), value.to_string()));
        }
        if let Some(value) = &self.page_token {
            query.push(("pageToken".into(), value.to_string()));
        }
        wire(
            e,
            HttpMethod::Get,
            &format!(
                "/api/v1/trades/{}",
                aux_rs_core::url_component(&self.trading_pair_id)
            ),
            &query,
            None,
        )
    }
    fn decode(r: HttpResponse) -> Result<GetTradesResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, A: Sync> MonacoClient<T, A> {
    pub async fn trades(
        &self,
        request: TradesRequest,
    ) -> Result<GetTradesResponse, ClientError<Error>> {
        self.request(request).await
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MarkPriceRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
}
impl Endpoint<Monaco> for MarkPriceRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = GetMarkPriceResponse;
    type Access = Public;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let query = Vec::new();
        uuid(&self.trading_pair_id)?;
        wire(
            e,
            HttpMethod::Get,
            &format!(
                "/api/v1/market/pairs/{}/mark-price",
                aux_rs_core::url_component(&self.trading_pair_id)
            ),
            &query,
            None,
        )
    }
    fn decode(r: HttpResponse) -> Result<GetMarkPriceResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, A: Sync> MonacoClient<T, A> {
    pub async fn mark_price(
        &self,
        request: MarkPriceRequest,
    ) -> Result<GetMarkPriceResponse, ClientError<Error>> {
        self.request(request).await
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IndexPriceRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
}
impl Endpoint<Monaco> for IndexPriceRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = GetIndexPriceResponse;
    type Access = Public;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let query = Vec::new();
        uuid(&self.trading_pair_id)?;
        wire(
            e,
            HttpMethod::Get,
            &format!(
                "/api/v1/market/pairs/{}/index-price",
                aux_rs_core::url_component(&self.trading_pair_id)
            ),
            &query,
            None,
        )
    }
    fn decode(r: HttpResponse) -> Result<GetIndexPriceResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, A: Sync> MonacoClient<T, A> {
    pub async fn index_price(
        &self,
        request: IndexPriceRequest,
    ) -> Result<GetIndexPriceResponse, ClientError<Error>> {
        self.request(request).await
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FundingRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
}
impl Endpoint<Monaco> for FundingRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = GetFundingStateResponse;
    type Access = Public;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let query = Vec::new();
        uuid(&self.trading_pair_id)?;
        wire(
            e,
            HttpMethod::Get,
            &format!(
                "/api/v1/market/pairs/{}/funding",
                aux_rs_core::url_component(&self.trading_pair_id)
            ),
            &query,
            None,
        )
    }
    fn decode(r: HttpResponse) -> Result<GetFundingStateResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, A: Sync> MonacoClient<T, A> {
    pub async fn funding(
        &self,
        request: FundingRequest,
    ) -> Result<GetFundingStateResponse, ClientError<Error>> {
        self.request(request).await
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FundingHistoryRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub start_time: Option<u64>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub end_time: Option<u64>,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_size: Option<u64>,
    #[serde(rename = "orderBy", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub order_by: Option<String>,
    #[serde(rename = "pageToken", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_token: Option<String>,
}
impl Endpoint<Monaco> for FundingHistoryRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = ListFundingHistoryResponse;
    type Access = Public;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let mut query = Vec::new();
        uuid(&self.trading_pair_id)?;
        if let Some(value) = &self.start_time {
            if *value > 4102444800000 {
                return Err(Error::InvalidRequest("startTime above maximum"));
            }
            query.push(("startTime".into(), value.to_string()));
        }
        if let Some(value) = &self.end_time {
            if *value > 4102444800000 {
                return Err(Error::InvalidRequest("endTime above maximum"));
            }
            query.push(("endTime".into(), value.to_string()));
        }
        if let Some(value) = &self.page_size {
            if *value < 1 {
                return Err(Error::InvalidRequest("pageSize below minimum"));
            }
            if *value > 1000 {
                return Err(Error::InvalidRequest("pageSize above maximum"));
            }
            query.push(("pageSize".into(), value.to_string()));
        }
        if let Some(value) = &self.order_by {
            query.push(("orderBy".into(), value.to_string()));
        }
        if let Some(value) = &self.page_token {
            query.push(("pageToken".into(), value.to_string()));
        }
        wire(
            e,
            HttpMethod::Get,
            &format!(
                "/api/v1/market/pairs/{}/funding/history",
                aux_rs_core::url_component(&self.trading_pair_id)
            ),
            &query,
            None,
        )
    }
    fn decode(r: HttpResponse) -> Result<ListFundingHistoryResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, A: Sync> MonacoClient<T, A> {
    pub async fn funding_history(
        &self,
        request: FundingHistoryRequest,
    ) -> Result<ListFundingHistoryResponse, ClientError<Error>> {
        self.request(request).await
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BalancesRequest {
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_size: Option<u64>,
    #[serde(rename = "pageToken", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_token: Option<String>,
}
impl Endpoint<Monaco> for BalancesRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = GetBalancesResponse;
    type Access = SessionRequired;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let mut query = Vec::new();
        if let Some(value) = &self.page_size {
            if *value < 1 {
                return Err(Error::InvalidRequest("pageSize below minimum"));
            }
            if *value > 1000 {
                return Err(Error::InvalidRequest("pageSize above maximum"));
            }
            query.push(("pageSize".into(), value.to_string()));
        }
        if let Some(value) = &self.page_token {
            query.push(("pageToken".into(), value.to_string()));
        }
        wire(
            e,
            HttpMethod::Get,
            "/api/v1/accounts/balances",
            &query,
            None,
        )
    }
    fn decode(r: HttpResponse) -> Result<GetBalancesResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, S: SessionSigner> MonacoClient<T, MonacoAuth<S>> {
    pub async fn balances(
        &self,
        request: BalancesRequest,
    ) -> Result<GetBalancesResponse, ClientError<Error>> {
        self.signed_request(request).await
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParentMarginAccountRequest {
    #[serde(
        rename = "tradingPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair_id: Option<String>,
}
impl Endpoint<Monaco> for ParentMarginAccountRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = GetMarginAccountSummaryResponse;
    type Access = SessionRequired;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let mut query = Vec::new();
        if let Some(value) = &self.trading_pair_id {
            uuid(value)?;
            query.push(("tradingPairId".into(), value.to_string()));
        }
        wire(
            e,
            HttpMethod::Get,
            "/api/v1/margin/parent-margin-account",
            &query,
            None,
        )
    }
    fn decode(r: HttpResponse) -> Result<GetMarginAccountSummaryResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, S: SessionSigner> MonacoClient<T, MonacoAuth<S>> {
    pub async fn parent_margin_account(
        &self,
        request: ParentMarginAccountRequest,
    ) -> Result<GetMarginAccountSummaryResponse, ClientError<Error>> {
        self.signed_request(request).await
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MarginAccountsRequest {
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_size: Option<u64>,
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub state: Option<String>,
    #[serde(
        rename = "tradingPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "pageToken", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_token: Option<String>,
}
impl Endpoint<Monaco> for MarginAccountsRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = ListMarginAccountsResponse;
    type Access = SessionRequired;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let mut query = Vec::new();
        if let Some(value) = &self.page_size {
            if *value < 1 {
                return Err(Error::InvalidRequest("pageSize below minimum"));
            }
            if *value > 1000 {
                return Err(Error::InvalidRequest("pageSize above maximum"));
            }
            query.push(("pageSize".into(), value.to_string()));
        }
        if let Some(value) = &self.state {
            query.push(("state".into(), value.to_string()));
        }
        if let Some(value) = &self.trading_pair_id {
            uuid(value)?;
            query.push(("tradingPairId".into(), value.to_string()));
        }
        if let Some(value) = &self.page_token {
            query.push(("pageToken".into(), value.to_string()));
        }
        wire(e, HttpMethod::Get, "/api/v1/margin/accounts", &query, None)
    }
    fn decode(r: HttpResponse) -> Result<ListMarginAccountsResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, S: SessionSigner> MonacoClient<T, MonacoAuth<S>> {
    pub async fn margin_accounts(
        &self,
        request: MarginAccountsRequest,
    ) -> Result<ListMarginAccountsResponse, ClientError<Error>> {
        self.signed_request(request).await
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PositionsRequest {
    #[serde(
        rename = "marginAccountId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_account_id: Option<String>,
    #[serde(
        rename = "tradingPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub status: Option<String>,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_size: Option<u64>,
    #[serde(rename = "pageToken", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_token: Option<String>,
}
impl Endpoint<Monaco> for PositionsRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = ListPositionsResponse;
    type Access = SessionRequired;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let mut query = Vec::new();
        if let Some(value) = &self.margin_account_id {
            uuid(value)?;
            query.push(("marginAccountId".into(), value.to_string()));
        }
        if let Some(value) = &self.trading_pair_id {
            uuid(value)?;
            query.push(("tradingPairId".into(), value.to_string()));
        }
        if let Some(value) = &self.status {
            query.push(("status".into(), value.to_string()));
        }
        if let Some(value) = &self.page_size {
            if *value < 1 {
                return Err(Error::InvalidRequest("pageSize below minimum"));
            }
            if *value > 1000 {
                return Err(Error::InvalidRequest("pageSize above maximum"));
            }
            query.push(("pageSize".into(), value.to_string()));
        }
        if let Some(value) = &self.page_token {
            query.push(("pageToken".into(), value.to_string()));
        }
        wire(e, HttpMethod::Get, "/api/v1/positions", &query, None)
    }
    fn decode(r: HttpResponse) -> Result<ListPositionsResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, S: SessionSigner> MonacoClient<T, MonacoAuth<S>> {
    pub async fn positions(
        &self,
        request: PositionsRequest,
    ) -> Result<ListPositionsResponse, ClientError<Error>> {
        self.signed_request(request).await
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OrdersRequest {
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_size: Option<u64>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub status: Option<String>,
    #[serde(
        rename = "tradingPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair_id: Option<String>,
    #[serde(
        rename = "tradingMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_mode: Option<String>,
    #[serde(
        rename = "marginAccountId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_account_id: Option<String>,
    #[serde(rename = "orderBy", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub order_by: Option<String>,
    #[serde(rename = "pageToken", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_token: Option<String>,
}
impl Endpoint<Monaco> for OrdersRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = ListOrdersResponse;
    type Access = SessionRequired;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let mut query = Vec::new();
        if let Some(value) = &self.page_size {
            if *value < 1 {
                return Err(Error::InvalidRequest("pageSize below minimum"));
            }
            if *value > 1000 {
                return Err(Error::InvalidRequest("pageSize above maximum"));
            }
            query.push(("pageSize".into(), value.to_string()));
        }
        if let Some(value) = &self.status {
            query.push(("status".into(), value.to_string()));
        }
        if let Some(value) = &self.trading_pair_id {
            uuid(value)?;
            query.push(("tradingPairId".into(), value.to_string()));
        }
        if let Some(value) = &self.trading_mode {
            one_of(value, &["SPOT", "MARGIN"])?;
            query.push(("tradingMode".into(), value.to_string()));
        }
        if let Some(value) = &self.margin_account_id {
            uuid(value)?;
            query.push(("marginAccountId".into(), value.to_string()));
        }
        if let Some(value) = &self.order_by {
            query.push(("orderBy".into(), value.to_string()));
        }
        if let Some(value) = &self.page_token {
            query.push(("pageToken".into(), value.to_string()));
        }
        wire(e, HttpMethod::Get, "/api/v1/orders", &query, None)
    }
    fn decode(r: HttpResponse) -> Result<ListOrdersResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, S: SessionSigner> MonacoClient<T, MonacoAuth<S>> {
    pub async fn orders(
        &self,
        request: OrdersRequest,
    ) -> Result<ListOrdersResponse, ClientError<Error>> {
        self.signed_request(request).await
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OrderRequest {
    #[serde(rename = "orderId")]
    pub order_id: String,
}
impl Endpoint<Monaco> for OrderRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = GetOrderResponse;
    type Access = SessionRequired;
    fn request(&self, e: &Monaco) -> Result<HttpRequest, Error> {
        let query = Vec::new();
        uuid(&self.order_id)?;
        wire(
            e,
            HttpMethod::Get,
            &format!(
                "/api/v1/orders/{}",
                aux_rs_core::url_component(&self.order_id)
            ),
            &query,
            None,
        )
    }
    fn decode(r: HttpResponse) -> Result<GetOrderResponse, Error> {
        decode(r)
    }
}
impl<T: HttpTransport, S: SessionSigner> MonacoClient<T, MonacoAuth<S>> {
    pub async fn order(
        &self,
        request: OrderRequest,
    ) -> Result<GetOrderResponse, ClientError<Error>> {
        self.signed_request(request).await
    }
}
