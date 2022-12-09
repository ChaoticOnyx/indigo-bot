use app_macros::validate_api_secret;
use app_shared::{
    chrono::Utc,
    models::{
        Secret, ServiceError, ServiceId, Webhook, WebhookConfiguration, WebhookPayload,
        WebhookResponse,
    },
    prelude::*,
};

use crate::{Api, ApiError};

impl Api {
    #[instrument]
    pub async fn create_webhook(
        &self,
        api_secret: Secret,
        target: ServiceId,
        name: String,
        configuration: WebhookConfiguration,
    ) -> Result<Webhook, ApiError> {
        trace!("create_webhook");

        let token = validate_api_secret!(api_secret);

        if !token.rights.service.can_create_tokens_for_service(&target) {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        if !self.services_storage.is_service_exists(&target) {
            return Err(ApiError::Other("invalid service".to_string()));
        }

        if name.trim().is_empty() {
            return Err(ApiError::Other("webhook name is empty".to_string()));
        }

        match self
            .services_storage
            .configure_webhook(self, &target, &configuration)
            .await
        {
            Ok(_) => (),
            Err(err) => return Err(ApiError::Other(format!("invalid configuration: {err}"))),
        }

        let secret = Secret::new_random_webhook_secret();
        let webhook = Webhook {
            name,
            secret,
            service_id: target,
            created_at: Utc::now(),
            configuration,
        };

        self.database.add_webhook(webhook.clone()).await;

        Ok(webhook)
    }

    #[instrument]
    pub async fn delete_webhook(
        &self,
        api_secret: Secret,
        webhook_secret: Secret,
    ) -> Result<(), ApiError> {
        trace!("delete_webhook");

        let token = validate_api_secret!(api_secret);
        let webhook = self.database.find_webhook_by_secret(webhook_secret).await;

        let Some(webhook) = webhook else {
            return Err(ApiError::Other("invalid webhook secret".to_string()))
        };

        if !token
            .rights
            .service
            .can_delete_tokens_for_service(&webhook.service_id)
        {
            return Err(ApiError::Forbidden("insufficient access".to_string()));
        }

        self.database.delete_webhook_by_secret(webhook.secret).await;

        Ok(())
    }

    #[instrument]
    pub async fn handle_webhook(
        &self,
        webhook_secret: Secret,
        payload: WebhookPayload,
    ) -> Result<WebhookResponse, ServiceError> {
        trace!("handle_webhook");

        let webhook = self
            .database
            .find_webhook_by_secret(webhook_secret.clone())
            .await;

        let Some(webhook) = webhook else {
            return Err(ServiceError::Any("invalid webhook".to_string()))
        };

        self.services_storage
            .handle(self, &webhook.service_id, &webhook.configuration, &payload)
            .await
    }
}
