use actix_web::web::Json;
use actix_web::{post, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use app_api::Api;
use serde::Deserialize;

use app_shared::{
    models::{Secret, ServiceId, WebhookConfiguration},
    prelude::*,
};

use crate::ResponseHelpers;

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    pub service_id: ServiceId,
    pub name: String,
    pub configuration: Option<WebhookConfiguration>,
}

#[instrument]
#[post("/webhook")]
pub async fn endpoint(body: Json<Body>, api_secret: BearerAuth) -> impl Responder {
    trace!("endpoint");

    let Body {
        service_id,
        name,
        configuration,
    } = body.0;
    let api_secret = Secret(api_secret.token().to_string());

    let result = Api::lock_async(|api| {
        api.create_webhook(
            api_secret,
            service_id,
            name,
            configuration.unwrap_or_default(),
        )
    })
    .await
    .unwrap();

    ResponseHelpers::from_api_result(result)
}
