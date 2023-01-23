use app_shared::{
    models::{ServiceError, WebhookConfiguration, WebhookPayload, WebhookResponse},
    prelude::*,
    serde_json,
};
use serde::{Deserialize, Serialize};

use crate::Service;

#[derive(Debug, Default)]
pub struct EchoService;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    pub message: String,
}

#[async_trait]
impl Service for EchoService {
    #[instrument]
    async fn handle(
        &self,
        configuration: &WebhookConfiguration,
        _payload: &WebhookPayload,
    ) -> Result<WebhookResponse, ServiceError> {
        trace!("handle");

        let config = serde_json::from_value::<Config>(configuration.0.clone()).unwrap();
        let response = WebhookResponse(serde_json::to_value(config.message).unwrap());

        Ok(response)
    }

    #[instrument]
    async fn configure(&self, configuration: &WebhookConfiguration) -> Result<(), ServiceError> {
        trace!("configure");

        match serde_json::from_value::<Config>(configuration.0.clone()) {
            Err(err) => Err(ServiceError::Any(err.to_string())),
            Ok(_) => Ok(()),
        }
    }
}
