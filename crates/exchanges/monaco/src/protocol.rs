use crate::Error;
use aux_rs_core::{Exchange, HttpMethod, HttpRequest, HttpResponse};
use serde::de::DeserializeOwned;
use serde_json::Value;
/// Endpoint and chain context. Custom endpoints accept an origin, /api or /api/v1 base.
#[derive(Clone, PartialEq, Eq)]
pub struct Monaco {
    http: String,
    ws: String,
    chain_id: u64,
}
impl std::fmt::Debug for Monaco {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monaco")
            .field("chain_id", &self.chain_id)
            .finish_non_exhaustive()
    }
}
impl Monaco {
    pub fn mainnet() -> Self {
        Self::custom("https://apimonaco.xyz", "wss://ws.apimonaco.xyz/ws", 1329)
            .expect("static endpoints")
    }
    pub fn testnet() -> Self {
        Self::custom(
            "https://staging.apimonaco.xyz",
            "wss://ws-staging.apimonaco.xyz/ws",
            1328,
        )
        .expect("static endpoints")
    }
    pub fn custom(
        http: impl AsRef<str>,
        ws: impl AsRef<str>,
        chain_id: u64,
    ) -> Result<Self, Error> {
        let mut http = url::Url::parse(http.as_ref())
            .map_err(|_| Error::InvalidRequest("invalid HTTP URL"))?;
        let ws = url::Url::parse(ws.as_ref())
            .map_err(|_| Error::InvalidRequest("invalid WebSocket URL"))?;
        if !matches!(http.scheme(), "http" | "https")
            || !matches!(ws.scheme(), "ws" | "wss")
            || chain_id == 0
            || [&http, &ws].iter().any(|u| {
                u.host_str().is_none()
                    || !u.username().is_empty()
                    || u.password().is_some()
                    || u.query().is_some()
                    || u.fragment().is_some()
            })
        {
            return Err(Error::InvalidRequest(
                "endpoints require supported schemes, hosts and no credentials/query/fragment; chain ID must be positive",
            ));
        }
        let path = http.path().trim_end_matches('/');
        let path = path
            .strip_suffix("/api/v1")
            .or_else(|| path.strip_suffix("/api"))
            .unwrap_or(path)
            .to_owned();
        http.set_path(&path);
        Ok(Self {
            http: http.as_str().trim_end_matches('/').into(),
            ws: ws.into(),
            chain_id,
        })
    }
    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }
}
impl Exchange for Monaco {
    type Error = Error;
    fn http_url(&self) -> &str {
        &self.http
    }
    fn ws_url(&self) -> &str {
        &self.ws
    }
}
pub(crate) fn uuid(s: &str) -> Result<(), Error> {
    if s.len() != 36
        || !s.bytes().enumerate().all(|(i, b)| {
            if [8, 13, 18, 23].contains(&i) {
                b == b'-'
            } else {
                b.is_ascii_hexdigit()
            }
        })
    {
        return Err(Error::InvalidRequest("expected a UUID"));
    }
    Ok(())
}
pub(crate) fn one_of(s: &str, choices: &[&str]) -> Result<(), Error> {
    if choices.contains(&s) {
        Ok(())
    } else {
        Err(Error::InvalidRequest("unsupported native field value"))
    }
}
pub(crate) fn nonempty(s: &str) -> Result<(), Error> {
    if s.trim().is_empty() {
        Err(Error::InvalidRequest("value cannot be empty"))
    } else {
        Ok(())
    }
}
pub(crate) fn wire(
    e: &Monaco,
    method: HttpMethod,
    path: &str,
    query: &[(String, String)],
    body: Option<&Value>,
) -> Result<HttpRequest, Error> {
    let query = query
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
    let url = format!(
        "{}{path}{}{}",
        e.http_url(),
        if query.is_empty() { "" } else { "?" },
        query
    );
    let url = url::Url::parse(&url)
        .map_err(|_| Error::InvalidRequest("invalid request URL"))?
        .to_string();
    Ok(HttpRequest {
        method,
        url,
        headers: vec![
            (
                "User-Agent".into(),
                concat!("aux-rs-monaco/", env!("CARGO_PKG_VERSION")).into(),
            ),
            ("Content-Type".into(), "application/json".into()),
        ],
        body: body
            .map(serde_json::to_vec)
            .transpose()?
            .unwrap_or_default(),
    })
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
    if !(200..300).contains(&r.status) || body.get("error").is_some_and(|e| !e.is_null()) {
        return Err(Error::Api {
            status: Some(r.status),
            body,
        });
    }
    serde_json::from_value(body).map_err(Error::Decode)
}
