use crate::Api;
use app_macros::validate_api_secret;
use app_shared::{
    models::{
        ApiError, Secret, ServiceError, ServiceId, Webhook, WebhookConfiguration, WebhookPayload,
        WebhookResponse,
    },
    prelude::*,
};

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

        if !self.private_api.services_storage.is_service_exists(&target) {
            return Err(ApiError::Other("invalid service".to_string()));
        }

        if name.trim().is_empty() {
            return Err(ApiError::Other("webhook name is empty".to_string()));
        }

        match self
            .private_api
            .services_storage
            .configure_webhook(self, &target, &configuration)
            .await
        {
            Ok(_) => (),
            Err(err) => return Err(ApiError::Other(format!("invalid configuration: {err}"))),
        }

        let webhook = Webhook::new(
            name,
            self.private_api.create_unique_webhook_secret().await,
            target,
            None,
            configuration,
        );
        self.private_api.database.add_webhook(webhook.clone()).await;

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
        let webhook = self
            .private_api
            .database
            .find_webhook_by_secret(webhook_secret)
            .await;

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

        self.private_api
            .database
            .delete_webhook_by_secret(webhook.secret)
            .await;

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
            .private_api
            .database
            .find_webhook_by_secret(webhook_secret.clone())
            .await;

        let Some(webhook) = webhook else {
            return Err(ServiceError::Any("invalid webhook".to_string()))
        };

        self.private_api
            .services_storage
            .handle(self, &webhook.service_id, &webhook.configuration, &payload)
            .await
    }
}
