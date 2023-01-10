use actix_web::{get, Responder};

use crate::html_response::HtmlResponse;
use app_shared::prelude::*;

#[instrument]
#[get("/")]
pub async fn endpoint() -> impl Responder {
    HtmlResponse::from_template("index.html", None::<()>).await
}
