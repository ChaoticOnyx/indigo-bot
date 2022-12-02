use crate::api::models::{Rights, TokenSecret};
use crate::prelude::*;
use actix_http::StatusCode;
use actix_web::{post, web, HttpResponseBuilder, Responder};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    secret: TokenSecret,
    rights: Rights,
    expiration: Option<DateTime<Utc>>,
}

#[instrument]
#[post("/api/token")]
pub async fn post_create_api_token(body: web::Json<Body>) -> impl Responder {
    info!("post_create_api_token");

    let Body {
        secret,
        rights,
        expiration,
    } = body.0;

    let result = Api::lock(async_closure!(|api| {
        let duration = match expiration {
            None => None,
            Some(expiration) => Some(expiration - Utc::now()),
        };

        api.create_api_token(secret, rights, duration).await
    }))
    .await;

    match result {
        Ok(token) => HttpResponseBuilder::new(StatusCode::OK).json(token),
        Err(err) => HttpResponseBuilder::new(err.clone().into()).json(err),
    }
}
