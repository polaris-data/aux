use crate::HyperliquidClient;
use crate::{Address, AuthHandler, Cloid, Error, ExtraFields};
use aux_rs_core::{Error as ClientError, Exchange, HttpMethod, HttpRequest, HttpTransport};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tif {
    Alo,
    Ioc,
    Gtc,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tpsl {
    Tp,
    Sl,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    Limit {
        tif: Tif,
    },
    Trigger {
        #[serde(rename = "isMarket")]
        is_market: bool,
        #[serde(rename = "triggerPx")]
        trigger_px: String,
        tpsl: Tpsl,
    },
}

/// Declaration order matches the official SDK's signed MessagePack order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
}

impl Order {
    pub fn limit(
        asset: u32,
        is_buy: bool,
        price: impl Into<String>,
        size: impl Into<String>,
        tif: Tif,
    ) -> Self {
        Self {
            asset,
            is_buy,
            limit_px: price.into(),
            size: size.into(),
            reduce_only: false,
            order_type: OrderType::Limit { tif },
            cloid: None,
        }
    }

    pub fn validate(&self) -> Result<(), Error> {
        validate_decimal(&self.limit_px)?;
        validate_decimal(&self.size)?;
        if let OrderType::Trigger { trigger_px, .. } = &self.order_type {
            validate_decimal(trigger_px)?;
        }
        Ok(())
    }

    /// Optional metadata-aware validation, without fetching or caching markets.
    /// Perp prices allow 6 - szDecimals decimal places; spot prices allow 8 -
    /// szDecimals. Noninteger prices have at most five significant figures.
    pub fn validate_precision(&self, sz_decimals: u32, is_spot: bool) -> Result<(), Error> {
        self.validate()?;
        let max_decimals: u32 = if is_spot { 8 } else { 6 };
        let price_decimals = max_decimals
            .checked_sub(sz_decimals)
            .ok_or(Error::InvalidRequest("invalid market size precision"))?;
        if fractional_digits(&self.size) > sz_decimals as usize {
            return Err(Error::InvalidRequest("size exceeds market precision"));
        }
        for price in std::iter::once(&self.limit_px).chain(match &self.order_type {
            OrderType::Trigger { trigger_px, .. } => Some(trigger_px),
            _ => None,
        }) {
            if price.contains('.')
                && (fractional_digits(price) > price_decimals as usize
                    || price
                        .chars()
                        .filter(|c| *c != '.')
                        .collect::<String>()
                        .trim_start_matches('0')
                        .len()
                        > 5)
            {
                return Err(Error::InvalidRequest("price exceeds market precision"));
            }
        }
        Ok(())
    }
}

fn fractional_digits(value: &str) -> usize {
    value
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len())
}

/// Reject noncanonical strings rather than silently rounding or normalizing.
pub(crate) fn validate_decimal(value: &str) -> Result<(), Error> {
    let (integer, fraction) = value
        .split_once('.')
        .map_or((value, None), |(i, f)| (i, Some(f)));
    if integer.is_empty()
        || !integer.bytes().all(|b| b.is_ascii_digit())
        || (integer.len() > 1 && integer.starts_with('0'))
        || fraction.is_some_and(|f| {
            f.is_empty() || !f.bytes().all(|b| b.is_ascii_digit()) || f.ends_with('0')
        })
        || !value.bytes().any(|b| b.is_ascii_digit() && b != b'0')
    {
        return Err(Error::InvalidRequest(
            "amounts must be positive canonical decimals without exponents, leading zeros, or trailing fractional zeros",
        ));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NamedGrouping {
    #[default]
    Na,
    NormalTpsl,
    PositionTpsl,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Grouping {
    Named(NamedGrouping),
    Priority { p: u32 },
}
impl Default for Grouping {
    fn default() -> Self {
        Self::Named(NamedGrouping::Na)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubBuilder {
    pub b: Address,
    pub s: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuilderFee {
    pub b: Address,
    pub f: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<Vec<SubBuilder>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderRequest {
    pub orders: Vec<Order>,
    #[serde(default)]
    pub grouping: Grouping,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder: Option<BuilderFee>,
}
impl OrderRequest {
    pub fn new(orders: Vec<Order>) -> Self {
        Self {
            orders,
            grouping: Grouping::default(),
            builder: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cancel {
    #[serde(rename = "a")]
    pub asset: u32,
    #[serde(rename = "o")]
    pub oid: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelRequest {
    pub cancels: Vec<Cancel>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelByCloid {
    pub asset: u32,
    pub cloid: Cloid,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelByCloidRequest {
    pub cancels: Vec<CancelByCloid>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Action {
    Order(OrderRequest),
    Cancel(CancelRequest),
    CancelByCloid(CancelByCloidRequest),
}
impl Action {
    pub fn validate(&self) -> Result<(), Error> {
        match self {
            Self::Order(request) => {
                if request.orders.is_empty() {
                    return Err(Error::InvalidRequest("order batch cannot be empty"));
                }
                for order in &request.orders {
                    order.validate()?;
                }
                if let Some(shares) = request.builder.as_ref().and_then(|b| b.s.as_ref())
                    && (shares.len() > 5
                        || shares.iter().any(|s| s.s == 0)
                        || shares.iter().map(|s| s.s as u64).sum::<u64>() > 100_000)
                {
                    return Err(Error::InvalidRequest(
                        "at most five positive sub-builder shares may sum to 100000",
                    ));
                }
            }
            Self::Cancel(request) if request.cancels.is_empty() => {
                return Err(Error::InvalidRequest("cancel batch cannot be empty"));
            }
            Self::CancelByCloid(request) if request.cancels.is_empty() => {
                return Err(Error::InvalidRequest("cancel batch cannot be empty"));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ExecutionOptions {
    /// Bypasses automatic nonce allocation. The caller owns collision avoidance.
    pub nonce: Option<u64>,
    pub expires_after: Option<u64>,
}

/// An HTTP success can still contain a rejected request or rejected batch items.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", content = "response", rename_all = "lowercase")]
pub enum ExchangeResponse<T> {
    Ok(T),
    Err(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchResponse {
    #[serde(rename = "type")]
    pub response_type: String,
    pub data: BatchData,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchData {
    pub statuses: Vec<ItemStatus>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemStatus {
    Resting { resting: RestingOrder },
    Filled { filled: FilledOrder },
    Error { error: String },
    Text(String),
    Unknown(Value),
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestingOrder {
    pub oid: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloid: Option<Cloid>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilledOrder {
    pub total_sz: String,
    pub avg_px: String,
    pub oid: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloid: Option<Cloid>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

pub type TradingResult = Result<ExchangeResponse<BatchResponse>, ClientError<Error>>;

async fn submit<T: HttpTransport, A: AuthHandler>(
    client: &HyperliquidClient<T, A>,
    action: Action,
    options: ExecutionOptions,
) -> TradingResult {
    client.transport().ensure_open()?;
    let network = client.exchange().network().ok_or({
        ClientError::Exchange(Error::InvalidRequest(
            "custom endpoints require explicit signing network",
        ))
    })?;
    let signed = client
        .auth()
        .prepare(action, network, options)
        .await
        .map_err(ClientError::Exchange)?;
    if signed.network() != network {
        return Err(ClientError::Exchange(Error::InvalidRequest(
            "auth handler returned a request for the wrong network",
        )));
    }
    if signed.signer_address() != client.auth().signer_address() {
        return Err(ClientError::Exchange(Error::Signing(
            crate::SignerError::AddressMismatch,
        )));
    }
    let request = HttpRequest {
        method: HttpMethod::Post,
        url: format!("{}/exchange", client.exchange().http_url()),
        headers: vec![("content-type".into(), "application/json".into())],
        body: serde_json::to_vec(&signed).map_err(|e| ClientError::Exchange(Error::Encode(e)))?,
    };
    client.transport().ensure_open()?;
    let response = client.transport().execute(request).await?;
    if !(200..300).contains(&response.status) {
        return Err(ClientError::Exchange(Error::Http {
            status: response.status,
            body: String::from_utf8_lossy(&response.body).into_owned(),
        }));
    }
    serde_json::from_slice(&response.body).map_err(|e| ClientError::Exchange(Error::Decode(e)))
}

/// Trading methods require a client with native authentication.
///
/// ```compile_fail
/// use aux_rs_core::HttpTransport;
/// use aux_rs_hyperliquid::{HyperliquidClient, OrderRequest};
/// fn unsigned_cannot_trade<T: HttpTransport>(client: HyperliquidClient<T>) {
///     let _ = client.order(OrderRequest::new(vec![]));
/// }
/// ```
impl<T: HttpTransport, A: AuthHandler> HyperliquidClient<T, A> {
    pub async fn order(&self, request: OrderRequest) -> TradingResult {
        self.order_with_options(request, ExecutionOptions::default())
            .await
    }
    pub async fn order_with_options(
        &self,
        request: OrderRequest,
        options: ExecutionOptions,
    ) -> TradingResult {
        submit(self, Action::Order(request), options).await
    }
    pub async fn cancel(&self, request: CancelRequest) -> TradingResult {
        self.cancel_with_options(request, ExecutionOptions::default())
            .await
    }
    pub async fn cancel_with_options(
        &self,
        request: CancelRequest,
        options: ExecutionOptions,
    ) -> TradingResult {
        submit(self, Action::Cancel(request), options).await
    }
    pub async fn cancel_by_cloid(&self, request: CancelByCloidRequest) -> TradingResult {
        self.cancel_by_cloid_with_options(request, ExecutionOptions::default())
            .await
    }
    pub async fn cancel_by_cloid_with_options(
        &self,
        request: CancelByCloidRequest,
        options: ExecutionOptions,
    ) -> TradingResult {
        submit(self, Action::CancelByCloid(request), options).await
    }
}
