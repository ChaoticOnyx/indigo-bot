use actix_web::Scope;

mod integrations_menu;
mod profile_menu;
mod sessions_menu;

pub fn endpoint() -> Scope {
    Scope::new("/account")
        .service(profile_menu::endpoint)
        .service(sessions_menu::endpoint)
        .service(integrations_menu::endpoint)
}
