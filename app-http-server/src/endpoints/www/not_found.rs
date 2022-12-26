use actix_web::Responder;

use crate::html_response::HtmlResponse;
use app_shared::prelude::*;

#[instrument]
pub async fn endpoint() -> impl Responder {
    HtmlResponse::from_template("404.html", None::<()>).await
}
