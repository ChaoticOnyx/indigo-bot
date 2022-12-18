use actix_web::{get, web, Responder};
use app_api::Api;
use app_macros::tokio_blocking;
use serde::Deserialize;

use app_shared::{models::Secret, prelude::*};

use crate::ResponseHelpers;

#[derive(Debug, Clone, Deserialize)]
pub struct Query {
    pub secret: Secret,
    pub tfa_secret: Secret,
    pub ckey: ByondUserId,
}

#[instrument]
#[get("/byond/connect/byond")]
pub async fn endpoint(query: web::Query<Query>) -> impl Responder {
    trace!("endpoint");

    let Query {
        secret,
        ckey,
        tfa_secret,
    } = query.0;

    let result = Api::lock(tokio_blocking!(|api| {
        api.connect_byond_account_by_2fa(secret, tfa_secret, ckey)
            .await
    }));

    ResponseHelpers::from_api_result(result.map(|_| "ok"))
}
