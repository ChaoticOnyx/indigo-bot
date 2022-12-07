use actix_http::StatusCode;
use actix_web::{get, HttpResponseBuilder, Responder};
use serde_json::json;

use crate::prelude::*;
use crate::server::response::Response;

#[instrument]
#[get("/api/identity")]
pub async fn get_identity() -> impl Responder {
    debug!("get_identity");

    let session = DiscordSession::clone_state().await;

    let Some(user) = session.user else {
        return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .json(Response::new("discord session not found"));
    };

    HttpResponseBuilder::new(StatusCode::OK).json(Response::new(json!({
        "id": user.id,
        "discriminator": user.discriminator,
        "name": user.name
    })))
}
