use actix_web::Scope;

mod account;
mod auth;
mod index;
pub mod not_found;

pub fn scope() -> Scope {
    actix_web::web::scope("")
        .service(index::endpoint)
        .service(auth::endpoint)
        .service(account::endpoint())
}
