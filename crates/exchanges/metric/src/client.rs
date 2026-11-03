use crate::*;
use aux_rs_core::{
    DefaultTransport, Error as ClientError, HttpMethod, HttpRequest, HttpTransport, Unauthenticated,
};
use serde::de::DeserializeOwned;
use zeroize::Zeroizing;

#[derive(Clone, Debug)]
pub struct Metric {
    pub chain_id: u64,
    pub api_url: String,
}
impl Metric {
    pub fn new(chain_id: u64) -> Result<Self, Error> {
        Self::custom(chain_id, "https://api.metric.xyz")
    }
    pub fn custom(chain_id: u64, api_url: impl Into<String>) -> Result<Self, Error> {
        let api_url = api_url.into();
        validate_url(&api_url)?;
        if chain_id == 0 {
            return Err(Error::InvalidRequest("chain_id must be positive"));
        }
        Ok(Self {
            chain_id,
            api_url: api_url.trim_end_matches('/').into(),
        })
    }
}
pub(crate) fn validate_url(s: &str) -> Result<(), Error> {
    let url = url::Url::parse(s).map_err(|_| Error::InvalidRequest("invalid endpoint URL"))?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(Error::InvalidRequest(
            "endpoint must be an HTTP origin without credentials, query or fragment",
        ));
    }
    Ok(())
}
#[derive(Clone)]
pub struct TradingKey(Zeroizing<String>);
impl std::fmt::Debug for TradingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TradingKey([redacted])")
    }
}
impl TradingKey {
    pub fn new(key: Zeroizing<String>) -> Result<Self, Error> {
        if key.is_empty() || key.bytes().any(|b| !(33..=126).contains(&b)) {
            return Err(Error::Authentication("invalid trading API key"));
        }
        Ok(Self(key))
    }
}
pub trait ApiAuthentication: Send + Sync {
    fn authorization(&self) -> Option<String>;
}
impl ApiAuthentication for Unauthenticated {
    fn authorization(&self) -> Option<String> {
        None
    }
}
impl ApiAuthentication for TradingKey {
    fn authorization(&self) -> Option<String> {
        Some(format!("Bearer {}", self.0.as_str()))
    }
}

/// Exchange-native HTTP client; propAMM submission requires a TradingKey.
/// ```compile_fail
/// use aux_rs_core::HttpTransport;
/// use aux_rs_metric::{MetricClient,PammQuoteRequest};
/// fn unsigned<T:HttpTransport>(client:MetricClient<T>,request:PammQuoteRequest){let _=client.pamm_quote(request);}
/// ```
#[derive(Clone)]
pub struct MetricClient<T = DefaultTransport, A = Unauthenticated> {
    exchange: Metric,
    transport: T,
    auth: A,
}
impl<T, A> std::fmt::Debug for MetricClient<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetricClient").finish_non_exhaustive()
    }
}
impl<T> MetricClient<T> {
    pub fn with_transport(exchange: Metric, transport: T) -> Self {
        Self {
            exchange,
            transport,
            auth: Unauthenticated,
        }
    }
}
impl<T, A> MetricClient<T, A> {
    pub fn exchange(&self) -> &Metric {
        &self.exchange
    }
    pub fn transport(&self) -> &T {
        &self.transport
    }
    pub fn with_auth(self, auth: TradingKey) -> MetricClient<T, TradingKey> {
        MetricClient {
            exchange: self.exchange,
            transport: self.transport,
            auth,
        }
    }
}
#[cfg(feature = "http")]
impl MetricClient<aux_rs_core::StandardTransport> {
    pub fn new(exchange: Metric) -> Result<Self, aux_rs_core::TransportError> {
        Self::with_config(exchange, Default::default())
    }
    pub fn with_config(
        exchange: Metric,
        config: aux_rs_core::TransportConfig,
    ) -> Result<Self, aux_rs_core::TransportError> {
        Ok(Self::with_transport(
            exchange,
            aux_rs_core::StandardTransport::new(config)?,
        ))
    }
}
impl<T: HttpTransport, A: ApiAuthentication> MetricClient<T, A> {
    async fn execute<R: DeserializeOwned>(
        &self,
        method: HttpMethod,
        path: &str,
        query: Option<serde_json::Value>,
        body: Option<serde_json::Value>,
        safe: bool,
    ) -> Result<R, ClientError<Error>> {
        self.transport.ensure_open()?;
        let mut url = url::Url::parse(&format!("{}{path}", self.exchange.api_url))
            .map_err(|_| ClientError::Exchange(Error::InvalidRequest("invalid endpoint")))?;
        if let Some(serde_json::Value::Object(params)) = query {
            let mut pairs = url.query_pairs_mut();
            for (k, v) in params {
                if !v.is_null() {
                    pairs.append_pair(
                        &k,
                        &match v {
                            serde_json::Value::String(s) => s,
                            _ => v.to_string(),
                        },
                    );
                }
            }
        }
        let mut headers = vec![("content-type".into(), "application/json".into())];
        if let Some(auth) = self.auth.authorization() {
            headers.push(("authorization".into(), auth));
        }
        let request = HttpRequest {
            method,
            url: url.to_string(),
            headers,
            body: body
                .map(|v| serde_json::to_vec(&v))
                .transpose()
                .map_err(|e| ClientError::Exchange(e.into()))?
                .unwrap_or_default(),
        };
        let response = if safe {
            self.transport.execute_read(request).await?
        } else {
            self.transport.execute(request).await?
        };
        decode_response(response).map_err(ClientError::Exchange)
    }
    pub async fn chains(&self) -> Result<ChainsResponse, ClientError<Error>> {
        self.execute(HttpMethod::Get, "/public/v1/chains", None, None, true)
            .await
    }
    pub async fn factory_settings(&self) -> Result<FactorySettingsResponse, ClientError<Error>> {
        self.execute(
            HttpMethod::Get,
            "/public/v1/evm/factory-settings",
            None,
            None,
            true,
        )
        .await
    }
    pub async fn metadata(
        &self,
        request: MetadataRequest,
    ) -> Result<MetadataResponse, ClientError<Error>> {
        self.execute(
            HttpMethod::Get,
            &format!("/public/v1/evm/{}/metadata", self.exchange.chain_id),
            Some(serde_json::to_value(request).map_err(|e| ClientError::Exchange(e.into()))?),
            None,
            true,
        )
        .await
    }
    pub async fn cross_chain_metadata(
        &self,
        request: MetadataRequest,
    ) -> Result<MetadataResponse, ClientError<Error>> {
        self.execute(
            HttpMethod::Get,
            "/public/v1/evm/metadata",
            Some(serde_json::to_value(request).map_err(|e| ClientError::Exchange(e.into()))?),
            None,
            true,
        )
        .await
    }
    pub async fn bid_ask(
        &self,
        pool: &str,
        depth: Option<bool>,
    ) -> Result<BidAskResponse, ClientError<Error>> {
        address(pool).map_err(ClientError::Exchange)?;
        self.execute(
            HttpMethod::Get,
            &format!("/public/v1/evm/{}/{pool}/bid_ask", self.exchange.chain_id),
            Some(serde_json::json!({"depth":depth})),
            None,
            true,
        )
        .await
    }
    pub async fn batch_bid_ask(
        &self,
        request: BatchBidAskRequest,
    ) -> Result<BatchBidAskResponse, ClientError<Error>> {
        if request.pools.is_empty() || request.pools.len() > 100 {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "batch requires 1 to 100 pools",
            )));
        }
        for p in &request.pools {
            address(p).map_err(ClientError::Exchange)?;
        }
        self.execute(
            HttpMethod::Post,
            &format!("/public/v1/evm/{}/bid_ask/batch", self.exchange.chain_id),
            None,
            Some(serde_json::to_value(request).map_err(|e| ClientError::Exchange(e.into()))?),
            true,
        )
        .await
    }
    pub async fn route(&self, request: RouteRequest) -> Result<RouteResponse, ClientError<Error>> {
        let a = address(&request.token_in).map_err(ClientError::Exchange)?;
        let b = address(&request.token_out).map_err(ClientError::Exchange)?;
        if a == b || a.is_zero() || b.is_zero() {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "route requires distinct nonzero tokens",
            )));
        }
        let trade = request
            .trade_type
            .or(match (&request.amount_in, &request.amount_out) {
                (Some(_), None) => Some(TradeType::ExactIn),
                (None, Some(_)) => Some(TradeType::ExactOut),
                _ => None,
            })
            .ok_or(ClientError::Exchange(Error::InvalidRequest(
                "ambiguous trade type",
            )))?;
        let amount = match trade {
            TradeType::ExactIn => &request.amount_in,
            TradeType::ExactOut => &request.amount_out,
        };
        if !amount.as_ref().is_some_and(|s| {
            !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) && s.bytes().any(|b| b != b'0')
        }) {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "positive raw integer amount required",
            )));
        }
        if let Some(pool) = &request.pool_address {
            address(pool).map_err(ClientError::Exchange)?;
        }
        self.execute(
            HttpMethod::Post,
            &format!("/public/v1/evm/{}/route", self.exchange.chain_id),
            None,
            Some(serde_json::to_value(request).map_err(|e| ClientError::Exchange(e.into()))?),
            true,
        )
        .await
    }
}
impl<T: HttpTransport> MetricClient<T, TradingKey> {
    pub async fn pamm_quote(
        &self,
        request: PammQuoteRequest,
    ) -> Result<PammQuoteResponse, ClientError<Error>> {
        validate_pamm(self.exchange.chain_id, &request).map_err(ClientError::Exchange)?;
        self.execute(
            HttpMethod::Post,
            &format!("/public/v1/evm/{}/pamm-quote", self.exchange.chain_id),
            None,
            Some(serde_json::to_value(request).map_err(|e| ClientError::Exchange(e.into()))?),
            false,
        )
        .await
    }
}
pub(crate) fn address(s: &str) -> Result<alloy_primitives::Address, Error> {
    s.parse()
        .map_err(|_| Error::InvalidRequest("invalid EVM address"))
}
pub(crate) fn decode_response<R: DeserializeOwned>(
    response: aux_rs_core::HttpResponse,
) -> Result<R, Error> {
    if !(200..300).contains(&response.status) {
        let body = serde_json::from_slice(&response.body).unwrap_or_else(|_| {
            serde_json::Value::String(String::from_utf8_lossy(&response.body).into_owned())
        });
        return Err(Error::Api {
            status: response.status,
            body,
        });
    }
    Ok(serde_json::from_slice(&response.body)?)
}
fn validate_pamm(chain: u64, r: &PammQuoteRequest) -> Result<(), Error> {
    if !matches!(chain, 1 | 56)
        || r.replacement_seq_number == 0
        || r.response_timeout_ms.is_some_and(|x| x > 500)
    {
        return Err(Error::InvalidRequest(
            "invalid propAMM chain, sequence or timeout",
        ));
    }
    let tx = alloy_primitives::hex::decode(&r.tx)
        .map_err(|_| Error::InvalidRequest("invalid signed transaction hex"))?;
    if !tx.is_empty() {
        crate::chain::validate_signed_transaction(&tx, chain)?;
    }
    if alloy_primitives::hex::decode(&r.replacement_uuid)
        .map_err(|_| Error::InvalidRequest("invalid replacement UUID"))?
        .len()
        != 16
    {
        return Err(Error::InvalidRequest("replacement UUID must be 16 bytes"));
    }
    if chain == 56 && !tx.is_empty() && r.block_number == 0 {
        return Err(Error::InvalidRequest("BSC live quotes require blockNumber"));
    }
    if let Some(ts) = r.max_timestamp {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| Error::InvalidRequest("invalid clock"))?
            .as_secs();
        if ts == 0
            || ts >= 1_000_000_000_000
            || chain == 56 && !tx.is_empty() && ts > now.saturating_add(5)
        {
            return Err(Error::InvalidRequest(
                "maxTimestamp must be seconds and no more than five seconds ahead",
            ));
        }
    }
    for p in r.pool_addresses.iter().flatten() {
        address(p)?;
    }
    if let Some(p) = &r.quote_address {
        address(p)?;
    }
    for pair in r.asset_pairs.iter().flatten() {
        if alloy_primitives::hex::decode(pair)
            .map_err(|_| Error::InvalidRequest("invalid asset pair"))?
            .len()
            != 40
        {
            return Err(Error::InvalidRequest(
                "asset pair must contain two addresses",
            ));
        }
    }
    Ok(())
}
