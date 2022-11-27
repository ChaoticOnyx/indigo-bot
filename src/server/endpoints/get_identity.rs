use actix_web::{get, Responder};
use serde_json::json;

use crate::prelude::*;

#[instrument]
#[get("/api/identity")]
pub async fn get_identity() -> impl Responder {
    debug!("get_identity");

    let session = DiscordSession::clone_state().await;

    if session.user.is_none() {
        return String::new();
    }

    let user = session.user.unwrap();

    json!({
        "id": user.id,
        "discriminator": user.discriminator,
        "name": user.name
    })
    .to_string()
}
