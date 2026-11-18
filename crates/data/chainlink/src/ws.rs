use crate::*;
use aux_rs_core::{
    HandshakeHeaders, RequestOutcome, TransportError, TransportErrorKind, WebSocketSubscription,
    WebSocketTransport,
};
use serde_json::Value;
struct Spec {
    url: String,
    credentials: Credentials,
}
impl WebSocketSubscription for Spec {
    fn handshake_headers(&self) -> HandshakeHeaders<'_> {
        Box::pin(async move {
            self.credentials.headers(&self.url).map_err(|_| {
                TransportError::new(
                    TransportErrorKind::Authentication,
                    RequestOutcome::NotSent,
                    "unable to authenticate Chainlink WebSocket",
                )
            })
        })
    }
    fn connection_scope(&self) -> String {
        self.credentials.scope()
    }
    fn key(&self) -> String {
        self.url.clone()
    }
    fn subscribe_message(&self) -> String {
        String::new()
    }
    fn unsubscribe_message(&self) -> String {
        String::new()
    }
    fn matches(&self, _: &str) -> bool {
        true
    }
    fn validate_message(&self, text: &str) -> std::result::Result<(), TransportError> {
        let value: Value = serde_json::from_str(text).map_err(|_| {
            TransportError::new(
                TransportErrorKind::Protocol,
                RequestOutcome::Unknown,
                "invalid Chainlink report event",
            )
        })?;
        if value
            .get("report")
            .and_then(|r| r.get("fullReport"))
            .and_then(Value::as_str)
            .is_none()
        {
            return Err(TransportError::new(
                TransportErrorKind::Protocol,
                RequestOutcome::Unknown,
                "invalid Chainlink report envelope",
            ));
        }
        Ok(())
    }
}
impl<T: WebSocketTransport> ChainlinkDataStreamsClient<T>
where
    T::Stream: 'static,
{
    pub(crate) async fn subscribe_native(
        &self,
        request: SubscribeReportsRequest,
    ) -> Result<JsonStream> {
        if !self.http.transport.supports_handshake_headers() {
            return Err(Error::InvalidRequest(
                "Chainlink WebSocket transport must support handshake headers",
            ));
        }
        if request.feed_ids.is_empty() {
            return Err(Error::InvalidRequest(
                "at least one Chainlink feed is required",
            ));
        }
        for id in &request.feed_ids {
            let s = id.strip_prefix("0x").unwrap_or(id);
            if s.len() != 64 || !s.bytes().all(|b| b.is_ascii_hexdigit()) {
                return Err(Error::InvalidRequest("invalid Chainlink feed ID"));
            }
        }
        let mut url = url::Url::parse(&self.ws_url)
            .map_err(|_| Error::InvalidRequest("invalid WebSocket URL"))?
            .join("api/v1/ws")
            .map_err(|_| Error::InvalidRequest("invalid WebSocket path"))?;
        url.query_pairs_mut()
            .append_pair("feedIDs", &request.feed_ids.join(","));
        let url = String::from(url);
        let credentials = self
            .credentials
            .clone()
            .ok_or(Error::InvalidRequest("Chainlink API credentials required"))?;
        let stream = self
            .http
            .transport
            .subscribe(
                &url,
                Spec {
                    url: url.clone(),
                    credentials,
                },
            )
            .await?;
        Ok(JsonStream::websockets(vec![stream], false))
    }
}
