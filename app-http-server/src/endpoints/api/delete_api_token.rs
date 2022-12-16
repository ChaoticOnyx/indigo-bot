use actix_web::{delete, web, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use app_api::Api;
use app_macros::tokio_blocking;
use serde::Deserialize;

use app_shared::{models::Secret, prelude::*};

use crate::ResponseHelpers;

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    pub target_secret: Secret,
}

#[instrument]
#[delete("/token")]
pub async fn endpoint(body: web::Json<Body>, secret: BearerAuth) -> impl Responder {
    trace!("delete_api_token");

    let Body { target_secret } = body.0;
    let secret = Secret(secret.token().to_string());

    let result = Api::lock(tokio_blocking!(|api| {
        api.delete_api_token(secret, target_secret).await
    }));

    ResponseHelpers::from_api_result(result)
}
