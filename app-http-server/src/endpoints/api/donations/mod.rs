mod get_accounts;
mod get_tiers;

use actix_web::Scope;

pub fn scope() -> Scope {
    Scope::new("/donations")
        .service(get_accounts::endpoint)
        .service(get_tiers::endpoint)
}
