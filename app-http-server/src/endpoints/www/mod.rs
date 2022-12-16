use actix_web::Scope;

pub mod hub;
mod index;
pub mod not_found;

pub fn scope() -> Scope {
    actix_web::web::scope("/").service(index::endpoint)
}
