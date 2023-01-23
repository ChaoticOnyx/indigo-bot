use actix_web::{delete, web, Responder};
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
    pub target_secret: Secret,
}

#[instrument]
#[delete("/token")]
pub async fn endpoint(body: web::Json<Body>, secret: BearerAuth) -> impl Responder {
    trace!("endpoint");

    let Body { target_secret } = body.0;
    let secret = Secret(secret.token().to_string());

    let result =
        Api::lock_async(|api| api.delete_api_token(ApiCaller::Token(secret), target_secret))
            .await
            .unwrap();

    ResponseHelpers::from_api_result(result)
}
