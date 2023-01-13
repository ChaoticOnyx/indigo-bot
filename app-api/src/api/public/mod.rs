mod account;
mod api_token;
mod session;
mod webhook;

use crate::api::PrivateApi;
use app_macros::global;

#[derive(Debug, Default)]
#[global(set, lock)]
pub struct Api {
    pub private_api: PrivateApi,
}
