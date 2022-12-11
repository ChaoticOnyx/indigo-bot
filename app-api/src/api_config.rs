use app_macros::config;
use app_shared::models::Secret;

#[config]
#[derive(Debug)]
pub struct ApiConfig {
    pub root_secret: Secret,
}
