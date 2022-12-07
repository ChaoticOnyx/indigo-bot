use std::fmt::Debug;

use app_shared::{
    models::{ServiceError, WebhookConfiguration, WebhookPayload, WebhookResponse},
    prelude::*,
};

use crate::Api;

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
