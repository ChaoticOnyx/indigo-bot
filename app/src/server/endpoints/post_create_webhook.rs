use crate::api::models::{Secret, ServiceId, WebhookConfiguration};
use crate::api::Api;
use crate::prelude::*;
use crate::server::response::Response;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{post, HttpResponseBuilder, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    pub service_id: ServiceId,
    pub configuration: Option<WebhookConfiguration>,
}

#[instrument]
#[post("/api/webhook")]
pub async fn post_create_webhook(body: Json<Body>, api_secret: BearerAuth) -> impl Responder {
    info!("post_create_api_token");

    let Body {
        service_id,
        configuration,
    } = body.0;
    let api_secret = Secret(api_secret.token().to_string());

    let result = Api::lock(async_closure!(|api| {
        api.create_webhook(api_secret, service_id, configuration.unwrap_or_default())
            .await
    }))
    .await;

    match result {
        Ok(webhook) => HttpResponseBuilder::new(StatusCode::OK).json(Response::new(webhook)),
        Err(err) => HttpResponseBuilder::new(err.clone().into()).json(Response::new(err)),
    }
}
