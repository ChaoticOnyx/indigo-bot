use crate::api::models::{ByondCkey, TokenSecret};
use actix_http::StatusCode;
use actix_web::{get, web, HttpResponseBuilder, Responder};
use serde::Deserialize;

use crate::prelude::*;
use crate::server::response::Response;

#[derive(Debug, Clone, Deserialize)]
pub struct Query {
    pub secret: TokenSecret,
    pub tfa_secret: TokenSecret,
    pub ckey: ByondCkey,
}

#[instrument]
#[get("/bapi/connect/byond")]
pub async fn get_connect_byond(query: web::Query<Query>) -> impl Responder {
    info!("get_connect_byond");

    let Query {
        secret,
        ckey,
        tfa_secret,
    } = query.0;

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
