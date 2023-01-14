use actix_http::StatusCode;
use actix_web::{get, Responder};

use app_api::Api;
pub use app_shared::{models::DonationTier, prelude::*};

use crate::ResponseHelpers;

#[instrument]
#[get("/tiers")]
pub async fn endpoint() -> impl Responder {
    trace!("endpoint");

    let tiers: Vec<DonationTier> = Api::lock_async(|api| api.get_donation_tiers())
        .await
        .unwrap();

    ResponseHelpers::new(StatusCode::OK, json!(tiers))
}
