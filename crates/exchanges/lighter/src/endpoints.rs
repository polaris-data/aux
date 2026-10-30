use crate::LighterClient;
use crate::*;
use aux_rs_core::{
    Endpoint, Error as ClientError, Exchange, HttpMethod, HttpRequest, HttpResponse, HttpTransport,
    Public,
};
use serde::de::DeserializeOwned;

#[derive(Clone, Debug, Default)]
pub struct OrderBooksRequest {
    pub market_id: Option<i64>,
}
#[derive(Clone, Debug)]
pub struct OrderBookOrdersRequest {
    pub market_id: i64,
    pub limit: u16,
}
#[derive(Clone, Debug)]
pub struct RecentTradesRequest {
    pub market_id: i64,
    pub limit: u16,
}
#[derive(Clone, Debug)]
pub enum AccountRequest {
    Index(i64),
    L1Address(String),
}
#[derive(Clone, Debug)]
pub struct NextNonceRequest {
    pub account_index: i64,
    pub api_key_index: u8,
}
#[derive(Clone, Debug)]
pub struct ActiveOrdersRequest {
    pub account_index: i64,
    pub market_id: Option<i64>,
}
/// Access marker: this endpoint requires an authenticated client method.
pub struct Authenticated;

pub(crate) fn get(exchange: &Lighter, path: &str, query: &str) -> HttpRequest {
    HttpRequest {
        method: HttpMethod::Get,
        url: format!("{}/api/v1/{path}{query}", exchange.http_url()),
        headers: vec![],
        body: vec![],
    }
}
pub(crate) fn decode<T: DeserializeOwned>(response: HttpResponse) -> Result<T, Error> {
    let value: serde_json::Value = serde_json::from_slice(&response.body).map_err(|e| {
        if !(200..300).contains(&response.status) {
            Error::Http {
                status: response.status,
                code: None,
            }
        } else {
            Error::Decode(e)
        }
    })?;
    if let Some(code) = value
        .get("code")
        .and_then(|v| v.as_i64())
        .filter(|v| *v != 200)
    {
        let mut extra: ExtraFields = value
            .as_object()
            .map(|v| v.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default();
        let message = extra
            .remove("message")
            .and_then(|v| v.as_str().map(str::to_owned));
        extra.remove("code");
        return Err(Error::Api {
            code,
            message,
            extra,
            http_status: Some(response.status),
        });
    }
    if !(200..300).contains(&response.status) {
        return Err(Error::Http {
            status: response.status,
            code: value.get("code").and_then(|v| v.as_i64()),
        });
    }
    serde_json::from_value(value).map_err(Error::Decode)
}
macro_rules! public_endpoint {
    ($request:ty,$response:ty,$path:literal,$query:expr) => {
        impl Endpoint<Lighter> for $request {
            const SAFE_TO_RETRY: bool = true;
            type Response = $response;
            type Access = Public;
            fn request(&self, exchange: &Lighter) -> Result<HttpRequest, Error> {
                let query: String = ($query)(self)?;
                Ok(get(exchange, $path, &query))
            }
            fn decode(response: HttpResponse) -> Result<Self::Response, Error> {
                decode(response)
            }
        }
    };
}
public_endpoint!(
    OrderBooksRequest,
    OrderBooks,
    "orderBooks",
    |r: &OrderBooksRequest| -> Result<String, Error> {
        match r.market_id {
            Some(id) => {
                trading::validate_market(id)?;
                Ok(format!("?market_id={id}"))
            }
            None => Ok(String::new()),
        }
    }
);
public_endpoint!(
    OrderBookOrdersRequest,
    OrderBookOrders,
    "orderBookOrders",
    |r: &OrderBookOrdersRequest| -> Result<String, Error> {
        trading::validate_market(r.market_id)?;
        if !(1..=250).contains(&r.limit) {
            return Err(Error::InvalidRequest("book limit must be 1..250"));
        }
        Ok(format!("?market_id={}&limit={}", r.market_id, r.limit))
    }
);
public_endpoint!(
    RecentTradesRequest,
    Trades,
    "recentTrades",
    |r: &RecentTradesRequest| -> Result<String, Error> {
        trading::validate_market(r.market_id)?;
        if !(1..=100).contains(&r.limit) {
            return Err(Error::InvalidRequest("trade limit must be 1..100"));
        }
        Ok(format!("?market_id={}&limit={}", r.market_id, r.limit))
    }
);
public_endpoint!(
    AccountRequest,
    Accounts,
    "account",
    |r: &AccountRequest| -> Result<String, Error> {
        match r {
            AccountRequest::Index(index) => {
                trading::validate_identity(*index, 0)?;
                Ok(format!("?by=index&value={index}"))
            }
            AccountRequest::L1Address(address) => {
                if address.len() != 42
                    || !address.starts_with("0x")
                    || !address[2..].bytes().all(|c| c.is_ascii_hexdigit())
                {
                    return Err(Error::InvalidRequest("invalid L1 address"));
                }
                Ok(format!("?by=l1_address&value={address}"))
            }
        }
    }
);
public_endpoint!(
    NextNonceRequest,
    NextNonce,
    "nextNonce",
    |r: &NextNonceRequest| -> Result<String, Error> {
        trading::validate_identity(r.account_index, r.api_key_index)?;
        Ok(format!(
            "?account_index={}&api_key_index={}",
            r.account_index, r.api_key_index
        ))
    }
);
impl Endpoint<Lighter> for ActiveOrdersRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = Orders;
    type Access = Authenticated;
    fn request(&self, exchange: &Lighter) -> Result<HttpRequest, Error> {
        trading::validate_identity(self.account_index, 0)?;
        let mut query = format!("?account_index={}", self.account_index);
        if let Some(id) = self.market_id {
            trading::validate_market(id)?;
            query.push_str(&format!("&market_id={id}"));
        }
        Ok(get(exchange, "accountActiveOrders", &query))
    }
    fn decode(response: HttpResponse) -> Result<Orders, Error> {
        decode(response)
    }
}

impl<T: HttpTransport, A: Sync> LighterClient<T, A> {
    pub async fn order_books(
        &self,
        request: OrderBooksRequest,
    ) -> Result<OrderBooks, ClientError<Error>> {
        self.request(request).await
    }
    pub async fn order_book_orders(
        &self,
        request: OrderBookOrdersRequest,
    ) -> Result<OrderBookOrders, ClientError<Error>> {
        self.request(request).await
    }
    pub async fn recent_trades(
        &self,
        request: RecentTradesRequest,
    ) -> Result<Trades, ClientError<Error>> {
        self.request(request).await
    }
    pub async fn account(&self, request: AccountRequest) -> Result<Accounts, ClientError<Error>> {
        self.request(request).await
    }
}

impl<T: HttpTransport, A: AuthHandler> LighterClient<T, A> {
    pub async fn active_orders(
        &self,
        market_id: Option<i64>,
    ) -> Result<Orders, ClientError<Error>> {
        self.transport().ensure_open()?;
        self.auth()
            .check_exchange(self.exchange())
            .map_err(ClientError::Exchange)?;
        let request = ActiveOrdersRequest {
            account_index: self.auth().account_index(),
            market_id,
        };
        let mut wire = request
            .request(self.exchange())
            .map_err(ClientError::Exchange)?;
        let token = sign_auth_token(
            self.auth(),
            auth::unix_seconds().map_err(ClientError::Exchange)? + 600,
        )
        .await
        .map_err(ClientError::Exchange)?;
        token
            .check(self.exchange(), self.auth().account_index())
            .map_err(ClientError::Exchange)?;
        wire.headers
            .push(("authorization".into(), token.as_str().into()));
        let response = self.transport().execute_read(wire).await?;
        decode(response).map_err(ClientError::Exchange)
    }
}
