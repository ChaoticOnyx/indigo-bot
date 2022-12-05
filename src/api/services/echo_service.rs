use crate::api::models::{ServiceError, WebhookConfiguration, WebhookPayload, WebhookResponse};
use crate::api::Api;
use crate::api::Service;
use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct EchoService {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub message: String,
}

#[async_trait]
impl Service for EchoService {
    async fn handle(
        &self,
        configuration: &WebhookConfiguration,
        _payload: &WebhookPayload,
        _api: &Api,
    ) -> Result<WebhookResponse, ServiceError> {
        let config = serde_json::from_value::<Config>(configuration.0.clone()).unwrap();
        let response = WebhookResponse(serde_json::to_value(config.message).unwrap());

        Ok(response)
    }

    async fn configure(
        &self,
        configuration: &WebhookConfiguration,
        _api: &Api,
    ) -> Result<(), ServiceError> {
        match serde_json::from_value::<Config>(configuration.0.clone()) {
            Err(err) => Err(ServiceError::Any(err.to_string())),
            Ok(_) => Ok(()),
        }
    }
}
