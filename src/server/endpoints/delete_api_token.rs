use crate::api::models::TokenSecret;
use crate::prelude::*;
use crate::server::response::Response;
use actix_http::StatusCode;
use actix_web::{delete, web, HttpResponseBuilder, Responder};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    pub secret: TokenSecret,
    pub target_secret: TokenSecret,
}

#[delete("/api/token")]
pub async fn delete_api_token(body: web::Json<Body>) -> impl Responder {
    let Body {
        secret,
        target_secret,
    } = body.0;

    let result = Api::lock(async_closure!(|api| {
        api.delete_api_token(secret, target_secret).await
    }))
    .await;

    match result {
        Ok(_) => HttpResponseBuilder::new(StatusCode::OK).json(Response::new("ok")),
        Err(err) => HttpResponseBuilder::new(err.clone().into()).json(Response::new(err)),
    }
}
