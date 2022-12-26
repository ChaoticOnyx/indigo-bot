use crate::extractors::AuthenticatedUser;
use crate::html_response::HtmlResponse;
use actix_web::{get, Responder};
use app_shared::prelude::*;

#[instrument]
#[get("/account")]
pub async fn endpoint(user: AuthenticatedUser) -> impl Responder {
    trace!("endpoint");

    let ctx = json!({ "user": user });
    HtmlResponse::from_template("hub/account.html", Some(ctx)).await
}
