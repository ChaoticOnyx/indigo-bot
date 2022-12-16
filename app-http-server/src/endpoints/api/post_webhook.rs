use actix_http::StatusCode;
use actix_web::web::{Json, Path};
use actix_web::{post, HttpResponseBuilder, Responder};

use app_api::Api;
use app_macros::tokio_blocking;
use app_shared::{
    models::{Secret, WebhookPayload},
    prelude::*,
};

#[instrument]
#[post("/webhook/{webhook_secret}")]
pub async fn endpoint(
    webhook_secret: Path<Secret>,
    payload: Option<Json<WebhookPayload>>,
) -> impl Responder {
    trace!("post_webhook");

    let webhook_secret = webhook_secret.into_inner();
    let payload = payload.map(|json| json.into_inner()).unwrap_or_default();

    let result = Api::lock(tokio_blocking!(|api| {
        api.handle_webhook(webhook_secret, payload).await
    }));

    match result {
        Ok(res) => HttpResponseBuilder::new(StatusCode::OK).json(res),
        Err(err) => HttpResponseBuilder::new(StatusCode::BAD_REQUEST).json(err),
    }
}
