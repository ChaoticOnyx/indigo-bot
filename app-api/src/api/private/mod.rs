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
use crate::discord_api::DiscordApi;

#[derive(Debug)]
pub struct PrivateApi {
    pub database: Database,
    pub github: Github,
    pub tokens_storage: TFATokensStorage,
    pub services_storage: ServicesStorage,
    pub discord_api: DiscordApi,
}

impl Default for PrivateApi {
    fn default() -> Self {
        // GitHub
        let github = Github::default();

        // Database
        let database = Database::connect();
        database.migrate();

        // Tokens storage
        let tokens_storage = TFATokensStorage::default();
        let config = ApiConfig::get().unwrap();

        let root_token = ApiToken::new(config.root_secret, Rights::full(), None, None, true, None);
        database.create_root_token_if_does_not_exist(root_token);

        // Services storage
        let mut services_storage = ServicesStorage::default();
        services_storage.register();

        // Discord API
        let discord_api = DiscordApi::default();

        Self {
            database,
            github,
            tokens_storage,
            services_storage,
            discord_api,
        }
    }
}
