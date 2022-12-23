use actix_web::web::Json;
use actix_web::{delete, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;

use app_api::Api;
use app_shared::{models::Secret, prelude::*};

use crate::ResponseHelpers;

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    pub webhook_secret: Secret,
}

#[instrument]
#[delete("/webhook")]
pub async fn endpoint(body: Json<Body>, api_secret: BearerAuth) -> impl Responder {
    trace!("endpoint");

    let Body { webhook_secret } = body.0;
    let api_secret = Secret(api_secret.token().to_string());

    let result = Api::lock_async(|api| api.delete_webhook(api_secret, webhook_secret))
        .await
        .unwrap();

    ResponseHelpers::from_api_result(result)
}
