use crate::*;
use aux_rs_core::{
    Endpoint, Error as ClientError, Exchange, HttpMethod, HttpRequest, HttpResponse, HttpTransport,
    Public,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
pub(crate) fn get(e: &Arcus, path: &str, q: &[(String, String)]) -> HttpRequest {
    let query = q
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
        url: format!(
            "{}{path}{}{}",
            e.http_url(),
            if query.is_empty() { "" } else { "?" },
            query
        ),
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
    if !(200..300).contains(&r.status) || body.get("error").is_some() {
        return Err(Error::Api {
            status: Some(r.status),
            body,
        });
    }
    serde_json::from_value(body).map_err(Error::Decode)
}
#[derive(Clone, Debug, Default)]
pub struct MarketsRequest {
    pub market: Option<String>,
}
#[derive(Clone, Debug)]
pub struct OrderBookRequest {
    pub market: String,
    pub n_levels: u8,
}
impl OrderBookRequest {
    pub fn new(market: impl Into<String>) -> Self {
        Self {
            market: market.into(),
            n_levels: 20,
        }
    }
}
#[derive(Clone, Debug)]
pub struct TradesRequest {
    pub market: String,
    pub limit: u16,
}
#[derive(Clone, Debug)]
pub struct AccountRequest {
    pub address: String,
    pub account_index: u8,
}
impl AccountRequest {
    pub fn new(value: impl AsRef<str>, account_index: u8) -> Result<Self, Error> {
        if account_index > 9 {
            return Err(Error::InvalidRequest("account index must be 0 to 9"));
        }
        Ok(Self {
            address: address(value.as_ref())?,
            account_index,
        })
    }
    fn query(&self) -> Result<Vec<(String, String)>, Error> {
        let checked = Self::new(&self.address, self.account_index)?;
        Ok(vec![
            ("address".into(), checked.address),
            ("accountIndex".into(), self.account_index.to_string()),
        ])
    }
}
pub struct PositionsRequest(pub AccountRequest);
pub struct OpenOrdersRequest(pub AccountRequest);
macro_rules! endpoint {
    ($request:ty,$response:ty,$wire:expr) => {
        impl Endpoint<Arcus> for $request {
            const SAFE_TO_RETRY: bool = true;
            type Response = $response;
            type Access = Public;
            fn request(&self, e: &Arcus) -> Result<HttpRequest, Error> {
                ($wire)(self, e)
            }
            fn decode(r: HttpResponse) -> Result<$response, Error> {
                decode(r)
            }
        }
    };
}
endpoint!(MarketsRequest, Markets, |r: &MarketsRequest, e: &Arcus| {
    let q = r
        .market
        .as_ref()
        .map(|m| nonempty(m).map(|_| vec![("market".into(), m.clone())]))
        .transpose()?
        .unwrap_or_default();
    Ok(get(e, "/v1/markets", &q))
});
endpoint!(
    OrderBookRequest,
    OrderBook,
    |r: &OrderBookRequest, e: &Arcus| {
        nonempty(&r.market)?;
        if !(1..=100).contains(&r.n_levels) {
            return Err(Error::InvalidRequest("n_levels must be 1 to 100"));
        }
        Ok(get(
            e,
            &format!("/v1/l2OrderBook/{}", aux_rs_core::url_component(&r.market)),
            &[("nLevels".into(), r.n_levels.to_string())],
        ))
    }
);
endpoint!(TradesRequest, Trades, |r: &TradesRequest, e: &Arcus| {
    nonempty(&r.market)?;
    if r.limit == 0 || r.limit > 1000 {
        return Err(Error::InvalidRequest("limit must be 1 to 1000"));
    }
    Ok(get(
        e,
        "/v1/trades",
        &[
            ("market".into(), r.market.clone()),
            ("limit".into(), r.limit.to_string()),
        ],
    ))
});
endpoint!(AccountRequest, Account, |r: &AccountRequest, e: &Arcus| Ok(
    get(e, "/v1/account", &r.query()?)
));
endpoint!(
    PositionsRequest,
    Positions,
    |r: &PositionsRequest, e: &Arcus| Ok(get(e, "/v1/positions", &r.0.query()?))
);
endpoint!(
    OpenOrdersRequest,
    OpenOrders,
    |r: &OpenOrdersRequest, e: &Arcus| Ok(get(e, "/v1/openOrders", &r.0.query()?))
);
impl<T: HttpTransport, A: Sync> ArcusClient<T, A> {
    pub async fn markets(&self, r: MarketsRequest) -> Result<Markets, ClientError<Error>> {
        self.request(r).await
    }
    pub async fn order_book(&self, r: OrderBookRequest) -> Result<OrderBook, ClientError<Error>> {
        self.request(r).await
    }
    pub async fn trades(&self, r: TradesRequest) -> Result<Trades, ClientError<Error>> {
        self.request(r).await
    }
    pub async fn account(&self, r: AccountRequest) -> Result<Account, ClientError<Error>> {
        self.request(r).await
    }
    pub async fn positions(&self, r: AccountRequest) -> Result<Positions, ClientError<Error>> {
        self.request(PositionsRequest(r)).await
    }
    pub async fn open_orders(&self, r: AccountRequest) -> Result<OpenOrders, ClientError<Error>> {
        self.request(OpenOrdersRequest(r)).await
    }
}
