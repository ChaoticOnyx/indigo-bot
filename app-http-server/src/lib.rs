mod constants;
mod cookies;
mod endpoints;
mod extractors;
mod filters;
mod html_response;
mod http_config;
mod manifest;
mod middleware;
mod response;
mod server;
mod templates;

use html_response::HtmlResponse;
use response::ResponseHelpers;
pub use server::Server;

pub type FormErrors = std::collections::HashMap<String, Vec<String>>;
