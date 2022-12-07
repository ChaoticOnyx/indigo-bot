use actix_web::{post, web, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use app_api::Api;
use serde::Deserialize;

use app_shared::{models::Secret, prelude::*};

use crate::response::ResponseHelpers;

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    pub tfa_secret: Secret,
    pub ckey: ByondUserId,
}

#[instrument]
#[post("/api/connect/byond")]
pub async fn post_connect_byond(query: web::Json<Body>, secret: BearerAuth) -> impl Responder {
    info!("get_connect_byond");

    let Body { ckey, tfa_secret } = query.0;
    let secret = Secret(secret.token().to_string());

    let result = Api::lock(async_closure!(|api| {
        api.connect_byond_account_by_2fa(secret, tfa_secret, ckey)
            .await
    }))
    .await;

    ResponseHelpers::from_api_result(result)
}
