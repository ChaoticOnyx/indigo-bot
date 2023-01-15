use actix_web::Scope;

mod get_ss14;

pub fn scope() -> Scope {
    Scope::new("/account").service(get_ss14::endpoint)
}
