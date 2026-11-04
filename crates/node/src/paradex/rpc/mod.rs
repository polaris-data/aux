use super::*;
use aux_rs::standard::StandardConnection;
use tokio::sync::{Mutex, OwnedMutexGuard};
mod operations;
type NativeSession = v::RpcSession<StandardConnection, v::ParadexAuth<Signer>>;
pub struct ParadexRpcSession {
    pub(super) inner: Arc<Mutex<NativeSession>>,
    pub(super) state: Arc<ClientState<Resources>>,
    pub(super) closing: tokio::sync::watch::Sender<bool>,
}
impl ParadexRpcSession {
    pub(crate) async fn run<F, T>(&self, future: F) -> Result<Output>
    where
        F: std::future::Future<Output = Result<T>> + Send + 'static,
        T: serde::Serialize + Send + 'static,
    {
        let mut closing = self.closing.subscribe();
        self.state.run(async move {
            if *closing.borrow() {return Err(errors::ClosedError::new_err("RPC session is closed"));}
            tokio::select! {biased; _=closing.changed()=>Err(errors::ClosedError::new_err("RPC session is closed")),result=future=>result}
        }).await
    }
    pub(super) fn acquire(&self) -> Result<OwnedMutexGuard<NativeSession>> {
        if *self.closing.borrow() {
            return Err(errors::ClosedError::new_err("RPC session is closed"));
        }
        self.inner.clone().try_lock_owned().map_err(|_| {
            errors::BusyError::new_err(
                "one reader or command is already active on this RPC session",
            )
        })
    }
}
impl ParadexRpcSession {
    pub(crate) fn pending(&self) -> Result<Output> {
        let guard = self.acquire()?;
        convert::output(
            guard
                .pending()
                .map(|p| serde_json::json!({"id":p.id,"method":p.method})),
        )
    }
    pub(crate) async fn reconcile(&self) -> Result<Output> {
        let mut guard = self.acquire()?;
        self.run(async move { guard.reconcile().await.map_err(native) })
            .await
    }
    pub(crate) async fn next_event(&self) -> Result<Output> {
        let mut guard = self.acquire()?;
        self.run(async move { guard.next_event().await.transpose().map_err(native) })
            .await
    }
    pub(crate) async fn aclose(&self) -> Result<()> {
        self.closing.send_replace(true);
        let inner = self.inner.clone();
        let cleanup = context::spawn(async move {
            let mut guard = inner.lock_owned().await;
            guard.close().await.map_err(native)
        });
        async move {
            cleanup
                .await
                .map_err(|_| errors::ConnectorError::new_err("RPC cleanup failed"))?
        }
        .await
    }
    pub(crate) fn __repr__(&self) -> &'static str {
        "ParadexRpcSession()"
    }
}
impl Drop for ParadexRpcSession {
    fn drop(&mut self) {
        self.closing.send_replace(true);
        let inner = self.inner.clone();
        napi::bindgen_prelude::spawn(async move {
            let mut guard = inner.lock_owned().await;
            let _ = async { guard.close().await.map_err(native) }.await;
        });
    }
}
