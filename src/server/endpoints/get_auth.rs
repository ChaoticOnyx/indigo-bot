use actix_web::{get, http::StatusCode, web, HttpResponseBuilder, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;
use serde_json::json;

use crate::server::utils::is_token_valid;
use crate::{api::models::TokenSecret, prelude::*};

#[derive(Debug, Clone, Deserialize)]
pub struct Query {
    pub secret: TokenSecret,
}

#[instrument]
#[get("/api/auth")]
pub async fn get_auth(query: web::Query<Query>, bearer: BearerAuth) -> impl Responder {
    info!("get_auth");

    if !is_token_valid(bearer.token()).await {
        return HttpResponseBuilder::new(StatusCode::UNAUTHORIZED)
            .body(json!({"message": "Invalid token"}).to_string());
    }

    let secret = query.secret.clone();
    let token = Api::lock(async_closure!(|api| {
        api.find_token_by_secret(secret).await
    }))
    .await;

    let body = match token {
        None => "{}".to_string(),
        Some(token) => serde_json::to_string(&token).unwrap(),
    };

    HttpResponseBuilder::new(StatusCode::OK).body(body)
}
