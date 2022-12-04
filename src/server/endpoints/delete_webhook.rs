use actix_http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, HttpResponseBuilder, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;

use crate::api::models::Secret;
use crate::prelude::*;
use crate::server::response::Response;

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    pub webhook_secret: Secret,
}

#[instrument]
#[delete("/api/webhook")]
pub async fn delete_webhook(body: Json<Body>, api_secret: BearerAuth) -> impl Responder {
    let Body { webhook_secret } = body.0;
    let api_secret = Secret(api_secret.token().to_string());

    let result = Api::lock(async_closure!(|api| {
        api.delete_webhook(api_secret, webhook_secret).await
    }))
    .await;

    match result {
        Ok(_) => HttpResponseBuilder::new(StatusCode::OK).json(Response::new("ok")),
        Err(err) => HttpResponseBuilder::new(err.clone().into()).json(Response::new(err)),
    }
}
