use crate::api::models::{Rights, Secret};
use crate::prelude::*;
use crate::server::response::Response;
use actix_http::StatusCode;
use actix_web::{post, web, HttpResponseBuilder, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{DateTime, Utc};
use serde::Deserialize;

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
        let duration = match expiration {
            None => None,
            Some(expiration) => Some(expiration - Utc::now()),
        };

        api.create_api_token(secret, rights, duration).await
    }))
    .await;

    match result {
        Ok(token) => HttpResponseBuilder::new(StatusCode::OK).json(Response::new(token)),
        Err(err) => HttpResponseBuilder::new(err.clone().into()).json(Response::new(err)),
    }
}
