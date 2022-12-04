use crate::api::models::Secret;
use crate::prelude::*;
use crate::server::response::Response;
use actix_http::StatusCode;
use actix_web::{delete, web, HttpResponseBuilder, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;

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

    match result {
        Ok(_) => HttpResponseBuilder::new(StatusCode::OK).json(Response::new("ok")),
        Err(err) => HttpResponseBuilder::new(err.clone().into()).json(Response::new(err)),
    }
}
