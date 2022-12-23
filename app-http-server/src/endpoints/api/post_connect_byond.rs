use actix_web::{post, web, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use app_api::Api;
use serde::Deserialize;

use app_shared::{models::Secret, prelude::*};

use crate::ResponseHelpers;

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    pub tfa_secret: Secret,
    pub ckey: ByondUserId,
}

#[instrument]
#[post("/connect/byond")]
pub async fn endpoint(query: web::Json<Body>, secret: BearerAuth) -> impl Responder {
    trace!("endpoint");

    let Body { ckey, tfa_secret } = query.0;
    let secret = Secret(secret.token().to_string());

    let result = Api::lock_async(|api| api.connect_byond_account_by_2fa(secret, tfa_secret, ckey))
        .await
        .unwrap();

    ResponseHelpers::from_api_result(result)
}
