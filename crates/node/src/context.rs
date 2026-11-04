//! Small synchronous helpers shared by the owned native adapters.
pub fn evaluate<T>(f: impl FnOnce() -> T) -> T {
    f()
}
pub fn new<T>(value: T) -> crate::prelude::Result<T> {
    Ok(value)
}
pub fn spawn<F: Future + Send + 'static>(future: F) -> tokio::task::JoinHandle<F::Output>
where
    F::Output: Send + 'static,
{
    napi::bindgen_prelude::within_runtime_if_available(|| tokio::spawn(future))
}
