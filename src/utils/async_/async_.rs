use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

pub fn block_on<F: Future>(future: F) -> F::Output {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(future)
}

pub async fn sleep(ms: u64) {
    tokio::time::sleep(Duration::from_millis(ms)).await;
}

pub async fn spawn<F>(future: F) -> tokio::task::JoinHandle<F::Output>
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

pub fn current_task_id() -> Option<tokio::task::Id> {
    tokio::task::try_id()
}

pub fn is_in_async_context() -> bool {
    current_task_id().is_some()
}
