use crate::extractors::AuthorizedSession;
use actix_http::header;
use actix_web::http::StatusCode;
use actix_web::{get, HttpResponseBuilder, Responder};
use app_shared::{prelude::*, serde_json::json, DiscordSession};

use crate::HtmlResponse;

#[get("/auth")]
pub async fn endpoint(session: Option<AuthorizedSession>) -> impl Responder {
    if session.is_some() {
        return HttpResponseBuilder::new(StatusCode::TEMPORARY_REDIRECT)
            .insert_header((header::LOCATION, "/"))
            .finish();
    }

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
