use std::{collections::HashSet, net::SocketAddr};

use crate::api::models::TokenSecret;
use crate::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serenity::{model::prelude::*, prelude::Mutex};

use super::GlobalState;

const SETTINGS_PATH: &str = "settings.toml";
static SETTINGS: Lazy<Mutex<Option<Settings>>> = Lazy::new(|| Mutex::new(None));

#[async_trait]
impl GlobalState for Settings {
    async fn get_static() -> &'static Lazy<Mutex<Option<Self>>> {
        &SETTINGS
    }
}

impl GlobalStateSet for Settings {}
impl GlobalStateClone for Settings {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub discord: DiscordSection,
    pub github: GithubSection,
    pub commands: CommandsSection,
    pub database: DatabaseSection,
    pub logging: LoggingSection,
    pub server: ServerSection,
    pub api: ApiSection,
}

impl Settings {
    #[instrument]
    pub fn load() -> Self {
        info!("loading settings");
        let config_content = std::fs::read(SETTINGS_PATH).unwrap();

        toml::from_slice(&config_content).unwrap()
    }

    #[instrument]
    pub fn save(&self) {
        info!("saving settings");
        let config_content = toml::to_string_pretty(&self).unwrap();

        std::fs::write(SETTINGS_PATH, &config_content).unwrap();
    }
}

/// `[github]`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubSection {
    pub token: String,
}

/// `[discord]`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordSection {
    pub guild_id: GuildId,
    pub token: String,
}

/// `[commands]`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandsSection {
    pub feedback: FeedbackSection,
}

/// `[commands.feedback]`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackSection {
    pub template: String,
    pub channel_id: ChannelId,
    pub template_message_id: Option<MessageId>,
    pub features_repository: String,
    pub bugs_repository: String,
    pub vote_up_emoji: String,
    pub vote_down_emoji: String,
    pub min_feature_up_votes: u64,
    pub feature_issue_labels: HashSet<String>,
    pub bug_issue_labels: HashSet<String>,
}

/// `[database]`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSection {
    pub connect: String,
}

/// `[logging]`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSection {
    pub loki: LokiSection,
    pub log_level: String,
}

/// `[logging.loki]`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LokiSection {
    pub enabled: bool,
    pub log_level: String,
    pub url: Option<String>,
}

/// `[server]`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSection {
    pub address: SocketAddr,
}

/// `[api]`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSection {
    pub root_secret: TokenSecret,
}
