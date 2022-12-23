mod account;
mod api_token;
mod session;
mod webhook;

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
    pub fn new() -> Self {
        info!("creating api");

        Self {
            private_api: PrivateApi::new(),
        }
    }
}
