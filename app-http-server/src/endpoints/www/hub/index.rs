use actix_web::{get, Responder};

use crate::HtmlResponse;

#[get("")]
pub async fn endpoint() -> impl Responder {
    HtmlResponse::from_template("hub/index.html", None).await
}
