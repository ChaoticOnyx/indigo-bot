use actix_http::StatusCode;
use actix_web::{get, Responder};

pub use app_shared::{prelude::*, state::discord_session::DiscordSession};

use crate::response::ResponseHelpers;

#[instrument]
#[get("/api/identity")]
pub async fn get_identity() -> impl Responder {
    debug!("get_identity");

    let session = DiscordSession::clone_state().await;

    let Some(user) = session.user else {
		return ResponseHelpers::new(StatusCode::INTERNAL_SERVER_ERROR, "discord session not found")
    };

    ResponseHelpers::new(
        StatusCode::OK,
        json!({
            "id": user.id,
            "discriminator": user.discriminator,
            "name": user.name
        }),
    )
}
