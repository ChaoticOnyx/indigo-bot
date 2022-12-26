mod account;
mod api_token;
mod session;
mod webhook;

use crate::api::PrivateApi;
use app_macros::global;

#[derive(Debug)]
#[global(set, lock)]
pub struct Api {
    pub private_api: PrivateApi,
}

impl Api {
    pub fn new() -> Self {
        Self {
            private_api: PrivateApi::new(),
        }
    }
}
