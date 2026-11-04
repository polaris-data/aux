use super::{OperationGuard, StandardTransport, closed, http::map_error};
use crate::{
    ByteStream, HttpMethod, HttpRequest, RequestOutcome, StreamingHttpResponse,
    StreamingHttpTransport, TransportError, TransportErrorKind,
};
use tokio::sync::mpsc;

/// One chunk of read-ahead. Dropping or closing the body cancels its driver.
pub struct StandardBody {
    receiver: mpsc::Receiver<Result<Vec<u8>, TransportError>>,
    driver: tokio::task::JoinHandle<()>,
    shutdown: tokio::sync::watch::Receiver<bool>,
    done: bool,
}
impl Drop for StandardBody {
    fn drop(&mut self) {
        self.driver.abort();
    }
}
impl ByteStream for StandardBody {
    async fn next(&mut self) -> Option<Result<Vec<u8>, TransportError>> {
        if self.done {
            return None;
        }
        if *self.shutdown.borrow() {
            self.done = true;
            self.receiver.close();
            self.driver.abort();
            return Some(Err(closed(RequestOutcome::Unknown)));
        }
        let item = tokio::select! {biased;
            _=self.shutdown.changed()=>Some(Err(closed(RequestOutcome::Unknown))),
            item=self.receiver.recv()=>item,
        };
        if item.is_none() || item.as_ref().is_some_and(Result::is_err) {
            self.done = true;
            self.receiver.close();
            self.driver.abort();
        }
        item
    }
    async fn close(&mut self) -> Result<(), TransportError> {
        self.done = true;
        self.receiver.close();
        self.driver.abort();
        let _ = (&mut self.driver).await;
        while self.receiver.try_recv().is_ok() {}
        Ok(())
    }
}
impl StreamingHttpTransport for StandardTransport {
    type Body = StandardBody;
    async fn execute_stream(
        &self,
        request: HttpRequest,
    ) -> Result<StreamingHttpResponse<StandardBody>, TransportError> {
        let guard = OperationGuard::start(&self.lifecycle)?;
        let mut shutdown = self.lifecycle.closed.subscribe();
        self.lifecycle.check()?;
        let client = self
            .streaming_client
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
            .ok_or_else(|| closed(RequestOutcome::NotSent))?;
        let method = match request.method {
            HttpMethod::Get => reqwest::Method::GET,
            HttpMethod::Post => reqwest::Method::POST,
            HttpMethod::Put => reqwest::Method::PUT,
            HttpMethod::Patch => reqwest::Method::PATCH,
            HttpMethod::Delete => reqwest::Method::DELETE,
        };
        let mut builder = client.request(method, &request.url);
        for (key, value) in request.headers {
            builder = builder.header(key, value);
        }
        let send = builder.body(request.body).send();
        let mut response = tokio::select! { biased;
            _ = shutdown.changed() => return Err(closed(RequestOutcome::Unknown)),
            result = tokio::time::timeout(self.config.timeout, send) => result
                .map_err(|_| TransportError::new(TransportErrorKind::Timeout,RequestOutcome::Unknown,"HTTP response headers timed out"))?
                .map_err(map_error)?,
        };
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .filter_map(|(k, v)| v.to_str().ok().map(|v| (k.to_string(), v.to_owned())))
            .collect();
        let (sender, receiver) = mpsc::channel(1);
        let max = self.config.max_message_bytes;
        let body_shutdown = shutdown.clone();
        let driver = tokio::spawn(async move {
            let _guard = guard;
            loop {
                let chunk = tokio::select! { biased;
                    _ = shutdown.changed() => { let _ = sender.try_send(Err(closed(RequestOutcome::Unknown))); break; },
                    result = response.chunk() => result,
                };
                let (item, terminal) = match chunk {
                    Ok(None) => break,
                    Ok(Some(bytes)) if bytes.len() <= max => (Ok(bytes.to_vec()), false),
                    Ok(Some(_)) => (
                        Err(TransportError::new(
                            TransportErrorKind::ResponseTooLarge,
                            RequestOutcome::Unknown,
                            "HTTP chunk exceeded configured limit",
                        )),
                        true,
                    ),
                    Err(e) => (Err(map_error(e)), true),
                };
                tokio::select! { biased;
                    _ = shutdown.changed() => break,
                    sent = sender.send(item) => if sent.is_err() { break; },
                }
                if terminal {
                    break;
                }
            }
        });
        Ok(StreamingHttpResponse {
            status,
            headers,
            body: StandardBody {
                receiver,
                driver,
                shutdown: body_shutdown,
                done: false,
            },
        })
    }
}
