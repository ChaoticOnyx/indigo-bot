use actix_http::StatusCode;
use actix_web::{get, Responder};

pub use app_shared::{prelude::*, DiscordSession};

use crate::ResponseHelpers;

#[instrument]
#[get("/identity")]
pub async fn endpoint() -> impl Responder {
    trace!("endpoint");

    let session = DiscordSession::clone_state();

    let Some(user) = session.user else {
		return ResponseHelpers::new(StatusCode::INTERNAL_SERVER_ERROR, "Discord сессия не найдена")
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
