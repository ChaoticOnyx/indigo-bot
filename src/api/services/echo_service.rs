use crate::api::models::{ServiceError, WebhookPayload, WebhookResponse};
use crate::api::Api;
use crate::api::Service;
use crate::prelude::*;

#[derive(Debug, Default)]
pub struct EchoService {}

#[async_trait]
impl Service for EchoService {
    async fn handle(
        &self,
        _payload: WebhookPayload,
        _api: &Api,
    ) -> Result<WebhookResponse, ServiceError> {
        Ok(serde_json::Value::String("hello, world!".to_string()).into())
    }
}
