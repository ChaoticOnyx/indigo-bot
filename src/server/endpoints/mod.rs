mod get_auth;
mod get_identity;

pub mod get {
    pub use super::get_auth::get_auth as auth;
    pub use super::get_identity::get_identity as identity;
}
