use crate::{DecodedBody, Error, MAX_RECORD_BYTES, Params, Result};
use aux_rs_core::{HttpMethod, HttpRequest, StreamingHttpTransport};
use serde_json::Value;
use std::sync::Arc;
use url::Url;
use zeroize::Zeroizing;

#[derive(Clone)]
pub struct HttpClient<T> {
    pub transport: T,
    base: Url,
    auth: Option<Arc<Zeroizing<String>>>,
    auth_header: &'static str,
    secrets: Vec<Arc<Zeroizing<String>>>,
}
impl<T> std::fmt::Debug for HttpClient<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataHttpClient").finish_non_exhaustive()
    }
}
impl<T> HttpClient<T> {
    pub fn new(base: &str, transport: T) -> Result<Self> {
        let base = Url::parse(&format!("{}/", base.trim_end_matches('/')))
            .map_err(|_| Error::InvalidRequest("invalid data API URL"))?;
        validate_url(&base)?;
        Ok(Self {
            base,
            transport,
            auth: None,
            auth_header: "authorization",
            secrets: Vec::new(),
        })
    }
    pub fn with_auth(mut self, authorization: String, secret: String) -> Self {
        self.auth_header = "authorization";
        self.secrets
            .push(Arc::new(Zeroizing::new(authorization.clone())));
        if let Some((_, credential)) = authorization.split_once(' ') {
            self.secrets
                .push(Arc::new(Zeroizing::new(credential.to_owned())));
        }
        self.auth = Some(Arc::new(Zeroizing::new(authorization)));
        self.secrets.push(Arc::new(Zeroizing::new(secret)));
        self
    }
    /// Authenticate with a vendor-specific header, retaining redirect/redaction policy.
    pub fn with_header_auth(mut self, name: &'static str, credential: String) -> Self {
        self.auth_header = name;
        self.secrets
            .push(Arc::new(Zeroizing::new(credential.clone())));
        self.auth = Some(Arc::new(Zeroizing::new(credential)));
        self
    }
    pub fn url(&self, path: &str) -> Result<Url> {
        let url = self
            .base
            .join(path)
            .map_err(|_| Error::InvalidRequest("invalid endpoint path"))?;
        if url.origin() != self.base.origin() {
            return Err(Error::InvalidRequest(
                "endpoint paths must remain on the configured origin",
            ));
        }
        Ok(url)
    }
    pub fn request(
        &self,
        method: HttpMethod,
        path: &str,
        params: &Params,
        json_params: bool,
    ) -> Result<HttpRequest> {
        let mut url = self.url(path)?;
        let pairs = encode_params(params, json_params)?;
        let mut headers = vec![("accept".into(), "application/json".into())];
        let mut body = Vec::new();
        if matches!(method, HttpMethod::Get) {
            if !pairs.is_empty() {
                url.query_pairs_mut().extend_pairs(pairs);
            }
        } else {
            body = url::form_urlencoded::Serializer::new(String::new())
                .extend_pairs(pairs)
                .finish()
                .into_bytes();
            headers.push((
                "content-type".into(),
                "application/x-www-form-urlencoded".into(),
            ));
        }
        if let Some(auth) = &self.auth {
            headers.push((self.auth_header.into(), auth.as_str().to_owned()));
        }
        Ok(HttpRequest {
            method,
            url: url.into(),
            headers,
            body,
        })
    }
    pub fn redact(&self, mut value: Value) -> Value {
        fn redact(value: &mut Value, secrets: &[Arc<Zeroizing<String>>]) {
            match value {
                Value::String(s) => {
                    for secret in secrets {
                        if !secret.is_empty() {
                            *s = s.replace(secret.as_str(), "[redacted]");
                        }
                    }
                }
                Value::Array(a) => {
                    for v in a {
                        redact(v, secrets);
                    }
                }
                Value::Object(o) => {
                    for (_, v) in o {
                        redact(v, secrets);
                    }
                }
                _ => {}
            }
        }
        redact(&mut value, &self.secrets);
        value
    }
}
fn validate_url(url: &Url) -> Result<()> {
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        return Err(Error::InvalidRequest(
            "data URLs require HTTP(S) without user information or fragments",
        ));
    }
    Ok(())
}
impl<T: StreamingHttpTransport> HttpClient<T> {
    /// Redirects preserve credentials only on the same origin; HTTPS cannot downgrade.
    /// No requests or partially consumed streams are automatically retried.
    pub async fn send(
        &self,
        mut request: HttpRequest,
    ) -> Result<aux_rs_core::StreamingHttpResponse<T::Body>> {
        for redirect in 0..=5 {
            let previous = Url::parse(&request.url)
                .map_err(|_| Error::InvalidRequest("invalid request URL"))?;
            let response = self.transport.execute_stream(request.clone()).await?;
            if !matches!(response.status, 301 | 302 | 303 | 307 | 308) {
                return Ok(response);
            }
            if !matches!(request.method, HttpMethod::Get) || redirect == 5 {
                return Err(Error::Decode("unexpected or excessive redirect"));
            }
            let location = header(&response.headers, "location")
                .ok_or(Error::Decode("redirect missing location"))?;
            let next = previous
                .join(location)
                .map_err(|_| Error::Decode("invalid redirect URL"))?;
            validate_url(&next)?;
            if previous.scheme() == "https" && next.scheme() != "https" {
                return Err(Error::Decode("insecure redirect"));
            }
            if previous.origin() != next.origin() {
                request.headers.retain(|(name, _)| {
                    !name.eq_ignore_ascii_case("authorization")
                        && !name.eq_ignore_ascii_case(self.auth_header)
                });
            }
            request.url = next.into();
        }
        unreachable!()
    }
    pub async fn open(&self, request: HttpRequest, decode_content: bool) -> Result<DecodedBody> {
        Ok(self.open_mode(request, decode_content, "").await?.1)
    }
    /// Open a bounded body while retaining native response headers such as pagination.
    pub async fn open_with_headers(
        &self,
        request: HttpRequest,
        decode_content: bool,
    ) -> Result<(Vec<(String, String)>, DecodedBody)> {
        self.open_mode(request, decode_content, "").await
    }
    /// Decode file-level compression when it is not specified in HTTP headers.
    pub async fn open_encoded(&self, request: HttpRequest, encoding: &str) -> Result<DecodedBody> {
        Ok(self.open_mode(request, true, encoding).await?.1)
    }
    async fn open_mode(
        &self,
        request: HttpRequest,
        decode_content: bool,
        fallback: &str,
    ) -> Result<(Vec<(String, String)>, DecodedBody)> {
        let response = self.send(request).await?;
        let success = (200..300).contains(&response.status);
        let encoding = if !success {
            header(&response.headers, "content-encoding").unwrap_or("")
        } else if decode_content {
            header(&response.headers, "content-encoding").unwrap_or(fallback)
        } else {
            ""
        };
        let body = DecodedBody::new(response.body, encoding)?;
        if !success {
            let bytes = body.collect(MAX_RECORD_BYTES).await?;
            let native = serde_json::from_slice(&bytes)
                .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into()));
            return Err(Error::Api {
                status: response.status,
                body: self.redact(native),
            });
        }
        Ok((response.headers, body))
    }
    pub async fn json(&self, request: HttpRequest) -> Result<Value> {
        let bytes = self
            .open(request, true)
            .await?
            .collect(MAX_RECORD_BYTES)
            .await?;
        serde_json::from_slice(&bytes).map_err(|_| Error::Decode("invalid vendor JSON response"))
    }
}
pub fn header<'a>(headers: &'a [(String, String)], name: &str) -> Option<&'a str> {
    headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.as_str())
}
pub fn encode_params(params: &Params, json_params: bool) -> Result<Vec<(String, String)>> {
    params
        .iter()
        .filter(|(_, v)| !v.is_null())
        .map(|(k, v)| {
            let value = match v {
                Value::String(s) => s.clone(),
                Value::Array(a) if !json_params => {
                    let mut parts = Vec::new();
                    for v in a {
                        match v {
                            Value::String(s) => parts.push(s.clone()),
                            Value::Number(n) => parts.push(n.to_string()),
                            _ => {
                                return Err(Error::InvalidRequest(
                                    "form arrays require strings or integers",
                                ));
                            }
                        }
                    }
                    parts.join(",")
                }
                Value::Object(_) if !json_params => {
                    return Err(Error::InvalidRequest(
                        "nested objects are not form parameters",
                    ));
                }
                _ => v.to_string(),
            };
            Ok((k.clone(), value))
        })
        .collect()
}
pub fn segment(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes())
        .collect::<String>()
        .replace('+', "%20")
}
