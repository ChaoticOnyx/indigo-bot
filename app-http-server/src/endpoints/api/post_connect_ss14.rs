use actix_web::{post, web, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use app_api::Api;
use serde::Deserialize;

use app_shared::{
    models::{ApiCaller, Secret},
    prelude::*,
};

use crate::ResponseHelpers;

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    pub tfa_secret: Secret,
    pub user_id: SS14UserId,
}

#[instrument]
#[post("/connect/ss14")]
pub async fn endpoint(payload: web::Json<Body>, secret: BearerAuth) -> impl Responder {
    trace!("endpoint");

    let Body {
        user_id,
        tfa_secret,
    } = payload.0;
    let secret = Secret(secret.token().to_string());

    let result = Api::lock_async(|api| {
        api.connect_ss14_account_by_2fa(ApiCaller::Token(secret), tfa_secret, user_id)
    })
    .await
    .unwrap();

    ResponseHelpers::from_api_result(result)
}
