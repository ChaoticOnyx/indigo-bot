use actix_web::{delete, web, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use app_api::Api;
use serde::Deserialize;

use app_shared::{models::Secret, prelude::*};

use crate::response::ResponseHelpers;

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    pub target_secret: Secret,
}

#[delete("/api/token")]
pub async fn delete_api_token(body: web::Json<Body>, secret: BearerAuth) -> impl Responder {
    let Body { target_secret } = body.0;
    let secret = Secret(secret.token().to_string());

    let result = Api::lock(async_closure!(|api| {
        api.delete_api_token(secret, target_secret).await
    }))
    .await;

    ResponseHelpers::from_api_result(result)
}
