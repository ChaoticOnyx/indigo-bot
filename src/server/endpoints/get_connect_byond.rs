use crate::api::models::{AnyUserId, ByondCkey, TokenSecret};
use actix_http::StatusCode;
use actix_web::{get, web, HttpResponseBuilder, Responder};
use serde::Deserialize;
use serde_json::json;

use crate::prelude::*;
use crate::server::utils::is_token_valid;

#[derive(Debug, Clone, Deserialize)]
pub struct Query {
    pub secret: TokenSecret,
    pub token: String,
    pub ckey: String,
}

#[instrument]
#[get("/api/connect/byond")]
pub async fn get_connect_byond(query: web::Query<Query>) -> impl Responder {
    info!("get_connect_byond");

    if !is_token_valid(&query.token).await {
        return HttpResponseBuilder::new(StatusCode::UNAUTHORIZED).body(
            json!({
                "response": "invalid token"
            })
            .to_string(),
        );
    }

    let secret = query.secret.clone();
    let ckey = query.ckey.clone();

    if ckey.trim().is_empty() {
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body(
            json!({
                "response": "ckey is empty"
            })
            .to_string(),
        );
    }

    Api::lock(async_closure!(|api| {
        let account = api.find_account_by_token_secret(secret).await;

        let Some(account) = account else {
            return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body(json!({
                "response": "invalid secret"
            }).to_string())
        };

        api.connect_byond_account(AnyUserId::InternalId(account.id), ByondCkey(ckey))
            .await;

        return HttpResponseBuilder::new(StatusCode::OK).body(
            json!({
                "response": "ok"
            })
            .to_string(),
        );
    }))
    .await
}
