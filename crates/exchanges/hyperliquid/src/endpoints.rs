use crate::HyperliquidClient;

use aux_rs_core::{
    Endpoint, Error as ClientError, Exchange, HttpMethod, HttpRequest, HttpResponse, HttpTransport,
    Public,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{Error, Hyperliquid, L2Book, Meta};

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex: Option<String>,
}

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L2BookRequest {
    pub coin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_sig_figs: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mantissa: Option<u8>,
}

impl L2BookRequest {
    pub fn new(coin: impl Into<String>) -> Self {
        Self {
            coin: coin.into(),
            n_sig_figs: None,
            mantissa: None,
        }
    }

    pub fn validate(&self) -> Result<(), Error> {
        validate_coin(&self.coin)?;
        if self.n_sig_figs.is_some_and(|n| !(2..=5).contains(&n)) {
            return Err(Error::InvalidRequest("nSigFigs must be 2, 3, 4, or 5"));
        }
        if self.mantissa.is_some_and(|n| ![1, 2, 5].contains(&n))
            || (self.mantissa.is_some() && self.n_sig_figs != Some(5))
        {
            return Err(Error::InvalidRequest(
                "mantissa must be 1, 2, or 5 and requires nSigFigs = 5",
            ));
        }
        Ok(())
    }
}

pub(crate) fn validate_coin(coin: &str) -> Result<(), Error> {
    if coin.trim().is_empty() {
        return Err(Error::InvalidRequest("coin cannot be empty"));
    }
    Ok(())
}

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Serialize)]
#[serde(tag = "type")]
enum InfoRequest<'a> {
    #[serde(rename = "meta")]
    Meta(&'a MetaRequest),
    #[serde(rename = "l2Book")]
    L2Book(&'a L2BookRequest),
}

fn encode(exchange: &Hyperliquid, request: InfoRequest<'_>) -> Result<HttpRequest, Error> {
    Ok(HttpRequest {
        method: HttpMethod::Post,
        url: format!("{}/info", exchange.http_url()),
        headers: vec![("content-type".into(), "application/json".into())],
        body: serde_json::to_vec(&request).map_err(Error::Encode)?,
    })
}

pub(crate) fn decode<T: DeserializeOwned>(response: HttpResponse) -> Result<T, Error> {
    if !(200..300).contains(&response.status) {
        return Err(Error::Http {
            status: response.status,
            body: String::from_utf8_lossy(&response.body).into_owned(),
        });
    }
    let value: serde_json::Value = serde_json::from_slice(&response.body).map_err(Error::Decode)?;
    if let Some(message) = value.as_str() {
        return Err(Error::Api {
            message: message.to_owned(),
        });
    }
    if value.get("status").and_then(|v| v.as_str()) == Some("err") {
        return Err(Error::Api {
            message: value["response"]
                .as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| value["response"].to_string()),
        });
    }
    serde_json::from_value(value).map_err(Error::Decode)
}

impl Endpoint<Hyperliquid> for MetaRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = Meta;
    type Access = Public;
    fn request(&self, exchange: &Hyperliquid) -> Result<HttpRequest, Error> {
        encode(exchange, InfoRequest::Meta(self))
    }
    fn decode(response: HttpResponse) -> Result<Meta, Error> {
        decode(response)
    }
}

impl Endpoint<Hyperliquid> for L2BookRequest {
    const SAFE_TO_RETRY: bool = true;
    type Response = L2Book;
    type Access = Public;
    fn request(&self, exchange: &Hyperliquid) -> Result<HttpRequest, Error> {
        self.validate()?;
        encode(exchange, InfoRequest::L2Book(self))
    }
    fn decode(response: HttpResponse) -> Result<L2Book, Error> {
        decode(response)
    }
}

impl<T: HttpTransport, A: Sync> HyperliquidClient<T, A> {
    pub async fn meta(&self) -> Result<Meta, ClientError<Error>> {
        self.request(MetaRequest::default()).await
    }
    pub async fn meta_with(&self, request: MetaRequest) -> Result<Meta, ClientError<Error>> {
        self.request(request).await
    }
    pub async fn l2_book(&self, request: L2BookRequest) -> Result<L2Book, ClientError<Error>> {
        self.request(request).await
    }
}
