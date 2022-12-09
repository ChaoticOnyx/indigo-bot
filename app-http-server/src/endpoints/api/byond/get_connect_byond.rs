use actix_web::{get, web, Responder};
use app_api::Api;
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
    trace!("get_connect_byond");

    let Query {
        secret,
        ckey,
        tfa_secret,
    } = query.0;

    let result = Api::lock(async_closure!(|api| {
        api.connect_byond_account_by_2fa(secret, tfa_secret, ckey)
            .await
    }))
    .await;

    ResponseHelpers::from_api_result(result.map(|_| "ok"))
}
