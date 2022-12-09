use std::fmt::{Display, Formatter};

use app_shared::{
    models::{ApiToken, Rights},
    prelude::*,
    serde::Serialize,
    state::Settings,
};

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
pub struct Api {
    pub database: Database,
    pub github: Github,
    pub tokens_storage: TFATokensStorage,
    pub services_storage: ServicesStorage,
}

impl Api {
    #[instrument]
    pub async fn new(settings: &Settings) -> Self {
        info!("creating api");

        // GitHub
        let github = Github::new(settings.github.token.clone());

        // Database
        let database = Database::connect(&settings.database.connect).await;
        database.migrate().await;

        // Tokens storage
        let tokens_storage = TFATokensStorage::default();

        let root_token = ApiToken::new(
            Settings::clone_state().await.api.root_secret,
            Rights::full(),
            None,
            true,
        );
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
