use actix_http::StatusCode;
use actix_web::{
    get,
    web::{Path, Query},
    HttpResponseBuilder, Responder,
};

use app_api::Api;
use app_shared::{
    models::{Secret, WebhookPayload},
    prelude::*,
    serde_json,
};

#[instrument]
#[get("/byond/webhook/{webhook_secret}")]
pub async fn endpoint(
    webhook_secret: Path<Secret>,
    payload: Option<Query<serde_json::Value>>,
) -> impl Responder {
    trace!("endpoint");

    let webhook_secret = webhook_secret.into_inner();
    let payload = payload.map(|json| json.into_inner()).unwrap_or_default();

    let result = Api::lock_async(|api| api.handle_webhook(webhook_secret, WebhookPayload(payload)))
        .await
        .unwrap();

    match result {
        Ok(res) => HttpResponseBuilder::new(StatusCode::OK).json(res),
        Err(err) => HttpResponseBuilder::new(StatusCode::BAD_REQUEST).json(err),
    }
}
