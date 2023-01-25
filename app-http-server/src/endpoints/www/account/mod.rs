use actix_http::{header, StatusCode};
use actix_web::{web, HttpResponseBuilder, Responder, Scope};

mod integrations_menu;
mod journal;
mod profile_menu;
mod sessions_menu;

async fn redirect() -> impl Responder {
    HttpResponseBuilder::new(StatusCode::TEMPORARY_REDIRECT)
        .insert_header((header::LOCATION, "/account/profile"))
        .finish()
}

pub fn endpoint() -> Scope {
    Scope::new("/account")
        .service(profile_menu::endpoint)
        .service(sessions_menu::endpoint)
        .service(integrations_menu::endpoint)
        .service(journal::endpoint)
        .default_service(web::to(redirect))
}
