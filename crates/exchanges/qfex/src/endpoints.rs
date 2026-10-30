use crate::*;
use aux_rs_core::{
    Endpoint, Error as ClientError, Exchange, HttpMethod, HttpRequest, HttpResponse, HttpTransport,
    Public,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
pub const API_VERSION: &str = "2026-10-29";
pub(crate) fn get(exchange: &Qfex, path: &str, query: &[(String, String)]) -> HttpRequest {
    let mut pairs = vec![("api_version".to_string(), API_VERSION.to_string())];
    pairs.extend_from_slice(query);
    let query = pairs
        .iter()
        .map(|(k, v)| {
            format!(
                "{}={}",
                aux_rs_core::url_component(k),
                aux_rs_core::url_component(v)
            )
        })
        .collect::<Vec<_>>()
        .join("&");
    HttpRequest {
        method: HttpMethod::Get,
        url: format!("{}{path}?{query}", exchange.http_url()),
        headers: vec![],
        body: vec![],
    }
}
pub(crate) fn decode<T: DeserializeOwned>(r: HttpResponse) -> Result<T, Error> {
    let body: Value = match serde_json::from_slice(&r.body) {
        Ok(body) => body,
        Err(e) if (200..300).contains(&r.status) => return Err(Error::Decode(e)),
        Err(_) => {
            return Err(Error::Api {
                status: Some(r.status),
                body: Value::String(String::from_utf8_lossy(&r.body).into_owned()),
            });
        }
    };
    if !(200..300).contains(&r.status)
        || body.get("error").is_some()
        || body.get("error_code").is_some()
    {
        return Err(Error::Api {
            status: Some(r.status),
            body,
        });
    }
    serde_json::from_value(body).map_err(Error::Decode)
}
#[derive(Clone, Debug, Default)]
pub struct RefdataRequest {
    pub ticker: Option<String>,
}
#[derive(Clone, Debug, Default)]
pub struct ContractsRequest {
    pub symbol: Option<String>,
}
#[derive(Clone, Debug)]
pub struct OrderBookRequest {
    pub symbol: String,
}
impl OrderBookRequest {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
        }
    }
}
macro_rules! endpoint {
    ($request:ty,$response:ty,$wire:expr) => {
        impl Endpoint<Qfex> for $request {
            const SAFE_TO_RETRY: bool = true;
            type Response = $response;
            type Access = Public;
            fn request(&self, exchange: &Qfex) -> Result<HttpRequest, Error> {
                ($wire)(self, exchange)
            }
            fn decode(response: HttpResponse) -> Result<$response, Error> {
                decode(response)
            }
        }
    };
}
endpoint!(RefdataRequest, Refdata, |r: &RefdataRequest, e: &Qfex| {
    let q = r
        .ticker
        .as_ref()
        .map(|s| nonempty(s).map(|_| vec![("ticker".into(), s.clone())]))
        .transpose()?
        .unwrap_or_default();
    Ok(get(e, "/refdata", &q))
});
endpoint!(
    ContractsRequest,
    Contracts,
    |r: &ContractsRequest, e: &Qfex| {
        let q = r
            .symbol
            .as_ref()
            .map(|s| nonempty(s).map(|_| vec![("symbol".into(), s.clone())]))
            .transpose()?
            .unwrap_or_default();
        Ok(get(e, "/md/contracts", &q))
    }
);
endpoint!(
    OrderBookRequest,
    OrderBook,
    |r: &OrderBookRequest, e: &Qfex| {
        nonempty(&r.symbol)?;
        Ok(get(
            e,
            &format!("/md/orderbook/{}", aux_rs_core::url_component(&r.symbol)),
            &[],
        ))
    }
);
impl<T: HttpTransport, A: Sync> QfexClient<T, A> {
    pub async fn refdata(&self, r: RefdataRequest) -> Result<Refdata, ClientError<Error>> {
        self.request(r).await
    }
    pub async fn contracts(&self, r: ContractsRequest) -> Result<Contracts, ClientError<Error>> {
        self.request(r).await
    }
    pub async fn order_book(&self, r: OrderBookRequest) -> Result<OrderBook, ClientError<Error>> {
        self.request(r).await
    }
}
impl<T: HttpTransport, A: AuthHandler> QfexClient<T, A> {
    pub async fn positions(&self) -> Result<PositionsBalance, ClientError<Error>> {
        self.transport.ensure_open()?;
        let mut r = get(&self.exchange, "/user/positions", &[]);
        r.headers = self.auth.headers().map_err(ClientError::Exchange)?;
        // HMAC nonces are unique per request, so authenticated reads execute once.
        decode(self.transport.execute(r).await?).map_err(ClientError::Exchange)
    }
    pub async fn account_balances(&self) -> Result<Value, ClientError<Error>> {
        self.transport.ensure_open()?;
        let mut r = get(&self.exchange, "/user/subaccounts/balance", &[]);
        r.headers = self.auth.headers().map_err(ClientError::Exchange)?;
        decode(self.transport.execute(r).await?).map_err(ClientError::Exchange)
    }
}
