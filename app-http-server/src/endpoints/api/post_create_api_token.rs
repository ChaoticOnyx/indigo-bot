use actix_web::{post, web, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use app_api::Api;
use app_macros::tokio_blocking;
use serde::Deserialize;

use app_shared::{
    chrono::{DateTime, Utc},
    models::{Rights, Secret},
    prelude::*,
};

use crate::ResponseHelpers;

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    rights: Rights,
    expiration: Option<DateTime<Utc>>,
    is_service: bool,
}

#[instrument]
#[post("/token")]
pub async fn endpoint(body: web::Json<Body>, secret: BearerAuth) -> impl Responder {
    trace!("endpoint");

    let Body {
        rights,
        expiration,
        is_service,
    } = body.0;
    let secret = Secret(secret.token().to_string());

    let result = Api::lock(tokio_blocking!(|api| {
        let duration = expiration.map(|expiration| expiration - Utc::now());

        api.create_api_token(secret, rights, duration, is_service)
            .await
    }));

    ResponseHelpers::from_api_result(result)
}
