use crate::api::models::{Secret, WebhookPayload};
use crate::api::Api;
use crate::prelude::*;
use actix_http::StatusCode;
use actix_web::web::{Json, Path};
use actix_web::{post, HttpResponseBuilder, Responder};

#[post("/api/webhook/{webhook_secret}")]
pub async fn post_webhook(
    webhook_secret: Path<Secret>,
    payload: Option<Json<WebhookPayload>>,
) -> impl Responder {
    let webhook_secret = webhook_secret.into_inner();
    let payload = match payload {
        None => WebhookPayload(None),
        Some(payload) => payload.into_inner(),
    };

    let result = Api::lock(async_closure!(|api| {
        api.handle_webhook(webhook_secret, payload).await
    }))
    .await;

    match result {
        Ok(res) => HttpResponseBuilder::new(StatusCode::OK).json(res),
        Err(err) => HttpResponseBuilder::new(StatusCode::BAD_REQUEST).json(err),
    }
}
