use actix_web::Scope;

mod get_index;

pub fn hub() -> Scope {
    actix_web::web::scope("/hub").service(get_index::endpoint)
}
