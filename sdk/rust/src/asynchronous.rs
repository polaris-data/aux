//! Tokio access to the bundled native SDK.
use crate::{Error, Result};
use std::sync::Arc;
use tokio::task::JoinHandle;

/// Configured clients share credentials and transport state across outstanding calls.
pub struct AsyncClient<C>(Arc<C>);

impl<C> Clone for AsyncClient<C> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

struct Task<T>(JoinHandle<T>);
impl<T> Drop for Task<T> {
    fn drop(&mut self) {
        // A queued native operation must not start after its Rust future was dropped.
        self.0.abort();
    }
}

impl<C: Send + Sync + 'static> AsyncClient<C> {
    pub fn new(client: C) -> Self {
        Self(Arc::new(client))
    }

    /// Run a chain workflow, typed call, or session operation off the executor thread.
    pub async fn run<T, F>(&self, operation: F) -> Result<T>
    where
        T: Send + 'static,
        F: FnOnce(&C) -> Result<T> + Send + 'static,
    {
        let runtime = tokio::runtime::Handle::try_current()
            .map_err(|_| Error::InvalidRequest("async client requires a Tokio runtime"))?;
        let client = self.0.clone();
        let mut task = Task(runtime.spawn_blocking(move || operation(&client)));
        (&mut task.0).await.map_err(|_| Error::Network)?
    }
}

/// A cancelled read leaves its result available to the next reader.
pub struct AsyncEventStream {
    stream: crate::EventStream,
    pending: tokio::sync::Mutex<Option<Task<Result<Option<Vec<u8>>>>>>,
}

impl crate::EventStream {
    pub fn into_async(self) -> AsyncEventStream {
        AsyncEventStream {
            stream: self,
            pending: tokio::sync::Mutex::new(None),
        }
    }
}

impl AsyncEventStream {
    pub async fn next_bytes(&self) -> Result<Option<Vec<u8>>> {
        let mut pending = self.pending.lock().await;
        if pending.is_none() {
            let runtime = tokio::runtime::Handle::try_current()
                .map_err(|_| Error::InvalidRequest("async stream requires a Tokio runtime"))?;
            let stream = self.stream.clone();
            *pending = Some(Task(runtime.spawn_blocking(move || stream.next_bytes())));
        }
        let task = pending.as_mut().expect("read was scheduled");
        let result = (&mut task.0).await;
        *pending = None;
        result.map_err(|_| Error::Network)?
    }

    pub async fn next_json(&self) -> Result<Option<String>> {
        self.next_bytes()
            .await?
            .map(|bytes| String::from_utf8(bytes).map_err(|_| Error::Decode("invalid native JSON")))
            .transpose()
    }

    pub async fn next_event(&self) -> Result<Option<serde_json::Value>> {
        self.next_bytes()
            .await?
            .map(|bytes| {
                serde_json::from_slice(&bytes).map_err(|_| Error::Decode("invalid stream JSON"))
            })
            .transpose()
    }

    /// Interrupt a blocked native read, including one left by a cancelled future.
    pub fn close(&self) {
        self.stream.close();
    }
}

impl Drop for AsyncEventStream {
    fn drop(&mut self) {
        self.close();
    }
}

pub struct AsyncSession<S> {
    pub(crate) commands: AsyncClient<S>,
    events: AsyncEventStream,
}

impl<S: Send + Sync + 'static> AsyncSession<S> {
    pub(crate) fn new(session: S, events: crate::EventStream) -> Self {
        Self {
            commands: AsyncClient::new(session),
            events: events.into_async(),
        }
    }

    pub async fn next_event(&self) -> Result<Option<serde_json::Value>> {
        self.events.next_event().await
    }

    pub fn close(&self) {
        self.events.close()
    }
}
