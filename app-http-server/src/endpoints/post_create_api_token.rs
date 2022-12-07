use actix_web::{post, web, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use app_api::Api;
use serde::Deserialize;

use app_shared::{
    chrono::{DateTime, Utc},
    models::{Rights, Secret},
    prelude::*,
};

use crate::response::ResponseHelpers;

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    rights: Rights,
    expiration: Option<DateTime<Utc>>,
}

#[instrument]
#[post("/api/token")]
pub async fn post_create_api_token(body: web::Json<Body>, secret: BearerAuth) -> impl Responder {
    info!("post_create_api_token");

    let Body { rights, expiration } = body.0;
    let secret = Secret(secret.token().to_string());

    let result = Api::lock(async_closure!(|api| {
        let duration = expiration.map(|expiration| expiration - Utc::now());

        api.create_api_token(secret, rights, duration).await
    }))
    .await;

    ResponseHelpers::from_api_result(result)
}
