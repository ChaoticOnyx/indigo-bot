use crate::api::models::{ByondCkey, Secret};
use actix_http::StatusCode;
use actix_web::{post, web, HttpResponseBuilder, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;

use crate::prelude::*;
use crate::server::response::Response;

#[derive(Debug, Clone, Deserialize)]
pub struct Body {
    pub tfa_secret: Secret,
    pub ckey: ByondCkey,
}

#[instrument]
#[post("/api/connect/byond")]
pub async fn post_connect_byond(query: web::Json<Body>, secret: BearerAuth) -> impl Responder {
    info!("get_connect_byond");

    let Body { ckey, tfa_secret } = query.0;
    let secret = Secret(secret.token().to_string());

    let result = Api::lock(async_closure!(|api| {
        api.connect_byond_account_by_2fa(secret, tfa_secret, ckey)
            .await
    }))
    .await;

    match result {
        Ok(_) => HttpResponseBuilder::new(StatusCode::OK).json(Response::new("ok")),
        Err(err) => HttpResponseBuilder::new(err.clone().into()).json(Response::new(err)),
    }
}
