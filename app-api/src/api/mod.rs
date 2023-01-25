mod account;
mod bug_feature;
mod journal;
mod roles;
mod session;
mod tfa;
mod token;
mod webhook;

use crate::github::Github;
use crate::services::ServicesStorage;
use crate::tfa_tokens_storage::TFATokensStorage;
use app_macros::global;
use app_shared::{
    models::{ApiToken, Rights},
    prelude::*,
    Database,
};

use crate::api_config::ApiConfig;
use crate::discord_api::DiscordApi;

#[derive(Debug)]
#[global(lock, set)]
pub struct Api {
    pub github: Github,
    pub tokens_storage: TFATokensStorage,
    pub services_storage: ServicesStorage,
    pub discord_api: DiscordApi,
}

impl Default for Api {
    fn default() -> Self {
        // GitHub
        let github = Github::default();

        // Tokens storage
        let tokens_storage = TFATokensStorage::default();
        let config = ApiConfig::get().unwrap();

        let root_token = ApiToken::new(config.root_secret, Rights::full(), None, None, true, None);

        Database::lock(|database| database.create_root_token_if_does_not_exist(root_token));

        // Services storage
        let mut services_storage = ServicesStorage::default();
        services_storage.register();

        // Discord API
        let discord_api = DiscordApi::default();

        Self {
            github,
            tokens_storage,
            services_storage,
            discord_api,
        }
    }
}
