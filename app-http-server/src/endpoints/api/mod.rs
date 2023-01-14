use actix_web::Scope;

mod byond;
mod delete_api_token;
mod delete_webhook;
mod donations;
mod get_identity;
mod post_add_account_role;
mod post_auth;
mod post_connect_byond;
mod post_connect_ss14;
mod post_create_api_token;
mod post_create_webhook;
mod post_webhook;

pub fn scope() -> Scope {
    actix_web::web::scope("/api")
        // GET
        .service(get_identity::endpoint)
        // POST
        .service(post_connect_byond::endpoint)
        .service(post_connect_ss14::endpoint)
        .service(post_create_api_token::endpoint)
        .service(post_webhook::endpoint)
        .service(post_create_webhook::endpoint)
        .service(post_add_account_role::endpoint)
        .service(post_auth::endpoint)
        // DELETE
        .service(delete_api_token::endpoint)
        .service(delete_webhook::endpoint)
        // BYOND-friendly (retarded) API
        .service(byond::get_connect_byond::endpoint)
        .service(byond::get_webhook::endpoint)
        // /api/donations
        .service(donations::scope())
}
