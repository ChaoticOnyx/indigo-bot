use actix_web::{get, Responder};

use crate::html_response::HtmlResponse;
use app_shared::prelude::*;

#[instrument]
#[get("/404")]
pub async fn endpoint() -> impl Responder {
    HtmlResponse::from_template("404.html", None).await
}
