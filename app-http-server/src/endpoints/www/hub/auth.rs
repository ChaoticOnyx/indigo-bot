use actix_web::{get, Responder};
use app_shared::{prelude::*, serde_json::json, DiscordSession};

use crate::HtmlResponse;

#[get("/auth")]
pub async fn endpoint() -> impl Responder {
    let session = DiscordSession::clone_state().user.unwrap();

    HtmlResponse::from_template(
        "hub/auth.html",
        Some(json!({
            "bot": {
                "name": session.name,
                "discriminator": session.discriminator,
            }
        })),
    )
    .await
}
