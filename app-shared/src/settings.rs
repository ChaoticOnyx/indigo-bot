use crate::prelude::*;
use app_macros::global;
use serde::{Deserialize, Serialize};

const SETTINGS_PATH: &str = "settings.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[global(set, clone)]
pub struct Settings {
    pub logging: LoggingSection,
}

impl Settings {
    #[instrument]
    pub fn load() -> Self {
        trace!("load");

        let config_content = std::fs::read(SETTINGS_PATH).unwrap();

        toml::from_slice(&config_content).unwrap()
    }

    #[instrument]
    pub fn save(&self) {
        trace!("save");

        let config_content = toml::to_string_pretty(&self).unwrap();

        std::fs::write(SETTINGS_PATH, &config_content).unwrap();
    }
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
