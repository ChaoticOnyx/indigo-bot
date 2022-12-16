use actix_http::header;
use actix_web::http::StatusCode;
use actix_web::{get, HttpResponseBuilder, Responder};

use app_shared::prelude::*;

#[instrument]
#[get("")]
pub async fn endpoint() -> impl Responder {
    // HtmlResponse::from_template("index.html", None).await
    HttpResponseBuilder::new(StatusCode::TEMPORARY_REDIRECT)
        .insert_header((header::LOCATION, "/404"))
        .finish()
}
