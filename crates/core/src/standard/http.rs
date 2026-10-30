use super::{OperationGuard, StandardTransport, closed};
use crate::{
    HttpMethod, HttpRequest, HttpResponse, HttpTransport, RequestOutcome, TransportError,
    TransportErrorKind,
};
use std::time::{Duration, SystemTime};

fn map_error(error: reqwest::Error) -> TransportError {
    let (kind, outcome, message) = if error.is_builder() {
        (
            TransportErrorKind::Configuration,
            RequestOutcome::NotSent,
            "invalid HTTP request",
        )
    } else if error.is_timeout() {
        (
            TransportErrorKind::Timeout,
            if error.is_connect() {
                RequestOutcome::NotSent
            } else {
                RequestOutcome::Unknown
            },
            "HTTP request timed out",
        )
    } else if error.is_connect() {
        (
            TransportErrorKind::Connection,
            RequestOutcome::NotSent,
            "HTTP connection failed",
        )
    } else {
        (
            TransportErrorKind::Connection,
            RequestOutcome::Unknown,
            "HTTP request failed",
        )
    };
    TransportError::new(kind, outcome, message)
}
impl StandardTransport {
    async fn once(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let client = self
            .client
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
            .ok_or_else(|| closed(RequestOutcome::NotSent))?;
        let method = match request.method {
            HttpMethod::Get => reqwest::Method::GET,
            HttpMethod::Post => reqwest::Method::POST,
            HttpMethod::Put => reqwest::Method::PUT,
            HttpMethod::Delete => reqwest::Method::DELETE,
        };
        let mut builder = client.request(method, &request.url);
        for (name, value) in &request.headers {
            builder = builder.header(name, value);
        }
        let mut response = builder
            .body(request.body.clone())
            .send()
            .await
            .map_err(map_error)?;
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .filter_map(|(name, value)| {
                value
                    .to_str()
                    .ok()
                    .map(|v| (name.to_string(), v.to_owned()))
            })
            .collect();
        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(map_error)? {
            if chunk.len() > self.config.max_message_bytes.saturating_sub(body.len()) {
                return Err(TransportError::new(
                    TransportErrorKind::ResponseTooLarge,
                    RequestOutcome::Unknown,
                    "HTTP response exceeded configured limit",
                ));
            }
            body.extend_from_slice(&chunk);
        }
        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }
    async fn http(
        &self,
        request: HttpRequest,
        safe_read: bool,
    ) -> Result<HttpResponse, TransportError> {
        let mut shutdown = self.lifecycle.closed.subscribe();
        let _operation = OperationGuard::start(&self.lifecycle)?;
        let retry = &self.config.read_retry;
        let mut attempt = 0;
        loop {
            let result = tokio::select! {biased;_=shutdown.changed()=>return Err(closed(RequestOutcome::Unknown)),result=self.once(&request)=>result};
            if !safe_read || attempt >= retry.max_retries {
                return result;
            }
            let mut delay = retry.delay(attempt);
            match &result {
                Ok(response) if matches!(response.status, 408 | 429 | 500 | 502 | 503 | 504) => {
                    if let Some(wait) = retry_after(response) {
                        // Return the native response rather than retry sooner
                        // than the server requested or exceed the configured cap.
                        if wait > retry.max_delay {
                            return result;
                        }
                        delay = delay.max(wait);
                    }
                }
                Err(error)
                    if matches!(
                        error.kind,
                        TransportErrorKind::Connection | TransportErrorKind::Timeout
                    ) => {}
                _ => return result,
            }
            attempt += 1;
            tokio::select! {biased;_=shutdown.changed()=>return Err(closed(RequestOutcome::NotSent)),_=tokio::time::sleep(delay)=>{}}
        }
    }
}
fn retry_after(response: &HttpResponse) -> Option<Duration> {
    let value = response
        .headers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case("retry-after"))?
        .1
        .trim();
    if !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) {
        return Some(
            value
                .parse::<u64>()
                .map(Duration::from_secs)
                .unwrap_or(Duration::MAX),
        );
    }
    httpdate::parse_http_date(value).ok().map(|deadline| {
        deadline
            .duration_since(SystemTime::now())
            .unwrap_or_default()
    })
}
impl HttpTransport for StandardTransport {
    fn ensure_open(&self) -> Result<(), TransportError> {
        self.lifecycle.check()
    }
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.http(request, false).await
    }
    async fn execute_read(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.http(request, true).await
    }
}
