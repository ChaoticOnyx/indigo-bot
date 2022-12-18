mod account;
mod api_token;
mod webhook;
mod session;

use crate::api::PrivateApi;
use app_macros::global;
use app_shared::prelude::*;

#[derive(Debug)]
#[global(set, lock)]
pub struct Api {
    pub private_api: PrivateApi,
}

impl Api {
    #[instrument]
    pub async fn new() -> Self {
        info!("creating api");

        Self {
            private_api: PrivateApi::new().await,
        }
    }
}
