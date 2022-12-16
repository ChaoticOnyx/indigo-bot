use actix_web::{post, web, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use app_api::Api;
use app_macros::tokio_blocking;
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
    trace!("get_connect_byond");

    let Body { ckey, tfa_secret } = query.0;
    let secret = Secret(secret.token().to_string());

    let result = Api::lock(tokio_blocking!(|api| {
        api.connect_byond_account_by_2fa(secret, tfa_secret, ckey)
            .await
    }));

    ResponseHelpers::from_api_result(result)
}
