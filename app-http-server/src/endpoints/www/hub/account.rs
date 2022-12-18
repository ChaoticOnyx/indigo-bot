use crate::extractors::AuthorizedSession;
use actix_web::{get, HttpResponse, Responder};
use app_shared::prelude::*;

#[instrument]
#[get("/account")]
pub async fn endpoint(session: AuthorizedSession) -> impl Responder {
    error!("session: {session:#?}");

    HttpResponse::Ok()
}
