use std::collections::HashSet;

use crate::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serenity::{model::prelude::*, prelude::Mutex};

const SETTINGS_PATH: &str = "settings.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub discord: DiscordSection,
    pub github: GithubSection,
    pub commands: CommandsSection,
    pub database: DatabaseSection,
    pub loki: LokiSection,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubSection {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordSection {
    pub guild_id: GuildId,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandsSection {
    pub feedback: FeedbackSection,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSection {
    pub connect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LokiSection {
    pub enabled: bool,
    pub url: Option<String>,
}

static SETTINGS: Lazy<Mutex<Option<Settings>>> = Lazy::new(|| Mutex::new(None));

#[async_trait]
impl GlobalState for Settings {
    #[instrument]
    async fn get_state() -> Settings {
        let lock = SETTINGS.lock().await;

        lock.clone().unwrap()
    }

    #[instrument]
    async fn set_state(settings: Settings) {
        let mut lock = SETTINGS.lock().await;
        *lock = Some(settings);
    }
}
