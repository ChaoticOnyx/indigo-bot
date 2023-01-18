use crate::services::Service;
use crate::Api;
use app_shared::{
    models::{ServiceError, WebhookConfiguration, WebhookPayload, WebhookResponse},
    prelude::*,
    serde::{Deserialize, Serialize},
    serde_json,
    serenity::model::{channel::Embed, id::RoleId},
};

#[derive(Debug, Default)]
pub struct RoundEndService {
    client: reqwest::Client,
}

impl RoundEndService {
    pub fn format_message(payload: &Payload) -> String {
        format!(
            "Раунд **{}** закончился\n**Режим:** {}\n**Игроков:** {}\n**Продолжительность:** {}",
            payload.round_id, payload.game_mode, payload.players, payload.round_duration
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    pub players: String,
    pub game_mode: String,
    pub round_id: String,
    pub round_duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    pub discord_webhook: String,
    pub role_id_to_mention: Option<RoleId>,
}

#[async_trait]
impl Service for RoundEndService {
    #[instrument]
    async fn handle(
        &self,
        configuration: &WebhookConfiguration,
        payload: &WebhookPayload,
        _api: &Api,
    ) -> Result<WebhookResponse, ServiceError> {
        trace!("handle");

        let payload = match serde_json::from_value::<Payload>(payload.0.clone()) {
            Ok(payload) => payload,
            Err(err) => return Err(ServiceError::Any(err.to_string())),
        };

        let config = serde_json::from_value::<Config>(configuration.0.clone()).unwrap();
        let content = match config.role_id_to_mention {
            None => String::new(),
            Some(role_id) => format!("<@&{role_id}>"),
        };

        let embed =
            Embed::fake(|embed| embed.description(RoundEndService::format_message(&payload)));

        let response = self
            .client
            .post(&config.discord_webhook.to_string())
            .json(&json!({ "content": content, "embeds": [embed] }))
            .send()
            .await
            .unwrap();

        if !response.status().is_success() {
            let response: serde_json::Value = response.json().await.unwrap();
            error!("{response:#?}");

            return Err(ServiceError::Any(response.to_string()));
        }

        Ok(WebhookResponse::default())
    }

    #[instrument]
    async fn configure(
        &self,
        configuration: &WebhookConfiguration,
        _api: &Api,
    ) -> Result<(), ServiceError> {
        trace!("configure");

        match serde_json::from_value::<Config>(configuration.0.clone()) {
            Err(err) => Err(ServiceError::Any(err.to_string())),
            Ok(_) => Ok(()),
        }
    }
}
