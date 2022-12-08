use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::Mutex;

use crate::{services::Service, Api};
use app_shared::{
    chrono::{DateTime, Duration, Utc},
    models::{ServiceError, WebhookConfiguration, WebhookPayload, WebhookResponse},
    prelude::*,
    serde_json,
    serenity::model::channel::Embed,
};

#[derive(Debug, Clone)]
struct MessageInfo {
    pub message: String,
    pub id: u64,
    pub last_send: DateTime<Utc>,
}

#[derive(Debug, Default)]
pub struct ChatToDiscordService {
    client: reqwest::Client,
    last_message_per_webhook: Mutex<RefCell<BTreeMap<String, MessageInfo>>>,
}

const DESCRIPTION_MAX: usize = 4000;

impl ChatToDiscordService {
    #[instrument]
    fn format_message(config: &Config, message: &str) -> String {
        debug!("format_message");

        let timestamp = match &config.timestamp_postfix {
            None => String::new(),
            Some(postfix) => format!("<t:{}:{postfix}> ", Utc::now().timestamp()),
        };

        let prefix = match &config.prefix {
            None => String::new(),
            Some(prefix) => format!("{prefix} "),
        };

        let result_length =
            timestamp.chars().count() + prefix.chars().count() + message.chars().count();

        let message = if result_length > DESCRIPTION_MAX {
            format!(
                "{}...",
                message
                    .chars()
                    .take(DESCRIPTION_MAX - 3)
                    .collect::<String>()
            )
        } else {
            message.to_string()
        };

        format!("{timestamp}{prefix}{message}")
    }

    #[instrument]
    fn update_last_id(&self, webhook: &str, response: serde_json::Value, message: String) {
        debug!("update_last_id");

        let id = response.get("id").unwrap().as_str().unwrap();
        let id = u64::from_str(id).unwrap();

        let _lock = self.last_message_per_webhook.lock().unwrap();
        let mut last_message_per_webhook = _lock.borrow_mut();

        let info = MessageInfo {
            id,
            message,
            last_send: Utc::now(),
        };

        match last_message_per_webhook.entry(webhook.to_string()) {
            Entry::Vacant(vac) => vac.insert(info),
            Entry::Occupied(mut occ) => &mut occ.insert(info),
        };
    }

    #[instrument]
    fn get_message_info_for_webhook(&self, webhook: &str) -> Option<MessageInfo> {
        debug!("get_message_info_for_webhook");

        let _lock = self.last_message_per_webhook.lock().unwrap();
        let last_message_per_webhook = _lock.borrow();

        last_message_per_webhook.get(webhook).cloned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    pub prefix: Option<String>,
    pub timestamp_postfix: Option<String>,
    pub discord_webhook: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Payload {
    pub message: String,
}

#[async_trait]
impl Service for ChatToDiscordService {
    #[instrument]
    async fn handle(
        &self,
        configuration: &WebhookConfiguration,
        payload: &WebhookPayload,
        _api: &Api,
    ) -> Result<WebhookResponse, ServiceError> {
        debug!("handle");

        let payload = match serde_json::from_value::<Payload>(payload.0.clone()) {
            Ok(payload) => payload,
            Err(err) => return Err(ServiceError::Any(err.to_string())),
        };

        let config = serde_json::from_value::<Config>(configuration.0.clone()).unwrap();
        let message = ChatToDiscordService::format_message(&config, &payload.message);
        let last_message = self.get_message_info_for_webhook(&config.discord_webhook);

        let do_patch = match last_message {
            None => false,
            Some(ref info) => {
                info.message.chars().count() + message.chars().count() <= DESCRIPTION_MAX
                    && (Utc::now() - info.last_send) < Duration::seconds(15)
            }
        };

        let message = if do_patch {
            format!("{}\n{message}", &last_message.clone().unwrap().message)
        } else {
            message
        };

        let embed = Embed::fake(|embed| embed.description(message.clone()));

        let response = if do_patch {
            self.client
                .patch(&format!(
                    "{}/messages/{}",
                    config.discord_webhook,
                    last_message.unwrap().id
                ))
                .json(&json!({ "embeds": [embed] }))
                .send()
                .await
                .unwrap()
        } else {
            self.client
                .post(&format!("{}?wait=true", config.discord_webhook))
                .json(&json!({ "embeds": [embed] }))
                .send()
                .await
                .unwrap()
        };

        if !response.status().is_success() {
            let response: serde_json::Value = response.json().await.unwrap();
            error!("{response:#?}");

            return Err(ServiceError::Any(response.to_string()));
        }

        self.update_last_id(
            &config.discord_webhook,
            response.json().await.unwrap(),
            message,
        );

        Ok(WebhookResponse::default())
    }

    #[instrument]
    async fn configure(
        &self,
        configuration: &WebhookConfiguration,
        _api: &Api,
    ) -> Result<(), ServiceError> {
        debug!("configure");

        match serde_json::from_value::<Config>(configuration.0.clone()) {
            Ok(_) => Ok(()),
            Err(err) => Err(ServiceError::Any(err.to_string())),
        }
    }
}
