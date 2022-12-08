mod get_connect_byond;
mod get_webhook;

pub mod get {
    pub use super::get_connect_byond::get_connect_byond as connect_byond;
    pub use super::get_webhook::get_webhook as webhook;
}
