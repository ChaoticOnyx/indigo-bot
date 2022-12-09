use std::fmt::Debug;

use futures_util::future::BoxFuture;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::prelude::*;

#[async_trait]
pub trait GlobalState
where
    Self: Sized + 'static + Debug,
{
    async fn get_static() -> &'static Lazy<Mutex<Option<Self>>>;
}

#[async_trait]
pub trait GlobalStateClone: GlobalState
where
    Self: Clone,
{
    async fn clone_state() -> Self {
        let var = Self::get_static().await;
        let lock = var.lock().await;

        lock.clone().unwrap()
    }
}

#[async_trait]
pub trait GlobalStateSet: GlobalState {
    async fn set_state(value: Self) {
        let var = Self::get_static().await;
        let mut lock = var.lock().await;

        *lock = Some(value);
    }
}

#[async_trait]
pub trait GlobalStateLock: GlobalState {
    #[must_use]
    async fn lock<F, O>(f: F) -> O
    where
        F: Send + FnOnce(&mut Self) -> BoxFuture<'_, O>,
        Self: Sync + Send,
    {
        let var = Self::get_static().await;
        let mut lock = var.lock().await;

        f(lock.as_mut().unwrap()).await
    }

    fn lock_sync<F, O>(f: F) -> O
    where
        F: Send + FnOnce(&mut Self) -> BoxFuture<'_, O>,
        Self: Sync + Send,
    {
        tokio::runtime::Handle::current().block_on(async {
            let var = Self::get_static().await;
            let mut lock = var.lock().await;

            f(lock.as_mut().unwrap()).await
        })
    }
}
