use crate::api::models::{ServiceError, WebhookPayload, WebhookResponse};
use crate::prelude::*;
use std::fmt::Debug;

#[async_trait]
pub trait Service: Debug + Send + Sync {
    async fn handle(
        &self,
        payload: WebhookPayload,
        api: &Api,
    ) -> Result<WebhookResponse, ServiceError>;
}
