mod account;
mod bug_feature;
mod session;
mod tfa;
mod token;
mod webhook;

use crate::database::Database;
use crate::github::Github;
use crate::services::ServicesStorage;
use crate::tfa_tokens_storage::TFATokensStorage;
use app_shared::{
    models::{ApiToken, Rights},
    prelude::*,
};

use crate::api_config::ApiConfig;

#[derive(Debug)]
pub struct PrivateApi {
    pub database: Database,
    pub github: Github,
    pub tokens_storage: TFATokensStorage,
    pub services_storage: ServicesStorage,
}

impl PrivateApi {
    #[instrument]
    pub async fn new() -> Self {
        info!("creating private api");

        // GitHub
        let github = Github::new().await;

        // Database
        let database = Database::connect().await;
        database.migrate().await;

        // Tokens storage
        let tokens_storage = TFATokensStorage::default();
        let config = ApiConfig::get().unwrap();

        let root_token = ApiToken::new(config.root_secret, Rights::full(), None, None, true);
        database
            .create_root_token_if_does_not_exist(root_token)
            .await;

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
