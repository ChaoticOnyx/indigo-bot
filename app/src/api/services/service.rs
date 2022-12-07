use crate::api::models::{ServiceError, WebhookConfiguration, WebhookPayload, WebhookResponse};
use crate::prelude::*;
use std::fmt::Debug;

#[async_trait]
pub trait Service: Debug + Send + Sync {
    async fn handle(
        &self,
        configuration: &WebhookConfiguration,
        payload: &WebhookPayload,
        api: &Api,
    ) -> Result<WebhookResponse, ServiceError>;

    async fn configure(
        &self,
        configuration: &WebhookConfiguration,
        api: &Api,
    ) -> Result<(), ServiceError>;
}
