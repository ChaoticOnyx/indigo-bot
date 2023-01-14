use actix_http::StatusCode;
use actix_web::{get, Responder};

use app_api::Api;
pub use app_shared::{models::Account, prelude::*};

use crate::ResponseHelpers;

#[instrument]
#[get("/accounts")]
pub async fn endpoint() -> impl Responder {
    trace!("endpoint");

    let donators: Vec<Account> = Api::lock_async(|api| api.get_accounts())
        .await
        .unwrap()
        .into_iter()
        .filter(|account| account.donation_tier.is_some())
        .collect();

    ResponseHelpers::new(StatusCode::OK, json!(donators))
}
