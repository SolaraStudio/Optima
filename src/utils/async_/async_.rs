use std::future::Future;
use std::time::Duration;

pub fn block_on<F: Future>(future: F) -> F::Output {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(future)
}

pub async fn async_sleep(ms: u64) {
    tokio::time::sleep(Duration::from_millis(ms)).await;
}

pub async fn async_spawn<F>(future: F) -> tokio::task::JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    tokio::spawn(future)
}

pub async fn spawn_blocking<F, R>(f: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(f).await.unwrap()
}

pub fn is_in_async_context() -> bool {
    !tokio::runtime::Handle::try_current().is_err()
}

pub async fn async_yield_now() {
    tokio::task::yield_now().await;
}
