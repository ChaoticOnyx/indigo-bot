use app_macros::global;
use std::fmt::{Display, Formatter};

use app_shared::{
    models::{ApiToken, Rights},
    prelude::*,
    serde::Serialize,
};

use crate::api_config::ApiConfig;
use crate::{Database, Github, ServicesStorage, TFATokensStorage};

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ApiError {
    Unauthorized(String),
    Forbidden(String),
    Other(String),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Unauthorized(msg) => f.write_str(msg),
            ApiError::Forbidden(msg) => f.write_str(msg),
            ApiError::Other(msg) => f.write_str(msg),
        }
    }
}

#[derive(Debug)]
#[global(set, lock)]
pub struct Api {
    pub database: Database,
    pub github: Github,
    pub tokens_storage: TFATokensStorage,
    pub services_storage: ServicesStorage,
}

impl Api {
    #[instrument]
    pub async fn new() -> Self {
        info!("creating api");

        // GitHub
        let github = Github::new().await;

        // Database
        let database = Database::connect().await;
        database.migrate().await;

        // Tokens storage
        let tokens_storage = TFATokensStorage::default();
        let config = ApiConfig::get().unwrap();

        let root_token = ApiToken::new(config.root_secret, Rights::full(), None, true);
        database.update_root_token(root_token).await;

        let mut services_storage = ServicesStorage::new();
        services_storage.register();

        Self {
            database,
            github,
            tokens_storage,
            services_storage,
        }
    }
}
